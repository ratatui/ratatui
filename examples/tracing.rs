//! # [Ratatui] Tracing example
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

// A simple example demonstrating how to use the [tracing] with Ratatui to log to a file.
//
// This example demonstrates how to use the [tracing] crate with Ratatui to log to a file. The
// example sets up a simple logger that logs to a file named `tracing.log` in the current directory.
//
// Run the example with `cargo run --example tracing` and then view the `tracing.log` file to see
// the logs. To see more logs, you can run the example with `RUST_LOG=tracing=debug cargo run
// --example`
//
// For a helpful widget that handles logging, see the [tui-logger] crate.
//
// [tracing]: https://crates.io/crates/tracing
// [tui-logger]: https://crates.io/crates/tui-logger

use std::{fs::File, io::stdout, panic, time::Duration};

use color_eyre::{
    config::HookBuilder,
    eyre::{self, Context},
    Result,
};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    terminal::Terminal,
    widgets::{Block, Paragraph},
};
use tracing::{debug, info, instrument, trace, Level};
use tracing_appender::{non_blocking, non_blocking::WorkerGuard};
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    init_error_hooks()?;
    let _guard = init_tracing()?;
    info!("Starting tracing example");

    let mut terminal = init_terminal()?;
    let mut events = vec![]; // a buffer to store the recent events to display in the UI
    while !should_exit(&events) {
        handle_events(&mut events)?;
        terminal.draw(|frame| ui(frame, &events))?;
    }
    restore_terminal()?;
    info!("Exiting tracing example");
    println!("See the tracing.log file for the logs");
    Ok(())
}

fn should_exit(events: &[Event]) -> bool {
    events
        .iter()
        .any(|event| matches!(event, Event::Key(key) if key.code == KeyCode::Char('q')))
}

/// Handle events and insert them into the events vector keeping only the last 10 events
#[instrument(skip(events))]
fn handle_events(events: &mut Vec<Event>) -> Result<()> {
    // Render the UI at least once every 100ms
    if event::poll(Duration::from_millis(100))? {
        let event = event::read()?;
        debug!(?event);
        events.insert(0, event);
    }
    events.truncate(10);
    Ok(())
}

#[instrument(skip_all)]
fn ui(frame: &mut ratatui::Frame, events: &[Event]) {
    // To view this event, run the example with `RUST_LOG=tracing=debug cargo run --example tracing`
    trace!(frame_count = frame.count(), event_count = events.len());
    let area = frame.size();
    let events = events.iter().map(|e| format!("{e:?}")).collect::<Vec<_>>();
    let paragraph = Paragraph::new(events.join("\n"))
        .block(Block::bordered().title("Tracing example. Press 'q' to quit."));
    frame.render_widget(paragraph, area);
}

/// Initialize the tracing subscriber to log to a file
///
/// This function initializes the tracing subscriber to log to a file named `tracing.log` in the
/// current directory. The function returns a [`WorkerGuard`] that must be kept alive for the
/// duration of the program to ensure that logs are flushed to the file on shutdown. The logs are
/// written in a non-blocking fashion to ensure that the logs do not block the main thread.
fn init_tracing() -> Result<WorkerGuard> {
    let file = File::create("tracing.log").wrap_err("failed to create tracing.log")?;
    let (non_blocking, guard) = non_blocking(file);

    // By default, the subscriber is configured to log all events with a level of `DEBUG` or higher,
    // but this can be changed by setting the `RUST_LOG` environment variable.
    let env_filter = EnvFilter::builder()
        .with_default_directive(Level::DEBUG.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(env_filter)
        .init();
    Ok(guard)
}

/// Initialize the error hooks to ensure that the terminal is restored to a sane state before
/// exiting
fn init_error_hooks() -> Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info);
    }));
    Ok(())
}

#[instrument]
fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    debug!("terminal initialized");
    Ok(terminal)
}

#[instrument]
fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    debug!("terminal restored");
    Ok(())
}
