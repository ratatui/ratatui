//! Keep dialog geometry separate from ordinary Ratatui rendering.
//!
//! This example uses `Container` for padded dialog layout, `FocusTargets` for field traversal, and
//! `CursorRequests` for terminal cursor placement. Blocks and paragraphs remain ordinary Ratatui
//! widgets rendered into the planned areas.

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Position, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui_layout::container::{Container, Padding};
use ratatui_layout::cursor::{CursorRequest, CursorRequests};
use ratatui_layout::focus::{FocusFallback, FocusState, FocusTarget, FocusTargets};
use ratatui_layout::linear::Column;
use ratatui_layout::regions::Regions;

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
    notes: String,
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let outer = centered(frame.area(), 52, 9);
        let container = Container::new()
            .padding(Padding::new(2, 2, 2, 1))
            .child("fields")
            .layout(outer);
        let field_rows = [
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ];
        let fields = Column::new(field_rows)
            .spacing(1)
            .regions(container.inner)
            .map_id(Field::from_index);
        let focus_plan = FocusTargets::from_regions(fields.regions().iter().copied());

        self.focus.ensure_visible(&focus_plan, FocusFallback::First);

        frame.render_widget(Clear, container.outer);
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("container dialog"),
            container.outer,
        );
        for region in fields.regions() {
            self.render_field(frame, region.id, region.area);
        }
        self.place_cursor(frame, &fields);
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        let plan = Self::static_focus_plan();
        match key {
            KeyCode::Esc | KeyCode::Char('q') => return false,
            KeyCode::Tab | KeyCode::Down => self.focus.next(&plan),
            KeyCode::BackTab | KeyCode::Up => self.focus.previous(&plan),
            KeyCode::Backspace => {
                self.value_mut().pop();
            }
            KeyCode::Char(ch) => self.value_mut().push(ch),
            _ => {}
        }
        true
    }

    fn render_field(&self, frame: &mut Frame, field: Field, area: Rect) {
        let style = if self.focus.focused() == Some(field) {
            Style::new().add_modifier(Modifier::REVERSED)
        } else {
            Style::new()
        };
        frame.render_widget(
            Paragraph::new(format!("{}: {}", field.label(), self.value(field))).style(style),
            area,
        );
    }

    fn place_cursor(&self, frame: &mut Frame, fields: &Regions<Field>) {
        let Some(field) = self.focus.focused() else {
            return;
        };
        let Some(region) = fields.regions().iter().find(|region| region.id == field) else {
            return;
        };
        let x = region
            .area
            .x
            .saturating_add(field.label_width())
            .saturating_add(self.value(field).chars().count() as u16)
            .min(region.area.right().saturating_sub(1));
        let cursor =
            CursorRequests::new().request(CursorRequest::visible(Position::new(x, region.area.y)));
        if let Some(cursor) = cursor.final_cursor() {
            frame.set_cursor_position(cursor.position);
        }
    }

    fn static_focus_plan() -> FocusTargets<Field> {
        FocusTargets::from_targets([
            FocusTarget::new(Field::Title, Rect::default(), 0),
            FocusTarget::new(Field::Owner, Rect::default(), 1),
            FocusTarget::new(Field::Notes, Rect::default(), 2),
        ])
    }

    fn value(&self, field: Field) -> &str {
        match field {
            Field::Title => &self.title,
            Field::Owner => &self.owner,
            Field::Notes => &self.notes,
        }
    }

    fn value_mut(&mut self) -> &mut String {
        match self.focus.focused().unwrap_or(Field::Title) {
            Field::Title => &mut self.title,
            Field::Owner => &mut self.owner,
            Field::Notes => &mut self.notes,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Field {
    Title,
    Owner,
    Notes,
}

impl Field {
    const fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Title,
            1 => Self::Owner,
            _ => Self::Notes,
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Owner => "owner",
            Self::Notes => "notes",
        }
    }

    const fn label_width(self) -> u16 {
        self.label().len() as u16 + 2
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
