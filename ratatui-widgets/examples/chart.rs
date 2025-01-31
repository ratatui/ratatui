//! # [Ratatui] `Chart` example
//!
//! The latest version of this example is available in the [widget examples] folder in the
//! repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [widget examples]: https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event},
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    symbols::Marker,
    text::{Line, Span},
    widgets::{Axis, Chart, Dataset, GraphType},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

/// Run the application.
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

/// Draw the UI with a chart.
fn draw(frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [top, main] = vertical.areas(frame.area());

    let title = Line::from_iter([
        Span::from("Chart Widget").bold(),
        Span::from(" (Press 'q' to quit)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_chart(frame, main);
}

/// Render a chart going upward.
pub fn render_chart(frame: &mut Frame, area: Rect) {
    let dataset = Dataset::default()
        .name("Stonks")
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Color::Blue)
        .data(&[
            (0.0, 1.0),
            (1.0, 3.0),
            (2.0, 0.5),
            (3.0, 2.0),
            (4.0, 0.8),
            (5.0, 4.0),
            (6.0, 1.0),
            (7.0, 6.0),
            (8.0, 3.0),
            (10.0, 10.0),
        ]);

    let x_axis = Axis::default()
        .title("Hustle".blue())
        .bounds([0.0, 10.0])
        .tick_marks(true)
        .labels([
            "0%", "10%", "20%", "30%", "40%", "50%", "60%", "70%", "80%", "90%", "100%",
        ]);

    let y_axis = Axis::default()
        .title("Profit".blue())
        .bounds([0.0, 10.0])
        .tick_marks(true)
        .labels(["0", "2.5", "5", "7.5", "10"]);

    let chart = Chart::new(vec![dataset])
        .x_axis(x_axis)
        .y_axis(y_axis)
        .show_grid(true);
    frame.render_widget(chart, area);
}
