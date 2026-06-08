//! Domain model for the routing lab.

/// Stable task id used by route and focus targets.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TaskId(pub u16);

/// Release task shown in the queue and edited in the details pane.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Task {
    /// Stable id.
    pub id: TaskId,
    /// Human-readable task title.
    pub title: String,
    /// Person or team currently responsible.
    pub owner: String,
    /// Current task state.
    pub state: TaskState,
}

impl Task {
    /// Creates a task from static sample data.
    pub fn new(id: u16, title: &str, owner: &str, state: TaskState) -> Self {
        Self {
            id: TaskId(id),
            title: title.into(),
            owner: owner.into(),
            state,
        }
    }

    /// Marks a task as running.
    pub const fn run(&mut self) {
        self.state = TaskState::Running;
    }
}

/// Small set of states rendered by the queue.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TaskState {
    /// Ready but not started.
    Queued,
    /// Currently running.
    Running,
    /// Waiting on another person or system.
    Blocked,
    /// Finished.
    Done,
}

impl TaskState {
    /// Returns the compact label rendered in queue rows.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Blocked => "blocked",
            Self::Done => "done",
        }
    }
}

/// Builds sample release tasks.
pub fn sample_tasks() -> Vec<Task> {
    vec![
        Task::new(
            0,
            "schema compatibility sweep",
            "platform",
            TaskState::Running,
        ),
        Task::new(
            1,
            "edge cache config promote",
            "traffic",
            TaskState::Blocked,
        ),
        Task::new(2, "queue drain rehearsal", "runtime", TaskState::Queued),
        Task::new(3, "backfill safety check", "data", TaskState::Done),
        Task::new(4, "operator guide publish", "docs", TaskState::Queued),
        Task::new(5, "go/no-go review", "release", TaskState::Blocked),
        Task::new(6, "metrics dashboard verify", "ops", TaskState::Queued),
        Task::new(7, "artifact mirror compare", "build", TaskState::Done),
    ]
}
