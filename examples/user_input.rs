/// A simple example demonstrating how to handle user input. This is
/// a bit out of the scope of the library as it does not provide any
/// input handling out of the box. However, it may helps some to get
/// started.
///
/// This is a very simple example:
///   * A input box always focused. Every character you type is registered
///   here
///   * Pressing Backspace erases a character
///   * Pressing Enter pushes the current input in the history of previous
///   messages

#[allow(dead_code)]
mod util;

use crate::util::event::{Event, Events};
use std::{
    error::Error,
    io::{self, Write},
};
use termion::{
    cursor::Goto, event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, Paragraph, Text},
    Terminal,
};
use unicode_width::UnicodeWidthStr;

enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handlers
    let mut events = Events::new();

    // Create default app state
    let mut app = App::default();

    loop {
        // Draw UI
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let msg = match app.input_mode {
                InputMode::Normal => "Press q to exit, e to start editing.",
                InputMode::Editing => "Press Esc to stop editing, Enter to record the message",
            };
            let text = [Text::raw(msg)];
            let help_message = Paragraph::new(text.iter());
            f.render_widget(help_message, chunks[0]);

            let text = [Text::raw(&app.input)];
            let input = Paragraph::new(text.iter())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input"));
            f.render_widget(input, chunks[1]);
            let messages = app
                .messages
                .iter()
                .enumerate()
                .map(|(i, m)| Text::raw(format!("{}: {}", i, m)));
            let messages =
                List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
            f.render_widget(messages, chunks[2]);
        })?;

        // Put the cursor back inside the input box
        write!(
            terminal.backend_mut(),
            "{}",
            Goto(4 + app.input.width() as u16, 5)
        )?;
        // stdout is buffered, flush it to see the effect immediately when hitting backspace
        io::stdout().flush().ok();

        // Handle input
        match events.next()? {
            Event::Input(input) => match app.input_mode {
                InputMode::Normal => match input {
                    Key::Char('e') => {
                        app.input_mode = InputMode::Editing;
                        events.disable_exit_key();
                    }
                    Key::Char('q') => {
                        break;
                    }
                    _ => {}
                },
                InputMode::Editing => match input {
                    Key::Char('\n') => {
                        app.messages.push(app.input.drain(..).collect());
                    }
                    Key::Char(c) => {
                        app.input.push(c);
                    }
                    Key::Backspace => {
                        app.input.pop();
                    }
                    Key::Esc => {
                        app.input_mode = InputMode::Normal;
                        events.enable_exit_key();
                    }
                    _ => {}
                },
            },
            _ => {}
        }
    }
    Ok(())
}
