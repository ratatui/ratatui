//! Compose child-local frame snapshots without introducing a component trait.
//!
//! This example sits between the tiny `frame_snapshot` example and the larger workspace inspector.
//! Each pane renders ordinary Ratatui widgets, returns a `FrameSnapshot` with local ids and
//! coordinates, and lets `App` map, translate, clip, and merge that data into one previous-frame
//! snapshot.

use std::io;

use color_eyre::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind};
use crossterm::execute;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::{
    CursorRequest, CursorRequests, FocusFallback, FocusState, FrameSnapshot, FrameTargets, Region,
    Regions,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    execute!(io::stdout(), EnableMouseCapture)?;
    let result = run();
    execute!(io::stdout(), DisableMouseCapture)?;
    result
}

fn run() -> Result<()> {
    let mut app = App::default();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| app.render(frame))?;
            match event::read()? {
                event::Event::Key(key) if key.is_press() && !app.handle_key(key.code) => {
                    break Ok(());
                }
                event::Event::Mouse(mouse) => app.handle_mouse(mouse),
                _ => {}
            }
        }
    })
}

#[derive(Debug)]
struct App {
    previous_frame: FrameSnapshot<Target>,
    focus: FocusState<Target>,
    project: ProjectPane,
    details: DetailsPane,
    command: CommandInput,
}

impl Default for App {
    fn default() -> Self {
        Self {
            previous_frame: FrameSnapshot::new(Rect::default()),
            focus: FocusState::default(),
            project: ProjectPane::default(),
            details: DetailsPane::default(),
            command: CommandInput::default(),
        }
    }
}

impl App {
    /// Renders each child in its screen area, then stores one merged frame snapshot for input
    /// routing.
    fn render(&mut self, frame: &mut Frame) {
        let page = PageAreas::new(frame.area());
        let project_focus = self.focus.focused().and_then(Target::project);
        let details_focus = self.focus.focused().and_then(Target::details);
        let command_focus = self.focus.focused().and_then(Target::command);

        let project = self
            .project
            .render(frame, page.project, project_focus)
            .map_id(Target::Project);
        let details = self
            .details
            .render(
                frame,
                page.details,
                self.project.selected_name(),
                details_focus,
            )
            .map_id(Target::Details);
        let command = self
            .command
            .render(frame, page.command, command_focus)
            .map_id(Target::Command);

        let frame_snapshot = FrameSnapshot::new(frame.area())
            .place_child(project, page.project)
            .place_child(details, page.details)
            .place_child(command, page.command);

        self.focus
            .ensure_visible(&frame_snapshot.focus, FocusFallback::First);
        if let Some(cursor) = frame_snapshot.cursor.final_cursor() {
            frame.set_cursor_position(cursor.position);
        }
        self.previous_frame = frame_snapshot;
    }

    /// Routes keys through the currently focused app-level target.
    fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Char('j') | KeyCode::Down => self.move_focused(1),
            KeyCode::Char('k') | KeyCode::Up => self.move_focused(-1),
            KeyCode::Char('/') => self
                .focus
                .focus(Some(Target::Command(CommandTarget::Input))),
            KeyCode::Tab => self.focus.next(&self.previous_frame.focus),
            KeyCode::BackTab => self.focus.previous(&self.previous_frame.focus),
            KeyCode::Char(ch) => self.handle_character(ch),
            KeyCode::Backspace => self.handle_backspace(),
            _ => {}
        }
        true
    }

    /// Routes pointer events through the frame-local data produced by the previous render.
    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        let position = (mouse.column, mouse.row);
        match mouse.kind {
            MouseEventKind::Down(_) => self.click(position),
            MouseEventKind::ScrollDown => self.scroll(position, 1),
            MouseEventKind::ScrollUp => self.scroll(position, -1),
            _ => {}
        }
    }

    fn click(&mut self, position: (u16, u16)) {
        let Some(hit) = self.previous_frame.route_click(position) else {
            return;
        };
        self.focus.focus(Some(hit.id));
        if let Target::Project(ProjectTarget::Row(index)) = hit.id {
            self.project.select(index);
            self.details.reset();
        }
    }

    fn scroll(&mut self, position: (u16, u16), delta: isize) {
        if matches!(
            self.previous_frame.route_scroll(position).map(|hit| hit.id),
            Some(Target::Details(DetailsTarget::Pane))
        ) {
            self.details.scroll(self.project.selected_name(), delta);
        }
    }

    fn handle_character(&mut self, ch: char) {
        if self.focus.focused() == Some(Target::Command(CommandTarget::Input)) {
            self.command.push(ch);
        }
    }

    fn handle_backspace(&mut self) {
        if self.focus.focused() == Some(Target::Command(CommandTarget::Input)) {
            self.command.pop();
        }
    }

    fn move_focused(&mut self, delta: isize) {
        match self.focus.focused() {
            Some(Target::Project(_)) => {
                self.project.select_relative(delta);
                self.details.reset();
            }
            Some(Target::Details(DetailsTarget::Pane)) => {
                self.details.scroll(self.project.selected_name(), delta);
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PageAreas {
    project: Rect,
    details: Rect,
    command: Rect,
}

impl PageAreas {
    fn new(area: Rect) -> Self {
        let [body, command] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(area);
        let [project, details] =
            Layout::horizontal([Constraint::Length(24), Constraint::Fill(1)]).areas(body);
        Self {
            project,
            details,
            command,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Target {
    Project(ProjectTarget),
    Details(DetailsTarget),
    Command(CommandTarget),
}

impl Target {
    const fn project(self) -> Option<ProjectTarget> {
        match self {
            Self::Project(target) => Some(target),
            Self::Details(_) | Self::Command(_) => None,
        }
    }

    const fn details(self) -> Option<DetailsTarget> {
        match self {
            Self::Details(target) => Some(target),
            Self::Project(_) | Self::Command(_) => None,
        }
    }

    const fn command(self) -> Option<CommandTarget> {
        match self {
            Self::Command(target) => Some(target),
            Self::Project(_) | Self::Details(_) => None,
        }
    }
}

#[derive(Debug, Default)]
struct ProjectPane {
    selected: usize,
}

impl ProjectPane {
    /// Renders project rows and returns focus/mouse/region data in project-local coordinates.
    fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        focused: Option<ProjectTarget>,
    ) -> FrameSnapshot<ProjectTarget> {
        let local = local_area(area);
        let rows = Self::row_slots(local);

        frame.render_widget(Block::new().borders(Borders::ALL).title("projects"), area);
        for region in &rows {
            let screen = to_screen(area, region.area);
            let style = self.row_style(region.id, focused);
            frame.render_widget(
                Paragraph::new(PROJECTS[region.id.index()]).style(style),
                screen,
            );
        }

        FrameTargets::from_regions(Regions::from_regions(local, rows), 0).build()
    }

    fn row_slots(area: Rect) -> Vec<Region<ProjectTarget>> {
        let inner = inner(area);
        PROJECTS
            .iter()
            .enumerate()
            .take(usize::from(inner.height))
            .map(|(index, _)| {
                let row = Rect::new(
                    inner.x,
                    inner.y.saturating_add(index as u16),
                    inner.width,
                    1,
                );
                Region::new(ProjectTarget::Row(index), row)
            })
            .collect()
    }

    fn row_style(&self, target: ProjectTarget, focused: Option<ProjectTarget>) -> Style {
        let mut style = Style::new();
        if self.selected == target.index() {
            style = style.add_modifier(Modifier::REVERSED);
        }
        if focused == Some(target) {
            style = style.add_modifier(Modifier::BOLD);
        }
        style
    }

    const fn selected_name(&self) -> &'static str {
        PROJECTS[self.selected]
    }

    fn select(&mut self, index: usize) {
        self.selected = index.min(PROJECTS.len().saturating_sub(1));
    }

    fn select_relative(&mut self, delta: isize) {
        self.selected = offset_index(self.selected, delta, PROJECTS.len());
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum ProjectTarget {
    Row(usize),
}

impl ProjectTarget {
    const fn index(self) -> usize {
        match self {
            Self::Row(index) => index,
        }
    }
}

#[derive(Debug, Default)]
struct DetailsPane {
    scroll: usize,
}

impl DetailsPane {
    /// Renders a scrollable pane and returns one local target for focus and wheel routing.
    fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        project: &str,
        focused: Option<DetailsTarget>,
    ) -> FrameSnapshot<DetailsTarget> {
        let local = local_area(area);
        let inner = inner(local);
        let block_style = if focused == Some(DetailsTarget::Pane) {
            Style::new().add_modifier(Modifier::BOLD)
        } else {
            Style::new()
        };

        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .title("details")
                .style(block_style),
            area,
        );
        let text = details_text(project);
        let visible = text
            .lines()
            .skip(self.scroll)
            .take(usize::from(inner.height))
            .collect::<Vec<_>>()
            .join("\n");
        frame.render_widget(Paragraph::new(visible), to_screen(area, inner));

        FrameTargets::new(local, 50).region(DetailsTarget::Pane, inner)
    }

    fn scroll(&mut self, project: &str, delta: isize) {
        self.scroll = offset_index(self.scroll, delta, details_text(project).lines().count());
    }

    const fn reset(&mut self) {
        self.scroll = 0;
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum DetailsTarget {
    Pane,
}

#[derive(Debug, Default)]
struct CommandInput {
    value: String,
}

impl CommandInput {
    /// Renders an input row and returns a local cursor request when it owns focus.
    fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        focused: Option<CommandTarget>,
    ) -> FrameSnapshot<CommandTarget> {
        let local = local_area(area);
        let input = inner(local);
        let style = if focused == Some(CommandTarget::Input) {
            Style::new().add_modifier(Modifier::BOLD)
        } else {
            Style::new()
        };
        let text = format!("> {}", self.value);

        frame.render_widget(Block::new().borders(Borders::ALL).title("command"), area);
        frame.render_widget(Paragraph::new(text).style(style), to_screen(area, input));

        let mut plan = FrameTargets::new(local, 90).region(CommandTarget::Input, input);
        if focused == Some(CommandTarget::Input) {
            let x = input
                .x
                .saturating_add(2)
                .saturating_add(self.value.chars().count() as u16)
                .min(input.right().saturating_sub(1));
            let cursor =
                CursorRequests::new().request(CursorRequest::visible(Position::new(x, input.y)));
            plan = plan.cursor(cursor);
        }
        plan
    }

    fn push(&mut self, ch: char) {
        self.value.push(ch);
    }

    fn pop(&mut self) {
        self.value.pop();
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum CommandTarget {
    Input,
}

const fn local_area(area: Rect) -> Rect {
    Rect::new(0, 0, area.width, area.height)
}

const fn inner(area: Rect) -> Rect {
    Rect::new(
        area.x.saturating_add(1),
        area.y.saturating_add(1),
        area.width.saturating_sub(2),
        area.height.saturating_sub(2),
    )
}

const fn to_screen(origin: Rect, local: Rect) -> Rect {
    Rect::new(
        origin.x.saturating_add(local.x),
        origin.y.saturating_add(local.y),
        local.width,
        local.height,
    )
}

fn offset_index(index: usize, delta: isize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    if delta.is_negative() {
        index.saturating_sub(delta.unsigned_abs())
    } else {
        index.saturating_add(delta as usize).min(len - 1)
    }
}

fn details_text(project: &str) -> String {
    format!(
        "{project}\n\nEach pane in this example returns local frame-local data.\n\n\
         The parent maps local ids into Target, translates local coordinates into screen \
         coordinates, clips them to the pane, and merges them into one previous frame.\n\n\
         Try tab, click, mouse wheel over this pane, / to focus the command input, and q to quit."
    )
}

const PROJECTS: [&str; 5] = ["api", "worker", "docs", "release", "ops"];
