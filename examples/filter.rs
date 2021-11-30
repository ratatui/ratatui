use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
struct App<'a> {
    filter: String,
    input_mode: InputMode,
    words: Vec<&'a str>,
}

impl<'a> App<'a> {
    fn new(words: impl IntoIterator<Item = &'a str>) -> Self {
        App {
            filter: String::new(),
            input_mode: InputMode::Normal,
            words: words.into_iter().collect(),
        }
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
    let app = App::new(WORDS);
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
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;
        if let Event::Key(key) = event::read()? {
            match (&app.input_mode, key.code) {
                (InputMode::Normal, KeyCode::Char('e')) => {
                    app.input_mode = InputMode::Editing;
                }
                (InputMode::Normal, KeyCode::Char('q')) => {
                    return Ok(());
                }
                (InputMode::Normal, _) => {}
                (InputMode::Editing, KeyCode::Char(c)) => {
                    app.filter.push(c);
                }
                (InputMode::Editing, KeyCode::Backspace) => {
                    app.filter.pop();
                }
                (InputMode::Editing, KeyCode::Esc) => {
                    app.input_mode = InputMode::Normal;
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing."),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Line::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.filter.clone())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}
        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after
            // rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.filter.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }

    let items: Vec<ListItem> = app
        .words
        .iter()
        .filter(|m| m.contains(&app.filter))
        .map(|m| {
            let content = vec![Line::from(Span::raw(*m))];
            ListItem::new(content)
        })
        .collect();
    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(list, chunks[2]);
}

const WORDS: [&str; 100] = [
    "the", "at", "there", "some", "my", "of", "be", "use", "her", "than", "and", "this", "an",
    "would", "first", "a", "have", "each", "make", "water", "to", "from", "which", "like", "been",
    "in", "or", "she", "him", "call", "is", "one", "do", "into", "who", "you", "had", "how",
    "time", "oil", "that", "by", "their", "has", "its", "it", "word", "if", "look", "now", "he",
    "but", "will", "two", "find", "was", "not", "up", "more", "long", "for", "what", "other",
    "write", "down", "on", "all", "about", "go", "day", "are", "were", "out", "see", "did", "as",
    "we", "many", "number", "get", "with", "when", "then", "no", "come", "his", "your", "them",
    "way", "made", "they", "can", "these", "could", "may", "I", "said", "so", "people", "part",
];
