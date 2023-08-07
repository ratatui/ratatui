use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
#[macro_use]
extern crate log;

pub fn color_log_format(
    w: &mut dyn std::io::Write,
    now: &mut flexi_logger::DeferredNow,
    record: &flexi_logger::Record,
) -> std::result::Result<(), std::io::Error> {
    let level = record.level();
    return write!(
        w,
        "[{} {} {}]: {}",
        now.format_rfc3339(), /* .color(Color::BrightBlack) */
        // Bright Black = Grey
        flexi_logger::style(level).paint(format!("{level:5}")), /* pad level to 2 characters,
                                                                 * cannot be done in the string
                                                                 * itself, because of the color
                                                                 * characters */
        record.module_path().unwrap_or("<unnamed module>"),
        &record.args() /* dont apply any color to the input, so that the input can dynamically
                        * set the color */
    );
}

fn main() -> Result<()> {
    let filespec = flexi_logger::FileSpec::try_from("/tmp/ratatui")
        .expect("Expected logging file to be parsed correctly");
    let logger = flexi_logger::Logger::try_with_env_or_str("warn")
        .expect("Expected flexi_logger to be able to parse env or string")
        .format_for_files(color_log_format)
        .log_to_file(filespec)
        .append();

    logger
        .start()
        .expect("Expected flexi_logger to be able to start");

    error!("Active");

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
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }));
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

/// Resets the terminal.
fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}
type Result<T> = std::result::Result<T, Box<dyn Error>>;
