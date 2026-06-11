//! Clamp a requested scroll offset and expose the visible content rectangle.
//!
//! A normal layout split gives a screen rectangle. `Viewport` adds content-space state: the
//! requested offset is clamped to the content size and returned with the visible content rectangle.

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Rect, Size};
use ratatui::widgets::Paragraph;
use ratatui_layout::viewport::{Viewport, ViewportState};

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

#[derive(Default)]
struct App {
    viewport: ViewportState,
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let content_size = Size::new(120, 80);
        let viewport = Viewport::new(content_size).layout(frame.area(), &mut self.viewport);
        let visible = viewport.visible_content;
        let mut text = format!(
            "requested offset is clamped in ViewportState\n\
             offset: {}\n\
             visible content rect: {}\n\n",
            self.viewport.offset, visible,
        );

        for y in visible.y..visible.bottom().min(visible.y.saturating_add(10)) {
            let row = format!("content row {y:02}, visible from x {}\n", visible.x);
            text.push_str(&row);
        }

        frame.render_widget(
            Paragraph::new(text),
            Rect::new(0, 0, frame.area().width, 14),
        );
    }

    const fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('j') | KeyCode::Down => self.scroll_down(),
            KeyCode::Char('k') | KeyCode::Up => self.scroll_up(),
            KeyCode::Char('l') | KeyCode::Right => self.scroll_right(),
            KeyCode::Char('h') | KeyCode::Left => self.scroll_left(),
            KeyCode::Char('q') | KeyCode::Esc => return Self::quit(),
            _ => {}
        }
        true
    }

    const fn scroll_down(&mut self) {
        self.viewport.offset.y = self.viewport.offset.y.saturating_add(1);
    }

    const fn scroll_up(&mut self) {
        self.viewport.offset.y = self.viewport.offset.y.saturating_sub(1);
    }

    const fn scroll_right(&mut self) {
        self.viewport.offset.x = self.viewport.offset.x.saturating_add(4);
    }

    const fn scroll_left(&mut self) {
        self.viewport.offset.x = self.viewport.offset.x.saturating_sub(4);
    }

    const fn quit() -> bool {
        false
    }
}
