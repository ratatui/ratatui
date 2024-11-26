//! # [Ratatui] `Block` example
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
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType},
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

/// Draw the UI with various blocks.
fn draw(frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Percentage(33); 3]).spacing(1);
    let [top, main] = vertical.areas(frame.area());
    let [left, middle, right] = horizontal.areas(main);

    let title = Line::from_iter([
        Span::from("Block Widget").bold(),
        Span::from(" (Press 'q' to quit)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_bordered_block(frame, left);
    render_styled_block(frame, middle);
    render_custom_bordered_block(frame, right);
}

/// Render a block with borders.
pub fn render_bordered_block(frame: &mut Frame, area: Rect) {
    let block = Block::bordered().title("Bordered block");
    frame.render_widget(block, area);
}

/// Render a styled block.
pub fn render_styled_block(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .style(Style::new().blue().on_black().bold().italic())
        .title("Styled block");
    frame.render_widget(block, area);
}

/// Render a block with custom borders.
pub fn render_custom_bordered_block(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().red())
        .title("Custom borders");
    frame.render_widget(block, area);
}
