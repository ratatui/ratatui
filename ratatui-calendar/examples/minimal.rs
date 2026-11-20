//! # [Ratatui] calendar `minimal` example
//!
//! Renders the current month and quits on `q` or Esc.
//!
//! The latest version of this example is available in the [calendar examples] folder in the
//! repository. The examples are designed to be run against the `main` branch of the GitHub
//! repository, so they may not compile with the latest release on crates.io.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [calendar examples]: https://github.com/ratatui/ratatui/blob/main/ratatui-calendar/examples

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use jiff::ZonedDateTime;
use ratatui::DefaultTerminal;
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(run)
}

/// Render the current month until `q` or Esc is pressed.
fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let today = ZonedDateTime::now().date();
    let calendar = Monthly::new(today, CalendarEventStore::today(Style::new().red().bold()))
        .show_month_header(Style::new().bold())
        .show_weekdays_header(Style::new().italic())
        .block(Block::bordered().title("Press q or Esc to quit"));
    loop {
        terminal.draw(|frame| frame.render_widget(&calendar, frame.area()))?;
        if let Some(key) = event::read()?.as_key_press_event()
            && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
        {
            return Ok(());
        }
    }
}
