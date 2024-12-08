//! # [Ratatui] `RatatuiLogo` example
//!
//! The latest version of this example is available in the [widget examples] folder in the
//! repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [widget examples]: https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use std::env::args;

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    widgets::{RatatuiLogo, RatatuiLogoSize},
    DefaultTerminal, Frame, TerminalOptions, Viewport,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(3),
    });
    let size = match args().nth(1).as_deref() {
        Some("small") => RatatuiLogoSize::Small,
        Some("tiny") => RatatuiLogoSize::Tiny,
        _ => RatatuiLogoSize::default(),
    };
    let result = run(terminal, size);
    ratatui::restore();
    println!();
    result
}

/// Run the application.
fn run(mut terminal: DefaultTerminal, size: RatatuiLogoSize) -> Result<()> {
    loop {
        terminal.draw(|frame| draw(frame, size))?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

/// Draw the UI with a logo.
fn draw(frame: &mut Frame, size: RatatuiLogoSize) {
    let [top, bottom] =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(frame.area());

    frame.render_widget("Powered by", top);
    frame.render_widget(RatatuiLogo::new(size), bottom);
}
