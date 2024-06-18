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

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use indoc::indoc;
use itertools::izip;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    terminal::Viewport,
    widgets::Paragraph,
    TerminalOptions,
};

fn main() -> Result<()> {
    let mut terminal =
        CrosstermBackend::stdout_with_defaults()?.to_terminal_with_options(TerminalOptions {
            viewport: Viewport::Inline(3),
        })?;
    terminal.draw(|frame| {
        frame.render_widget(logo(), frame.size());
    })?;
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    println!(); // necessary to avoid the cursor being on the same line as the logo
    Ok(())
}

/// A fun example of using half block characters to draw a logo
#[allow(clippy::many_single_char_names)]
fn logo() -> Paragraph<'static> {
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
    let lines = izip!(r.lines(), a.lines(), t.lines(), u.lines(), i.lines())
        .map(|(r, a, t, u, i)| format!("{r:5}{a:5}{t:4}{a:5}{t:4}{u:5}{i:5}"))
        .collect::<Vec<_>>()
        .join("\n");
    Paragraph::new(lines)
}
