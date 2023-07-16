use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{canvas::*, *},
};

struct App {
    x: f64,
    y: f64,
    ball: Rectangle,
    playground: Rect,
    vx: f64,
    vy: f64,
    dir_x: bool,
    dir_y: bool,
    tick_count: u64,
    marker: Marker,
}

impl App {
    fn new() -> App {
        App {
            x: 0.0,
            y: 0.0,
            ball: Rectangle {
                x: 10.0,
                y: 30.0,
                width: 10.0,
                height: 10.0,
                color: Color::Yellow,
            },
            playground: Rect::new(10, 10, 100, 100),
            vx: 1.0,
            vy: 1.0,
            dir_x: true,
            dir_y: true,
            tick_count: 0,
            marker: Marker::Dot,
        }
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
        // only change marker every 4 ticks (1s) to avoid stroboscopic effect
        if (self.tick_count % 4) == 0 {
            self.marker = match self.marker {
                Marker::Dot => Marker::Block,
                Marker::Block => Marker::Bar,
                Marker::Bar => Marker::Braille,
                Marker::Braille => Marker::Dot,
            };
        }
        if self.ball.x < self.playground.left() as f64
            || self.ball.x + self.ball.width > self.playground.right() as f64
        {
            self.dir_x = !self.dir_x;
        }
        if self.ball.y < self.playground.top() as f64
            || self.ball.y + self.ball.height > self.playground.bottom() as f64
        {
            self.dir_y = !self.dir_y;
        }

        if self.dir_x {
            self.ball.x += self.vx;
        } else {
            self.ball.x -= self.vx;
        }

        if self.dir_y {
            self.ball.y += self.vy;
        } else {
            self.ball.y -= self.vy
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Down => {
                        app.y += 1.0;
                    }
                    KeyCode::Up => {
                        app.y -= 1.0;
                    }
                    KeyCode::Right => {
                        app.x += 1.0;
                    }
                    KeyCode::Left => {
                        app.x -= 1.0;
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("World"))
        .marker(app.marker)
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::White,
                resolution: MapResolution::High,
            });
            ctx.print(app.x, -app.y, "You are here".yellow());
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    f.render_widget(canvas, chunks[0]);
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Pong"))
        .marker(app.marker)
        .paint(|ctx| {
            ctx.draw(&app.ball);
        })
        .x_bounds([10.0, 110.0])
        .y_bounds([10.0, 110.0]);
    f.render_widget(canvas, chunks[1]);
}
