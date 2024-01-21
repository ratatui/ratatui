use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};

const TODO_HEADER_BG: Color = tailwind::BLUE.c950;
const BG: Color = tailwind::SLATE.c950;
const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const ALT_ROW_COLOR: Color = tailwind::SLATE.c900;
const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
const TEXT_COLOR: Color = tailwind::SLATE.c200;
const COMPLETED_TEXT_COLOR: Color = tailwind::GREEN.c600;

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
    last_selected: Option<usize>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
            last_selected: None,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        let offset = self.state.offset();
        self.last_selected = self.state.selected();
        self.state.select(None);
        *self.state.offset_mut() = offset;
    }
}

/// This struct holds the current state of the app. In particular, it has the `items` field which is
/// a wrapper around `ListState`. Keeping track of the items state let us render the associated
/// widget with its state and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
struct App<'a> {
    items: StatefulList<(&'a str, &'a str, bool)>,
    events: Vec<String>,
}

impl<'a> App<'a> {
    /// Changes the status of the selected list item
    fn change_status(&mut self) {
        if let Some(i) = self.items.state.selected() {
            self.items.items[i].2 = !&self.items.items[i].2
        }
    }

    fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(vec![
                ("Rewrite everything with Rust!", "I can't hold my inner voice. He tells me to rewrite the complete universe with Rust", false),
                ("Rewrite all of your tui apps with Ratatui", "Yes, you heard that right. Go and replace your tui with Ratatui.", true),
                ("Pet your cat", "Minnak loves to be pet by you! Don't forget to pet and give some treats!", false),
                ("Walk with your dog", "Max is bored, go walk with him!", false),
                ("Pay the bills", "Pay the train subscription!!!", true),
                ("Refactor list example", "If you see this info that means I completed this task!", true),
            ]),
            events: Self::create_random_events()
        }
    }

    /// This only exists to simulate some kind of "progress"
    fn on_tick(&mut self) {
        let event = self.events.remove(0);
        self.events.push(event);
    }

    fn create_random_events() -> Vec<String> {
        let mut events_vec = vec![];
        for _ in 0..50 {
            let event = match rand::random::<u8>() % 5 {
                0 => "INFO".into(),
                1 => "CRITICAL".into(),
                2 => "WARNING".into(),
                3 => "PANIC!".into(),
                4 => "ERROR".into(),
                _ => unreachable!(),
            };
            events_vec.push(event)
        }
        events_vec
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
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    use KeyCode::*;
                    match key.code {
                        Char('q') | Esc => return Ok(()),
                        Char('h') | Left => app.items.unselect(),
                        Char('j') | Down => app.items.next(),
                        Char('k') | Up => app.items.previous(),
                        Char('l') | Right | Enter => app.change_status(),
                        _ => {}
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    // Create a space for header
    let vertical = Layout::vertical([Constraint::Length(2), Constraint::Percentage(100)]);
    let [header_area, rest_area] = f.size().split(&vertical);

    // Create two chunks with equal horizontal screen space
    let horizontal = Layout::horizontal([Constraint::Percentage(50), Constraint::Max(15)]);
    let [item_list_area, event_list_area] = rest_area.split(&horizontal);

    // Create two chunks with equal vertical screen space. One for the list and the other for the
    // info block.
    let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
    let [upper_item_list_area, lower_item_list_area] = item_list_area.split(&vertical);

    let header = Block::default()
        .title("Use j k or ↓ ↑ to move, h or ← to unselect, l → to change status.")
        .title_alignment(Alignment::Center);
    f.render_widget(header, header_area);

    render_todo(f, app, &[upper_item_list_area, lower_item_list_area]);

    render_events_list(f, app, event_list_area)
}

// Renders todo list and info part
fn render_todo(f: &mut Frame, app: &mut App, area: &[Rect; 2]) {
    // We create two blocks, one is for the header (outer) and the other is for list (inner).
    let outer_block = Block::default()
        .borders(Borders::NONE)
        .fg(TEXT_COLOR)
        .bg(TODO_HEADER_BG)
        .title("TODO List")
        .title_alignment(Alignment::Center);
    let inner_block = Block::default()
        .borders(Borders::NONE)
        .fg(TEXT_COLOR)
        .bg(NORMAL_ROW_COLOR);

    // We get the inner area from outer_block. We'll use this area later to render the table.
    let outer_area = area[0];
    let inner_area = outer_block.inner(outer_area);

    // We can render the header in outer_area.
    f.render_widget(outer_block, outer_area);

    // Iterate through all elements in the `items` and stylize them.
    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .enumerate()
        .map(|(i, (todo, _info, status))| {
            let bg_color = match i % 2 {
                0 => NORMAL_ROW_COLOR,
                _ => ALT_ROW_COLOR,
            };
            let (style, todo_str) = match status {
                true => (
                    Style::default().fg(COMPLETED_TEXT_COLOR).bg(bg_color),
                    " ✓ ".to_string() + todo,
                ),
                false => (
                    Style::default().fg(TEXT_COLOR).bg(bg_color),
                    " ☐ ".to_string() + todo,
                ),
            };
            ListItem::new(Line::from(todo_str)).style(style)
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(inner_block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
                .fg(SELECTED_STYLE_FG),
        )
        .highlight_symbol(">");

    // We can now render the item list
    f.render_stateful_widget(items, inner_area, &mut app.items.state);

    let info = if let Some(i) = app.items.state.selected() {
        match app.items.items[i].2 {
            true => "✓ DONE: ".to_string() + app.items.items[i].1,
            false => "TODO: ".to_string() + app.items.items[i].1,
        }
    } else {
        "Nothing to see here...".to_string()
    };

    // We show the list item's info under the list in this paragraph
    let outer_info_block = Block::default()
        .borders(Borders::NONE)
        .fg(TEXT_COLOR)
        .bg(TODO_HEADER_BG)
        .title("TODO Info")
        .title_alignment(Alignment::Center);
    let inner_info_block = Block::default()
        .borders(Borders::NONE)
        .bg(NORMAL_ROW_COLOR)
        .padding(Padding::horizontal(1));

    // This is a similar process to what we did for list. outer_info_area will be used for header
    // inner_info_area will be used for the list info.
    let outer_info_area = area[1];
    let inner_info_area = outer_info_block.inner(outer_info_area);

    // We can render the header. Inner info will be rendered later
    f.render_widget(outer_info_block, outer_info_area);

    let info_paragraph = Paragraph::new(info)
        .block(inner_info_block)
        .fg(TEXT_COLOR)
        .wrap(Wrap { trim: false });

    // We can now render the item info
    f.render_widget(info_paragraph, inner_info_area);
}

// Renders event list
fn render_events_list(f: &mut Frame, app: &App, area: Rect) {
    let events: Vec<ListItem> = app
        .events
        .iter()
        .rev()
        .map(|level| {
            // sets event_style to the tailwind color depending on its level
            let event_style = match level.as_str() {
                "CRITICAL" => Style::default().fg(tailwind::RED.c800),
                "ERROR" => Style::default().fg(tailwind::RED.c600),
                "WARNING" => Style::default().fg(tailwind::YELLOW.c600),
                "INFO" => Style::default().fg(tailwind::BLUE.c700),
                "PANIC!" => Style::default().fg(tailwind::ORANGE.c600),
                _ => Style::default(),
            };

            // Creates a list item with styled span
            ListItem::new(Span::styled(level, event_style))
        })
        .collect();
    let events_list = List::new(events)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .fg(TEXT_COLOR)
                .bg(BG)
                .title("Events List")
                .title_alignment(Alignment::Center),
        )
        .direction(ListDirection::BottomToTop);

    // Renders the event table to the right side of the screen.
    f.render_widget(events_list, area);
}
