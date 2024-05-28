//! # [Ratatui] Canvas example
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

use std::{
    io::{self, stdout, Stdout},
    time::{Duration, Instant},
};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    symbols::Marker,
    terminal::{Frame, Terminal},
    widgets::{
        canvas::{Canvas, Circle, Map, MapResolution, Rectangle},
        Block, Widget,
    },
};

fn main() -> io::Result<()> {
    App::run()
}

struct App {
    x: f64,
    y: f64,
    ball: Circle,
    playground: Rect,
    vx: f64,
    vy: f64,
    tick_count: u64,
    marker: Marker,
}

impl App {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            ball: Circle {
                x: 20.0,
                y: 40.0,
                radius: 10.0,
                color: Color::Yellow,
            },
            playground: Rect::new(10, 10, 200, 100),
            vx: 1.0,
            vy: 1.0,
            tick_count: 0,
            marker: Marker::Dot,
        }
    }

    pub fn run() -> io::Result<()> {
        let mut terminal = init_terminal()?;
        let mut app = Self::new();
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(16);
        loop {
            let _ = terminal.draw(|frame| app.ui(frame));
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Down | KeyCode::Char('j') => app.y += 1.0,
                        KeyCode::Up | KeyCode::Char('k') => app.y -= 1.0,
                        KeyCode::Right | KeyCode::Char('l') => app.x += 1.0,
                        KeyCode::Left | KeyCode::Char('h') => app.x -= 1.0,
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                app.on_tick();
                last_tick = Instant::now();
            }
        }
        restore_terminal()
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
        // only change marker every 180 ticks (3s) to avoid stroboscopic effect
        if (self.tick_count % 180) == 0 {
            self.marker = match self.marker {
                Marker::Dot => Marker::Braille,
                Marker::Braille => Marker::Block,
                Marker::Block => Marker::HalfBlock,
                Marker::HalfBlock => Marker::Bar,
                Marker::Bar => Marker::Dot,
            };
        }
        // bounce the ball by flipping the velocity vector
        let ball = &self.ball;
        let playground = self.playground;
        if ball.x - ball.radius < f64::from(playground.left())
            || ball.x + ball.radius > f64::from(playground.right())
        {
            self.vx = -self.vx;
        }
        if ball.y - ball.radius < f64::from(playground.top())
            || ball.y + ball.radius > f64::from(playground.bottom())
        {
            self.vy = -self.vy;
        }

        self.ball.x += self.vx;
        self.ball.y += self.vy;
    }

    fn ui(&self, frame: &mut Frame) {
        let horizontal =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [map, right] = horizontal.areas(frame.size());
        let [pong, boxes] = vertical.areas(right);

        frame.render_widget(self.map_canvas(), map);
        frame.render_widget(self.pong_canvas(), pong);
        frame.render_widget(self.boxes_canvas(boxes), boxes);
    }

    fn map_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered().title("World"))
            .marker(self.marker)
            .paint(|ctx| {
                ctx.draw(&Map {
                    color: Color::Green,
                    resolution: MapResolution::High,
                });
                ctx.print(self.x, -self.y, "You are here".yellow());
            })
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
    }

    fn pong_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered().title("Pong"))
            .marker(self.marker)
            .paint(|ctx| {
                ctx.draw(&self.ball);
            })
            .x_bounds([10.0, 210.0])
            .y_bounds([10.0, 110.0])
    }

    fn boxes_canvas(&self, area: Rect) -> impl Widget {
        let left = 0.0;
        let right = f64::from(area.width);
        let bottom = 0.0;
        let top = f64::from(area.height).mul_add(2.0, -4.0);
        Canvas::default()
            .block(Block::bordered().title("Rects"))
            .marker(self.marker)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(|ctx| {
                for i in 0..=11 {
                    ctx.draw(&Rectangle {
                        x: f64::from(i * i + 3 * i) / 2.0 + 2.0,
                        y: 2.0,
                        width: f64::from(i),
                        height: f64::from(i),
                        color: Color::Red,
                    });
                    ctx.draw(&Rectangle {
                        x: f64::from(i * i + 3 * i) / 2.0 + 2.0,
                        y: 21.0,
                        width: f64::from(i),
                        height: f64::from(i),
                        color: Color::Blue,
                    });
                }
                for i in 0..100 {
                    if i % 10 != 0 {
                        ctx.print(f64::from(i) + 1.0, 0.0, format!("{i}", i = i % 10));
                    }
                    if i % 2 == 0 && i % 10 != 0 {
                        ctx.print(0.0, f64::from(i), format!("{i}", i = i % 10));
                    }
                }
            })
    }
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
