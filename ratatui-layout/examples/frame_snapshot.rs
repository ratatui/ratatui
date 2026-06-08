//! Store one frame's UI data and route the next input event through them.
//!
//! This example keeps the previous `FrameSnapshot` on `App`. Rendering rebuilds layout and pointer
//! data; key and mouse events then use the stored plan instead of reconstructing geometry in
//! handlers.

use color_eyre::Result;
use crossterm::event::{self, KeyCode, MouseEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::{
    FocusFallback, FocusState, FocusTargets, FrameSnapshot, PointerTarget, PointerTargets, Row,
    SelectionMode, SelectionState,
};

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
    previous_frame: FrameSnapshot<Action>,
    focus: FocusState<Action>,
    selection: SelectionState<Action>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            previous_frame: FrameSnapshot::new(Rect::default()),
            focus: FocusState::default(),
            selection: SelectionState::new(SelectionMode::Single),
        }
    }
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let area = centered(frame.area(), 48, 5);
        let action_widths = [
            Constraint::Length(14),
            Constraint::Length(14),
            Constraint::Length(14),
        ];
        let plan = Row::new(action_widths)
            .spacing(1)
            .flex(Flex::Center)
            .regions(area)
            .map_id(Action::from_index);
        let focus = FocusTargets::from_regions(plan.regions().iter().copied());
        let mouse = PointerTargets::from_targets(
            plan.regions()
                .iter()
                .map(|region| PointerTarget::from_region(*region))
                .collect::<Vec<_>>(),
        );

        self.focus.ensure_visible(&focus, FocusFallback::First);
        if self.selection.primary().is_none() {
            self.selection.select(Action::Open);
        }

        frame.render_widget(
            Block::new().borders(Borders::ALL).title("frame snapshot"),
            area,
        );
        for region in plan.regions() {
            self.render_action(frame, region.id, region.area);
        }

        self.previous_frame = FrameSnapshot::from_layout(plan).focus(focus).mouse(mouse);
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => return false,
            KeyCode::Right | KeyCode::Tab => self.focus.next(&self.previous_frame.focus),
            KeyCode::Left | KeyCode::BackTab => self.focus.previous(&self.previous_frame.focus),
            KeyCode::Enter | KeyCode::Char(' ') => {
                if let Some(action) = self.focus.focused() {
                    self.selection.select(action);
                }
            }
            _ => {}
        }
        true
    }

    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        if matches!(mouse.kind, MouseEventKind::Down(_) | MouseEventKind::Up(_))
            && let Some(hit) = self
                .previous_frame
                .route_position((mouse.column, mouse.row))
        {
            self.focus.focus(Some(hit.id));
            self.selection.select(hit.id);
        }
    }

    fn render_action(&self, frame: &mut Frame, action: Action, area: Rect) {
        let mut style = Style::new();
        if self.selection.is_selected(action) {
            style = style.add_modifier(Modifier::REVERSED);
        } else if self.focus.focused() == Some(action) {
            style = style.add_modifier(Modifier::BOLD);
        } else {
            style = style.remove_modifier(Modifier::REVERSED);
        }
        frame.render_widget(
            Paragraph::new(action.label())
                .block(Block::new().borders(Borders::ALL))
                .centered()
                .style(style),
            area,
        );
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Action {
    Open,
    Save,
    Close,
}

impl Action {
    const fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Open,
            1 => Self::Save,
            _ => Self::Close,
        }
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
