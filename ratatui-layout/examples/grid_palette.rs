//! Route palette input through typed grid cells.
//!
//! This example uses `GridLayout` for `{ row, column }` cell ids, `PointerTargets` for pointer
//! hits, `FocusTargets` for keyboard traversal, and `SelectionState` for app-owned selection.

use color_eyre::Result;
use crossterm::event::{self, KeyCode, MouseEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::{
    FocusFallback, FocusState, FocusTargets, Grid, GridPosition, PointerPhase, PointerState,
    PointerTarget, PointerTargets, SelectionMode, SelectionState,
};

fn main() -> Result<()> {
    color_eyre::install()?;

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
    focus: FocusState<GridPosition>,
    mouse: PointerState<GridPosition>,
    selection: SelectionState<GridPosition>,
    previous_mouse_plan: PointerTargets<GridPosition>,
    previous_focus_plan: FocusTargets<GridPosition>,
}

impl App {
    fn new() -> Self {
        Self {
            focus: FocusState::default(),
            mouse: PointerState::default(),
            selection: SelectionState::new(SelectionMode::Single),
            previous_mouse_plan: PointerTargets::new(),
            previous_focus_plan: FocusTargets::new(),
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = centered(frame.area(), 42, 9);
        let row_heights = [Constraint::Length(3), Constraint::Length(3)];
        let column_widths = [
            Constraint::Length(14),
            Constraint::Length(14),
            Constraint::Length(14),
        ];
        let grid = Grid::new(row_heights, column_widths).layout(area);
        let focus_plan = FocusTargets::from_regions(grid.cells().regions().iter().copied());
        let mouse_plan = PointerTargets::from_targets(
            grid.cells()
                .regions()
                .iter()
                .map(|region| PointerTarget::from_region(*region))
                .collect::<Vec<_>>(),
        );
        let visible_ids = grid
            .cells()
            .regions()
            .iter()
            .map(|region| region.id)
            .collect::<Vec<_>>();

        self.focus.ensure_visible(&focus_plan, FocusFallback::First);
        if self.selection.primary().is_none() {
            self.selection.select(visible_ids[0]);
        }

        frame.render_widget(Block::new().borders(Borders::ALL).title("palette"), area);
        for region in grid.cells().regions() {
            self.render_cell(frame, region.id, region.area);
        }

        self.previous_focus_plan = focus_plan;
        self.previous_mouse_plan = mouse_plan;
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => return false,
            KeyCode::Right | KeyCode::Tab => self.focus.next(&self.previous_focus_plan),
            KeyCode::Left | KeyCode::BackTab => self.focus.previous(&self.previous_focus_plan),
            KeyCode::Enter | KeyCode::Char(' ') => {
                if let Some(id) = self.focus.focused() {
                    self.selection.select(id);
                }
            }
            _ => {}
        }
        true
    }

    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        let position = (mouse.column, mouse.row);
        let Some(phase) = pointer_phase(mouse.kind) else {
            return;
        };
        let hit = self.mouse.route(&self.previous_mouse_plan, position, phase);
        match (phase, hit) {
            (PointerPhase::Press, Some(hit)) => {
                self.focus.focus(Some(hit.id));
            }
            (PointerPhase::Release, Some(hit)) => {
                self.selection.select(hit.id);
            }
            _ => {}
        }
    }

    fn render_cell(&self, frame: &mut Frame, id: GridPosition, area: Rect) {
        let color = PALETTE[id.row * 3 + id.column];
        let mut style = Style::new().fg(color.color);
        if self.selection.is_selected(id) {
            style = style.bg(Color::White).fg(Color::Black);
        } else if self.focus.focused() == Some(id) || self.mouse.hovered() == Some(id) {
            style = style.add_modifier(Modifier::REVERSED);
        } else {
            style = style.bg(Color::Reset);
        }
        frame.render_widget(
            Paragraph::new(format!("  {}", color.name))
                .block(Block::new().borders(Borders::ALL))
                .style(style),
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

#[derive(Debug, Clone, Copy)]
struct Swatch {
    name: &'static str,
    color: Color,
}

const PALETTE: [Swatch; 6] = [
    Swatch {
        name: "red",
        color: Color::Red,
    },
    Swatch {
        name: "green",
        color: Color::Green,
    },
    Swatch {
        name: "yellow",
        color: Color::Yellow,
    },
    Swatch {
        name: "blue",
        color: Color::Blue,
    },
    Swatch {
        name: "magenta",
        color: Color::Magenta,
    },
    Swatch {
        name: "cyan",
        color: Color::Cyan,
    },
];

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    Rect::new(
        area.x + area.width.saturating_sub(width) / 2,
        area.y + area.height.saturating_sub(height) / 2,
        width.min(area.width),
        height.min(area.height),
    )
}
