//! Coordinate a modal prompt without turning it into a form framework.
//!
//! The page behind the prompt still renders, but input is routed through the prompt's previous
//! `FrameSnapshot`. The prompt contributes only shell geometry, button targets, focus order,
//! z-order, and an explicit backdrop target for outside clicks.

use std::io;

use color_eyre::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind};
use crossterm::execute;
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui_layout::{
    Container, FocusFallback, FocusState, FrameSnapshot, FrameTargets, Padding, Row,
};

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
    prompt_open: bool,
    deployment_cancelled: bool,
    message: String,
    focus: FocusState<PromptTarget>,
    previous_prompt: FrameSnapshot<PromptTarget>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            prompt_open: true,
            deployment_cancelled: false,
            message: String::from("prompt is open; click outside to dismiss"),
            focus: FocusState::default(),
            previous_prompt: FrameSnapshot::new(Rect::default()),
        }
    }
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        self.render_release_page(frame);
        self.previous_prompt = FrameSnapshot::new(frame.area());

        if self.prompt_open {
            self.previous_prompt = self.render_prompt(frame);
        }
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        if self.prompt_open {
            self.handle_prompt_key(key);
            return true;
        }

        match key {
            KeyCode::Char('q') | KeyCode::Esc => false,
            KeyCode::Char('o') => {
                self.open_prompt();
                true
            }
            _ => true,
        }
    }

    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        if !self.prompt_open || !matches!(mouse.kind, MouseEventKind::Down(_)) {
            return;
        }

        let position = (mouse.column, mouse.row);
        let Some(hit) = self.previous_prompt.route_click(position) else {
            self.message = String::from("negative hit: prompt ignored page target");
            return;
        };
        self.activate(hit.id);
    }

    fn handle_prompt_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('n') => self.dismiss_prompt("cancelled by keyboard"),
            KeyCode::Tab | KeyCode::Right => self.focus.next(&self.previous_prompt.focus),
            KeyCode::BackTab | KeyCode::Left => self.focus.previous(&self.previous_prompt.focus),
            KeyCode::Enter | KeyCode::Char(' ') => {
                if let Some(target) = self.focus.focused() {
                    self.activate(target);
                }
            }
            _ => {}
        }
    }

    fn activate(&mut self, target: PromptTarget) {
        match target {
            PromptTarget::Backdrop => self.dismiss_prompt("dismissed by backdrop click"),
            PromptTarget::Button(PromptButton::Cancel) => {
                self.dismiss_prompt("cancelled; deployment is still running");
            }
            PromptTarget::Button(PromptButton::Confirm) => {
                self.prompt_open = false;
                self.deployment_cancelled = true;
                self.message = String::from("deployment cancelled");
            }
        }
    }

    fn open_prompt(&mut self) {
        self.prompt_open = true;
        self.message = String::from("prompt is open; click outside to dismiss");
        self.focus.clear();
    }

    fn dismiss_prompt(&mut self, message: &'static str) {
        self.prompt_open = false;
        self.message = String::from(message);
        self.focus.clear();
    }

    fn render_release_page(&self, frame: &mut Frame) {
        let [summary_area, status_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());
        let status = if self.deployment_cancelled {
            "cancelled"
        } else {
            "running"
        };
        let summary = format!(
            "release train\n\nstate: {status}\nowner: release\n\n\
             o opens prompt   q quits after prompt closes"
        );

        frame.render_widget(
            Paragraph::new(summary).block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("page behind modal"),
            ),
            summary_area,
        );
        frame.render_widget(Paragraph::new(self.message.as_str()), status_area);
    }

    fn render_prompt(&mut self, frame: &mut Frame) -> FrameSnapshot<PromptTarget> {
        let outer = centered(frame.area(), 44, 9);
        let container = Container::<PromptTarget>::new()
            .padding(Padding::new(2, 2, 2, 1))
            .layout(outer);
        let [question_area, button_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(container.inner);
        let button_constraints = [
            (PromptButton::Cancel, Constraint::Length(10)),
            (PromptButton::Confirm, Constraint::Length(14)),
        ];
        let buttons = Row::named(button_constraints)
            .spacing(2)
            .flex(Flex::Center)
            .regions(button_area);
        let prompt_frame = FrameTargets::new(frame.area(), 100)
            .z(20)
            .mouse_region(PromptTarget::Backdrop, frame.area())
            .build_focusable(buttons.regions().iter().copied(), PromptTarget::Button);
        self.focus
            .ensure_visible(&prompt_frame.focus, FocusFallback::First);

        frame.render_widget(Clear, container.outer);
        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .title("cancel deployment"),
            container.outer,
        );
        frame.render_widget(
            Paragraph::new("Cancel the deployment now?").centered(),
            question_area,
        );
        for region in buttons.regions() {
            self.render_button(frame, region.id, region.area);
        }

        prompt_frame
    }

    fn render_button(&self, frame: &mut Frame, button: PromptButton, area: Rect) {
        let target = PromptTarget::Button(button);
        let focused = self.focus.focused() == Some(target);
        let style = if focused {
            Style::new()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::new().fg(Color::White).bg(Color::DarkGray)
        };
        frame.render_widget(
            Paragraph::new(button.label())
                .centered()
                .style(style)
                .block(Block::new().borders(Borders::ALL)),
            area,
        );
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum PromptTarget {
    Backdrop,
    Button(PromptButton),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum PromptButton {
    Cancel,
    Confirm,
}

impl PromptButton {
    const fn label(self) -> &'static str {
        match self {
            Self::Cancel => "cancel",
            Self::Confirm => "confirm",
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
