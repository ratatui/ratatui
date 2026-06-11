//! Render a command row from one action description.
//!
//! A toolbar, button row, tab bar, and horizontal menu all need the same coordination: solve named
//! action regions, draw labels into those regions, skip disabled actions for input, and route
//! clicks back to command ids. `CommandRow` keeps the local left/right focus state while
//! `ActionSurface` creates frame-local regions and targets.

use std::io;

use color_eyre::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind};
use crossterm::execute;
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::action::CommandRow;
use ratatui_layout::frame::FrameSnapshot;

fn main() -> Result<()> {
    color_eyre::install()?;

    execute!(io::stdout(), EnableMouseCapture)?;
    let result = run();
    execute!(io::stdout(), DisableMouseCapture)?;
    result
}

fn run() -> Result<()> {
    let mut app = App::new();
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
    dirty: bool,
    selected: Command,
    row: CommandRow<Command>,
    previous_frame: FrameSnapshot<Command>,
}

impl App {
    fn new() -> Self {
        let row = CommandRow::new(Command::slots())
            .spacing(1)
            .flex(Flex::Center);
        Self {
            dirty: false,
            selected: Command::Open,
            row,
            previous_frame: FrameSnapshot::new(Rect::default()),
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = centered(frame.area(), 42, 5);
        let row_area = area.inner(ratatui::layout::Margin {
            horizontal: 2,
            vertical: 2,
        });
        let layout = self
            .row
            .layout_with(row_area, |command| !self.is_enabled(command));

        frame.render_widget(
            Block::new().borders(Borders::ALL).title("action surface"),
            area,
        );
        for region in layout.regions().regions() {
            self.render_command(frame, region.id, region.area);
        }

        self.previous_frame = layout.into_frame();
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Char('d') => self.dirty = !self.dirty,
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Tab => {
                let dirty = self.dirty;
                self.row
                    .move_next_enabled(|command| !is_enabled(command, dirty));
            }
            KeyCode::Char('h') | KeyCode::Left | KeyCode::BackTab => {
                let dirty = self.dirty;
                self.row
                    .move_previous_enabled(|command| !is_enabled(command, dirty));
            }
            KeyCode::Char(' ') | KeyCode::Enter => self.activate_focused(),
            _ => {}
        }
        true
    }

    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        if !matches!(mouse.kind, MouseEventKind::Down(_)) {
            return;
        }
        if let Some(hit) = self.previous_frame.route_click((mouse.column, mouse.row)) {
            self.row.focus_id(hit.id);
            self.activate(hit.id);
        }
    }

    fn activate_focused(&mut self) {
        if let Some(command) = self.row.focused_id() {
            self.activate(command);
        }
    }

    fn activate(&mut self, command: Command) {
        if !self.is_enabled(command) {
            return;
        }
        self.selected = command;
        if command == Command::Open {
            self.dirty = true;
        }
        if command == Command::Save {
            self.dirty = false;
        }
    }

    fn is_enabled(&self, command: Command) -> bool {
        is_enabled(command, self.dirty)
    }

    fn render_command(&self, frame: &mut Frame, command: Command, area: Rect) {
        let focused = self.row.focused_id() == Some(command);
        let style = if !self.is_enabled(command) {
            Style::new().fg(Color::DarkGray)
        } else if focused {
            Style::new()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else if self.selected == command {
            Style::new().fg(Color::Black).bg(Color::Green)
        } else {
            Style::new().fg(Color::White).bg(Color::DarkGray)
        };
        frame.render_widget(
            Paragraph::new(command.label()).centered().style(style),
            area,
        );
    }
}

fn is_enabled(command: Command, dirty: bool) -> bool {
    command != Command::Save || dirty
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Command {
    Open,
    Save,
    Close,
}

impl Command {
    const fn slots() -> [(Self, Constraint); 3] {
        [
            (Self::Open, Constraint::Length(10)),
            (Self::Save, Constraint::Length(10)),
            (Self::Close, Constraint::Length(10)),
        ]
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Save => "save",
            Self::Close => "close",
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
