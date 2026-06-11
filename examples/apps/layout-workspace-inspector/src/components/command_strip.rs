use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::frame::{FrameSnapshot, FrameTargets};
use ratatui_layout::grid::{Grid, GridLayout, GridPosition};
use ratatui_layout::pointer::PointerState;
use ratatui_layout::regions::{Region, Regions};
use ratatui_layout::selection::{SelectionMode, SelectionState};

use crate::COMMAND_FOCUS;
use crate::ids::{CommandId, TargetId};
use crate::ui::margin;

/// Stateful controller for the footer command strip.
///
/// Commands are a small toolbar, but they still need the same coordination as larger widgets:
/// disabled state, hover styling, focus traversal, mouse routing, and activation by stable ids.
/// Keeping that logic here leaves `App` responsible for command effects rather than command
/// presentation.
#[derive(Debug)]
pub(crate) struct CommandStrip {
    /// Last activated command, used as a persistent visual cue.
    selection: SelectionState<CommandId>,
}

impl CommandStrip {
    /// Creates a command strip with single-command selection.
    pub(crate) const fn new() -> Self {
        Self {
            selection: SelectionState::new(SelectionMode::Single),
        }
    }

    /// Renders all commands and returns focus and mouse targets for enabled commands.
    ///
    /// Commands remain visible even when disabled. Disabled targets are recorded as disabled in the
    /// frame snapshot so mouse hit testing and focus traversal skip them without changing the visual
    /// grid.
    pub(crate) fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        focused: Option<TargetId>,
        mouse: &PointerState<TargetId>,
        selected_item_exists: bool,
    ) -> FrameSnapshot<TargetId> {
        Self::render_shell(frame, area);

        let grid = Self::layout(area);
        self.render_commands(frame, &grid, focused, mouse, selected_item_exists);
        Self::frame_snapshot(&grid, selected_item_exists)
    }

    /// Records a command as the last activated footer command.
    pub(crate) fn select(&mut self, command: CommandId) {
        self.selection.select(command);
    }

    /// Draws the footer shell.
    fn render_shell(frame: &mut Frame, area: Rect) {
        frame.render_widget(Block::new().borders(Borders::TOP).title("commands"), area);
    }

    /// Plans one row of typed command cells.
    fn layout(area: Rect) -> GridLayout {
        let rows = [Constraint::Length(1)];
        let columns = CommandId::ALL.map(|command| Constraint::Length(command.width()));
        Grid::new(rows, columns).layout(area.inner(margin(1, 1)))
    }

    /// Renders command labels into the planned grid cells.
    fn render_commands(
        &self,
        frame: &mut Frame,
        grid: &GridLayout,
        focused: Option<TargetId>,
        mouse: &PointerState<TargetId>,
        selected_item_exists: bool,
    ) {
        for (region, command) in Self::command_slots(grid) {
            let enabled = command.enabled(selected_item_exists);
            let style = self.command_style(command, enabled, focused, mouse);
            frame.render_widget(
                Paragraph::new(command.label()).centered().style(style),
                region.area,
            );
        }
    }

    /// Builds layout, focus, and pointer data from the command grid.
    fn frame_snapshot(grid: &GridLayout, selected_item_exists: bool) -> FrameSnapshot<TargetId> {
        let regions =
            Self::command_slots(grid).map(|(region, command)| Region::new(command, region.area));
        let plan = Regions::from_regions(grid.area(), regions.collect::<Vec<_>>());

        FrameTargets::from_regions(plan, COMMAND_FOCUS)
            .disabled(|command| !command.enabled(selected_item_exists))
            .build()
            .map_id(TargetId::Command)
    }

    /// Returns planned command regions paired with the command they render and route.
    fn command_slots(
        grid: &GridLayout,
    ) -> impl Iterator<Item = (&Region<GridPosition>, CommandId)> {
        grid.cells().zip_ids(&CommandId::ALL)
    }

    /// Chooses command styling from enabled, hover, focus, and last-activated state.
    fn command_style(
        &self,
        command: CommandId,
        enabled: bool,
        focused: Option<TargetId>,
        mouse: &PointerState<TargetId>,
    ) -> Style {
        if !enabled {
            Style::new().fg(Color::DarkGray)
        } else if mouse.hovered() == Some(TargetId::Command(command))
            || focused == Some(TargetId::Command(command))
        {
            Style::new()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else if self.selection.is_selected(command) {
            Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::new().fg(Color::White)
        }
    }
}
