#[allow(dead_code)]
mod util;

use crate::util::{
    event::{Event, Events},
    TabsState,
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Tabs},
    Terminal,
};

struct App<'a> {
    tabs: TabsState<'a>,
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
    let mut app = App {
        tabs: TabsState::new(vec!["Tab0", "Tab1", "Tab2", "Tab3"]),
    };

    // Main loop
    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);

            let block = Block::default().style(Style::default().bg(Color::White));
            f.render_widget(block, size);
            let tabs = Tabs::default()
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .titles(&app.tabs.titles)
                .select(app.tabs.index)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(Style::default().fg(Color::Yellow));
            f.render_widget(tabs, chunks[0]);
            let inner = match app.tabs.index {
                0 => Block::default().title("Inner 0").borders(Borders::ALL),
                1 => Block::default().title("Inner 1").borders(Borders::ALL),
                2 => Block::default().title("Inner 2").borders(Borders::ALL),
                3 => Block::default().title("Inner 3").borders(Borders::ALL),
                _ => unreachable!(),
            };
            f.render_widget(inner, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
