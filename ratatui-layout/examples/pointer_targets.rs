//! Route hover, press, release, disabled controls, and overlays through one pointer target
//! collection.
//!
//! Ratatui can draw overlapping rectangles, but it does not remember which app control owned each
//! rectangle after the frame is gone. This example stores `PointerTargets` from the last render
//! pass, so the next mouse event can route through z-order and disabled-state rules.

use std::io;

use color_eyre::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, MouseEventKind};
use crossterm::execute;
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::pointer::{PointerPhase, PointerState, PointerTarget, PointerTargets};

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
    enabled: bool,
    mouse: PointerState<Target>,
    previous_mouse: PointerTargets<Target>,
    message: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            enabled: true,
            mouse: PointerState::default(),
            previous_mouse: PointerTargets::new(),
            message: String::from("move or click; d toggles the disabled target"),
        }
    }
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let area = centered(frame.area(), 54, 9);
        let [buttons, status] =
            Layout::vertical([Constraint::Length(5), Constraint::Length(1)]).areas(area);
        let button_areas = Layout::horizontal([
            Constraint::Length(16),
            Constraint::Length(16),
            Constraint::Length(16),
        ])
        .spacing(1)
        .flex(Flex::Center)
        .split(buttons);

        self.previous_mouse = PointerTargets::from_targets([
            PointerTarget::new(Target::Open, button_areas[0]),
            PointerTarget::new(Target::Save, button_areas[1]).disabled(!self.enabled),
            PointerTarget::new(Target::Overlay, overlay_area(buttons)).z(10),
            PointerTarget::new(Target::Close, button_areas[2]),
        ]);

        frame.render_widget(
            Block::new().borders(Borders::ALL).title("mouse targets"),
            area,
        );
        self.render_button(frame, Target::Open, button_areas[0]);
        self.render_button(frame, Target::Save, button_areas[1]);
        self.render_button(frame, Target::Close, button_areas[2]);
        frame.render_widget(
            Paragraph::new("z=10 overlay").centered().style(
                Style::new()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            overlay_area(buttons),
        );
        frame.render_widget(Paragraph::new(self.message.as_str()), status);
    }

    const fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Char('d') => self.enabled = !self.enabled,
            _ => {}
        }
        true
    }

    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        let position = (mouse.column, mouse.row);
        let Some(phase) = pointer_phase(mouse.kind) else {
            return;
        };
        let hit = self.mouse.route(&self.previous_mouse, position, phase);
        match (phase, hit) {
            (PointerPhase::Hover, Some(hit)) => {
                self.message = format!(
                    "hover {:?} at local {},{}",
                    hit.id, hit.relative_x, hit.relative_y
                );
            }
            (PointerPhase::Press, Some(hit)) => {
                self.message = format!("pressed {:?}", hit.id);
            }
            (PointerPhase::Release, Some(hit)) => {
                self.message = format!(
                    "clicked {:?} at local {},{}",
                    hit.id, hit.relative_x, hit.relative_y
                );
            }
            (PointerPhase::Release, None) => {
                self.message = String::from("release did not match the pressed target");
            }
            (PointerPhase::Hover | PointerPhase::Press, None) => {}
        }
    }

    fn render_button(&self, frame: &mut Frame, target: Target, area: Rect) {
        let disabled = target == Target::Save && !self.enabled;
        let style = if disabled {
            Style::new().fg(Color::DarkGray)
        } else if self.mouse.hovered() == Some(target) {
            Style::new().fg(Color::Black).bg(Color::Cyan)
        } else {
            Style::new().fg(Color::White)
        };
        frame.render_widget(
            Paragraph::new(target.label())
                .centered()
                .style(style)
                .block(Block::new().borders(Borders::ALL).title(if disabled {
                    "disabled"
                } else {
                    ""
                })),
            area,
        );
    }
}

const fn pointer_phase(kind: MouseEventKind) -> Option<PointerPhase> {
    match kind {
        MouseEventKind::Moved => Some(PointerPhase::Hover),
        MouseEventKind::Down(_) => Some(PointerPhase::Press),
        MouseEventKind::Up(_) => Some(PointerPhase::Release),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Target {
    Open,
    Save,
    Close,
    Overlay,
}

impl Target {
    const fn label(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Save => "save",
            Self::Close => "close",
            Self::Overlay => "overlay",
        }
    }
}

const fn overlay_area(area: Rect) -> Rect {
    Rect::new(area.x + 20, area.y + 1, 14, 3)
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
