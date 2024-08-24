//! # [Ratatui] Paragraph example
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
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use std::{
    io::{self},
    time::{Duration, Instant},
};

use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Masked, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
    DefaultTerminal,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
struct App {
    should_exit: bool,
    scroll: u16,
    last_tick: Instant,
}

impl App {
    /// The duration between each tick.
    const TICK_RATE: Duration = Duration::from_millis(250);

    /// Create a new instance of the app.
    fn new() -> Self {
        Self {
            should_exit: false,
            scroll: 0,
            last_tick: Instant::now(),
        }
    }

    /// Run the app until the user exits.
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
            if self.last_tick.elapsed() >= Self::TICK_RATE {
                self.on_tick();
                self.last_tick = Instant::now();
            }
        }
        Ok(())
    }

    /// Handle events from the terminal.
    fn handle_events(&mut self) -> io::Result<()> {
        let timeout = Self::TICK_RATE.saturating_sub(self.last_tick.elapsed());
        while event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.should_exit = true;
                }
            }
        }
        Ok(())
    }

    /// Update the app state on each tick.
    fn on_tick(&mut self) {
        self.scroll = (self.scroll + 1) % 10;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let areas = Layout::vertical([Constraint::Max(9); 4]).split(area);
        Paragraph::new(create_lines(area))
            .block(title_block("Default alignment (Left), no wrap"))
            .gray()
            .render(areas[0], buf);
        Paragraph::new(create_lines(area))
            .block(title_block("Default alignment (Left), with wrap"))
            .gray()
            .wrap(Wrap { trim: true })
            .render(areas[1], buf);
        Paragraph::new(create_lines(area))
            .block(title_block("Right alignment, with wrap"))
            .gray()
            .right_aligned()
            .wrap(Wrap { trim: true })
            .render(areas[2], buf);
        Paragraph::new(create_lines(area))
            .block(title_block("Center alignment, with wrap, with scroll"))
            .gray()
            .centered()
            .wrap(Wrap { trim: true })
            .scroll((self.scroll, 0))
            .render(areas[3], buf);
    }
}

/// Create a bordered block with a title.
fn title_block(title: &str) -> Block {
    Block::bordered()
        .gray()
        .title(title.bold().into_centered_line())
}

/// Create some lines to display in the paragraph.
fn create_lines(area: Rect) -> Vec<Line<'static>> {
    let short_line = "A long line to demonstrate line wrapping. ";
    let long_line = short_line.repeat(usize::from(area.width) / short_line.len() + 4);
    let mut styled_spans = vec![];
    for span in [
        "Styled".blue(),
        "Spans".red().on_white(),
        "Bold".bold(),
        "Italic".italic(),
        "Underlined".underlined(),
        "Strikethrough".crossed_out(),
    ] {
        styled_spans.push(span);
        styled_spans.push(" ".into());
    }
    vec![
        Line::raw("Unstyled Line"),
        Line::raw("Styled Line").black().on_red().bold().italic(),
        Line::from(styled_spans),
        Line::from(long_line.green().italic()),
        Line::from_iter([
            "Masked text: ".into(),
            Span::styled(Masked::new("my secret password", '*'), Color::Red),
        ]),
    ]
}
