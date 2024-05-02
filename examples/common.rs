//! # [Ratatui] common code for use in the examples
//!
//! This module contains common code that is used in the examples. This includes functions to
//! initialize and restore the terminal, and to install the panic and error hooks.
//!
//! The latest version of this code is available in the [examples] folder in the repository.
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
    error::Error,
    io::{self, stdout, Stdout},
    panic,
};

use color_eyre::{
    config::{EyreHook, HookBuilder, PanicHook},
    eyre,
};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

/// Initialize the terminal by enabling raw mode and entering the alternate screen.
pub fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore the terminal by leaving the alternate screen and disabling raw mode.
pub fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen,)?;
    Ok(())
}

/// This replaces the standard `color_eyre` panic and error hooks with hooks that restore the
/// terminal before printing the panic or error.
pub fn install_hooks() -> color_eyre::Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();
    install_panic_hook(panic_hook);
    install_error_hook(eyre_hook)?;
    Ok(())
}

/// Install a panic hook that restores the terminal before printing the panic.
fn install_panic_hook(panic_hook: PanicHook) {
    // convert from a color_eyre PanicHook to a standard panic hook
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        panic_hook(panic_info);
    }));
}

/// Install an error hook that restores the terminal before printing the error.
fn install_error_hook(eyre_hook: EyreHook) -> eyre::Result<()> {
    // convert from a color_eyre EyreHook to a standard eyre hook
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |error: &(dyn Error + 'static)| {
        let _ = restore_terminal();
        eyre_hook(error)
    }))?;
    Ok(())
}
