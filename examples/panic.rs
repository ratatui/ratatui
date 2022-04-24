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

#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]

use std::error::Error;
use std::io;

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

use tui::backend::{Backend, CrosstermBackend};
use tui::layout::Alignment;
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph};
use tui::{Frame, Terminal};

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
        println!("{:?}", err);
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
fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let text = vec![
        if app.hook_enabled {
            Spans::from("HOOK IS CURRENTLY **ENABLED**")
        } else {
            Spans::from("HOOK IS CURRENTLY **DISABLED**")
        },
        Spans::from(""),
        Spans::from("press `p` to panic"),
        Spans::from("press `e` to enable the terminal-resetting panic hook"),
        Spans::from("press any other key to quit without panic"),
        Spans::from(""),
        Spans::from("when you panic without the chained hook,"),
        Spans::from("you will likely have to reset your terminal afterwards"),
        Spans::from("with the `reset` command"),
        Spans::from(""),
        Spans::from("with the chained panic hook enabled,"),
        Spans::from("you should see the panic report as you would without tui"),
        Spans::from(""),
        Spans::from("try first without the panic handler to see the difference"),
    ];

    let b = Block::default()
        .title("Panic Handler Demo")
        .borders(Borders::ALL);

    let p = Paragraph::new(text).block(b).alignment(Alignment::Center);

    f.render_widget(p, f.size());
}
