//! Keep selection on durable record ids while rendering through a virtual list.
//!
//! `VirtualList` measures and renders by source index. `VirtualRecordList` adds a durable id layer
//! so clicks, keyboard movement, and app commands can keep referring to records after sorting or
//! filtering changes the visible order.

use std::io;

use color_eyre::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind};
use crossterm::execute;
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use ratatui_layout::frame::{FrameSnapshot, FrameTargets};
use ratatui_layout::list::{ListItemContext, ListItems};
use ratatui_layout::participant::MeasureContext;
use ratatui_layout::record_list::{VirtualRecordList, VirtualRecordListState};

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
    reversed: bool,
    state: VirtualRecordListState<RecordId>,
    previous_frame: FrameSnapshot<Target>,
}

impl Default for App {
    fn default() -> Self {
        let mut state = VirtualRecordListState::new();
        state.select_id(RecordId::Api, &RecordId::normal_order());
        Self {
            reversed: false,
            state,
            previous_frame: FrameSnapshot::new(Rect::default()),
        }
    }
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let ids = self.visible_ids();
        let mut rows = ReleaseRows::new(ids);
        let area = centered(frame.area(), 56, 10);
        let list_area = area.inner(ratatui::layout::Margin {
            horizontal: 1,
            vertical: 1,
        });
        let layout = VirtualRecordList::new().render(
            list_area,
            frame.buffer_mut(),
            &mut self.state,
            ids,
            &mut rows,
        );
        let row_regions = layout.regions().clone().map_id(Target::Row);

        self.previous_frame = FrameTargets::from_regions(row_regions, 0)
            .mouse_region(Target::Pane, list_area)
            .build();

        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .title("virtual record list"),
            area,
        );
        let selected = self.state.selected_id().map_or("none", RecordId::label);
        let status = format!("selected: {selected}    r reverses order");
        let status_area = Rect {
            y: area.bottom().saturating_sub(1),
            height: 1,
            ..area
        };
        frame.render_widget(Paragraph::new(status), status_area);
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        let ids = self.visible_ids();
        match key {
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Char('r') => self.reversed = !self.reversed,
            KeyCode::Char('j') | KeyCode::Down => {
                self.state.move_selection_by(1, ids);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.state.move_selection_by(-1, ids);
            }
            _ => {}
        }
        true
    }

    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        let ids = self.visible_ids();
        match mouse.kind {
            MouseEventKind::Down(_) => {
                if let Some(hit) = self.previous_frame.route_click((mouse.column, mouse.row))
                    && let Target::Row(id) = hit.id
                {
                    self.state.select_id(id, ids);
                }
            }
            MouseEventKind::ScrollDown => self.state.scroll_viewport_by(1),
            MouseEventKind::ScrollUp => self.state.scroll_viewport_by(-1),
            _ => {}
        }
    }

    const fn visible_ids(&self) -> &'static [RecordId] {
        if self.reversed {
            &RecordId::REVERSED
        } else {
            &RecordId::NORMAL
        }
    }
}

#[derive(Debug)]
struct ReleaseRows {
    ids: &'static [RecordId],
}

impl ReleaseRows {
    const fn new(ids: &'static [RecordId]) -> Self {
        Self { ids }
    }
}

impl ListItems for ReleaseRows {
    fn len(&self) -> usize {
        self.ids.len()
    }

    fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
        1
    }

    fn render_item(&mut self, index: usize, area: Rect, buf: &mut Buffer, ctx: ListItemContext) {
        let record = self.ids[index];
        let style = if ctx.render.state.selected {
            Style::new().fg(Color::Black).bg(Color::Green)
        } else {
            Style::new().fg(Color::White)
        };
        let line = format!("{:<8} {}", record.code(), record.label());
        Line::from(line).style(style).render(area, buf);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Target {
    Pane,
    Row(RecordId),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum RecordId {
    Api,
    Worker,
    Docs,
    Ops,
    Data,
    Traffic,
}

impl RecordId {
    const NORMAL: [Self; 6] = [
        Self::Api,
        Self::Worker,
        Self::Docs,
        Self::Ops,
        Self::Data,
        Self::Traffic,
    ];
    const REVERSED: [Self; 6] = [
        Self::Traffic,
        Self::Data,
        Self::Ops,
        Self::Docs,
        Self::Worker,
        Self::Api,
    ];

    const fn normal_order() -> [Self; 6] {
        Self::NORMAL
    }

    const fn code(self) -> &'static str {
        match self {
            Self::Api => "API",
            Self::Worker => "WRK",
            Self::Docs => "DOC",
            Self::Ops => "OPS",
            Self::Data => "DATA",
            Self::Traffic => "TRAF",
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Api => "schema compatibility sweep",
            Self::Worker => "queue drain rehearsal",
            Self::Docs => "operator guide publish",
            Self::Ops => "go/no-go review",
            Self::Data => "backfill safety check",
            Self::Traffic => "edge cache promote",
        }
    }
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let [vertical] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    let [horizontal] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(vertical);
    horizontal
}
