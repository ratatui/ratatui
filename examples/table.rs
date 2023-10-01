use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

struct App {
    state: TableState,
    items: Vec<Vec<String>>,
    opts: AppOpts,
}

#[derive(Debug, Clone, Default)]
struct AppOpts {
    highlight_area: HighlightArea,
    columns_with_spacing: ColumnHighlightSpacing,
    highlight_spacing: HighlightSpacing,
}

impl App {
    fn new() -> App {
        let items_str: Vec<Vec<&str>> = vec![
            vec!["Row11", "Row12", "Row13"],
            vec!["Row21", "Row22", "Row23"],
            vec!["Row31", "Row32", "Row33"],
            vec!["Row41", "Row42", "Row43"],
            vec!["Row51", "Row52", "Row53"],
            vec!["Row61", "Row62\nTest", "Row63"],
            vec!["Row71", "Row72", "Row73"],
            vec!["Row81", "Row82", "Row83"],
            vec!["Row91", "Row92", "Row93"],
            vec!["Use Arrows to move selection", "Row142", "Row143"],
            vec!["Press 'Esc' to remove selection", "", ""],
            vec![
                "Press 's' to toggle Cell/Row/Col/RowAndCol highlighting",
                "Current -->",
                "Row",
            ],
            vec![
                "Press 'h' to toggle highlight_spacing Never->Always->WhenSelected",
                "Current -->",
                "WhenSelected",
            ],
            vec![
                "Press 'a' to toggle space from selected column to all columns",
                "Current -->",
                "FirstColumnOnly",
            ],
        ];
        let mut items: Vec<Vec<String>> = vec![];
        for row in items_str {
            let mut tmp_vec: Vec<String> = vec![];
            for item in row {
                tmp_vec.push(item.to_string());
            }
            items.push(tmp_vec);
        }
        App {
            state: TableState::default(),
            items,

            opts: AppOpts::default(),
        }
    }
    pub fn next_row(&mut self) {
        let next_row = match self.state.selected_row() {
            Some(row) => {
                if row == self.items.len() - 1 {
                    0
                } else {
                    row + 1
                }
            }

            None => 0,
        };
        self.state.select_row(Some(next_row));
    }

    pub fn prev_row(&mut self) {
        let prev_row = match self.state.selected_row() {
            Some(row) => {
                if row == 0 {
                    self.items.len() - 1
                } else {
                    row - 1
                }
            }
            None => 0,
        };

        self.state.select_row(Some(prev_row));
    }

    pub fn next_col(&mut self) {
        let next_col = match self.state.selected_col() {
            // Arguably you should count the number of columns for the case of rows with different
            // columns than the first row, but for this example we'll just assume they're the same.
            Some(col) => {
                if col == self.items[0].len() - 1 {
                    0
                } else {
                    col + 1
                }
            }
            None => 0,
        };
        self.state.select_col(Some(next_col));
    }

    pub fn prev_col(&mut self) {
        let prev_col = match self.state.selected_col() {
            Some(col) => {
                if col == 0 {
                    self.items[0].len() - 1
                } else {
                    col - 1
                }
            }
            None => 0,
        };
        self.state.select_col(Some(prev_col));
    }

    pub fn toggle_columns_with_spacing(&mut self) {
        self.items[13][2] = format!("{:?}", self.opts.columns_with_spacing);
        self.opts.columns_with_spacing = match self.opts.columns_with_spacing {
            ColumnHighlightSpacing::FirstColumnOnly => ColumnHighlightSpacing::SelectedColumn,
            ColumnHighlightSpacing::SelectedColumn => ColumnHighlightSpacing::AllColumns,
            ColumnHighlightSpacing::AllColumns => {
                ColumnHighlightSpacing::SpecificColumns(vec![0, 2])
            }
            ColumnHighlightSpacing::SpecificColumns(_) => ColumnHighlightSpacing::FirstColumnOnly,
        };
    }

    pub fn toggle_highlight_spacing(&mut self) {
        self.items[12][2] = format!("{:?}", self.opts.highlight_spacing);
        self.opts.highlight_spacing = match self.opts.highlight_spacing {
            HighlightSpacing::Never => HighlightSpacing::Always,
            HighlightSpacing::Always => HighlightSpacing::WhenSelected,
            HighlightSpacing::WhenSelected => HighlightSpacing::Never,
        }
    }

    pub fn toggle_selection_type(&mut self) {
        self.items[11][2] = format!("{:?}", self.opts.highlight_area);
        self.opts.highlight_area = match self.opts.highlight_area {
            HighlightArea::None => HighlightArea::Cell,
            HighlightArea::Cell => HighlightArea::Row,
            HighlightArea::Row => HighlightArea::Col,
            HighlightArea::Col => HighlightArea::RowAndCol,
            HighlightArea::RowAndCol => HighlightArea::None,
        }
    }

    fn test(&mut self) {
        self.state.select(Some(TableSelection::Row(1)));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.next_row(),
                    KeyCode::Up => app.prev_row(),
                    KeyCode::Right => app.next_col(),
                    KeyCode::Left => app.prev_col(),
                    KeyCode::Char('s') => app.toggle_selection_type(),
                    KeyCode::Char('a') => app.toggle_columns_with_spacing(),
                    KeyCode::Char('h') => app.toggle_highlight_spacing(),
                    KeyCode::Char('t') => app.test(),
                    KeyCode::Esc => app.state.select(None::<TableSelection>),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["Header1", "Header2", "Header3"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(c.clone()));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .highlight_area(app.opts.highlight_area.clone())
        .columns_with_highlight_spacing(app.opts.columns_with_spacing.clone())
        .highlight_spacing(app.opts.highlight_spacing.clone())
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Max(30),
            Constraint::Min(24),
        ]);
    f.render_stateful_widget(t, rects[0], &mut app.state);
}
