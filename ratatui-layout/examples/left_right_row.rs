//! Split each row into left and right regions without hand-calculating x positions.
//!
//! A normal `Layout::horizontal(...).areas(area)` call works for one row. This example uses `Row`
//! because every row repeats the same split and the returned plan makes the two render targets
//! explicit before rendering.

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui_layout::Row;

fn main() -> Result<()> {
    color_eyre::install()?;

    ratatui::run(|terminal| {
        loop {
            terminal.draw(render)?;
            if let Some(key) = event::read()?.as_key_press_event()
                && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
            {
                break Ok(());
            }
        }
    })
}

fn render(frame: &mut Frame) {
    let rows = [
        ("Inbox", "14 unread"),
        ("Builds", "2 running"),
        ("Deploys", "healthy"),
        ("Backups", "03:00"),
    ];

    for (index, (name, status)) in rows.iter().enumerate() {
        let area = Rect::new(0, index as u16, frame.area().width, 1);
        render_status_row(frame, area, name, status, index == 0);
    }
}

fn render_status_row(frame: &mut Frame, area: Rect, name: &str, status: &str, selected: bool) {
    // The row planner is the important part: the caller describes the two regions, receives a
    // plan, and then renders ordinary Ratatui widgets into the assigned rectangles.
    let row_columns = [
        Constraint::Min(0),
        Constraint::Length(status.chars().count() as u16),
    ];
    let plan = Row::new(row_columns).regions(area);
    let name_area = plan.regions()[0].area;
    let status_area = plan.regions()[1].area;
    let style = if selected {
        Style::new().add_modifier(Modifier::REVERSED)
    } else {
        Style::new()
    };

    frame.render_widget(Paragraph::new(name).style(style), name_area);
    frame.render_widget(
        Paragraph::new(status).right_aligned().style(style),
        status_area,
    );
}
