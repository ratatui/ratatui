//! A Ratatui example that demonstrates 3D perspective projection in the terminal.
//!
//! This example visualizes an implied volatility surface as an interactive 3D wireframe
//! using Braille canvas for high-resolution rendering. It demonstrates rotation matrices,
//! perspective projection, and real-time animation.

mod display;
mod volatility;

use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use display::Surface3D;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use volatility::VolatilityEngine;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

struct App {
    paused: bool,
    time: f64,
    fps: usize,
    frame_count: usize,
    vol_engine: VolatilityEngine,
    surface_3d: Surface3D,
}

impl App {
    fn new() -> Self {
        Self {
            paused: false,
            time: 0.0,
            fps: 0,
            frame_count: 0,
            vol_engine: VolatilityEngine::new(),
            surface_3d: Surface3D::new(),
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(33); // 30 FPS
        let mut last_tick = Instant::now();
        let mut fps_timer = Instant::now();

        loop {
            terminal.draw(|frame| self.render(frame))?;

            self.frame_count += 1;
            if fps_timer.elapsed() >= Duration::from_secs(1) {
                self.fps = self.frame_count;
                self.frame_count = 0;
                fps_timer = Instant::now();
            }

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if self.handle_key(key.code, key.modifiers) {
                        break;
                    }
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
        self.time += 0.05;
        self.vol_engine.update(self.time);
    }

    fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) -> bool {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => return true,
            KeyCode::Char(' ') => self.paused = !self.paused,
            KeyCode::Char('r') if modifiers.contains(KeyModifiers::CONTROL) => {
                self.time = 0.0;
                self.vol_engine = VolatilityEngine::new();
            }
            KeyCode::Up => self.surface_3d.rotate_x(0.1),
            KeyCode::Down => self.surface_3d.rotate_x(-0.1),
            KeyCode::Left => self.surface_3d.rotate_z(0.1),
            KeyCode::Right => self.surface_3d.rotate_z(-0.1),
            KeyCode::Char('z') => self.surface_3d.zoom(1.1),
            KeyCode::Char('x') => self.surface_3d.zoom(0.9),
            KeyCode::Char('p') => self.surface_3d.cycle_palette(),
            _ => {}
        }
        false
    }

    fn render(&self, frame: &mut Frame) {
        let chunks = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

        self.render_header(frame, chunks[0]);
        self.render_surface(frame, chunks[1]);
        Self::render_footer(frame, chunks[2]);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = Line::from(vec![
            Span::styled(
                "3D Volatility Surface Visualizer",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(" | FPS: {} | ", self.fps)),
            if self.paused {
                Span::styled("PAUSED", Style::default().fg(Color::Red))
            } else {
                Span::styled("LIVE", Style::default().fg(Color::Green))
            },
        ]);

        let header = Paragraph::new(vec![title])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .style(Style::default().bg(Color::Black));

        frame.render_widget(header, area);
    }

    fn render_surface(&self, frame: &mut Frame, area: Rect) {
        let surface_data = self.vol_engine.get_surface();
        self.surface_3d.render(frame, area, surface_data, self.time);
    }

    fn render_footer(frame: &mut Frame, area: Rect) {
        let controls = Line::from(vec![
            Span::styled("↑↓←→", Style::default().fg(Color::Yellow)),
            Span::raw(" Rotate | "),
            Span::styled("Z/X", Style::default().fg(Color::Yellow)),
            Span::raw(" Zoom | "),
            Span::styled("P", Style::default().fg(Color::Yellow)),
            Span::raw(" Palette | "),
            Span::styled("Space", Style::default().fg(Color::Cyan)),
            Span::raw(" Pause | "),
            Span::styled("Ctrl+R", Style::default().fg(Color::Cyan)),
            Span::raw(" Reset | "),
            Span::styled("Q", Style::default().fg(Color::Red)),
            Span::raw(" Quit"),
        ]);

        let footer = Paragraph::new(vec![controls])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .style(Style::default().bg(Color::Black).fg(Color::Gray));

        frame.render_widget(footer, area);
    }
}
