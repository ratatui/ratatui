//! A Ratatui example that demonstrates how to render weather data using [`BarChart`] widget.
//!
//! Generates random temperature data for each hour of the day and renders it as a vertical bar.
//!
//! This example runs with the Ratatui library code in the branch that you are currently reading.
//! See the [`latest`] branch for the code which works with the most recent Ratatui release.
//!
//! [`latest`]: https://github.com/ratatui/ratatui/tree/latest
//! [`BarChart`]: https://docs.rs/ratatui/latest/ratatui/widgets/struct.BarChart.html

use color_eyre::Result;
use rand::{rng, Rng};
use ratatui::{
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup},
    Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let temperatures: Vec<u8> = (0..24).map(|_| rng().random_range(50..90)).collect();
    ratatui::run(|terminal| loop {
        terminal.draw(|frame| render(frame, &temperatures))?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    })
}

fn render(frame: &mut Frame, temperatures: &[u8]) {
    let [title, main] = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
        .spacing(1)
        .areas(frame.area());

    frame.render_widget("Weather demo".bold().into_centered_line(), title);
    frame.render_widget(vertical_barchart(temperatures), main);
}

/// Create a vertical bar chart from the temperatures data.
fn vertical_barchart(temperatures: &[u8]) -> BarChart {
    let bars: Vec<Bar> = temperatures
        .iter()
        .enumerate()
        .map(|(hour, value)| vertical_bar(hour, value))
        .collect();
    BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .bar_width(5)
}

fn vertical_bar(hour: usize, temperature: &u8) -> Bar {
    Bar::default()
        .value(u64::from(*temperature))
        .label(Line::from(format!("{hour:>02}:00")))
        .text_value(format!("{temperature:>3}°"))
        .style(temperature_style(*temperature))
        .value_style(temperature_style(*temperature).reversed())
}

/// create a yellow to red value based on the value (50-90)
fn temperature_style(value: u8) -> Style {
    let green = (255.0 * (1.0 - f64::from(value - 50) / 40.0)) as u8;
    let color = Color::Rgb(255, green, 0);
    Style::new().fg(color)
}
