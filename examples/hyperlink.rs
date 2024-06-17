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

use std::{
    io::{self, stdout, Stdout},
    panic,
};

use color_eyre::{
    config::{EyreHook, HookBuilder, PanicHook},
    eyre, Result,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use itertools::Itertools;
use ratatui::{prelude::*, widgets::WidgetRef};

fn main() -> Result<()> {
    init_error_handling()?;
    let mut terminal = init_terminal()?;
    let app = App::new();
    app.run(&mut terminal)?;
    restore_terminal()?;
    Ok(())
}

struct App {
    hyperlink: Hyperlink<'static>,
}

impl App {
    fn new() -> Self {
        let text = Line::from(vec!["Example ".into(), "hyperlink".blue()]);
        let hyperlink = Hyperlink::new(text, "https://example.com");
        Self { hyperlink }
    }

    fn run(self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        loop {
            terminal.draw(|frame| frame.render_widget(&self.hyperlink, frame.size()))?;
            if let Event::Key(key) = event::read()? {
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                    break;
                }
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

/// Initialize the terminal with raw mode and alternate screen.
fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state.
fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

/// Initialize error handling with color-eyre.
pub fn init_error_handling() -> Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();
    set_panic_hook(panic_hook);
    set_error_hook(eyre_hook)?;
    Ok(())
}

/// Install a panic hook that restores the terminal before printing the panic.
fn set_panic_hook(panic_hook: PanicHook) {
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        panic_hook(panic_info);
    }));
}

/// Install an error hook that restores the terminal before printing the error.
fn set_error_hook(eyre_hook: EyreHook) -> Result<()> {
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |error| {
        let _ = restore_terminal();
        eyre_hook(error)
    }))?;
    Ok(())
}
