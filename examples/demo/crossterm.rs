use std::time::{Duration, Instant};

use color_eyre::Result;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    Terminal,
};

use crate::{app::App, ui};

pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<()> {
    let terminal = CrosstermBackend::stdout_with_defaults()?
        .with_mouse_capture()?
        .to_terminal()?;

    let app = App::new("Crossterm Demo", enhanced_graphics);
    run_app(app, terminal, tick_rate)
}

fn run_app(mut app: App, mut terminal: Terminal<impl Backend>, tick_rate: Duration) -> Result<()> {
    let mut last_tick = Instant::now();
    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Left | KeyCode::Char('h') => app.on_left(),
                        KeyCode::Up | KeyCode::Char('k') => app.on_up(),
                        KeyCode::Right | KeyCode::Char('l') => app.on_right(),
                        KeyCode::Down | KeyCode::Char('j') => app.on_down(),
                        KeyCode::Char(c) => app.on_key(c),
                        _ => {}
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
    Ok(())
}
