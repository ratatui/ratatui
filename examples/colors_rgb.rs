/// This example shows the full range of RGB colors that can be displayed in the terminal.
///
/// Requires a terminal that supports 24-bit color (true color) and unicode.
use std::{
    error::Error,
    io::{stdout, Stdout},
    rc::Rc,
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    install_panic_hook();
    App::new()?.run()
}

struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    should_quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
            should_quit: false,
        })
    }

    pub fn run(mut self) -> Result<()> {
        init_terminal()?;
        self.terminal.clear()?;
        while !self.should_quit {
            self.draw()?;
            self.handle_events()?;
        }
        restore_terminal()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            frame.render_widget(RgbColors, frame.size());
        })?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.should_quit = true;
                };
            }
        }
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = restore_terminal();
    }
}

struct RgbColors;

impl Widget for RgbColors {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Self::layout(area);
        let rgb_colors = Self::create_rgb_color_grid(area.width, area.height * 2);
        Self::render_title(layout[0], buf);
        Self::render_colors(layout[1], buf, rgb_colors);
    }
}

impl RgbColors {
    fn layout(area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Min(0)])
            .split(area)
    }

    fn render_title(area: Rect, buf: &mut Buffer) {
        Paragraph::new("colors_rgb example. Press q to quit")
            .dark_gray()
            .alignment(Alignment::Center)
            .render(area, buf);
    }

    /// Render a colored grid of half block characters (`"▀"`) each with a different RGB color.
    fn render_colors(area: Rect, buf: &mut Buffer, rgb_colors: Vec<Vec<Color>>) {
        for (x, column) in (area.left()..area.right()).zip(rgb_colors.iter()) {
            for (y, (fg, bg)) in (area.top()..area.bottom()).zip(column.iter().tuples()) {
                let cell = buf.get_mut(x, y);
                cell.fg = *fg;
                cell.bg = *bg;
                cell.symbol = "▀".into();
            }
        }
    }

    /// Generate a smooth grid of colors
    ///
    /// Red ranges from 0 to 255 across the x axis. Green ranges from 0 to 255 across the y axis.
    /// Blue repeats every 32 pixels in both directions, but flipped every 16 pixels so that it
    /// doesn't transition sharply from light to dark.
    ///
    /// The result stored in a 2d vector of colors with the x axis as the first dimension, and the
    /// y axis the second dimension.
    fn create_rgb_color_grid(width: u16, height: u16) -> Vec<Vec<Color>> {
        let mut result = vec![];
        for x in 0..width {
            let mut column = vec![];
            for y in 0..height {
                // flip both axes every 16 pixels. E.g. [0, 1, ... 15, 15, ... 1, 0]
                let yy = if (y % 32) < 16 { y % 32 } else { 31 - y % 32 };
                let xx = if (x % 32) < 16 { x % 32 } else { 31 - x % 32 };
                let r = (256 * x / width) as u8;
                let g = (256 * y / height) as u8;
                let b = (yy * 16 + xx) as u8;
                column.push(Color::Rgb(r, g, b))
            }
            result.push(column);
        }
        result
    }
}

/// Install a panic hook that restores the terminal before panicking.
fn install_panic_hook() {
    better_panic::install();
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        prev_hook(info);
    }));
}

fn init_terminal() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
