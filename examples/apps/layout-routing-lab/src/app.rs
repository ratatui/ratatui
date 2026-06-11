//! Top-level routing lab app.

use crossterm::event::{KeyCode, MouseButton, MouseEvent, MouseEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::focus::{FocusFallback, FocusState, FocusTargets};

use crate::details::{DetailsAction, DetailsPane};
use crate::help::HelpModal;
use crate::model::{Task, TaskId, sample_tasks};
use crate::queue::{QueueAction, QueuePane};
use crate::route::{FocusScope, PointerCapture, RouteMap, RoutePath, Target};

/// App state for the nested routing lab.
#[derive(Debug)]
pub struct App {
    tasks: Vec<Task>,
    selected: usize,
    details: DetailsPane,
    help_open: bool,
    focus: FocusState<Target>,
    previous_route: RouteMap,
    previous_focus: FocusTargets<Target>,
    queue_scope: FocusScope,
    details_scope: FocusScope,
    help_scope: FocusScope,
    capture: PointerCapture,
    left_width: u16,
    last_route: String,
    last_action: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tasks: sample_tasks(),
            selected: 0,
            details: DetailsPane::default(),
            help_open: false,
            focus: FocusState::default(),
            previous_route: RouteMap::new(),
            previous_focus: FocusTargets::new(),
            queue_scope: FocusScope::new(Target::QueuePane, FocusTargets::new(), false),
            details_scope: FocusScope::new(Target::DetailsForm, FocusTargets::new(), false),
            help_scope: FocusScope::new(Target::HelpDialog, FocusTargets::new(), true),
            capture: PointerCapture::default(),
            left_width: 42,
            last_route: "no route yet".into(),
            last_action: "press ? for help, F6 to move between panes, q to quit".into(),
        }
    }
}

impl App {
    /// Renders the app and stores previous-frame-local data for the next event.
    pub fn render(&mut self, frame: &mut Frame) {
        let layout = AppLayout::new(frame.area(), self.left_width);
        self.left_width = layout.left_width();
        let selected_task = self.selected_task().clone();
        self.details.sync_task(self.selected, &selected_task);

        let mut route = RouteMap::new()
            .target(Target::Page, frame.area(), 0)
            .target(Target::Splitter, layout.splitter, 5)
            .target(Target::Diagnostics, layout.diagnostics, 0);
        let queue = QueuePane::render(
            frame,
            layout.queue,
            &self.tasks,
            self.selected,
            self.focus.focused(),
        );
        let details =
            self.details
                .render(frame, layout.details, &selected_task, self.focus.focused());
        route.extend(queue.route);
        route.extend(details.route);
        self.queue_scope = queue.scope;
        self.details_scope = details.scope;
        self.previous_focus = self
            .queue_scope
            .targets
            .clone()
            .merge(self.details_scope.targets.clone());

        self.render_splitter(frame, layout.splitter);
        self.render_diagnostics(frame, layout.diagnostics);

        if self.help_open {
            let help = HelpModal::render(frame, frame.area());
            route.extend(help.route);
            self.help_scope = help.scope;
            self.help_scope.ensure_visible(&mut self.focus);
            self.previous_focus = self.help_scope.targets.clone();
        } else {
            self.focus
                .ensure_visible(&self.previous_focus, FocusFallback::First);
            if let Some(cursor) = details.cursor {
                frame.set_cursor_position(cursor);
            }
        }

        self.previous_route = route;
    }

    /// Handles a key event and returns whether the app should keep running.
    pub fn handle_key(&mut self, key: KeyCode) -> bool {
        if self.help_open {
            return self.handle_help_key(key);
        }
        self.record_focus_route("key");
        let selected = self.selected_task().clone();
        match self
            .details
            .handle_key(key, &mut self.focus, &self.details_scope, &selected)
        {
            DetailsAction::Unhandled => {}
            action => {
                self.apply_details_action(action);
                return true;
            }
        }
        match QueuePane::handle_key(
            key,
            &mut self.focus,
            &self.queue_scope,
            &self.tasks,
            self.selected,
        ) {
            QueueAction::Unhandled => self.handle_page_key(key),
            action => {
                self.apply_queue_action(action);
                true
            }
        }
    }

    /// Handles a mouse event through capture or the previous route map.
    pub fn handle_mouse(&mut self, mouse: MouseEvent) {
        if self.handle_captured_mouse(mouse) {
            return;
        }
        let position = (mouse.column, mouse.row);
        let Some(path) = self.previous_route.hit_path(position) else {
            return;
        };
        self.last_route = format!("mouse {path}");
        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => self.mouse_down(&path, mouse.column),
            MouseEventKind::Up(MouseButton::Left) => self.mouse_up(&path),
            _ => {}
        }
    }

    fn handle_help_key(&mut self, key: KeyCode) -> bool {
        if HelpModal::handle_key(key, &mut self.focus) {
            self.help_open = false;
            self.last_action = "modal handled close key".into();
            true
        } else {
            true
        }
    }

    fn handle_page_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => false,
            KeyCode::Char('?') => {
                self.help_open = true;
                self.focus.focus(Some(Target::HelpClose));
                self.last_action = "page opened modal help".into();
                true
            }
            KeyCode::F(6) => {
                self.toggle_pane_focus();
                true
            }
            _ => {
                self.last_action = "page did not handle key".into();
                true
            }
        }
    }

    fn mouse_down(&mut self, path: &RoutePath, column: u16) {
        let Some(leaf) = path.leaf() else {
            return;
        };
        match leaf {
            Target::Splitter => {
                self.capture.capture(Target::Splitter);
                self.resize_split(column);
                self.last_action = "splitter captured pointer".into();
            }
            Target::QueueRun(id) => self.run_task(id, "inline run button handled click"),
            Target::QueueRow(id) => self.select_task(id, Some(Target::QueueRow(id))),
            Target::Field(field) => {
                self.focus.focus(Some(Target::Field(field)));
                self.last_action = "field leaf took keyboard focus".into();
            }
            Target::FormCommand(command) => {
                self.focus.focus(Some(Target::FormCommand(command)));
                self.last_action = format!("form command {} focused", command.label());
            }
            Target::HelpBackdrop | Target::HelpClose if self.help_open => {
                self.help_open = false;
                self.last_action = "modal route handled outside/close click".into();
            }
            _ => {
                self.last_action = format!("leaf {} had no activation", leaf.label());
            }
        }
    }

    fn mouse_up(&mut self, path: &RoutePath) {
        if self.help_open && path.contains(Target::HelpBackdrop) {
            self.help_open = false;
            self.last_action = "modal backdrop handled release".into();
        }
    }

    fn handle_captured_mouse(&mut self, mouse: MouseEvent) -> bool {
        if self.capture.target() != Some(Target::Splitter) {
            return false;
        }
        match mouse.kind {
            MouseEventKind::Drag(MouseButton::Left) | MouseEventKind::Down(MouseButton::Left) => {
                self.resize_split(mouse.column);
                self.last_route = "capture splitter -> page".into();
                self.last_action = "pointer capture resized panes".into();
                true
            }
            MouseEventKind::Up(MouseButton::Left) => {
                self.capture.release();
                self.last_route = "capture splitter -> page".into();
                self.last_action = "pointer capture released".into();
                true
            }
            _ => false,
        }
    }

    fn apply_details_action(&mut self, action: DetailsAction) {
        match action {
            DetailsAction::Handled(message) => self.last_action = message,
            DetailsAction::Save { title, owner } => {
                let task = &mut self.tasks[self.selected];
                task.title = title;
                task.owner = owner;
                self.details.reset_from(task);
                self.last_action = "details form saved selected task".into();
            }
            DetailsAction::Unhandled => {}
        }
    }

    fn apply_queue_action(&mut self, action: QueueAction) {
        match action {
            QueueAction::Handled(message) => self.last_action = message.into(),
            QueueAction::Select(index) => {
                self.selected = index.min(self.tasks.len().saturating_sub(1));
                let task = self.tasks[self.selected].clone();
                self.details.sync_task(self.selected, &task);
                self.last_action = format!("queue selected row {}", self.selected);
            }
            QueueAction::Run(id) => self.run_task(id, "queue inline action ran task"),
            QueueAction::Unhandled => {}
        }
    }

    fn record_focus_route(&mut self, prefix: &str) {
        if let Some(target) = self.focus.focused() {
            self.last_route = format!("{prefix} {}", RoutePath::from_leaf(target));
        }
    }

    fn toggle_pane_focus(&mut self) {
        if self.queue_scope.owns(self.focus.focused()) {
            self.details_scope.next(&mut self.focus);
            self.last_action = "page moved focus to details scope".into();
        } else {
            let target = self
                .tasks
                .get(self.selected)
                .map(|task| Target::QueueRow(task.id));
            self.focus.focus(target);
            self.last_action = "page moved focus to queue scope".into();
        }
    }

    fn select_task(&mut self, id: TaskId, focus: Option<Target>) {
        if let Some(index) = self.tasks.iter().position(|task| task.id == id) {
            self.selected = index;
            self.details.sync_task(index, &self.tasks[index]);
            self.focus.focus(focus);
            self.last_action = format!("row {} selected task", id.0);
        }
    }

    fn run_task(&mut self, id: TaskId, message: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.run();
            self.last_action = format!("{message} #{:02}", id.0);
        }
    }

    fn resize_split(&mut self, column: u16) {
        self.left_width = column.saturating_sub(1).clamp(24, 72);
    }

    fn selected_task(&self) -> &Task {
        &self.tasks[self.selected]
    }

    fn render_splitter(&self, frame: &mut Frame, area: Rect) {
        let style = if self.capture.is_active() {
            Style::new()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::new().fg(Color::DarkGray)
        };
        for y in area.y..area.bottom() {
            frame.render_widget(Paragraph::new("|").style(style), Rect::new(area.x, y, 1, 1));
        }
    }

    fn render_diagnostics(&self, frame: &mut Frame, area: Rect) {
        let text = format!(
            "{}\n{}\nfocus: {:?}    capture: {:?}\nF6 pane focus    ? help    drag splitter    q quit",
            self.last_route,
            self.last_action,
            self.focus.focused(),
            self.capture.target()
        );
        frame.render_widget(
            Paragraph::new(text).block(Block::new().borders(Borders::ALL).title("route log")),
            area,
        );
    }
}

/// Solved page areas for the app.
#[derive(Debug, Clone, Copy)]
struct AppLayout {
    queue: Rect,
    splitter: Rect,
    details: Rect,
    diagnostics: Rect,
}

impl AppLayout {
    fn new(area: Rect, left_width: u16) -> Self {
        let [body, diagnostics] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(5)]).areas(area);
        let width = left_width.clamp(24, body.width.saturating_sub(24).max(24));
        let columns = [
            Constraint::Length(width),
            Constraint::Length(1),
            Constraint::Fill(1),
        ];
        let [queue, splitter, details] = Layout::horizontal(columns).areas(body);
        Self {
            queue,
            splitter,
            details,
            diagnostics,
        }
    }

    const fn left_width(self) -> u16 {
        self.queue.width
    }
}
