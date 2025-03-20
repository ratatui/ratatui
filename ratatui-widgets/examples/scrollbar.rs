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
use crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::symbols::scrollbar::Set;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::{DefaultTerminal, Frame};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

/// Run the application.
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut vertical = ScrollbarState::new(100);
    let mut horizontal = ScrollbarState::new(100);
    loop {
        terminal.draw(|frame| draw(frame, &mut vertical, &mut horizontal))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break Ok(()),
                KeyCode::Down | KeyCode::Char('j') => vertical.next(),
                KeyCode::Up | KeyCode::Char('k') => vertical.prev(),
                KeyCode::Right | KeyCode::Char('l') => horizontal.next(),
                KeyCode::Left | KeyCode::Char('h') => horizontal.prev(),
                _ => {}
            }
        }
    }
}

/// Draw the UI with vertical/horizontal scrollbars.
fn draw(frame: &mut Frame, vertical: &mut ScrollbarState, horizontal: &mut ScrollbarState) {
    let vertical_layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [top, main] = vertical_layout.areas(frame.area());

    let title = Line::from_iter([
        Span::from("Scrollbar Widget").bold(),
        Span::from(" (Press 'q' to quit, arrow keys to scroll)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_content(frame, main, vertical, horizontal);
    render_vertical_scrollbar(frame, main, vertical);
    render_horizontal_scrollbar(frame, main, horizontal);
}

/// Render a vertical scrollbar on the right side of the area.
pub fn render_vertical_scrollbar(frame: &mut Frame, area: Rect, vertical: &mut ScrollbarState) {
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            vertical: 1,
            horizontal: 0,
        }),
        vertical,
    );
}

/// Render a horizontal scrollbar at the bottom of the area.
pub fn render_horizontal_scrollbar(frame: &mut Frame, area: Rect, horizontal: &mut ScrollbarState) {
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

    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            vertical: 0,
            horizontal: 1,
        }),
        horizontal,
    );
}

/// Render some content.
fn render_content(
    frame: &mut Frame,
    area: Rect,
    vertical: &ScrollbarState,
    horizontal: &ScrollbarState,
) {
    let content = vec![
        Line::from("This is a paragraph with a vertical and horizontal scrollbar."),
        Line::from_iter(["Lorem ipsum dolor sit amet, consectetur adipiscing elit.".repeat(10)]),
        Line::from_iter([
            "Horizontal: ".bold(),
            horizontal.get_position().to_string().yellow(),
        ]),
        Line::from_iter([
            "Vertical: ".bold(),
            vertical.get_position().to_string().yellow(),
        ]),
    ];
    frame.render_widget(
        Paragraph::new(content).scroll((
            vertical.get_position() as u16,
            horizontal.get_position() as u16,
        )),
        area,
    );
}
