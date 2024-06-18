use std::time::{Duration, Instant};

use color_eyre::{eyre::eyre, Result};
use ratatui::{
    backend::TermwizBackend,
    terminal::Terminal,
    termwiz::{
        input::{InputEvent, KeyCode},
        terminal::Terminal as TermwizTerminal,
    },
};

use crate::{app::App, ui};

pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<()> {
    let backend =
        TermwizBackend::new().map_err(|error| eyre!("failed to init termwiz backend. {error}"))?;
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let app = App::new("Termwiz Demo", enhanced_graphics);
    let res = run_app(app, &mut terminal, tick_rate);

    terminal.show_cursor()?;
    terminal.flush()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app(
    mut app: App,
    terminal: &mut Terminal<TermwizBackend>,
    tick_rate: Duration,
) -> Result<()> {
    let mut last_tick = Instant::now();
    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if let Some(input) = terminal
            .backend_mut()
            .buffered_terminal_mut()
            .terminal()
            .poll_input(Some(timeout))?
        {
            match input {
                InputEvent::Key(key_code) => match key_code.key {
                    KeyCode::UpArrow | KeyCode::Char('k') => app.on_up(),
                    KeyCode::DownArrow | KeyCode::Char('j') => app.on_down(),
                    KeyCode::LeftArrow | KeyCode::Char('h') => app.on_left(),
                    KeyCode::RightArrow | KeyCode::Char('l') => app.on_right(),
                    KeyCode::Char(c) => app.on_key(c),
                    _ => {}
                },
                InputEvent::Resized { cols, rows } => {
                    terminal
                        .backend_mut()
                        .buffered_terminal_mut()
                        .resize(cols, rows);
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
    Ok(())
}
