use itertools::Itertools;
use palette::Okhsv;
use ratatui::{
    prelude::*,
    widgets::{calendar::CalendarEventStore, *},
};
use time::OffsetDateTime;

use crate::{color_from_oklab, RgbSwatch, THEME};

pub struct WeatherTab {
    pub selected_row: usize,
}

impl WeatherTab {
    pub fn new(selected_row: usize) -> Self {
        Self { selected_row }
    }
}

impl Widget for WeatherTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        RgbSwatch.render(area, buf);
        let area = area.inner(&Margin {
            vertical: 1,
            horizontal: 2,
        });
        Clear.render(area, buf);
        Block::new().style(THEME.content).render(area, buf);

        let area = area.inner(&Margin {
            horizontal: 2,
            vertical: 1,
        });
        let [main, _, gauges] = area.split(&Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ]));
        let [calendar, charts] = main.split(&Layout::horizontal([
            Constraint::Length(23),
            Constraint::Min(0),
        ]));
        let [simple, horizontal] = charts.split(&Layout::vertical([
            Constraint::Length(29),
            Constraint::Min(0),
        ]));

        render_calendar(calendar, buf);
        render_simple_barchart(simple, buf);
        render_horizontal_barchart(horizontal, buf);
        render_gauge(self.selected_row, gauges, buf);
    }
}

fn render_calendar(area: Rect, buf: &mut Buffer) {
    let date = OffsetDateTime::now_utc().date();
    calendar::Monthly::new(date, CalendarEventStore::today(Style::new().red().bold()))
        .block(Block::new().padding(Padding::new(0, 0, 2, 0)))
        .show_month_header(Style::new().bold())
        .show_weekdays_header(Style::new().italic())
        .render(area, buf);
}

fn render_simple_barchart(area: Rect, buf: &mut Buffer) {
    let data = [
        ("Sat", 76),
        ("Sun", 69),
        ("Mon", 65),
        ("Tue", 67),
        ("Wed", 65),
        ("Thu", 69),
        ("Fri", 73),
    ];
    let data = data
        .into_iter()
        .map(|(label, value)| {
            Bar::default()
                .value(value)
                // This doesn't actually render correctly as the text is too wide for the bar
                // See https://github.com/ratatui-org/ratatui/issues/513 for more info
                // (the demo GIFs hack around this by hacking the calculation in bars.rs)
                .text_value(format!("{}°", value))
                .style(if value > 70 {
                    Style::new().fg(Color::Red)
                } else {
                    Style::new().fg(Color::Yellow)
                })
                .value_style(if value > 70 {
                    Style::new().fg(Color::Gray).bg(Color::Red).bold()
                } else {
                    Style::new().fg(Color::DarkGray).bg(Color::Yellow).bold()
                })
                .label(label.into())
        })
        .collect_vec();
    let group = BarGroup::default().bars(&data);
    BarChart::default()
        .data(group)
        .bar_width(3)
        .bar_gap(1)
        .render(area, buf);
}

fn render_horizontal_barchart(area: Rect, buf: &mut Buffer) {
    let bg = Color::Rgb(32, 48, 96);
    let data = [
        Bar::default().text_value("Winter 37-51".into()).value(51),
        Bar::default().text_value("Spring 40-65".into()).value(65),
        Bar::default().text_value("Summer 54-77".into()).value(77),
        Bar::default()
            .text_value("Fall 41-71".into())
            .value(71)
            .value_style(Style::new().bold()), // current season
    ];
    let group = BarGroup::default().label("GPU".into()).bars(&data);
    BarChart::default()
        .block(Block::new().padding(Padding::new(0, 0, 2, 0)))
        .direction(Direction::Horizontal)
        .data(group)
        .bar_gap(1)
        .bar_style(Style::new().fg(bg))
        .value_style(Style::new().bg(bg).fg(Color::Gray))
        .render(area, buf);
}

pub fn render_gauge(progress: usize, area: Rect, buf: &mut Buffer) {
    let percent = (progress * 3).min(100) as f64;

    render_line_gauge(percent, area, buf);
}

fn render_line_gauge(percent: f64, area: Rect, buf: &mut Buffer) {
    // cycle color hue based on the percent for a neat effect yellow -> red
    let hue = 90.0 - (percent as f32 * 0.6);
    let value = Okhsv::max_value();
    let fg = color_from_oklab(hue, Okhsv::max_saturation(), value);
    let bg = color_from_oklab(hue, Okhsv::max_saturation(), value * 0.5);
    let label = if percent < 100.0 {
        format!("Downloading: {}%", percent)
    } else {
        "Download Complete!".into()
    };
    LineGauge::default()
        .ratio(percent / 100.0)
        .label(label)
        .style(Style::new().light_blue())
        .gauge_style(Style::new().fg(fg).bg(bg))
        .line_set(symbols::line::THICK)
        .render(area, buf);
}
