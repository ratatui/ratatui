//! # [Ratatui] `Table` borders example
//!
//! The latest version of this example is available in the [widget examples] folder in the
//! repository.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [widget examples]: https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    widgets::{Block, Cell, Row, Table, TableBorders, TableState},
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut state = TableState::default();
    state.select_first();

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|f| render(f, &mut state))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => state.select_next(),
                    KeyCode::Up | KeyCode::Char('k') => state.select_previous(),
                    _ => {}
                }
            }
        }
    })
}

fn render(f: &mut Frame, state: &mut TableState) {
    let area = f.area();
    let chunks =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);

    let items = (0..20)
        .map(|i| {
            vec![
                format!("Row{} Col1", i),
                format!("Row{} Col2", i),
                format!("Row{} Col3", i),
            ]
        })
        .collect::<Vec<_>>();

    let rows = items
        .iter()
        .map(|item| Row::new(item.iter().map(|c| Cell::from(c.as_str()))));

    // Left Table: ALL borders, row_spacing 1, column_spacing 1
    let table1 = Table::new(
        rows.clone(),
        [
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ],
    )
    .header(Row::new(vec!["Col 1", "Col 2", "Col 3"]).bottom_margin(1))
    .block(Block::bordered().title("Table: ALL borders, spacing 1"))
    .borders(TableBorders::ALL)
    .row_spacing(1)
    .column_spacing(1)
    .row_highlight_style(Style::new().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(table1, chunks[0], state);

    // Right Table: VERTICAL borders only, column_spacing 1
    let table2 = Table::new(
        rows.clone(),
        [
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ],
    )
    .header(Row::new(vec!["Col 1", "Col 2", "Col 3"]))
    .block(Block::bordered().title("Table: VERTICAL only, spacing 1"))
    .borders(TableBorders::VERTICAL)
    .column_spacing(1)
    .row_highlight_style(Style::new().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(table2, chunks[1], state);
}
