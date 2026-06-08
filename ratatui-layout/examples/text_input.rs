//! Edit a single-line value while keeping cursor and pointer behavior aligned.
//!
//! The app owns the text. `TextInputState` owns only the cursor, `TextEdit` applies common edit
//! commands, and `TextInput` produces the field target plus cursor request for the current frame.

use std::io;

use color_eyre::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind};
use crossterm::execute;
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::{FrameSnapshot, TextEdit, TextInput, TextInputLayout, TextInputState};

fn main() -> Result<()> {
    color_eyre::install()?;

    execute!(io::stdout(), EnableMouseCapture)?;
    let result = run();
    execute!(io::stdout(), DisableMouseCapture)?;
    result
}

fn run() -> Result<()> {
    let mut app = App::default();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| app.render(frame))?;
            match event::read()? {
                event::Event::Key(key) if key.is_press() && !app.handle_key(key.code) => {
                    break Ok(());
                }
                event::Event::Mouse(mouse) => app.handle_mouse(mouse),
                _ => {}
            }
        }
    })
}

#[derive(Debug)]
struct App {
    title: String,
    state: TextInputState,
    previous_layout: Option<TextInputLayout<Field>>,
    previous_frame: FrameSnapshot<Field>,
}

impl Default for App {
    fn default() -> Self {
        let title = String::from("release train");
        Self {
            state: TextInputState::at_end(&title),
            title,
            previous_layout: None,
            previous_frame: FrameSnapshot::new(Rect::default()),
        }
    }
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let area = centered(frame.area(), 50, 5);
        let field_area = area.inner(ratatui::layout::Margin {
            horizontal: 2,
            vertical: 2,
        });
        let input = TextInput::new(Field::Title).prefix_width(Field::label_width());
        let layout = input.layout(field_area, self.state, true);

        frame.render_widget(Block::new().borders(Borders::ALL).title("text input"), area);
        self.render_field(frame, &layout);
        if let Some(cursor) = layout.frame().cursor.final_cursor() {
            frame.set_cursor_position(cursor.position);
        }

        self.previous_frame = layout.frame().clone();
        self.previous_layout = Some(layout);
    }

    fn render_field(&self, frame: &mut Frame, layout: &TextInputLayout<Field>) {
        let text = format!("{}: {}", Field::label(), self.title);
        frame.render_widget(
            Paragraph::new(text).style(Style::new().fg(Color::Black).bg(Color::Green)),
            layout.area(),
        );
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Char(character) => {
                self.state
                    .apply(TextEdit::Insert(character), &mut self.title);
            }
            KeyCode::Backspace => self.state.apply(TextEdit::Backspace, &mut self.title),
            KeyCode::Delete => self.state.apply(TextEdit::Delete, &mut self.title),
            KeyCode::Left => self.state.apply(TextEdit::Left, &mut self.title),
            KeyCode::Right => self.state.apply(TextEdit::Right, &mut self.title),
            KeyCode::Home => self.state.apply(TextEdit::Home, &mut self.title),
            KeyCode::End => self.state.apply(TextEdit::End, &mut self.title),
            _ => {}
        }
        true
    }

    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        if !matches!(mouse.kind, MouseEventKind::Down(_)) {
            return;
        }
        let position = (mouse.column, mouse.row);
        if self.previous_frame.route_click(position).is_some()
            && let Some(layout) = &self.previous_layout
        {
            layout.place_cursor_from_position(position, &self.title, &mut self.state);
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Field {
    Title,
}

impl Field {
    const fn label() -> &'static str {
        "title"
    }

    const fn label_width() -> u16 {
        7
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
