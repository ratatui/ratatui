use std::{error::Error, io, str::FromStr};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};

struct App {
    state: TableState,
    items: Vec<Vec<String>>,
}

impl App {
    fn new() -> App {
        App {
            state: TableState::default().with_selected(0),
            items: generate_fake_names(),
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn generate_fake_names() -> Vec<Vec<String>> {
    use fakeit::{address, contact, name};

    (0..20)
        .map(|_| {
            let name = name::full();
            let address = format!(
                "{}\n{}, {} {}",
                address::street(),
                address::city(),
                address::state(),
                address::zip()
            );
            let email = contact::email();
            vec![name, address, email]
        })
        .sorted_by(|a, b| a[0].cmp(&b[0]))
        .collect_vec()
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
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::vertical([Constraint::Percentage(100)]).split(f.size());

    // colors from https://tailwindcss.com/docs/customizing-colors
    let header_bg = Color::from_str("#1e3a8a").unwrap();
    let header_fg = Color::from_str("#eff6ff").unwrap();
    let header_style = Style::default().fg(header_fg).bg(header_bg);
    let normal_row_color = Color::from_str("#1e293b").unwrap();
    let alt_row_color = Color::from_str("#0f172a").unwrap();
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    let header = ["Name", "Address", "Email"]
        .iter()
        .cloned()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);
    let rows = app.items.iter().enumerate().map(|(i, item)| {
        let color = match i % 2 {
            0 => normal_row_color,
            _ => alt_row_color,
        };
        item.iter()
            .cloned()
            .map(|content| Cell::from(Text::from(format!("\n{}\n", content))))
            .collect::<Row>()
            .style(Style::new().bg(color))
            .height(4)
    });
    let bar = " █ ";
    let t = Table::new(
        rows,
        [
            Constraint::Length(15),
            Constraint::Min(30),
            Constraint::Min(25),
        ],
    )
    .header(header)
    .highlight_style(selected_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]))
    .highlight_spacing(HighlightSpacing::Always);
    f.render_stateful_widget(t, rects[0], &mut app.state);
}
