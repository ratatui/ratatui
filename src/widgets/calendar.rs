//! A simple calendar widget. `(feature: widget-calendar)`
//!
//!
//!
//! The [`Monthly`] widget will display a calendar for the month provided in `display_date`. Days
//! are styled using the default style unless:
//! * `show_surrounding` is set, then days not in the `display_date` month will use that style.
//! * a style is returned by the [`DateStyler`] for the day
//!
//! [`Monthly`] has several controls for what should be displayed
use std::collections::HashMap;

use time::{Date, Duration, OffsetDateTime};

use crate::{prelude::*, widgets::Block};

/// Display a month calendar for the month containing `display_date`
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Monthly<'a, DS: DateStyler> {
    display_date: Date,
    events: DS,
    show_surrounding: Option<Style>,
    show_weekday: Option<Style>,
    show_month: Option<Style>,
    default_style: Style,
    block: Option<Block<'a>>,
}

impl<'a, DS: DateStyler> Monthly<'a, DS> {
    /// Construct a calendar for the `display_date` and highlight the `events`
    pub const fn new(display_date: Date, events: DS) -> Self {
        Self {
            display_date,
            events,
            show_surrounding: None,
            show_weekday: None,
            show_month: None,
            default_style: Style::new(),
            block: None,
        }
    }

    /// Fill the calendar slots for days not in the current month also, this causes each line to be
    /// completely filled. If there is an event style for a date, this style will be patched with
    /// the event's style
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn show_surrounding<S: Into<Style>>(mut self, style: S) -> Self {
        self.show_surrounding = Some(style.into());
        self
    }

    /// Display a header containing weekday abbreviations
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn show_weekdays_header<S: Into<Style>>(mut self, style: S) -> Self {
        self.show_weekday = Some(style.into());
        self
    }

    /// Display a header containing the month and year
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn show_month_header<S: Into<Style>>(mut self, style: S) -> Self {
        self.show_month = Some(style.into());
        self
    }

    /// How to render otherwise unstyled dates
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn default_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.default_style = style.into();
        self
    }

    /// Render the calendar within a [Block]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Return a style with only the background from the default style
    const fn default_bg(&self) -> Style {
        match self.default_style.bg {
            None => Style::new(),
            Some(c) => Style::new().bg(c),
        }
    }

    /// All logic to style a date goes here.
    fn format_date(&self, date: Date) -> Span {
        if date.month() == self.display_date.month() {
            Span::styled(
                format!("{:2?}", date.day()),
                self.default_style.patch(self.events.get_style(date)),
            )
        } else {
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
        }
    }
}

impl<DS: DateStyler> Widget for Monthly<'_, DS> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

impl<DS: DateStyler> WidgetRef for Monthly<'_, DS> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.block.render_ref(area, buf);
        let inner = self.block.inner_if_some(area);
        self.render_monthly(inner, buf);
    }
}

impl<DS: DateStyler> Monthly<'_, DS> {
    fn render_monthly(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(self.show_month.is_some().into()),
            Constraint::Length(self.show_weekday.is_some().into()),
            Constraint::Fill(1),
        ]);
        let [month_header, days_header, days_area] = layout.areas(area);

        // Draw the month name and year
        if let Some(style) = self.show_month {
            Line::styled(
                format!("{} {}", self.display_date.month(), self.display_date.year()),
                style,
            )
            .alignment(Alignment::Center)
            .render(month_header, buf);
        }

        // Draw days of week
        if let Some(style) = self.show_weekday {
            Span::styled(" Su Mo Tu We Th Fr Sa", style).render(days_header, buf);
        }

        // Set the start of the calendar to the Sunday before the 1st (or the sunday of the first)
        let first_of_month = self.display_date.replace_day(1).unwrap();
        let offset = Duration::days(first_of_month.weekday().number_days_from_sunday().into());
        let mut curr_day = first_of_month - offset;

        let mut y = days_area.y;
        // go through all the weeks containing a day in the target month.
        while curr_day.month() != self.display_date.month().next() {
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
            if buf.area.height > y {
                buf.set_line(days_area.x, y, &spans.into(), area.width);
            }
            y += 1;
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
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    pub fn today<S: Into<Style>>(style: S) -> Self {
        let mut res = Self::default();
        res.add(
            OffsetDateTime::now_local()
                .unwrap_or_else(|_| OffsetDateTime::now_utc())
                .date(),
            style.into(),
        );
        res
    }

    /// Add a date and style to the store
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    pub fn add<S: Into<Style>>(&mut self, date: Date, style: S) {
        // to simplify style nonsense, last write wins
        let _ = self.0.insert(date, style.into());
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

    #[test]
    fn test_today() {
        CalendarEventStore::today(Style::default());
    }
}
