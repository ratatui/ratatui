use std::{error::Error, io, time::Duration};

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
    terminal.clear()?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    let outer = f.size();
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .title(block::Title::from("Main block with round corners").alignment(Alignment::Center))
        .border_type(BorderType::Rounded);
    let inner = outer_block.inner(outer);
    let [top, bottom] = *Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(inner)
    else {
        return;
    };
    let [top_left, top_right] = *Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(top)
    else {
        return;
    };
    let [bottom_left, bottom_right] = *Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(bottom)
    else {
        return;
    };

    let top_left_block = Block::default()
        .title("With Green Background")
        .borders(Borders::all())
        .on_green();
    let top_right_block = Block::default()
        .title(
            block::Title::from("With styled title".white().on_red().bold())
                .alignment(Alignment::Right),
        )
        .borders(Borders::ALL);
    let bottom_left_block = Paragraph::new("Text inside padded block").block(
        Block::default()
            .title("With borders")
            .borders(Borders::ALL)
            .padding(Padding {
                left: 4,
                right: 4,
                top: 2,
                bottom: 2,
            }),
    );
    let bottom_right_block = Block::default()
        .title("With styled borders and doubled borders")
        .border_style(Style::default().fg(Color::Cyan))
        .borders(Borders::LEFT | Borders::RIGHT)
        .border_type(BorderType::Double)
        .padding(Padding::uniform(1));
    let bottom_inner_block = Block::default()
        .title("Block inside padded block")
        .borders(Borders::ALL);

    f.render_widget(outer_block, outer);
    f.render_widget(Clear, top_left);
    f.render_widget(top_left_block, top_left);
    f.render_widget(top_right_block, top_right);
    f.render_widget(bottom_left_block, bottom_left);
    let bottom_right_inner = bottom_right_block.inner(bottom_right);
    f.render_widget(bottom_right_block, bottom_right);
    f.render_widget(bottom_inner_block, bottom_right_inner);
}
