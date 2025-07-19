//! This is an example used for heading image in `README.md` and crate documentation.
//!
//! It is updated for every release, starting with Ratatui 0.30.0.

use ratatui::layout::{Offset, Spacing};
use ratatui::prelude::*;
use ratatui::symbols::merge::MergeStrategy;
use ratatui::widgets::{Block, BorderType, Padding, Paragraph, RatatuiLogo};
use ratatui::{TerminalOptions, Viewport};

const VERSION: &str = "v0.30.0";

const BG_COLOR: Color = Color::Rgb(20, 20, 50);
const MENU_BORDER_COLOR: Color = Color::Rgb(255, 255, 160);

const GRADIENT_RED: [u8; 5] = [70, 80, 110, 170, 255];
const GRADIENT_GREEN: [u8; 5] = [40, 60, 100, 160, 255];
const GRADIENT_BLUE: [u8; 5] = [70, 80, 110, 170, 255];
const GRADIENT_AMBIENT: [u8; 5] = [20, 30, 40, 50, 60];

enum RainbowColor {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
}

const ROYGBIV: [RainbowColor; 7] = [
    RainbowColor::Red,
    RainbowColor::Orange,
    RainbowColor::Yellow,
    RainbowColor::Green,
    RainbowColor::Blue,
    RainbowColor::Indigo,
    RainbowColor::Violet,
];

const fn get_gradient_color(color: &RainbowColor, index: usize) -> Color {
    let (r, g, b) = match color {
        RainbowColor::Red => (
            GRADIENT_RED[index],
            GRADIENT_AMBIENT[index],
            GRADIENT_AMBIENT[index],
        ),
        RainbowColor::Orange => (
            GRADIENT_RED[index],
            GRADIENT_GREEN[index] / 2,
            GRADIENT_AMBIENT[index],
        ),
        RainbowColor::Yellow => (
            GRADIENT_RED[index],
            GRADIENT_GREEN[index],
            GRADIENT_AMBIENT[index],
        ),
        RainbowColor::Green => (
            GRADIENT_AMBIENT[index],
            GRADIENT_GREEN[index],
            GRADIENT_AMBIENT[index],
        ),
        RainbowColor::Blue => (
            GRADIENT_AMBIENT[index],
            GRADIENT_AMBIENT[index],
            GRADIENT_BLUE[index],
        ),
        RainbowColor::Indigo => (
            GRADIENT_BLUE[index],
            GRADIENT_AMBIENT[index],
            GRADIENT_BLUE[index],
        ),
        RainbowColor::Violet => (
            GRADIENT_RED[index],
            GRADIENT_AMBIENT[index],
            GRADIENT_BLUE[index],
        ),
    };
    Color::Rgb(r, g, b)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let viewport = Viewport::Inline(10);
    let mut terminal = ratatui::init_with_options(TerminalOptions { viewport });
    loop {
        terminal.draw(render_header)?;
        if ratatui::crossterm::event::read()?.is_key_press() {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}

fn render_header(frame: &mut Frame) {
    let area = frame.area();
    frame
        .buffer_mut()
        .set_style(area, Style::new().bg(BG_COLOR));

    let horizontal_layout = Layout::horizontal([Constraint::Length(31), Constraint::Length(23)])
        .spacing(Spacing::Overlap(1));
    let [left_area, right_area] = area.layout(&horizontal_layout);

    let logo_block = Block::new().padding(Padding::uniform(1));

    let left_area_inner = logo_block.inner(left_area);
    let left_vertical_layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]);
    let [logo_area, ratatui_version_area] = left_area_inner.layout(&left_vertical_layout);

    // render rainbow shadow
    // divide RatatuiLogo into letters so we can color them individually
    let letters_layout = Layout::horizontal([
        Constraint::Length(5),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(5),
        Constraint::Length(1),
    ]);
    let letters: [Rect; 7] = logo_area.layout(&letters_layout);
    for idx in 0..5 {
        let offset = Offset::new(0, idx);

        for (rect, color) in letters.iter().zip(ROYGBIV) {
            let gradient_color = get_gradient_color(&color, idx as usize);
            frame.render_widget(
                Block::new().style(Style::new().fg(gradient_color).dim()),
                rect.offset(offset),
            );
        }
        frame.render_widget(RatatuiLogo::small(), logo_area.offset(offset));
    }

    // render the final logo
    frame.render_widget(
        Block::new().style(Color::Reset).not_dim(),
        logo_area
            .offset(Offset::new(0, 5))
            .intersection(left_area_inner),
    );
    frame.render_widget(
        RatatuiLogo::small(),
        logo_area.offset(Offset::new(0, 5)).intersection(logo_area),
    );
    frame.render_widget(VERSION, ratatui_version_area);

    let menu_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(MENU_BORDER_COLOR)
        .padding(Padding::horizontal(1))
        .merge_borders(MergeStrategy::Fuzzy);

    let menu_layout =
        Layout::vertical([Constraint::Length(6), Constraint::Fill(1)]).spacing(Spacing::Overlap(1));
    let [main_dishes_area, backends_area] = right_area.layout(&menu_layout);
    frame.render_widget(menu_block.clone().title("Main dishes"), main_dishes_area);
    frame.render_widget(menu_block.clone().title("Backends"), backends_area);

    let inner_main_dishes = menu_block.inner(main_dishes_area);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("> ratatui"),
            Line::from("> ratatui-core"),
            Line::from("> ratatui-widgets"),
            Line::from("> ratatui-macros"),
        ]),
        inner_main_dishes,
    );

    let inner_backends = menu_block.inner(backends_area);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("> ratatui-crossterm"),
            Line::from("> ratatui-termion"),
            Line::from("> ratatui-termwiz"),
        ]),
        inner_backends,
    );
}
