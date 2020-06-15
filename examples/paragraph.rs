#[allow(dead_code)]
mod util;

use crate::util::event::{Event, Events};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Text},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let mut scroll: u16 = 0;
    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Words made "loooong" to demonstrate line breaking.
            let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
            let mut long_line = s.repeat(usize::from(size.width) / s.len() + 4);
            long_line.push('\n');

            let block = Block::default()
                .style(Style::default().bg(Color::White));
            f.render_widget(block, size);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints(
                    [
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ]
                    .as_ref(),
                )
                .split(size);

            let text = [
                Text::raw("This is a line \n"),
                Text::styled("This is a line   \n", Style::default().fg(Color::Red)),
                Text::styled("This is a line\n", Style::default().bg(Color::Blue)),
                Text::styled(
                    "This is a longer line\n",
                    Style::default().modifier(Modifier::CROSSED_OUT),
                ),
                Text::styled(&long_line, Style::default().bg(Color::Green)),
                Text::styled(
                    "This is a line\n",
                    Style::default().fg(Color::Green).modifier(Modifier::ITALIC),
                ),
            ];

            let block = Block::default()
                .borders(Borders::ALL)
                .title_style(Style::default().modifier(Modifier::BOLD));
            let paragraph = Paragraph::new(text.iter())
                .block(block.clone().title("Left, no wrap"))
                .alignment(Alignment::Left);
            f.render_widget(paragraph, chunks[0]);
            let paragraph = Paragraph::new(text.iter())
                .block(block.clone().title("Left, wrap"))
                .alignment(Alignment::Left)
                .wrap(true);
            f.render_widget(paragraph, chunks[1]);
            let paragraph = Paragraph::new(text.iter())
                .block(block.clone().title("Center, wrap"))
                .alignment(Alignment::Center)
                .wrap(true)
                .scroll(scroll);
            f.render_widget(paragraph, chunks[2]);
            let paragraph = Paragraph::new(text.iter())
                .block(block.clone().title("Right, wrap"))
                .alignment(Alignment::Right)
                .wrap(true);
            f.render_widget(paragraph, chunks[3]);
        })?;

        scroll += 1;
        scroll %= 10;

        if let Event::Input(key) = events.next()? {
            if key == Key::Char('q') {
                break;
            }
        }
    }
    Ok(())
}
