//! Terminal initialization and cleanup
//!
//! This module contains functions similar to the ones in ratatui for initializing and cleaning up
//! the terminal, but they are tailored to the needs of the demo2 example.
use std::{
    io::{self, stdout},
    time::Duration,
};

use color_eyre::{eyre::WrapErr, Result};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::Rect,
    DefaultTerminal, Terminal, TerminalOptions, Viewport,
};

pub fn init() -> DefaultTerminal {
    set_panic_hook();
    try_init().expect("failed to initialize terminal")
}

fn try_init() -> Result<DefaultTerminal> {
    // this size is to match the size of the terminal when running the demo
    // using vhs in a 1280x640 sized window (github social preview size)
    let options = TerminalOptions {
        viewport: Viewport::Fixed(Rect::new(0, 0, 81, 18)),
    };
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::with_options(backend, options)?;
    enable_raw_mode().context("enable raw mode")?;
    stdout()
        .execute(EnterAlternateScreen)
        .wrap_err("enter alternate screen")?;
    Ok(terminal)
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        restore();
        hook(panic_info);
    }));
}

pub fn restore() {
    if let Err(err) = try_restore() {
        // We can't do much if restoring the terminal fails, so just print the error
        eprintln!("Error restoring terminal: {err}");
    }
}

fn try_restore() -> Result<()> {
    disable_raw_mode().context("disable raw mode")?;
    stdout()
        .execute(LeaveAlternateScreen)
        .wrap_err("leave alternate screen")?;
    Ok(())
}

pub fn next_event(timeout: Duration) -> Result<Option<Event>> {
    if !event::poll(timeout)? {
        return Ok(None);
    }
    let event = event::read()?;
    Ok(Some(event))
}
