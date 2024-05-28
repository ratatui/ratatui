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

use std::{io::stdout, time::Duration};

use color_eyre::{config::HookBuilder, Result};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    buffer::Buffer,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Alignment, Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Style, Stylize},
    terminal::Terminal,
    text::Span,
    widgets::{block::Title, Block, Borders, Gauge, Padding, Paragraph, Widget},
};

const GAUGE1_COLOR: Color = tailwind::RED.c800;
const GAUGE2_COLOR: Color = tailwind::GREEN.c800;
const GAUGE3_COLOR: Color = tailwind::BLUE.c800;
const GAUGE4_COLOR: Color = tailwind::ORANGE.c800;
const CUSTOM_LABEL_COLOR: Color = tailwind::SLATE.c200;

#[derive(Debug, Default, Clone, Copy)]
struct App {
    state: AppState,
    progress_columns: u16,
    progress1: u16,
    progress2: f64,
    progress3: f64,
    progress4: f64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Started,
    Quitting,
}

fn main() -> Result<()> {
    init_error_hooks()?;
    let terminal = init_terminal()?;
    App::default().run(terminal)?;
    restore_terminal()?;
    Ok(())
}

impl App {
    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        while self.state != AppState::Quitting {
            self.draw(&mut terminal)?;
            self.handle_events()?;
            self.update(terminal.size()?.width);
        }
        Ok(())
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        Ok(())
    }

    fn update(&mut self, terminal_width: u16) {
        if self.state != AppState::Started {
            return;
        }

        // progress1 and progress2 help show the difference between ratio and percentage measuring
        // the same thing, but converting to either a u16 or f64. Effectively, we're showing the
        // difference between how a continuous gauge acts for floor and rounded values.
        self.progress_columns = (self.progress_columns + 1).clamp(0, terminal_width);
        self.progress1 = self.progress_columns * 100 / terminal_width;
        self.progress2 = f64::from(self.progress_columns) * 100.0 / f64::from(terminal_width);

        // progress3 and progress4 similarly show the difference between unicode and non-unicode
        // gauges measuring the same thing.
        self.progress3 = (self.progress3 + 0.1).clamp(40.0, 100.0);
        self.progress4 = (self.progress4 + 0.1).clamp(40.0, 100.0);
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f32(1.0 / 20.0);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(' ') | KeyCode::Enter => self.start(),
                        KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn start(&mut self) {
        self.state = AppState::Started;
    }

    fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl Widget for &App {
    #[allow(clippy::similar_names)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min, Ratio};
        let layout = Layout::vertical([Length(2), Min(0), Length(1)]);
        let [header_area, gauge_area, footer_area] = layout.areas(area);

        let layout = Layout::vertical([Ratio(1, 4); 4]);
        let [gauge1_area, gauge2_area, gauge3_area, gauge4_area] = layout.areas(gauge_area);

        render_header(header_area, buf);
        render_footer(footer_area, buf);

        self.render_gauge1(gauge1_area, buf);
        self.render_gauge2(gauge2_area, buf);
        self.render_gauge3(gauge3_area, buf);
        self.render_gauge4(gauge4_area, buf);
    }
}

fn render_header(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Ratatui Gauge Example")
        .bold()
        .alignment(Alignment::Center)
        .fg(CUSTOM_LABEL_COLOR)
        .render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Press ENTER to start")
        .alignment(Alignment::Center)
        .fg(CUSTOM_LABEL_COLOR)
        .bold()
        .render(area, buf);
}

impl App {
    fn render_gauge1(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with percentage");
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE1_COLOR)
            .percent(self.progress1)
            .render(area, buf);
    }

    fn render_gauge2(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with ratio and custom label");
        let label = Span::styled(
            format!("{:.1}/100", self.progress2),
            Style::new().italic().bold().fg(CUSTOM_LABEL_COLOR),
        );
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE2_COLOR)
            .ratio(self.progress2 / 100.0)
            .label(label)
            .render(area, buf);
    }

    fn render_gauge3(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with ratio (no unicode)");
        let label = format!("{:.1}%", self.progress3);
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE3_COLOR)
            .ratio(self.progress3 / 100.0)
            .label(label)
            .render(area, buf);
    }

    fn render_gauge4(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with ratio (unicode)");
        let label = format!("{:.1}%", self.progress3);
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE4_COLOR)
            .ratio(self.progress4 / 100.0)
            .label(label)
            .use_unicode(true)
            .render(area, buf);
    }
}

fn title_block(title: &str) -> Block {
    let title = Title::from(title).alignment(Alignment::Center);
    Block::new()
        .borders(Borders::NONE)
        .padding(Padding::vertical(1))
        .title(title)
        .fg(CUSTOM_LABEL_COLOR)
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
        panic(info);
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
