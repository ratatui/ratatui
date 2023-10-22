/// This example shows the full range of RGB colors that can be displayed in the terminal.
///
/// Requires a terminal that supports 24-bit color (true color) and unicode.
///
use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use palette::{convert::FromColorUnclamped, Okhsv, Srgb};
use ratatui::{prelude::*, widgets::*};

fn main() -> Result<()> {
    // stdout runs ~ 2x faster than stderr on a 2023 Macbook Pro M2 Max (~22-26fps vs ~11-12fps)
    App::run(stdout())
}

struct App {
    should_quit: bool,
    colors: Vec<Vec<Color>>,
    frame_count: usize,
    last_fps_frame_count: usize,
    last_fps_instant: Instant,
    fps: f64,
}

impl App {
    fn new() -> Result<Self> {
        Ok(Self {
            should_quit: false,
            colors: vec![],
            frame_count: 0,
            last_fps_frame_count: 0,
            last_fps_instant: Instant::now(),
            fps: 0.0,
        })
    }

    pub fn run<W: Write>(writer: W) -> Result<()> {
        install_panic_hook();
        init_terminal()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(writer))?;
        let size = terminal.size()?;
        let mut app = Self::new()?;
        app.setup_colors(size.width, size.height * 2);
        terminal.clear()?;
        while !app.should_quit {
            terminal.draw(|frame| {
                app.render(frame);
            })?;
            app.handle_events()?;
            app.frame_count += 1;
        }
        restore_terminal()?;
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(frame.size());
        self.calculate_fps();
        let title = format!(
            "colors_rgb example. Press q to quit. {fps:.2} fps",
            fps = self.fps
        );
        let title = Paragraph::new(title).alignment(Alignment::Center);
        let rgb_colors = RgbColors {
            colors: &self.colors,
            frame_count: self.frame_count,
        };
        frame.render_widget(title, layout[0]);
        frame.render_widget(rgb_colors, layout[1]);
    }

    fn calculate_fps(&mut self) {
        let elapsed_frames = self.frame_count - self.last_fps_frame_count;
        let elapsed_duration = self.last_fps_instant.elapsed();
        // avoid noise in the FPS calculation by only updating it if at least 3 frames have passed
        if elapsed_frames > 2 && elapsed_duration > Duration::from_secs(1) {
            self.fps = elapsed_frames as f64 / elapsed_duration.as_secs_f64();
            self.last_fps_frame_count = self.frame_count;
            self.last_fps_instant = Instant::now();
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(1))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        self.should_quit = true;
                    };
                }
                Event::Resize(width, height) => {
                    self.setup_colors(width, height * 2);
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn setup_colors(&mut self, width: u16, height: u16) {
        self.colors.clear();
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                let hue = x as f32 * 360.0 / width as f32;
                let value = (height - y) as f32 / height as f32;
                let saturation = Okhsv::max_saturation();
                let color = Okhsv::new(hue, saturation, value);
                let color = Srgb::<f32>::from_color_unclamped(color);
                let color: Srgb<u8> = color.into_format();
                let color = Color::Rgb(color.red, color.green, color.blue);
                row.push(color);
            }
            self.colors.push(row);
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = restore_terminal();
    }
}

struct RgbColors<'a> {
    // The colors to render - should be double the height of the area
    colors: &'a Vec<Vec<Color>>,
    // the number of elapsed frames that have passed - used to animate the colors
    frame_count: usize,
}

impl Widget for RgbColors<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = self.colors;
        for (xi, x) in (area.left()..area.right()).enumerate() {
            // animate the colors by shifting the x index by the frame number
            let xi = (xi + self.frame_count) % (area.width as usize);
            for (yi, y) in (area.top()..area.bottom()).enumerate() {
                let fg = colors[yi * 2][xi];
                let bg = colors[yi * 2 + 1][xi];
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
