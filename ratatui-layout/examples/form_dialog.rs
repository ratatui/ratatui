//! Route focus and cursor placement through app-owned controls in an overlay dialog.
//!
//! The overlay only values regions. `App` owns focus state and field data, and the cursor request
//! is produced from the focused field after layout.

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui_layout::cursor::CursorRequests;
use ratatui_layout::focus::{FocusFallback, FocusState, FocusTarget, FocusTargets};
use ratatui_layout::input::TextFieldState;
use ratatui_layout::overlay::Overlay;
use ratatui_layout::regions::{Region, Regions};

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

#[derive(Debug, Default)]
struct App {
    focus: FocusState<Field>,
    title: String,
    owner: String,
    title_field: TextFieldState,
    owner_field: TextFieldState,
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let dialog = centered(frame.area(), 48, 7);
        let plan = Overlay::new()
            .region(Region::new(
                Field::Title,
                Rect::new(dialog.x + 2, dialog.y + 2, dialog.width - 4, 1),
            ))
            .region(Region::new(
                Field::Owner,
                Rect::new(dialog.x + 2, dialog.y + 4, dialog.width - 4, 1),
            ))
            .regions(dialog);
        let focus_plan = Self::focus_plan(&plan);
        self.focus.ensure_visible(&focus_plan, FocusFallback::First);

        frame.render_widget(Clear, dialog);
        frame.render_widget(Block::new().borders(Borders::ALL).title("task"), dialog);
        self.render_field(frame, &plan, Field::Title, "title");
        self.render_field(frame, &plan, Field::Owner, "owner");
        self.place_cursor(frame, &plan);
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Tab | KeyCode::Down => self.focus_next(),
            KeyCode::BackTab | KeyCode::Up => self.focus_previous(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Home => self.move_cursor_home(),
            KeyCode::End => self.move_cursor_end(),
            KeyCode::Backspace => self.backspace(),
            KeyCode::Delete => self.delete(),
            KeyCode::Char('q') | KeyCode::Esc => return Self::quit(),
            KeyCode::Char(ch) => self.insert(ch),
            _ => {}
        }
        true
    }

    fn focus_next(&mut self) {
        let plan = Self::static_focus_plan();
        self.focus.next(&plan);
    }

    fn focus_previous(&mut self) {
        let plan = Self::static_focus_plan();
        self.focus.previous(&plan);
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

    fn delete(&mut self) {
        match self.focus.focused() {
            Some(Field::Title) => self.title_field.delete(&mut self.title),
            Some(Field::Owner) => self.owner_field.delete(&mut self.owner),
            None => {}
        }
    }

    const fn move_cursor_left(&mut self) {
        match self.focus.focused() {
            Some(Field::Title) => self.title_field.move_left(),
            Some(Field::Owner) => self.owner_field.move_left(),
            None => {}
        }
    }

    fn move_cursor_right(&mut self) {
        match self.focus.focused() {
            Some(Field::Title) => self.title_field.move_right(&self.title),
            Some(Field::Owner) => self.owner_field.move_right(&self.owner),
            None => {}
        }
    }

    const fn move_cursor_home(&mut self) {
        match self.focus.focused() {
            Some(Field::Title) => self.title_field.move_home(),
            Some(Field::Owner) => self.owner_field.move_home(),
            None => {}
        }
    }

    fn move_cursor_end(&mut self) {
        match self.focus.focused() {
            Some(Field::Title) => self.title_field.move_end(&self.title),
            Some(Field::Owner) => self.owner_field.move_end(&self.owner),
            None => {}
        }
    }

    fn render_field(&self, frame: &mut Frame, plan: &Regions<Field>, field: Field, label: &str) {
        let area = plan
            .regions()
            .iter()
            .find(|region| region.id == field)
            .unwrap()
            .area;
        let text = format!("{label}: {}", self.value(field));
        let style = if self.focus.focused() == Some(field) {
            Style::new().add_modifier(Modifier::REVERSED)
        } else {
            Style::new()
        };
        frame.render_widget(Paragraph::new(text).style(style), area);
    }

    fn place_cursor(&self, frame: &mut Frame, plan: &Regions<Field>) {
        let Some(field) = self.focus.focused() else {
            return;
        };
        let region = plan
            .regions()
            .iter()
            .find(|region| region.id == field)
            .unwrap();
        let cursor = CursorRequests::new().request(
            self.field_state(field)
                .cursor_request_after_prefix(region.area, field.label_width()),
        );
        if let Some(cursor) = cursor.final_cursor() {
            frame.set_cursor_position(cursor.position);
        }
    }

    fn focus_plan(plan: &Regions<Field>) -> FocusTargets<Field> {
        FocusTargets::from_regions(plan.regions().iter().copied())
    }

    fn static_focus_plan() -> FocusTargets<Field> {
        FocusTargets::from_targets([
            FocusTarget::new(Field::Title, Rect::default(), 0),
            FocusTarget::new(Field::Owner, Rect::default(), 1),
        ])
    }

    fn value(&self, field: Field) -> &str {
        match field {
            Field::Title => &self.title,
            Field::Owner => &self.owner,
        }
    }

    const fn field_state(&self, field: Field) -> TextFieldState {
        match field {
            Field::Title => self.title_field,
            Field::Owner => self.owner_field,
        }
    }

    const fn quit() -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Field {
    Title,
    Owner,
}

impl Field {
    const fn label_width(self) -> u16 {
        match self {
            Self::Title | Self::Owner => 7,
        }
    }
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let [vertical] = Layout::vertical([Constraint::Length(height)])
        .flex(ratatui::layout::Flex::Center)
        .areas(area);
    let [horizontal] = Layout::horizontal([Constraint::Length(width)])
        .flex(ratatui::layout::Flex::Center)
        .areas(vertical);
    horizontal
}
