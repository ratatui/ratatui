//! Keep selection tied to record ids while the visible list changes.
//!
//! A normal selected row index breaks as soon as filtering hides rows above it. `VisibleSelection`
//! stores the durable record id and the current visible position together, so rendering can use the
//! visible position while commands can still act on the selected record.

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::selection::VisibleSelection;

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

#[derive(Debug)]
struct App {
    show_blocked_only: bool,
    selection: VisibleSelection<TaskId>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            show_blocked_only: false,
            selection: VisibleSelection::new(),
        }
    }
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let [list_area, status_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());
        let visible = self.visible_tasks();
        let visible_ids = task_ids(&visible);
        self.sync_selection(&visible_ids);

        frame.render_widget(Block::new().borders(Borders::ALL).title("tasks"), list_area);
        for (position, task) in visible.iter().enumerate() {
            self.render_task(frame, list_area, position, task);
        }
        self.render_status(frame, status_area, &visible);
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        let visible = self.visible_tasks();
        let visible_ids = task_ids(&visible);
        match key {
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Char('b') => self.toggle_blocked_filter(),
            KeyCode::Char('j') | KeyCode::Down => self.move_selection(1, &visible_ids),
            KeyCode::Char('k') | KeyCode::Up => self.move_selection(-1, &visible_ids),
            _ => {}
        }
        true
    }

    fn toggle_blocked_filter(&mut self) {
        self.show_blocked_only = !self.show_blocked_only;
        let visible = self.visible_tasks();
        let visible_ids = task_ids(&visible);
        self.sync_selection(&visible_ids);
    }

    fn move_selection(&mut self, delta: isize, visible_ids: &[TaskId]) {
        self.selection.move_by(delta, visible_ids);
    }

    fn sync_selection(&mut self, visible_ids: &[TaskId]) {
        self.selection.sync_ids(visible_ids);
    }

    fn visible_tasks(&self) -> Vec<&'static Task> {
        TASKS
            .iter()
            .filter(|task| !self.show_blocked_only || task.state == TaskState::Blocked)
            .collect()
    }

    fn render_task(&self, frame: &mut Frame, list_area: Rect, position: usize, task: &Task) {
        let row = list_area.y + 1 + position as u16;
        if row >= list_area.bottom().saturating_sub(1) {
            return;
        }

        let selected = self.selection.selected_id() == Some(task.id);
        let style = task.state.style(selected);
        let text = format!(
            "{:<8} {:<9} {}",
            task.id.label(),
            task.state.label(),
            task.title
        );
        let area = Rect::new(list_area.x + 1, row, list_area.width.saturating_sub(2), 1);
        frame.render_widget(Paragraph::new(text).style(style), area);
    }

    fn render_status(&self, frame: &mut Frame, area: Rect, visible: &[&Task]) {
        let filter = if self.show_blocked_only {
            "blocked only"
        } else {
            "all tasks"
        };
        let selected = self.selection.selected_id().map_or("none", |id| id.label());
        let text = format!(
            "b filter: {filter}   j/k move   selected: {selected}   visible: {}",
            visible.len(),
        );
        frame.render_widget(Paragraph::new(text), area);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct TaskId(u8);

impl TaskId {
    fn label(self) -> &'static str {
        TASKS
            .iter()
            .find(|task| task.id == self)
            .map_or("unknown", |task| task.key)
    }
}

#[derive(Debug)]
struct Task {
    id: TaskId,
    key: &'static str,
    title: &'static str,
    state: TaskState,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum TaskState {
    Queued,
    Running,
    Blocked,
    Done,
}

impl TaskState {
    const fn label(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Blocked => "blocked",
            Self::Done => "done",
        }
    }

    const fn style(self, selected: bool) -> Style {
        let style = match self {
            Self::Queued => Style::new().fg(Color::Cyan),
            Self::Running => Style::new().fg(Color::Yellow),
            Self::Blocked => Style::new().fg(Color::Red),
            Self::Done => Style::new().fg(Color::Green),
        };
        if selected {
            style.add_modifier(Modifier::REVERSED)
        } else {
            style
        }
    }
}

const TASKS: [Task; 8] = [
    Task {
        id: TaskId(0),
        key: "API-001",
        title: "schema compatibility sweep",
        state: TaskState::Running,
    },
    Task {
        id: TaskId(1),
        key: "WEB-002",
        title: "edge cache config promote",
        state: TaskState::Blocked,
    },
    Task {
        id: TaskId(2),
        key: "WRK-003",
        title: "queue drain rehearsal",
        state: TaskState::Queued,
    },
    Task {
        id: TaskId(3),
        key: "DOC-004",
        title: "operator guide publish",
        state: TaskState::Done,
    },
    Task {
        id: TaskId(4),
        key: "OPS-005",
        title: "go/no-go review",
        state: TaskState::Blocked,
    },
    Task {
        id: TaskId(5),
        key: "API-006",
        title: "rollout metrics audit",
        state: TaskState::Running,
    },
    Task {
        id: TaskId(6),
        key: "DB-007",
        title: "backup restore verification",
        state: TaskState::Blocked,
    },
    Task {
        id: TaskId(7),
        key: "QA-008",
        title: "smoke test signoff",
        state: TaskState::Queued,
    },
];

fn task_ids(tasks: &[&Task]) -> Vec<TaskId> {
    tasks.iter().map(|task| task.id).collect()
}
