use crate::{app::App, ui};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction},
    Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new_split(
        backend,
        vec![Constraint::Length(3), Constraint::Min(0)],
        Direction::Vertical,
    )?;
    terminal.hide_cursor()?;

    // create app and run it
    let app = App::new("Crossterm Demo", enhanced_graphics);
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        ui::draw_ui(terminal, &mut app)?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    // Skipping viewport overscroll attempts.
                    // See `viewport_scroll()` documentation for more.
                    KeyCode::Char(c) if c == 'd' => terminal.viewport_scroll(0, 1)?.unwrap_or(()),
                    KeyCode::Char(c) if c == 'u' => terminal.viewport_scroll(0, -1)?.unwrap_or(()),
                    KeyCode::Char(c) if c == 'f' => terminal.viewport_scroll(1, 0)?.unwrap_or(()),
                    KeyCode::Char(c) if c == 'p' => terminal.viewport_scroll(-1, 0)?.unwrap_or(()),
                    // TEMP:
                    KeyCode::Char(c) if c == 'n' => terminal
                        .split_viewport_scroll(|index| match index {
                            1 => (0, 1),
                            _ => (0, 0),
                        })?
                        .unwrap_or(()),
                    KeyCode::Char(c) if c == 'e' => terminal
                        .split_viewport_scroll(|index| match index {
                            1 => (0, -1),
                            _ => (0, 0),
                        })?
                        .unwrap_or(()),
                    KeyCode::Char(c) if c == 'i' => terminal
                        .split_viewport_scroll(|index| match index {
                            1 => (1, 0),
                            _ => (0, 0),
                        })?
                        .unwrap_or(()),
                    KeyCode::Char(c) if c == 'm' => terminal
                        .split_viewport_scroll(|index| match index {
                            1 => (-1, 0),
                            _ => (0, 0),
                        })?
                        .unwrap_or(()),
                    KeyCode::Char(c) => app.on_key(c),
                    KeyCode::Left => app.on_left(),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Right => app.on_right(),
                    KeyCode::Down => app.on_down(),
                    _ => {}
                }
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
