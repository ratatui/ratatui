//! # [Ratatui] `Shadow` example
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
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Offset, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Clear, Paragraph, Shadow, Wrap, dimmed};

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| {
        loop {
            terminal.draw(render)?;
            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}

fn render(frame: &mut Frame) {
    let [title_area, content_area] = frame
        .area()
        .layout(&Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1));
    let title = Line::from("Shadow Widget (press any key to quit)").bold();
    frame.render_widget(title.centered(), title_area);

    let [top, bottom] = content_area.layout(
        &Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).spacing(1),
    );
    let [top_left, top_right] = top.layout(
        &Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).spacing(1),
    );
    let [bottom_left, bottom_right] = bottom.layout(
        &Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).spacing(1),
    );

    render_overlay_shadow(frame, top_left);
    render_block_shadow(frame, top_right);
    render_symbol_shadow(frame, bottom_left);
    render_dimmed_shadow(frame, bottom_right);
}

fn render_overlay_shadow(frame: &mut Frame, area: Rect) {
    render_background_paragraph(frame, area, Style::new().fg(Color::White).bg(Color::Blue));
    let shadow = Shadow::overlay().style(Style::new().bg(Color::DarkGray));
    let block = Block::bordered()
        .title("Overlay shadow")
        .style(Style::new().black().on_yellow())
        .shadow(shadow);
    render_popup(frame, area, block);
}

fn render_block_shadow(frame: &mut Frame, area: Rect) {
    render_background_paragraph(frame, area, Style::new().fg(Color::White));
    let shadow = Shadow::block()
        .style(Style::new().fg(Color::DarkGray))
        .offset(Offset::new(2, 1));
    let block = Block::bordered()
        .title("Block shadow")
        .style(Style::new().black().on_yellow())
        .shadow(shadow);
    render_popup(frame, area, block);
}

fn render_symbol_shadow(frame: &mut Frame, area: Rect) {
    render_background_paragraph(frame, area, Style::new().fg(Color::White));
    let shadow = Shadow::symbol("$")
        .style(Style::new().fg(Color::DarkGray))
        .offset(Offset::new(2, 1));
    let block = Block::bordered()
        .title("Symbol shadow")
        .style(Style::new().white().on_red())
        .shadow(shadow);
    render_popup(frame, area, block);
}

fn render_dimmed_shadow(frame: &mut Frame, area: Rect) {
    render_background_paragraph(frame, area, Style::new().fg(Color::White).bg(Color::Blue));
    let shadow = Shadow::custom(dimmed())
        .style(Style::new().bg(Color::DarkGray))
        .offset(Offset::new(2, 1));
    let block = Block::bordered()
        .title("Dimmed shadow")
        .style(Style::new().black().on_green())
        .shadow(shadow);
    render_popup(frame, area, block);
}

fn render_background_paragraph(frame: &mut Frame, area: Rect, style: Style) {
    let background = Paragraph::new(background_text(area))
        .block(Block::bordered())
        .wrap(Wrap { trim: true })
        .style(style);
    frame.render_widget(background, area);
}

fn render_popup(frame: &mut Frame, area: Rect, block: Block<'_>) {
    let popup_area = area.centered(
        Constraint::Length(area.width.saturating_sub(18)),
        Constraint::Length(area.height.saturating_sub(8)),
    );
    frame.render_widget(Clear, popup_area);
    frame.render_widget(block, popup_area);
}

fn background_text(area: Rect) -> Text<'static> {
    let sentence = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ";
    let repeated = sentence.repeat(area.height as usize);
    Text::from(Line::from(repeated))
}
