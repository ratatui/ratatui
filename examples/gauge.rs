//! # [Ratatui] Gauge example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

use std::{
    io::{self, stdout, Stdout},
    time::Duration,
};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{block::Title, *},
};

fn main() -> Result<()> {
    App::run()
}

struct App {
    term: Term,
    should_quit: bool,
    state: AppState,
}

#[derive(Debug, Default, Clone, Copy)]
struct AppState {
    progress1: u16,
    progress2: u16,
    progress3: f64,
    progress4: u16,
}

impl App {
    fn run() -> Result<()> {
        // run at ~10 fps minus the time it takes to draw
        let timeout = Duration::from_secs_f32(1.0 / 10.0);
        let mut app = Self::start()?;
        while !app.should_quit {
            app.update();
            app.draw()?;
            app.handle_events(timeout)?;
        }
        app.stop()?;
        Ok(())
    }

    fn start() -> Result<Self> {
        Ok(App {
            term: Term::start()?,
            should_quit: false,
            state: AppState {
                progress1: 0,
                progress2: 0,
                progress3: 0.0,
                progress4: 0,
            },
        })
    }

    fn stop(&mut self) -> Result<()> {
        Term::stop()?;
        Ok(())
    }

    fn update(&mut self) {
        self.state.progress1 = (self.state.progress1 + 4).min(100);
        self.state.progress2 = (self.state.progress2 + 3).min(100);
        self.state.progress3 = (self.state.progress3 + 0.02).min(1.0);
        self.state.progress4 = (self.state.progress4 + 1).min(100);
    }

    fn draw(&mut self) -> Result<()> {
        self.term.draw(|frame| {
            let state = self.state;
            let layout = Layout::vertical([Constraint::Ratio(1, 4); 4]).split(frame.size());
            Self::render_gauge1(state.progress1, frame, layout[0]);
            Self::render_gauge2(state.progress2, frame, layout[1]);
            Self::render_gauge3(state.progress3, frame, layout[2]);
            Self::render_gauge4(state.progress4, frame, layout[3]);
        })?;
        Ok(())
    }

    fn handle_events(&mut self, timeout: Duration) -> io::Result<()> {
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let KeyCode::Char('q') = key.code {
                        self.should_quit = true;
                    }
                }
            }
        }
        Ok(())
    }

    fn render_gauge1(progress: u16, frame: &mut Frame, area: Rect) {
        let title = Self::title_block("Gauge with percentage progress");
        let gauge = Gauge::default()
            .block(title)
            .gauge_style(Style::new().light_red())
            .percent(progress);
        frame.render_widget(gauge, area);
    }

    fn render_gauge2(progress: u16, frame: &mut Frame, area: Rect) {
        let title = Self::title_block("Gauge with percentage progress and custom label");
        let label = format!("{}/100", progress);
        let gauge = Gauge::default()
            .block(title)
            .gauge_style(Style::new().blue().on_light_blue())
            .percent(progress)
            .label(label);
        frame.render_widget(gauge, area);
    }

    fn render_gauge3(progress: f64, frame: &mut Frame, area: Rect) {
        let title =
            Self::title_block("Gauge with ratio progress, custom label with style, and unicode");
        let label = Span::styled(
            format!("{:.2}%", progress * 100.0),
            Style::new().red().italic().bold(),
        );
        let gauge = Gauge::default()
            .block(title)
            .gauge_style(Style::default().fg(Color::Yellow))
            .ratio(progress)
            .label(label)
            .use_unicode(true);
        frame.render_widget(gauge, area);
    }

    fn render_gauge4(progress: u16, frame: &mut Frame, area: Rect) {
        let title = Self::title_block("Gauge with percentage progress and label");
        let label = format!("{}/100", progress);
        let gauge = Gauge::default()
            .block(title)
            .gauge_style(Style::new().green().italic())
            .percent(progress)
            .label(label);
        frame.render_widget(gauge, area);
    }

    fn title_block(title: &str) -> Block {
        let title = Title::from(title).alignment(Alignment::Center);
        Block::default().title(title).borders(Borders::TOP)
    }
}

struct Term {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Term {
    pub fn start() -> io::Result<Term> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(Self { terminal })
    }

    pub fn stop() -> io::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn draw(&mut self, frame: impl FnOnce(&mut Frame)) -> Result<()> {
        self.terminal.draw(frame)?;
        Ok(())
    }
}
