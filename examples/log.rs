use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{set_logger, set_max_level, warn, Log};
use once_cell::sync::Lazy;
use rand::random;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    text::Text,
    widgets::{Block, Paragraph},
    Frame, Terminal,
};
use std::{
    error::Error,
    fmt::Write,
    io,
    sync::Mutex,
    time::{Duration, Instant},
};

struct App {}

struct ParagraphLogger {
    text: Mutex<Text<'static>>,
}

impl ParagraphLogger {
    fn new() -> ParagraphLogger {
        ParagraphLogger {
            text: Mutex::new(Text::raw("")),
        }
    }
}

impl Log for ParagraphLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        writeln!(
            self.text.lock().unwrap(),
            "{}: {}",
            record.level(),
            record.args()
        )
        .unwrap();
    }

    fn flush(&self) {}
}

static LOGGER: Lazy<ParagraphLogger> = Lazy::new(ParagraphLogger::new);

impl App {
    fn new() -> App {
        App {}
    }

    fn on_tick(&mut self) {
        let rand: f32 = random();
        if rand > 0.75 {
            warn!("Tolerance exceeded!");
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup logger
    set_logger(&*LOGGER).unwrap();
    set_max_level(log::LevelFilter::Info);

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
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
        println!("{err:?}");
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
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, _: &App) {
    let size = f.size();

    let paragraph = Paragraph::new(LOGGER.text.lock().unwrap().clone()).block(Block::default());

    f.render_widget(paragraph, size);
}
