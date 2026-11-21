/// A Ratatui example that demonstrates how to use modifiers.
///
/// Each row shows a modifier, each column uses a different foreground color, and tabs select
/// the background color.
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
use std::iter::{once, repeat_n};

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Cell, Paragraph, Row, Table, Tabs};

const COLORS: [(&str, Color); 5] = [
    ("Black", Color::Black),
    ("Dark gray", Color::DarkGray),
    ("Gray", Color::Gray),
    ("White", Color::White),
    ("Red", Color::Red),
];

const MODIFIERS: [(&str, Modifier); 14] = [
    ("None", Modifier::empty()),
    ("Bold", Modifier::BOLD),
    ("Dim", Modifier::DIM),
    ("Italic", Modifier::ITALIC),
    ("Single underline", Modifier::UNDERLINED),
    ("Double underline", Modifier::DOUBLE_UNDERLINED),
    ("Curly underline", Modifier::UNDER_CURLED),
    ("Dotted underline", Modifier::UNDER_DOTTED),
    ("Dashed underline", Modifier::UNDER_DASHED),
    ("Slow blink", Modifier::SLOW_BLINK),
    ("Rapid blink", Modifier::RAPID_BLINK),
    ("Reversed", Modifier::REVERSED),
    ("Hidden", Modifier::HIDDEN),
    ("Strikethrough", Modifier::CROSSED_OUT),
];

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut background = 0;
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, background))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                    KeyCode::Right | KeyCode::Tab => background = (background + 1) % COLORS.len(),
                    KeyCode::Left | KeyCode::BackTab => {
                        background = (background + COLORS.len() - 1) % COLORS.len();
                    }
                    _ => {}
                }
            }
        }
    })
}

fn render(frame: &mut Frame, background: usize) {
    let [area] = frame
        .area()
        .layout(&Layout::horizontal([Constraint::Max(76)]));
    let [tabs_area, table_area, help_area] = area.layout(&Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(2),
    ]));

    frame.render_widget(
        Tabs::new(COLORS.map(|(name, _)| name))
            .select(background)
            .highlight_style(Style::new().black().on_yellow().bold())
            .block(Block::bordered().title(" Background ")),
        tabs_area,
    );

    let header = Row::new(once("Modifier").chain(COLORS.map(|(name, _)| name)))
        .bold()
        .bottom_margin(1);
    let rows = MODIFIERS.map(|(name, modifier)| {
        let samples = COLORS.map(|(_, foreground)| {
            Cell::from("Sample").style(
                Style::new()
                    .fg(foreground)
                    .bg(COLORS[background].1)
                    .add_modifier(modifier),
            )
        });
        Row::new(once(Cell::from(name)).chain(samples))
    });
    let widths = once(Constraint::Length(16)).chain(repeat_n(Constraint::Fill(1), COLORS.len()));
    frame.render_widget(
        Table::new(rows, widths)
            .header(header)
            .column_spacing(2)
            .block(Block::bordered().title(" Foreground colors ")),
        table_area,
    );
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("←/→ or Tab: background   q/Esc: quit"),
            Line::from("Some effects depend on your terminal; hidden samples are blank.")
                .dark_gray(),
        ]),
        help_area,
    );
}
