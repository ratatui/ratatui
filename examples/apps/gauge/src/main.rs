/// A Ratatui example that demonstrates different types of gauges.
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
use std::sync::LazyLock;
use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::DefaultTerminal;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::palette::tailwind;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, Padding, Paragraph, Widget};

// Colors are resolved once at startup. On terminals that don't support 24-bit color
// (e.g. Apple Terminal.app before build 465) the truecolor tailwind palette looks bad,
// so we fall back to 4-bit/indexed colors. See issue #1972.
static THEME: LazyLock<Theme> = LazyLock::new(Theme::new);

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
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.state != AppState::Quitting {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
            self.update(terminal.size()?.width);
        }
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
        if !event::poll(timeout)? {
            return Ok(());
        }
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char(' ') | KeyCode::Enter => self.start(),
                KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                _ => {}
            }
        }
        Ok(())
    }

    const fn start(&mut self) {
        self.state = AppState::Started;
    }

    const fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl Widget for &App {
    #[expect(clippy::similar_names)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min, Ratio};
        let layout = Layout::vertical([Length(2), Min(0), Length(1)]);
        let [header_area, gauge_area, footer_area] = area.layout(&layout);

        let layout = Layout::vertical([Ratio(1, 4); 4]);
        let [gauge1_area, gauge2_area, gauge3_area, gauge4_area] = gauge_area.layout(&layout);

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
        .fg(THEME.custom_label)
        .render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Press ENTER to start")
        .alignment(Alignment::Center)
        .fg(THEME.custom_label)
        .bold()
        .render(area, buf);
}

impl App {
    fn render_gauge1(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with percentage");
        Gauge::default()
            .block(title)
            .gauge_style(THEME.gauge1)
            .percent(self.progress1)
            .render(area, buf);
    }

    fn render_gauge2(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with ratio and custom label");
        let label = Span::styled(
            format!("{:.1}/100", self.progress2),
            Style::new().italic().bold().fg(THEME.custom_label),
        );
        Gauge::default()
            .block(title)
            .gauge_style(THEME.gauge2)
            .ratio(self.progress2 / 100.0)
            .label(label)
            .render(area, buf);
    }

    fn render_gauge3(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with ratio (no unicode)");
        let label = format!("{:.1}%", self.progress3);
        Gauge::default()
            .block(title)
            .gauge_style(THEME.gauge3)
            .ratio(self.progress3 / 100.0)
            .label(label)
            .render(area, buf);
    }

    fn render_gauge4(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with ratio (unicode)");
        let label = format!("{:.1}%", self.progress3);
        Gauge::default()
            .block(title)
            .gauge_style(THEME.gauge4)
            .ratio(self.progress4 / 100.0)
            .label(label)
            .use_unicode(true)
            .render(area, buf);
    }
}

fn title_block(title: &str) -> Block<'_> {
    let title = Line::from(title).centered();
    Block::new()
        .borders(Borders::NONE)
        .padding(Padding::vertical(1))
        .title(title)
        .fg(THEME.custom_label)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Theme {
    gauge1: Color,
    gauge2: Color,
    gauge3: Color,
    gauge4: Color,
    custom_label: Color,
}

impl Theme {
    fn new() -> Self {
        use tailwind::{BLUE, GREEN, ORANGE, RED, SLATE};

        let is_true_color = Self::is_true_color_supported();
        let color = |true_color, ansi_color| {
            if is_true_color {
                true_color
            } else {
                ansi_color
            }
        };

        // The fallbacks are 4-bit/indexed colors chosen to read reasonably on
        // pre-truecolor terminals. Tune these on a real pre-Tahoe Terminal.app.
        Self {
            gauge1: color(RED.c800, Color::Red),
            gauge2: color(GREEN.c800, Color::Green),
            gauge3: color(BLUE.c800, Color::Blue),
            gauge4: color(ORANGE.c800, Color::Indexed(208)),
            custom_label: color(SLATE.c200, Color::Gray),
        }
    }

    // Checks whether truecolor (24-bit color) is supported in the current terminal.
    //
    // Terminals known *not* to support truecolor:
    // - Apple Terminal.app: all versions before 2.15 (build 465)
    //
    // Environment variables used:
    // - "TERM_PROGRAM": identifies the terminal application in use
    // - "TERM_PROGRAM_VERSION": version number of that terminal application
    fn is_true_color_supported() -> bool {
        let term = std::env::var("TERM_PROGRAM").unwrap_or_default();
        if term == "Apple_Terminal" {
            let term_v = std::env::var("TERM_PROGRAM_VERSION")
                .unwrap_or_default()
                .parse()
                .unwrap_or(0);
            if term_v < 465 {
                return false;
            }
        }
        true
    }
}
