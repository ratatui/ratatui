//! Route wheel events to the pane under the pointer, including blank space.
//!
//! Row hit testing only works when the pointer is over a row. Scrollable panes usually need the
//! whole pane to be a wheel target, so this example stores one mouse region per pane in
//! `FrameSnapshot`.

use std::io;

use color_eyre::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind};
use crossterm::execute;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::{FrameSnapshot, FrameTargets, ScrollMetrics};

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
                event::Event::Key(key) if key.is_press() && !App::should_continue(key.code) => {
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
    left_offset: u32,
    right_offset: u32,
    previous_frame: FrameSnapshot<Pane>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            left_offset: 0,
            right_offset: 0,
            previous_frame: FrameSnapshot::new(Rect::default()),
        }
    }
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let panes = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .spacing(1)
            .split(frame.area());
        let left = render_pane(frame, panes[0], Pane::Builds, &mut self.left_offset);
        let right = render_pane(frame, panes[1], Pane::Logs, &mut self.right_offset);
        self.previous_frame = FrameSnapshot::new(frame.area()).merge(left).merge(right);
    }

    const fn should_continue(key: KeyCode) -> bool {
        !matches!(key, KeyCode::Char('q') | KeyCode::Esc)
    }

    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        let delta = match mouse.kind {
            MouseEventKind::ScrollDown => 1,
            MouseEventKind::ScrollUp => -1,
            _ => return,
        };
        let Some(hit) = self.previous_frame.route_scroll((mouse.column, mouse.row)) else {
            return;
        };
        self.scroll(hit.id, delta);
    }

    const fn scroll(&mut self, pane: Pane, delta: i32) {
        let offset = match pane {
            Pane::Builds => &mut self.left_offset,
            Pane::Logs => &mut self.right_offset,
        };
        *offset = offset.saturating_add_signed(delta);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Pane {
    Builds,
    Logs,
}

impl Pane {
    const fn title(self) -> &'static str {
        match self {
            Self::Builds => "builds",
            Self::Logs => "logs",
        }
    }

    const fn rows(self) -> &'static [&'static str] {
        match self {
            Self::Builds => &BUILD_ROWS,
            Self::Logs => &LOG_ROWS,
        }
    }
}

fn render_pane(frame: &mut Frame, area: Rect, pane: Pane, offset: &mut u32) -> FrameSnapshot<Pane> {
    let inner = Rect::new(
        area.x + 1,
        area.y + 1,
        area.width.saturating_sub(2),
        area.height.saturating_sub(2),
    );
    let content_height = inner.height.saturating_sub(1);
    let metrics = ScrollMetrics::new(pane.rows().len() as u32, content_height, *offset);
    *offset = metrics.offset;

    frame.render_widget(Block::new().borders(Borders::ALL).title(pane.title()), area);
    for (line, row) in pane.rows()[metrics.visible_range()].iter().enumerate() {
        let row_area = Rect::new(inner.x, inner.y + line as u16, inner.width, 1);
        frame.render_widget(Paragraph::new(*row), row_area);
    }
    let status = format!("offset {} / {}", metrics.offset, metrics.max_offset);
    frame.render_widget(
        Paragraph::new(status),
        Rect::new(inner.x, inner.bottom().saturating_sub(1), inner.width, 1),
    );

    FrameTargets::new(area, 0)
        .mouse_region(pane, area)
        .region(pane, area)
}

const BUILD_ROWS: [&str; 10] = [
    "queued: api",
    "running: worker",
    "running: docs",
    "blocked: gateway",
    "done: cli",
    "done: parser",
    "queued: website",
    "queued: fixtures",
    "done: release notes",
    "blocked: deploy",
];

const LOG_ROWS: [&str; 12] = [
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
];
