//! How to use a panic hook to reset the terminal before printing the panic to the terminal.
//!
//! When a panic occurs in an app using ratatui, the terminal will be left in an inconsistent state.
//! Generally the backend's `Drop` implementation will reset the terminal, but because the panic
//! interrupts the normal control flow, the `Drop` impl is called after the panic handler. This
//! results in the panic being printed to the terminal on the alternate screen, and in raw mode.
//!
//! This can be fixed by setting a panic hook that resets the terminal before printing the panic.
//! The example shows both situations, with and without the chained panic hook, to see the
//! difference.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::*};

#[derive(Default)]
struct App {
    hook_enabled: bool,
}

impl App {
    fn chain_hook(&mut self) {
        let original_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic| {
            let mut backend = CrosstermBackend::on_stdout().unwrap();
            backend.leave_alternate_screen().unwrap();
            backend.disable_raw_mode().unwrap();
            original_hook(panic);
        }));

        self.hook_enabled = true;
    }
}

fn main() -> Result<()> {
    let backend = CrosstermBackend::on_stdout()?;
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();
    run_tui(&mut terminal, &mut app)
}

fn run_tui<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
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

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let text = vec![
        if app.hook_enabled {
            Line::from("HOOK IS CURRENTLY **ENABLED**")
        } else {
            Line::from("HOOK IS CURRENTLY **DISABLED**")
        },
        Line::from(""),
        Line::from("Press `p` to panic"),
        Line::from("Press `e` to enable the terminal-resetting panic hook"),
        Line::from("Press any other key to quit without panic"),
        Line::from(""),
        Line::from("When you panic without the chained hook, you may not see the panic report as"),
        Line::from("it is printed to the alternate screen. Some terminal emulators may let you"),
        Line::from("manually switch to the alternate screen to view this message."),
        Line::from(""),
        Line::from("With the chained panic hook enabled, the terminal will be reset before the"),
        Line::from("panic is printed. You should see the panic report as you would without "),
        Line::from("ratatui"),
        Line::from(""),
        Line::from("Try first without the panic handler to see the difference."),
    ];

    let b = Block::default()
        .title("Panic Handler Demo")
        .borders(Borders::ALL);

    let p = Paragraph::new(text).block(b).alignment(Alignment::Center);

    f.render_widget(p, f.size());
}
