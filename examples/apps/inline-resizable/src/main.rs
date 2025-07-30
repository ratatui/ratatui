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

use std::io;
use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout};
use ratatui::macros::line;
use ratatui::style::{Color, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Block, Gauge, List, ListItem, Widget};
use ratatui::{DefaultTerminal, Frame, TerminalOptions, Viewport};

const INITIAL_HEIGHT: u16 = 6;
const MAX_HEIGHT: u16 = 15;
const MIN_HEIGHT: u16 = 3;

fn main() -> Result<()> {
    color_eyre::install()?;

    // Start with an inline viewport of height 6
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(INITIAL_HEIGHT),
    });

    let mut app = App::new();
    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}

struct App {
    height: u16,
    start_time: Instant,
    frame_count: u64,
    messages: Vec<String>,
    insert_count: u32,
}

impl App {
    fn new() -> Self {
        Self {
            height: INITIAL_HEIGHT,
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

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            self.frame_count += 1;
            terminal.draw(|frame| self.render(frame))?;
            if !event::poll(Duration::from_millis(100))? {
                // redraw at least 10fps
                continue;
            }
            let Some(key) = event::read()?.as_key_press_event() else {
                // other events (resize, mouse, etc.) just cause the viewport to redraw
                continue;
            };

            match key.code {
                KeyCode::Char('q') => break Ok(()),
                KeyCode::Char('+') | KeyCode::Char('=') => self.increase_height(terminal)?,
                KeyCode::Char('-') | KeyCode::Char('_') => self.decrease_height(terminal)?,
                KeyCode::Char('r') => self.reset_height(terminal)?,
                KeyCode::Char('i') => self.insert_line(terminal)?,

                _ => {}
            }
        }
    }

    fn increase_height(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        if self.height >= MAX_HEIGHT {
            let message = format!("Maximum height reached ({MAX_HEIGHT})");
            self.messages.push(message);
            return Ok(());
        }
        self.height += 1;
        let message = format!("Height increased to {}", self.height);
        self.messages.push(message);
        terminal.set_viewport_height(self.height)
    }

    fn decrease_height(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        if self.height <= MIN_HEIGHT {
            let message = format!("Minimum height reached ({MIN_HEIGHT})");
            self.messages.push(message);
            return Ok(());
        }
        self.height -= 1;
        let message = format!("Height decreased to {}", self.height);
        self.messages.push(message);
        terminal.set_viewport_height(self.height)
    }

    fn reset_height(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.height = INITIAL_HEIGHT;
        let message = format!("Height reset to {}", INITIAL_HEIGHT);
        self.messages.push(message);
        terminal.set_viewport_height(self.height)
    }

    fn insert_line(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.insert_count += 1;
        let message = format!("Inserted line #{}", self.insert_count);
        self.messages.push(message);

        let now = self.start_time.elapsed();
        terminal.insert_before(1, |buf| {
            let inserted_line = format!(
                "📝 Inserted line #{} at {:02}:{:02}",
                self.insert_count,
                now.as_secs() / 60,
                now.as_secs() % 60
            );
            inserted_line.green().bold().render(buf.area, buf);
        })?;
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        // Create a block with border and title showing current height
        let title = format!(" Resizable Viewport (Height: {}) ", self.height);
        let main_block = Block::bordered().title(title.cyan().bold());
        frame.render_widget(&main_block, frame.area());

        let [status_area, controls_area, messages_area, bottom_area] =
            main_block.inner(frame.area()).layout(&Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Min(0),
                Constraint::Length(1),
            ]));

        // Render the status area with runtime and frame count
        let elapsed = self.start_time.elapsed();
        let status = format!(
            "Runtime: {:02}:{:02} | Frames: {} | Current Height: {}",
            elapsed.as_secs() / 60,
            elapsed.as_secs() % 60,
            self.frame_count,
            self.height
        );
        frame.render_widget(status.green(), status_area);

        // Render the controls area with instructions
        let controls = line![
            "Controls: ".yellow().bold(),
            "'+' increase | '-' decrease | 'r' reset | 'i' insert | 'q' quit",
        ];
        frame.render_widget(controls, controls_area);

        // Render the messages area with the most recent messages
        let message_list = self
            .messages
            .iter()
            .rev() // show only the most recent messages
            .take(messages_area.height as usize)
            .rev() // reverse again to maintain order
            .enumerate()
            .map(|(i, msg)| {
                let color = if i == 0 {
                    Color::Yellow // Highlight newest message
                } else {
                    Color::White
                };
                ListItem::new(Span::styled(msg, color))
            })
            .collect::<List>();
        frame.render_widget(message_list, messages_area);

        // Bottom info with dynamic gauge showing height percentage
        let height_percentage = (self.height as f64 / 20.0).min(1.0);
        let gauge = Gauge::default()
            .gauge_style(Color::Blue)
            .ratio(height_percentage)
            .label(format!("Height: {}/20", self.height));
        frame.render_widget(gauge, bottom_area);
    }
}
