//! # [Ratatui] calendar `calendar-explorer` example
//!
//! This is the most complete calendar example, and deliberately the most complex: it shows every
//! `Monthly` style option across a whole year. It marks holidays, seasons and fortnightly
//! paydays, computing the dates that move each year (US Thanksgiving and the paydays) with jiff
//! rather than listing them by hand. Move around with the arrow keys or `hjkl`, change month with
//! `n` and `p`, cycle styles with `s`, switch weeks between starting on Sunday and Monday with
//! `w`, and quit with `q` or Esc.
//!
//! For something easier to start from, see the smaller examples in the same folder: [`minimal`],
//! [`calendar`], and [`time`] or [`chrono`] for apps that use those crates for their own dates.
//!
//! The latest version of this example is available in the [calendar examples] folder in the
//! repository. The examples are designed to be run against the `main` branch of the GitHub
//! repository, so they may not compile with the latest release on crates.io.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [calendar examples]: https://github.com/ratatui/ratatui/blob/main/ratatui-calendar/examples
//! [`minimal`]: https://github.com/ratatui/ratatui/blob/main/ratatui-calendar/examples/minimal.rs
//! [`calendar`]: https://github.com/ratatui/ratatui/blob/main/ratatui-calendar/examples/calendar.rs
//! [`time`]: https://github.com/ratatui/ratatui/blob/main/ratatui-calendar/examples/time.rs
//! [`chrono`]: https://github.com/ratatui/ratatui/blob/main/ratatui-calendar/examples/chrono.rs

use std::fmt;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use jiff::civil::{Date, Weekday, date};
use jiff::{ToSpan, ZonedDateTime};
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use ratatui::{DefaultTerminal, Frame};

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::new().run(terminal))
}

/// The calendar explorer's state.
struct App {
    selected_date: Date,
    calendar_style: StyledCalendar,
    first_day_of_week: Weekday,
    exit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            selected_date: ZonedDateTime::now().date(),
            calendar_style: StyledCalendar::Default,
            first_day_of_week: Weekday::Sunday,
            exit: false,
        }
    }

    /// Run the application until the user quits.
    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                self.handle_key(key.code);
            }
        }
        Ok(())
    }

    /// Render the UI with a calendar.
    fn render(&self, frame: &mut Frame) {
        let header = Text::from_iter([
            Line::from("Calendar Example".bold()),
            Line::from(
                "<q/Esc> Quit | <s> Change Style | <w> Week Start | <n> Next Month | <p> Previous \
                 Month, <hjkl> Move",
            ),
            Line::from(format!(
                "Current date: {} | Current style: {} | Week starts: {:?}",
                self.selected_date, self.calendar_style, self.first_day_of_week
            )),
        ]);

        let [text_area, area] = frame.area().layout(&Layout::vertical([
            Constraint::Length(header.height() as u16),
            Constraint::Fill(1),
        ]));
        frame.render_widget(header.centered(), text_area);
        self.calendar_style
            .render_year(frame, area, self.selected_date, self.first_day_of_week)
            .unwrap();
    }

    fn handle_key(&mut self, code: KeyCode) {
        let step = match code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.exit = true;
                return;
            }
            KeyCode::Char('s') => {
                self.calendar_style = self.calendar_style.next();
                return;
            }
            KeyCode::Char('w') => {
                self.first_day_of_week = match self.first_day_of_week {
                    Weekday::Sunday => Weekday::Monday,
                    _ => Weekday::Sunday,
                };
                return;
            }
            KeyCode::Char('n') | KeyCode::Tab => 1.month(),
            KeyCode::Char('p') | KeyCode::BackTab => -1.month(),
            KeyCode::Char('h') | KeyCode::Left => -1.day(),
            KeyCode::Char('j') | KeyCode::Down => 1.week(),
            KeyCode::Char('k') | KeyCode::Up => -1.week(),
            KeyCode::Char('l') | KeyCode::Right => 1.day(),
            _ => return,
        };
        // saturate at the first and last dates jiff supports instead of panicking
        self.selected_date = self.selected_date.saturating_add(step);
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
    fn render_year(
        self,
        frame: &mut Frame,
        area: Rect,
        date: Date,
        first_day_of_week: Weekday,
    ) -> Result<()> {
        let events = events(date)?;

        let vertical = Layout::vertical([Constraint::Ratio(1, 3); 3]);
        let horizontal = &Layout::horizontal([Constraint::Ratio(1, 4); 4]);
        let areas = area
            .inner(Margin::new(1, 1))
            .layout_vec(&vertical)
            .into_iter()
            .flat_map(|row| row.layout_vec(horizontal));
        let months = date.first_of_year().series(1.month());
        for (area, month) in areas.zip(months) {
            self.render_month(frame, area, month, &events, first_day_of_week);
        }
        Ok(())
    }

    fn render_month(
        self,
        frame: &mut Frame,
        area: Rect,
        date: Date,
        events: &CalendarEventStore,
        first_day_of_week: Weekday,
    ) {
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
        frame.render_widget(calendar.first_day_of_week(first_day_of_week), area);
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
    const PAYDAY: Style = Style::new().fg(Color::Yellow);

    let mut list = CalendarEventStore::today(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Blue),
    );
    let y = selected_date.year();

    // new year's
    list.add(date(y, 1, 1), HOLIDAY);
    // next new_year's for December "show surrounding"
    let next_new_year = selected_date.last_of_year().tomorrow()?;
    list.add(next_new_year, HOLIDAY);
    // groundhog day
    list.add(date(y, 2, 2), HOLIDAY);
    // april fool's
    list.add(date(y, 4, 1), HOLIDAY);
    // earth day
    list.add(date(y, 4, 22), HOLIDAY);
    // star wars day
    list.add(date(y, 5, 4), HOLIDAY);
    // thanksgiving (US), the fourth Thursday of November
    let thanksgiving = date(y, 11, 1).nth_weekday_of_month(4, Weekday::Thursday)?;
    list.add(thanksgiving, HOLIDAY);
    // festivus
    list.add(date(y, 12, 23), HOLIDAY);
    // new year's eve
    list.add(date(y, 12, 31), HOLIDAY);

    // seasons
    // spring equinox
    list.add(date(y, 3, 22), SEASON);
    // summer solstice
    list.add(date(y, 6, 21), SEASON);
    // fall equinox
    list.add(date(y, 9, 22), SEASON);
    // winter solstice
    list.add(date(y, 12, 21), SEASON);

    // payday, every other Friday from the first Friday of the year
    let first_friday = date(y, 1, 1).nth_weekday_of_month(1, Weekday::Friday)?;
    for payday in first_friday
        .series(2.weeks())
        .take_while(|day| day.year() == y)
    {
        list.add(payday, PAYDAY);
    }

    // selected date
    list.add(selected_date, SELECTED);

    Ok(list)
}
