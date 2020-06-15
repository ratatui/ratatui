#[allow(dead_code)]
mod util;

use crate::util::{
    event::{Event, Events},
    StatefulList,
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, Text},
    Terminal,
};

struct App<'a> {
    items: StatefulList<&'a str>,
    events: Vec<(&'a str, &'a str)>,
    info_style: Style,
    warning_style: Style,
    error_style: Style,
    critical_style: Style,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(vec![
                "Item0", "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8",
                "Item9", "Item10", "Item11", "Item12", "Item13", "Item14", "Item15", "Item16",
                "Item17", "Item18", "Item19", "Item20", "Item21", "Item22", "Item23", "Item24",
            ]),
            events: vec![
                ("Event1", "INFO"),
                ("Event2", "INFO"),
                ("Event3", "CRITICAL"),
                ("Event4", "ERROR"),
                ("Event5", "INFO"),
                ("Event6", "INFO"),
                ("Event7", "WARNING"),
                ("Event8", "INFO"),
                ("Event9", "INFO"),
                ("Event10", "INFO"),
                ("Event11", "CRITICAL"),
                ("Event12", "INFO"),
                ("Event13", "INFO"),
                ("Event14", "INFO"),
                ("Event15", "INFO"),
                ("Event16", "INFO"),
                ("Event17", "ERROR"),
                ("Event18", "ERROR"),
                ("Event19", "INFO"),
                ("Event20", "INFO"),
                ("Event21", "WARNING"),
                ("Event22", "INFO"),
                ("Event23", "INFO"),
                ("Event24", "WARNING"),
                ("Event25", "INFO"),
                ("Event26", "INFO"),
            ],
            info_style: Style::default().fg(Color::White),
            warning_style: Style::default().fg(Color::Yellow),
            error_style: Style::default().fg(Color::Magenta),
            critical_style: Style::default().fg(Color::Red),
        }
    }

    fn advance(&mut self) {
        let event = self.events.pop().unwrap();
        self.events.insert(0, event);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    // App
    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let style = Style::default().fg(Color::Black).bg(Color::White);

            let items = app.items.items.iter().map(|i| Text::raw(*i));
            let items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .style(style)
                .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))
                .highlight_symbol(">");
            f.render_stateful_widget(items, chunks[0], &mut app.items.state);

            let events = app.events.iter().map(|&(evt, level)| {
                Text::styled(
                    format!("{}: {}", level, evt),
                    match level {
                        "ERROR" => app.error_style,
                        "CRITICAL" => app.critical_style,
                        "WARNING" => app.warning_style,
                        _ => app.info_style,
                    },
                )
            });
            let events_list = List::new(events)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .start_corner(Corner::BottomLeft);
            f.render_widget(events_list, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left => {
                    app.items.unselect();
                }
                Key::Down => {
                    app.items.next();
                }
                Key::Up => {
                    app.items.previous();
                }
                _ => {}
            },
            Event::Tick => {
                app.advance();
            }
        }
    }

    Ok(())
}
