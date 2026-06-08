//! Edit a saved connection profile through a pending draft.
//!
//! This example shows the form flow that appears in many TUIs: copy a saved value into editable
//! fields, let the user move between fields and buttons, validate the draft, and only apply the
//! draft when Save is activated. `ratatui-layout` supplies the planned regions, focus repair,
//! button-row movement, and text-field cursor mechanics. The application still owns the domain
//! data and chooses the rendering style.

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui_layout::{
    ButtonRow, Column, CursorRequests, FocusFallback, FocusState, FocusTarget, FocusTargets,
    Regions, Row, TextFieldState,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = App::default();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| app.render(frame))?;
            if let Some(key) = event::read()?.as_key_press_event()
                && !app.handle_key(key.code)
            {
                break Ok(());
            }
        }
    })
}

/// Application state for the whole editor.
///
/// The saved profile is the committed value. The draft is the editable buffer. Field and button
/// state only store control mechanics, so Save and Cancel stay ordinary domain actions.
#[derive(Debug)]
struct App {
    /// Last committed value shown in the summary row.
    saved: Profile,
    /// Pending user edits that can be saved or discarded.
    draft: ProfileDraft,
    /// Cursor positions for each editable string in the draft.
    fields: FieldStates,
    /// App-level focus, using one id enum for fields and commands.
    focus: FocusState<Target>,
    /// Row-local focus for the horizontal command buttons.
    buttons: ButtonRow<Command>,
    /// Focus data produced by the previous render and used by the next key event.
    focus_plan: FocusTargets<Target>,
}

impl Default for App {
    fn default() -> Self {
        let saved = Profile::example();
        let draft = ProfileDraft::from_profile(&saved);
        Self {
            fields: FieldStates::from_draft(&draft),
            saved,
            draft,
            focus: FocusState::default(),
            buttons: ButtonRow::new(Command::ALL),
            focus_plan: FocusTargets::new(),
        }
    }
}

impl App {
    /// Renders the current frame and stores the focus target data needed by the next input event.
    ///
    /// The render pass owns geometry. It computes field and button regions, builds a focus target
    /// collection from those regions, repairs stale focus, and then draws the domain state into
    /// the planned areas.
    fn render(&mut self, frame: &mut Frame) {
        let areas = PageAreas::new(frame.area());
        let field_plan = areas.field_plan();
        let button_plan = areas.button_plan();
        let validation = self.draft.validate_against(&self.saved);

        self.focus_plan = Self::build_focus_plan(&field_plan, &button_plan, validation);
        self.focus
            .ensure_visible(&self.focus_plan, FocusFallback::First);
        self.sync_button_focus();

        frame.render_widget(Clear, areas.panel);
        let block = Block::new()
            .borders(Borders::ALL)
            .title("connection profile");
        frame.render_widget(block, areas.panel);
        self.render_summary(frame, areas.summary);
        self.render_fields(frame, &field_plan);
        Self::render_validation(frame, areas.validation, validation);
        self.render_buttons(frame, &button_plan, validation);
        self.place_cursor(frame, &field_plan);
    }

    /// Routes one key press to the current focus target.
    ///
    /// Fields use ordinary text editing keys. Buttons use left/right movement. Tab and vertical
    /// movement traverse the app-level focus target collection that was produced by the last
    /// render.
    fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Esc => return self.escape(),
            KeyCode::Tab | KeyCode::Down => self.focus.next(&self.focus_plan),
            KeyCode::BackTab | KeyCode::Up => self.focus.previous(&self.focus_plan),
            KeyCode::Left => self.move_left(),
            KeyCode::Right => self.move_right(),
            KeyCode::Home => self.edit_focused(Edit::Home),
            KeyCode::End => self.edit_focused(Edit::End),
            KeyCode::Backspace => self.edit_focused(Edit::Backspace),
            KeyCode::Delete => self.edit_focused(Edit::Delete),
            KeyCode::Enter => self.activate_focused(),
            KeyCode::Char(ch) => self.edit_focused(Edit::Insert(ch)),
            _ => {}
        }
        true
    }

    /// Cancels dirty edits, or exits when there is nothing left to discard.
    fn escape(&mut self) -> bool {
        if self.draft.changed_from(&self.saved) {
            self.cancel();
            true
        } else {
            false
        }
    }

    /// Moves left inside a field or to the previous enabled command in the button row.
    fn move_left(&mut self) {
        match self.focus.focused() {
            Some(Target::Field(_)) => self.edit_focused(Edit::Left),
            Some(Target::Command(_)) => self.move_button(Direction::Previous),
            None => {}
        }
    }

    /// Moves right inside a field or to the next enabled command in the button row.
    fn move_right(&mut self) {
        match self.focus.focused() {
            Some(Target::Field(_)) => self.edit_focused(Edit::Right),
            Some(Target::Command(_)) => self.move_button(Direction::Next),
            None => {}
        }
    }

    /// Moves button-row focus while skipping disabled commands.
    ///
    /// `ButtonRow` knows only left-to-right movement. The app applies the domain rule that Save is
    /// unavailable until the draft is valid and changed.
    fn move_button(&mut self, direction: Direction) {
        let button_count = self.buttons.ids().len();
        for _ in 0..button_count {
            match direction {
                Direction::Previous => self.buttons.move_previous(),
                Direction::Next => self.buttons.move_next(),
            }

            let Some(command) = self.buttons.focused_id() else {
                return;
            };
            if self.command_enabled(command) {
                self.focus.focus(Some(Target::Command(command)));
                return;
            }
        }
    }

    /// Applies a text-edit command to the currently focused field.
    fn edit_focused(&mut self, edit: Edit) {
        let Some(Target::Field(field)) = self.focus.focused() else {
            return;
        };

        let value = self.draft.value_mut(field);
        let state = self.fields.state_mut(field);
        edit.apply(state, value);
    }

    /// Activates the focused field or command.
    ///
    /// Enter advances out of a field, activates Cancel, or attempts Save. Save performs its own
    /// validation check because disabled focus is a UI affordance, not the domain guard.
    fn activate_focused(&mut self) {
        match self.focus.focused() {
            Some(Target::Field(_)) => self.focus.next(&self.focus_plan),
            Some(Target::Command(Command::Cancel)) => self.cancel(),
            Some(Target::Command(Command::Save)) => self.save(),
            None => {}
        }
    }

    /// Restores the draft and field cursors from the saved profile.
    fn cancel(&mut self) {
        self.draft = ProfileDraft::from_profile(&self.saved);
        self.fields = FieldStates::from_draft(&self.draft);
    }

    /// Commits a valid draft to the saved profile.
    fn save(&mut self) {
        if !self.draft.validate_against(&self.saved).save_enabled {
            return;
        }
        self.draft.apply_to(&mut self.saved);
        self.fields = FieldStates::from_draft(&self.draft);
    }

    /// Returns whether a command can currently be focused or activated.
    fn command_enabled(&self, command: Command) -> bool {
        match command {
            Command::Cancel => true,
            Command::Save => self.draft.validate_against(&self.saved).save_enabled,
        }
    }

    /// Keeps row-local button focus aligned with app-level focus.
    ///
    /// This lets Tab move into the command row while left/right still use `ButtonRow`.
    fn sync_button_focus(&mut self) {
        if let Some(Target::Command(command)) = self.focus.focused() {
            self.buttons.focus_id(&command);
        }
    }

    /// Builds app-level focus from field regions and button regions.
    ///
    /// Field regions map directly into `Target::Field`. Button regions are rebuilt as focus targets
    /// so the Save command can stay visible while being skipped by traversal.
    fn build_focus_plan(
        fields: &Regions<Field>,
        buttons: &Regions<Command>,
        validation: Validation,
    ) -> FocusTargets<Target> {
        let field_slots = fields.regions().iter().copied();
        let field_focus = FocusTargets::from_regions(field_slots).map_id(Target::Field);
        let button_targets = buttons
            .iter()
            .enumerate()
            .map(|(index, region)| {
                let command = region.id;
                let disabled = command == Command::Save && !validation.save_enabled;
                let order = Field::ALL.len() as u16 + index as u16;
                FocusTarget::new(Target::Command(command), region.area, order).disabled(disabled)
            })
            .collect::<Vec<_>>();
        let button_focus = FocusTargets::from_targets(button_targets);
        field_focus.merge(button_focus)
    }

    /// Renders the committed value so the draft/save relationship is visible.
    fn render_summary(&self, frame: &mut Frame, area: Rect) {
        let text = format!(
            "saved: {}@{}:{}/{}",
            self.saved.name, self.saved.host, self.saved.port, self.saved.database
        );
        frame.render_widget(
            Paragraph::new(text).style(Style::new().fg(Color::Gray)),
            area,
        );
    }

    /// Renders every planned field region.
    fn render_fields(&self, frame: &mut Frame, plan: &Regions<Field>) {
        for field in Field::ALL {
            let Some(area) = plan.area_for(field) else {
                continue;
            };
            self.render_field(frame, area, field);
        }
    }

    /// Renders one label/value row and highlights the focused editable value.
    fn render_field(&self, frame: &mut Frame, area: Rect, field: Field) {
        let text = format!("{}: {}", field.label(), self.draft.value(field));
        let style = match self.focus.focused() {
            Some(Target::Field(focused)) if focused == field => Style::new()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            _ => Style::new(),
        };
        frame.render_widget(Paragraph::new(text).style(style), area);
    }

    /// Renders the validation message that explains whether Save is available.
    fn render_validation(frame: &mut Frame, area: Rect, validation: Validation) {
        let style = if validation.save_enabled {
            Style::new().fg(Color::Green)
        } else {
            Style::new().fg(Color::Yellow)
        };
        frame.render_widget(Paragraph::new(validation.message).style(style), area);
    }

    /// Renders the command row from its planned button regions.
    fn render_buttons(&self, frame: &mut Frame, plan: &Regions<Command>, validation: Validation) {
        for command in Command::ALL {
            let Some(area) = plan.area_for(command) else {
                continue;
            };
            self.render_button(frame, area, command, validation);
        }
    }

    /// Renders one command button with enabled, disabled, and focused states.
    fn render_button(
        &self,
        frame: &mut Frame,
        area: Rect,
        command: Command,
        validation: Validation,
    ) {
        let enabled = command != Command::Save || validation.save_enabled;
        let focused = self.focus.focused() == Some(Target::Command(command));
        let style = match (enabled, focused) {
            (false, _) => Style::new().fg(Color::DarkGray).bg(Color::Black),
            (true, true) => Style::new()
                .fg(Color::Black)
                .bg(command.selected_color())
                .add_modifier(Modifier::BOLD),
            (true, false) => Style::new().fg(Color::Black).bg(command.color()),
        };
        frame.render_widget(Paragraph::new(command.label()).style(style), area);
    }

    /// Places the terminal cursor at the focused field's text cursor.
    ///
    /// `TextFieldState` stores a character index. The cursor request combines that index with the
    /// field's rendered label prefix and current frame area.
    fn place_cursor(&self, frame: &mut Frame, plan: &Regions<Field>) {
        let Some(Target::Field(field)) = self.focus.focused() else {
            return;
        };
        let Some(area) = plan.area_for(field) else {
            return;
        };
        let request = self
            .fields
            .state(field)
            .cursor_request_after_prefix(area, field.prefix_width());
        if let Some(cursor) = CursorRequests::new().request(request).final_cursor() {
            frame.set_cursor_position(cursor.position);
        }
    }
}

/// Committed connection settings.
///
/// A real app would load this from disk or a service. Keeping it separate from `ProfileDraft` shows
/// how a form can edit safely without mutating committed data on every key press.
#[derive(Debug, Clone)]
struct Profile {
    /// Display name for the connection.
    name: String,
    /// Hostname or address.
    host: String,
    /// Validated numeric port.
    port: u16,
    /// Database name.
    database: String,
}

impl Profile {
    /// Builds the initial saved profile used by the example.
    fn example() -> Self {
        Self {
            name: "release-bot".into(),
            host: "db.internal".into(),
            port: 5432,
            database: "ratatui".into(),
        }
    }
}

/// Editable copy of a [`Profile`].
///
/// The port is a string here because users can temporarily type invalid text. Validation decides
/// when that draft can be converted back into the committed domain value.
#[derive(Debug, Clone)]
struct ProfileDraft {
    /// Editable profile name.
    name: String,
    /// Editable host.
    host: String,
    /// Editable port text, which may be invalid while typing.
    port: String,
    /// Editable database name.
    database: String,
}

impl ProfileDraft {
    /// Copies a committed profile into an editable draft.
    fn from_profile(profile: &Profile) -> Self {
        Self {
            name: profile.name.clone(),
            host: profile.host.clone(),
            port: profile.port.to_string(),
            database: profile.database.clone(),
        }
    }

    /// Returns the string value for a field.
    ///
    /// Rendering and cursor placement use this read-only path.
    fn value(&self, field: Field) -> &str {
        match field {
            Field::Name => &self.name,
            Field::Host => &self.host,
            Field::Port => &self.port,
            Field::Database => &self.database,
        }
    }

    /// Returns the mutable string value for a field.
    ///
    /// Text editing uses this to keep field state and domain draft state separate but adjacent.
    const fn value_mut(&mut self, field: Field) -> &mut String {
        match field {
            Field::Name => &mut self.name,
            Field::Host => &mut self.host,
            Field::Port => &mut self.port,
            Field::Database => &mut self.database,
        }
    }

    /// Validates the draft and reports whether Save should be enabled.
    ///
    /// The same result drives both the status text and the disabled Save focus target.
    fn validate_against(&self, saved: &Profile) -> Validation {
        if self.name.trim().is_empty() {
            return Validation::disabled("name is required");
        }
        if self.host.trim().is_empty() {
            return Validation::disabled("host is required");
        }
        if !self.port_is_valid() {
            return Validation::disabled("port must be 1-65535");
        }
        if self.database.trim().is_empty() {
            return Validation::disabled("database is required");
        }
        if !self.changed_from(saved) {
            return Validation::disabled("no pending changes");
        }
        Validation::enabled("ready to save")
    }

    /// Returns whether the port can become the committed numeric port.
    fn port_is_valid(&self) -> bool {
        self.port.parse::<u16>().is_ok_and(|port| port > 0)
    }

    /// Returns whether the draft differs from the committed profile.
    fn changed_from(&self, saved: &Profile) -> bool {
        self.name != saved.name
            || self.host != saved.host
            || self.database != saved.database
            || self.port.parse::<u16>().ok() != Some(saved.port)
    }

    /// Applies a previously validated draft to the committed profile.
    fn apply_to(&self, profile: &mut Profile) {
        profile.name.clone_from(&self.name);
        profile.host.clone_from(&self.host);
        profile.port = self.port.parse::<u16>().unwrap_or(profile.port);
        profile.database.clone_from(&self.database);
    }
}

/// Result of validating a draft for display and command availability.
#[derive(Debug, Clone, Copy)]
struct Validation {
    /// Whether the Save command should be focusable and actionable.
    save_enabled: bool,
    /// Short status text shown below the fields.
    message: &'static str,
}

impl Validation {
    /// Creates a validation result that enables Save.
    const fn enabled(message: &'static str) -> Self {
        Self {
            save_enabled: true,
            message,
        }
    }

    /// Creates a validation result that keeps Save visible but disabled.
    const fn disabled(message: &'static str) -> Self {
        Self {
            save_enabled: false,
            message,
        }
    }
}

/// Editable fields in visual traversal order.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Field {
    /// Profile display name.
    Name,
    /// Hostname or address.
    Host,
    /// Port text.
    Port,
    /// Database name.
    Database,
}

impl Field {
    /// All fields in top-to-bottom render and focus order.
    const ALL: [Self; 4] = [Self::Name, Self::Host, Self::Port, Self::Database];

    /// Returns the label rendered before the editable value.
    const fn label(self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::Host => "host",
            Self::Port => "port",
            Self::Database => "database",
        }
    }

    /// Returns the terminal width before editable text begins.
    const fn prefix_width(self) -> u16 {
        self.label().len() as u16 + 2
    }
}

/// Commands in the bottom button row.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Command {
    /// Discard pending edits.
    Cancel,
    /// Commit a valid changed draft.
    Save,
}

impl Command {
    /// All commands in left-to-right render and movement order.
    const ALL: [Self; 2] = [Self::Cancel, Self::Save];

    /// Returns the rendered button text, including horizontal padding.
    const fn label(self) -> &'static str {
        match self {
            Self::Cancel => " cancel ",
            Self::Save => " save ",
        }
    }

    /// Returns the base background color for an enabled button.
    const fn color(self) -> Color {
        match self {
            Self::Cancel => Color::DarkGray,
            Self::Save => Color::Green,
        }
    }

    /// Returns the brighter background color for a focused button.
    const fn selected_color(self) -> Color {
        match self {
            Self::Cancel => Color::Gray,
            Self::Save => Color::LightGreen,
        }
    }
}

/// App-level focus target.
///
/// A single focus id lets vertical traversal move through fields and then into the horizontal
/// command row. The inner ids stay domain-specific.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Target {
    /// One editable field row.
    Field(Field),
    /// One command button.
    Command(Command),
}

/// Horizontal movement direction for the command row.
#[derive(Debug, Clone, Copy)]
enum Direction {
    /// Move to the previous enabled command.
    Previous,
    /// Move to the next enabled command.
    Next,
}

/// Text edit operation routed to the focused field.
#[derive(Debug, Clone, Copy)]
enum Edit {
    /// Insert one character at the field cursor.
    Insert(char),
    /// Delete the character before the field cursor.
    Backspace,
    /// Delete the character under the field cursor.
    Delete,
    /// Move the field cursor one character left.
    Left,
    /// Move the field cursor one character right.
    Right,
    /// Move the field cursor to the start.
    Home,
    /// Move the field cursor to the end.
    End,
}

impl Edit {
    /// Applies the edit to a cursor state and its externally owned value.
    fn apply(self, field: &mut TextFieldState, value: &mut String) {
        match self {
            Self::Insert(ch) => field.insert_char(value, ch),
            Self::Backspace => field.backspace(value),
            Self::Delete => field.delete(value),
            Self::Left => field.move_left(),
            Self::Right => field.move_right(value),
            Self::Home => field.move_home(),
            Self::End => field.move_end(value),
        }
    }
}

/// Cursor state for each field in the draft.
///
/// This mirrors the draft fields because `TextFieldState` deliberately owns only cursor position,
/// not the text itself.
#[derive(Debug, Clone)]
struct FieldStates {
    /// Cursor state for [`Field::Name`].
    name: TextFieldState,
    /// Cursor state for [`Field::Host`].
    host: TextFieldState,
    /// Cursor state for [`Field::Port`].
    port: TextFieldState,
    /// Cursor state for [`Field::Database`].
    database: TextFieldState,
}

impl FieldStates {
    /// Creates field states with each cursor at the end of its current draft value.
    fn from_draft(draft: &ProfileDraft) -> Self {
        Self {
            name: TextFieldState::at_end(&draft.name),
            host: TextFieldState::at_end(&draft.host),
            port: TextFieldState::at_end(&draft.port),
            database: TextFieldState::at_end(&draft.database),
        }
    }

    /// Returns a copy of the state for rendering cursor placement.
    const fn state(&self, field: Field) -> TextFieldState {
        match field {
            Field::Name => self.name,
            Field::Host => self.host,
            Field::Port => self.port,
            Field::Database => self.database,
        }
    }

    /// Returns mutable state for editing the focused field.
    const fn state_mut(&mut self, field: Field) -> &mut TextFieldState {
        match field {
            Field::Name => &mut self.name,
            Field::Host => &mut self.host,
            Field::Port => &mut self.port,
            Field::Database => &mut self.database,
        }
    }
}

/// Page-level areas solved before field and button values are created.
#[derive(Debug, Clone, Copy)]
struct PageAreas {
    /// Outer panel containing the whole editor.
    panel: Rect,
    /// Summary of committed settings.
    summary: Rect,
    /// Area for the vertical field plan.
    fields: Rect,
    /// Status and validation text.
    validation: Rect,
    /// Area for the horizontal command plan.
    buttons: Rect,
}

impl PageAreas {
    /// Centers and splits the editor panel into named sections.
    fn new(area: Rect) -> Self {
        let panel = centered(area, 64, 14);
        let rows = [
            Constraint::Length(1),
            Constraint::Length(7),
            Constraint::Length(1),
            Constraint::Length(1),
        ];
        let [summary, fields, validation, buttons] =
            Layout::vertical(rows).margin(1).spacing(1).areas(panel);
        Self {
            panel,
            summary,
            fields,
            validation,
            buttons,
        }
    }

    /// Builds typed field regions from the field section.
    fn field_plan(self) -> Regions<Field> {
        let rows = [
            (Field::Name, Constraint::Length(1)),
            (Field::Host, Constraint::Length(1)),
            (Field::Port, Constraint::Length(1)),
            (Field::Database, Constraint::Length(1)),
        ];
        Column::named(rows).spacing(1).regions(self.fields)
    }

    /// Builds typed command regions from the button section.
    fn button_plan(self) -> Regions<Command> {
        let buttons = [
            (Command::Cancel, Constraint::Length(8)),
            (Command::Save, Constraint::Length(6)),
        ];
        let row = Row::named(buttons).spacing(2).flex(Flex::End);
        row.regions(self.buttons)
    }
}

/// Centers a fixed-size rectangle inside `area`.
fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let [vertical] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    let [horizontal] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(vertical);
    horizontal
}
