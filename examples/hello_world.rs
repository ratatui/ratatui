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

use std::{
    io::{self, Stdout},
    time::Duration,
};

use color_eyre::{eyre::Context, Result};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    widgets::Paragraph,
    Frame, Terminal,
};

/// This is a bare minimum example. There are many approaches to running an application loop, so
/// this is not meant to be prescriptive. It is only meant to demonstrate the basic setup and
/// teardown of a terminal application.
///
/// This example does not handle events or update the application state. It just draws a greeting
/// and exits when the user presses 'q'.
fn main() -> Result<()> {
    color_eyre::install()?; // augment errors / panics with easy to read messages
    let mut terminal = init_terminal().context("setup failed")?;
    let result = run(&mut terminal).context("app loop failed");
    restore_terminal();
    result
}

/// Setup the terminal. This is where you would enable raw mode, enter the alternate screen, and
/// hide the cursor. This example does not handle errors.
fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    set_panic_hook();
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

/// Restore the terminal. This is where you disable raw mode, leave the alternate screen, and show
/// the cursor.
fn restore_terminal() {
    // There's not a lot we can do if these fail, so we just print an error message.
    if let Err(err) = disable_raw_mode() {
        eprintln!("Error disabling raw mode: {err}");
    }
    if let Err(err) = execute!(io::stdout(), LeaveAlternateScreen) {
        eprintln!("Error leaving alternate screen: {err}");
    }
}

/// Replace the default panic hook with one that restores the terminal before panicking.
fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        restore_terminal();
        hook(panic_info);
    }));
}

/// Run the application loop. This is where you would handle events and update the application
/// state. This example exits when the user presses 'q'. Other styles of application loops are
/// possible, for example, you could have multiple application states and switch between them based
/// on events, or you could have a single application state and update it based on events.
fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    loop {
        terminal.draw(render_app)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

/// Render the application. This is where you would draw the application UI. This example just
/// draws a greeting.
fn render_app(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.area());
}

/// Check if the user has pressed 'q'. This is where you would handle events. This example just
/// checks if the user has pressed 'q' and returns true if they have. It does not handle any other
/// events. There is a 250ms timeout on the event poll so that the application can exit in a timely
/// manner, and to ensure that the terminal is rendered at least once every 250ms. This allows you
/// to do other work in the application loop, such as updating the application state, without
/// blocking the event loop for too long.
fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
