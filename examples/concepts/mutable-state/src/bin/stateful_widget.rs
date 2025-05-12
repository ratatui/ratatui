//! This example shows how to mutate state while rendering using the `StatefulWidget` trait. This
//! avoids borrow checker issues by storing the state outside the widget. This is a simple approach
//! that works well for most cases, but you might choose instead to implement `Widget` on a `&mut`
//! reference to the widget if there's not a good distinction between the widget's state and the
//! configuration.
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{StatefulWidget, Widget};
use ratatui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    let mut counter = 0;
    loop {
        terminal.draw(|frame| render(frame, &mut counter))?;
        if esc_pressed()? {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, counter: &mut usize) {
    frame.render_stateful_widget(CounterWidget, frame.area(), counter);
}

struct CounterWidget;

impl StatefulWidget for CounterWidget {
    type State = usize;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        *state += 1;
        let text = format!("Counter: {}", state);
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
