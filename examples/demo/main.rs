mod app;
#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "termion")]
mod termion;
#[cfg(feature = "termwiz")]
mod termwiz;

mod ui;

use std::{error::Error, time::Duration};

use argh::FromArgs;

#[cfg(feature = "crossterm")]
use crate::crossterm::run;
#[cfg(feature = "termion")]
use crate::termion::run;
#[cfg(feature = "termwiz")]
use crate::termwiz::run;

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
    run(tick_rate, cli.enhanced_graphics)?;
    Ok(())
}
