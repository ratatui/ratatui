//! # [Ratatui] Calendar example
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
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin},
    style::{Color, Modifier, Style},
    widgets::calendar::{CalendarEventStore, DateStyler, Monthly},
    DefaultTerminal, Frame,
};
use time::{Date, Month, OffsetDateTime};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break Ok(());
            }
        }
    }
}

fn draw(frame: &mut Frame) {
    let area = frame.area().inner(Margin {
        vertical: 1,
        horizontal: 1,
    });

    let mut start = OffsetDateTime::now_local()
        .unwrap()
        .date()
        .replace_month(Month::January)
        .unwrap()
        .replace_day(1)
        .unwrap();

    let list = make_dates(start.year());

    let rows = Layout::vertical([Constraint::Ratio(1, 3); 3]).split(area);
    let cols = rows.iter().flat_map(|row| {
        Layout::horizontal([Constraint::Ratio(1, 4); 4])
            .split(*row)
            .to_vec()
    });
    for col in cols {
        let cal = cals::get_cal(start.month(), start.year(), &list);
        frame.render_widget(cal, col);
        start = start.replace_month(start.month().next()).unwrap();
    }
}

fn make_dates(current_year: i32) -> CalendarEventStore {
    let mut list = CalendarEventStore::today(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Blue),
    );

    // Holidays
    let holiday_style = Style::default()
        .fg(Color::Red)
        .add_modifier(Modifier::UNDERLINED);

    // new year's
    list.add(
        Date::from_calendar_date(current_year, Month::January, 1).unwrap(),
        holiday_style,
    );
    // next new_year's for December "show surrounding"
    list.add(
        Date::from_calendar_date(current_year + 1, Month::January, 1).unwrap(),
        holiday_style,
    );
    // groundhog day
    list.add(
        Date::from_calendar_date(current_year, Month::February, 2).unwrap(),
        holiday_style,
    );
    // april fool's
    list.add(
        Date::from_calendar_date(current_year, Month::April, 1).unwrap(),
        holiday_style,
    );
    // earth day
    list.add(
        Date::from_calendar_date(current_year, Month::April, 22).unwrap(),
        holiday_style,
    );
    // star wars day
    list.add(
        Date::from_calendar_date(current_year, Month::May, 4).unwrap(),
        holiday_style,
    );
    // festivus
    list.add(
        Date::from_calendar_date(current_year, Month::December, 23).unwrap(),
        holiday_style,
    );
    // new year's eve
    list.add(
        Date::from_calendar_date(current_year, Month::December, 31).unwrap(),
        holiday_style,
    );

    // seasons
    let season_style = Style::default()
        .fg(Color::White)
        .bg(Color::Yellow)
        .add_modifier(Modifier::UNDERLINED);
    // spring equinox
    list.add(
        Date::from_calendar_date(current_year, Month::March, 22).unwrap(),
        season_style,
    );
    // summer solstice
    list.add(
        Date::from_calendar_date(current_year, Month::June, 21).unwrap(),
        season_style,
    );
    // fall equinox
    list.add(
        Date::from_calendar_date(current_year, Month::September, 22).unwrap(),
        season_style,
    );
    list.add(
        Date::from_calendar_date(current_year, Month::December, 21).unwrap(),
        season_style,
    );
    list
}

mod cals {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    pub fn get_cal<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        match m {
            Month::May => example1(m, y, es),
            Month::June => example2(m, y, es),
            Month::July | Month::December => example3(m, y, es),
            Month::February => example4(m, y, es),
            Month::November => example5(m, y, es),
            _ => default(m, y, es),
        }
    }

    fn default<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_month_header(Style::default())
            .default_style(default_style)
    }

    fn example1<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_surrounding(default_style)
            .default_style(default_style)
            .show_month_header(Style::default())
    }

    fn example2<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        let header_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::DIM)
            .fg(Color::LightYellow);

        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_weekdays_header(header_style)
            .default_style(default_style)
            .show_month_header(Style::default())
    }

    fn example3<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        let header_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Green);

        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_surrounding(Style::default().add_modifier(Modifier::DIM))
            .show_weekdays_header(header_style)
            .default_style(default_style)
            .show_month_header(Style::default())
    }

    fn example4<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        let header_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Green);

        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_weekdays_header(header_style)
            .default_style(default_style)
    }

    fn example5<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        let header_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Green);

        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_month_header(header_style)
            .default_style(default_style)
    }
}
