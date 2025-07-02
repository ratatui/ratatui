//! A Ratatui example that demonstrates how to use the resizable inline viewport.
//!
//! This example shows how to create an inline viewport with a fixed height that can be
//! dynamically resized during runtime using the new `set_viewport_height()` method.
//!
//! Controls:
//! - '+' or '=' : Increase viewport height
//! - '-' or '_' : Decrease viewport height
//! - 'r'        : Reset viewport to initial height
//! - 'i'        : Insert a line before the viewport
//! - 'q'        : Quit
//!
//! This example runs with the Ratatui library code in the branch that you are currently reading.
//! See the [`latest`] branch for the code which works with the most recent Ratatui release.
//!
//! [`latest`]: https://github.com/ratatui/ratatui/tree/latest
//! [`BarChart`]: https://docs.rs/ratatui/latest/ratatui/widgets/struct.BarChart.html

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Gauge, List, ListItem, Paragraph, Widget};
use ratatui::{Frame, Terminal, TerminalOptions, Viewport};

fn main() -> Result<()> {
    color_eyre::install()?;

    // Start with an inline viewport of height 6
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(6),
    });

    let (tx, rx) = mpsc::channel();
    input_handling(tx.clone());

    let mut app_state = App::new();
    let app_result = app_state.run(&mut terminal, rx);

    ratatui::restore();
    app_result
}

struct App {
    height: u16,
    initial_height: u16,
    start_time: Instant,
    frame_count: u64,
    messages: Vec<String>,
    insert_count: u32,
}

impl App {
    fn new() -> Self {
        Self {
            height: 6,
            initial_height: 6,
            start_time: Instant::now(),
            frame_count: 0,
            insert_count: 0,
            messages: vec![
                "Welcome to Resizable Inline Viewport Demo!".to_string(),
                "Use '+' to increase height, '-' to decrease".to_string(),
                "Press 'r' to reset, 'i' to insert line, 'q' to quit".to_string(),
            ],
        }
    }

    fn increase_height(&mut self) {
        if self.height < 20 {
            self.height += 1;
            self.messages
                .push(format!("Height increased to {}", self.height));
        } else {
            self.messages
                .push("Maximum height reached (20)".to_string());
        }
    }

    fn decrease_height(&mut self) {
        if self.height > 3 {
            self.height -= 1;
            self.messages
                .push(format!("Height decreased to {}", self.height));
        } else {
            self.messages.push("Minimum height reached (3)".to_string());
        }
    }

    fn reset_height(&mut self) {
        self.height = self.initial_height;
        self.messages
            .push(format!("Height reset to {}", self.initial_height));
    }

    fn insert_line(&mut self) {
        self.insert_count += 1;
        self.messages.push(format!(
            "Inserted line #{} before viewport",
            self.insert_count
        ));
    }

    fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        rx: mpsc::Receiver<Event>,
    ) -> Result<()>
    where
        B::Error: Send + Sync + 'static,
    {
        let mut redraw = true;

        loop {
            if redraw {
                terminal.draw(|frame| self.render(frame))?;
            }
            redraw = true;

            match rx.recv()? {
                Event::Input(key_event) => if key_event.is_press() {
                    match key_event.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            self.increase_height();
                            terminal.set_viewport_height(self.height)?;
                        }
                        KeyCode::Char('-') | KeyCode::Char('_') => {
                            self.decrease_height();
                            terminal.set_viewport_height(self.height)?;
                        }
                        KeyCode::Char('r') => {
                            self.reset_height();
                            terminal.set_viewport_height(self.height)?;
                        }
                        KeyCode::Char('i') => {
                            self.insert_line();
                            let count = self.insert_count;
                            let now = self.start_time.elapsed();
                            terminal.insert_before(1, |buf| {
                                Paragraph::new(format!(
                                    "📝 Inserted line #{} at {:02}:{:02}",
                                    count,
                                    now.as_secs() / 60,
                                    now.as_secs() % 60
                                ))
                                .style(
                                    Style::default()
                                        .fg(Color::Green)
                                        .add_modifier(Modifier::BOLD),
                                )
                                .render(buf.area, buf);
                            })?;
                        }
                        _ => {}
                    }
                }
                Event::Tick => {
                    self.frame_count += 1;
                    redraw = false; // Don't redraw on every tick unless content changed
                }
                _ => {}
            }
        }
        Ok(())
    }


    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // Create a block with border and title showing current height
        let title = format!(" Resizable Viewport (Height: {}) ", self.height).cyan().bold();
        let main_block = Block::bordered().title(title);

        let inner_area = main_block.inner(area);
        frame.render_widget(main_block, area);

        // Split the inner area into sections
        let vertical_layout = Layout::vertical([
            Constraint::Length(1), // Status line
            Constraint::Length(2), // Controls info
            Constraint::Min(0),    // Messages list
            Constraint::Length(1), // Bottom info
        ]);
        let [status_area, controls_area, messages_area, bottom_area] =
            vertical_layout.areas(inner_area);

        // Status line
        let elapsed = self.start_time.elapsed();
        let status_line = Paragraph::new(format!(
            "Runtime: {:02}:{:02} | Frames: {} | Current Height: {}",
            elapsed.as_secs() / 60,
            elapsed.as_secs() % 60,
            self.frame_count,
            self.height
        ))
        .style(Style::default().fg(Color::Green));
        frame.render_widget(status_line, status_area);

        // Controls info
        let controls = Paragraph::new(vec![Line::from(vec![
            Span::styled(
                "Controls: ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("'+' increase | '-' decrease | 'r' reset | 'i' insert | 'q' quit"),
        ])]);
        frame.render_widget(controls, controls_area);

        // Messages list (scrollable)
        let message_items: Vec<ListItem> = self
            .messages
            .iter()
            .rev() // Show newest messages first
            .take(messages_area.height as usize)
            .enumerate()
            .map(|(i, msg)| {
                let style = if i == 0 {
                    Style::default().fg(Color::Yellow) // Highlight newest message
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(Line::from(Span::styled(msg, style)))
            })
            .collect();

        let messages_list = List::new(message_items);
        frame.render_widget(messages_list, messages_area);

        // Bottom info with dynamic gauge showing height percentage
        let height_percentage = (self.height as f64 / 20.0).min(1.0);
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Blue))
            .ratio(height_percentage)
            .label(format!("Height: {}/20", self.height));
        frame.render_widget(gauge, bottom_area);
    }
}

#[derive(Debug)]
enum Event {
    Input(event::KeyEvent),
    Tick,
    Resize,
}

fn input_handling(tx: mpsc::Sender<Event>) {
    let tick_rate = Duration::from_millis(250);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout).unwrap() {
                match event::read().unwrap() {
                    event::Event::Key(key) => tx.send(Event::Input(key)).unwrap(),
                    event::Event::Resize(_, _) => tx.send(Event::Resize).unwrap(),
                    _ => {}
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });
}

