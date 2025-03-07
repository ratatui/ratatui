//! A Ratatui example that demonstrates how to render calendar with different styles.
//!
//! Marks the holidays and seasons on the calendar.
//!
//! This example runs with the Ratatui library code in the branch that you are currently reading.
//! See the [`latest`] branch for the code which works with the most recent Ratatui release.
//!
//! [`latest`]: https://github.com/ratatui/ratatui/tree/latest
//! [`BarChart`]: https://docs.rs/ratatui/latest/ratatui/widgets/struct.BarChart.html

use std::fmt;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::calendar::{CalendarEventStore, Monthly},
    DefaultTerminal, Frame,
};
use time::{ext::NumericalDuration, Date, Month, OffsetDateTime};

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))
}

struct App {
    exit: bool,
    selected_date: Date,
    calendar_style: StyledCalendar,
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            selected_date: OffsetDateTime::now_local()
                .expect("cannot get current date")
                .date(),
            calendar_style: StyledCalendar::Default,
        }
    }
}

impl App {
    /// Run the application.
    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Draw the UI with a calendar.
    fn render(&self, frame: &mut Frame) {
        let header = Text::from_iter([
            Line::from("Calendar Example".bold()),
            Line::from(
                "<q> Quit | <s> Change Style | <n> Next Month | <p> Previous Month, <hjkl> Move",
            ),
            Line::from(format!(
                "Current date: {} | Current style: {}",
                self.selected_date, self.calendar_style
            )),
        ]);

        let vertical = Layout::vertical([
            Constraint::Length(header.height() as u16),
            Constraint::Fill(1),
        ]);
        let [text_area, area] = vertical.areas(frame.area());
        frame.render_widget(header.centered(), text_area);
        self.calendar_style
            .render_year(frame, area, self.selected_date)
            .unwrap();
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Char('s') => self.select_next_style(),
                    KeyCode::Char('n') | KeyCode::Tab => self.increment_month(),
                    KeyCode::Char('p') | KeyCode::BackTab => self.decrement_month(),
                    KeyCode::Char('h') | KeyCode::Left => self.selected_date -= 1.days(),
                    KeyCode::Char('j') | KeyCode::Down => self.selected_date += 1.weeks(),
                    KeyCode::Char('k') | KeyCode::Up => self.selected_date -= 1.weeks(),
                    KeyCode::Char('l') | KeyCode::Right => self.selected_date += 1.days(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn select_next_style(&mut self) {
        self.calendar_style = self.calendar_style.next();
    }

    fn increment_month(&mut self) {
        let date = &mut self.selected_date;
        if date.month() == Month::December {
            date.replace_year(date.year() + 1).unwrap();
        }
        date.replace_month(date.month().next()).unwrap();
    }

    fn decrement_month(&mut self) {
        let date = &mut self.selected_date;
        if date.month() == Month::January {
            date.replace_year(date.year() - 1).unwrap();
        }
        date.replace_month(date.month().previous()).unwrap();
    }
}

#[derive(Debug, Clone, Copy)]
enum StyledCalendar {
    Default,
    Surrounding,
    WeekdaysHeader,
    SurroundingAndWeekdaysHeader,
    MonthHeader,
    MonthAndWeekdaysHeader,
}

impl StyledCalendar {
    // Cycle through the different styles.
    const fn next(self) -> Self {
        match self {
            Self::Default => Self::Surrounding,
            Self::Surrounding => Self::WeekdaysHeader,
            Self::WeekdaysHeader => Self::SurroundingAndWeekdaysHeader,
            Self::SurroundingAndWeekdaysHeader => Self::MonthHeader,
            Self::MonthHeader => Self::MonthAndWeekdaysHeader,
            Self::MonthAndWeekdaysHeader => Self::Default,
        }
    }
}

impl fmt::Display for StyledCalendar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "Default"),
            Self::Surrounding => write!(f, "Show Surrounding"),
            Self::WeekdaysHeader => write!(f, "Show Weekdays Header"),
            Self::SurroundingAndWeekdaysHeader => write!(f, "Show Surrounding and Weekdays Header"),
            Self::MonthHeader => write!(f, "Show Month Header"),
            Self::MonthAndWeekdaysHeader => write!(f, "Show Month Header and Weekdays Header"),
        }
    }
}

impl StyledCalendar {
    fn render_year(self, frame: &mut Frame, area: Rect, date: Date) -> Result<()> {
        let events = events(date)?;

        let area = area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        });
        let rows = Layout::vertical([Constraint::Ratio(1, 3); 3]).split(area);
        let areas = rows.iter().flat_map(|row| {
            Layout::horizontal([Constraint::Ratio(1, 4); 4])
                .split(*row)
                .to_vec()
        });
        for (i, area) in areas.enumerate() {
            let month = date
                .replace_day(1)
                .unwrap()
                .replace_month(Month::try_from(i as u8 + 1).unwrap())
                .unwrap();
            self.render_month(frame, area, month, &events);
        }
        Ok(())
    }

    fn render_month(self, frame: &mut Frame, area: Rect, date: Date, events: &CalendarEventStore) {
        let calendar = match self {
            Self::Default => Monthly::new(date, events)
                .default_style(Style::new().bold().bg(Color::Rgb(50, 50, 50)))
                .show_month_header(Style::default()),
            Self::Surrounding => Monthly::new(date, events)
                .default_style(Style::new().bold().bg(Color::Rgb(50, 50, 50)))
                .show_month_header(Style::default())
                .show_surrounding(Style::new().dim()),
            Self::WeekdaysHeader => Monthly::new(date, events)
                .default_style(Style::new().bold().bg(Color::Rgb(50, 50, 50)))
                .show_month_header(Style::default())
                .show_weekdays_header(Style::new().bold().green()),
            Self::SurroundingAndWeekdaysHeader => Monthly::new(date, events)
                .default_style(Style::new().bold().bg(Color::Rgb(50, 50, 50)))
                .show_month_header(Style::default())
                .show_surrounding(Style::new().dim())
                .show_weekdays_header(Style::new().bold().green()),
            Self::MonthHeader => Monthly::new(date, events)
                .default_style(Style::new().bold().bg(Color::Rgb(50, 50, 50)))
                .show_month_header(Style::default())
                .show_month_header(Style::new().bold().green()),
            Self::MonthAndWeekdaysHeader => Monthly::new(date, events)
                .default_style(Style::new().bold().bg(Color::Rgb(50, 50, 50)))
                .show_month_header(Style::default())
                .show_weekdays_header(Style::new().bold().dim().light_yellow()),
        };
        frame.render_widget(calendar, area);
    }
}

/// Makes a list of dates for the current year.
fn events(selected_date: Date) -> Result<CalendarEventStore> {
    const SELECTED: Style = Style::new()
        .fg(Color::White)
        .bg(Color::Red)
        .add_modifier(Modifier::BOLD);
    const HOLIDAY: Style = Style::new()
        .fg(Color::Red)
        .add_modifier(Modifier::UNDERLINED);
    const SEASON: Style = Style::new()
        .fg(Color::Green)
        .bg(Color::Black)
        .add_modifier(Modifier::UNDERLINED);

    let mut list = CalendarEventStore::today(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Blue),
    );
    let y = selected_date.year();

    // new year's
    list.add(Date::from_calendar_date(y, Month::January, 1)?, HOLIDAY);
    // next new_year's for December "show surrounding"
    list.add(Date::from_calendar_date(y + 1, Month::January, 1)?, HOLIDAY);
    // groundhog day
    list.add(Date::from_calendar_date(y, Month::February, 2)?, HOLIDAY);
    // april fool's
    list.add(Date::from_calendar_date(y, Month::April, 1)?, HOLIDAY);
    // earth day
    list.add(Date::from_calendar_date(y, Month::April, 22)?, HOLIDAY);
    // star wars day
    list.add(Date::from_calendar_date(y, Month::May, 4)?, HOLIDAY);
    // festivus
    list.add(Date::from_calendar_date(y, Month::December, 23)?, HOLIDAY);
    // new year's eve
    list.add(Date::from_calendar_date(y, Month::December, 31)?, HOLIDAY);

    // seasons
    // spring equinox
    list.add(Date::from_calendar_date(y, Month::March, 22)?, SEASON);
    // summer solstice
    list.add(Date::from_calendar_date(y, Month::June, 21)?, SEASON);
    // fall equinox
    list.add(Date::from_calendar_date(y, Month::September, 22)?, SEASON);
    // winter solstice
    list.add(Date::from_calendar_date(y, Month::December, 21)?, SEASON);

    // selected date
    list.add(selected_date, SELECTED);

    Ok(list)
}
