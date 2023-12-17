/// This example shows the full range of RGB colors that can be displayed in the terminal.
///
/// Requires a terminal that supports 24-bit color (true color) and unicode.
use std::{
    io::stdout,
    time::{Duration, Instant},
};

use color_eyre::config::HookBuilder;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use palette::{convert::FromColorUnclamped, Okhsv, Srgb};
use ratatui::{prelude::*, widgets::*};

fn main() -> color_eyre::Result<()> {
    App::run()
}

#[derive(Debug, Default)]
struct App {
    should_quit: bool,
    // a 2d vec of the colors to render, calculated when the size changes as this is expensive
    // to calculate every frame
    colors: Vec<Vec<Color>>,
    last_size: Rect,
    fps: Fps,
    frame_count: usize,
}

#[derive(Debug)]
struct Fps {
    frame_count: usize,
    last_instant: Instant,
    fps: Option<f32>,
}

struct AppWidget<'a> {
    title: Paragraph<'a>,
    fps_widget: FpsWidget<'a>,
    rgb_colors_widget: RgbColorsWidget<'a>,
}

struct FpsWidget<'a> {
    fps: &'a Fps,
}

struct RgbColorsWidget<'a> {
    /// The colors to render - should be double the height of the area
    colors: &'a Vec<Vec<Color>>,
    /// the number of elapsed frames that have passed - used to animate the colors
    frame_count: usize,
}

impl App {
    pub fn run() -> color_eyre::Result<()> {
        install_panic_hook()?;

        let mut terminal = init_terminal()?;
        let mut app = Self::default();

        while !app.should_quit {
            app.tick();
            terminal.draw(|frame| {
                let size = frame.size();
                app.setup_colors(size);
                frame.render_widget(AppWidget::new(&app), size);
            })?;
            app.handle_events()?;
        }
        restore_terminal()?;
        Ok(())
    }

    fn tick(&mut self) {
        self.frame_count += 1;
        self.fps.tick();
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        if event::poll(Duration::from_secs_f32(1.0 / 60.0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.should_quit = true;
                };
            }
        }
        Ok(())
    }

    fn setup_colors(&mut self, size: Rect) {
        // only update the colors if the size has changed since the last time we rendered
        if self.last_size.width == size.width && self.last_size.height == size.height {
            return;
        }
        self.last_size = size;
        let Rect { width, height, .. } = size;
        // double the height because each screen row has two rows of half block pixels
        let height = height * 2;
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

impl Fps {
    fn tick(&mut self) {
        self.frame_count += 1;
        let elapsed = self.last_instant.elapsed();
        // update the fps every second, but only if we've rendered at least 2 frames (to avoid
        // noise in the fps calculation)
        if elapsed > Duration::from_secs(1) && self.frame_count > 2 {
            self.fps = Some(self.frame_count as f32 / elapsed.as_secs_f32());
            self.frame_count = 0;
            self.last_instant = Instant::now();
        }
    }
}

impl Default for Fps {
    fn default() -> Self {
        Self {
            frame_count: 0,
            last_instant: Instant::now(),
            fps: None,
        }
    }
}

impl<'a> AppWidget<'a> {
    fn new(app: &'a App) -> Self {
        let title =
            Paragraph::new("colors_rgb example. Press q to quit").alignment(Alignment::Center);
        Self {
            title,
            fps_widget: FpsWidget { fps: &app.fps },
            rgb_colors_widget: RgbColorsWidget {
                colors: &app.colors,
                frame_count: app.frame_count,
            },
        }
    }
}

impl Widget for AppWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(area);
        let title_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(8)])
            .split(main_layout[0]);

        self.title.render(title_layout[0], buf);
        self.fps_widget.render(title_layout[1], buf);
        self.rgb_colors_widget.render(main_layout[1], buf);
    }
}

impl Widget for RgbColorsWidget<'_> {
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

impl<'a> Widget for FpsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(fps) = self.fps.fps {
            let text = format!("{:.1} fps", fps);
            Paragraph::new(text).render(area, buf);
        }
    }
}

/// Install a panic hook that restores the terminal before panicking.
fn install_panic_hook() -> color_eyre::Result<()> {
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
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    Ok(terminal)
}

fn restore_terminal() -> color_eyre::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
