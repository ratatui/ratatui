use std::io::stdout;
use std::panic;
/// A Ratatui example that demonstrates the Elm architecture with a basic list - detail
/// application.
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
use std::{fmt, time::Duration};

use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind,
};
use crossterm::ExecutableCommand;
use fakeit::{address, contact, name};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Padding, Paragraph, Row, Table, TableState};
use ratatui::{DefaultTerminal, Frame};

/// Application data model and state
#[derive(Debug)]
struct AppModel {
    people: Vec<Person>,
    table_state: TableState,
}

#[derive(Debug)]
struct Person {
    name: String,
    address: Address,
    email: String,
}

#[derive(Debug)]
struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
}

#[derive(PartialEq, Copy, Clone)]
enum Message {
    SelectNext,
    SelectPrevious,
    Quit,
}

fn main() -> color_eyre::Result<()> {
    let terminal = ratatui::init();
    install_panic_hook();
    stdout().execute(EnableMouseCapture)?;

    let model = AppModel::default();
    let result = model.run(terminal);

    stdout().execute(DisableMouseCapture)?;
    ratatui::restore();
    result
}

pub fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        stdout().execute(DisableMouseCapture).ok();
        original_hook(panic_info);
    }));
}

impl Default for AppModel {
    fn default() -> Self {
        Self {
            people: (0..50).map(|_| Person::fake()).collect(),
            table_state: TableState::default().with_selected(0),
        }
    }
}

impl AppModel {
    fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        loop {
            terminal.draw(|frame| self.view(frame))?;

            let mut maybe_message = handle_event()?;
            while let Some(message) = maybe_message {
                if message == Message::Quit {
                    return Ok(());
                }
                maybe_message = self.update(message);
            }
        }
    }

    fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Quit => {}
            Message::SelectNext => self.table_state.select_next(),
            Message::SelectPrevious => self.table_state.select_previous(),
        };
        None
    }

    /// Render the current view
    fn view(&mut self, frame: &mut Frame) {
        let [top, bottom] = Layout::vertical([Constraint::Fill(1); 2]).areas(frame.area());
        self.render_table(frame, top);
        self.render_detail(frame, bottom);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header = Row::new(vec!["Name", "Address", "Email"])
            .style(Modifier::BOLD)
            .height(1);
        let rows = self.people.iter().map(Row::from);
        let widths = [
            Constraint::Length(20),
            Constraint::Min(10),
            Constraint::Fill(1),
        ];
        let table = Table::new(rows, widths)
            .header(header)
            .row_highlight_style(Style::new().add_modifier(Modifier::REVERSED | Modifier::ITALIC))
            .block(Block::bordered().title(" People "));

        frame.render_stateful_widget(table, area, &mut self.table_state);
    }

    fn render_detail(&self, frame: &mut Frame, area: Rect) {
        let selected_item = &self.people[self.table_state.selected().unwrap()];
        let block = Block::bordered()
            .title(format!(" {} ", selected_item.name))
            .title_bottom(" (Esc/q) quit | (↑) move up | (↓) move down ")
            .padding(Padding::new(1, 1, 1, 1));
        let text = format!("{}\n\n{}", selected_item.email, selected_item.address);
        let detail = Paragraph::new(text).block(block);
        frame.render_widget(detail, area);
    }
}

/// Handle events and map to a Message
fn handle_event() -> color_eyre::Result<Option<Message>> {
    if !event::poll(Duration::from_millis(250))? {
        return Ok(None);
    }
    match event::read()? {
        Event::Key(key) if key.kind == event::KeyEventKind::Press => Ok(handle_key(key)),
        Event::Mouse(mouse) => Ok(handle_mouse(mouse)),
        _ => Ok(None),
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

impl Person {
    fn fake() -> Self {
        Self {
            name: name::full(),
            address: Address::fake(),
            email: contact::email(),
        }
    }
}

impl<'a> From<&'a Person> for Row<'a> {
    fn from(person: &'a Person) -> Self {
        Row::new(vec![
            person.name.clone(),
            person.address.to_string(),
            person.email.clone(),
        ])
        .height(2)
    }
}

impl Address {
    fn fake() -> Self {
        Self {
            street: address::street(),
            city: address::city(),
            state: address::state(),
            zip: address::zip(),
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}, {} {}",
            self.street, self.city, self.state, self.zip
        )
    }
}
