use std::io::stdout;
/// A Ratatui example that demonstrates the Elm architecture with a basic list - detail
/// application.
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
use std::time::Duration;

use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind,
};
use crossterm::ExecutableCommand;
use fakeit::{address, contact, name};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Row, Table, TableState};
use ratatui::Frame;

/// Application data model and state
#[derive(Debug, Default)]
struct AppModel {
    table_items: Vec<Person>,
    table_state: TableState,
    running_state: RunningState,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default)]
struct Person {
    name: String,
    address: String,
    email: String,
}

#[derive(PartialEq, Copy, Clone)]
enum Message {
    SelectNext,
    SelectPrevious,
    Quit,
}

fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    stdout().execute(EnableMouseCapture)?;

    let mut model = AppModel {
        table_items: generate_some_people(),
        ..Default::default()
    };

    // Select the first row if no row is selected
    if model.table_state.selected().is_none() {
        model.table_state.select_first();
    }

    while model.running_state != RunningState::Done {
        // Render the current view
        terminal.draw(|f| view(&mut model, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&model)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }
    stdout().execute(DisableMouseCapture)?;
    tui::restore_terminal()?;
    Ok(())
}

fn view(model: &mut AppModel, frame: &mut Frame) {
    let [top, bottom] = Layout::vertical([Constraint::Fill(1); 2]).areas(frame.area());
    render_table(model, top, frame);
    render_detail(model, bottom, frame);
}

fn render_table(model: &mut AppModel, area: Rect, frame: &mut Frame) {
    let header = Row::new(vec!["Name", "Address", "Email"])
        .style(Style::default().add_modifier(Modifier::BOLD))
        .height(1);

    let rows = model.table_items.iter().map(|data| {
        Row::new(vec![
            data.name.as_str(),
            data.address.as_str(),
            data.email.as_str(),
        ])
        .height(2)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(20),
            Constraint::Min(10),
            Constraint::Fill(1),
        ],
    )
    .header(header)
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED | Modifier::ITALIC))
    .block(Block::default().borders(Borders::ALL).title(" People "));

    frame.render_stateful_widget(table, area, &mut model.table_state);
}

fn render_detail(model: &mut AppModel, area: Rect, frame: &mut Frame) {
    let selected_item = &model.table_items[model.table_state.selected().unwrap()];
    let detail = Paragraph::new(format!(
        "{}\n\n{}",
        selected_item.email, selected_item.address
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", selected_item.name))
            .title_bottom(" (Esc/q) quit | (↑) move up | (↓) move down ")
            .padding(Padding::new(1, 1, 1, 1)),
    );
    frame.render_widget(detail, area);
}

fn handle_event(_: &AppModel) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        match event::read()? {
            Event::Key(key) if key.kind == event::KeyEventKind::Press => Ok(handle_key(key)),
            Event::Mouse(mouse) => Ok(handle_mouse(mouse)),
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

const fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Some(Message::SelectNext),
        KeyCode::Char('k') | KeyCode::Up => Some(Message::SelectPrevious),
        KeyCode::Char('q') | KeyCode::Esc => Some(Message::Quit),
        _ => None,
    }
}

const fn handle_mouse(mouse: event::MouseEvent) -> Option<Message> {
    match mouse.kind {
        MouseEventKind::ScrollDown => Some(Message::SelectPrevious),
        MouseEventKind::ScrollUp => Some(Message::SelectNext),
        _ => None,
    }
}

fn update(model: &mut AppModel, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => model.running_state = RunningState::Done,
        Message::SelectNext => {
            let current_index = model.table_state.selected().unwrap_or(0);
            if current_index < model.table_items.len() - 1 {
                model.table_state.select(Some(current_index + 1));
            }
        }
        Message::SelectPrevious => {
            let current_index = model.table_state.selected().unwrap_or(0);
            if current_index > 0 {
                model.table_state.select(Some(current_index - 1));
            }
        }
    };
    None
}

mod tui {
    use std::io::stdout;
    use std::panic;

    use ratatui::backend::{Backend, CrosstermBackend};
    use ratatui::crossterm::terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    };
    use ratatui::crossterm::ExecutableCommand;
    use ratatui::Terminal;

    pub fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> color_eyre::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn install_panic_hook() {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            original_hook(panic_info);
        }));
    }
}

fn generate_some_people() -> Vec<Person> {
    (0..50)
        .map(|_| Person {
            name: name::full(),
            address: generate_fake_address(),
            email: contact::email(),
        })
        .collect()
}

fn generate_fake_address() -> String {
    format!(
        "{}\n{}, {} {}",
        address::street(),
        address::city(),
        address::state(),
        address::zip()
    )
}
