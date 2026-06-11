//! Durable-id adapter for virtual lists.
//!
//! [`crate::list::VirtualList`] works in source indexes because measurement and rendering need to
//! ask an app-owned collection for item `n`. Real apps often want selection and input routing to
//! use stable record ids instead, especially when rows can be filtered, sorted, inserted, or
//! removed. [`VirtualRecordList`](crate::record_list::VirtualRecordList) bridges those two
//! identities.
//!
//! # Types
//!
//! - [`VirtualRecordListState`](crate::record_list::VirtualRecordListState) stores index-based list
//!   state plus durable-id selection.
//! - [`VirtualRecordList`](crate::record_list::VirtualRecordList) computes or renders a list and
//!   produces durable-id row regions.
//! - [`VirtualRecordListLayout`](crate::record_list::VirtualRecordListLayout) exposes the
//!   underlying list layout and durable row regions.
//!
//! # Examples
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::record_list::VirtualRecordListState;
//!
//! let ids = ["api", "worker", "docs"];
//! let mut state = VirtualRecordListState::new();
//! state.select_id("worker", &ids);
//! state.move_selection_by(1, &ids);
//!
//! assert_eq!(state.selected_id(), Some("docs"));
//! assert_eq!(state.list_state().selected(), Some(2));
//! ```

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Position, Rect};

use crate::list::{ListItems, ListLayout, VirtualList, VirtualListState};
use crate::regions::{Hit, Regions};
use crate::selection::VisibleSelection;

/// Persistent state for a [`VirtualRecordList`](crate::record_list::VirtualRecordList).
///
/// The state keeps two related views of selection:
///
/// - [`VirtualListState`] stores the source index needed by measurement and rendering.
/// - [`VisibleSelection`](crate::selection::VisibleSelection) stores the durable record id used by
///   application commands.
///
/// Keep this state beside the app collection and pass the current ordered id slice on each input or
/// render pass. The adapter repairs stale ids and indexes against that slice.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct VirtualRecordListState<Id = usize> {
    list: VirtualListState,
    selection: VisibleSelection<Id>,
}

impl<Id> Default for VirtualRecordListState<Id> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Id> VirtualRecordListState<Id> {
    /// Creates empty list and durable-selection state.
    pub fn new() -> Self {
        Self {
            list: VirtualListState::default(),
            selection: VisibleSelection::new(),
        }
    }

    /// Returns the underlying index-based list state.
    pub const fn list_state(&self) -> &VirtualListState {
        &self.list
    }

    /// Returns mutable access to the underlying index-based list state.
    ///
    /// Use this for advanced scroll control that is still index-based. Prefer the durable-id
    /// methods on this type for selection.
    pub const fn list_state_mut(&mut self) -> &mut VirtualListState {
        &mut self.list
    }

    /// Returns the durable visible selection state.
    pub const fn selection(&self) -> &VisibleSelection<Id> {
        &self.selection
    }

    /// Returns the selected durable record id.
    pub const fn selected_id(&self) -> Option<Id>
    where
        Id: Copy,
    {
        self.selection.selected_id()
    }

    /// Selects a durable id and syncs the backing list index.
    ///
    /// Use this after pointer routing, command activation, or restoring selection from app state.
    pub fn select_id(&mut self, id: Id, visible_ids: &[Id]) -> bool
    where
        Id: Copy + Eq,
    {
        let Some(index) = visible_ids.iter().position(|visible| *visible == id) else {
            return false;
        };
        self.selection.select_visible(index, id);
        self.list.select(Some(index));
        true
    }

    /// Moves durable selection by a signed row delta.
    ///
    /// Keyboard row movement should normally call this rather than moving the raw list index so
    /// app commands continue to work with durable ids.
    pub fn move_selection_by(&mut self, delta: isize, visible_ids: &[Id]) -> Option<Id>
    where
        Id: Copy + Eq,
    {
        self.selection.move_by(delta, visible_ids);
        self.list.select(self.selection.position());
        self.selection.selected_id()
    }

    /// Scrolls the viewport without changing durable selection.
    ///
    /// This is the mouse-wheel path: move what is visible, leave the selected record alone, and let
    /// the next layout decide whether that record is still visible.
    pub const fn scroll_viewport_by(&mut self, delta: isize) {
        self.list.scroll_viewport_by(delta);
    }

    /// Repairs durable and index selection against the current ordered ids.
    ///
    /// Call this after filtering, sorting, or replacing the collection. The previous durable id is
    /// kept when possible; otherwise selection moves to the first visible id.
    pub fn sync_ids(&mut self, visible_ids: &[Id])
    where
        Id: Copy + Eq,
    {
        self.selection.sync_ids(visible_ids);
        self.list
            .select_without_scrolling(self.selection.position());
    }

    fn prepare_for_layout(&mut self, visible_ids: &[Id])
    where
        Id: Copy + Eq,
    {
        self.sync_ids(visible_ids);
        if self.list.scrolls_selected_into_view() {
            self.list.select(self.selection.position());
        }
    }
}

/// Durable-id wrapper around [`VirtualList`].
///
/// The wrapper keeps the existing list measurement and rendering model, but returns row regions
/// keyed by stable ids instead of source indexes. The app must pass an `ids` slice in the same
/// order as the [`ListItems`] source.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct VirtualRecordList {
    list: VirtualList,
}

impl VirtualRecordList {
    /// Creates a durable-id virtual list adapter.
    pub const fn new() -> Self {
        Self {
            list: VirtualList::new(),
        }
    }

    /// Computes list layout and durable-id row regions without rendering.
    ///
    /// Use this when another widget owns row rendering but the app still needs virtualized
    /// selection and pointer routing.
    pub fn layout<Id, Items>(
        self,
        area: Rect,
        state: &mut VirtualRecordListState<Id>,
        ids: &[Id],
        items: &Items,
    ) -> VirtualRecordListLayout<Id>
    where
        Id: Copy + Eq,
        Items: ListItems,
    {
        state.prepare_for_layout(ids);
        let layout = self.list.layout(area, &mut state.list, items);
        let regions = durable_regions(&layout, ids);
        VirtualRecordListLayout { layout, regions }
    }

    /// Renders visible rows and returns durable-id row regions.
    ///
    /// This delegates measurement and row rendering to [`VirtualList`]. The durable adapter only
    /// keeps selection ids and returned row regions aligned with the app's ids.
    pub fn render<Id, Items>(
        self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut VirtualRecordListState<Id>,
        ids: &[Id],
        items: &mut Items,
    ) -> VirtualRecordListLayout<Id>
    where
        Id: Copy + Eq,
        Items: ListItems,
    {
        state.prepare_for_layout(ids);
        let layout = self.list.render(area, buf, &mut state.list, items);
        let regions = durable_regions(&layout, ids);
        VirtualRecordListLayout { layout, regions }
    }
}

/// Solved output for a [`VirtualRecordList`](crate::record_list::VirtualRecordList) pass.
///
/// The underlying [`ListLayout`] remains available for scroll metrics and visible-row metadata.
/// [`VirtualRecordListLayout::regions`](crate::record_list::VirtualRecordListLayout::regions) gives
/// pointer and hit-test code durable record ids.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct VirtualRecordListLayout<Id = usize> {
    layout: ListLayout,
    regions: Regions<Id>,
}

impl<Id> VirtualRecordListLayout<Id> {
    /// Returns the underlying index-based virtual-list layout.
    pub const fn layout(&self) -> &ListLayout {
        &self.layout
    }

    /// Returns row regions keyed by durable ids.
    pub const fn regions(&self) -> &Regions<Id> {
        &self.regions
    }

    /// Hit tests a position against durable row regions.
    pub fn hit_test(&self, position: impl Into<Position>) -> Option<Hit<Id>>
    where
        Id: Copy,
    {
        self.regions.hit_test(position)
    }

    /// Selects the row at a pointer position.
    ///
    /// This updates both durable-id selection and the backing list index when the position hits a
    /// visible row.
    pub fn select_hit(
        &self,
        position: impl Into<Position>,
        state: &mut VirtualRecordListState<Id>,
        visible_ids: &[Id],
    ) -> Option<Id>
    where
        Id: Copy + Eq,
    {
        let hit = self.hit_test(position)?;
        state.select_id(hit.id, visible_ids);
        Some(hit.id)
    }
}

fn durable_regions<Id>(layout: &ListLayout, ids: &[Id]) -> Regions<Id>
where
    Id: Copy,
{
    layout.rows_regions(|index| ids[index])
}

#[cfg(test)]
mod tests {
    use ratatui_core::buffer::Buffer;
    use ratatui_core::layout::Rect;

    use super::{VirtualRecordList, VirtualRecordListState};
    use crate::list::{ListItemContext, ListItems};
    use crate::participant::MeasureContext;

    struct Rows(usize);

    impl ListItems for Rows {
        fn len(&self) -> usize {
            self.0
        }

        fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
            1
        }

        fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
    }

    #[test]
    fn keeps_durable_selection_across_reordered_ids() {
        let ids = ["api", "worker", "docs"];
        let mut state = VirtualRecordListState::new();

        state.select_id("worker", &ids);
        state.sync_ids(&["docs", "worker", "api"]);

        assert_eq!(state.selected_id(), Some("worker"));
        assert_eq!(state.list_state().selected(), Some(1));
    }

    #[test]
    fn wheel_scroll_does_not_change_durable_selection() {
        let ids = ["api", "worker", "docs"];
        let mut state = VirtualRecordListState::new();
        state.select_id("worker", &ids);

        state.scroll_viewport_by(1);

        assert_eq!(state.selected_id(), Some("worker"));
    }

    #[test]
    fn hit_testing_uses_durable_row_ids() {
        let ids = ["api", "worker", "docs"];
        let mut rows = Rows(ids.len());
        let mut state = VirtualRecordListState::new();
        let layout =
            VirtualRecordList::new().layout(Rect::new(0, 0, 20, 3), &mut state, &ids, &rows);

        assert_eq!(layout.hit_test((0, 1)).unwrap().id, "worker");
        assert_eq!(layout.select_hit((0, 2), &mut state, &ids), Some("docs"));
        assert_eq!(state.selected_id(), Some("docs"));

        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 3));
        VirtualRecordList::new().render(buffer.area, &mut buffer, &mut state, &ids, &mut rows);
    }
}
