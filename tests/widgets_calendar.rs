#![cfg(feature = "widget-calendar")]
use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    style::Style,
    widgets::{
        calendar::{CalendarEventStore, Monthly},
        Widget,
    },
    Terminal,
};
use time::{Date, Month};

#[track_caller]
fn test_render<W: Widget>(widget: W, expected: Buffer, size: (u16, u16)) {
    let backend = TestBackend::new(size.0, size.1);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| f.render_widget(widget, f.size()))
        .unwrap();
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn days_layout() {
    let c = Monthly::new(
        Date::from_calendar_date(2023, Month::January, 1).unwrap(),
        CalendarEventStore::default(),
    );
    let expected = Buffer::with_lines(vec![
        "  1  2  3  4  5  6  7",
        "  8  9 10 11 12 13 14",
        " 15 16 17 18 19 20 21",
        " 22 23 24 25 26 27 28",
        " 29 30 31",
    ]);
    test_render(c, expected, (21, 5));
}

#[test]
fn days_layout_show_surrounding() {
    let c = Monthly::new(
        Date::from_calendar_date(2023, Month::December, 1).unwrap(),
        CalendarEventStore::default(),
    )
    .show_surrounding(Style::default());
    let expected = Buffer::with_lines(vec![
        " 26 27 28 29 30  1  2",
        "  3  4  5  6  7  8  9",
        " 10 11 12 13 14 15 16",
        " 17 18 19 20 21 22 23",
        " 24 25 26 27 28 29 30",
        " 31  1  2  3  4  5  6",
    ]);
    test_render(c, expected, (21, 6));
}

#[test]
fn show_month_header() {
    let c = Monthly::new(
        Date::from_calendar_date(2023, Month::January, 1).unwrap(),
        CalendarEventStore::default(),
    )
    .show_month_header(Style::default());
    let expected = Buffer::with_lines(vec![
        "    January 2023     ",
        "  1  2  3  4  5  6  7",
        "  8  9 10 11 12 13 14",
        " 15 16 17 18 19 20 21",
        " 22 23 24 25 26 27 28",
        " 29 30 31",
    ]);
    test_render(c, expected, (21, 6));
}

#[test]
fn show_weekdays_header() {
    let c = Monthly::new(
        Date::from_calendar_date(2023, Month::January, 1).unwrap(),
        CalendarEventStore::default(),
    )
    .show_weekdays_header(Style::default());
    let expected = Buffer::with_lines(vec![
        " Su Mo Tu We Th Fr Sa",
        "  1  2  3  4  5  6  7",
        "  8  9 10 11 12 13 14",
        " 15 16 17 18 19 20 21",
        " 22 23 24 25 26 27 28",
        " 29 30 31",
    ]);
    test_render(c, expected, (21, 6));
}

#[test]
fn show_combo() {
    let c = Monthly::new(
        Date::from_calendar_date(2023, Month::January, 1).unwrap(),
        CalendarEventStore::default(),
    )
    .show_weekdays_header(Style::default())
    .show_month_header(Style::default())
    .show_surrounding(Style::default());
    let expected = Buffer::with_lines(vec![
        "    January 2023     ",
        " Su Mo Tu We Th Fr Sa",
        "  1  2  3  4  5  6  7",
        "  8  9 10 11 12 13 14",
        " 15 16 17 18 19 20 21",
        " 22 23 24 25 26 27 28",
        " 29 30 31  1  2  3  4",
    ]);
    test_render(c, expected, (21, 7));
}
