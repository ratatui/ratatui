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

use indoc::indoc;
use itertools::izip;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::terminal::{disable_raw_mode, enable_raw_mode},
    terminal::{Terminal, Viewport},
    widgets::Paragraph,
    TerminalOptions,
};

/// A fun example of using half block characters to draw a logo
#[allow(clippy::many_single_char_names)]
fn logo() -> String {
    let r = indoc! {"
            ▄▄▄
            █▄▄▀
            █  █
        "};
    let a = indoc! {"
             ▄▄
            █▄▄█
            █  █
        "};
    let t = indoc! {"
            ▄▄▄
             █
             █
        "};
    let u = indoc! {"
            ▄  ▄
            █  █
            ▀▄▄▀
        "};
    let i = indoc! {"
            ▄
            █
            █
        "};
    izip!(r.lines(), a.lines(), t.lines(), u.lines(), i.lines())
        .map(|(r, a, t, u, i)| format!("{r:5}{a:5}{t:4}{a:5}{t:4}{u:5}{i:5}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() -> io::Result<()> {
    let mut terminal = init()?;
    terminal.draw(|frame| {
        frame.render_widget(Paragraph::new(logo()), frame.size());
    })?;
    sleep(Duration::from_secs(5));
    restore()?;
    println!();
    Ok(())
}

fn init() -> io::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    let options = TerminalOptions {
        viewport: Viewport::Inline(3),
    };
    Terminal::with_options(CrosstermBackend::new(stdout()), options)
}

fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    Ok(())
}
