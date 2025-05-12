//! This example shows how to mutate state while rendering by storing it as a mutable reference in
//! the widget struct. This works, but requires a good understanding of how the borrow checker and
//! lifetimes work in Rust.
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use ratatui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    let mut count = 0;
    loop {
        let counter = CounterWidget { count: &mut count };
        terminal.draw(|frame| render(frame, counter))?;
        if esc_pressed()? {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, counter: CounterWidget) {
    frame.render_widget(counter, frame.area());
}

struct CounterWidget<'a> {
    count: &'a mut usize,
}

impl Widget for CounterWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        *self.count += 1;
        let text = format!("Counter: {}", self.count);
        text.render(area, buf);
    }
}

fn esc_pressed() -> io::Result<bool> {
    Ok(matches!(
        event::read()?,
        Event::Key(KeyEvent {
            kind: KeyEventKind::Press,
            code: KeyCode::Esc,
            ..
        })
    ))
}
