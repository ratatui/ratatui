use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>) {
    let [top, mid, bottom] = *Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(4),
                Constraint::Percentage(50),
                Constraint::Min(4),
            ]
            .as_ref(),
        )
        .split(frame.size())
    else {
        return;
    };
    let [left, right] = *Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(5)
        .vertical_margin(2)
        .constraints([Constraint::Ratio(2, 5), Constraint::Ratio(3, 5)].as_ref())
        .split(mid)
    else {
        return;
    };
    frame.render_widget(
        Paragraph::new("Constraint::Length(4)").block(Block::default().borders(Borders::ALL)),
        top,
    );

    frame.render_widget(
        Paragraph::new("Constraint::Percentage(50)").block(Block::default().borders(Borders::ALL)),
        mid,
    );

    frame.render_widget(
        Paragraph::new("Constraint::Ratio(2, 5)\nhorizontal_margin(5)\nvertical_margin(2)")
            .block(Block::default().borders(Borders::ALL)),
        left,
    );
    frame.render_widget(
        Paragraph::new("Constraint::Ratio(3, 5)").block(Block::default().borders(Borders::ALL)),
        right,
    );
    frame.render_widget(
        Paragraph::new("Constraint::Min(4)").block(Block::default().borders(Borders::ALL)),
        bottom,
    );
}
