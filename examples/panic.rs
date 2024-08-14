//! # [Ratatui] Panic Hook example
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

//! How to use a panic hook to reset the terminal before printing the panic to
//! the terminal.
//!
//! When exiting normally or when handling `Result::Err`, we can reset the
//! terminal manually at the end of `main` just before we print the error.
//!
//! Because a panic interrupts the normal control flow, manually resetting the
//! terminal at the end of `main` won't do us any good. Instead, we need to
//! make sure to set up a panic hook that first resets the terminal before
//! handling the panic. This both reuses the standard panic hook to ensure a
//! consistent panic handling UX and properly resets the terminal to not
//! distort the output.
//!
//! That's why this example is set up to show both situations, with and without
//! the chained panic hook, to see the difference.

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    text::Line,
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result?;
    panic!("fix this example - it no longer needs to manually add the panic hook");
}

#[derive(Default)]
struct App {
    hook_enabled: bool,
}

impl App {
    /// Runs the TUI loop.
    fn run(self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('p') => {
                        panic!("intentional demo panic");
                    }

                    KeyCode::Char('e') => {
                        // self.chain_hook();
                    }

                    _ => {
                        return Ok(());
                    }
                }
            }
        }
    }

    /// Render the TUI.
    fn draw(&self, frame: &mut Frame) {
        let text = vec![
            if self.hook_enabled {
                Line::from("HOOK IS CURRENTLY **ENABLED**")
            } else {
                Line::from("HOOK IS CURRENTLY **DISABLED**")
            },
            Line::from(""),
            Line::from("press `p` to panic"),
            Line::from("press `e` to enable the terminal-resetting panic hook"),
            Line::from("press any other key to quit without panic"),
            Line::from(""),
            Line::from("when you panic without the chained hook,"),
            Line::from("you will likely have to reset your terminal afterwards"),
            Line::from("with the `reset` command"),
            Line::from(""),
            Line::from("with the chained panic hook enabled,"),
            Line::from("you should see the panic report as you would without ratatui"),
            Line::from(""),
            Line::from("try first without the panic handler to see the difference"),
        ];

        let paragraph = Paragraph::new(text)
            .block(Block::bordered().title("Panic Handler Demo"))
            .centered();

        frame.render_widget(paragraph, frame.area());
    }
}
