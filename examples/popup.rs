#[allow(dead_code)]
mod util;

use crate::util::event::{Event, Events};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::layout::Rect;
use tui::widgets::Clear;
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Text, Wrap},
    Terminal,
};

/// helper function to create a centered rect using up
/// certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
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

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(size);

                let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
                let mut long_line = s.repeat(usize::from(size.width)*usize::from(size.height)/300);
                long_line.push('\n');

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

            let paragraph = Paragraph::new(text.iter())
                .block(Block::default().title("Left Block").borders(Borders::ALL))
                .alignment(Alignment::Left).wrap(Wrap { trim: true });
            f.render_widget(paragraph, chunks[0]);

            let paragraph = Paragraph::new(text.iter())
                .block(Block::default().title("Right Block").borders(Borders::ALL))
                .alignment(Alignment::Left).wrap(Wrap { trim: true });
            f.render_widget(paragraph, chunks[1]);

            let block = Block::default().title("Popup").borders(Borders::ALL);
            let area = centered_rect(60, 20, size);
            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(block, area);
        })?;

        if let Event::Input(input) = events.next()? {
            if let Key::Char('q') = input {
                break;
            }
        }
    }

    Ok(())
}
