/// A Ratatui example that demonstrates how to use the resizable inline viewport.
///
/// This example shows how to create an inline viewport with a fixed height that can be
/// dynamically resized during runtime using the new `set_viewport_height()` method.
///
/// Controls:
/// - '+' or '=' : Increase viewport height
/// - '-' or '_' : Decrease viewport height  
/// - 'r'        : Reset viewport to initial height
/// - 'i'        : Insert a line before the viewport
/// - 'q'        : Quit
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Widget};
use ratatui::{Frame, Terminal, TerminalOptions, Viewport};

fn main() -> Result<()> {
    color_eyre::install()?;
    
    // Start with an inline viewport of height 6
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(6),
    });

    let (tx, rx) = mpsc::channel();
    input_handling(tx.clone());
    
    let mut app_state = AppState::new();
    let app_result = run(&mut terminal, &mut app_state, rx);

    ratatui::restore();
    app_result
}

struct AppState {
    current_height: u16,
    initial_height: u16,
    start_time: Instant,
    frame_count: u64,
    messages: Vec<String>,
    insert_count: u32,
}

impl AppState {
    fn new() -> Self {
        Self {
            current_height: 6,
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
        if self.current_height < 20 {
            self.current_height += 1;
            self.messages.push(format!("Height increased to {}", self.current_height));
        } else {
            self.messages.push("Maximum height reached (20)".to_string());
        }
    }
    
    fn decrease_height(&mut self) {
        if self.current_height > 3 {
            self.current_height -= 1;
            self.messages.push(format!("Height decreased to {}", self.current_height));
        } else {
            self.messages.push("Minimum height reached (3)".to_string());
        }
    }
    
    fn reset_height(&mut self) {
        self.current_height = self.initial_height;
        self.messages.push(format!("Height reset to {}", self.initial_height));
    }
    
    fn insert_line(&mut self) {
        self.insert_count += 1;
        self.messages.push(format!("Inserted line #{} before viewport", self.insert_count));
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

fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    app_state: &mut AppState,
    rx: mpsc::Receiver<Event>,
) -> Result<()>
where
    B::Error: Send + Sync + 'static,
{
    let mut redraw = true;
    
    loop {
        if redraw {
            terminal.draw(|frame| render(frame, app_state))?;
        }
        redraw = true;

        match rx.recv()? {
            Event::Input(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            app_state.increase_height();
                            terminal.set_viewport_height(app_state.current_height)?;
                        }
                        KeyCode::Char('-') | KeyCode::Char('_') => {
                            app_state.decrease_height();
                            terminal.set_viewport_height(app_state.current_height)?;
                        }
                        KeyCode::Char('r') => {
                            app_state.reset_height();
                            terminal.set_viewport_height(app_state.current_height)?;
                        }
                        KeyCode::Char('i') => {
                            app_state.insert_line();
                            let count = app_state.insert_count;
                            let now = app_state.start_time.elapsed();
                            terminal.insert_before(1, |buf| {
                                Paragraph::new(format!(
                                    "📝 Inserted line #{} at {:02}:{:02}",
                                    count,
                                    now.as_secs() / 60,
                                    now.as_secs() % 60
                                ))
                                .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                                .render(buf.area, buf);
                            })?;
                        }
                        _ => {}
                    }
                }
            }
            Event::Resize => {
                terminal.autoresize()?;
            }
            Event::Tick => {
                app_state.frame_count += 1;
                redraw = false; // Don't redraw on every tick unless content changed
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &AppState) {
    let area = frame.area();
    
    // Create a block with border and title showing current height
    let main_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Resizable Viewport (Height: {}) ", app_state.current_height))
        .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    
    let inner_area = main_block.inner(area);
    frame.render_widget(main_block, area);
    
    // Split the inner area into sections
    let vertical_layout = Layout::vertical([
        Constraint::Length(1), // Status line
        Constraint::Length(2), // Controls info
        Constraint::Min(0),    // Messages list
        Constraint::Length(1), // Bottom info
    ]);
    let [status_area, controls_area, messages_area, bottom_area] = vertical_layout.areas(inner_area);
    
    // Status line
    let elapsed = app_state.start_time.elapsed();
    let status_line = Paragraph::new(format!(
        "Runtime: {:02}:{:02} | Frames: {} | Current Height: {}",
        elapsed.as_secs() / 60,
        elapsed.as_secs() % 60,
        app_state.frame_count,
        app_state.current_height
    ))
    .style(Style::default().fg(Color::Green));
    frame.render_widget(status_line, status_area);
    
    // Controls info
    let controls = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Controls: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("'+' increase | '-' decrease | 'r' reset | 'i' insert | 'q' quit"),
        ]),
    ]);
    frame.render_widget(controls, controls_area);
    
    // Messages list (scrollable)
    let message_items: Vec<ListItem> = app_state
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
    let height_percentage = (app_state.current_height as f64 / 20.0).min(1.0);
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Blue))
        .ratio(height_percentage)
        .label(format!("Height: {}/20", app_state.current_height));
    frame.render_widget(gauge, bottom_area);
}