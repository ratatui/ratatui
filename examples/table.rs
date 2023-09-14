use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

struct App<'a> {
    state: TableState,
    items: Vec<Vec<&'a str>>,
    opts: AppOpts,
}

#[derive(Debug, Clone, Default)]
struct AppOpts {
    // highlight_area: HighlightArea,
    space_in_all_cols: bool,
    highlight_spacing: HighlightSpacing,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            state: TableState::default(),
            items: vec![
                vec!["Row11", "Row12", "Row13"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row31", "Row32", "Row33"],
                vec!["Row41", "Row42", "Row43"],
                vec!["Row51", "Row52", "Row53"],
                vec!["Row61", "Row62\nTest", "Row63"],
                vec!["Row71", "Row72", "Row73"],
                vec!["Row81", "Row82", "Row83"],
                vec!["Row91", "Row92", "Row93"],
                vec!["Row101", "Row102", "Row103"],
                vec!["Row111", "Row112", "Row113"],
                vec!["Row121", "Row122", "Row123"],
                vec!["Row131", "Row132", "Row133"],
                vec!["Row141", "Row142", "Row143"],
                vec!["Row151", "Row152", "Row153"],
                vec!["Row161", "Row162", "Row163"],
                vec!["Row171", "Row172", "Row173"],
                vec!["Row172", "Row182", "Row183"],
                vec!["Use Arrows to move selection", "Row142", "Row143"],
                vec!["Press 'Esc' to remove selection", "", ""],
                vec!["Press 'f' to toggle full row highlighting", "", ""],
                vec![
                    "Press 'h' to toggle highlight_spacing Never->Always->WhenSelected",
                    "",
                    "",
                ],
                vec![
                    "Press 'a' to toggle space from selected column to all columns",
                    "",
                    "",
                ],
            ],
            opts: AppOpts::default(),
        }
    }
    pub fn next_row(&mut self) {
        let sel = match self.state.selected() {
            Some(mut new_sel) => {
                if new_sel.row >= self.items.len() - 1 {
                    new_sel.row = 0;
                    new_sel
                } else {
                    new_sel.row += 1;
                    new_sel
                }
            }
            None => TableSelection::default(),
        };
        self.state.select(Some(sel));
    }

    pub fn prev_row(&mut self) {
        let sel = match self.state.selected() {
            Some(mut new_sel) => {
                if new_sel.row == 0 {
                    new_sel.row = self.items.len() - 1;
                    new_sel
                } else {
                    new_sel.row -= 1;
                    new_sel
                }
            }
            None => TableSelection::default(),
        };
        self.state.select(Some(sel));
    }

    pub fn next_col(&mut self) {
        let sel = match self.state.selected() {
            Some(mut new_sel) => {
                if new_sel.col == self.items[new_sel.row].len() - 1 {
                    new_sel.col = 0;
                    new_sel
                } else {
                    new_sel.col += 1;
                    new_sel
                }
            }
            None => TableSelection::default(),
        };
        self.state.select(Some(sel));
    }

    pub fn prev_col(&mut self) {
        let sel = match self.state.selected() {
            Some(mut new_sel) => {
                if new_sel.col == 0 {
                    new_sel.col = self.items[new_sel.row].len() - 1;
                    new_sel
                } else {
                    new_sel.col -= 1;
                    new_sel
                }
            }
            None => TableSelection::default(),
        };
        self.state.select(Some(sel));
    }

    pub fn toggle_highlight_spacing_all_cols(&mut self) {
        self.opts.space_in_all_cols = !self.opts.space_in_all_cols;
    }
    pub fn toggle_highlight_spacing(&mut self) {
        self.opts.highlight_spacing = match self.opts.highlight_spacing {
            HighlightSpacing::Never => HighlightSpacing::Always,
            HighlightSpacing::Always => HighlightSpacing::WhenSelected,
            HighlightSpacing::WhenSelected => HighlightSpacing::Never,
        }
    }

    // pub fn toggle_highlight_area(&mut self) {
    //     self.opts.highlight_area = match self.opts.highlight_area {
    //         HighlightArea::None => HighlightArea::Cell,
    //         HighlightArea::Cell => HighlightArea::Row,
    //         HighlightArea::Row => HighlightArea::Col,
    //         HighlightArea::Col => HighlightArea::RowAndCol,
    //         HighlightArea::RowAndCol => HighlightArea::None,
    //     }
    // }
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
                    // KeyCode::Char('f') => app.toggle_highlight_area(),
                    KeyCode::Char('a') => app.toggle_highlight_spacing_all_cols(),
                    KeyCode::Char('h') => app.toggle_highlight_spacing(),
                    KeyCode::Esc => app.state.select(None),
                    _ => {}
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
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
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        // .highlight_area(app.opts.highlight_area.clone())
        // .highlight_spacing_all_columns(app.opts.space_in_all_cols)
        .highlight_spacing(app.opts.highlight_spacing.clone())
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Max(30),
            Constraint::Min(10),
        ]);
    f.render_stateful_widget(t, rects[0], &mut app.state);
}
