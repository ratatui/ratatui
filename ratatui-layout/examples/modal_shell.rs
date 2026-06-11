//! Build a modal shell, then merge child button targets into it.
//!
//! `ModalShell` solves the centered dialog area, padded content area, and optional backdrop target.
//! The app still chooses the form fields, buttons, styles, and whether an outside click dismisses
//! the dialog.

use std::io;

use color_eyre::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind};
use crossterm::execute;
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui_layout::action::ActionSurface;
use ratatui_layout::container::Padding;
use ratatui_layout::focus::{FocusFallback, FocusState};
use ratatui_layout::frame::FrameSnapshot;
use ratatui_layout::modal::ModalShell;

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
    open: bool,
    message: &'static str,
    focus: FocusState<Target>,
    previous_frame: FrameSnapshot<Target>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            open: true,
            message: "modal open; click outside to cancel",
            focus: FocusState::default(),
            previous_frame: FrameSnapshot::new(Rect::default()),
        }
    }
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        self.render_page(frame);
        self.previous_frame = FrameSnapshot::new(frame.area());
        if self.open {
            self.previous_frame = self.render_modal(frame);
        }
    }

    fn render_page(&self, frame: &mut Frame) {
        let [body, status] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());
        frame.render_widget(
            Paragraph::new("release action pending\n\npress o to reopen, q to quit").block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("page behind modal"),
            ),
            body,
        );
        frame.render_widget(Paragraph::new(self.message), status);
    }

    fn render_modal(&mut self, frame: &mut Frame) -> FrameSnapshot<Target> {
        let modal = ModalShell::new(44, 9)
            .padding(Padding::new(2, 2, 2, 1))
            .backdrop(Target::Backdrop)
            .layout(frame.area());
        let [question, buttons] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(modal.inner());
        let button_slots = [
            (Target::Cancel, Constraint::Length(10)),
            (Target::Confirm, Constraint::Length(12)),
        ];
        let button_layout = ActionSurface::horizontal(button_slots)
            .spacing(2)
            .flex(Flex::Center)
            .focus_start(100)
            .z(101)
            .layout(buttons);
        let child_frame = button_layout.clone().into_frame();
        let next_frame = modal.clone().merge_child(child_frame);
        self.focus
            .ensure_visible(&next_frame.focus, FocusFallback::First);

        frame.render_widget(Clear, modal.outer());
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("modal shell"),
            modal.outer(),
        );
        frame.render_widget(Paragraph::new("Ship this release?").centered(), question);
        for region in button_layout.regions().regions() {
            self.render_button(frame, region.id, region.area);
        }

        next_frame
    }

    fn render_button(&self, frame: &mut Frame, target: Target, area: Rect) {
        let focused = self.focus.focused() == Some(target);
        let style = if focused {
            Style::new()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::new().fg(Color::White).bg(Color::DarkGray)
        };
        frame.render_widget(Paragraph::new(target.label()).centered().style(style), area);
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        if self.open {
            self.handle_modal_key(key);
            return true;
        }
        match key {
            KeyCode::Char('q') | KeyCode::Esc => false,
            KeyCode::Char('o') => {
                self.open_modal();
                true
            }
            _ => true,
        }
    }

    fn handle_modal_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => self.cancel("cancelled with Esc"),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Tab => {
                self.focus.next(&self.previous_frame.focus);
            }
            KeyCode::Char('h') | KeyCode::Left | KeyCode::BackTab => {
                self.focus.previous(&self.previous_frame.focus);
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                if let Some(target) = self.focus.focused() {
                    self.activate(target);
                }
            }
            _ => {}
        }
    }

    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        if !self.open || !matches!(mouse.kind, MouseEventKind::Down(_)) {
            return;
        }
        if let Some(hit) = self.previous_frame.route_click((mouse.column, mouse.row)) {
            self.activate(hit.id);
        }
    }

    fn activate(&mut self, target: Target) {
        match target {
            Target::Backdrop | Target::Cancel => self.cancel("modal cancelled"),
            Target::Confirm => {
                self.open = false;
                self.message = "release confirmed";
                self.focus.clear();
            }
        }
    }

    fn cancel(&mut self, message: &'static str) {
        self.open = false;
        self.message = message;
        self.focus.clear();
    }

    const fn open_modal(&mut self) {
        self.open = true;
        self.message = "modal open; click outside to cancel";
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Target {
    Backdrop,
    Cancel,
    Confirm,
}

impl Target {
    const fn label(self) -> &'static str {
        match self {
            Self::Backdrop => "",
            Self::Cancel => "cancel",
            Self::Confirm => "confirm",
        }
    }
}
