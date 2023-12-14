use std::{
    io::{self, stdout},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use indoc::indoc;
use itertools::izip;
use ratatui::{prelude::*, widgets::*};

/// A fun example of using half block characters to draw a logo
fn main() -> io::Result<()> {
    let mut terminal = init()?;
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
    Ok(())
}

pub fn init() -> io::Result<Terminal<impl Backend>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore() -> io::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
