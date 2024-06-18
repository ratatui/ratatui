//! # [Ratatui] Hello World example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

use color_eyre::{eyre::WrapErr, Result};
use crossterm::event::KeyEventKind;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
};

/// This is a bare minimum example. There are many approaches to running an application loop, so
/// this is not meant to be prescriptive. It is only meant to demonstrate the basic setup and
/// teardown of a terminal application.
///
/// A more robust application would probably want to handle errors more thouroughly. It also does
/// not handle events or update any application state. It just draws a greeting and exits when the
/// user presses 'q'.
fn main() -> Result<()> {
    let mut terminal = CrosstermBackend::stdout_with_defaults()?
        .to_terminal()
        .wrap_err("failed to start terminal")?;

    // Run the application loop. This is where you would handle events and update the application
    // state. This example exits when the user presses 'q'. Other styles of application loops are
    // possible, for example, you could have multiple application states and switch between them
    // based on events, or you could have a single application state and update it based on events.
    loop {
        terminal
            .draw(|frame| {
                // Render the application. This is where you would draw the application UI. This
                // example just draws a greeting.
                let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
                frame.render_widget(greeting, frame.size());
            })
            .wrap_err("failed to draw")?;
        // Check if the user has pressed 'q'. This is where you would handle events. This example
        // just checks if the user has pressed 'q' and returns true if they have. It does not
        // handle any other events. This is a very basic event loop. A more robust application
        // would probably want to handle more events and consider not blocking the thread when
        // there are no events (by using [`crossterm::event::poll`])
        if let Event::Key(key) = event::read().wrap_err("failed to read events")? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
