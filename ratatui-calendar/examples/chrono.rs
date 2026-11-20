//! # [Ratatui] calendar `chrono` example
//!
//! Shows an app that uses the `chrono` crate for its own dates. The calendar takes jiff
//! dates, so the app converts each `chrono::NaiveDate` at the boundary.
//!
//! The latest version of this example is available in the [calendar examples] folder in the
//! repository. The examples are designed to be run against the `main` branch of the GitHub
//! repository, so they may not compile with the latest release on crates.io.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [calendar examples]: https://github.com/ratatui/ratatui/blob/main/ratatui-calendar/examples

use chrono::{Datelike, Local, NaiveDate};
use color_eyre::Result;
use color_eyre::eyre::OptionExt;
use crossterm::event::{self, KeyCode};
use ratatui::DefaultTerminal;
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(run)
}

/// Render this month, with a deadline marked, until `q` or Esc is pressed.
fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    // The app's own dates, as `chrono` values.
    let today = Local::now().date_naive();
    let deadline = NaiveDate::from_ymd_opt(today.year(), 12, 24).ok_or_eyre("invalid date")?;

    let today = to_jiff(today)?;
    let deadline = to_jiff(deadline)?;

    let mut events = CalendarEventStore::default();
    events.add(deadline, Style::new().red().bold());
    let calendar = Monthly::new(today, events)
        .show_month_header(Style::new().bold())
        .show_weekdays_header(Style::new().italic())
        .block(Block::bordered().title("chrono::NaiveDate (press q or Esc to quit)"));
    loop {
        terminal.draw(|frame| frame.render_widget(&calendar, frame.area()))?;
        if let Some(key) = event::read()?.as_key_press_event()
            && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
        {
            return Ok(());
        }
    }
}

/// Converts a `chrono` date to the jiff date the calendar uses.
fn to_jiff(date: NaiveDate) -> Result<jiff::civil::Date> {
    let year = i16::try_from(date.year())?;
    let month = i8::try_from(date.month())?;
    let day = i8::try_from(date.day())?;
    let date = jiff::civil::Date::new(year, month, day)?;
    Ok(date)
}
