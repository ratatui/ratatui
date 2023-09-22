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
use palette::{
    convert::{FromColorUnclamped, IntoColorUnclamped},
    Okhsv, Srgb,
};
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
        Self::render_title(layout[0], buf);
        Self::render_colors(layout[1], buf);
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
    fn render_colors(area: Rect, buf: &mut Buffer) {
        for (xi, x) in (area.left()..area.right()).enumerate() {
            for (yi, y) in (area.top()..area.bottom()).enumerate() {
                let hue = xi as f32 * 360.0 / area.width as f32;

                let value_fg = (yi as f32) / (area.height as f32 - 0.5);
                let fg = Okhsv::<f32>::new(hue, Okhsv::max_saturation(), value_fg);
                let fg: Srgb = fg.into_color_unclamped();
                let fg: Srgb<u8> = fg.into_format();
                let fg = Color::Rgb(fg.red, fg.green, fg.blue);

                let value_bg = (yi as f32 + 0.5) / (area.height as f32 - 0.5);
                let bg = Okhsv::new(hue, Okhsv::max_saturation(), value_bg);
                let bg = Srgb::<f32>::from_color_unclamped(bg);
                let bg: Srgb<u8> = bg.into_format();
                let bg = Color::Rgb(bg.red, bg.green, bg.blue);

                buf.get_mut(x, y).set_char('▀').set_fg(fg).set_bg(bg);
            }
        }
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
