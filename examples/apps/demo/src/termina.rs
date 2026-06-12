#![allow(dead_code)]
use std::error::Error;
use std::io::Write as _;
use std::time::{Duration, Instant};

use ratatui::Terminal;
use ratatui::backend::TerminaBackend;
use ratatui::termina::escape::csi::{self, Csi};
use ratatui::termina::event::{KeyCode, KeyEventKind};
use ratatui::termina::{Event, EventReader, PlatformTerminal, Terminal as _};

use crate::app::App;
use crate::ui;

type AppTerminal = Terminal<TerminaBackend<PlatformTerminal>>;

macro_rules! decset {
    ($mode:ident) => {{
        let mode = csi::DecPrivateMode::Code(csi::DecPrivateModeCode::$mode);
        Csi::Mode(csi::Mode::SetDecPrivateMode(mode))
    }};
}

macro_rules! decreset {
    ($mode:ident) => {{
        let mode = csi::DecPrivateMode::Code(csi::DecPrivateModeCode::$mode);
        Csi::Mode(csi::Mode::ResetDecPrivateMode(mode))
    }};
}

pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    let (mut terminal, events) = init_terminal()?;
    let app = App::new("Termina Demo", enhanced_graphics);
    let app_result = run_app(&mut terminal, &events, app, tick_rate);

    restore_terminal(&mut terminal)?;

    if let Err(err) = app_result {
        println!("{err:?}");
    }

    Ok(())
}

fn init_terminal() -> Result<(AppTerminal, EventReader), Box<dyn Error>> {
    let mut output = PlatformTerminal::new()?;
    output.enter_raw_mode()?;

    let enter_alternate_screen = decset!(ClearAndEnableAlternateScreen);
    let show_cursor = decset!(ShowCursor);
    write!(output, "{enter_alternate_screen}{show_cursor}")?;
    output.flush()?;

    let events = output.event_reader();
    let backend = TerminaBackend::new(output);
    let terminal = Terminal::new(backend)?;
    Ok((terminal, events))
}

fn run_app(
    terminal: &mut AppTerminal,
    events: &EventReader,
    mut app: App,
    tick_rate: Duration,
) -> Result<(), Box<dyn Error>> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if events.poll(Some(timeout), is_key_event)? {
            let event = events.read(is_key_event)?;
            handle_event(&event, &mut app);
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

fn is_key_event(event: &Event) -> bool {
    matches!(event, Event::Key(_))
}

fn handle_event(event: &Event, app: &mut App) {
    if let Event::Key(key) = event {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('k') | KeyCode::Up => app.on_up(),
            KeyCode::Char('j') | KeyCode::Down => app.on_down(),
            KeyCode::Char('h') | KeyCode::Left => app.on_left(),
            KeyCode::Char('l') | KeyCode::Right => app.on_right(),
            KeyCode::Char(c) => app.on_key(c),
            _ => {}
        }
    }
}

fn restore_terminal(terminal: &mut AppTerminal) -> Result<(), Box<dyn Error>> {
    let leave_alternate_screen = decreset!(ClearAndEnableAlternateScreen);
    let show_cursor = decset!(ShowCursor);
    let backend = terminal.backend_mut();
    write!(backend, "{leave_alternate_screen}{show_cursor}")?;
    backend.flush()?;
    Ok(())
}
