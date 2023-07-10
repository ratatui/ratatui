use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use ratatui::prelude::*;
use termwiz::{input::*, terminal::Terminal as TermwizTerminal};

use crate::{app::App, ui};

pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    let backend = TermwizBackend::new()?;
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // create app and run it
    let app = App::new("Termwiz Demo", enhanced_graphics);
    let res = run_app(&mut terminal, app, tick_rate);

    terminal.show_cursor()?;
    terminal.flush()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<TermwizBackend>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if let Ok(Some(input)) = terminal
            .backend_mut()
            .buffered_terminal_mut()
            .terminal()
            .poll_input(Some(timeout))
        {
            match input {
                InputEvent::Key(key_code) => match key_code.key {
                    KeyCode::UpArrow => app.on_up(),
                    KeyCode::DownArrow => app.on_down(),
                    KeyCode::LeftArrow => app.on_left(),
                    KeyCode::RightArrow => app.on_right(),
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
        if app.should_quit {
            return Ok(());
        }
    }
}
