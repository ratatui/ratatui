//! # [Ratatui] Docs.rs example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

use std::io::{self, stdout};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

/// Example code for libr.rs
///
/// When cargo-rdme supports doc comments that import from code, this will be imported
/// rather than copied to the lib.rs file.
fn main() -> io::Result<()> {
    let arg = std::env::args().nth(1).unwrap_or_default();
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(match arg.as_str() {
            "hello_world" => hello_world,
            "layout" => layout,
            "styling" => styling,
            _ => hello_world,
        })?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn hello_world(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello World!")
            .block(Block::default().title("Greeting").borders(Borders::ALL)),
        frame.size(),
    );
}

use crossterm::event::{self, Event, KeyCode};
fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn layout(frame: &mut Frame) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ]);
    let horizontal = Layout::horizontal([Constraint::Ratio(1, 2); 2]);
    let [title_bar, main_area, status_bar] = frame.size().split(&vertical);
    let [left, right] = main_area.split(&horizontal);

    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Title Bar"),
        title_bar,
    );
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Status Bar"),
        status_bar,
    );
    frame.render_widget(Block::default().borders(Borders::ALL).title("Left"), left);
    frame.render_widget(Block::default().borders(Borders::ALL).title("Right"), right);
}

fn styling(frame: &mut Frame) {
    let areas = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
    ])
    .split(frame.size());

    let span1 = Span::raw("Hello ");
    let span2 = Span::styled(
        "World",
        Style::new()
            .fg(Color::Green)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    let span3 = "!".red().on_light_yellow().italic();

    let line = Line::from(vec![span1, span2, span3]);
    let text: Text = Text::from(vec![line]);

    frame.render_widget(Paragraph::new(text), areas[0]);
    // or using the short-hand syntax and implicit conversions
    frame.render_widget(
        Paragraph::new("Hello World!".red().on_white().bold()),
        areas[1],
    );

    // to style the whole widget instead of just the text
    frame.render_widget(
        Paragraph::new("Hello World!").style(Style::new().red().on_white()),
        areas[2],
    );
    // or using the short-hand syntax
    frame.render_widget(Paragraph::new("Hello World!").blue().on_yellow(), areas[3]);
}
