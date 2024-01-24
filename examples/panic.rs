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

use std::{error::Error, io};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Default)]
struct App {
    hook_enabled: bool,
}

impl App {
    fn chain_hook(&mut self) {
        let original_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic| {
            reset_terminal().unwrap();
            original_hook(panic);
        }));

        self.hook_enabled = true;
    }
}

fn main() -> Result<()> {
    let mut terminal = init_terminal()?;

    let mut app = App::default();
    let res = run_tui(&mut terminal, &mut app);

    reset_terminal()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

/// Initializes the terminal.
fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(io::stdout());

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

/// Resets the terminal.
fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}

/// Runs the TUI loop.
fn run_tui<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('p') => {
                    panic!("intentional demo panic");
                }

                KeyCode::Char('e') => {
                    app.chain_hook();
                }

                _ => {
                    return Ok(());
                }
            }
        }
    }
}

/// Render the TUI.
fn ui(f: &mut Frame, app: &App) {
    let text = vec![
        if app.hook_enabled {
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

    let b = Block::default()
        .title("Panic Handler Demo")
        .borders(Borders::ALL);

    let p = Paragraph::new(text).block(b).centered();

    f.render_widget(p, f.size());
}
