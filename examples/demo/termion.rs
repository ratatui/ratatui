use std::{io, sync::mpsc, thread, time::Duration};

use color_eyre::Result;
use ratatui::{
    backend::{Backend, TermionBackend},
    terminal::Terminal,
    termion::{
        event::Key,
        input::{MouseTerminal, TermRead},
        raw::IntoRawMode,
        screen::IntoAlternateScreen,
    },
};

use crate::{app::App, ui};

pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<()> {
    // setup terminal
    let stdout = io::stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();
    let stdout = MouseTerminal::from(stdout);
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    let app = App::new("Termion demo", enhanced_graphics);
    run_app(app, terminal, tick_rate)?;

    Ok(())
}

fn run_app(mut app: App, mut terminal: Terminal<impl Backend>, tick_rate: Duration) -> Result<()> {
    let events = events(tick_rate);
    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match events.recv()? {
            Event::Input(key) => match key {
                Key::Up | Key::Char('k') => app.on_up(),
                Key::Down | Key::Char('j') => app.on_down(),
                Key::Left | Key::Char('h') => app.on_left(),
                Key::Right | Key::Char('l') => app.on_right(),
                Key::Char(c) => app.on_key(c),
                _ => {}
            },
            Event::Tick => app.on_tick(),
        }
    }
    Ok(())
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
                eprintln!("{err}");
                return;
            }
        }
    });
    thread::spawn(move || loop {
        if let Err(err) = tx.send(Event::Tick) {
            eprintln!("{err}");
            break;
        }
        thread::sleep(tick_rate);
    });
    rx
}
