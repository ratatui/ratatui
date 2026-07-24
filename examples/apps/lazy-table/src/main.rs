/// A Ratatui example that demonstrates a table with ten million rows.
///
/// [`Table::lazy_rows`] builds rows on demand, so only the rows that are actually on screen
/// are ever created. The status bar shows how many rows
/// the last frame built and how long it took to draw, which stays flat however far into the
/// dataset you scroll. Press `v` to switch between uniform and variable row heights.
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::palette::tailwind;
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, HighlightSpacing, Row, Table, TableState};
use ratatui::{DefaultTerminal, Frame};

/// Ten million rows, none of which exist until they are drawn.
const ROW_COUNT: usize = 10_000_000;

/// Every twenty-fifth row is a taller, three line row.
const TALL_ROW_INTERVAL: usize = 25;

const WIDTHS: [Constraint; 4] = [
    Constraint::Length(10),
    Constraint::Length(18),
    Constraint::Length(12),
    Constraint::Fill(1),
];

const INSTRUCTIONS: &str =
    "(q) quit  (↑/↓) move  (PgUp/PgDn) page  (Home/End) jump  (v) toggle row heights";

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::new().run(terminal))
}

struct App {
    state: TableState,
    /// Whether rows have varying heights, or all share the same height.
    variable_heights: bool,
    /// How many rows the row factory built, reset before every frame.
    rows_built: Arc<AtomicUsize>,
    /// How long the last frame took to draw, and how many rows it built.
    last_frame: (Duration, usize),
    exit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            state: TableState::new().with_selected(0),
            variable_heights: true,
            rows_built: Arc::new(AtomicUsize::new(0)),
            last_frame: (Duration::ZERO, 0),
            exit: false,
        }
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            self.rows_built.store(0, Ordering::Relaxed);
            let started = Instant::now();
            terminal.draw(|frame| self.render(frame))?;
            self.last_frame = (started.elapsed(), self.rows_built.load(Ordering::Relaxed));

            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
                KeyCode::Char('j') | KeyCode::Down => self.state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.state.select_previous(),
                KeyCode::PageDown => self.state.scroll_down_by(20),
                KeyCode::PageUp => self.state.scroll_up_by(20),
                KeyCode::Home => self.state.select_first(),
                // `select_last` selects `usize::MAX`, which the table clamps to the last row.
                KeyCode::End => self.state.select_last(),
                KeyCode::Char('v') => self.variable_heights = !self.variable_heights,
                _ => {}
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let [header_area, table_area, footer_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        frame.render_widget(
            Line::from(format!(" {ROW_COUNT} rows, built on demand ")).bold(),
            header_area,
        );
        frame.render_stateful_widget(self.table(), table_area, &mut self.state);
        frame.render_widget(self.status_line(), footer_area);
    }

    /// Build the table for this frame.
    ///
    /// This is cheap no matter how large `ROW_COUNT` is: the table only stores the row count and
    /// the closures, and calls the row factory while it draws.
    fn table(&self) -> Table<'static> {
        let rows_built = Arc::clone(&self.rows_built);
        let row = move |index: usize| {
            rows_built.fetch_add(1, Ordering::Relaxed);
            build_row(index)
        };

        let table = if self.variable_heights {
            Table::lazy_rows(ROW_COUNT, WIDTHS, row).row_height_with(row_height)
        } else {
            Table::lazy_rows(ROW_COUNT, WIDTHS, row)
        };

        table
            .header(
                Row::new(["Id", "Name", "Value", "Notes"])
                    .style(Style::new().bold().fg(tailwind::SLATE.c200)),
            )
            .block(Block::bordered().title(" Lazy table "))
            .row_highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(" ▶ ")
            .highlight_spacing(HighlightSpacing::Always)
    }

    fn status_line(&self) -> Line<'static> {
        let (elapsed, rows_built) = self.last_frame;
        let selected = self.state.selected().unwrap_or(0);
        let heights = if self.variable_heights {
            "variable"
        } else {
            "uniform"
        };
        Line::from(vec![
            format!(
                " row {selected} | {heights} heights | {rows_built} rows built in {:.2}ms ",
                elapsed.as_secs_f64() * 1000.0
            )
            .into(),
            INSTRUCTIONS.dim(),
        ])
    }
}

/// The height of a row, which scrolling needs to know without building the row itself.
const fn row_height(index: usize) -> u16 {
    if index.is_multiple_of(TALL_ROW_INTERVAL) {
        3
    } else {
        1
    }
}

/// Build the row at `index`, deriving its content from the index alone.
///
/// A real application would look the row up in a database, a log file, or an in-memory dataset;
/// the point is that this only ever runs for the rows that are on screen.
fn build_row(index: usize) -> Row<'static> {
    const NAMES: [&str; 8] = [
        "Alpha", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel",
    ];

    let name = format!("{} {index}", NAMES[index % NAMES.len()]);
    let value = format!("{:.2}", (index % 10_000) as f64 / 100.0);

    if row_height(index) > 1 {
        // A taller row: its extra lines come from a multi-line cell.
        let notes = Text::from(vec![
            Line::from("section marker").bold(),
            Line::from(format!("rows {index}..{}", index + TALL_ROW_INTERVAL)),
            Line::from("─".repeat(20)).dim(),
        ]);
        Row::new([
            Text::from(index.to_string()),
            Text::from(name),
            Text::from(value),
            notes,
        ])
        .style(Style::new().fg(tailwind::EMERALD.c400))
    } else {
        Row::new([
            index.to_string(),
            name,
            value,
            format!("generated on demand for row {index}"),
        ])
    }
}
