//! A Ratatui example that demonstrates 3D perspective projection in the terminal.
//!
//! This example visualizes an implied volatility surface as an interactive 3D wireframe
//! using Braille canvas for high-resolution rendering. It demonstrates rotation matrices,
//! perspective projection, and real-time animation.

mod display;
mod volatility;

use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEventKind, KeyModifiers};
use display::{Surface3D, VolatilitySurface};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::palette::tailwind::SLATE;
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::{DefaultTerminal, Frame};
use volatility::VolatilityEngine;

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::new().run(terminal))
}

struct App {
    paused: bool,
    fps_counter: FpsCounter,
    vol_engine: VolatilityEngine,
    surface_3d: Surface3D,
}

struct FpsCounter {
    frame_count: usize,
    last_instant: Instant,
    fps: f64,
}

impl FpsCounter {
    fn new() -> Self {
        Self {
            frame_count: 0,
            last_instant: Instant::now(),
            fps: 0.0,
        }
    }

    fn update(&mut self) {
        self.frame_count += 1;
        let elapsed = self.last_instant.elapsed();
        // Update FPS calculation every second
        if elapsed >= Duration::from_secs(1) {
            self.fps = self.frame_count as f64 / elapsed.as_secs_f64();
            self.frame_count = 0;
            self.last_instant = Instant::now();
        }
    }

    const fn fps(&self) -> usize {
        self.fps as usize
    }
}

impl App {
    fn new() -> Self {
        Self {
            paused: false,
            fps_counter: FpsCounter::new(),
            vol_engine: VolatilityEngine::new(),
            surface_3d: Surface3D::new(),
        }
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_secs_f64(1.0 / 50.0); // 50 FPS (better for gifs)
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|frame| self.render(frame))?;
            self.fps_counter.update();

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                // Only handle key press events, not repeat or release
                if let event::Event::Key(key) = event::read()?
                    && key.kind == KeyEventKind::Press
                    && self.handle_key(key.code, key.modifiers)
                {
                    break;
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if !self.paused {
                    self.update();
                }
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    fn update(&mut self) {
        self.vol_engine.update();
    }

    fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) -> bool {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => return true,
            KeyCode::Char(' ') => self.paused = !self.paused,
            KeyCode::Char('r') if modifiers.contains(KeyModifiers::CONTROL) => {
                self.vol_engine.reset();
            }
            KeyCode::Up | KeyCode::Char('k') => self.surface_3d.rotate_x(0.1),
            KeyCode::Down | KeyCode::Char('j') => self.surface_3d.rotate_x(-0.1),
            KeyCode::Left | KeyCode::Char('h') => self.surface_3d.rotate_z(0.1),
            KeyCode::Right | KeyCode::Char('l') => self.surface_3d.rotate_z(-0.1),
            KeyCode::Char('z') => self.surface_3d.zoom(1.1),
            KeyCode::Char('x') => self.surface_3d.zoom(0.9),
            KeyCode::Char('p') => self.surface_3d.cycle_palette(),
            _ => {}
        }
        false
    }

    fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.area());

        self.render_header(frame, chunks[0]);
        self.render_surface(frame, chunks[1]);
        Self::render_footer(frame, chunks[2]);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let status = if self.paused { "Paused" } else { "Live" };
        let palette_name = self.surface_3d.palette_name();
        let title = format!(
            "volatility-surface - Status: {}, Palette: {}, FPS: {}",
            status,
            palette_name,
            self.fps_counter.fps()
        );

        let header = Paragraph::new(title)
            .centered()
            .style(Style::default().bg(SLATE.c100).fg(SLATE.c800));

        frame.render_widget(header, area);
    }

    fn render_surface(&mut self, frame: &mut Frame, area: Rect) {
        let surface_data = self.vol_engine.get_surface();
        frame.render_stateful_widget(
            VolatilitySurface::new(surface_data),
            area,
            &mut self.surface_3d,
        );
    }

    fn render_footer(frame: &mut Frame, area: Rect) {
        let controls = Line::from(vec![
            "↑↓←→/hjkl".cyan(),
            " Rotate | ".into(),
            "zx".cyan(),
            " Zoom | ".into(),
            "p".cyan(),
            " Palette | ".into(),
            "space".cyan(),
            " Pause | ".into(),
            "ctrl-r".cyan(),
            " Reset | ".into(),
            "q".cyan(),
            " Quit".into(),
        ]);

        let footer = Paragraph::new(controls)
            .centered()
            .style(Style::default().fg(SLATE.c400).bg(SLATE.c950));

        frame.render_widget(footer, area);
    }
}
