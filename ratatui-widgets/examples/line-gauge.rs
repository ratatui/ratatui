//! # [Ratatui] `LineGauge` example
//!
//! The latest version of this example is available in the [widget examples] folder in the
//! repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [widget examples]: https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use core::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::DefaultTerminal;
use ratatui::buffer::Buffer;
use ratatui::layout::Constraint::{Length, Min};
use ratatui::layout::{Layout, Rect};
use ratatui::style::palette::tailwind;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{LineGauge, Paragraph, Widget};

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

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug, Clone, Copy)]
struct Theme {
    gauge1_filled: Color,
    gauge1_unfilled: Color,
    gauge2_filled: Color,
    gauge2_unfilled: Color,
    gauge3_filled: Color,
    gauge3_unfilled: Color,
}

impl Theme {
    fn new() -> Self {
        if is_true_color_supported() {
            // Original palette
            Self {
                gauge1_filled: tailwind::LIME.c400,
                gauge1_unfilled: tailwind::LIME.c800,
                gauge2_filled: tailwind::CYAN.c400,
                gauge2_unfilled: tailwind::CYAN.c800,
                gauge3_filled: tailwind::BLUE.c400,
                gauge3_unfilled: tailwind::BLUE.c800,
            }
        } else {
            // Fall back on ansi indexed color
            Self {
                gauge1_filled: Color::Indexed(149),
                gauge1_unfilled: Color::Indexed(58),
                gauge2_filled: Color::Indexed(45),
                gauge2_unfilled: Color::Indexed(24),
                gauge3_filled: Color::Indexed(75),
                gauge3_unfilled: Color::Indexed(25),
            }
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct App {
    state: AppState,
    progress_columns: u16,
    progress: f64,
    theme: Theme,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Start,
    Stop,
    Quit,
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.state != AppState::Quit {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
            self.update(terminal.size()?.width);
        }
        Ok(())
    }

    fn update(&mut self, terminal_width: u16) {
        if self.state != AppState::Start {
            return;
        }

        self.progress_columns = (self.progress_columns + 1).clamp(0, terminal_width);
        self.progress = f64::from(self.progress_columns) / f64::from(terminal_width);
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f32(1.0 / 20.0);
        if event::poll(timeout)?
            && let Some(key) = event::read()?.as_key_press_event()
        {
            match key.code {
                KeyCode::Char(' ') => self.toggle_start(),
                KeyCode::Char('r') => self.reset(),
                KeyCode::Char('q') => self.state = AppState::Quit,
                _ => {}
            }
        }
        Ok(())
    }

    fn toggle_start(&mut self) {
        self.state = if self.state == AppState::Start {
            AppState::Stop
        } else {
            AppState::Start
        };
    }

    const fn reset(&mut self) {
        self.progress = 0.0;
        self.progress_columns = 0;
        self.state = AppState::Stop;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Length(3), Min(0)]);
        let [header_area, main_area] = area.layout(&layout);

        let gauges_layout = Layout::vertical([Length(2); 3]);
        let [gauge1_area, gauge4_area, gauge6_area] = main_area.layout(&gauges_layout);

        header(header_area, buf);

        self.render_gauge1(gauge1_area, buf);
        self.render_gauge2(gauge4_area, buf);
        self.render_gauge3(gauge6_area, buf);
    }
}

fn header(area: Rect, buf: &mut Buffer) {
    let [p1_area, p2_area] = area.layout(&Layout::vertical([Length(1), Min(1)]));
    Paragraph::new("LineGauge Example")
        .bold()
        .centered()
        .render(p1_area, buf);

    Paragraph::new("(Press 'SPACE' to start/stop progress, 'r' to reset progress, 'q' to quit)")
        .centered()
        .render(p2_area, buf);
}

impl App {
    fn render_gauge1(&self, area: Rect, buf: &mut Buffer) {
        LineGauge::default()
            .filled_style(Style::default().fg(self.theme.gauge1_filled))
            .unfilled_style(Style::default().fg(self.theme.gauge1_unfilled))
            .ratio(self.progress)
            .render(area, buf);
    }

    fn render_gauge2(&self, area: Rect, buf: &mut Buffer) {
        LineGauge::default()
            .filled_symbol("⣿")
            .unfilled_symbol("⣿")
            .filled_style(Style::default().fg(self.theme.gauge2_filled))
            .unfilled_style(Style::default().fg(self.theme.gauge2_unfilled))
            .ratio(self.progress)
            .render(area, buf);
    }

    fn render_gauge3(&self, area: Rect, buf: &mut Buffer) {
        LineGauge::default()
            .filled_symbol("▰")
            .unfilled_symbol("▱")
            .filled_style(Style::default().fg(self.theme.gauge3_filled))
            .unfilled_style(Style::default().fg(self.theme.gauge3_unfilled))
            .ratio(self.progress)
            .render(area, buf);
    }
}
