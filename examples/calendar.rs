use std::{error::Error, io, rc::Rc};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::calendar::*};
use time::{Date, Month, OffsetDateTime};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        let _ = terminal.draw(|f| draw(f));

        if let Event::Key(key) = event::read()? {
            #[allow(clippy::single_match)]
            match key.code {
                KeyCode::Char(_) => {
                    break;
                }
                _ => {}
            };
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn draw<B: Backend>(f: &mut Frame<B>) {
    let app_area = f.size();

    let calarea = Rect {
        x: app_area.x + 1,
        y: app_area.y + 1,
        height: app_area.height - 1,
        width: app_area.width - 1,
    };

    let mut start = OffsetDateTime::now_local()
        .unwrap()
        .date()
        .replace_month(Month::January)
        .unwrap()
        .replace_day(1)
        .unwrap();

    let list = make_dates(start.year());

    for chunk in split_rows(&calarea)
        .iter()
        .flat_map(|row| split_cols(row).to_vec())
    {
        let cal = cals::get_cal(start.month(), start.year(), &list);
        f.render_widget(cal, chunk);
        start = start.replace_month(start.month().next()).unwrap();
    }
}

fn split_rows(area: &Rect) -> Rc<[Rect]> {
    let list_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        );

    list_layout.split(*area)
}

fn split_cols(area: &Rect) -> Rc<[Rect]> {
    let list_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        );

    list_layout.split(*area)
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
    use super::*;

    pub(super) fn get_cal<'a, S: DateStyler>(m: Month, y: i32, es: S) -> Monthly<'a, S> {
        use Month::*;
        match m {
            May => example1(m, y, es),
            June => example2(m, y, es),
            July => example3(m, y, es),
            December => example3(m, y, es),
            February => example4(m, y, es),
            November => example5(m, y, es),
            _ => default(m, y, es),
        }
    }

    fn default<'a, S: DateStyler>(m: Month, y: i32, es: S) -> Monthly<'a, S> {
        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_month_header(Style::default())
            .default_style(default_style)
    }

    fn example1<'a, S: DateStyler>(m: Month, y: i32, es: S) -> Monthly<'a, S> {
        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_surrounding(default_style)
            .default_style(default_style)
            .show_month_header(Style::default())
    }

    fn example2<'a, S: DateStyler>(m: Month, y: i32, es: S) -> Monthly<'a, S> {
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

    fn example3<'a, S: DateStyler>(m: Month, y: i32, es: S) -> Monthly<'a, S> {
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

    fn example4<'a, S: DateStyler>(m: Month, y: i32, es: S) -> Monthly<'a, S> {
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

    fn example5<'a, S: DateStyler>(m: Month, y: i32, es: S) -> Monthly<'a, S> {
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
