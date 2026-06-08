//! App-owned selection state over visible ids.
//!
//! Selection is often durable application state: a selected row may stay selected while it scrolls
//! out of view, and a selected command may matter independently from hover or keyboard focus.
//! [`SelectionState`] is therefore deliberately not a widget state type. It has no geometry and
//! does not know how rows, cells, or buttons are rendered. Pair it with a frame-local ordered list
//! of visible ids from a layout, grid, list, or table when keyboard or pointer input changes
//! selection.
//!
//! The shared idea in this module is that selection is about application meaning, not terminal
//! coordinates. A [`crate::Regions`], [`crate::PointerTargets`], [`crate::FocusTargets`], grid,
//! list, or table can tell the app which ids are visible this frame. [`SelectionState`] decides
//! which of those ids are selected and keeps that decision after the frame is gone.
//!
//! Use existing Ratatui widget state when that widget fully owns the list or table selection
//! behavior. Use this module when selection belongs to application data and must coordinate with
//! custom layout, focus, pointer routing, or virtualization.
//!
//! That split is useful for common UI workflows:
//!
//! - a list can render the selected row differently while using [`SelectionState::select_next`] and
//!   [`SelectionState::select_previous`] to move through only currently visible ids;
//! - a pointer click can route through [`crate::PointerTargets`] and then call
//!   [`SelectionState::select`] or [`SelectionState::toggle`] with the hit id;
//! - a palette can use [`crate::FocusState`] for keyboard focus and [`SelectionState`] for the
//!   chosen item, avoiding a single overloaded state value;
//! - a multi-select table can preserve selected ids even when filtering or scrolling temporarily
//!   hides them.
//!
//! # Types
//!
//! - [`SelectionMode`] describes whether selection is disabled, single-choice, or multi-choice.
//! - [`SelectionState`] stores selected ids in insertion order and applies the mode rules.
//! - [`VisibleSelection`] bridges one durable selected id to the visible position used by a
//!   virtualized view.
//!
//! See [`crate::docs::interaction`] for how selection differs from focus and hover, and why it
//! remains app-owned rather than part of a region set.
//!
//! # Examples
//!
//! Single selection replaces the previous id:
//!
//! ```rust
//! use ratatui_layout::{SelectionMode, SelectionState};
//!
//! let mut selection = SelectionState::new(SelectionMode::Single);
//! selection.select("open");
//! selection.select("save");
//!
//! assert_eq!(selection.selected(), &["save"]);
//! ```
//!
//! Multi-selection keeps insertion order and supports toggling:
//!
//! ```rust
//! use ratatui_layout::{SelectionMode, SelectionState};
//!
//! let mut selection = SelectionState::new(SelectionMode::Multi);
//! selection.toggle("alpha");
//! selection.toggle("beta");
//! selection.toggle("alpha");
//!
//! assert_eq!(selection.selected(), &["beta"]);
//! ```

use alloc::vec::Vec;

/// Selection behavior for a collection of app-owned ids.
///
/// The mode explains how [`SelectionState`] should interpret [`SelectionState::select`] and
/// [`SelectionState::toggle`]. It does not decide which ids are visible or enabled; callers pass
/// the ordered visible ids used for traversal.
///
/// Use [`SelectionMode::None`] when a component should keep the same input code but temporarily
/// avoid retaining a selection, [`SelectionMode::Single`] for menus and palettes where one choice
/// replaces another, and [`SelectionMode::Multi`] for checklist or table workflows where several
/// ids can be active at once.
///
/// # Examples
///
/// Pick the mode from the interaction model, then let [`SelectionState`] enforce it:
///
/// ```rust
/// use ratatui_layout::{SelectionMode, SelectionState};
///
/// let mut menu = SelectionState::new(SelectionMode::Single);
/// menu.select("open");
/// menu.select("save");
/// assert_eq!(menu.selected(), &["save"]);
///
/// let mut checklist = SelectionState::new(SelectionMode::Multi);
/// checklist.toggle("fast");
/// checklist.toggle("verbose");
/// assert_eq!(checklist.selected(), &["fast", "verbose"]);
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SelectionMode {
    /// Selection is disabled.
    ///
    /// Use this when a component wants to share traversal code with selectable views but should not
    /// currently retain a selected id.
    #[default]
    None,
    /// At most one id is selected.
    ///
    /// This fits menus, radio-like palettes, selected table cells, and other views where the
    /// current choice replaces the previous choice.
    Single,
    /// Multiple ids can be selected in insertion order.
    ///
    /// This fits checklists and multi-select tables. Insertion order is preserved so callers can
    /// display or process selections predictably.
    Multi,
}

/// Persistent selection for app-owned ids.
///
/// [`SelectionState`] owns only the selected ids and the [`SelectionMode`]. It does not own layout
/// rectangles, item data, focus, or hover state. Use the current frame's visible ids to move the
/// selection with [`select_next`](Self::select_next) or [`select_previous`](Self::select_previous).
///
/// The id type should match the ids emitted by the frame-local data that drive the view, such as
/// [`crate::GridPosition`] for a grid palette or [`crate::table::CellPosition`] for a virtual
/// table.
///
/// # Constructors and configuration
///
/// - [`SelectionState::new`] creates empty state in a chosen [`SelectionMode`].
/// - [`SelectionState::mode`] returns the current behavior.
/// - [`SelectionState::set_mode`] changes behavior and drops selections that no longer fit.
///
/// # Reading and clearing selection
///
/// - [`SelectionState::selected`] returns all selected ids in insertion order.
/// - [`SelectionState::primary`] returns the representative selected id for rendering status text
///   or moving from the current item.
/// - [`SelectionState::is_selected`] checks one id while rendering rows, cells, or commands.
/// - [`SelectionState::clear`] removes all selected ids, commonly for Escape or filter changes.
///
/// # User actions
///
/// - [`SelectionState::select`] applies normal selection rules for click, Enter, or focus commit.
/// - [`SelectionState::toggle`] applies checkbox-like or modifier-click behavior.
/// - [`SelectionState::select_next`] and [`SelectionState::select_previous`] move over the ordered
///   ids visible in the current frame.
///
/// # Examples
///
/// Traverse over the ids visible in the current frame:
///
/// ```rust
/// use ratatui_layout::{SelectionMode, SelectionState};
///
/// let visible_ids = ["first", "second", "third"];
/// let mut selection = SelectionState::new(SelectionMode::Single);
///
/// selection.select_next(&visible_ids);
/// assert_eq!(selection.primary(), Some("first"));
///
/// selection.select_next(&visible_ids);
/// assert_eq!(selection.primary(), Some("second"));
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectionState<Id = usize> {
    mode: SelectionMode,
    selected: Vec<Id>,
}

impl<Id> Default for SelectionState<Id> {
    fn default() -> Self {
        Self {
            mode: SelectionMode::default(),
            selected: Vec::new(),
        }
    }
}

impl<Id> SelectionState<Id> {
    /// Creates selection state for the given mode.
    ///
    /// New state starts empty. Call [`SelectionState::select`] or one of the traversal methods
    /// after the first frame has produced visible ids.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::{SelectionMode, SelectionState};
    ///
    /// let selection = SelectionState::<usize>::new(SelectionMode::None);
    /// assert!(selection.selected().is_empty());
    /// ```
    pub const fn new(mode: SelectionMode) -> Self {
        Self {
            mode,
            selected: Vec::new(),
        }
    }

    /// Returns the current selection mode.
    ///
    /// # Examples
    ///
    /// Branch input behavior based on whether the current view supports multi-select:
    ///
    /// ```rust
    /// use ratatui_layout::{SelectionMode, SelectionState};
    ///
    /// let selection = SelectionState::<usize>::new(SelectionMode::Multi);
    /// let uses_checkbox_gutter = selection.mode() == SelectionMode::Multi;
    ///
    /// assert!(uses_checkbox_gutter);
    /// ```
    pub const fn mode(&self) -> SelectionMode {
        self.mode
    }

    /// Sets the current selection mode and removes selections that cannot be represented.
    ///
    /// Switching to [`SelectionMode::None`] clears all ids. Switching to [`SelectionMode::Single`]
    /// keeps only the primary id.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::{SelectionMode, SelectionState};
    ///
    /// let mut selection = SelectionState::new(SelectionMode::Multi);
    /// selection.select("a");
    /// selection.select("b");
    ///
    /// selection.set_mode(SelectionMode::Single);
    /// assert_eq!(selection.selected(), &["a"]);
    /// ```
    pub fn set_mode(&mut self, mode: SelectionMode) {
        self.mode = mode;
        match self.mode {
            SelectionMode::None => self.clear(),
            SelectionMode::Single => self.selected.truncate(1),
            SelectionMode::Multi => {}
        }
    }

    /// Returns selected ids in insertion order.
    ///
    /// Single-selection state returns either an empty slice or one id.
    ///
    /// # Examples
    ///
    /// Read selected ids when applying a bulk action:
    ///
    /// ```rust
    /// use ratatui_layout::{SelectionMode, SelectionState};
    ///
    /// let mut selection = SelectionState::new(SelectionMode::Multi);
    /// selection.select("alpha");
    /// selection.select("beta");
    ///
    /// let selected: Vec<_> = selection.selected().iter().copied().collect();
    /// assert_eq!(selected, ["alpha", "beta"]);
    /// ```
    pub fn selected(&self) -> &[Id] {
        &self.selected
    }

    /// Clears selection.
    ///
    /// # Examples
    ///
    /// Clear row selection when Escape closes selection mode:
    ///
    /// ```rust
    /// use ratatui_layout::{SelectionMode, SelectionState};
    ///
    /// let mut selection = SelectionState::new(SelectionMode::Single);
    /// selection.select("row-4");
    /// selection.clear();
    ///
    /// assert!(selection.selected().is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.selected.clear();
    }
}

impl<Id: Copy + Eq> SelectionState<Id> {
    /// Returns the primary selected id.
    ///
    /// The primary id is the first selected id. Multi-selection traversal advances from the most
    /// recently selected id, but this method is stable for callers that need a representative
    /// selection.
    ///
    /// # Examples
    ///
    /// Show a stable representative item in status text even when multiple ids are selected:
    ///
    /// ```rust
    /// use ratatui_layout::{SelectionMode, SelectionState};
    ///
    /// let mut selection = SelectionState::new(SelectionMode::Multi);
    /// selection.select("first");
    /// selection.select("second");
    ///
    /// assert_eq!(selection.primary(), Some("first"));
    /// ```
    pub fn primary(&self) -> Option<Id> {
        self.selected.first().copied()
    }

    /// Returns true when the id is selected.
    ///
    /// # Examples
    ///
    /// Check each rendered row id while choosing its visual style:
    ///
    /// ```rust
    /// use ratatui_layout::{SelectionMode, SelectionState};
    ///
    /// let rows = ["alpha", "beta"];
    /// let mut selection = SelectionState::new(SelectionMode::Single);
    /// selection.select("beta");
    ///
    /// let selected_rows: Vec<_> = rows
    ///     .into_iter()
    ///     .filter(|row| selection.is_selected(*row))
    ///     .collect();
    ///
    /// assert_eq!(selected_rows, ["beta"]);
    /// ```
    pub fn is_selected(&self, id: Id) -> bool {
        self.selected.contains(&id)
    }

    /// Selects an id according to the current mode.
    ///
    /// In single-selection mode, this replaces the previous id. In multi-selection mode, this adds
    /// the id if it is not already present. In none mode, it does nothing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::{SelectionMode, SelectionState};
    ///
    /// let mut single = SelectionState::new(SelectionMode::Single);
    /// single.select(1);
    /// single.select(2);
    /// assert_eq!(single.selected(), &[2]);
    /// ```
    pub fn select(&mut self, id: Id) {
        match self.mode {
            SelectionMode::None => {}
            SelectionMode::Single => {
                self.selected.clear();
                self.selected.push(id);
            }
            SelectionMode::Multi => {
                if !self.selected.contains(&id) {
                    self.selected.push(id);
                }
            }
        }
    }

    /// Toggles an id according to the current mode.
    ///
    /// Toggle is useful for checkbox-like input and multi-select mouse clicks. In single-selection
    /// mode, toggling the current id clears it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::{SelectionMode, SelectionState};
    ///
    /// let mut multi = SelectionState::new(SelectionMode::Multi);
    /// multi.toggle("debug");
    /// assert!(multi.is_selected("debug"));
    /// multi.toggle("debug");
    /// assert!(!multi.is_selected("debug"));
    /// ```
    pub fn toggle(&mut self, id: Id) {
        match self.mode {
            SelectionMode::None => {}
            SelectionMode::Single => {
                if self.selected == [id] {
                    self.clear();
                } else {
                    self.select(id);
                }
            }
            SelectionMode::Multi => {
                if let Some(index) = self.selected.iter().position(|selected| *selected == id) {
                    self.selected.remove(index);
                } else {
                    self.selected.push(id);
                }
            }
        }
    }

    /// Selects the next visible id, wrapping at the end.
    ///
    /// The visible ids come from the current frame's layout order. This keeps traversal aligned
    /// with what the user can see, even when filtering or scrolling changes the underlying data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::{SelectionMode, SelectionState};
    ///
    /// let mut selection = SelectionState::new(SelectionMode::Single);
    /// selection.select("third");
    /// selection.select_next(&["first", "second", "third"]);
    ///
    /// assert_eq!(selection.primary(), Some("first"));
    /// ```
    pub fn select_next(&mut self, visible_ids: &[Id]) {
        self.select_relative(visible_ids, Direction::Next);
    }

    /// Selects the previous visible id, wrapping at the start.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::{SelectionMode, SelectionState};
    ///
    /// let mut selection = SelectionState::new(SelectionMode::Single);
    /// selection.select("first");
    /// selection.select_previous(&["first", "second", "third"]);
    ///
    /// assert_eq!(selection.primary(), Some("third"));
    /// ```
    pub fn select_previous(&mut self, visible_ids: &[Id]) {
        self.select_relative(visible_ids, Direction::Previous);
    }

    fn select_relative(&mut self, visible_ids: &[Id], direction: Direction) {
        if self.mode == SelectionMode::None || visible_ids.is_empty() {
            return;
        }
        let next = match self
            .selected
            .last()
            .copied()
            .and_then(|id| visible_ids.iter().position(|visible| *visible == id))
        {
            Some(index) => match direction {
                Direction::Next => visible_ids[(index + 1) % visible_ids.len()],
                Direction::Previous => {
                    visible_ids[(index + visible_ids.len() - 1) % visible_ids.len()]
                }
            },
            None => visible_ids[0],
        };
        self.select(next);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Next,
    Previous,
}

/// Bridge between durable selection ids and frame-local visible positions.
///
/// Virtualized views often need two identities for the same selection. Application commands should
/// operate on a durable id such as a record key, while render and focus code often need a visible
/// position such as a row index or table cell. [`VisibleSelection`] stores both values together and
/// provides small helpers for the common conversions.
///
/// The type does not know how to look up data. Callers provide closures that map a visible position
/// to a durable id or map a durable id back to its current visible position. That keeps it useful
/// for lists, tables, grids, and filtered views without making it depend on any one collection
/// type.
///
/// Use [`SelectionState`] when selection is simply a set of ids. Use [`VisibleSelection`] when a
/// component must also keep a rendered position synchronized with a durable application id.
///
/// # Common uses
///
/// - Keep a selected row attached to a stable record id while filtering changes visible indexes.
/// - Convert a clicked table cell into the durable id used by commands.
/// - Keep keyboard focus on a visible cell while details panes and commands use a record key.
/// - Pair `VisibleSelection<RecordId>` with `VirtualListState` so rendering can select by visible
///   source index while commands still use a durable record id.
///
/// # Visible id helpers
///
/// When the visible positions are ordinary indexes into a slice of ids, use
/// [`VisibleSelection::sync_ids`], [`VisibleSelection::select_index`], and
/// [`VisibleSelection::move_by`]. Keep [`VisibleSelection::sync`] and
/// [`VisibleSelection::select_position`] for custom projections such as tables, grouped rows, or
/// views where a visible position is not a `usize` index.
///
/// # Examples
///
/// Synchronize a durable id after filtering changes the visible rows:
///
/// ```rust
/// use ratatui_layout::VisibleSelection;
///
/// let rows = [(0, "api"), (1, "docs")];
/// let mut selection = VisibleSelection::new();
///
/// selection.select_position(1, |row| rows.get(row).map(|(_, id)| *id));
/// assert_eq!(selection.selected_id(), Some("docs"));
///
/// let filtered = [(0, "docs")];
/// selection.sync(
///     || filtered.first().copied(),
///     |id| filtered.iter().copied().find(|(_, row_id)| *row_id == id),
/// );
///
/// assert_eq!(selection.position(), Some(0));
/// assert_eq!(selection.selected_id(), Some("docs"));
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VisibleSelection<Id, Position = usize> {
    id: Option<Id>,
    position: Option<Position>,
}

impl<Id, Position> Default for VisibleSelection<Id, Position> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Id, Position> VisibleSelection<Id, Position> {
    /// Creates an empty visible-selection bridge.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::VisibleSelection;
    ///
    /// let selection = VisibleSelection::<&str>::new();
    ///
    /// assert_eq!(selection.selected_id(), None);
    /// ```
    pub const fn new() -> Self {
        Self {
            id: None,
            position: None,
        }
    }

    /// Clears both the durable id and the visible position.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::VisibleSelection;
    ///
    /// let mut selection = VisibleSelection::new();
    /// selection.select_visible(2, "row-2");
    /// selection.clear();
    ///
    /// assert_eq!(selection.position(), None);
    /// ```
    pub fn clear(&mut self) {
        self.id = None;
        self.position = None;
    }
}

impl<Id: Copy, Position: Copy> VisibleSelection<Id, Position> {
    /// Returns the durable selected id, if any.
    ///
    /// Commands and details panes usually want this value because it remains meaningful after
    /// filtering, sorting, or scrolling.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::VisibleSelection;
    ///
    /// let mut selection = VisibleSelection::new();
    /// selection.select_visible(0, "api");
    ///
    /// assert_eq!(selection.selected_id(), Some("api"));
    /// ```
    pub const fn selected_id(&self) -> Option<Id> {
        self.id
    }

    /// Returns the current visible position, if any.
    ///
    /// Render and focus code usually want this value because it names what is currently on screen.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::VisibleSelection;
    ///
    /// let mut selection = VisibleSelection::new();
    /// selection.select_visible(3, "docs");
    ///
    /// assert_eq!(selection.position(), Some(3));
    /// ```
    pub const fn position(&self) -> Option<Position> {
        self.position
    }

    /// Selects a known visible position and durable id together.
    ///
    /// Use this after a caller has already resolved a click or keyboard movement to both pieces of
    /// information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::VisibleSelection;
    ///
    /// let mut selection = VisibleSelection::new();
    /// let position = selection.select_visible(4, "release-4");
    ///
    /// assert_eq!(position, 4);
    /// assert_eq!(selection.selected_id(), Some("release-4"));
    /// ```
    pub const fn select_visible(&mut self, position: Position, id: Id) -> Position {
        self.id = Some(id);
        self.position = Some(position);
        position
    }

    /// Selects a visible position by looking up its durable id.
    ///
    /// This is the mouse-click path for virtualized views: input routing gives a visible position,
    /// and the view supplies the durable id at that position. If the position is no longer visible,
    /// the selection is left unchanged.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::VisibleSelection;
    ///
    /// let rows = ["api", "docs"];
    /// let mut selection = VisibleSelection::new();
    ///
    /// assert_eq!(
    ///     selection.select_position(1, |row| rows.get(row).copied()),
    ///     Some(1)
    /// );
    /// assert_eq!(selection.selected_id(), Some("docs"));
    /// ```
    pub fn select_position(
        &mut self,
        position: Position,
        id_at: impl FnOnce(Position) -> Option<Id>,
    ) -> Option<Position> {
        let id = id_at(position)?;
        Some(self.select_visible(position, id))
    }

    /// Synchronizes the stored durable id with the current visible positions.
    ///
    /// `fallback` returns the position and id to use when there is no selection or the selected id
    /// is filtered out. `locate` returns the current visible position for an existing durable id.
    /// When neither closure can produce a visible row, the selection is cleared.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::VisibleSelection;
    ///
    /// let mut selection = VisibleSelection::new();
    /// selection.select_visible(2, "docs");
    ///
    /// let visible = [(0, "api")];
    /// let position = selection.sync(
    ///     || visible.first().copied(),
    ///     |id| visible.iter().copied().find(|(_, row_id)| *row_id == id),
    /// );
    ///
    /// assert_eq!(position, Some(0));
    /// assert_eq!(selection.selected_id(), Some("api"));
    /// ```
    pub fn sync(
        &mut self,
        fallback: impl FnOnce() -> Option<(Position, Id)>,
        locate: impl FnOnce(Id) -> Option<(Position, Id)>,
    ) -> Option<Position> {
        let next = self.id.and_then(locate).or_else(fallback);
        if let Some((position, id)) = next {
            Some(self.select_visible(position, id))
        } else {
            self.clear();
            None
        }
    }
}

impl<Id: Copy + Eq> VisibleSelection<Id, usize> {
    /// Synchronizes selection against a visible slice of durable ids.
    ///
    /// This is the common filtered-list path. If the current selected id is still visible, its
    /// visible index is refreshed. If the selected id is missing or there is no selection yet, the
    /// first visible id becomes selected. Empty visible slices clear the selection.
    ///
    /// Use [`VisibleSelection::sync`] when visible positions are not ordinary indexes or when the
    /// fallback should be something other than the first visible id.
    ///
    /// # Examples
    ///
    /// Keep a selected record id valid after filtering hides some rows:
    ///
    /// ```rust
    /// use ratatui_layout::VisibleSelection;
    ///
    /// let mut selection = VisibleSelection::new();
    /// selection.select_visible(1, "docs");
    ///
    /// let visible_ids = ["api", "docs"];
    /// assert_eq!(selection.sync_ids(&visible_ids), Some(1));
    ///
    /// let filtered_ids = ["api"];
    /// assert_eq!(selection.sync_ids(&filtered_ids), Some(0));
    /// assert_eq!(selection.selected_id(), Some("api"));
    /// ```
    pub fn sync_ids(&mut self, visible_ids: &[Id]) -> Option<usize> {
        self.sync(
            || visible_ids.first().copied().map(|id| (0, id)),
            |id| {
                visible_ids
                    .iter()
                    .position(|visible| *visible == id)
                    .map(|position| (position, id))
            },
        )
    }

    /// Selects the durable id at a visible index.
    ///
    /// This is the direct click or keyboard-commit path when input has already resolved to a
    /// visible row index. Invalid indexes leave the selection unchanged and return `None`.
    ///
    /// # Examples
    ///
    /// Select the record id at the clicked visible row:
    ///
    /// ```rust
    /// use ratatui_layout::VisibleSelection;
    ///
    /// let visible_ids = ["api", "docs"];
    /// let mut selection = VisibleSelection::new();
    ///
    /// assert_eq!(selection.select_index(1, &visible_ids), Some(1));
    /// assert_eq!(selection.selected_id(), Some("docs"));
    /// assert_eq!(selection.select_index(9, &visible_ids), None);
    /// assert_eq!(selection.selected_id(), Some("docs"));
    /// ```
    pub fn select_index(&mut self, index: usize, visible_ids: &[Id]) -> Option<usize> {
        let id = visible_ids.get(index).copied()?;
        Some(self.select_visible(index, id))
    }

    /// Moves selection by a signed offset through visible ids.
    ///
    /// Movement is clamped at the first and last visible id. If the current selected id is visible,
    /// movement starts there. If there is no current visible id, the first visible id becomes the
    /// fallback selection. Empty visible slices clear the selection.
    ///
    /// # Examples
    ///
    /// Move through filtered ids without losing the durable selected id:
    ///
    /// ```rust
    /// use ratatui_layout::VisibleSelection;
    ///
    /// let visible_ids = ["api", "docs", "ops"];
    /// let mut selection = VisibleSelection::new();
    ///
    /// assert_eq!(selection.move_by(1, &visible_ids), Some(0));
    /// assert_eq!(selection.selected_id(), Some("api"));
    ///
    /// assert_eq!(selection.move_by(1, &visible_ids), Some(1));
    /// assert_eq!(selection.selected_id(), Some("docs"));
    /// ```
    pub fn move_by(&mut self, delta: isize, visible_ids: &[Id]) -> Option<usize> {
        if visible_ids.is_empty() {
            self.clear();
            return None;
        }

        let Some(current) = self
            .id
            .and_then(|id| visible_ids.iter().position(|visible| *visible == id))
        else {
            return self.select_index(0, visible_ids);
        };
        let index = current
            .saturating_add_signed(delta)
            .min(visible_ids.len() - 1);

        self.select_index(index, visible_ids)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn none_mode_ignores_selection() {
        let mut state = SelectionState::new(SelectionMode::None);
        state.select(1);
        state.toggle(2);
        state.select_next(&[1, 2]);

        assert!(state.selected().is_empty());
    }

    #[test]
    fn single_mode_replaces_and_toggles() {
        let mut state = SelectionState::new(SelectionMode::Single);
        state.select(1);
        state.select(2);
        assert_eq!(state.selected(), &[2]);

        state.toggle(2);
        assert!(state.selected().is_empty());
    }

    #[test]
    fn multi_mode_toggles_in_insertion_order() {
        let mut state = SelectionState::new(SelectionMode::Multi);
        state.toggle("a");
        state.toggle("b");
        state.toggle("a");

        assert_eq!(state.selected(), &["b"]);
    }

    #[test]
    fn traversal_wraps_visible_ids() {
        let mut state = SelectionState::new(SelectionMode::Single);
        state.select_next(&[1, 2, 3]);
        assert_eq!(state.primary(), Some(1));
        state.select_previous(&[1, 2, 3]);
        assert_eq!(state.primary(), Some(3));
    }

    #[test]
    fn multi_traversal_uses_most_recent_selection() {
        let mut state = SelectionState::new(SelectionMode::Multi);
        state.select_next(&[1, 2, 3]);
        state.select_next(&[1, 2, 3]);
        state.select_next(&[1, 2, 3]);

        assert_eq!(state.selected(), &[1, 2, 3]);
    }

    #[test]
    fn visible_selection_bridges_position_and_durable_id() {
        let rows = [(0, "api"), (1, "docs")];
        let mut selection = VisibleSelection::new();

        selection.select_position(1, |row| rows.get(row).map(|(_, id)| *id));

        assert_eq!(selection.selected_id(), Some("docs"));
        assert_eq!(selection.position(), Some(1));
    }

    #[test]
    fn visible_selection_falls_back_when_id_is_filtered_out() {
        let mut selection = VisibleSelection::new();
        selection.select_visible(1, "docs");

        let visible = [(0, "api")];
        let position = selection.sync(
            || visible.first().copied(),
            |id| visible.iter().copied().find(|(_, row_id)| *row_id == id),
        );

        assert_eq!(position, Some(0));
        assert_eq!(selection.selected_id(), Some("api"));
    }

    #[test]
    fn visible_selection_syncs_visible_id_slices() {
        let mut selection = VisibleSelection::new();
        selection.select_visible(1, "docs");

        assert_eq!(selection.sync_ids(&["api", "docs"]), Some(1));
        assert_eq!(selection.position(), Some(1));
        assert_eq!(selection.selected_id(), Some("docs"));

        assert_eq!(selection.sync_ids(&["api"]), Some(0));
        assert_eq!(selection.position(), Some(0));
        assert_eq!(selection.selected_id(), Some("api"));

        assert_eq!(selection.sync_ids(&[]), None);
        assert_eq!(selection.selected_id(), None);
    }

    #[test]
    fn visible_selection_selects_visible_index() {
        let mut selection = VisibleSelection::new();

        assert_eq!(selection.select_index(1, &["api", "docs"]), Some(1));
        assert_eq!(selection.position(), Some(1));
        assert_eq!(selection.selected_id(), Some("docs"));

        assert_eq!(selection.select_index(9, &["api", "docs"]), None);
        assert_eq!(selection.position(), Some(1));
        assert_eq!(selection.selected_id(), Some("docs"));
    }

    #[test]
    fn visible_selection_moves_over_visible_ids() {
        let mut selection = VisibleSelection::new();
        let visible = ["api", "docs", "ops"];

        assert_eq!(selection.move_by(1, &visible), Some(0));
        assert_eq!(selection.selected_id(), Some("api"));

        assert_eq!(selection.move_by(1, &visible), Some(1));
        assert_eq!(selection.selected_id(), Some("docs"));

        assert_eq!(selection.move_by(99, &visible), Some(2));
        assert_eq!(selection.selected_id(), Some("ops"));

        assert_eq!(selection.move_by(-99, &visible), Some(0));
        assert_eq!(selection.selected_id(), Some("api"));

        assert_eq!(selection.move_by(1, &[]), None);
        assert_eq!(selection.selected_id(), None);
    }
}
