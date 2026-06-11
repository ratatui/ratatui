//! Move focus through visible controls while skipping disabled controls.
//!
//! The render pass rebuilds the focus target collection from visible fields. Input handling then
//! moves through that previous plan, so hidden or disabled controls do not need special cases in
//! key handling.

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::focus::{FocusFallback, FocusState, FocusTarget, FocusTargets};
use ratatui_layout::linear::Row;

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
    show_advanced: bool,
    save_enabled: bool,
    focus: FocusState<Field>,
    previous_focus: FocusTargets<Field>,
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let area = centered(frame.area(), 58, 5);
        let fields = self.visible_fields();
        let constraints = fields.iter().map(|_| Constraint::Length(12));
        let plan = Row::new(constraints)
            .spacing(1)
            .flex(Flex::Center)
            .regions(area);
        self.previous_focus = FocusTargets::from_targets(
            plan.zip_ids(&fields)
                .enumerate()
                .map(|(order, (region, field))| {
                    FocusTarget::new(field, region.area, order as u16)
                        .disabled(!self.is_enabled(field))
                })
                .collect::<Vec<_>>(),
        );
        self.focus
            .ensure_visible(&self.previous_focus, FocusFallback::First);

        frame.render_widget(Block::new().borders(Borders::ALL).title("focus"), area);
        for (region, field) in plan.zip_ids(&fields) {
            self.render_field(frame, field, region.area);
        }
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Char('a') => self.show_advanced = !self.show_advanced,
            KeyCode::Char('s') => self.save_enabled = !self.save_enabled,
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Tab => {
                self.focus.next(&self.previous_focus);
            }
            KeyCode::Char('h') | KeyCode::Left | KeyCode::BackTab => {
                self.focus.previous(&self.previous_focus);
            }
            _ => {}
        }
        true
    }

    fn visible_fields(&self) -> Vec<Field> {
        let mut fields = vec![Field::Title, Field::Owner, Field::Save];
        if self.show_advanced {
            fields.insert(2, Field::Advanced);
        }
        fields
    }

    fn is_enabled(&self, field: Field) -> bool {
        field != Field::Save || self.save_enabled
    }

    fn render_field(&self, frame: &mut Frame, field: Field, area: Rect) {
        let disabled = !self.is_enabled(field);
        let focused = self.focus.focused() == Some(field);
        let style = if disabled {
            Style::new().fg(Color::DarkGray)
        } else if focused {
            Style::new()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::new().fg(Color::White)
        };
        frame.render_widget(
            Paragraph::new(field.label()).centered().style(style).block(
                Block::new()
                    .borders(Borders::ALL)
                    .title(if disabled { "disabled" } else { "" }),
            ),
            area,
        );
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Field {
    Title,
    Owner,
    Advanced,
    Save,
}

impl Field {
    const fn label(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Owner => "owner",
            Self::Advanced => "advanced",
            Self::Save => "save",
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
