//! Inspect a virtual table with app-owned cells, pinned headers, and two-axis selection.
//!
//! The important part is that the table owns layout policy, while `Rows` owns cell rendering. The
//! returned `TableLayout` still exposes hit-testable cells and scroll metrics for app chrome.

use std::io;

use color_eyre::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind};
use crossterm::execute;
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Paragraph, Widget};
use ratatui_layout::{
    CellPosition, TableCellContext, TableItems, TableLayout, VirtualTable, VirtualTableState,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    execute!(io::stdout(), EnableMouseCapture)?;
    let result = run();
    execute!(io::stdout(), DisableMouseCapture)?;
    result
}

fn run() -> Result<()> {
    let mut app = App::default();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| app.render(frame))?;
            match event::read()? {
                event::Event::Key(key) if key.is_press() && !app.handle_key(key.code) => {
                    break Ok(());
                }
                event::Event::Mouse(mouse) => app.handle_mouse(mouse),
                _ => {}
            }
        }
    })
}

const HEADERS: [&str; 5] = ["id", "name", "state", "owner", "age"];

#[derive(Default)]
struct App {
    state: VirtualTableState,
    previous_layout: Option<TableLayout>,
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        self.ensure_selection();
        let table_area = Rect::new(
            0,
            0,
            frame.area().width,
            frame.area().height.saturating_sub(1),
        );
        let mut rows = Rows;
        let layout = VirtualTable::new([
            Constraint::Length(5),
            Constraint::Length(18),
            Constraint::Length(12),
            Constraint::Length(14),
            Constraint::Length(8),
        ])
        .scroll_padding(1)
        .render(table_area, frame.buffer_mut(), &mut self.state, &mut rows);
        self.previous_layout = Some(layout.clone());

        let status = format!(
            "row {} of {}  col {} of {}  click select  wheel scrolls rows only",
            layout.vertical_metrics().offset + 1,
            layout.row_count,
            layout.horizontal_metrics().offset + 1,
            layout.column_count,
        );
        let status_area = Rect::new(0, table_area.bottom(), frame.area().width, 1);
        frame.render_widget(Paragraph::new(status), status_area);
    }

    const fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('j') | KeyCode::Down => self.move_selection(1, 0),
            KeyCode::Char('k') | KeyCode::Up => self.move_selection(-1, 0),
            KeyCode::Char('l') | KeyCode::Right => self.move_selection(0, 1),
            KeyCode::Char('h') | KeyCode::Left => self.move_selection(0, -1),
            KeyCode::Char('q') | KeyCode::Esc => return Self::quit(),
            _ => {}
        }
        true
    }

    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        match mouse.kind {
            MouseEventKind::ScrollDown => self.state.scroll_rows_by(1),
            MouseEventKind::ScrollUp => self.state.scroll_rows_by(-1),
            MouseEventKind::Down(_) => self.select_at((mouse.column, mouse.row)),
            _ => {}
        }
    }

    const fn ensure_selection(&mut self) {
        if self.state.selected().is_none() {
            self.state.select(Some(CellPosition::body(0, 0)));
        }
    }

    const fn move_selection(&mut self, row_delta: isize, column_delta: isize) {
        self.state
            .select_relative(row_delta, column_delta, Rows::ROW_COUNT, HEADERS.len());
    }

    fn select_at(&mut self, position: (u16, u16)) {
        if let Some(layout) = self.previous_layout.as_ref() {
            layout.select_hit(position, &mut self.state);
        }
    }

    const fn quit() -> bool {
        false
    }
}

struct Rows;

impl Rows {
    const ROW_COUNT: usize = 40;

    fn cell(position: CellPosition) -> String {
        match position.row {
            None => HEADERS[position.column].to_string(),
            Some(row) => match position.column {
                0 => format!("#{row:03}"),
                1 => format!("task-{row:02}"),
                2 => ["queued", "running", "done", "blocked"][row % 4].to_string(),
                3 => ["ops", "ui", "core", "docs"][row % 4].to_string(),
                4 => format!("{}m", 3 + row * 2),
                _ => String::new(),
            },
        }
    }
}

impl TableItems for Rows {
    fn row_count(&self) -> usize {
        Self::ROW_COUNT
    }

    fn column_count(&self) -> usize {
        HEADERS.len()
    }

    fn render_cell(
        &mut self,
        position: CellPosition,
        area: Rect,
        buf: &mut Buffer,
        ctx: TableCellContext,
    ) {
        let header = position.row.is_none();
        let selected = ctx.render.state.selected;
        let style = match (header, selected) {
            (true, _) => Style::new().add_modifier(Modifier::BOLD),
            (false, true) => Style::new().add_modifier(Modifier::REVERSED),
            (false, false) => Style::new(),
        };
        Paragraph::new(Self::cell(position))
            .style(style)
            .render(area, buf);
    }
}
