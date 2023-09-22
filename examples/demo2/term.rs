use std::{
    io::{self, stdout, Stdout},
    ops::{Deref, DerefMut},
    time::Duration,
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;

/// A wrapper around the terminal that handles setting up and tearing down the terminal
/// and provides a helper method to read events from the terminal.
#[derive(Debug)]
pub struct Term {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Term {
    pub fn start() -> Result<Self> {
        // this size is to match the size of the terminal when running the demo
        // using vhs in a 1280x640 sized window (github social preview size)
        let options = TerminalOptions {
            viewport: Viewport::Fixed(Rect::new(0, 0, 81, 18)),
        };
        let terminal = Terminal::with_options(CrosstermBackend::new(io::stdout()), options)?;
        enable_raw_mode().context("enable raw mode")?;
        stdout()
            .execute(EnterAlternateScreen)
            .context("enter alternate screen")?;
        Ok(Self { terminal })
    }

    pub fn stop() -> Result<()> {
        disable_raw_mode().context("disable raw mode")?;
        stdout()
            .execute(LeaveAlternateScreen)
            .context("leave alternate screen")?;
        Ok(())
    }

    pub fn next_event(timeout: Duration) -> io::Result<Option<Event>> {
        if !event::poll(timeout)? {
            return Ok(None);
        }
        let event = event::read()?;
        Ok(Some(event))
    }
}

impl Deref for Term {
    type Target = Terminal<CrosstermBackend<Stdout>>;
    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Term {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        let _ = Term::stop();
    }
}
