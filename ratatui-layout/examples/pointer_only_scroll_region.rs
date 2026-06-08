//! Route wheel input with `PointerTargets` instead of a full `FrameSnapshot`.
//!
//! A scrollable pane often needs only one previous-frame value: which pane was under the pointer
//! when the wheel event arrived. This example stores a `PointerTargets` with one whole-region
//! target per pane. The app owns scroll offsets, and `ScrollMetrics` handles the render-time range
//! math.

use std::io;

use color_eyre::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind};
use crossterm::execute;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::{PointerTargets, ScrollMetrics};

fn main() -> Result<()> {
    color_eyre::install()?;

    execute!(io::stdout(), EnableMouseCapture)?;
    let result = run();
    execute!(io::stdout(), DisableMouseCapture)?;
    result
}

/// Runs the terminal event loop.
///
/// Rendering rebuilds the pointer target collection from the current pane rectangles. Mouse-wheel
/// events then use that stored plan to choose which persistent offset should change.
fn run() -> Result<()> {
    let mut app = App::default();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| app.render(frame))?;
            match event::read()? {
                event::Event::Key(key) if key.is_press() && !App::should_continue(key.code) => {
                    break Ok(());
                }
                event::Event::Mouse(mouse) => app.handle_mouse(mouse),
                _ => {}
            }
        }
    })
}

/// Persistent app state for two independently scrollable panes.
#[derive(Debug)]
struct App {
    /// Scroll offset for the queue pane.
    queue_offset: u32,
    /// Scroll offset for the log pane.
    log_offset: u32,
    /// Mouse data produced by the previous render.
    previous_mouse: PointerTargets<Pane>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            queue_offset: 0,
            log_offset: 0,
            previous_mouse: PointerTargets::new(),
        }
    }
}

impl App {
    /// Renders panes and stores whole-pane mouse targets for the next wheel event.
    fn render(&mut self, frame: &mut Frame) {
        let pane_constraints = [Constraint::Percentage(50), Constraint::Percentage(50)];
        let panes = Layout::horizontal(pane_constraints)
            .spacing(1)
            .split(frame.area());

        self.render_pane(frame, panes[0], Pane::Queue);
        self.render_pane(frame, panes[1], Pane::Log);
        self.previous_mouse = PointerTargets::new()
            .region(Pane::Queue, panes[0])
            .region(Pane::Log, panes[1]);
    }

    /// Returns whether the app should keep running for a key press.
    const fn should_continue(key: KeyCode) -> bool {
        !matches!(key, KeyCode::Char('q') | KeyCode::Esc)
    }

    /// Applies wheel movement to the pane under the pointer.
    ///
    /// The wheel changes only scroll offset. It does not move selection, focus, or any richer frame
    /// aggregate because this surface does not need that data.
    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        let delta = match mouse.kind {
            MouseEventKind::ScrollDown => 1,
            MouseEventKind::ScrollUp => -1,
            _ => return,
        };
        let position = (mouse.column, mouse.row);
        let Some(hit) = self.previous_mouse.hit_test(position) else {
            return;
        };
        self.scroll(hit.id, delta);
    }

    /// Scrolls one pane by a signed line delta.
    const fn scroll(&mut self, pane: Pane, delta: i32) {
        let offset = match pane {
            Pane::Queue => &mut self.queue_offset,
            Pane::Log => &mut self.log_offset,
        };
        *offset = offset.saturating_add_signed(delta);
    }

    /// Renders one pane from its persistent scroll offset.
    fn render_pane(&mut self, frame: &mut Frame, area: Rect, pane: Pane) {
        let offset = self.offset_mut(pane);
        let inner = inner_area(area);
        let content_height = inner.height.saturating_sub(1);
        let metrics = ScrollMetrics::new(pane.rows().len() as u32, content_height, *offset);
        *offset = metrics.offset;

        frame.render_widget(Block::new().borders(Borders::ALL).title(pane.title()), area);
        for (line, row) in pane.rows()[metrics.visible_range()].iter().enumerate() {
            let row_area = Rect::new(inner.x, inner.y + line as u16, inner.width, 1);
            frame.render_widget(Paragraph::new(*row), row_area);
        }
        let status = format!(
            "wheel scroll only    offset {} / {}",
            metrics.offset, metrics.max_offset
        );
        let status_area = Rect::new(inner.x, inner.bottom().saturating_sub(1), inner.width, 1);
        frame.render_widget(Paragraph::new(status), status_area);
    }

    /// Returns mutable scroll offset storage for a pane.
    const fn offset_mut(&mut self, pane: Pane) -> &mut u32 {
        match pane {
            Pane::Queue => &mut self.queue_offset,
            Pane::Log => &mut self.log_offset,
        }
    }
}

/// Scrollable pane ids used by the pointer target collection.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Pane {
    /// Work queue pane.
    Queue,
    /// Activity log pane.
    Log,
}

impl Pane {
    /// Returns the pane title.
    const fn title(self) -> &'static str {
        match self {
            Self::Queue => "queue",
            Self::Log => "log",
        }
    }

    /// Returns the static rows rendered by the pane.
    const fn rows(self) -> &'static [&'static str] {
        match self {
            Self::Queue => &QUEUE_ROWS,
            Self::Log => &LOG_ROWS,
        }
    }
}

/// Returns the drawable interior of a bordered pane.
const fn inner_area(area: Rect) -> Rect {
    Rect::new(
        area.x + 1,
        area.y + 1,
        area.width.saturating_sub(2),
        area.height.saturating_sub(2),
    )
}

const QUEUE_ROWS: [&str; 14] = [
    "build api",
    "build worker",
    "publish docs",
    "promote cache",
    "drain queue",
    "validate backup",
    "check migrations",
    "stage release",
    "run smoke tests",
    "notify on-call",
    "verify dashboards",
    "tag release",
    "archive bundle",
    "close window",
];

const LOG_ROWS: [&str; 16] = [
    "feed imported",
    "inputs normalized",
    "checks started",
    "cache warmed",
    "worker heartbeat",
    "doc publish queued",
    "release window open",
    "operator acknowledged",
    "traffic split updated",
    "metrics stable",
    "rollback plan ready",
    "handoff complete",
    "release tagged",
    "artifact mirrored",
    "status broadcast",
    "cleanup complete",
];
