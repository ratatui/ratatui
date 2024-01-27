//! # [Ratatui] Logo example
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
    io::{self, stdout},
    thread::sleep,
    time::Duration,
};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use indoc::indoc;
use itertools::izip;
use ratatui::{prelude::*, widgets::*};

/// A fun example of using half block characters to draw a logo
fn main() -> io::Result<()> {
    let r = indoc! {"
            ▄▄▄
            █▄▄▀
            █  █
        "}
    .lines();
    let a = indoc! {"
             ▄▄
            █▄▄█
            █  █
        "}
    .lines();
    let t = indoc! {"
            ▄▄▄
             █
             █
        "}
    .lines();
    let u = indoc! {"
            ▄  ▄
            █  █
            ▀▄▄▀
        "}
    .lines();
    let i = indoc! {"
            ▄
            █
            █
        "}
    .lines();
    let mut terminal = init()?;
    terminal.draw(|frame| {
        let logo = izip!(r, a.clone(), t.clone(), a, t, u, i)
            .map(|(r, a, t, a2, t2, u, i)| {
                format!("{:5}{:5}{:4}{:5}{:4}{:5}{:5}", r, a, t, a2, t2, u, i)
            })
            .collect::<Vec<_>>()
            .join("\n");
        frame.render_widget(Paragraph::new(logo), frame.size());
    })?;
    sleep(Duration::from_secs(5));
    restore()?;
    println!();
    Ok(())
}

pub fn init() -> io::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    let options = TerminalOptions {
        viewport: Viewport::Inline(3),
    };
    Terminal::with_options(CrosstermBackend::new(stdout()), options)
}

pub fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    Ok(())
}
