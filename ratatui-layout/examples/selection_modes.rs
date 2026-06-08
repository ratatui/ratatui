//! Compare none, single, and multi selection with the same visible ids.
//!
//! Selection is app state, not layout state. This example keeps the visible ids in a simple slice
//! and lets `SelectionState` enforce how Enter and Space affect the selected commands.

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::{Column, SelectionMode, SelectionState};

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
    mode: SelectionMode,
    cursor: usize,
    selection: SelectionState<Item>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            mode: SelectionMode::Single,
            cursor: 0,
            selection: SelectionState::new(SelectionMode::Single),
        }
    }
}

impl App {
    fn render(&self, frame: &mut Frame) {
        let area = centered(frame.area(), 44, 9);
        let rows = [Constraint::Length(1); ITEMS.len()];
        let plan = Column::new(rows).vertical_margin(1).regions(area);

        frame.render_widget(Block::new().borders(Borders::ALL).title("selection"), area);
        for (region, item) in plan.zip_ids(&ITEMS) {
            self.render_item(frame, item, region.area);
        }
        self.render_status(
            frame,
            Rect::new(area.x + 1, area.bottom() - 2, area.width - 2, 1),
        );
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Char('n') => self.set_mode(SelectionMode::None),
            KeyCode::Char('s') => self.set_mode(SelectionMode::Single),
            KeyCode::Char('m') => self.set_mode(SelectionMode::Multi),
            KeyCode::Char('j') | KeyCode::Down => self.move_cursor(1),
            KeyCode::Char('k') | KeyCode::Up => self.move_cursor(-1),
            KeyCode::Char(' ') | KeyCode::Enter => self.toggle_current(),
            KeyCode::Char('c') => self.selection.clear(),
            _ => {}
        }
        true
    }

    fn set_mode(&mut self, mode: SelectionMode) {
        self.mode = mode;
        self.selection.set_mode(mode);
    }

    fn move_cursor(&mut self, delta: isize) {
        let last = ITEMS.len() - 1;
        self.cursor = self.cursor.saturating_add_signed(delta).min(last);
    }

    fn toggle_current(&mut self) {
        let item = ITEMS[self.cursor];
        match self.mode {
            SelectionMode::None | SelectionMode::Single => self.selection.select(item),
            SelectionMode::Multi => self.selection.toggle(item),
        }
    }

    fn render_item(&self, frame: &mut Frame, item: Item, area: Rect) {
        let cursor = ITEMS[self.cursor] == item;
        let selected = self.selection.is_selected(item);
        let marker = match (cursor, selected) {
            (true, true) => "> [x]",
            (true, false) => "> [ ]",
            (false, true) => "  [x]",
            (false, false) => "  [ ]",
        };
        let style = if selected {
            Style::new().fg(Color::Black).bg(Color::Green)
        } else if cursor {
            Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::new().fg(Color::White)
        };
        frame.render_widget(
            Paragraph::new(format!("{marker} {}", item.label())).style(style),
            area,
        );
    }

    fn render_status(&self, frame: &mut Frame, area: Rect) {
        let text = format!(
            "n none  s single  m multi  c clear   mode: {:?}  selected: {:?}",
            self.mode,
            self.selection.selected(),
        );
        frame.render_widget(Paragraph::new(text), area);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Item {
    Build,
    Test,
    Package,
    Publish,
}

impl Item {
    const fn label(self) -> &'static str {
        match self {
            Self::Build => "build artifacts",
            Self::Test => "run tests",
            Self::Package => "package release",
            Self::Publish => "publish notes",
        }
    }
}

const ITEMS: [Item; 4] = [Item::Build, Item::Test, Item::Package, Item::Publish];

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let [vertical] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    let [horizontal] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(vertical);
    horizontal
}
