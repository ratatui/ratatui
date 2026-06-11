//! Measure and render variable-height list rows by index, without storing row widgets.
//!
//! The built-in `List` is simpler for text rows. `VirtualList` is useful when rows are external app
//! data, have variable measured heights, and should only be rendered when visible.

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Paragraph, Widget};
use ratatui_layout::list::{ListItemContext, ListItems, VirtualList, VirtualListState};
use ratatui_layout::participant::MeasureContext;

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

const ROWS: [&str; 6] = [
    "Short row.",
    "This row is longer and measures itself from the final list width.",
    "A much longer row shows why item-index scrolling is not enough for multiline terminal content.",
    "Only visible rows are rendered.",
    "The app owns row data; the list owns viewport math.",
    "Selection is passed through ListItemContext.",
];

struct App {
    state: VirtualListState,
}

impl Default for App {
    fn default() -> Self {
        let mut state = VirtualListState::default();
        state.select(Some(0));
        Self { state }
    }
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let mut rows = Rows;
        let layout = VirtualList::new().scroll_padding(1).render(
            frame.area(),
            frame.buffer_mut(),
            &mut self.state,
            &mut rows,
        );

        let status = format!(
            "visible indexes: {:?}  scroll: {:?}",
            layout
                .visible_items
                .iter()
                .map(|item| item.index)
                .collect::<Vec<_>>(),
            layout.scroll,
        );
        let status_area = Rect::new(
            0,
            frame.area().bottom().saturating_sub(1),
            frame.area().width,
            1,
        );
        frame.render_widget(Paragraph::new(status), status_area);
    }

    const fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('q') | KeyCode::Esc => return Self::quit(),
            _ => {}
        }
        true
    }

    const fn select_next(&mut self) {
        self.state.select_relative(1, ROWS.len());
    }

    const fn select_previous(&mut self) {
        self.state.select_relative(-1, ROWS.len());
    }

    const fn quit() -> bool {
        false
    }
}

struct Rows;

impl ListItems for Rows {
    fn len(&self) -> usize {
        ROWS.len()
    }

    fn height_for_width(&self, index: usize, width: u16, _: MeasureContext) -> u16 {
        let text_width = width.saturating_sub(5).max(1) as usize;
        ROWS[index].len().div_ceil(text_width).max(1) as u16
    }

    fn render_item(&mut self, index: usize, area: Rect, buf: &mut Buffer, ctx: ListItemContext) {
        let style = if ctx.render.state.selected {
            Style::new().add_modifier(Modifier::REVERSED)
        } else {
            Style::new()
        };
        let clipped = if ctx.clipped_top { "^" } else { " " };
        let text = format!("{clipped}{:02} {}", index + 1, ROWS[index]);

        Paragraph::new(text).style(style).render(area, buf);
    }
}
