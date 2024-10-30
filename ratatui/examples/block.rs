//! # [Ratatui] Block example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break Ok(());
            }
        }
    }
}

fn draw(frame: &mut Frame) {
    let (title_area, layout) = calculate_layout(frame.area());

    render_title(frame, title_area);

    let paragraph = placeholder_paragraph();

    render_borders(&paragraph, Borders::ALL, frame, layout[0][0]);
    render_borders(&paragraph, Borders::NONE, frame, layout[0][1]);
    render_borders(&paragraph, Borders::LEFT, frame, layout[1][0]);
    render_borders(&paragraph, Borders::RIGHT, frame, layout[1][1]);
    render_borders(&paragraph, Borders::TOP, frame, layout[2][0]);
    render_borders(&paragraph, Borders::BOTTOM, frame, layout[2][1]);

    render_border_type(&paragraph, BorderType::Plain, frame, layout[3][0]);
    render_border_type(&paragraph, BorderType::Rounded, frame, layout[3][1]);
    render_border_type(&paragraph, BorderType::Double, frame, layout[4][0]);
    render_border_type(&paragraph, BorderType::Thick, frame, layout[4][1]);

    render_styled_block(&paragraph, frame, layout[5][0]);
    render_styled_borders(&paragraph, frame, layout[5][1]);
    render_styled_title(&paragraph, frame, layout[6][0]);
    render_styled_title_content(&paragraph, frame, layout[6][1]);
    render_multiple_titles(&paragraph, frame, layout[7][0]);
    render_multiple_title_positions(&paragraph, frame, layout[7][1]);
    render_padding(&paragraph, frame, layout[8][0]);
    render_nested_blocks(&paragraph, frame, layout[8][1]);
}

/// Calculate the layout of the UI elements.
///
/// Returns a tuple of the title area and the main areas.
fn calculate_layout(area: Rect) -> (Rect, Vec<Vec<Rect>>) {
    let main_layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
    let block_layout = Layout::vertical([Constraint::Max(4); 9]);
    let [title_area, main_area] = main_layout.areas(area);
    let main_areas = block_layout
        .split(main_area)
        .iter()
        .map(|&area| {
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area)
                .to_vec()
        })
        .collect();
    (title_area, main_areas)
}

fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Block example. Press q to quit")
            .dark_gray()
            .alignment(Alignment::Center),
        area,
    );
}

fn placeholder_paragraph() -> Paragraph<'static> {
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true })
}

fn render_borders(paragraph: &Paragraph, border: Borders, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(border)
        .title(format!("Borders::{border:#?}"));
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_border_type(
    paragraph: &Paragraph,
    border_type: BorderType,
    frame: &mut Frame,
    area: Rect,
) {
    let block = Block::bordered()
        .border_type(border_type)
        .title(format!("BorderType::{border_type:#?}"));
    frame.render_widget(paragraph.clone().block(block), area);
}
fn render_styled_borders(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .border_style(Style::new().blue().on_white().bold().italic())
        .title("Styled borders");
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_styled_block(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .style(Style::new().blue().on_white().bold().italic())
        .title("Styled block");
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_styled_title(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .title("Styled title")
        .title_style(Style::new().blue().on_white().bold().italic());
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_styled_title_content(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let title = Line::from(vec![
        "Styled ".blue().on_white().bold().italic(),
        "title content".red().on_white().bold().italic(),
    ]);
    let block = Block::bordered().title(title);
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_multiple_titles(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .title("Multiple".blue().on_white().bold().italic())
        .title("Titles".red().on_white().bold().italic());
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_multiple_title_positions(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .title(Line::from("top left").left_aligned())
        .title(Line::from("top center").centered())
        .title(Line::from("top right").right_aligned())
        .title_bottom(Line::from("bottom left").left_aligned())
        .title_bottom(Line::from("bottom center").centered())
        .title_bottom(Line::from("bottom right").right_aligned());
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_padding(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .padding(Padding::new(5, 10, 1, 2))
        .title("Padding");
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_nested_blocks(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let outer_block = Block::bordered().title("Outer block");
    let inner_block = Block::bordered().title("Inner block");
    let inner = outer_block.inner(area);
    frame.render_widget(outer_block, area);
    frame.render_widget(paragraph.clone().block(inner_block), inner);
}
