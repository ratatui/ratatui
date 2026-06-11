use super::queue_rows::{QueueColumn, QueueRows};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::frame::{FrameSnapshot, FrameTargets};
use ratatui_layout::pointer::PointerState;
use ratatui_layout::regions::Region;
use ratatui_layout::selection::VisibleSelection;
use ratatui_layout::table::{CellPosition, TableLayout, VirtualTable, VirtualTableState};

use crate::QUEUE_FOCUS;
use crate::domain::{ItemId, QueueView, ReleaseItem};
use crate::ids::{PaneId, TargetId};
use crate::ui::{margin, offset_index, pane_style_for};

/// Stateful controller for the work queue pane.
///
/// The table owns virtual-table state but not release items themselves. That keeps domain data in
/// `ReleaseBoard`, while this region owns only the UI state needed to navigate and render the
/// filtered queue.
///
/// The queue renders borrowed `ReleaseItem` rows through `VirtualTable`. `QueueSelection` keeps the
/// durable item id and active visual column together, because visible row numbers can change when the
/// tree filter changes. Mouse hover and clicks are still routed through `TargetId::QueueCell`, and
/// `VirtualTableState` still receives a `CellPosition` during render so it can keep the selected cell
/// visible.
#[derive(Debug)]
pub(crate) struct WorkQueue {
    /// Scroll and selected-cell state owned by the virtual table.
    state: VirtualTableState,

    /// Durable item selection plus the active visual column.
    selection: QueueSelection,
}

/// Stable queue selection that can be projected into the current visible rows.
///
/// A queue has two useful identities at once: commands need a durable [`ItemId`], while the virtual
/// table needs a visible [`CellPosition`]. This helper keeps that bridge in one place so render,
/// mouse selection, and keyboard movement do not each open-code the same row/id synchronization.
#[derive(Debug, Default)]
struct QueueSelection {
    /// Stable release item selected by the queue.
    visible: VisibleSelection<ItemId, CellPosition>,

    /// Active body column for keyboard movement and focus styling.
    column: usize,
}

impl QueueSelection {
    /// Returns the selected durable item id, if any.
    const fn item(&self) -> Option<ItemId> {
        self.visible.selected_id()
    }

    /// Returns the active visual column.
    const fn column(&self) -> usize {
        self.column
    }

    /// Selects a visible row and column.
    fn select_visible(
        &mut self,
        row: usize,
        column: usize,
        view: &QueueView<'_>,
    ) -> Option<CellPosition> {
        self.column = column.min(QueueColumn::COUNT - 1);
        let position = CellPosition::body(row, self.column);
        self.visible
            .select_position(position, |position| item_id_at(position, view))
    }

    /// Selects a routed body cell if it still exists in the current view.
    fn select_position(
        &mut self,
        position: CellPosition,
        view: &QueueView<'_>,
    ) -> Option<CellPosition> {
        let row = position.row?;
        self.select_visible(row, position.column, view)
    }

    /// Moves within the current visible rows and returns the new body cell.
    fn move_visible(
        &mut self,
        current: CellPosition,
        row_delta: isize,
        column_delta: isize,
        view: &QueueView<'_>,
    ) -> Option<CellPosition> {
        if view.is_empty() {
            self.visible.clear();
            return None;
        }
        let row = offset_index(
            current.row.unwrap_or_default(),
            row_delta,
            view.items().len(),
        );
        let column = offset_index(current.column, column_delta, QueueColumn::COUNT);
        self.select_visible(row, column, view)
    }

    /// Projects durable selection into the currently visible rows.
    ///
    /// Filtering can hide the previously selected item. When that happens, the queue falls back to the
    /// first visible row so commands keep operating on a real item.
    fn sync_to_view(&mut self, view: &QueueView<'_>) -> Option<CellPosition> {
        if view.is_empty() {
            self.visible.clear();
            return None;
        }
        self.column = self.column.min(QueueColumn::COUNT - 1);
        self.visible.sync(
            || visible_item_at(0, self.column, view),
            |id| {
                let row = view.position_of(id)?;
                visible_item_at(row, self.column, view)
            },
        )
    }
}

/// Returns the durable item id at a visible table position.
fn item_id_at(position: CellPosition, view: &QueueView<'_>) -> Option<ItemId> {
    let row = position.row?;
    view.item_at(row).map(|item| item.id)
}

/// Returns a visible table position and durable id for one row and column.
fn visible_item_at(
    row: usize,
    column: usize,
    view: &QueueView<'_>,
) -> Option<(CellPosition, ItemId)> {
    let id = view.item_at(row).map(|item| item.id)?;
    Some((CellPosition::body(row, column), id))
}

#[allow(
    clippy::unused_self,
    reason = "region phase helpers stay as methods so the example reads by component"
)]
impl WorkQueue {
    /// Creates a table ready to select the first visible item on render.
    ///
    /// The component cannot choose a durable item id until `App` provides the filtered rows. The table
    /// state still starts at the first body cell so the first non-empty render has useful focus and
    /// scroll defaults.
    pub(crate) fn new() -> Self {
        let mut state = VirtualTableState::default();
        state.select(Some(CellPosition::body(0, 0)));
        Self {
            state,
            selection: QueueSelection::default(),
        }
    }

    /// Renders the work queue and returns routed data for visible cells.
    ///
    /// This draws the pane shell, renders the virtual table body and header, shows row/column
    /// metrics, and returns the frame-local targets for visible cells. Header cells are mouse
    /// targets, but only body cells are focus targets so keyboard navigation stays row-oriented.
    pub(crate) fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        focused: Option<TargetId>,
        mouse: &PointerState<TargetId>,
        view: &QueueView<'_>,
    ) -> FrameSnapshot<TargetId> {
        self.render_shell(frame, area, focused);

        let table_area = area.inner(margin(1, 1));
        let layout = self.render_cells(frame, table_area, mouse, view.items());
        self.render_metrics(frame, area, &layout);

        self.frame_snapshot(area, table_area, &layout)
    }

    /// Draws the table border and title.
    ///
    /// The border is highlighted when focus is on any queue cell. The component does not own global
    /// focus; it receives the current routed id from `App` and maps that id to a pane style.
    fn render_shell(&self, frame: &mut Frame, area: Rect, focused: Option<TargetId>) {
        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .title("queue")
                .border_style(pane_style_for(focused, PaneId::Queue)),
            area,
        );
    }

    /// Renders visible cells through `VirtualTable`.
    ///
    /// `QueueRows` adapts domain rows into cell text and styles. `VirtualTable` owns scrolling,
    /// pinned header layout, selected-cell visibility, and visible-cell metadata.
    fn render_cells(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        mouse: &PointerState<TargetId>,
        items: &[&ReleaseItem],
    ) -> TableLayout {
        let mut rows = QueueRows {
            items,
            selected: self.state.selected(),
            hovered: self.hovered(mouse),
        };
        let columns = QueueColumn::ALL.map(QueueColumn::constraint);
        let table = VirtualTable::new(columns).scroll_padding(1);
        table.render(area, frame.buffer_mut(), &mut self.state, &mut rows)
    }

    /// Draws the small row/column metric footer in the table pane.
    ///
    /// The metrics come from `TableLayout`, not from persistent app state. That keeps the footer
    /// honest when the terminal size, selected row, or filtered item count changes.
    fn render_metrics(&self, frame: &mut Frame, area: Rect, layout: &TableLayout) {
        let metrics = format!(
            "row {} / {}   selected col {} / {}",
            layout.vertical_metrics().offset + 1,
            layout.row_count,
            self.selection.column() + 1,
            layout.column_count
        );
        let status = Rect::new(
            area.x + 1,
            area.bottom().saturating_sub(1),
            area.width - 2,
            1,
        );
        frame.render_widget(
            Paragraph::new(metrics).style(Style::new().fg(Color::DarkGray)),
            status,
        );
    }

    /// Converts visible table cells into layout, mouse, and focus target collections.
    ///
    /// Every visible cell becomes a layout region and mouse target so clicks can route to the exact
    /// cell under the pointer. Only body cells become focus targets because header cells are useful
    /// for pointer routing but not part of row selection in this example.
    fn frame_snapshot(
        &self,
        area: Rect,
        table_area: Rect,
        layout: &TableLayout,
    ) -> FrameSnapshot<TargetId> {
        let regions = layout
            .visible_cells
            .iter()
            .map(|cell| Region::new(cell.position, cell.area));
        FrameTargets::new(area, QUEUE_FOCUS)
            .mouse_region(TargetId::Pane(PaneId::Queue), table_area)
            .build_with_focus(regions, TargetId::QueueCell, |position| {
                position.row.is_some()
            })
            .clip_to(table_area)
    }

    /// Returns the hovered table cell, if the mouse is over one.
    ///
    /// `PointerState` stores only the routed id from the previous frame. This helper narrows that id
    /// back to a `CellPosition` so `QueueRows` can style the hovered cell during rendering.
    fn hovered(&self, mouse: &PointerState<TargetId>) -> Option<CellPosition> {
        mouse.hovered().and_then(|id| match id {
            TargetId::QueueCell(position) => Some(position),
            _ => None,
        })
    }

    /// Selects a body cell directly.
    ///
    /// Activation uses this after focus or mouse routing has already chosen a body cell. The table
    /// state then keeps that cell visible on the next render.
    pub(crate) fn select(&mut self, position: CellPosition, view: &QueueView<'_>) {
        if let Some(position) = self.selection.select_position(position, view) {
            self.state.select(Some(position));
        }
    }

    /// Moves the selected cell and returns the new position for focus synchronization.
    ///
    /// The queue accepts signed row and column deltas from keyboard input. It clamps movement to the
    /// current filtered row count and known column count, updates `VirtualTableState`, and returns
    /// the new position so `App` can keep `FocusState` pointed at the same routed cell. Empty views
    /// return `None` because there is no rendered body cell to focus.
    pub(crate) fn move_selection(
        &mut self,
        row_delta: isize,
        column_delta: isize,
        view: &QueueView<'_>,
    ) -> Option<CellPosition> {
        self.prepare_for_view(view);
        let current = self.state.selected().unwrap_or(CellPosition::body(0, 0));
        let position = self
            .selection
            .move_visible(current, row_delta, column_delta, view)?;
        self.state.select(Some(position));
        Some(position)
    }

    /// Scrolls the queue vertically without changing the selected item.
    ///
    /// `VirtualTable` clamps the requested scroll offset during the next render. Selection and the
    /// details pane remain attached to the same durable item id.
    pub(crate) const fn scroll_rows(&mut self, delta: isize) {
        self.state.scroll_rows_by(delta);
    }

    /// Returns the selected durable item id, if any.
    ///
    /// App-level command handling can use this directly with `ReleaseBoard::item` or
    /// `ReleaseBoard::item_mut`; it does not need to reverse-map the current visible row.
    pub(crate) const fn selected_item_id(&self) -> Option<ItemId> {
        self.selection.item()
    }

    /// Synchronizes durable item selection with the visible table row used by `VirtualTable`.
    ///
    /// Filtering can remove the previously selected item from the current queue view. When that
    /// happens, the queue selects the first visible item so commands keep operating on a real row.
    /// `App` calls this before rendering so the selection/view reconciliation is an explicit phase
    /// rather than a hidden side effect of drawing the component.
    pub(crate) fn prepare_for_view(&mut self, view: &QueueView<'_>) {
        let Some(position) = self.selection.sync_to_view(view) else {
            self.state.select_without_scrolling(None);
            return;
        };
        let position = Some(position);
        if self.state.scrolls_selected_into_view() {
            self.state.select(position);
        } else {
            self.state.select_without_scrolling(position);
        }
    }

    /// Returns the requested row scroll for focused tests.
    #[cfg(test)]
    pub(crate) const fn row_scroll(&self) -> usize {
        self.state.row_scroll()
    }
}
