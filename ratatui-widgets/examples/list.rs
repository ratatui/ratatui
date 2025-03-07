//! # [Ratatui] `List` example
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
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{List, ListDirection, ListState},
    Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut list_state = ListState::default();
    list_state.select_first();
    ratatui::run(|terminal| loop {
        terminal.draw(|frame| render(frame, &mut list_state))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => list_state.select_next(),
                    KeyCode::Up | KeyCode::Char('k') => list_state.select_previous(),
                    _ => {}
                }
            }
        }
    })
}

/// Draw the UI with various lists.
fn render(frame: &mut Frame, list_state: &mut ListState) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ])
    .spacing(1);
    let [top, first, second] = vertical.areas(frame.area());

    let title = Line::from_iter([
        Span::from("List Widget").bold(),
        Span::from(" (Press 'q' to quit and arrow keys to navigate)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_list(frame, first, list_state);
    render_bottom_list(frame, second);
}

/// Render a list.
pub fn render_list(frame: &mut Frame, area: Rect, list_state: &mut ListState) {
    let items = ["Item 1", "Item 2", "Item 3", "Item 4"];
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, list_state);
}

/// Render a bottom-to-top list.
pub fn render_bottom_list(frame: &mut Frame, area: Rect) {
    let items = [
        "[Remy]: I'm building one now.\nIt even supports multiline text!",
        "[Gusteau]: With enough passion, yes.",
        "[Remy]: But can anyone build a TUI in Rust?",
        "[Gusteau]: Anyone can cook!",
    ];
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Style::new().yellow().italic())
        .highlight_symbol("> ".red())
        .scroll_padding(1)
        .direction(ListDirection::BottomToTop)
        .repeat_highlight_symbol(true);

    let mut state = ListState::default();
    state.select_first();

    frame.render_stateful_widget(list, area, &mut state);
}
