//! Route wheel input to a pane even when the pointer is over blank space.
//!
//! `ScrollablePane` computes simple fixed-row scroll metrics and creates one pointer target for the
//! whole pane. That target lets the app scroll the viewport without changing selection.

use std::io;

use color_eyre::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind};
use crossterm::execute;
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::{FrameSnapshot, ScrollablePane};

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
    offset: usize,
    previous_frame: FrameSnapshot<Pane>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            offset: 0,
            previous_frame: FrameSnapshot::new(Rect::default()),
        }
    }
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let area = centered(frame.area(), 54, 12);
        let inner = area.inner(ratatui::layout::Margin {
            horizontal: 1,
            vertical: 1,
        });
        let layout = ScrollablePane::new(Pane::Log).status_line(true).layout(
            inner,
            LOG_LINES.len(),
            self.offset,
        );
        self.offset = layout.metrics().offset as usize;

        frame.render_widget(
            Block::new().borders(Borders::ALL).title("scrollable pane"),
            area,
        );
        let visible = &LOG_LINES[layout.metrics().visible_range()];
        frame.render_widget(Paragraph::new(visible.join("\n")), layout.content_area());
        if let Some(status_area) = layout.status_area() {
            let status = format!(
                "rows {}..{} / {}",
                layout.metrics().visible_range().start,
                layout.metrics().visible_range().end,
                LOG_LINES.len()
            );
            frame.render_widget(Paragraph::new(status), status_area);
        }
        self.previous_frame = layout.frame().clone();
    }

    const fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Char('j') | KeyCode::Down => self.scroll_by(1),
            KeyCode::Char('k') | KeyCode::Up => self.scroll_by(-1),
            _ => {}
        }
        true
    }

    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        let Some(hit) = self.previous_frame.route_scroll((mouse.column, mouse.row)) else {
            return;
        };
        if hit.id != Pane::Log {
            return;
        }
        match mouse.kind {
            MouseEventKind::ScrollDown => self.scroll_by(1),
            MouseEventKind::ScrollUp => self.scroll_by(-1),
            _ => {}
        }
    }

    const fn scroll_by(&mut self, delta: isize) {
        self.offset = self.offset.saturating_add_signed(delta);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Pane {
    Log,
}

const LOG_LINES: [&str; 18] = [
    "feed imported",
    "workspace inputs normalized",
    "schema check started",
    "cache warmup requested",
    "worker drain rehearsed",
    "manual approval pending",
    "release window opened",
    "traffic ramp at 10%",
    "traffic ramp at 25%",
    "traffic ramp at 50%",
    "traffic ramp at 75%",
    "traffic ramp complete",
    "docs published",
    "operator guide linked",
    "post-release checks queued",
    "metrics stable",
    "archive generated",
    "release closed",
];

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let [vertical] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    let [horizontal] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(vertical);
    horizontal
}
