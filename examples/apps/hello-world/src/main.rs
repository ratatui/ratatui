/// A Ratatui example that demonstrates a basic hello world application.
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
use std::time::Duration;

use color_eyre::eyre::Context;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::Paragraph;
use ratatui::{DefaultTerminal, Frame};

/// This is a bare minimum example. There are many approaches to running an application loop, so
/// this is not meant to be prescriptive. It is only meant to demonstrate the basic setup and
/// teardown of a terminal application.
///
/// This example does not handle events or update the application state. It just draws a greeting
/// and exits when the user presses 'q'.
fn main() -> Result<()> {
    color_eyre::install()?; // augment errors / panics with easy to read messages
    let terminal = ratatui::init();
    let app_result = run(terminal).context("app loop failed");
    ratatui::restore();
    app_result
}

/// Run the application loop. This is where you would handle events and update the application
/// state. This example exits when the user presses 'q'. Other styles of application loops are
/// possible, for example, you could have multiple application states and switch between them based
/// on events, or you could have a single application state and update it based on events.
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

/// Render the application. This is where you would draw the application UI. This example draws a
/// greeting.
fn draw(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.area());
}

/// Check if the user has pressed 'q'. This is where you would handle events. This example just
/// checks if the user has pressed 'q' and returns true if they have. It does not handle any other
/// events. There is a 250ms timeout on the event poll to ensure that the terminal is rendered at
/// least once every 250ms. This allows you to do other work in the application loop, such as
/// updating the application state, without blocking the event loop for too long.
fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
