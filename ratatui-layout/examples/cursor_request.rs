//! Request terminal cursor placement from whichever field has focus.
//!
//! The focused field computes a cursor position during rendering and records it in
//! `CursorRequests`. The frame applies the final visible request after all fields have had a chance
//! to contribute.

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::cursor::CursorRequests;
use ratatui_layout::focus::{FocusFallback, FocusState, FocusTarget, FocusTargets};
use ratatui_layout::input::TextFieldState;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = App::default();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| app.render(frame))?;
            if let Some(key) = event::read()?.as_key_press_event()
                && !app.handle_key(key.code)
            {
                break Ok(());
            }
        }
    })
}

#[derive(Debug)]
struct App {
    title: String,
    owner: String,
    title_field: TextFieldState,
    owner_field: TextFieldState,
    focus: FocusState<Field>,
    previous_focus: FocusTargets<Field>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            title: String::from("release train"),
            owner: String::from("platform"),
            title_field: TextFieldState::at_end("release train"),
            owner_field: TextFieldState::at_end("platform"),
            focus: FocusState::default(),
            previous_focus: FocusTargets::new(),
        }
    }
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let area = centered(frame.area(), 48, 6);
        let [title_area, owner_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
                .spacing(1)
                .margin(2)
                .areas(area);
        self.previous_focus = FocusTargets::from_targets([
            FocusTarget::new(Field::Title, title_area, 0),
            FocusTarget::new(Field::Owner, owner_area, 1),
        ]);
        self.focus
            .ensure_visible(&self.previous_focus, FocusFallback::First);

        let mut cursor = CursorRequests::new();
        frame.render_widget(Block::new().borders(Borders::ALL).title("cursor"), area);
        cursor = cursor.merge(self.render_field(frame, Field::Title, title_area));
        cursor = cursor.merge(self.render_field(frame, Field::Owner, owner_area));
        if let Some(cursor) = cursor.final_cursor() {
            frame.set_cursor_position(cursor.position);
        }
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Tab | KeyCode::Down => self.focus.next(&self.previous_focus),
            KeyCode::BackTab | KeyCode::Up => self.focus.previous(&self.previous_focus),
            KeyCode::Left => self.field_state().move_left(),
            KeyCode::Right => {
                let value = self.field_value().to_string();
                self.field_state().move_right(&value);
            }
            KeyCode::Backspace => self.backspace(),
            KeyCode::Char(ch) => self.insert(ch),
            _ => {}
        }
        true
    }

    fn insert(&mut self, ch: char) {
        match self.focus.focused() {
            Some(Field::Title) => self.title_field.insert_char(&mut self.title, ch),
            Some(Field::Owner) => self.owner_field.insert_char(&mut self.owner, ch),
            None => {}
        }
    }

    fn backspace(&mut self) {
        match self.focus.focused() {
            Some(Field::Title) => self.title_field.backspace(&mut self.title),
            Some(Field::Owner) => self.owner_field.backspace(&mut self.owner),
            None => {}
        }
    }

    fn render_field(&self, frame: &mut Frame, field: Field, area: Rect) -> CursorRequests {
        let focused = self.focus.focused() == Some(field);
        let style = if focused {
            Style::new().fg(Color::Black).bg(Color::Green)
        } else {
            Style::new().fg(Color::White)
        };
        let text = format!("{}: {}", field.label(), self.value(field));
        frame.render_widget(Paragraph::new(text).style(style), area);
        if focused {
            let field_state = self.text_field(field);
            let request = field_state.cursor_request_after_prefix(area, field.label_width());
            CursorRequests::new().request(request)
        } else {
            CursorRequests::new()
        }
    }

    const fn field_state(&mut self) -> &mut TextFieldState {
        match self.focus.focused() {
            Some(Field::Owner) => &mut self.owner_field,
            Some(Field::Title) | None => &mut self.title_field,
        }
    }

    fn field_value(&self) -> &str {
        match self.focus.focused() {
            Some(Field::Owner) => &self.owner,
            Some(Field::Title) | None => &self.title,
        }
    }

    fn value(&self, field: Field) -> &str {
        match field {
            Field::Title => &self.title,
            Field::Owner => &self.owner,
        }
    }

    const fn text_field(&self, field: Field) -> TextFieldState {
        match field {
            Field::Title => self.title_field,
            Field::Owner => self.owner_field,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Field {
    Title,
    Owner,
}

impl Field {
    const fn label(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Owner => "owner",
        }
    }

    const fn label_width(self) -> u16 {
        match self {
            Self::Title | Self::Owner => 7,
        }
    }
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let [vertical] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    let [horizontal] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(vertical);
    horizontal
}
