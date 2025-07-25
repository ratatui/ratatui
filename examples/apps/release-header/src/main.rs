//! Generates a terminal banner for Ratatui releases featuring a Ratatui logo, version info, and
//! a list of crates.
//!
//! Used for README.md, documentation, and release materials. Updated for every release starting
//! with v0.30.0 "Bryndza".
//!
//! This example runs with the Ratatui library code in the branch that you are currently
//! reading. See the [`latest`] branch for the code which works with the most recent Ratatui
//! release.
//!
//! [`latest`]: https://github.com/ratatui/ratatui/tree/latest

use std::io::stdout;

use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::layout::{Offset, Spacing};
use ratatui::prelude::*;
use ratatui::symbols::merge::MergeStrategy;
use ratatui::widgets::{Block, BorderType, Padding, Paragraph, RatatuiLogo};
use ratatui::{DefaultTerminal, TerminalOptions, Viewport};

const SEMVER: &str = "0.30.0";
const RELEASE_NAME: &str = "Bryndza";

const MAIN_DISHES: [&str; 4] = [
    "> ratatui",
    "> ratatui-core",
    "> ratatui-widgets",
    "> ratatui-macros",
];
const BACKENDS: [&str; 3] = [
    "> ratatui-crossterm",
    "> ratatui-termion",
    "> ratatui-termwiz",
];

const FG_COLOR: Color = Color::Rgb(246, 214, 187); // #F6D6BB
const BG_COLOR: Color = Color::Rgb(20, 20, 50); // #141432
const MENU_BORDER_COLOR: Color = Color::Rgb(255, 255, 160); // #FFFFA0

const GRADIENT_RED: [u8; 6] = [41, 43, 50, 68, 104, 156];
const GRADIENT_GREEN: [u8; 6] = [24, 30, 41, 65, 105, 168];
const GRADIENT_BLUE: [u8; 6] = [55, 57, 62, 78, 113, 166];
const GRADIENT_AMBIENT: [u8; 6] = [17, 18, 20, 25, 40, 60];

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

fn get_gradient_color(color: &RainbowColor, index: usize) -> Color {
    let blue_ambience = GRADIENT_AMBIENT[index].saturating_mul(6 - index as u8);
    let (r, g, b) = match color {
        RainbowColor::Red => (GRADIENT_RED[index], GRADIENT_AMBIENT[index], blue_ambience),
        RainbowColor::Orange => (
            GRADIENT_RED[index],
            GRADIENT_GREEN[index] / 2,
            blue_ambience,
        ),
        RainbowColor::Yellow => (GRADIENT_RED[index], GRADIENT_GREEN[index], blue_ambience),
        RainbowColor::Green => (
            GRADIENT_AMBIENT[index],
            GRADIENT_GREEN[index],
            blue_ambience,
        ),
        RainbowColor::Blue => (
            GRADIENT_AMBIENT[index],
            GRADIENT_AMBIENT[index],
            GRADIENT_BLUE[index].max(blue_ambience),
        ),
        RainbowColor::Indigo => (
            GRADIENT_BLUE[index],
            GRADIENT_AMBIENT[index],
            GRADIENT_BLUE[index].max(blue_ambience),
        ),
        RainbowColor::Violet => (
            GRADIENT_RED[index],
            GRADIENT_AMBIENT[index],
            GRADIENT_BLUE[index].max(blue_ambience),
        ),
    };
    Color::Rgb(r, g, b)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let viewport = Viewport::Fixed(Rect::new(0, 0, 68, 16));
    let terminal = ratatui::init_with_options(TerminalOptions { viewport });
    execute!(stdout(), EnterAlternateScreen).expect("failed to enter alternate screen");
    let result = run(terminal);
    execute!(stdout(), LeaveAlternateScreen).expect("failed to leave alternate screen");
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(render_header)?;
        if ratatui::crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render_header(frame: &mut Frame) {
    let area = frame.area();
    frame
        .buffer_mut()
        .set_style(area, Style::new().fg(FG_COLOR).bg(BG_COLOR));

    let horizontal_layout = Layout::horizontal([Constraint::Length(31), Constraint::Length(23)])
        .spacing(Spacing::Overlap(1));
    let [left_area, right_area] = area
        .centered(
            Constraint::Length(53),
            Constraint::Length(MAIN_DISHES.len() as u16 + BACKENDS.len() as u16 + 3),
        )
        .layout(&horizontal_layout);

    render_logo(frame, left_area);
    render_menu(frame, right_area);
}

fn render_logo(frame: &mut Frame, area: Rect) {
    let logo_block = Block::new().padding(Padding::new(1, 1, 1, 0));
    let logo_inner_area = logo_block.inner(area);
    let vertical_layout = Layout::vertical([
        Constraint::Length(2),
        Constraint::Fill(1),
        Constraint::Length(1),
    ]);
    let [logo_area, _, version_area] = logo_inner_area.layout(&vertical_layout);

    render_logo_rainbow_shadow(frame, logo_area);

    let logo_offset = Offset::new(0, 6);
    frame.render_widget(Block::new().style(FG_COLOR), logo_area.offset(logo_offset));
    frame.render_widget(RatatuiLogo::small(), logo_area.offset(logo_offset));

    frame.render_widget(
        Line::from(format!("v{SEMVER} \"{RELEASE_NAME}\"")).dim(),
        version_area,
    );
}

fn render_logo_rainbow_shadow(frame: &mut Frame, area: Rect) {
    // Divide the logo into letter sections for individual coloring
    let letter_constraints = [
        Constraint::Length(5),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(5),
        Constraint::Length(1),
    ];
    let letter_layout = Layout::horizontal(letter_constraints);
    let letter_areas: [Rect; 7] = area.layout(&letter_layout);

    // Render multiple shadow layers
    for layer_idx in 0..6 {
        let shadow_offset = Offset::new(0, layer_idx as i32);

        // Apply rainbow colors to each letter
        for (letter_area, rainbow_color) in letter_areas.iter().zip(ROYGBIV) {
            let gradient_color = get_gradient_color(&rainbow_color, layer_idx);
            frame.render_widget(
                Block::new().style(gradient_color),
                letter_area.offset(shadow_offset),
            );
        }
        frame.render_widget(RatatuiLogo::small(), area.offset(shadow_offset));
    }
}

fn render_menu(frame: &mut Frame, area: Rect) {
    let vertical_layout = Layout::vertical([
        Constraint::Length(MAIN_DISHES.len() as u16 + 2),
        Constraint::Fill(BACKENDS.len() as u16 + 2),
    ])
    .spacing(Spacing::Overlap(1));
    let [main_dishes_area, backends_area] = area.layout(&vertical_layout);

    render_menu_block(frame, main_dishes_area, "Main Courses", &MAIN_DISHES);
    render_menu_block(frame, backends_area, "Pairings", &BACKENDS);
}

fn render_menu_block(frame: &mut Frame, area: Rect, title: &str, menu_items: &[&str]) {
    let menu_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(MENU_BORDER_COLOR)
        .padding(Padding::horizontal(1))
        .merge_borders(MergeStrategy::Fuzzy)
        .title(title);

    let menu_lines: Vec<Line> = menu_items.iter().map(|&item| Line::from(item)).collect();
    frame.render_widget(Paragraph::new(menu_lines).block(menu_block), area);
}
