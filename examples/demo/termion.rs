use crate::{app::App, ui};
use ratatui::{
    backend::{Backend, TermionBackend},
    Terminal,
};
use std::{error::Error, io, sync::mpsc, thread, time::Duration};
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::IntoAlternateScreen,
};

pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    // setup terminal
    let stdout = io::stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();
    let stdout = MouseTerminal::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // create app and run it
    let app = App::new("Termion demo", enhanced_graphics);
    run_app(&mut terminal, app, tick_rate)?;

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> Result<(), Box<dyn Error>> {
    let events = events(tick_rate);
    loop {
        ui::draw_ui(terminal, &mut app)?;

        match events.recv()? {
            Event::Input(key) => match key {
                // Silently ignore viewport overscroll attempts.
                Key::Char(c) if c == 'u' => terminal.viewport_scroll(0, -1)?.unwrap_or(()),
                Key::Char(c) if c == 'd' => terminal.viewport_scroll(0, 1)?.unwrap_or(()),
                Key::Char(c) if c == 'n' => terminal.viewport_scroll(1, 0)?.unwrap_or(()),
                Key::Char(c) if c == 'p' => terminal.viewport_scroll(-1, 0)?.unwrap_or(()),
                Key::Char(c) => app.on_key(c),
                Key::Up => app.on_up(),
                Key::Down => app.on_down(),
                Key::Left => app.on_left(),
                Key::Right => app.on_right(),
                _ => {}
            },
            Event::Tick => app.on_tick(),
        }
        if app.should_quit {
            return Ok(());
        }
    }
}

enum Event {
    Input(Key),
    Tick,
}

fn events(tick_rate: Duration) -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel();
    let keys_tx = tx.clone();
    thread::spawn(move || {
        let stdin = io::stdin();
        for key in stdin.keys().flatten() {
            if let Err(err) = keys_tx.send(Event::Input(key)) {
                eprintln!("{}", err);
                return;
            }
        }
    });
    thread::spawn(move || loop {
        if let Err(err) = tx.send(Event::Tick) {
            eprintln!("{}", err);
            break;
        }
        thread::sleep(tick_rate);
    });
    rx
}
