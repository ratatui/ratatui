//! A Ratatui example that demonstrates how to render calendar with different styles.
//!
//! Marks the holidays and seasons on the calendar.
//!
//! This example runs with the Ratatui library code in the branch that you are currently reading.
//! See the [`latest`] branch for the code which works with the most recent Ratatui release.
//!
//! [`latest`]: https://github.com/ratatui/ratatui/tree/latest
//! [`BarChart`]: https://docs.rs/ratatui/latest/ratatui/widgets/struct.BarChart.html

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{
        calendar::{CalendarEventStore, DateStyler, Monthly},
        Paragraph,
    },
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

/// Run the application.
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut style_index = 0;
    loop {
        terminal.draw(|frame| draw(frame, style_index))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Enter => style_index = (style_index + 1) % 6,
                    _ => {}
                }
            }
        }
    }
}

/// Draw the UI with a calendar.
fn draw(frame: &mut Frame, style_index: usize) {
    let vertical = Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]);
    let [text_area, area] = vertical.areas(frame.area());

    frame.render_widget(
        Paragraph::new(vec![
            Line::from("Calendar Example. Press q to quit, Enter to change the style".bold())
                .centered(),
            Line::from(format!(
                "Current style: {}",
                match style_index {
                    0 => "default",
                    1 => "show surrounding",
                    2 => "show weekdays header",
                    3 => "show surrounding and weekdays header",
                    4 => "show month header",
                    5 => "show month header and weekdays header",
                    _ => unreachable!(),
                }
            ))
            .centered(),
        ]),
        text_area,
    );

    let area = area.inner(Margin {
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
    let columns = rows.iter().flat_map(|row| {
        Layout::horizontal([Constraint::Ratio(1, 4); 4])
            .split(*row)
            .to_vec()
    });
    for column in columns {
        let calendar = match style_index {
            0 => default_calendar(start.month(), start.year(), &list),
            1 => example_calendar1(start.month(), start.year(), &list),
            2 => example_calendar2(start.month(), start.year(), &list),
            3 => example_calendar3(start.month(), start.year(), &list),
            4 => example_calendar4(start.month(), start.year(), &list),
            5 => example_calendar5(start.month(), start.year(), &list),
            _ => unreachable!(),
        };
        frame.render_widget(calendar, column);
        start = start.replace_month(start.month().next()).unwrap();
    }
}

fn default_calendar<'a, DS: DateStyler>(m: Month, y: i32, date_styler: DS) -> Monthly<'a, DS> {
    let default_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .bg(Color::Rgb(50, 50, 50));

    Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), date_styler)
        .show_month_header(Style::default())
        .default_style(default_style)
}

fn example_calendar1<'a, DS: DateStyler>(m: Month, y: i32, date_styler: DS) -> Monthly<'a, DS> {
    let default_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .bg(Color::Rgb(50, 50, 50));

    Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), date_styler)
        .show_surrounding(default_style)
        .default_style(default_style)
        .show_month_header(Style::default())
}

fn example_calendar2<'a, DS: DateStyler>(m: Month, y: i32, date_styler: DS) -> Monthly<'a, DS> {
    let header_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::DIM)
        .fg(Color::LightYellow);

    let default_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .bg(Color::Rgb(50, 50, 50));

    Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), date_styler)
        .show_weekdays_header(header_style)
        .default_style(default_style)
        .show_month_header(Style::default())
}

fn example_calendar3<'a, DS: DateStyler>(m: Month, y: i32, date_styler: DS) -> Monthly<'a, DS> {
    let header_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(Color::Green);

    let default_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .bg(Color::Rgb(50, 50, 50));

    Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), date_styler)
        .show_surrounding(Style::default().add_modifier(Modifier::DIM))
        .show_weekdays_header(header_style)
        .default_style(default_style)
        .show_month_header(Style::default())
}

fn example_calendar4<'a, DS: DateStyler>(m: Month, y: i32, date_styler: DS) -> Monthly<'a, DS> {
    let header_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(Color::Green);

    let default_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .bg(Color::Rgb(50, 50, 50));

    Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), date_styler)
        .show_weekdays_header(header_style)
        .default_style(default_style)
}

fn example_calendar5<'a, DS: DateStyler>(m: Month, y: i32, date_styler: DS) -> Monthly<'a, DS> {
    let header_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(Color::Green);

    let default_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .bg(Color::Rgb(50, 50, 50));

    Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), date_styler)
        .show_month_header(header_style)
        .default_style(default_style)
}

/// Makes a list of dates for the current year.
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
