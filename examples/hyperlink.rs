//! # [Ratatui] Hyperlink examplew
//!
//! Shows how to use [OSC 8] to create hyperlinks in the terminal.
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
//! [OSC 8]: https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

use std::io::{self, stdout, Stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use itertools::Itertools;
use ratatui::{prelude::*, widgets::WidgetRef};

fn main() -> io::Result<()> {
    let mut terminal = init_terminal()?;
    App::new().run(&mut terminal)?;
    restore_terminal()?;
    Ok(())
}

struct App {
    hyperlink: Hyperlink<'static>,
}

impl App {
    fn new() -> Self {
        let text = Line::from(vec!["Example ".into(), "hyperlink".blue()]);
        Self {
            hyperlink: Hyperlink::new(text, "https://example.com"),
        }
    }

    fn run(self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        loop {
            terminal.draw(|frame| {
                frame.render_widget(&self.hyperlink, frame.size());
            })?;
            if should_quit()? {
                break;
            }
        }
        Ok(())
    }
}

/// A hyperlink widget that renders a hyperlink in the terminal using [OSC 8].
///
/// [OSC 8]: https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
struct Hyperlink<'content> {
    text: Text<'content>,
    url: String,
}

impl<'content> Hyperlink<'content> {
    fn new(text: impl Into<Text<'content>>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: url.into(),
        }
    }
}

impl WidgetRef for Hyperlink<'_> {
    fn render_ref(&self, area: Rect, buffer: &mut Buffer) {
        self.text.render_ref(area, buffer);

        // this is a hacky workaround for https://github.com/ratatui-org/ratatui/issues/902, a bug
        // in the terminal code that incorrectly calculates the width of ANSI escape sequences. It
        // works by rendering the hyperlink as a series of 2-character chunks, which is the
        // calculated width of the hyperlink text.
        for (i, two_chars) in self
            .text
            .to_string()
            .chars()
            .chunks(2)
            .into_iter()
            .enumerate()
        {
            let text = two_chars.collect::<String>();
            let hyperlink = format!("\x1B]8;;{}\x07{}\x1B]8;;\x07", self.url, text);
            buffer
                .get_mut(area.x + i as u16 * 2, area.y)
                .set_symbol(hyperlink.as_str());
        }
    }
}

fn should_quit() -> io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        return Ok(KeyCode::Char('q') == key.code);
    }
    Ok(false)
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(terminal)
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
