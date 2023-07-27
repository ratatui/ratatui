//! A simple calendar widget. `(feature: widget-calendar)`
//!
//!
//!
//! The [`Monthly`] widget will display a calendar for the monh provided in `display_date`. Days are
//! styled using the default style unless:
//! * `show_surrounding` is set, then days not in the `display_date` month will use that style.
//! * a style is returned by the [`DateStyler`] for the day
//!
//! [`Monthly`] has several controls for what should be displayed
use std::collections::HashMap;

use time::{Date, Duration, OffsetDateTime};

use crate::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Span,
    widgets::{Block, Widget},
};

/// Display a month calendar for the month containing `display_date`
#[derive(Debug, Clone)]
pub struct Monthly<'a, S: DateStyler> {
    display_date: Date,
    events: S,
    show_surrounding: Option<Style>,
    show_weekday: Option<Style>,
    show_month: Option<Style>,
    default_style: Style,
    block: Option<Block<'a>>,
}

impl<'a, S: DateStyler> Monthly<'a, S> {
    /// Construct a calendar for the `display_date` and highlight the `events`
    pub fn new(display_date: Date, events: S) -> Self {
        Self {
            display_date,
            events,
            show_surrounding: None,
            show_weekday: None,
            show_month: None,
            default_style: Style::default(),
            block: None,
        }
    }

    /// Fill the calendar slots for days not in the current month also, this causes each line to be
    /// completely filled. If there is an event style for a date, this style will be patched with
    /// the event's style
    pub fn show_surrounding(mut self, style: Style) -> Self {
        self.show_surrounding = Some(style);
        self
    }

    /// Display a header containing weekday abbreviations
    pub fn show_weekdays_header(mut self, style: Style) -> Self {
        self.show_weekday = Some(style);
        self
    }

    /// Display a header containing the month and year
    pub fn show_month_header(mut self, style: Style) -> Self {
        self.show_month = Some(style);
        self
    }

    /// How to render otherwise unstyled dates
    pub fn default_style(mut self, s: Style) -> Self {
        self.default_style = s;
        self
    }

    /// Render the calendar within a [Block](crate::widgets::Block)
    pub fn block(mut self, b: Block<'a>) -> Self {
        self.block = Some(b);
        self
    }

    /// Return a style with only the background from the default style
    fn default_bg(&self) -> Style {
        match self.default_style.bg {
            None => Style::default(),
            Some(c) => Style::default().bg(c),
        }
    }

    /// All logic to style a date goes here.
    fn format_date(&self, date: Date) -> Span {
        if date.month() != self.display_date.month() {
            match self.show_surrounding {
                None => Span::styled("  ", self.default_bg()),
                Some(s) => {
                    let style = self
                        .default_style
                        .patch(s)
                        .patch(self.events.get_style(date));
                    Span::styled(format!("{:2?}", date.day()), style)
                }
            }
        } else {
            Span::styled(
                format!("{:2?}", date.day()),
                self.default_style.patch(self.events.get_style(date)),
            )
        }
    }
}

impl<'a, S: DateStyler> Widget for Monthly<'a, S> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        // Block is used for borders and such
        // Draw that first, and use the blank area inside the block for our own purposes
        let mut area = match self.block.take() {
            None => area,
            Some(b) => {
                let inner = b.inner(area);
                b.render(area, buf);
                inner
            }
        };

        // Draw the month name and year
        if let Some(style) = self.show_month {
            let line = Span::styled(
                format!("{} {}", self.display_date.month(), self.display_date.year()),
                style,
            );
            // cal is 21 cells wide, so hard code the 11
            let x_off = 11_u16.saturating_sub(line.width() as u16 / 2);
            buf.set_line(area.x + x_off, area.y, &line.into(), area.width);
            area.y += 1
        }

        // Draw days of week
        if let Some(style) = self.show_weekday {
            let days = String::from(" Su Mo Tu We Th Fr Sa");
            buf.set_string(area.x, area.y, days, style);
            area.y += 1;
        }

        // Set the start of the calendar to the Sunday before the 1st (or the sunday of the first)
        let first_of_month = self.display_date.replace_day(1).unwrap();
        let offset = Duration::days(first_of_month.weekday().number_days_from_sunday().into());
        let mut curr_day = first_of_month - offset;

        // go through all the weeks containing a day in the target month.
        while curr_day.month() as u8 != self.display_date.month().next() as u8 {
            let mut spans = Vec::with_capacity(14);
            for i in 0..7 {
                // Draw the gutter. Do it here so we can avoid worrying about
                // styling the ' ' in the format_date method
                if i == 0 {
                    spans.push(Span::styled(" ", Style::default()));
                } else {
                    spans.push(Span::styled(" ", self.default_bg()));
                }
                spans.push(self.format_date(curr_day));
                curr_day += Duration::DAY;
            }
            buf.set_line(area.x, area.y, &spans.into(), area.width);
            area.y += 1;
        }
    }
}

/// Provides a method for styling a given date. [Monthly] is generic on this trait, so any type
/// that implements this trait can be used.
pub trait DateStyler {
    /// Given a date, return a style for that date
    fn get_style(&self, date: Date) -> Style;
}

/// A simple `DateStyler` based on a [`HashMap`]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CalendarEventStore(pub HashMap<Date, Style>);

impl CalendarEventStore {
    /// Construct a store that has the current date styled.
    pub fn today(style: Style) -> Self {
        let mut res = Self::default();
        res.add(OffsetDateTime::now_local().unwrap().date(), style);
        res
    }

    /// Add a date and style to the store
    pub fn add(&mut self, date: Date, style: Style) {
        // to simplify style nonsense, last write wins
        let _ = self.0.insert(date, style);
    }

    /// Helper for trait impls
    fn lookup_style(&self, date: Date) -> Style {
        self.0.get(&date).copied().unwrap_or_default()
    }
}

impl DateStyler for CalendarEventStore {
    fn get_style(&self, date: Date) -> Style {
        self.lookup_style(date)
    }
}

impl DateStyler for &CalendarEventStore {
    fn get_style(&self, date: Date) -> Style {
        self.lookup_style(date)
    }
}

impl Default for CalendarEventStore {
    fn default() -> Self {
        Self(HashMap::with_capacity(4))
    }
}

#[cfg(test)]
mod tests {
    use time::Month;

    use super::*;
    use crate::style::Color;

    #[test]
    fn event_store() {
        let a = (
            Date::from_calendar_date(2023, Month::January, 1).unwrap(),
            Style::default(),
        );
        let b = (
            Date::from_calendar_date(2023, Month::January, 2).unwrap(),
            Style::default().bg(Color::Red).fg(Color::Blue),
        );
        let mut s = CalendarEventStore::default();
        s.add(b.0, b.1);

        assert_eq!(
            s.get_style(a.0),
            a.1,
            "Date not added to the styler should look up as Style::default()"
        );
        assert_eq!(
            s.get_style(b.0),
            b.1,
            "Date added to styler should return the provided style"
        );
    }
}
