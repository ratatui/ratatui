//! # [Ratatui] [`Block`] with collapsed quadrant borders example
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
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect, Spacing};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::symbols::merge::MergeStrategy;
use ratatui::text::{Line, Text};
use ratatui::widgets::Block;

const MERGE_STRATEGIES: [MergeStrategy; 3] = [
    MergeStrategy::Exact,
    MergeStrategy::Fuzzy,
    MergeStrategy::Replace,
];

/// Border sets made of block element quadrants, which collapse when merged.
const BORDER_SETS: [(&str, border::Set); 4] = [
    ("QUADRANT_INSIDE", border::QUADRANT_INSIDE),
    ("QUADRANT_OUTSIDE", border::QUADRANT_OUTSIDE),
    ("PROPORTIONAL_WIDE", border::PROPORTIONAL_WIDE),
    ("PROPORTIONAL_TALL", border::PROPORTIONAL_TALL),
];

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| {
        let mut strategy_index = 0;
        let mut border_index = 0;
        loop {
            terminal.draw(|frame| render(frame, strategy_index, border_index))?;
            if let Some(key_event) = event::read()?.as_key_press_event() {
                match key_event.code {
                    KeyCode::Char('m') => {
                        strategy_index = (strategy_index + 1) % MERGE_STRATEGIES.len();
                    }
                    KeyCode::Char('b') => border_index = (border_index + 1) % BORDER_SETS.len(),
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }
        }
    })
}

fn render(frame: &mut Frame, strategy_index: usize, border_index: usize) {
    let [header, grid] = frame.area().layout(&Layout::vertical([
        Constraint::Length(2),
        Constraint::Fill(1),
    ]));
    let strategy = MERGE_STRATEGIES[strategy_index];
    let (border_name, border_set) = BORDER_SETS[border_index];

    let header_text = Text::from(vec![
        Line::from_iter([
            "Collapsed Quadrant Borders".bold(),
            " (m: strategy, b: border, q: quit)".into(),
        ]),
        Line::from(format!("{strategy:?} merge of {border_name} borders")),
    ]);
    frame.render_widget(header_text.centered(), header);

    // Overlapping the blocks by one cell makes their borders share cells, which the merge
    // strategy then combines. With `MergeStrategy::Replace`, the last block drawn wins instead.
    let rows: [Rect; 2] =
        grid.layout(&Layout::vertical([Constraint::Fill(1); 2]).spacing(Spacing::Overlap(1)));
    for row in rows {
        let cells: [Rect; 3] =
            row.layout(&Layout::horizontal([Constraint::Fill(1); 3]).spacing(Spacing::Overlap(1)));
        for cell in cells {
            let block = Block::bordered()
                .border_set(border_set)
                .merge_borders(strategy);
            frame.render_widget(block, cell);
        }
    }
}
