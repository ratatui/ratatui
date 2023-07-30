use std::{error::Error, time::Duration};

use argh::FromArgs;

mod app;
#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "termion")]
mod termion;
#[cfg(feature = "termwiz")]
mod termwiz;

mod ui;

/// Demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    #[cfg(feature = "crossterm")]
    crossterm::run(tick_rate, cli.enhanced_graphics)?;
    #[cfg(feature = "termion")]
    termion::run(tick_rate, cli.enhanced_graphics)?;
    #[cfg(feature = "termwiz")]
    termwiz::run(tick_rate, cli.enhanced_graphics)?;
    Ok(())
}
