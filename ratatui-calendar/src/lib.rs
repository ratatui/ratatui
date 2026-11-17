#![no_std]
// show the feature flags in the generated documentation
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/favicon.ico"
)]
#![warn(missing_docs)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::alloc_instead_of_core)]
//! **ratatui-calendar** provides a monthly calendar widget for [Ratatui] terminal user
//! interfaces.
//!
//! [Ratatui]: https://crates.io/crates/ratatui
//!
//! Most applications use this crate through `ratatui::widgets::calendar` by enabling the
//! `widget-calendar` feature of the `ratatui` crate (on by default via `all-widgets`). Depend on
//! `ratatui-calendar` directly if you are building a widget library, or if you want finer-grained
//! dependencies than the `ratatui` crate gives you.
//!
//! Dates are [`jiff::civil::Date`] values from the [jiff] crate. Use [`jiff::civil::date`] to
//! build one from a year, month and day.
//!
//! The calendar previously lived in the `calendar` module of `ratatui-widgets`. It moved to its
//! own crate because it is the only built-in widget whose public API uses a third-party type (the
//! date type), and a separate crate lets that dependency change without a breaking release of
//! `ratatui-widgets`.
//!
//! [jiff]: https://crates.io/crates/jiff
//!
//! # Installation
//!
//! Run the following command to add this crate to your project:
//!
//! ```sh
//! cargo add ratatui-calendar jiff
//! ```
//!
//! # Usage
//!
//! [`Monthly`] renders the month that contains its `display_date`, one row per week starting on
//! Sunday. Each day uses the default style unless the [`DateStyler`] returns a style for it, and
//! days outside the month are only drawn when [`Monthly::show_surrounding`] is set. Optional
//! headers show the month name and the weekdays, and a [`Block`] can wrap the calendar.
//!
//! [`CalendarEventStore`] is a ready-made [`DateStyler`] that maps dates to styles. Implement
//! [`DateStyler`] yourself to compute styles on the fly, for example from an events database.
//!
//! [`Monthly::width`] and [`Monthly::height`] report the size the calendar needs, which is useful
//! for laying it out with a fixed-size constraint.
//!
//! ```rust
//! use jiff::civil::date;
//! use ratatui_calendar::{CalendarEventStore, Monthly};
//! use ratatui_core::buffer::Buffer;
//! use ratatui_core::layout::Rect;
//! use ratatui_core::style::{Color, Modifier, Style};
//! use ratatui_core::widgets::Widget;
//! use ratatui_widgets::block::Block;
//!
//! let november = date(2026, 11, 1);
//!
//! let mut events = CalendarEventStore::default();
//! events.add(
//!     date(2026, 11, 26),
//!     Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
//! );
//!
//! let calendar = Monthly::new(november, events)
//!     .show_month_header(Style::new().add_modifier(Modifier::BOLD))
//!     .show_weekdays_header(Style::new().fg(Color::Cyan))
//!     .show_surrounding(Style::new().fg(Color::DarkGray))
//!     .block(Block::bordered().title("Calendar"));
//!
//! let area = Rect::new(0, 0, calendar.width(), calendar.height());
//! let mut buf = Buffer::empty(area);
//! calendar.render(area, &mut buf);
//! ```
//!
//! # Examples
//!
//! Run these with `cargo run -p ratatui-calendar --example <name>`.
//!
//! The main example is [`calendar-explorer`]. It is intentionally the most complex one, to show
//! off the features: every style option across a whole year, with holidays and recurring events
//! computed using jiff, and keyboard navigation.
//!
//! The smaller examples are easier to start from:
//!
//! - [`minimal`]: the current month in a bordered block.
//! - [`calendar`]: two months side by side with different styles.
//! - [`time`]: an app that keeps its own dates as `time::Date` and converts them at the boundary.
//! - [`chrono`]: the same for an app that uses `chrono::NaiveDate`.
//!
//! [`Block`]: ratatui_widgets::block::Block
//! [`calendar-explorer`]: https://github.com/ratatui/ratatui/blob/main/ratatui-calendar/examples/calendar-explorer.rs
//! [`minimal`]: https://github.com/ratatui/ratatui/blob/main/ratatui-calendar/examples/minimal.rs
//! [`calendar`]: https://github.com/ratatui/ratatui/blob/main/ratatui-calendar/examples/calendar.rs
//! [`time`]: https://github.com/ratatui/ratatui/blob/main/ratatui-calendar/examples/time.rs
//! [`chrono`]: https://github.com/ratatui/ratatui/blob/main/ratatui-calendar/examples/chrono.rs
//!
//! # Crate Organization
//!
//! `ratatui-calendar` is part of the Ratatui workspace. It builds on [`ratatui-core`] for the
//! widget traits and on [`ratatui-widgets`] for [`Block`], and the main [`ratatui`] crate
//! re-exports it as `ratatui::widgets::calendar`.
//!
//! **When to use `ratatui-calendar`:**
//!
//! - Building widget libraries that embed or wrap the calendar
//! - You want finer-grained dependencies than the `ratatui` crate
//!
//! **When to use the main [`ratatui`] crate:**
//!
//! - Building applications (recommended - includes everything you need)
//!
//! For detailed information about the workspace organization, see [ARCHITECTURE.md].
//!
//! [`ratatui`]: https://crates.io/crates/ratatui
//! [`ratatui-core`]: https://crates.io/crates/ratatui-core
//! [`ratatui-widgets`]: https://crates.io/crates/ratatui-widgets
//! [ARCHITECTURE.md]: https://github.com/ratatui/ratatui/blob/main/ARCHITECTURE.md
#![cfg_attr(feature = "document-features", doc = "\n## Features")]
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!
//! # Contributing
//!
//! Contributions are welcome! Please open an issue or submit a pull request on GitHub. For more
//! details on contributing, please see the [CONTRIBUTING](../CONTRIBUTING.md) document.
//!
//! # License
//!
//! This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use hashbrown::HashMap;
use jiff::ToSpan;
use jiff::civil::{Date, Weekday};
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Alignment, Constraint, Layout, Rect};
use ratatui_core::style::Style;
use ratatui_core::text::{Line, Span};
use ratatui_core::widgets::Widget;
use ratatui_widgets::block::{Block, BlockExt};

/// Display a month calendar for the month containing `display_date`
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Monthly<'a, DS: DateStyler> {
    display_date: Date,
    events: DS,
    show_surrounding: Option<Style>,
    show_weekday: Option<Style>,
    show_month: Option<Style>,
    default_style: Style,
    first_day_of_week: Weekday,
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
            first_day_of_week: Weekday::Sunday,
            block: None,
        }
    }

    /// Fill the calendar slots for days not in the current month also, this causes each line to be
    /// completely filled. If there is an event style for a date, this style will be patched with
    /// the event's style
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn show_surrounding<S: Into<Style>>(mut self, style: S) -> Self {
        self.show_surrounding = Some(style.into());
        self
    }

    /// Display a header containing weekday abbreviations
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn show_weekdays_header<S: Into<Style>>(mut self, style: S) -> Self {
        self.show_weekday = Some(style.into());
        self
    }

    /// Display a header containing the month and year
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn show_month_header<S: Into<Style>>(mut self, style: S) -> Self {
        self.show_month = Some(style.into());
        self
    }

    /// How to render otherwise unstyled dates
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn default_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.default_style = style.into();
        self
    }

    /// Start each week on `weekday` instead of Sunday
    ///
    /// The weekday header and the day grid both start on this day. For example, use
    /// [`Weekday::Monday`] for the ISO 8601 week used in much of Europe and Asia.
    ///
    /// ```rust
    /// use jiff::civil::{Weekday, date};
    /// use ratatui_calendar::{CalendarEventStore, Monthly};
    ///
    /// let calendar = Monthly::new(date(2026, 11, 1), CalendarEventStore::default())
    ///     .first_day_of_week(Weekday::Monday);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn first_day_of_week(mut self, weekday: Weekday) -> Self {
        self.first_day_of_week = weekday;
        self
    }

    /// Render the calendar within a [Block]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Return the width required to render the calendar.
    #[must_use]
    pub fn width(&self) -> u16 {
        const DAYS_PER_WEEK: u16 = 7;
        const GUTTER_WIDTH: u16 = 1;
        const DAY_WIDTH: u16 = 2;

        let mut width = DAYS_PER_WEEK * (GUTTER_WIDTH + DAY_WIDTH);
        if let Some(block) = &self.block {
            let (left, right) = block.horizontal_space();
            width = width.saturating_add(left).saturating_add(right);
        }
        width
    }

    /// Return the height required to render the calendar.
    #[must_use]
    pub fn height(&self) -> u16 {
        let mut height = u16::from(self.weeks())
            .saturating_add(u16::from(self.show_month.is_some()))
            .saturating_add(u16::from(self.show_weekday.is_some()));

        if let Some(block) = &self.block {
            let (top, bottom) = block.vertical_space();
            height = height.saturating_add(top).saturating_add(bottom);
        }

        height
    }

    /// Return a style with only the background from the default style
    const fn default_bg(&self) -> Style {
        match self.default_style.bg {
            None => Style::new(),
            Some(c) => Style::new().bg(c),
        }
    }

    /// All logic to style a date goes here.
    fn format_date(&self, date: Date) -> Span<'_> {
        if date.month() == self.display_date.month() {
            Span::styled(
                format!("{:2}", date.day()),
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
                    Span::styled(format!("{:2}", date.day()), style)
                }
            }
        }
    }
}

impl<DS: DateStyler> Widget for Monthly<'_, DS> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl<DS: DateStyler> Widget for &Monthly<'_, DS> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        if area.is_empty() {
            return;
        }

        self.block.as_ref().render(area, buf);
        let inner = self.block.inner_if_some(area);
        self.render_monthly(inner, buf);
    }
}

impl<DS: DateStyler> Monthly<'_, DS> {
    /// The days before the 1st in its first week.
    fn leading_days(&self) -> i8 {
        self.display_date
            .first_of_month()
            .weekday()
            .since(self.first_day_of_week)
    }

    /// How many week rows the month needs: the leading days plus the days of the month, rounded
    /// up to whole weeks. Both rendering and [`Monthly::height`] use this, so they always agree.
    fn weeks(&self) -> u8 {
        let days = u8::try_from(self.leading_days() + self.display_date.days_in_month())
            .expect("at most 6 leading days plus 31 days");
        days.div_ceil(7)
    }

    fn render_monthly(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(self.show_month.is_some().into()),
            Constraint::Length(self.show_weekday.is_some().into()),
            Constraint::Fill(1),
        ]);
        let [month_header, days_header, days_area] = layout.areas(area);

        // Draw the month name and year
        if let Some(style) = self.show_month {
            let header = self.display_date.strftime("%B %Y").to_string();
            Line::styled(header, style)
                .alignment(Alignment::Center)
                .render(month_header, buf);
        }

        // Draw days of week
        if let Some(style) = self.show_weekday {
            let header: String = self
                .first_day_of_week
                .cycle_forward()
                .take(7)
                .flat_map(|weekday| [" ", weekday_abbreviation(weekday)])
                .collect();
            Span::styled(header, style).render(days_header, buf);
        }

        // Start the calendar on the first day of the week on or before the 1st. The series ends
        // early only past the last date jiff supports (9999-12-31).
        let first_of_month = self.display_date.first_of_month();
        let leading_days = self.leading_days();
        let mut days = first_of_month
            .saturating_sub(leading_days.days())
            .series(1.day());
        // `Date::MIN` has no preceding dates to start the series from. Keep its weekday
        // alignment by rendering those cells as blanks before the first supported date.
        let mut leading_padding = if first_of_month == Date::MIN {
            leading_days
        } else {
            0
        };

        let weeks = usize::from(self.weeks());
        for row in days_area.rows().take(weeks) {
            let mut spans = Vec::with_capacity(14);
            for i in 0..7 {
                // Draw the gutter. Do it here so we can avoid worrying about
                // styling the ' ' in the format_date method
                if i == 0 {
                    spans.push(Span::styled(" ", Style::default()));
                } else {
                    spans.push(Span::styled(" ", self.default_bg()));
                }
                let day = if leading_padding > 0 {
                    leading_padding -= 1;
                    None
                } else {
                    days.next()
                };
                spans.push(day.map_or_else(
                    || Span::styled("  ", Style::default()),
                    |day| self.format_date(day),
                ));
            }
            buf.set_line(row.x, row.y, &spans.into(), row.width);
        }
    }
}

/// The two-letter English abbreviation for `weekday`, used in the weekday header.
const fn weekday_abbreviation(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Monday => "Mo",
        Weekday::Tuesday => "Tu",
        Weekday::Wednesday => "We",
        Weekday::Thursday => "Th",
        Weekday::Friday => "Fr",
        Weekday::Saturday => "Sa",
        Weekday::Sunday => "Su",
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
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[cfg(feature = "std")]
    pub fn today<S: Into<Style>>(style: S) -> Self {
        let mut res = Self::default();
        res.add(jiff::ZonedDateTime::now().date(), style.into());
        res
    }

    /// Add a date and style to the store
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// [`Color`]: ratatui_core::style::Color
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

/// Lets generic code that takes `impl AsRef<Monthly>` accept either an owned [`Monthly`] or a
/// reference to one, matching the other Ratatui widgets.
///
/// The standard library has no blanket `impl<T> AsRef<T> for T`, so each widget implements it.
impl<DS: DateStyler> AsRef<Self> for Monthly<'_, DS> {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use jiff::civil::date;
    use ratatui_core::style::{Color, Style};
    use ratatui_widgets::block::{Block, Padding};
    use rstest::rstest;

    use super::*;

    #[test]
    fn event_store() {
        let a = (date(2023, 1, 1), Style::default());
        let b = (
            date(2023, 1, 2),
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

    /// The week after 9999-12-31 is past the last date jiff supports, so rendering stops there
    /// instead of panicking.
    #[test]
    fn render_last_supported_month() {
        let calendar = Monthly::new(date(9999, 12, 1), CalendarEventStore::default())
            .show_surrounding(Style::default());
        let area = Rect::new(0, 0, calendar.width(), calendar.height());
        let mut buf = Buffer::empty(area);
        calendar.render(area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                " 28 29 30  1  2  3  4",
                "  5  6  7  8  9 10 11",
                " 12 13 14 15 16 17 18",
                " 19 20 21 22 23 24 25",
                " 26 27 28 29 30 31   ",
            ])
        );
    }

    #[test]
    fn monthly_implements_as_ref() {
        let date = date(2024, 1, 1);
        let calendar = Monthly::new(date, CalendarEventStore::default());
        let _: &Monthly<_> = calendar.as_ref();
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_today() {
        CalendarEventStore::today(Style::default());
    }

    #[test]
    fn render_in_minimal_buffer() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));
        let calendar = Monthly::new(date(1984, 1, 1), CalendarEventStore::default());
        // This should not panic, even if the buffer is too small to render the calendar.
        calendar.render(buffer.area, &mut buffer);
        assert_eq!(buffer, Buffer::with_lines([" "]));
    }

    #[test]
    fn render_in_zero_size_buffer() {
        let mut buffer = Buffer::empty(Rect::ZERO);
        let calendar = Monthly::new(date(1984, 1, 1), CalendarEventStore::default());
        // This should not panic, even if the buffer has zero size.
        calendar.render(buffer.area, &mut buffer);
    }

    #[test]
    fn calendar_width_reflects_grid_layout() {
        let date = date(2023, 1, 1);
        let calendar = Monthly::new(date, CalendarEventStore::default());
        assert_eq!(calendar.width(), 21);
    }

    #[test]
    fn calendar_height_counts_weeks_and_headers() {
        let date = date(2015, 2, 1);
        let base_calendar = Monthly::new(date, CalendarEventStore::default());
        assert_eq!(base_calendar.height(), 4);

        let decorated_calendar = Monthly::new(date, CalendarEventStore::default())
            .show_month_header(Style::default())
            .show_weekdays_header(Style::default());
        assert_eq!(decorated_calendar.height(), 6);
    }

    #[test]
    fn calendar_dimensions_examples() {
        // Feb 2015 starts Sunday and spans 4 rows.
        let feb_2015 = date(2015, 2, 1);
        let cal = Monthly::new(feb_2015, CalendarEventStore::default());
        assert_eq!(cal.width(), 21, "4w base width");
        assert_eq!(cal.height(), 4, "Feb 2015 rows");

        let cal = Monthly::new(feb_2015, CalendarEventStore::default())
            .show_month_header(Style::default())
            .show_weekdays_header(Style::default());
        assert_eq!(cal.height(), 6, "Headers add 2 rows");

        let block = Block::bordered().padding(Padding::new(2, 3, 1, 2));
        let cal = Monthly::new(feb_2015, CalendarEventStore::default()).block(block);
        assert_eq!(cal.width(), 28, "Padding widens width");
        assert_eq!(cal.height(), 9, "Padding grows height");

        // Feb 2024 starts Thursday and spans 5 rows.
        let feb_2024 = date(2024, 2, 1);
        let cal = Monthly::new(feb_2024, CalendarEventStore::default());
        assert_eq!(cal.width(), 21, "5w base width");
        assert_eq!(cal.height(), 5, "Feb 2024 rows");

        let cal = Monthly::new(feb_2024, CalendarEventStore::default())
            .show_month_header(Style::default())
            .show_weekdays_header(Style::default());
        assert_eq!(cal.height(), 7, "Headers add 2 rows (5w)");

        let cal = Monthly::new(feb_2024, CalendarEventStore::default()).block(Block::bordered());
        assert_eq!(cal.width(), 23, "Border adds 2 cols");
        assert_eq!(cal.height(), 7, "Border adds 2 rows");

        // Apr 2023 starts Saturday and spans 6 rows.
        let apr_2023 = date(2023, 4, 1);
        let cal = Monthly::new(apr_2023, CalendarEventStore::default());
        assert_eq!(cal.width(), 21, "6w base width");
        assert_eq!(cal.height(), 6, "Apr 2023 rows");

        let cal = Monthly::new(apr_2023, CalendarEventStore::default())
            .show_month_header(Style::default())
            .show_weekdays_header(Style::default());
        assert_eq!(cal.height(), 8, "Headers add 2 rows (6w)");

        let block = Block::bordered().padding(Padding::symmetric(1, 1));
        let cal = Monthly::new(apr_2023, CalendarEventStore::default()).block(block);
        assert_eq!(cal.width(), 25, "Symmetric padding width");
        assert_eq!(cal.height(), 10, "Symmetric padding height");
    }

    #[test]
    fn sunday_based_weeks_shapes() {
        let sunday_start = Monthly::new(date(2015, 2, 11), CalendarEventStore::default());
        let saturday_start = Monthly::new(date(2023, 4, 9), CalendarEventStore::default());
        let leap_year = Monthly::new(date(2024, 2, 29), CalendarEventStore::default());

        assert_eq!(sunday_start.weeks(), 4);
        assert_eq!(saturday_start.weeks(), 6);
        assert_eq!(leap_year.weeks(), 5);
    }

    #[test]
    fn monday_based_weeks_shapes() {
        // February 2015 starts on a Sunday, so it needs a fifth row when weeks start on Monday.
        let sunday_start = Monthly::new(date(2015, 2, 11), CalendarEventStore::default())
            .first_day_of_week(Weekday::Monday);
        // April 2023 starts on a Saturday.
        let saturday_start = Monthly::new(date(2023, 4, 9), CalendarEventStore::default())
            .first_day_of_week(Weekday::Monday);
        // June 2026 starts on a Monday.
        let monday_start = Monthly::new(date(2026, 6, 1), CalendarEventStore::default())
            .first_day_of_week(Weekday::Monday);

        assert_eq!(sunday_start.weeks(), 5);
        assert_eq!(saturday_start.weeks(), 5);
        assert_eq!(monday_start.weeks(), 5);
    }

    #[test]
    fn render_monday_start() {
        let calendar = Monthly::new(date(2015, 2, 1), CalendarEventStore::default())
            .first_day_of_week(Weekday::Monday)
            .show_weekdays_header(Style::default())
            .show_surrounding(Style::default());
        let area = Rect::new(0, 0, calendar.width(), calendar.height());
        let mut buf = Buffer::empty(area);
        calendar.render(area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                " Mo Tu We Th Fr Sa Su",
                " 26 27 28 29 30 31  1",
                "  2  3  4  5  6  7  8",
                "  9 10 11 12 13 14 15",
                " 16 17 18 19 20 21 22",
                " 23 24 25 26 27 28  1",
            ])
        );
    }

    #[rstest]
    fn render_calendar_outside_buffer_is_noop(
        #[values(
            Rect::new(0, 10, 10, 8),
            Rect::new(40, 10, 10, 8),
            Rect::new(10, 0, 30, 10),
            Rect::new(10, 18, 30, 8),
            Rect::new(10, 10, 0, 8),
            Rect::new(10, 10, 30, 0)
        )]
        area: Rect,
    ) {
        let buffer_area = Rect::new(10, 10, 30, 8);
        let mut buf = Buffer::filled(buffer_area, ratatui_core::buffer::Cell::new("x"));
        let expected = buf.clone();
        let calendar = Monthly::new(date(2026, 10, 8), CalendarEventStore::default());
        calendar.render(area, &mut buf);
        assert_eq!(buf, expected, "rendering into {area:?}");
    }

    #[test]
    fn render_calendar_clips_to_offset_buffer() {
        let area = Rect::new(10, 10, 30, 8);
        let mut buf = Buffer::empty(area);
        let mut expected = Buffer::empty(area);
        let widget = Monthly::new(date(2026, 10, 8), CalendarEventStore::default());
        (&widget).render(Rect::new(9, 9, 40, 10), &mut buf);
        (&widget).render(area, &mut expected);
        assert_eq!(buf, expected);
        assert_ne!(buf, Buffer::empty(area));
    }

    #[test]
    fn render_calendar_stays_within_requested_rows() {
        let buffer_area = Rect::new(10, 10, 30, 8);
        let mut buf = Buffer::filled(buffer_area, ratatui_core::buffer::Cell::new("x"));
        let calendar = Monthly::new(date(2026, 10, 8), CalendarEventStore::default());
        calendar.render(Rect::new(10, 11, 30, 1), &mut buf);
        assert_ne!(buf[(11, 11)].symbol(), "x");
        for y in [10, 12, 13, 14, 15, 16, 17] {
            for x in 10..40 {
                assert_eq!(buf[(x, y)].symbol(), "x");
            }
        }
    }
}
