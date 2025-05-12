//! This example shows how to mutate state while rendering by storing a widget inside another
//! mutable widget both implemented using the `Widget` trait on a `&mut` reference. This is a simple
//! approach suitable for widgets that are arranged in a parent-child relationship.
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use ratatui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::default().run(terminal);
    ratatui::restore();
    result
}

#[derive(Default)]
struct App {
    counter: CounterWidget,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            if esc_pressed()? {
                break Ok(());
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.counter.render(area, buf);
    }
}

#[derive(Default)]
struct CounterWidget {
    count: usize,
}

impl Widget for &mut CounterWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.count += 1;
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
