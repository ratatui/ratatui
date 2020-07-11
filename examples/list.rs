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
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

struct App<'a> {
    items: StatefulList<(&'a str, usize)>,
    events: Vec<(&'a str, &'a str)>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(vec![
                ("Item0", 1),
                ("Item1", 2),
                ("Item2", 1),
                ("Item3", 3),
                ("Item4", 1),
                ("Item5", 4),
                ("Item6", 1),
                ("Item7", 3),
                ("Item8", 1),
                ("Item9", 6),
                ("Item10", 1),
                ("Item11", 3),
                ("Item12", 1),
                ("Item13", 2),
                ("Item14", 1),
                ("Item15", 1),
                ("Item16", 4),
                ("Item17", 1),
                ("Item18", 5),
                ("Item19", 4),
                ("Item20", 1),
                ("Item21", 2),
                ("Item22", 1),
                ("Item23", 3),
                ("Item24", 1),
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

    let events = Events::new();

    // App
    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let items: Vec<ListItem> = app
                .items
                .items
                .iter()
                .map(|i| {
                    let mut lines = vec![Spans::from(i.0)];
                    for _ in 0..i.1 {
                        lines.push(Spans::from(Span::styled(
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                            Style::default().add_modifier(Modifier::ITALIC),
                        )));
                    }
                    ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
                })
                .collect();
            let items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");
            f.render_stateful_widget(items, chunks[0], &mut app.items.state);

            let events: Vec<ListItem> = app
                .events
                .iter()
                .map(|&(evt, level)| {
                    let s = match level {
                        "CRITICAL" => Style::default().fg(Color::Red),
                        "ERROR" => Style::default().fg(Color::Magenta),
                        "WARNING" => Style::default().fg(Color::Yellow),
                        "INFO" => Style::default().fg(Color::Blue),
                        _ => Style::default(),
                    };
                    let header = Spans::from(vec![
                        Span::styled(format!("{:<9}", level), s),
                        Span::raw(" "),
                        Span::styled(
                            "2020-01-01 10:00:00",
                            Style::default().add_modifier(Modifier::ITALIC),
                        ),
                    ]);
                    let log = Spans::from(vec![Span::raw(evt)]);
                    ListItem::new(vec![
                        Spans::from("-".repeat(chunks[1].width as usize)),
                        header,
                        Spans::from(""),
                        log,
                    ])
                })
                .collect();
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
