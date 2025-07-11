use ratatui::crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, Cell, Row, Table, TableBorderType, TableState},
    Frame, Terminal,
};
use std::{error::Error, io};

struct App {
    state: TableState,
    border_type: TableBorderType,
}

impl App {
    fn new() -> App {
        App {
            state: TableState::default(),
            border_type: TableBorderType::None,
        }
    }

    fn next_border_type(&mut self) {
        self.border_type = match self.border_type {
            TableBorderType::None => TableBorderType::Horizontal,
            TableBorderType::Horizontal => TableBorderType::Vertical,
            TableBorderType::Vertical => TableBorderType::All,
            TableBorderType::All => TableBorderType::None,
        };
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= 4 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    4
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
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
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Char(' ') => app.next_border_type(),
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .margin(1)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["Name", "Age", "City", "Country", "Occupation"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = [
        ["John", "25", "New York", "USA", "Engineer"],
        ["Jane", "30", "London", "UK", "Designer"],
        ["Bob", "35", "Paris", "France", "Manager"],
        ["Alice", "28", "Tokyo", "Japan", "Developer"],
        ["Charlie", "32", "Sydney", "Australia", "Analyst"],
    ]
    .iter()
    .map(|item| {
        let height = 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height).bottom_margin(1)
    });

    let border_type_str = match app.border_type {
        TableBorderType::None => "None",
        TableBorderType::Horizontal => "Horizontal",
        TableBorderType::Vertical => "Vertical",
        TableBorderType::All => "All",
    };

    let t = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Length(5),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(12),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Table with Internal Borders: {} (Space to cycle, q to quit)", border_type_str))
    )
    .row_highlight_style(selected_style)
    .border_type(app.border_type)
    .border_style(Style::default().fg(Color::Yellow));

    f.render_stateful_widget(t, rects[0], &mut app.state);
}
