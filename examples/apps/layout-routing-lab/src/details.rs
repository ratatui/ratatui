//! Details pane with a nested form focus scope.

use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::cursor::CursorRequests;
use ratatui_layout::focus::{FocusState, FocusTarget, FocusTargets};
use ratatui_layout::input::{ButtonRow, TextFieldState};
use ratatui_layout::linear::{Column, Row};
use ratatui_layout::regions::Regions;

use crate::model::Task;
use crate::route::{FocusScope, RouteMap, Target};

/// Editable form fields.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum FormField {
    /// Task title field.
    Title,
    /// Task owner field.
    Owner,
}

impl FormField {
    /// All fields in visual order.
    const ALL: [Self; 2] = [Self::Title, Self::Owner];

    /// Returns the field label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Owner => "owner",
        }
    }

    /// Returns the rendered prefix width before editable text.
    const fn prefix_width(self) -> u16 {
        self.label().len() as u16 + 2
    }
}

/// Form command buttons.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum FormCommand {
    /// Restore the draft from the selected task.
    Cancel,
    /// Apply the draft to the selected task.
    Save,
}

impl FormCommand {
    /// All commands in left-to-right order.
    const ALL: [Self; 2] = [Self::Cancel, Self::Save];

    /// Returns the button label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Cancel => "cancel",
            Self::Save => "save",
        }
    }
}

/// Result of handling a details-pane key.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DetailsAction {
    /// The key was handled without changing the selected task.
    Handled(String),
    /// Save the draft into the selected task.
    Save { title: String, owner: String },
    /// The details pane did not handle the key.
    Unhandled,
}

/// Details pane state owned by the app.
#[derive(Debug)]
pub struct DetailsPane {
    task_index: Option<usize>,
    draft: TaskDraft,
    fields: FieldStates,
    buttons: ButtonRow<FormCommand>,
}

impl Default for DetailsPane {
    fn default() -> Self {
        Self {
            task_index: None,
            draft: TaskDraft::default(),
            fields: FieldStates::default(),
            buttons: ButtonRow::new(FormCommand::ALL),
        }
    }
}

impl DetailsPane {
    /// Loads the selected task into the draft when selection changes.
    pub fn sync_task(&mut self, task_index: usize, task: &Task) {
        if self.task_index == Some(task_index) {
            return;
        }
        self.task_index = Some(task_index);
        self.reset_from(task);
    }

    /// Renders the pane and returns route, focus, and cursor requests.
    pub fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        task: &Task,
        focused: Option<Target>,
    ) -> DetailsFrame {
        let areas = DetailsAreas::new(area);
        let fields = areas.field_plan();
        let commands = areas.command_plan();
        let save_enabled = self.draft.can_save(task);

        self.sync_button_focus(focused);
        frame.render_widget(Block::new().borders(Borders::ALL).title("details"), area);
        Self::render_summary(frame, areas.summary, task);
        self.render_fields(frame, &fields, focused);
        Self::render_commands(frame, &commands, focused, save_enabled);

        let route = Self::route_plan(area, &fields, &commands, save_enabled);
        let scope = Self::focus_scope(&fields, &commands, save_enabled);
        let cursor = self
            .cursor_plan(&fields, focused)
            .final_cursor()
            .map(|request| request.position);
        DetailsFrame {
            route,
            scope,
            cursor,
        }
    }

    /// Handles a key after the focused leaf had first chance.
    pub fn handle_key(
        &mut self,
        key: KeyCode,
        focus: &mut FocusState<Target>,
        scope: &FocusScope,
        selected: &Task,
    ) -> DetailsAction {
        if self.handle_field_key(key, focus.focused()) {
            return DetailsAction::Handled("field handled text/cursor key".into());
        }
        if !scope.owns(focus.focused()) {
            return DetailsAction::Unhandled;
        }
        match key {
            KeyCode::Tab | KeyCode::Down => {
                scope.next(focus);
                DetailsAction::Handled("form scope handled next focus".into())
            }
            KeyCode::BackTab | KeyCode::Up => {
                scope.previous(focus);
                DetailsAction::Handled("form scope handled previous focus".into())
            }
            KeyCode::Left => self.move_button(focus, ButtonMove::Previous),
            KeyCode::Right => self.move_button(focus, ButtonMove::Next),
            KeyCode::Enter => self.activate(focus.focused(), selected),
            _ => DetailsAction::Unhandled,
        }
    }

    /// Restores the draft to the selected task.
    pub fn reset_from(&mut self, task: &Task) {
        self.draft = TaskDraft::from_task(task);
        self.fields = FieldStates::from_draft(&self.draft);
    }

    fn handle_field_key(&mut self, key: KeyCode, focused: Option<Target>) -> bool {
        let Some(Target::Field(field)) = focused else {
            return false;
        };
        let value = self.draft.value_mut(field);
        let state = self.fields.state_mut(field);
        match key {
            KeyCode::Char(ch) => state.insert_char(value, ch),
            KeyCode::Backspace => state.backspace(value),
            KeyCode::Delete => state.delete(value),
            KeyCode::Home => state.move_home(),
            KeyCode::End => state.move_end(value),
            KeyCode::Left => state.move_left(),
            KeyCode::Right => state.move_right(value),
            _ => return false,
        }
        true
    }

    fn move_button(
        &mut self,
        focus: &mut FocusState<Target>,
        direction: ButtonMove,
    ) -> DetailsAction {
        let Some(Target::FormCommand(_)) = focus.focused() else {
            return DetailsAction::Unhandled;
        };
        match direction {
            ButtonMove::Previous => self.buttons.move_previous(),
            ButtonMove::Next => self.buttons.move_next(),
        }
        if let Some(command) = self.buttons.focused_id() {
            focus.focus(Some(Target::FormCommand(command)));
        }
        DetailsAction::Handled("button row handled horizontal focus".into())
    }

    fn activate(&mut self, focused: Option<Target>, selected: &Task) -> DetailsAction {
        match focused {
            Some(Target::FormCommand(FormCommand::Cancel)) => {
                self.reset_from(selected);
                DetailsAction::Handled("cancel restored the selected task".into())
            }
            Some(Target::FormCommand(FormCommand::Save)) if self.draft.can_save(selected) => {
                DetailsAction::Save {
                    title: self.draft.title.clone(),
                    owner: self.draft.owner.clone(),
                }
            }
            Some(Target::FormCommand(FormCommand::Save)) => {
                DetailsAction::Handled("save is disabled until the draft changes".into())
            }
            Some(Target::Field(_)) => DetailsAction::Handled("field accepted Enter".into()),
            _ => DetailsAction::Unhandled,
        }
    }

    fn render_summary(frame: &mut Frame, area: Rect, task: &Task) {
        let text = format!("selected #{:02}    state {}", task.id.0, task.state.label());
        frame.render_widget(
            Paragraph::new(text).style(Style::new().fg(Color::Gray)),
            area,
        );
    }

    fn render_fields(&self, frame: &mut Frame, plan: &Regions<FormField>, focused: Option<Target>) {
        for field in FormField::ALL {
            let Some(area) = plan.area_for(field) else {
                continue;
            };
            let selected = focused == Some(Target::Field(field));
            let style = if selected {
                Style::new()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::new()
            };
            let text = format!("{}: {}", field.label(), self.draft.value(field));
            frame.render_widget(Paragraph::new(text).style(style), area);
        }
    }

    fn render_commands(
        frame: &mut Frame,
        plan: &Regions<FormCommand>,
        focused: Option<Target>,
        save_enabled: bool,
    ) {
        for command in FormCommand::ALL {
            let Some(area) = plan.area_for(command) else {
                continue;
            };
            let enabled = command != FormCommand::Save || save_enabled;
            let selected = focused == Some(Target::FormCommand(command));
            let style = command_style(enabled, selected);
            frame.render_widget(Paragraph::new(button_label(command)).style(style), area);
        }
    }

    fn route_plan(
        pane: Rect,
        fields: &Regions<FormField>,
        commands: &Regions<FormCommand>,
        save_enabled: bool,
    ) -> RouteMap {
        let mut route = RouteMap::new().target(Target::DetailsPane, pane, 0).target(
            Target::DetailsForm,
            fields.area(),
            1,
        );
        for field in FormField::ALL {
            if let Some(area) = fields.area_for(field) {
                route = route.target(Target::Field(field), area, 2);
            }
        }
        for command in FormCommand::ALL {
            if let Some(area) = commands.area_for(command) {
                let target = Target::FormCommand(command);
                route = if command == FormCommand::Save && !save_enabled {
                    route.disabled_target(target, area, 2)
                } else {
                    route.target(target, area, 2)
                };
            }
        }
        route
    }

    fn focus_scope(
        fields: &Regions<FormField>,
        commands: &Regions<FormCommand>,
        save_enabled: bool,
    ) -> FocusScope {
        let field_targets = fields.iter().enumerate().map(|(order, region)| {
            FocusTarget::new(Target::Field(region.id), region.area, order as u16)
        });
        let button_targets = commands.iter().enumerate().map(|(index, region)| {
            let command = region.id;
            let order = FormField::ALL.len() as u16 + index as u16;
            let disabled = command == FormCommand::Save && !save_enabled;
            FocusTarget::new(Target::FormCommand(command), region.area, order).disabled(disabled)
        });
        let targets = field_targets.chain(button_targets).collect::<Vec<_>>();
        FocusScope::new(
            Target::DetailsForm,
            FocusTargets::from_targets(targets),
            false,
        )
    }

    fn cursor_plan(&self, fields: &Regions<FormField>, focused: Option<Target>) -> CursorRequests {
        let mut cursor = CursorRequests::new();
        let Some(Target::Field(field)) = focused else {
            return cursor;
        };
        let Some(area) = fields.area_for(field) else {
            return cursor;
        };
        cursor.push(
            self.fields
                .state(field)
                .cursor_request_after_prefix(area, field.prefix_width()),
        );
        cursor
    }

    fn sync_button_focus(&mut self, focused: Option<Target>) {
        if let Some(Target::FormCommand(command)) = focused {
            self.buttons.focus_id(&command);
        }
    }
}

/// Facts produced by rendering the details pane.
#[derive(Debug, Clone)]
pub struct DetailsFrame {
    /// Mouse and parent-chain route data.
    pub route: RouteMap,
    /// Local form focus scope.
    pub scope: FocusScope,
    /// Requested terminal cursor position.
    pub cursor: Option<ratatui::layout::Position>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ButtonMove {
    Previous,
    Next,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct TaskDraft {
    title: String,
    owner: String,
}

impl TaskDraft {
    fn from_task(task: &Task) -> Self {
        Self {
            title: task.title.clone(),
            owner: task.owner.clone(),
        }
    }

    fn can_save(&self, task: &Task) -> bool {
        !self.title.trim().is_empty()
            && !self.owner.trim().is_empty()
            && (self.title != task.title || self.owner != task.owner)
    }

    fn value(&self, field: FormField) -> &str {
        match field {
            FormField::Title => &self.title,
            FormField::Owner => &self.owner,
        }
    }

    const fn value_mut(&mut self, field: FormField) -> &mut String {
        match field {
            FormField::Title => &mut self.title,
            FormField::Owner => &mut self.owner,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
struct FieldStates {
    title: TextFieldState,
    owner: TextFieldState,
}

impl FieldStates {
    fn from_draft(draft: &TaskDraft) -> Self {
        Self {
            title: TextFieldState::at_end(&draft.title),
            owner: TextFieldState::at_end(&draft.owner),
        }
    }

    const fn state(self, field: FormField) -> TextFieldState {
        match field {
            FormField::Title => self.title,
            FormField::Owner => self.owner,
        }
    }

    const fn state_mut(&mut self, field: FormField) -> &mut TextFieldState {
        match field {
            FormField::Title => &mut self.title,
            FormField::Owner => &mut self.owner,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct DetailsAreas {
    summary: Rect,
    fields: Rect,
    commands: Rect,
}

impl DetailsAreas {
    fn new(area: Rect) -> Self {
        let inner = Rect::new(
            area.x + 1,
            area.y + 1,
            area.width.saturating_sub(2),
            area.height.saturating_sub(2),
        );
        let rows = [
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ];
        let [summary, fields, commands] = Layout::vertical(rows).spacing(1).areas(inner);
        Self {
            summary,
            fields,
            commands,
        }
    }

    fn field_plan(self) -> Regions<FormField> {
        let rows = [
            (FormField::Title, Constraint::Length(1)),
            (FormField::Owner, Constraint::Length(1)),
        ];
        Column::named(rows).spacing(1).regions(self.fields)
    }

    fn command_plan(self) -> Regions<FormCommand> {
        let buttons = [
            (FormCommand::Cancel, Constraint::Length(10)),
            (FormCommand::Save, Constraint::Length(8)),
        ];
        Row::named(buttons)
            .spacing(2)
            .flex(Flex::End)
            .regions(self.commands)
    }
}

const fn button_label(command: FormCommand) -> &'static str {
    match command {
        FormCommand::Cancel => " cancel ",
        FormCommand::Save => " save ",
    }
}

const fn command_style(enabled: bool, selected: bool) -> Style {
    match (enabled, selected) {
        (false, _) => Style::new().fg(Color::DarkGray).bg(Color::Black),
        (true, true) => Style::new()
            .fg(Color::Black)
            .bg(Color::LightGreen)
            .add_modifier(Modifier::BOLD),
        (true, false) => Style::new().fg(Color::Black).bg(Color::Green),
    }
}
