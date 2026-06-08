//! Keep durable record selection while a virtual list filters, clicks, and scrolls.
//!
//! `VirtualListState` selects by visible source index because that is what row rendering needs.
//! `VisibleSelection` stores the selected task id because that is what commands and details panes
//! need after filtering or sorting changes the visible rows.

use std::io;

use color_eyre::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind};
use crossterm::execute;
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use ratatui_layout::list::{ListItemContext, ListItems, ListLayout, VirtualList, VirtualListState};
use ratatui_layout::{MeasureContext, VisibleSelection};

fn main() -> Result<()> {
    color_eyre::install()?;

    execute!(io::stdout(), EnableMouseCapture)?;
    let result = run();
    execute!(io::stdout(), DisableMouseCapture)?;
    result
}

fn run() -> Result<()> {
    let mut app = App::new();
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
    filter: TaskFilter,
    selection: VisibleSelection<TaskId>,
    list_state: VirtualListState,
    previous_layout: Option<ListLayout>,
    reveal_selection: bool,
}

impl App {
    fn new() -> Self {
        let mut app = Self {
            filter: TaskFilter::All,
            selection: VisibleSelection::new(),
            list_state: VirtualListState::default(),
            previous_layout: None,
            reveal_selection: true,
        };
        app.sync_selection_to_visible_tasks();
        app
    }

    fn render(&mut self, frame: &mut Frame) {
        let [list_area, details_area, status_area] = page_areas(frame.area());
        let rows = TaskRows::new(self.filter);
        self.apply_selection_to_virtual_list();

        frame.render_widget(Block::new().borders(Borders::ALL).title("tasks"), list_area);
        let inner_list_area = inner(list_area);
        let list = VirtualList::new().scroll_padding(1);
        let mut row_items = rows.clone();
        self.previous_layout = Some(list.render(
            inner_list_area,
            frame.buffer_mut(),
            &mut self.list_state,
            &mut row_items,
        ));

        self.render_details(frame, details_area);
        self.render_status(frame, status_area, &rows);
        self.reveal_selection = false;
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Char('b') => self.toggle_blocked_filter(),
            KeyCode::Char('j') | KeyCode::Down => self.move_selection(1),
            KeyCode::Char('k') | KeyCode::Up => self.move_selection(-1),
            _ => {}
        }
        true
    }

    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        let position = (mouse.column, mouse.row);
        match mouse.kind {
            MouseEventKind::ScrollDown => self.scroll_viewport(1),
            MouseEventKind::ScrollUp => self.scroll_viewport(-1),
            MouseEventKind::Down(_) => self.select_at(position),
            _ => {}
        }
    }

    fn toggle_blocked_filter(&mut self) {
        self.filter = self.filter.toggled_blocked();
        self.sync_selection_to_visible_tasks();
        self.reveal_selection = true;
    }

    fn move_selection(&mut self, delta: isize) {
        let visible_ids = self.visible_task_ids();
        self.selection.move_by(delta, &visible_ids);
        self.reveal_selection = true;
    }

    const fn scroll_viewport(&mut self, delta: isize) {
        self.list_state.scroll_viewport_by(delta);
        self.reveal_selection = false;
    }

    fn select_at(&mut self, position: (u16, u16)) {
        let Some(layout) = self.previous_layout.as_ref() else {
            return;
        };
        let visible_ids = self.visible_task_ids();
        if layout
            .select_hit(position, &mut self.selection, &visible_ids)
            .is_some()
        {
            self.reveal_selection = true;
        }
    }

    fn sync_selection_to_visible_tasks(&mut self) {
        let visible_ids = self.visible_task_ids();
        self.selection.sync_ids(&visible_ids);
    }

    const fn apply_selection_to_virtual_list(&mut self) {
        if self.reveal_selection {
            self.list_state.select(self.selection.position());
        } else {
            self.list_state
                .select_without_scrolling(self.selection.position());
        }
    }

    fn render_details(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Block::new().borders(Borders::ALL).title("selected"), area);
        let task = self.selection.selected_id().and_then(Task::find);
        let text = task.map_or_else(
            || "no task selected".to_string(),
            |task| {
                format!(
                    "{}\nstate: {}\nowner: {}\n\nCommands use this durable task id even when the \
                     visible row index changes.",
                    task.id.label(),
                    task.state.label(),
                    task.owner,
                )
            },
        );
        frame.render_widget(Paragraph::new(text), inner(area));
    }

    fn render_status(&self, frame: &mut Frame, area: Rect, rows: &TaskRows) {
        let selected = self.selection.selected_id().map_or("none", TaskId::label);
        let status = format!(
            "b filter: {}   j/k move   click select   wheel scrolls only   selected: {}   visible: {}",
            self.filter.label(),
            selected,
            rows.len(),
        );
        frame.render_widget(Paragraph::new(status), area);
    }

    fn visible_task_ids(&self) -> Vec<TaskId> {
        TaskRows::new(self.filter).ids()
    }
}

#[derive(Debug, Clone)]
struct TaskRows {
    tasks: Vec<&'static Task>,
}

impl TaskRows {
    fn new(filter: TaskFilter) -> Self {
        let tasks = TASKS.iter().filter(|task| filter.matches(task)).collect();
        Self { tasks }
    }

    fn ids(&self) -> Vec<TaskId> {
        self.tasks.iter().map(|task| task.id).collect()
    }
}

impl ListItems for TaskRows {
    fn len(&self) -> usize {
        self.tasks.len()
    }

    fn height_for_width(&self, index: usize, width: u16, _: MeasureContext) -> u16 {
        let text_width = width.saturating_sub(26).max(1) as usize;
        self.tasks[index].summary.len().div_ceil(text_width).max(1) as u16
    }

    fn render_item(&mut self, index: usize, area: Rect, buf: &mut Buffer, ctx: ListItemContext) {
        let task = self.tasks[index];
        let style = task.state.style(ctx.render.state.selected);
        let clipped = if ctx.clipped_top { "^" } else { " " };
        let text = format!(
            "{clipped}{:<7} {:<8} {:<9} {}",
            task.id.label(),
            task.state.label(),
            task.owner,
            task.summary,
        );
        Paragraph::new(text).style(style).render(area, buf);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct TaskId(u8);

impl TaskId {
    fn label(self) -> &'static str {
        Task::find(self).map_or("unknown", |task| task.key)
    }
}

#[derive(Debug)]
struct Task {
    id: TaskId,
    key: &'static str,
    summary: &'static str,
    owner: &'static str,
    state: TaskState,
}

impl Task {
    fn find(id: TaskId) -> Option<&'static Self> {
        TASKS.iter().find(|task| task.id == id)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum TaskFilter {
    All,
    Blocked,
}

impl TaskFilter {
    const fn label(self) -> &'static str {
        match self {
            Self::All => "all tasks",
            Self::Blocked => "blocked only",
        }
    }

    const fn toggled_blocked(self) -> Self {
        match self {
            Self::All => Self::Blocked,
            Self::Blocked => Self::All,
        }
    }

    const fn matches(self, task: &Task) -> bool {
        match self {
            Self::All => true,
            Self::Blocked => matches!(task.state, TaskState::Blocked),
        }
    }
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

fn page_areas(area: Rect) -> [Rect; 3] {
    let columns = [Constraint::Percentage(62), Constraint::Percentage(38)];
    let [body_area, status_area] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);
    let [list_area, details_area] = Layout::horizontal(columns).areas(body_area);
    [list_area, details_area, status_area]
}

const fn inner(area: Rect) -> Rect {
    Rect::new(
        area.x.saturating_add(1),
        area.y.saturating_add(1),
        area.width.saturating_sub(2),
        area.height.saturating_sub(2),
    )
}

const TASKS: [Task; 24] = [
    Task {
        id: TaskId(0),
        key: "API-00",
        summary: "schema compatibility sweep before the release branch closes",
        owner: "platform",
        state: TaskState::Running,
    },
    Task {
        id: TaskId(1),
        key: "WEB-01",
        summary: "edge cache config promote with rollback notes attached",
        owner: "traffic",
        state: TaskState::Blocked,
    },
    Task {
        id: TaskId(2),
        key: "WRK-02",
        summary: "queue drain rehearsal for long-running background jobs",
        owner: "runtime",
        state: TaskState::Queued,
    },
    Task {
        id: TaskId(3),
        key: "DOC-03",
        summary: "operator guide publish after screenshots are refreshed",
        owner: "docs",
        state: TaskState::Done,
    },
    Task {
        id: TaskId(4),
        key: "OPS-04",
        summary: "go/no-go review waiting on signoff from release captain",
        owner: "release",
        state: TaskState::Blocked,
    },
    Task {
        id: TaskId(5),
        key: "API-05",
        summary: "rollout metrics audit for the canary region",
        owner: "platform",
        state: TaskState::Running,
    },
    Task {
        id: TaskId(6),
        key: "DB-06",
        summary: "backup restore verification on the staging dataset",
        owner: "data",
        state: TaskState::Blocked,
    },
    Task {
        id: TaskId(7),
        key: "QA-07",
        summary: "smoke test signoff for desktop and wasm targets",
        owner: "qa",
        state: TaskState::Queued,
    },
    Task {
        id: TaskId(8),
        key: "SEC-08",
        summary: "dependency exception review for the CLI packaging job",
        owner: "security",
        state: TaskState::Blocked,
    },
    Task {
        id: TaskId(9),
        key: "REL-09",
        summary: "release notes diff against the previous public tag",
        owner: "release",
        state: TaskState::Done,
    },
    Task {
        id: TaskId(10),
        key: "API-10",
        summary: "rate-limit dashboard verification for partner traffic",
        owner: "platform",
        state: TaskState::Queued,
    },
    Task {
        id: TaskId(11),
        key: "WEB-11",
        summary: "browser preview bundle smoke test after theme asset rebuild",
        owner: "ui",
        state: TaskState::Running,
    },
    Task {
        id: TaskId(12),
        key: "WRK-12",
        summary: "worker drain timeout calibration with delayed retries",
        owner: "runtime",
        state: TaskState::Blocked,
    },
    Task {
        id: TaskId(13),
        key: "DOC-13",
        summary: "migration guide trim pass for repeated setup instructions",
        owner: "docs",
        state: TaskState::Queued,
    },
    Task {
        id: TaskId(14),
        key: "OPS-14",
        summary: "incident handoff dry run with paging policy notes",
        owner: "ops",
        state: TaskState::Done,
    },
    Task {
        id: TaskId(15),
        key: "DB-15",
        summary: "read replica lag audit before traffic moves to the new pool",
        owner: "data",
        state: TaskState::Running,
    },
    Task {
        id: TaskId(16),
        key: "SEC-16",
        summary: "signing key rotation checklist waiting on hardware token access",
        owner: "security",
        state: TaskState::Blocked,
    },
    Task {
        id: TaskId(17),
        key: "QA-17",
        summary: "mobile terminal viewport pass for narrow-width rendering",
        owner: "qa",
        state: TaskState::Queued,
    },
    Task {
        id: TaskId(18),
        key: "REL-18",
        summary: "artifact mirror verification across both package registries",
        owner: "release",
        state: TaskState::Running,
    },
    Task {
        id: TaskId(19),
        key: "WEB-19",
        summary: "settings page copy review for the launch banner",
        owner: "ui",
        state: TaskState::Done,
    },
    Task {
        id: TaskId(20),
        key: "API-20",
        summary: "customer import backfill paused on malformed legacy payloads",
        owner: "platform",
        state: TaskState::Blocked,
    },
    Task {
        id: TaskId(21),
        key: "WRK-21",
        summary: "scheduled job ownership transfer for overnight maintenance",
        owner: "runtime",
        state: TaskState::Queued,
    },
    Task {
        id: TaskId(22),
        key: "DOC-22",
        summary: "troubleshooting page publish after support review",
        owner: "docs",
        state: TaskState::Running,
    },
    Task {
        id: TaskId(23),
        key: "OPS-23",
        summary: "launch room checklist blocked on final stakeholder attendance",
        owner: "ops",
        state: TaskState::Blocked,
    },
];
