//! # [Ratatui] [`Block`] with collapsed borders example
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
use crossterm::event;
use ratatui::layout::{Constraint, Layout, Spacing};
use ratatui::style::Stylize;
use ratatui::symbols::merge::MergeStrategy;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType};
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
    loop {
        terminal.draw(render)?;
        if event::read()?.is_key_press() {
            return Ok(());
        }
    }
}

/// Render the UI with various blocks.
fn render(frame: &mut Frame) {
    let main_layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
    let horizontal = Layout::horizontal([Constraint::Fill(1); 2]).spacing(Spacing::Overlap(1));
    let vertical = Layout::vertical([Constraint::Fill(1); 2]).spacing(Spacing::Overlap(1));
    let [title, main] = main_layout.areas(frame.area());
    let [left, right] = horizontal.areas(main);
    let [top, bottom] = vertical.areas(right);

    let line = Line::from_iter([
        Span::from("Block With Collapsed Borders").bold(),
        Span::from(" (Press 'q' to quit)"),
    ]);
    frame.render_widget(line.centered(), title);

    let left_block = Block::bordered()
        .title("Left Block")
        .merge_borders(MergeStrategy::Exact);
    let top_block = Block::bordered()
        .border_type(BorderType::Thick)
        .title("Top Block")
        .merge_borders(MergeStrategy::Exact);
    let bottom_block = Block::bordered()
        .title("Bottom Block")
        .merge_borders(MergeStrategy::Exact);

    frame.render_widget(left_block, left);
    frame.render_widget(bottom_block, bottom);
    frame.render_widget(top_block, top);
}
