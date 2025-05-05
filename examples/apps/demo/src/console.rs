use std::error::Error;
use std::time::{Duration, Instant};

use console::Key;
use ratatui::backend::ConsoleBackend;
use ratatui::Terminal;

use crate::app::App;
use crate::ui;

pub fn run(tick_rate: Duration, unicode: bool) -> Result<(), Box<dyn Error>> {
    let backend = ConsoleBackend::default();
    let mut terminal = Terminal::new(backend)?;
    // println!("\x1b[?1049h"); // enter alternate mode
    terminal.hide_cursor()?;

    // create app and run it
    let app = App::new("Console Demo", unicode);
    let app_result = run_app(&mut terminal, app, tick_rate);

    terminal.show_cursor()?;
    // println!("\x1b[?1049l"); // leave alternate mode
    terminal.flush()?;

    if let Err(err) = app_result {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<ConsoleBackend>,
    mut app: App,
    tick_rate: Duration,
) -> Result<(), Box<dyn Error>> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        if let Ok(input) = terminal.backend_mut().get_term().read_key() {
            match input {
                Key::ArrowLeft | Key::Char('h') => app.on_left(),
                Key::ArrowDown | Key::Char('j') => app.on_down(),
                Key::ArrowUp | Key::Char('k') => app.on_up(),
                Key::ArrowRight | Key::Char('l') => app.on_right(),
                Key::Char(ch) => app.on_key(ch),
                _ => {} /* InputEvent::Resized { cols, rows } => {
                         *     terminal
                         *         .backend_mut()
                         *         .buffered_terminal_mut()
                         *         .resize(cols, rows);
                         * } */
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
