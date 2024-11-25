//! # [Ratatui] `Calendar` example
//!
//! The latest version of this example is available in the [widget examples] folder in the
//! repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [widget examples]: https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use color_eyre::Result;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{
        calendar::{CalendarEventStore, Monthly},
        Block, Padding,
    },
    DefaultTerminal, Frame,
};
use time::OffsetDateTime;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

/// Run the application.
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if quit_key_pressed()? {
            break Ok(());
        }
    }
}

/// Draw the UI with a calendar.
fn draw(frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [top, main] = vertical.areas(frame.area());

    let title = Line::from_iter([
        Span::from("Monthly Widget").bold(),
        Span::from(" (Press 'q' to quit)"),
    ]);
    frame.render_widget(title.centered(), top);
    render_current_month(frame, main);
}

/// Render the current month calendar.
fn render_current_month(frame: &mut Frame, area: Rect) {
    let date = OffsetDateTime::now_utc().date();

    let monthly = Monthly::new(date, CalendarEventStore::today(Style::new().red().bold()))
        .block(Block::new().padding(Padding::new(0, 0, 2, 0)))
        .show_weekdays_header(Style::new().italic());
    frame.render_widget(monthly, area);
}

/// Wait for an event and return `true` if the Esc or 'q' key is pressed.
fn quit_key_pressed() -> Result<bool> {
    use ratatui::crossterm::event::{self, Event, KeyCode};
    match event::read()? {
        Event::Key(event) if matches!(event.code, KeyCode::Esc | KeyCode::Char('q')) => Ok(true),
        _ => Ok(false),
    }
}
