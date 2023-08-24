use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};
use ratatui::{prelude::*, widgets::*};

#[derive(Clone)]
pub struct RandomSignal {
    distribution: Uniform<u64>,
    rng: ThreadRng,
}

impl RandomSignal {
    pub fn new(lower: u64, upper: u64) -> RandomSignal {
        RandomSignal {
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

struct App {
    signal: RandomSignal,
    data1: Vec<u64>,
    data2: Vec<u64>,
    data3: Vec<u64>,
}

impl App {
    fn new() -> App {
        let mut signal = RandomSignal::new(0, 100);
        let data1 = signal.by_ref().take(200).collect::<Vec<u64>>();
        let data2 = signal.by_ref().take(200).collect::<Vec<u64>>();
        let data3 = signal.by_ref().take(200).collect::<Vec<u64>>();
        App {
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
}

fn main() -> Result<()> {
    let mut terminal = TerminalBuilder::crossterm_on_stdout().build()?;

    let tick_rate = Duration::from_millis(250);
    let mut app = App::new();

    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
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
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(7),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(f.size());
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title("Data1")
                .borders(Borders::LEFT | Borders::RIGHT),
        )
        .data(&app.data1)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(sparkline, chunks[0]);
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title("Data2")
                .borders(Borders::LEFT | Borders::RIGHT),
        )
        .data(&app.data2)
        .style(Style::default().bg(Color::Green));
    f.render_widget(sparkline, chunks[1]);
    // Multiline
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title("Data3")
                .borders(Borders::LEFT | Borders::RIGHT),
        )
        .data(&app.data3)
        .style(Style::default().fg(Color::Red));
    f.render_widget(sparkline, chunks[2]);
}
