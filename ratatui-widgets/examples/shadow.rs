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

    render_block(
        frame,
        top_left,
        "Shadow with overlay",
        Style::new().fg(Color::White).bg(Color::Blue),
        Style::new().black().on_yellow(),
        Shadow::overlay().style(Style::new().bg(Color::DarkGray)),
    );
    render_block(
        frame,
        top_right,
        "Shadow with block symbols",
        Style::new().fg(Color::White),
        Style::new().black().on_yellow(),
        Shadow::block()
            .style(Style::new().fg(Color::DarkGray))
            .offset(Offset::new(2, 1)),
    );
    render_block(
        frame,
        bottom_left,
        "Custom shadow symbol",
        Style::new().fg(Color::White),
        Style::new().white().on_red(),
        Shadow::symbol("$")
            .style(Style::new().fg(Color::DarkGray))
            .offset(Offset::new(2, 1)),
    );
    render_block(
        frame,
        bottom_right,
        "Dimmed shadow",
        Style::new().fg(Color::White).bg(Color::Blue),
        Style::new().black().on_green(),
        Shadow::new(dimmed())
            .style(Style::new().bg(Color::DarkGray))
            .offset(Offset::new(2, 1)),
    );
}

fn render_block(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    background_style: Style,
    style: Style,
    shadow: Shadow,
) {
    let background = Paragraph::new(background_text(area))
        .block(Block::bordered())
        .wrap(Wrap { trim: true })
        .style(background_style);
    frame.render_widget(background, area);

    let popup_area = area.centered(
        Constraint::Length(area.width.saturating_sub(18)),
        Constraint::Length(area.height.saturating_sub(8)),
    );

    let block = Block::bordered().title(title).style(style).shadow(shadow);
    frame.render_widget(Clear, popup_area);
    frame.render_widget(block, popup_area);
}

fn background_text(area: Rect) -> Text<'static> {
    let sentence = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ";
    let repeated = sentence.repeat(area.height as usize);
    Text::from(Line::from(repeated))
}
