//! This example shows how to mutate state while rendering, by using methods that work with the
//! frame instead of implementing the `Widget` trait. This works well for simple cases where there's
//! not a large amount of state to manage, but it tends to mean that state gets passed around in
//! function arguments instead of having a single place to store it.
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
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

/// This approach has no widgets, just a simple state variable that is passed to the render callback
fn render(frame: &mut Frame, counter: &mut usize) {
    *counter += 1;
    let text = format!("Counter: {counter}");
    frame.render_widget(text, frame.area());
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
