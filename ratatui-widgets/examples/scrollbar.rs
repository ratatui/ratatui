//! # [Ratatui] `Scrollbar` example
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
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Stylize},
    symbols::scrollbar::Set,
    text::{Line, Span},
    widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState},
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

/// Draw the UI with vertical/horizontal scrollbars.
fn draw(frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [top, main] = vertical.areas(frame.area());

    let title = Line::from_iter([
        Span::from("Scrollbar Widget").bold(),
        Span::from(" (Press 'q' to quit)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_vertical_scrollbar(frame, main);
    render_horizontal_scrollbar(frame, main);
}

/// Render a vertical scrollbar on the right side of the area.
pub fn render_vertical_scrollbar(frame: &mut Frame, area: Rect) {
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    let mut scrollbar_state = ScrollbarState::new(10).position(5);
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut scrollbar_state,
    );
}

/// Render a horizontal scrollbar at the bottom of the area.
pub fn render_horizontal_scrollbar(frame: &mut Frame, area: Rect) {
    let scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
        .symbols(Set {
            track: "-",
            thumb: "▮",
            begin: "<",
            end: ">",
        })
        .track_style(Color::Yellow)
        .begin_style(Color::Green)
        .end_style(Color::Red);

    let mut scrollbar_state = ScrollbarState::new(100).position(20);
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            vertical: 0,
            horizontal: 1,
        }),
        &mut scrollbar_state,
    );
}
