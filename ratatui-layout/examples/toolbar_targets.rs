//! Build toolbar regions once, then reuse them for rendering, focus, and mouse routing.
//!
//! Ratatui's `Layout` can split the toolbar, but event handlers still need to know which command
//! occupied each rectangle in the previous frame. This example keeps command ids beside the planned
//! regions, then asks `FrameTargets` to produce the matching `FrameSnapshot`.

use color_eyre::Result;
use crossterm::event::{self, KeyCode, MouseEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::focus::{FocusFallback, FocusState};
use ratatui_layout::frame::{FrameSnapshot, FrameTargets};
use ratatui_layout::linear::Row;

fn main() -> Result<()> {
    color_eyre::install()?;

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
    dirty: bool,
    focus: FocusState<Command>,
    selected: Command,
    previous_frame: FrameSnapshot<Command>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            dirty: false,
            focus: FocusState::default(),
            selected: Command::Open,
            previous_frame: FrameSnapshot::new(Rect::default()),
        }
    }
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let area = centered(frame.area(), 40, 3);
        let commands = Command::regions();
        let plan = Row::named(commands)
            .spacing(1)
            .flex(Flex::Center)
            .regions(area);

        let next_frame = FrameTargets::from_regions(plan, 0)
            .disabled(|command| !self.is_enabled(command))
            .build();
        self.focus
            .ensure_visible(&next_frame.focus, FocusFallback::First);

        frame.render_widget(Block::new().borders(Borders::ALL).title("toolbar"), area);
        for region in next_frame.layout.regions() {
            self.render_command(frame, region.id, region.area);
        }
        self.previous_frame = next_frame;
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Char('d') => self.dirty = !self.dirty,
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Tab => {
                self.focus.next(&self.previous_frame.focus);
            }
            KeyCode::Char('h') | KeyCode::Left | KeyCode::BackTab => {
                self.focus.previous(&self.previous_frame.focus);
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
            self.activate(hit.id);
        }
    }

    fn activate_focused(&mut self) {
        if let Some(command) = self.focus.focused() {
            self.activate(command);
        }
    }

    fn activate(&mut self, command: Command) {
        if !self.is_enabled(command) {
            return;
        }
        self.selected = command;
        self.dirty = command == Command::Open;
    }

    fn is_enabled(&self, command: Command) -> bool {
        command != Command::Save || self.dirty
    }

    fn render_command(&self, frame: &mut Frame, command: Command, area: Rect) {
        let enabled = self.is_enabled(command);
        let style = if !enabled {
            Style::new().fg(Color::DarkGray)
        } else if self.selected == command {
            Style::new().fg(Color::Black).bg(Color::Green)
        } else if self.focus.focused() == Some(command) {
            Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::new().fg(Color::White)
        };

        frame.render_widget(
            Paragraph::new(command.label()).centered().style(style),
            area,
        );
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Command {
    Open,
    Save,
    Close,
}

impl Command {
    const fn regions() -> [(Self, Constraint); 3] {
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
