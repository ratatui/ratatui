//! # [Ratatui] Sparkline example
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

use std::time::{Duration, Instant};

use color_eyre::Result;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Sparkline},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    signal: RandomSignal,
    data1: Vec<u64>,
    data2: Vec<u64>,
    data3: Vec<u64>,
}

#[derive(Clone)]
struct RandomSignal {
    distribution: Uniform<u64>,
    rng: ThreadRng,
}

impl RandomSignal {
    fn new(lower: u64, upper: u64) -> Self {
        Self {
            distribution: Uniform::new(lower, upper),
            rng: rand::thread_rng(),
        }
    }
}

impl Iterator for RandomSignal {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(self.distribution.sample(&mut self.rng))
    }
}

impl App {
    fn new() -> Self {
        let mut signal = RandomSignal::new(0, 100);
        let data1 = signal.by_ref().take(200).collect::<Vec<u64>>();
        let data2 = signal.by_ref().take(200).collect::<Vec<u64>>();
        let data3 = signal.by_ref().take(200).collect::<Vec<u64>>();
        Self {
            signal,
            data1,
            data2,
            data3,
        }
    }

    fn on_tick(&mut self) {
        let value = self.signal.next().unwrap();
        self.data1.pop();
        self.data1.insert(0, value);
        let value = self.signal.next().unwrap();
        self.data2.pop();
        self.data2.insert(0, value);
        let value = self.signal.next().unwrap();
        self.data3.pop();
        self.data3.insert(0, value);
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(250);

        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let chunks = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.area());
        let sparkline = Sparkline::default()
            .block(
                Block::new()
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .title("Data1"),
            )
            .data(&self.data1)
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(sparkline, chunks[0]);
        let sparkline = Sparkline::default()
            .block(
                Block::new()
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .title("Data2"),
            )
            .data(&self.data2)
            .style(Style::default().bg(Color::Green));
        frame.render_widget(sparkline, chunks[1]);
        // Multiline
        let sparkline = Sparkline::default()
            .block(
                Block::new()
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .title("Data3"),
            )
            .data(&self.data3)
            .style(Style::default().fg(Color::Red));
        frame.render_widget(sparkline, chunks[2]);
    }
}
