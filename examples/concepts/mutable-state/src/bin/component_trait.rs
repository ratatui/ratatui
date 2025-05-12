//! This example shows how to mutate state while rendering by using a custom trait instead of the
//! `Widget` trait. You might choose this approach if there's some behavior that you want to
//! implement on multiple widgets, but you don't want to implement the `Widget` trait for each of
//! them.
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::Rect;
use ratatui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    let mut counter = CounterWidget::default();
    loop {
        terminal.draw(|frame| render(frame, &mut counter))?;
        if esc_pressed()? {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, counter: &mut CounterWidget) {
    counter.render(frame, frame.area());
}

#[derive(Default, Clone)]
struct CounterWidget {
    count: usize,
}

trait Component {
    fn render(&mut self, area: &mut Frame, area: Rect);
}

impl Component for CounterWidget {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.count += 1;
        let text = format!("Counter: {}", self.count);
        frame.render_widget(text, area);
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
