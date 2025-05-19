//! # [Ratatui] `Tabs` example
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
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Alignment, Constraint, Layout, Offset, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, Tabs};
use ratatui::{DefaultTerminal, Frame, symbols};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

/// Run the application.
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut selected_tab = 0;
    loop {
        terminal.draw(|frame| render(frame, selected_tab))?;
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('l') | KeyCode::Right | KeyCode::Tab => {
                    selected_tab = (selected_tab + 1) % 3;
                }
                KeyCode::Char('h') | KeyCode::Left => selected_tab = (selected_tab + 2) % 3,
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                _ => {}
            }
        }
    }
}

/// Render the UI with tabs.
fn render(frame: &mut Frame, selected_tab: usize) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [top, main] = vertical.areas(frame.area());

    let title = Line::from_iter([
        Span::from("Tabs Widget").bold(),
        Span::from(" (Press 'q' to quit, arrow keys to navigate tabs)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_content(frame, main, selected_tab);
    render_tabs(frame, main.offset(Offset { x: 1, y: 0 }), selected_tab);
}

/// Render the tabs.
pub fn render_tabs(frame: &mut Frame, area: Rect, selected_tab: usize) {
    let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3"])
        .style(Color::White)
        .highlight_style(Style::default().magenta().on_black().bold())
        .select(selected_tab)
        .divider(symbols::DOT)
        .padding(" ", " ");
    frame.render_widget(tabs, area);
}

/// Render the tab content.
pub fn render_content(frame: &mut Frame, area: Rect, selected_tab: usize) {
    let text = match selected_tab {
        0 => "Great terminal interfaces start with a single widget.".into(),
        1 => "In the terminal, we don't just render widgets; we create dreams.".into(),
        2 => "Render boldly, style with purpose.".bold(),
        _ => unreachable!(),
    };
    let block = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::bordered());
    frame.render_widget(block, area);
}
