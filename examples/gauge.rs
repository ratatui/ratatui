use std::{
    error::Error,
    io::{self, stdout},
    time::Duration,
};

use anyhow::Result;
use color_eyre::config::HookBuilder;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    style::palette::tailwind,
    widgets::{block::Title, *},
};

const GAUGE1_COLOR: Color = tailwind::RED.c800;
const GAUGE2_COLOR: Color = tailwind::GREEN.c800;
const GAUGE3_COLOR: Color = tailwind::BLUE.c800;
const CUSTOM_LABEL_COLOR: Color = tailwind::SLATE.c200;
const GAUGE4_COLOR: Color = tailwind::ORANGE.c800;

struct App {
    state: AppState,
    start: bool,
}

#[derive(Debug, Default, Clone, Copy)]
struct AppState {
    progress1: u16,
    progress2: u16,
    progress3: f64,
    progress4: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    init_error_hooks()?;
    let terminal = init_terminal()?;

    // create app and run it
    App::new().run(terminal)?;

    restore_terminal()?;

    Ok(())
}

fn init_error_hooks() -> color_eyre::Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info)
    }));
    Ok(())
}

fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> color_eyre::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

impl App {
    fn new() -> App {
        App {
            state: AppState::default(),
            start: false,
        }
    }

    fn update(&mut self) {
        self.state.progress1 = (self.state.progress1 + 4).min(100);
        self.state.progress2 = (self.state.progress2 + 3).min(100);
        self.state.progress3 = (self.state.progress3 + 0.02).min(1.0);
        self.state.progress4 = (self.state.progress4 + 1).min(100);
    }
}

impl App {
    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<()> {
        let timeout = Duration::from_secs_f32(1.0 / 10.0);

        loop {
            self.draw(&mut terminal)?;
            if self.start {
                self.update();
            }

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        use KeyCode::*;
                        match key.code {
                            Char('q') | Esc => return Ok(()),
                            Enter => self.start = true,
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let [header_area, gauge_area, footer_area] = area.split(&layout);

        let layout = Layout::vertical([Constraint::Ratio(1, 4); 4]);
        let [gauge1_area, gauge2_area, gauge3_area, gauge4_area] = gauge_area.split(&layout);

        self.render_header(header_area, buf);

        self.render_gauge1(gauge1_area, buf, self.state.progress1);
        self.render_gauge2(gauge2_area, buf, self.state.progress2);
        self.render_gauge3(gauge3_area, buf, self.state.progress3);
        self.render_gauge4(gauge4_area, buf, self.state.progress4);

        self.render_footer(footer_area, buf);
    }
}

impl App {
    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Ratatui Gauge Example")
            .bold()
            .alignment(Alignment::Center)
            .fg(CUSTOM_LABEL_COLOR)
            .render(area, buf);
    }

    fn render_gauge1(&self, area: Rect, buf: &mut Buffer, progress: u16) {
        let title = Self::title_block("Gauge with percentage progress");
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE1_COLOR)
            .percent(progress)
            .render(area, buf);
    }

    fn render_gauge2(&self, area: Rect, buf: &mut Buffer, progress: u16) {
        let title = Self::title_block("Gauge with percentage progress and custom label");
        let label = format!("{}/100", progress);
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE2_COLOR)
            .percent(progress)
            .label(label)
            .render(area, buf);
    }

    fn render_gauge3(&self, area: Rect, buf: &mut Buffer, progress: f64) {
        let title =
            Self::title_block("Gauge with ratio progress, custom label with style, and unicode");
        let label = Span::styled(
            format!("{:.2}%", progress * 100.0),
            Style::new().italic().bold().fg(CUSTOM_LABEL_COLOR),
        );
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE3_COLOR)
            .ratio(progress)
            .label(label)
            .use_unicode(true)
            .render(area, buf);
    }

    fn render_gauge4(&self, area: Rect, buf: &mut Buffer, progress: u16) {
        let title = Self::title_block("Gauge with percentage progress and label");
        let label = format!("{}/100", progress);
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE4_COLOR)
            .percent(progress)
            .label(label)
            .render(area, buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Press ENTER to start")
            .alignment(Alignment::Center)
            .fg(CUSTOM_LABEL_COLOR)
            .bold()
            .render(area, buf);
    }

    fn title_block(title: &str) -> Block {
        let title = Title::from(title).alignment(Alignment::Center);
        Block::default()
            .title(title)
            .borders(Borders::NONE)
            .fg(CUSTOM_LABEL_COLOR)
            .padding(Padding::vertical(1))
    }
}
