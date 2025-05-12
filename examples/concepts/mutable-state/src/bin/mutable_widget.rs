//! This example shows how to mutate state while rendering  by using the `Widget` trait implement on
//! the `&mut` reference to the widget. This is a simple approach that works well for most cases.
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
    let mut counter = CounterWidget { counter: 0 };
    loop {
        terminal.draw(|frame| render(frame, &mut counter))?;
        if esc_pressed()? {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, counter: &mut CounterWidget) {
    frame.render_widget(counter, frame.area());
}

struct CounterWidget {
    counter: usize,
}

impl Widget for &mut CounterWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.counter += 1;
        let text = format!("Counter: {}", self.counter);
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
