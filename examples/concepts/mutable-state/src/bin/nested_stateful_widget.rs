//! This example shows how to mutate state while rendering, by storing a widget inside another
//! mutable widget both implemented using the `StatefulWidget` trait. This is a simple approach
//! that avoids borrow checker issues by storing the state outside the widget.
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{StatefulWidget, Widget};
use ratatui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::run(terminal);
    ratatui::restore();
    result
}

struct App;

impl App {
    fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        let mut state = AppState { counter: 0 };
        loop {
            terminal.draw(|frame| Self::render(frame, &mut state))?;
            if esc_pressed()? {
                break Ok(());
            }
        }
    }

    fn render(frame: &mut Frame, state: &mut AppState) {
        frame.render_stateful_widget(App, frame.area(), state);
    }
}

struct AppState {
    counter: usize,
}

impl StatefulWidget for App {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        CounterWidget.render(area, buf, &mut state.counter);
    }
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
