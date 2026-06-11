//! Virtual list layout and external row rendering.
//!
//! [`VirtualList`](crate::list::VirtualList) is a vertical viewport over externally owned items. It
//! does not store row widgets, row data, or row state. Instead, it asks a
//! [`ListItems`](crate::list::ListItems) implementation how many items exist, how tall each item is
//! at the final width, and how to render a visible item into an assigned
//! [`Rect`](ratatui_core::layout::Rect).
//!
//! Use Ratatui's built-in list widget for ordinary text lists. `VirtualList` is for cases where the
//! list behavior is the problem: large collections, variable-height rows, custom row rendering,
//! externally owned row state, hit testing, or line-aware scrolling through multiline items.
//!
//! See [`crate::docs::virtualization`] for how virtual lists relate to viewports, virtual tables,
//! visible rows, and scroll metrics.
//!
//! This design keeps application ownership visible:
//!
//! - the app owns the collection and any per-row state;
//! - [`VirtualListState`](crate::list::VirtualListState) owns selection and scroll state between
//!   frames;
//! - [`VirtualList`](crate::list::VirtualList) owns the viewport policy for one layout or render
//!   call;
//! - [`ListLayout`](crate::list::ListLayout) exposes the computed visible rows, clipping, and
//!   hit-test data.
//!
//! # Types and traits
//!
//! - [`ScrollPosition`](crate::list::ScrollPosition) stores an item index plus an in-item line
//!   offset for line-aware scrolling.
//! - [`VirtualListState`](crate::list::VirtualListState) stores persistent selection and scroll
//!   state between frames.
//! - [`ListItems`](crate::list::ListItems) is the trait for app-owned row data that can measure and
//!   render visible rows.
//! - [`ListItemsFn`](crate::list::ListItemsFn) adapts simple closures into
//!   [`ListItems`](crate::list::ListItems) for tests, examples, and small apps.
//! - [`ListItemContext`](crate::list::ListItemContext) is passed to row renderers with selection,
//!   visible-index, and clipping metadata.
//! - [`VisibleItem`](crate::list::VisibleItem) describes one row slice visible in the current
//!   frame.
//! - [`ListLayout`](crate::list::ListLayout) is the solved frame-local layout for visible rows, hit
//!   testing, and scrollbars.
//! - [`VirtualList`](crate::list::VirtualList) is the reusable list configuration that computes or
//!   renders a [`ListLayout`](crate::list::ListLayout).
//! - [`ListHeightCache`](crate::list::ListHeightCache) stores measured row heights when measurement
//!   is expensive.
//!
//! # Measurement and rendering
//!
//! Measurement happens before rendering.
//! [`ListItems::height_for_width`](crate::list::ListItems::height_for_width) receives the final
//! list width and returns the full logical height of the item. The list then computes which item
//! slices are visible and calls [`ListItems::render_item`](crate::list::ListItems::render_item)
//! only for those items.
//!
//! Partially visible items are allowed. A renderer receives
//! [`ListItemContext::y_offset`](crate::list::ListItemContext::y_offset) and clipping flags so it
//! can decide whether to skip lines, draw a continuation marker, or render a simplified clipped
//! representation.
//!
//! # Examples
//!
//! ```rust
//! use ratatui_core::buffer::Buffer;
//! use ratatui_core::layout::Rect;
//! use ratatui_core::text::Line;
//! use ratatui_core::widgets::Widget;
//! use ratatui_layout::list::{ListItemContext, ListItems, VirtualList, VirtualListState};
//! use ratatui_layout::participant::MeasureContext;
//!
//! struct Rows(&'static [&'static str]);
//!
//! impl ListItems for Rows {
//!     fn len(&self) -> usize {
//!         self.0.len()
//!     }
//!
//!     fn height_for_width(&self, index: usize, width: u16, _: MeasureContext) -> u16 {
//!         let width = width.max(1) as usize;
//!         self.0[index].len().div_ceil(width).max(1) as u16
//!     }
//!
//!     fn render_item(
//!         &mut self,
//!         index: usize,
//!         area: Rect,
//!         buf: &mut Buffer,
//!         ctx: ListItemContext,
//!     ) {
//!         let marker = if ctx.render.state.selected {
//!             "> "
//!         } else {
//!             "  "
//!         };
//!         Line::from(format!("{marker}{}", self.0[index])).render(area, buf);
//!     }
//! }
//!
//! let mut rows = Rows(&["short", "a longer row"]);
//! let mut state = VirtualListState::default();
//! state.select(Some(1));
//!
//! let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 2));
//! let layout = VirtualList::new().render(buffer.area, &mut buffer, &mut state, &mut rows);
//!
//! assert_eq!(layout.selected.unwrap().index, 1);
//! ```

use alloc::vec::Vec;

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Position, Rect};

use crate::participant::{MeasureContext, RenderContext, RenderState};
use crate::regions::{Clip, Hit, Region, Regions};
use crate::selection::VisibleSelection;

/// Line-aware scroll position for a virtual list.
///
/// Use [`ScrollPosition`] when rows can be taller than one terminal line. An item-only offset can
/// say "start at item 10", but it cannot say "start on the fourth line of item 10." That
/// distinction matters for smooth scrolling, page scrolling, and selected rows that are taller than
/// the viewport.
///
/// [`ScrollPosition`] identifies the first logical line shown in the viewport as an item index plus
/// a line offset inside that item.
///
/// # Constructor
///
/// - [`ScrollPosition::new`] creates a desired item/line offset. [`VirtualList::layout`] clamps it
///   against the current rows and measured heights.
///
/// # Examples
///
/// Keep smooth scrolling state beside the selected row:
///
/// ```rust
/// use ratatui_layout::list::{ScrollPosition, VirtualListState};
///
/// let mut state = VirtualListState::default();
/// state.select(Some(10));
/// state.set_scroll(ScrollPosition::new(8, 2));
///
/// assert_eq!(state.selected(), Some(10));
/// assert_eq!(
///     state.scroll(),
///     ScrollPosition {
///         index: 8,
///         y_offset: 2
///     }
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScrollPosition {
    /// The item index where the viewport starts.
    pub index: usize,
    /// The number of lines skipped from the top of `index`.
    pub y_offset: u16,
}

impl ScrollPosition {
    /// Creates a scroll position.
    ///
    /// The value is clamped during [`VirtualList::layout`] and [`VirtualList::render`].
    /// Constructing a position with an out-of-bounds index or offset is allowed so applications
    /// can restore stale state before the current item count is known.
    ///
    /// # Examples
    ///
    /// Restore a line-aware scroll position before the next layout clamps it:
    ///
    /// ```rust
    /// use ratatui_layout::list::{ScrollPosition, VirtualListState};
    ///
    /// let mut state = VirtualListState::default();
    /// state.set_scroll(ScrollPosition::new(5, 2));
    ///
    /// assert_eq!(
    ///     state.scroll(),
    ///     ScrollPosition {
    ///         index: 5,
    ///         y_offset: 2
    ///     }
    /// );
    /// ```
    pub const fn new(index: usize, y_offset: u16) -> Self {
        Self { index, y_offset }
    }
}

/// State for [`VirtualList`].
///
/// Use [`VirtualListState`] as the persistent UI state for a [`VirtualList`]. Store it in the
/// application next to the data being listed. The list will update it when selection or scroll
/// positions need to be clamped to the current content.
///
/// The state stores only scroll and selection. It does not store item count, measured heights,
/// visible rows, or child widget state. Those values are either owned by the application or exposed
/// in the [`ListLayout`] returned by each layout pass.
///
/// # Selection
///
/// - [`VirtualListState::selected`] reads the selected source item index.
/// - [`VirtualListState::select`] sets or clears the selected source item index and asks layout to
///   reveal it.
/// - [`VirtualListState::select_relative`] moves selection by a signed row delta and asks layout to
///   reveal the new row.
/// - [`VirtualListState::select_without_scrolling`] sets selection for styling or commands without
///   moving the viewport back to it.
/// - [`VirtualListState::scroll_to_selected`] asks the next layout to reveal the current selected
///   item.
/// - [`VirtualListState::scrolls_selected_into_view`] reports which selection policy the next
///   layout pass will use.
///
/// # Scrolling
///
/// - [`VirtualListState::scroll`] reads the current line-aware scroll position.
/// - [`VirtualListState::set_scroll`] stores a desired line-aware scroll position for the next
///   layout pass to clamp.
/// - [`VirtualListState::scroll_viewport_by`] moves the viewport by item indexes without changing
///   selection.
///
/// # Examples
///
/// Store selection and scroll between frames while rebuilding the list layout as needed:
///
/// ```rust
/// use ratatui_layout::list::{ScrollPosition, VirtualListState};
///
/// let mut state = VirtualListState::default();
/// state.select(Some(3));
/// state.set_scroll(ScrollPosition::new(2, 0));
///
/// assert_eq!(state.selected(), Some(3));
/// assert_eq!(state.scroll().index, 2);
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VirtualListState {
    selected: Option<usize>,
    scroll: ScrollPosition,
    #[cfg_attr(feature = "serde", serde(default))]
    scroll_selected_into_view: bool,
}

impl VirtualListState {
    /// Returns the selected item index.
    ///
    /// # Examples
    ///
    /// Read the selected source row after input updates list state:
    ///
    /// ```rust
    /// use ratatui_layout::list::VirtualListState;
    ///
    /// let mut state = VirtualListState::default();
    /// state.select(Some(3));
    ///
    /// assert_eq!(state.selected(), Some(3));
    /// ```
    pub const fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Returns whether layout should keep the selected item visible.
    ///
    /// This flag separates two common intents that otherwise look the same in state: keyboard
    /// selection should usually move the viewport so the selected row can be seen, while
    /// mouse-wheel scrolling should usually move the viewport without snapping back to the
    /// selected row. Calling [`VirtualListState::select`] turns this on. Calling
    /// [`VirtualListState::scroll_viewport_by`] turns it off.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::list::VirtualListState;
    ///
    /// let mut state = VirtualListState::default();
    /// state.select(Some(4));
    /// assert!(state.scrolls_selected_into_view());
    ///
    /// state.scroll_viewport_by(1);
    /// assert!(!state.scrolls_selected_into_view());
    /// ```
    pub const fn scrolls_selected_into_view(&self) -> bool {
        self.scroll_selected_into_view
    }

    /// Sets the selected item index.
    ///
    /// The selected index is clamped to the last available item during layout. Passing `None`
    /// clears selection without changing scroll until the next layout pass decides whether the
    /// scroll position also needs clamping.
    ///
    /// Selecting an item also asks the next layout pass to keep that item visible. Use
    /// [`VirtualListState::select_without_scrolling`] when a caller wants selected styling for a
    /// row that may or may not currently be in the viewport.
    ///
    /// # Examples
    ///
    /// Select a source row before rendering so the row renderer receives selected state:
    ///
    /// ```rust
    /// use ratatui_layout::list::VirtualListState;
    ///
    /// let mut state = VirtualListState::default();
    /// state.select(Some(1));
    ///
    /// assert_eq!(state.selected(), Some(1));
    /// ```
    pub const fn select(&mut self, selected: Option<usize>) {
        self.selected = selected;
        self.scroll_selected_into_view = selected.is_some();
    }

    /// Moves selection by a signed row delta.
    ///
    /// Use this for keyboard row movement in source-index lists. The caller passes the current item
    /// count because the list state intentionally does not own the app's collection. Empty lists
    /// clear selection. If nothing is selected yet, the first movement selects the first item. When
    /// a row is already selected, movement is clamped to the first and last item and the next
    /// layout pass will reveal the selected row.
    ///
    /// This changes selection. Use [`VirtualListState::scroll_viewport_by`] for mouse-wheel or page
    /// scrolling that should move the viewport without changing the selected row.
    ///
    /// # Examples
    ///
    /// Move selected rows without hand-writing index clamping in the app:
    ///
    /// ```rust
    /// use ratatui_layout::list::VirtualListState;
    ///
    /// let mut state = VirtualListState::default();
    ///
    /// assert_eq!(state.select_relative(1, 3), Some(0));
    /// assert_eq!(state.select_relative(1, 3), Some(1));
    /// assert_eq!(state.select_relative(99, 3), Some(2));
    /// assert_eq!(state.select_relative(-99, 3), Some(0));
    /// ```
    pub const fn select_relative(&mut self, delta: isize, item_count: usize) -> Option<usize> {
        if item_count == 0 {
            self.select(None);
            return None;
        }

        let index = match self.selected {
            Some(selected) => {
                let index = offset_index(selected, delta);
                if index >= item_count {
                    item_count - 1
                } else {
                    index
                }
            }
            None => 0,
        };
        self.select(Some(index));
        Some(index)
    }

    /// Sets selection without asking layout to move the viewport.
    ///
    /// Use this when selection is stable app state but the current input changed only the viewport.
    /// For example, a mouse wheel over a list should usually scroll rows while leaving the selected
    /// record unchanged. If that record is still visible, row renderers will still receive selected
    /// state; if it is off-screen, the viewport is left alone.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::list::VirtualListState;
    ///
    /// let mut state = VirtualListState::default();
    /// state.select_without_scrolling(Some(8));
    ///
    /// assert_eq!(state.selected(), Some(8));
    /// assert!(!state.scrolls_selected_into_view());
    /// ```
    pub const fn select_without_scrolling(&mut self, selected: Option<usize>) {
        self.selected = selected;
        self.scroll_selected_into_view = false;
    }

    /// Asks the next layout pass to bring the current selection into view.
    ///
    /// This is useful when selection is restored from app state with
    /// [`VirtualListState::select_without_scrolling`] and a later keyboard command should reveal
    /// that selected row. The actual scroll offset still depends on the measured item heights and
    /// is computed by [`VirtualList::layout`] or [`VirtualList::render`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::list::VirtualListState;
    ///
    /// let mut state = VirtualListState::default();
    /// state.select_without_scrolling(Some(12));
    /// state.scroll_to_selected();
    ///
    /// assert!(state.scrolls_selected_into_view());
    /// ```
    pub const fn scroll_to_selected(&mut self) {
        self.scroll_selected_into_view = self.selected.is_some();
    }

    /// Returns the current scroll position.
    ///
    /// # Examples
    ///
    /// Read the clamped line-aware scroll position after layout:
    ///
    /// ```rust
    /// use ratatui_layout::list::{ScrollPosition, VirtualListState};
    ///
    /// let mut state = VirtualListState::default();
    /// state.set_scroll(ScrollPosition::new(2, 1));
    ///
    /// assert_eq!(state.scroll().index, 2);
    /// assert_eq!(state.scroll().y_offset, 1);
    /// ```
    pub const fn scroll(&self) -> ScrollPosition {
        self.scroll
    }

    /// Sets the current scroll position.
    ///
    /// The position is clamped during layout. This lets applications apply input deltas before
    /// measuring the current content.
    ///
    /// # Examples
    ///
    /// Apply a scroll command before the list knows current row heights:
    ///
    /// ```rust
    /// use ratatui_layout::list::{ScrollPosition, VirtualListState};
    ///
    /// let mut state = VirtualListState::default();
    /// state.set_scroll(ScrollPosition::new(10, 0));
    ///
    /// assert_eq!(state.scroll(), ScrollPosition::new(10, 0));
    /// ```
    pub const fn set_scroll(&mut self, scroll: ScrollPosition) {
        self.scroll = scroll;
    }

    /// Scrolls the viewport by item indexes without changing selection.
    ///
    /// This is the canonical mouse-wheel helper for fixed-height rows and simple tree/list
    /// navigation. It updates the desired starting item and lets the next layout pass clamp the
    /// value to the current item count. It also disables automatic selection reveal so wheel input
    /// does not snap back to the selected row.
    ///
    /// For variable-height rows, this remains item-based rather than line-based. Call
    /// [`VirtualListState::set_scroll`] with an explicit [`ScrollPosition`] when an app needs
    /// smooth line-aware scrolling within a tall row.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::list::{ScrollPosition, VirtualListState};
    ///
    /// let mut state = VirtualListState::default();
    /// state.select(Some(10));
    /// state.set_scroll(ScrollPosition::new(3, 0));
    /// state.scroll_viewport_by(2);
    ///
    /// assert_eq!(state.selected(), Some(10));
    /// assert_eq!(state.scroll(), ScrollPosition::new(5, 0));
    /// assert!(!state.scrolls_selected_into_view());
    /// ```
    pub const fn scroll_viewport_by(&mut self, delta: isize) {
        let index = offset_index(self.scroll.index, delta);
        self.scroll = ScrollPosition::new(index, 0);
        self.scroll_selected_into_view = false;
    }
}

/// Externally owned items rendered by a [`VirtualList`].
///
/// Implement this trait for the application object or a small adapter that can look up rows by
/// index. The list calls [`ListItems::height_for_width`] for measurement and
/// [`ListItems::render_item`] for visible rows only.
///
/// Returning zero height is treated as one line. This prevents invisible items from causing
/// non-terminating viewport calculations.
///
/// # Required methods
///
/// - [`ListItems::len`] returns the current source item count.
/// - [`ListItems::height_for_width`] measures one source item at the final list width.
/// - [`ListItems::render_item`] renders one visible item slice into its assigned rectangle.
///
/// # Provided methods
///
/// - [`ListItems::is_empty`] derives empty-state behavior from [`ListItems::len`].
///
/// # Examples
///
/// Implement the trait on app-owned data so [`VirtualList`] can measure and render only visible
/// rows:
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_core::text::Line;
/// use ratatui_core::widgets::Widget;
/// use ratatui_layout::list::{ListItemContext, ListItems};
/// use ratatui_layout::participant::MeasureContext;
///
/// struct Rows(&'static [&'static str]);
///
/// impl ListItems for Rows {
///     fn len(&self) -> usize {
///         self.0.len()
///     }
///
///     fn height_for_width(&self, index: usize, width: u16, _: MeasureContext) -> u16 {
///         self.0[index].len().div_ceil(width.max(1) as usize).max(1) as u16
///     }
///
///     fn render_item(&mut self, index: usize, area: Rect, buf: &mut Buffer, _: ListItemContext) {
///         Line::from(self.0[index]).render(area, buf);
///     }
/// }
/// ```
pub trait ListItems {
    /// Returns the number of items.
    ///
    /// The list may call this before measurement and rendering in each frame.
    ///
    /// # Examples
    ///
    /// Back a virtual list with app-owned row data:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{ListItemContext, ListItems};
    /// use ratatui_layout::participant::MeasureContext;
    ///
    /// struct Rows(&'static [&'static str]);
    ///
    /// impl ListItems for Rows {
    ///     fn len(&self) -> usize {
    ///         self.0.len()
    ///     }
    ///     fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
    ///         1
    ///     }
    ///     fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
    /// }
    ///
    /// assert_eq!(Rows(&["a", "b"]).len(), 2);
    /// ```
    fn len(&self) -> usize;

    /// Returns true when there are no items.
    ///
    /// # Examples
    ///
    /// Use the provided empty check before deciding whether to render an empty state:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{ListItemContext, ListItems};
    /// use ratatui_layout::participant::MeasureContext;
    ///
    /// struct Rows;
    ///
    /// impl ListItems for Rows {
    ///     fn len(&self) -> usize {
    ///         0
    ///     }
    ///     fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
    ///         1
    ///     }
    ///     fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
    /// }
    ///
    /// assert!(Rows.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the item height for the final width.
    ///
    /// The returned height is the full logical height, not just the visible height. It should be
    /// stable for the same item and width during a single layout pass.
    ///
    /// # Examples
    ///
    /// Measure wrapped text rows from the final list width:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{ListItemContext, ListItems};
    /// use ratatui_layout::participant::MeasureContext;
    ///
    /// struct Rows(&'static [&'static str]);
    ///
    /// impl ListItems for Rows {
    ///     fn len(&self) -> usize {
    ///         self.0.len()
    ///     }
    ///     fn height_for_width(&self, index: usize, width: u16, _: MeasureContext) -> u16 {
    ///         self.0[index].len().div_ceil(width.max(1) as usize).max(1) as u16
    ///     }
    ///     fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
    /// }
    ///
    /// assert_eq!(Rows(&["abcdef"]).height_for_width(0, 3, MeasureContext), 2);
    /// ```
    fn height_for_width(&self, index: usize, width: u16, ctx: MeasureContext) -> u16;

    /// Renders an item into the assigned area.
    ///
    /// The area may be shorter than the full measured item height when the item is partially
    /// clipped. Use [`ListItemContext::y_offset`] and the clipping flags to decide which logical
    /// lines to draw.
    ///
    /// # Examples
    ///
    /// Render only rows that [`VirtualList::render`] reports as visible:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_core::text::Line;
    /// use ratatui_core::widgets::Widget;
    /// use ratatui_layout::list::{ListItemContext, ListItems};
    /// use ratatui_layout::participant::MeasureContext;
    ///
    /// struct Rows(&'static [&'static str]);
    ///
    /// impl ListItems for Rows {
    ///     fn len(&self) -> usize {
    ///         self.0.len()
    ///     }
    ///     fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
    ///         1
    ///     }
    ///     fn render_item(
    ///         &mut self,
    ///         index: usize,
    ///         area: Rect,
    ///         buf: &mut Buffer,
    ///         ctx: ListItemContext,
    ///     ) {
    ///         let marker = if ctx.render.state.selected {
    ///             "> "
    ///         } else {
    ///             "  "
    ///         };
    ///         Line::from(format!("{marker}{}", self.0[index])).render(area, buf);
    ///     }
    /// }
    /// ```
    fn render_item(&mut self, index: usize, area: Rect, buf: &mut Buffer, ctx: ListItemContext);
}

/// Closure adapter for [`ListItems`].
///
/// Use [`ListItemsFn`] when a list is backed by simple local closures instead of a reusable row
/// source type. It is useful in examples, tests, and small applications where a named adapter would
/// obscure the relationship between the data, measurement, and rendering.
///
/// Larger apps should usually implement [`ListItems`] directly so the ownership and measurement
/// contracts are documented near the data.
///
/// # Constructor
///
/// - [`ListItemsFn::new`] stores len, height, and render closures behind the [`ListItems`] trait.
///
/// # Examples
///
/// Build a local row source without naming a new adapter type:
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::list::{ListItems, ListItemsFn, VirtualList, VirtualListState};
/// use ratatui_layout::participant::MeasureContext;
///
/// let rows = ["open", "save", "close"];
/// let mut items = ListItemsFn::new(
///     || rows.len(),
///     |index: usize, width: u16, _: MeasureContext| {
///         rows[index].len().div_ceil(width.max(1) as usize) as u16
///     },
///     |_, _: Rect, _: &mut Buffer, _| {},
/// );
///
/// let mut state = VirtualListState::default();
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 2));
/// let layout = VirtualList::new().render(buffer.area, &mut buffer, &mut state, &mut items);
///
/// assert_eq!(layout.visible_items.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct ListItemsFn<L, H, R> {
    len: L,
    height: H,
    render: R,
}

impl<L, H, R> ListItemsFn<L, H, R> {
    /// Creates list items from len, height, and render closures.
    ///
    /// # Examples
    ///
    /// Adapt local closures into [`ListItems`] for a small list:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{ListItems, ListItemsFn};
    /// use ratatui_layout::participant::MeasureContext;
    ///
    /// let rows = ["open", "save"];
    /// let items = ListItemsFn::new(
    ///     || rows.len(),
    ///     |_, _, _: MeasureContext| 1,
    ///     |_, _: Rect, _: &mut Buffer, _| {},
    /// );
    ///
    /// assert_eq!(items.len(), 2);
    /// ```
    pub const fn new(len: L, height: H, render: R) -> Self {
        Self {
            len,
            height,
            render,
        }
    }
}

impl<L, H, R> ListItems for ListItemsFn<L, H, R>
where
    L: Fn() -> usize,
    H: Fn(usize, u16, MeasureContext) -> u16,
    R: FnMut(usize, Rect, &mut Buffer, ListItemContext),
{
    fn len(&self) -> usize {
        (self.len)()
    }

    fn height_for_width(&self, index: usize, width: u16, ctx: MeasureContext) -> u16 {
        (self.height)(index, width, ctx)
    }

    fn render_item(&mut self, index: usize, area: Rect, buf: &mut Buffer, ctx: ListItemContext) {
        (self.render)(index, area, buf, ctx);
    }
}

/// Render context for a visible list item.
///
/// Use `ListItemContext` inside [`ListItems::render_item`] to decide how a row should look in its
/// assigned area. It tells the row whether it is selected, which visible row it is, and whether it
/// was clipped by the viewport.
///
/// [`ListItemContext`] combines common [`RenderContext`](crate::participant::RenderContext) state
/// with list-specific metadata. It is passed only to visible items during rendering.
///
/// # Fields
///
/// - [`ListItemContext::render`] carries shared interaction flags such as selection.
/// - [`ListItemContext::index`] is the source item index being rendered.
/// - [`ListItemContext::visible_index`] is the item position among visible rows.
/// - [`ListItemContext::y_offset`] tells how many logical lines were clipped from the top.
/// - [`ListItemContext::clipped_top`] and [`ListItemContext::clipped_bottom`] describe whether the
///   assigned area is a partial slice of a taller item.
///
/// # Examples
///
/// Use context fields together to render selected and clipped rows differently:
///
/// ```rust
/// use ratatui_layout::list::ListItemContext;
/// use ratatui_layout::participant::RenderContext;
///
/// let context = ListItemContext {
///     render: RenderContext::selected(true),
///     index: 4,
///     visible_index: 0,
///     y_offset: 2,
///     clipped_top: true,
///     clipped_bottom: false,
/// };
///
/// let marker = if context.render.state.selected {
///     "> "
/// } else {
///     "  "
/// };
/// assert_eq!(marker, "> ");
/// assert!(context.clipped_top);
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListItemContext {
    /// Common render context.
    ///
    /// Selection is set when this item is the state's selected index.
    pub render: RenderContext,
    /// The item index in the source list.
    pub index: usize,
    /// The item index among visible items.
    pub visible_index: usize,
    /// Lines skipped from the top of the item.
    ///
    /// A non-zero value means the item begins above the viewport and the renderer is receiving
    /// only the visible tail of the item.
    pub y_offset: u16,
    /// Whether the top of the item is clipped.
    pub clipped_top: bool,
    /// Whether the bottom of the item is clipped.
    pub clipped_bottom: bool,
}

/// Metadata for a visible list item.
///
/// Use [`VisibleItem`] when code outside the row renderer needs to inspect what the list did. A
/// pointer handler can map a click to a visible item. A debug line can show visible indexes. A
/// custom scrollbar can compare visible rows with total content height.
///
/// [`VisibleItem`] is returned in [`ListLayout::visible_items`]. It is the list's public render
/// layout output: each value says which source item is visible, where it should be rendered, how
/// tall the full item is, and whether the assigned area is clipped.
///
/// # Fields
///
/// - [`VisibleItem::index`] is the source item id for rendering and hit testing.
/// - [`VisibleItem::area`] is the terminal-space visible rectangle.
/// - [`VisibleItem::full_height`] is the measured logical item height.
/// - [`VisibleItem::y_offset`] is the number of hidden logical lines above [`VisibleItem::area`].
/// - [`VisibleItem::clipped_top`] and [`VisibleItem::clipped_bottom`] tell whether the item
///   continues beyond the viewport.
///
/// # Examples
///
/// Combine visible item metadata with hit testing to recover the logical row line:
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::list::{ListLayout, ScrollPosition, VisibleItem};
///
/// let item = VisibleItem {
///     index: 10,
///     area: Rect::new(0, 0, 20, 2),
///     full_height: 4,
///     y_offset: 1,
///     clipped_top: true,
///     clipped_bottom: true,
/// };
/// let layout = ListLayout {
///     viewport: Rect::new(0, 0, 20, 2),
///     item_count: 20,
///     content_height: 40,
///     scroll_offset: 21,
///     scroll: ScrollPosition::new(10, 1),
///     visible_items: vec![item],
///     selected: Some(item),
/// };
///
/// let hit = layout.hit_test((3, 1)).unwrap();
/// let logical_y = item.y_offset + hit.relative_y;
/// assert_eq!((hit.id, logical_y), (10, 2));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VisibleItem {
    /// The item index in the source list.
    pub index: usize,
    /// The visible area assigned to this item.
    pub area: Rect,
    /// The full measured item height.
    pub full_height: u16,
    /// Lines skipped from the top of the item.
    pub y_offset: u16,
    /// Whether the top of the item is clipped.
    pub clipped_top: bool,
    /// Whether the bottom of the item is clipped.
    pub clipped_bottom: bool,
}

impl VisibleItem {
    fn context(self, visible_index: usize, selected: bool) -> ListItemContext {
        ListItemContext {
            render: RenderContext {
                state: RenderState {
                    selected,
                    ..RenderState::default()
                },
            },
            index: self.index,
            visible_index,
            y_offset: self.y_offset,
            clipped_top: self.clipped_top,
            clipped_bottom: self.clipped_bottom,
        }
    }
}

/// Solved virtual-list layout.
///
/// Use [`ListLayout`] when list rendering needs to be observable. The high-level
/// [`VirtualList::render`] method still returns this value so callers can render normally and then
/// use the same frame's layout for hit testing, diagnostics, scrollbars, or status text.
///
/// [`ListLayout`] is returned from both [`VirtualList::layout`] and [`VirtualList::render`]. Keep
/// it around for the current frame when routing pointer input or displaying scroll diagnostics.
///
/// # Fields and methods
///
/// - [`ListLayout::viewport`] is the terminal area assigned to the list.
/// - [`ListLayout::item_count`] is the source collection length.
/// - [`ListLayout::content_height`] is the total measured height in logical lines.
/// - [`ListLayout::scroll_offset`] is the clamped offset in logical content lines.
/// - [`ListLayout::scroll`] is the clamped [`ScrollPosition`].
/// - [`ListLayout::visible_items`] is the ordered list of visible row slices.
/// - [`ListLayout::selected`] is the selected row when it is visible.
/// - [`ListLayout::visible_indices`] returns only the visible source row indexes.
/// - [`ListLayout::row_regions`] converts visible rows into a generic
///   [`Regions`](crate::regions::Regions).
/// - [`ListLayout::hit_test`] maps a terminal position to the visible source item index.
/// - [`ListLayout::hit_index`] returns only the source item index for common click handling.
/// - [`ListLayout::select_hit`] selects a durable id through
///   [`VisibleSelection`](crate::selection::VisibleSelection) after a click.
///
/// # Examples
///
/// Keep the layout returned by rendering so the next pointer event can route to a source row:
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::list::{ListItemContext, ListItems, VirtualList, VirtualListState};
/// use ratatui_layout::participant::MeasureContext;
///
/// struct Rows;
/// impl ListItems for Rows {
///     fn len(&self) -> usize {
///         5
///     }
///     fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
///         1
///     }
///     fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
/// }
///
/// let mut rows = Rows;
/// let mut state = VirtualListState::default();
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 12, 3));
/// let layout = VirtualList::new().render(buffer.area, &mut buffer, &mut state, &mut rows);
///
/// assert_eq!(layout.hit_test((1, 2)).unwrap().id, 2);
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListLayout {
    /// The list viewport.
    pub viewport: Rect,
    /// Total item count.
    pub item_count: usize,
    /// Total content height in lines.
    pub content_height: u32,
    /// Clamped scroll offset in logical content lines.
    pub scroll_offset: u32,
    /// Clamped scroll position.
    pub scroll: ScrollPosition,
    /// Visible item metadata.
    pub visible_items: Vec<VisibleItem>,
    /// Selected item if it is visible.
    pub selected: Option<VisibleItem>,
}

impl ListLayout {
    /// Returns source item indexes visible in this layout.
    ///
    /// Use this when keyboard traversal or selection needs the same visible order that rendering
    /// used, without exposing callers to row rectangles and clipping metadata. For repeated rows,
    /// the index is the frame-local id that maps back into the app-owned collection.
    ///
    /// # Examples
    ///
    /// Move selection over only the rows visible in the current list layout:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{ListLayout, ScrollPosition, VisibleItem};
    /// use ratatui_layout::selection::{SelectionMode, SelectionState};
    ///
    /// let layout = ListLayout {
    ///     viewport: Rect::new(0, 0, 10, 2),
    ///     item_count: 3,
    ///     content_height: 3,
    ///     scroll_offset: 1,
    ///     scroll: ScrollPosition::new(1, 0),
    ///     visible_items: vec![
    ///         VisibleItem {
    ///             index: 1,
    ///             area: Rect::new(0, 0, 10, 1),
    ///             full_height: 1,
    ///             y_offset: 0,
    ///             clipped_top: false,
    ///             clipped_bottom: false,
    ///         },
    ///         VisibleItem {
    ///             index: 2,
    ///             area: Rect::new(0, 1, 10, 1),
    ///             full_height: 1,
    ///             y_offset: 0,
    ///             clipped_top: false,
    ///             clipped_bottom: false,
    ///         },
    ///     ],
    ///     selected: None,
    /// };
    /// let visible = layout.visible_indices().collect::<Vec<_>>();
    /// let mut selection = SelectionState::new(SelectionMode::Single);
    /// selection.select_next(&visible);
    ///
    /// assert_eq!(selection.primary(), Some(1));
    /// ```
    pub fn visible_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.visible_items.iter().map(|item| item.index)
    }

    /// Converts visible rows into a generic region set keyed by source item index.
    ///
    /// Use this when a virtual list needs to participate in APIs that understand
    /// [`Regions`](crate::regions::Regions) rather than [`ListLayout`]. The list layout remains the
    /// richer source of truth for scroll offsets, measured heights, and clipped row slices; the
    /// returned [`Regions`](crate::regions::Regions) value is the geometry projection used for
    /// generic composition, pointer routing, or diagnostics.
    ///
    /// Clipping metadata is preserved in each [`Region`](crate::regions::Region). A row that starts
    /// above the viewport gets a top clip equal to [`VisibleItem::y_offset`], and a row that
    /// continues below the viewport gets a bottom clip for the hidden tail.
    ///
    /// Use [`ListLayout::rows_regions`] when app code needs stable record ids instead of source
    /// item indexes.
    ///
    /// # Examples
    ///
    /// Merge visible list rows into a larger frame snapshot:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{ListLayout, ScrollPosition, VisibleItem};
    ///
    /// let layout = ListLayout {
    ///     viewport: Rect::new(0, 0, 10, 2),
    ///     item_count: 3,
    ///     content_height: 3,
    ///     scroll_offset: 0,
    ///     scroll: ScrollPosition::default(),
    ///     visible_items: vec![
    ///         VisibleItem {
    ///             index: 0,
    ///             area: Rect::new(0, 0, 10, 1),
    ///             full_height: 1,
    ///             y_offset: 0,
    ///             clipped_top: false,
    ///             clipped_bottom: false,
    ///         },
    ///         VisibleItem {
    ///             index: 1,
    ///             area: Rect::new(0, 1, 10, 1),
    ///             full_height: 1,
    ///             y_offset: 0,
    ///             clipped_top: false,
    ///             clipped_bottom: false,
    ///         },
    ///     ],
    ///     selected: None,
    /// };
    /// let plan = layout.row_regions();
    ///
    /// assert_eq!(plan.hit_test((1, 1)).unwrap().id, 1);
    /// ```
    pub fn row_regions(&self) -> Regions<usize> {
        self.rows_regions(core::convert::identity)
    }

    /// Converts visible rows into a generic region set with caller-chosen row ids.
    ///
    /// Use this when a virtual list renders source indexes but the rest of the app routes input
    /// through semantic ids. For example, a filtered task list can map each source index to a
    /// `TaskId`, then merge the resulting regions into a
    /// [`FrameSnapshot`](crate::frame::FrameSnapshot) or
    /// [`PointerTargets`](crate::pointer::PointerTargets). This keeps row measurement in
    /// [`ListLayout`] while letting outer coordination code speak in application terms.
    ///
    /// The mapper receives the source index from [`VisibleItem::index`]. If an app has filtered or
    /// sorted rows, the mapper should use the same row order as the [`ListItems`] implementation
    /// passed to [`VirtualList::layout`] or [`VirtualList::render`].
    ///
    /// # Examples
    ///
    /// Project source indexes into durable record ids:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{ListLayout, ScrollPosition, VisibleItem};
    ///
    /// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    /// enum TaskId {
    ///     Api,
    ///     Docs,
    /// }
    ///
    /// let ids = [TaskId::Api, TaskId::Docs];
    /// let layout = ListLayout {
    ///     viewport: Rect::new(0, 0, 10, 2),
    ///     item_count: ids.len(),
    ///     content_height: 2,
    ///     scroll_offset: 0,
    ///     scroll: ScrollPosition::default(),
    ///     visible_items: vec![
    ///         VisibleItem {
    ///             index: 0,
    ///             area: Rect::new(0, 0, 10, 1),
    ///             full_height: 1,
    ///             y_offset: 0,
    ///             clipped_top: false,
    ///             clipped_bottom: false,
    ///         },
    ///         VisibleItem {
    ///             index: 1,
    ///             area: Rect::new(0, 1, 10, 1),
    ///             full_height: 1,
    ///             y_offset: 0,
    ///             clipped_top: false,
    ///             clipped_bottom: false,
    ///         },
    ///     ],
    ///     selected: None,
    /// };
    /// let plan = layout.rows_regions(|index| ids[index]);
    ///
    /// assert_eq!(plan.hit_test((1, 1)).unwrap().id, TaskId::Docs);
    /// ```
    pub fn rows_regions<Id>(&self, mut id_for: impl FnMut(usize) -> Id) -> Regions<Id> {
        let regions: Vec<_> = self
            .visible_items
            .iter()
            .map(|item| item.region(id_for(item.index)))
            .collect();
        Regions::from_regions(self.viewport, regions)
    }

    /// Returns the visible item hit by the position.
    ///
    /// The returned id is the source item index. Coordinates are relative to the visible area, not
    /// to the full logical item. If the item is clipped at the top, add the corresponding
    /// [`VisibleItem::y_offset`] if the row renderer needs a logical line coordinate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{ListLayout, ScrollPosition, VisibleItem};
    ///
    /// let layout = ListLayout {
    ///     viewport: Rect::new(0, 0, 10, 2),
    ///     item_count: 1,
    ///     content_height: 2,
    ///     scroll_offset: 0,
    ///     scroll: ScrollPosition::default(),
    ///     visible_items: vec![VisibleItem {
    ///         index: 7,
    ///         area: Rect::new(0, 0, 10, 2),
    ///         full_height: 2,
    ///         y_offset: 0,
    ///         clipped_top: false,
    ///         clipped_bottom: false,
    ///     }],
    ///     selected: None,
    /// };
    ///
    /// assert_eq!(layout.hit_test((2, 1)).unwrap().id, 7);
    /// ```
    pub fn hit_test<P: Into<Position>>(&self, position: P) -> Option<Hit<usize>> {
        let position = position.into();
        self.visible_items
            .iter()
            .rev()
            .find(|item| item.area.contains(position))
            .map(|item| Hit {
                id: item.index,
                area: item.area,
                relative_x: position.x.saturating_sub(item.area.x),
                relative_y: position.y.saturating_sub(item.area.y),
            })
    }

    /// Returns the source item index hit by the position.
    ///
    /// This is the common click path when an app only needs to know which row was clicked. Use
    /// [`ListLayout::hit_test`] when the row also needs local pointer coordinates, for example to
    /// distinguish a disclosure icon from the rest of a tree row.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{ListLayout, ScrollPosition, VisibleItem};
    ///
    /// let layout = ListLayout {
    ///     viewport: Rect::new(0, 0, 10, 1),
    ///     item_count: 1,
    ///     content_height: 1,
    ///     scroll_offset: 0,
    ///     scroll: ScrollPosition::default(),
    ///     visible_items: vec![VisibleItem {
    ///         index: 3,
    ///         area: Rect::new(0, 0, 10, 1),
    ///         full_height: 1,
    ///         y_offset: 0,
    ///         clipped_top: false,
    ///         clipped_bottom: false,
    ///     }],
    ///     selected: None,
    /// };
    ///
    /// assert_eq!(layout.hit_index((4, 0)), Some(3));
    /// ```
    pub fn hit_index<P: Into<Position>>(&self, position: P) -> Option<usize> {
        self.hit_test(position).map(|hit| hit.id)
    }

    /// Selects the durable id for the row hit by the position.
    ///
    /// Use this in filtered or sorted lists where commands should target a stable record id, while
    /// row rendering still uses the source index required by [`VirtualList`]. The `ids` slice must
    /// be indexed the same way as the [`ListItems`] implementation used for the layout. Invalid
    /// positions, blank space, or stale indexes leave `selection` unchanged and return `None`.
    ///
    /// Use [`ListLayout::hit_index`] when selection is only an index, or [`ListLayout::hit_test`]
    /// when a row needs local coordinates.
    ///
    /// # Examples
    ///
    /// Click a filtered row and keep the selected task id:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{ListLayout, ScrollPosition, VisibleItem};
    /// use ratatui_layout::selection::VisibleSelection;
    ///
    /// let ids = ["api", "docs"];
    /// let layout = ListLayout {
    ///     viewport: Rect::new(0, 0, 10, 2),
    ///     item_count: ids.len(),
    ///     content_height: 2,
    ///     scroll_offset: 0,
    ///     scroll: ScrollPosition::default(),
    ///     visible_items: vec![
    ///         VisibleItem {
    ///             index: 0,
    ///             area: Rect::new(0, 0, 10, 1),
    ///             full_height: 1,
    ///             y_offset: 0,
    ///             clipped_top: false,
    ///             clipped_bottom: false,
    ///         },
    ///         VisibleItem {
    ///             index: 1,
    ///             area: Rect::new(0, 1, 10, 1),
    ///             full_height: 1,
    ///             y_offset: 0,
    ///             clipped_top: false,
    ///             clipped_bottom: false,
    ///         },
    ///     ],
    ///     selected: None,
    /// };
    /// let mut selection = VisibleSelection::new();
    ///
    /// assert_eq!(layout.select_hit((2, 1), &mut selection, &ids), Some(1));
    /// assert_eq!(selection.selected_id(), Some("docs"));
    /// ```
    pub fn select_hit<Id: Copy + Eq, P: Into<Position>>(
        &self,
        position: P,
        selection: &mut VisibleSelection<Id>,
        ids: &[Id],
    ) -> Option<usize> {
        let index = self.hit_index(position)?;
        selection.select_index(index, ids)
    }
}

impl VisibleItem {
    const fn region<Id>(self, id: Id) -> Region<Id> {
        Region::new(id, self.area).clip(self.clip())
    }

    const fn clip(self) -> Clip {
        Clip {
            left: 0,
            top: if self.clipped_top { self.y_offset } else { 0 },
            right: 0,
            bottom: if self.clipped_bottom {
                self.full_height
                    .saturating_sub(self.y_offset.saturating_add(self.area.height))
            } else {
                0
            },
        }
    }
}

/// A vertical virtual list that asks external items to measure and render.
///
/// Use [`VirtualList`] when the built-in text list is the wrong ownership shape: rows are generated
/// lazily, rows have custom renderers, row state belongs to the application, or row height depends
/// on the final width. The list owns viewport policy; the application owns the rows.
///
/// [`VirtualList`] is a configuration value. It can be rebuilt each frame, like other Ratatui
/// widgets. Persistent selection and scroll live in [`VirtualListState`].
///
/// # Constructors and setters
///
/// - [`VirtualList::new`] creates default list configuration.
/// - [`VirtualList::scroll_padding`] asks layout to keep space around the selected item when
///   possible.
///
/// # Layout and rendering
///
/// - [`VirtualList::layout`] computes [`ListLayout`] without drawing.
/// - [`VirtualList::layout_cached`] computes [`ListLayout`] with explicit height caching.
/// - [`VirtualList::render`] computes layout, renders visible rows, and returns the layout.
/// - [`VirtualList::render_cached`] renders with explicit height caching.
///
/// # Examples
///
/// Render visible rows, keep the returned layout, and use it to route a later click:
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::list::{ListItemContext, ListItems, VirtualList, VirtualListState};
/// use ratatui_layout::participant::MeasureContext;
///
/// struct Rows;
/// impl ListItems for Rows {
///     fn len(&self) -> usize {
///         4
///     }
///     fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
///         1
///     }
///     fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
/// }
///
/// let mut rows = Rows;
/// let mut state = VirtualListState::default();
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 2));
/// let layout = VirtualList::new().render(buffer.area, &mut buffer, &mut state, &mut rows);
///
/// assert_eq!(layout.visible_items.len(), 2);
/// assert_eq!(layout.hit_test((0, 1)).unwrap().id, 1);
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct VirtualList {
    scroll_padding: u16,
}

impl VirtualList {
    /// Creates a virtual list.
    ///
    /// # Examples
    ///
    /// Compute visible rows without rendering when the app wants custom drawing or hit testing:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{ListItemContext, ListItems, VirtualList, VirtualListState};
    /// use ratatui_layout::participant::MeasureContext;
    ///
    /// struct Rows;
    /// impl ListItems for Rows {
    ///     fn len(&self) -> usize {
    ///         3
    ///     }
    ///     fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
    ///         1
    ///     }
    ///     fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
    /// }
    ///
    /// let mut state = VirtualListState::default();
    /// let layout = VirtualList::new().layout(Rect::new(0, 0, 10, 2), &mut state, &Rows);
    ///
    /// assert_eq!(layout.visible_items.len(), 2);
    /// ```
    pub const fn new() -> Self {
        Self { scroll_padding: 0 }
    }

    /// Sets scroll padding in lines.
    ///
    /// Scroll padding asks the layout to keep this many lines visible before and after the selected
    /// item when possible. Padding is reduced naturally when the viewport is too small or the
    /// selected item is too tall.
    ///
    /// # Examples
    ///
    /// Keep context around the selected row when scrolling a long list:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{ListItemContext, ListItems, VirtualList, VirtualListState};
    /// use ratatui_layout::participant::MeasureContext;
    ///
    /// struct Rows;
    /// impl ListItems for Rows {
    ///     fn len(&self) -> usize {
    ///         10
    ///     }
    ///     fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
    ///         1
    ///     }
    ///     fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
    /// }
    ///
    /// let mut state = VirtualListState::default();
    /// state.select(Some(8));
    /// VirtualList::new()
    ///     .scroll_padding(1)
    ///     .layout(Rect::new(0, 0, 10, 3), &mut state, &Rows);
    ///
    /// assert_eq!(state.scroll().index, 7);
    /// ```
    #[must_use = "method returns the modified value"]
    pub const fn scroll_padding(mut self, scroll_padding: u16) -> Self {
        self.scroll_padding = scroll_padding;
        self
    }

    /// Computes the list layout and updates state with clamped scroll and selection values.
    ///
    /// This method does not render. Use it when the caller wants to inspect the visible rows, route
    /// input, build a scrollbar, or perform custom rendering with the returned [`ListLayout`].
    ///
    /// The method may mutate `state` by clamping selection and scroll to the current item count and
    /// measured heights. If the list is empty or the area is empty, selection is cleared and scroll
    /// is reset.
    ///
    /// # Examples
    ///
    /// Compute a layout for hit testing and scrollbar metrics without drawing rows:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{ListItemContext, ListItems, VirtualList, VirtualListState};
    /// use ratatui_layout::participant::MeasureContext;
    ///
    /// struct Rows;
    /// impl ListItems for Rows {
    ///     fn len(&self) -> usize {
    ///         4
    ///     }
    ///     fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
    ///         1
    ///     }
    ///     fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
    /// }
    ///
    /// let mut state = VirtualListState::default();
    /// let layout = VirtualList::new().layout(Rect::new(0, 0, 10, 2), &mut state, &Rows);
    ///
    /// assert_eq!(layout.hit_test((1, 1)).unwrap().id, 1);
    /// ```
    pub fn layout<I: ListItems>(
        self,
        area: Rect,
        state: &mut VirtualListState,
        items: &I,
    ) -> ListLayout {
        let item_count = items.len();
        let heights = measure_heights(items, item_count, area.width);
        self.layout_with_heights(area, state, item_count, &heights)
    }

    /// Computes the list layout using a measurement cache.
    ///
    /// Use this when item heights are expensive to compute and the list is rebuilt each frame with
    /// the same width and item count. The cache is explicit application state so invalidation stays
    /// visible.
    ///
    /// # Examples
    ///
    /// Reuse measured heights across frames while keeping invalidation in app state:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{
    ///     ListHeightCache, ListItemContext, ListItems, VirtualList, VirtualListState,
    /// };
    /// use ratatui_layout::participant::MeasureContext;
    ///
    /// struct Rows;
    /// impl ListItems for Rows {
    ///     fn len(&self) -> usize {
    ///         3
    ///     }
    ///     fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
    ///         2
    ///     }
    ///     fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
    /// }
    ///
    /// let mut state = VirtualListState::default();
    /// let mut cache = ListHeightCache::new();
    /// let layout =
    ///     VirtualList::new().layout_cached(Rect::new(0, 0, 10, 3), &mut state, &Rows, &mut cache);
    ///
    /// assert_eq!(layout.content_height, 6);
    /// ```
    pub fn layout_cached<I: ListItems>(
        self,
        area: Rect,
        state: &mut VirtualListState,
        items: &I,
        cache: &mut ListHeightCache,
    ) -> ListLayout {
        let item_count = items.len();
        let heights = cache.heights_for(items, item_count, area.width);
        self.layout_with_heights(area, state, item_count, heights)
    }

    fn layout_with_heights(
        self,
        area: Rect,
        state: &mut VirtualListState,
        item_count: usize,
        heights: &[u16],
    ) -> ListLayout {
        if area.is_empty() || item_count == 0 {
            state.select(None);
            state.set_scroll(ScrollPosition::default());
            return ListLayout {
                viewport: area,
                item_count,
                content_height: 0,
                scroll_offset: 0,
                scroll: state.scroll(),
                visible_items: Vec::new(),
                selected: None,
            };
        }

        if let Some(selected) = state.selected
            && selected >= item_count
        {
            state.selected = Some(item_count - 1);
        }

        let content_height = heights.iter().map(|height| u32::from(*height)).sum();
        state.scroll = clamp_scroll(state.scroll, heights);
        if let Some(selected) = state.selected
            && state.scroll_selected_into_view
        {
            state.scroll = scroll_to_selected(
                state.scroll,
                selected,
                heights,
                area.height,
                self.scroll_padding,
            );
        }

        let visible_items = visible_items(area, state.scroll, heights);
        let selected = state.selected.and_then(|selected| {
            visible_items
                .iter()
                .find(|item| item.index == selected)
                .copied()
        });

        ListLayout {
            viewport: area,
            item_count,
            content_height,
            scroll_offset: item_top(state.scroll.index, heights) + u32::from(state.scroll.y_offset),
            scroll: state.scroll,
            visible_items,
            selected,
        }
    }

    /// Computes and renders the list.
    ///
    /// This is the high-level convenience method. It computes a [`ListLayout`], calls
    /// [`ListItems::render_item`] for each visible item, and returns the layout so the caller can
    /// still inspect what happened during the frame.
    ///
    /// # Examples
    ///
    /// Render visible rows and keep the layout for the next input event:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_core::text::Line;
    /// use ratatui_core::widgets::Widget;
    /// use ratatui_layout::list::{ListItemContext, ListItems, VirtualList, VirtualListState};
    /// use ratatui_layout::participant::MeasureContext;
    ///
    /// struct Rows(&'static [&'static str]);
    /// impl ListItems for Rows {
    ///     fn len(&self) -> usize {
    ///         self.0.len()
    ///     }
    ///     fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
    ///         1
    ///     }
    ///     fn render_item(&mut self, index: usize, area: Rect, buf: &mut Buffer, _: ListItemContext) {
    ///         Line::from(self.0[index]).render(area, buf);
    ///     }
    /// }
    ///
    /// let mut rows = Rows(&["open", "save"]);
    /// let mut state = VirtualListState::default();
    /// let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 2));
    /// let layout = VirtualList::new().render(buffer.area, &mut buffer, &mut state, &mut rows);
    ///
    /// assert_eq!(layout.visible_items.len(), 2);
    /// ```
    pub fn render<I: ListItems>(
        self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut VirtualListState,
        items: &mut I,
    ) -> ListLayout {
        let layout = self.layout(area, state, items);
        render_visible_items(&layout, state, items, buf);
        layout
    }

    /// Computes and renders the list using a measurement cache.
    ///
    /// # Examples
    ///
    /// Render a variable-height list while reusing row measurements:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{
    ///     ListHeightCache, ListItemContext, ListItems, VirtualList, VirtualListState,
    /// };
    /// use ratatui_layout::participant::MeasureContext;
    ///
    /// struct Rows;
    /// impl ListItems for Rows {
    ///     fn len(&self) -> usize {
    ///         2
    ///     }
    ///     fn height_for_width(&self, index: usize, _: u16, _: MeasureContext) -> u16 {
    ///         (index + 1) as u16
    ///     }
    ///     fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
    /// }
    ///
    /// let mut rows = Rows;
    /// let mut state = VirtualListState::default();
    /// let mut cache = ListHeightCache::new();
    /// let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 3));
    /// let layout = VirtualList::new().render_cached(
    ///     buffer.area,
    ///     &mut buffer,
    ///     &mut state,
    ///     &mut rows,
    ///     &mut cache,
    /// );
    ///
    /// assert_eq!(layout.content_height, 3);
    /// ```
    pub fn render_cached<I: ListItems>(
        self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut VirtualListState,
        items: &mut I,
        cache: &mut ListHeightCache,
    ) -> ListLayout {
        let layout = self.layout_cached(area, state, items, cache);
        render_visible_items(&layout, state, items, buf);
        layout
    }
}

fn render_visible_items<I: ListItems>(
    layout: &ListLayout,
    state: &VirtualListState,
    items: &mut I,
    buf: &mut Buffer,
) {
    for (visible_index, item) in layout.visible_items.iter().copied().enumerate() {
        let selected = state.selected == Some(item.index);
        items.render_item(
            item.index,
            item.area,
            buf,
            item.context(visible_index, selected),
        );
    }
}

/// Explicit height cache for [`VirtualList`].
///
/// Use `ListHeightCache` when measuring row height is non-trivial. The cache is keyed by final
/// width and item count; apps can invalidate all heights or selected ranges when row content
/// changes.
///
/// # Constructors and invalidation
///
/// - [`ListHeightCache::new`] creates an empty cache.
/// - [`ListHeightCache::clear`] drops all measured heights.
/// - [`ListHeightCache::invalidate`] marks one item dirty.
/// - [`ListHeightCache::invalidate_range`] marks a range of items dirty.
///
/// # Examples
///
/// Keep the cache beside list state, then invalidate affected rows when app data changes:
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::list::{
///     ListHeightCache, ListItemContext, ListItems, VirtualList, VirtualListState,
/// };
/// use ratatui_layout::participant::MeasureContext;
///
/// struct Rows;
/// impl ListItems for Rows {
///     fn len(&self) -> usize {
///         3
///     }
///     fn height_for_width(&self, index: usize, _: u16, _: MeasureContext) -> u16 {
///         index as u16 + 1
///     }
///     fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
/// }
///
/// let mut state = VirtualListState::default();
/// let mut cache = ListHeightCache::new();
/// let layout =
///     VirtualList::new().layout_cached(Rect::new(0, 0, 10, 4), &mut state, &Rows, &mut cache);
/// cache.invalidate(1);
///
/// assert_eq!(layout.content_height, 6);
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListHeightCache {
    width: Option<u16>,
    item_count: usize,
    heights: Vec<u16>,
    dirty: Vec<bool>,
}

impl ListHeightCache {
    /// Creates an empty height cache.
    ///
    /// # Examples
    ///
    /// Store the cache beside list state when row measurement is expensive:
    ///
    /// ```rust
    /// use ratatui_layout::list::ListHeightCache;
    ///
    /// let cache = ListHeightCache::new();
    ///
    /// assert_eq!(cache, ListHeightCache::default());
    /// ```
    pub const fn new() -> Self {
        Self {
            width: None,
            item_count: 0,
            heights: Vec::new(),
            dirty: Vec::new(),
        }
    }

    /// Invalidates all cached heights.
    ///
    /// # Examples
    ///
    /// Clear all cached heights when a global display setting changes:
    ///
    /// ```rust
    /// use ratatui_layout::list::ListHeightCache;
    ///
    /// let mut cache = ListHeightCache::new();
    /// cache.clear();
    ///
    /// assert_eq!(cache, ListHeightCache::new());
    /// ```
    pub fn clear(&mut self) {
        self.width = None;
        self.item_count = 0;
        self.heights.clear();
        self.dirty.clear();
    }

    /// Invalidates one item height.
    ///
    /// # Examples
    ///
    /// Mark one edited row dirty before the next cached layout pass:
    ///
    /// ```rust
    /// use ratatui_layout::list::ListHeightCache;
    ///
    /// let mut cache = ListHeightCache::new();
    /// cache.invalidate(3);
    /// ```
    pub fn invalidate(&mut self, index: usize) {
        if let Some(dirty) = self.dirty.get_mut(index) {
            *dirty = true;
        }
    }

    /// Invalidates a half-open item range.
    ///
    /// # Examples
    ///
    /// Mark the rows affected by a batch update dirty:
    ///
    /// ```rust
    /// use ratatui_layout::list::ListHeightCache;
    ///
    /// let mut cache = ListHeightCache::new();
    /// cache.invalidate_range(10..20);
    /// ```
    pub fn invalidate_range(&mut self, range: core::ops::Range<usize>) {
        for index in range {
            self.invalidate(index);
        }
    }

    fn heights_for<I: ListItems>(&mut self, items: &I, item_count: usize, width: u16) -> &[u16] {
        self.prepare(item_count, width);
        for index in 0..item_count {
            if self.dirty[index] {
                self.heights[index] = items.height_for_width(index, width, MeasureContext).max(1);
                self.dirty[index] = false;
            }
        }
        &self.heights
    }

    fn prepare(&mut self, item_count: usize, width: u16) {
        if self.width != Some(width) || self.item_count != item_count {
            self.width = Some(width);
            self.item_count = item_count;
            self.heights.clear();
            self.heights.resize(item_count, 1);
            self.dirty.clear();
            self.dirty.resize(item_count, true);
        }
    }
}

fn measure_heights<I: ListItems>(items: &I, item_count: usize, width: u16) -> Vec<u16> {
    (0..item_count)
        .map(|index| items.height_for_width(index, width, MeasureContext).max(1))
        .collect()
}

fn clamp_scroll(scroll: ScrollPosition, heights: &[u16]) -> ScrollPosition {
    let Some(height) = heights.get(scroll.index).copied() else {
        return ScrollPosition::new(heights.len().saturating_sub(1), 0);
    };
    ScrollPosition::new(scroll.index, scroll.y_offset.min(height.saturating_sub(1)))
}

fn item_top(index: usize, heights: &[u16]) -> u32 {
    heights[..index]
        .iter()
        .map(|height| u32::from(*height))
        .sum()
}

fn scroll_to_line(line: u32, heights: &[u16]) -> ScrollPosition {
    let mut remaining = line;
    for (index, height) in heights.iter().copied().enumerate() {
        let height = u32::from(height);
        if remaining < height {
            return ScrollPosition::new(index, remaining as u16);
        }
        remaining = remaining.saturating_sub(height);
    }
    ScrollPosition::new(heights.len().saturating_sub(1), 0)
}

fn scroll_to_selected(
    scroll: ScrollPosition,
    selected: usize,
    heights: &[u16],
    viewport_height: u16,
    scroll_padding: u16,
) -> ScrollPosition {
    let viewport_height = u32::from(viewport_height);
    let padding = u32::from(scroll_padding).min(viewport_height.saturating_sub(1) / 2);
    let scroll_line = item_top(scroll.index, heights) + u32::from(scroll.y_offset);
    let selected_top = item_top(selected, heights);
    let selected_bottom = selected_top + u32::from(heights[selected]);

    if selected_top < scroll_line + padding {
        return scroll_to_line(selected_top.saturating_sub(padding), heights);
    }

    if selected_bottom + padding > scroll_line + viewport_height {
        let target = selected_bottom
            .saturating_add(padding)
            .saturating_sub(viewport_height);
        return scroll_to_line(target, heights);
    }

    scroll
}

fn visible_items(area: Rect, scroll: ScrollPosition, heights: &[u16]) -> Vec<VisibleItem> {
    let mut items = Vec::new();
    let mut index = scroll.index;
    let mut y = area.y;
    let mut remaining_height = area.height;
    let mut y_offset = scroll.y_offset;

    while remaining_height > 0 {
        let Some(full_height) = heights.get(index).copied() else {
            break;
        };
        let available_item_height = full_height.saturating_sub(y_offset);
        let visible_height = available_item_height.min(remaining_height);
        if visible_height == 0 {
            break;
        }

        items.push(VisibleItem {
            index,
            area: Rect::new(area.x, y, area.width, visible_height),
            full_height,
            y_offset,
            clipped_top: y_offset > 0,
            clipped_bottom: y_offset + visible_height < full_height,
        });

        y = y.saturating_add(visible_height);
        remaining_height = remaining_height.saturating_sub(visible_height);
        index += 1;
        y_offset = 0;
    }

    items
}

const fn offset_index(index: usize, delta: isize) -> usize {
    if delta.is_negative() {
        index.saturating_sub(delta.unsigned_abs())
    } else {
        index.saturating_add(delta as usize)
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use pretty_assertions::assert_eq;
    use ratatui_core::buffer::Buffer;
    use ratatui_core::layout::Rect;

    use super::*;

    struct Heights(Vec<u16>);

    impl ListItems for Heights {
        fn len(&self) -> usize {
            self.0.len()
        }

        fn height_for_width(&self, index: usize, _: u16, _: MeasureContext) -> u16 {
            self.0[index]
        }

        fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
    }

    #[test]
    fn empty_list_resets_state() {
        let mut state = VirtualListState {
            selected: Some(3),
            scroll: ScrollPosition::new(2, 1),
            scroll_selected_into_view: true,
        };
        let layout =
            VirtualList::new().layout(Rect::new(0, 0, 10, 5), &mut state, &Heights(vec![]));

        assert_eq!(state.selected(), None);
        assert_eq!(state.scroll(), ScrollPosition::default());
        assert!(layout.visible_items.is_empty());
    }

    #[test]
    fn supports_line_aware_scroll_inside_item() {
        let mut state = VirtualListState::default();
        state.set_scroll(ScrollPosition::new(0, 2));
        let layout =
            VirtualList::new().layout(Rect::new(0, 0, 10, 4), &mut state, &Heights(vec![5, 2]));

        assert_eq!(
            layout.visible_items,
            vec![
                VisibleItem {
                    index: 0,
                    area: Rect::new(0, 0, 10, 3),
                    full_height: 5,
                    y_offset: 2,
                    clipped_top: true,
                    clipped_bottom: false,
                },
                VisibleItem {
                    index: 1,
                    area: Rect::new(0, 3, 10, 1),
                    full_height: 2,
                    y_offset: 0,
                    clipped_top: false,
                    clipped_bottom: true,
                },
            ]
        );
    }

    #[test]
    fn keeps_selected_visible_with_line_padding() {
        let mut state = VirtualListState::default();
        state.select(Some(3));
        let layout = VirtualList::new().scroll_padding(1).layout(
            Rect::new(0, 0, 10, 3),
            &mut state,
            &Heights(vec![1, 1, 1, 1, 1]),
        );

        assert_eq!(state.scroll(), ScrollPosition::new(2, 0));
        assert_eq!(
            layout
                .visible_items
                .iter()
                .map(|item| item.index)
                .collect::<Vec<_>>(),
            vec![2, 3, 4]
        );
    }

    #[test]
    fn relative_selection_clamps_to_item_count() {
        let mut state = VirtualListState::default();

        assert_eq!(state.select_relative(1, 3), Some(0));
        assert_eq!(state.selected(), Some(0));
        assert!(state.scrolls_selected_into_view());

        assert_eq!(state.select_relative(1, 3), Some(1));
        assert_eq!(state.select_relative(99, 3), Some(2));
        assert_eq!(state.select_relative(-99, 3), Some(0));

        assert_eq!(state.select_relative(1, 0), None);
        assert_eq!(state.selected(), None);
    }

    #[test]
    fn viewport_scroll_does_not_snap_back_to_selection() {
        let mut state = VirtualListState::default();
        state.select(Some(0));
        state.scroll_viewport_by(3);
        let layout = VirtualList::new().layout(
            Rect::new(0, 0, 10, 2),
            &mut state,
            &Heights(vec![1, 1, 1, 1, 1]),
        );

        assert_eq!(state.selected(), Some(0));
        assert_eq!(state.scroll(), ScrollPosition::new(3, 0));
        assert_eq!(
            layout
                .visible_items
                .iter()
                .map(|item| item.index)
                .collect::<Vec<_>>(),
            vec![3, 4]
        );
        assert_eq!(layout.selected, None);
    }

    #[test]
    fn hit_test_returns_item_relative_coordinates() {
        let mut state = VirtualListState::default();
        let layout =
            VirtualList::new().layout(Rect::new(2, 3, 10, 4), &mut state, &Heights(vec![2, 2]));

        assert_eq!(
            layout.hit_test((4, 6)),
            Some(Hit {
                id: 1,
                area: Rect::new(2, 5, 10, 2),
                relative_x: 2,
                relative_y: 1,
            })
        );
    }

    #[test]
    fn row_plan_preserves_visible_row_geometry_and_clipping() {
        let mut state = VirtualListState::default();
        state.set_scroll(ScrollPosition::new(0, 2));
        let layout =
            VirtualList::new().layout(Rect::new(2, 3, 10, 4), &mut state, &Heights(vec![5, 3]));
        let plan = layout.row_regions();

        assert_eq!(plan.area(), Rect::new(2, 3, 10, 4));
        assert_eq!(plan.hit_test((4, 6)).unwrap().id, 1);
        assert_eq!(plan.regions()[0].id, 0);
        assert_eq!(plan.regions()[0].area, Rect::new(2, 3, 10, 3));
        assert_eq!(plan.regions()[0].clip.top, 2);
        assert_eq!(plan.regions()[1].id, 1);
        assert_eq!(plan.regions()[1].area, Rect::new(2, 6, 10, 1));
        assert_eq!(plan.regions()[1].clip.bottom, 2);
    }

    #[test]
    fn rows_plan_maps_source_indexes_to_app_ids() {
        let ids = ["api", "docs"];
        let mut state = VirtualListState::default();
        let layout =
            VirtualList::new().layout(Rect::new(0, 0, 10, 2), &mut state, &Heights(vec![1, 1]));
        let plan = layout.rows_regions(|index| ids[index]);

        assert_eq!(plan.hit_test((1, 0)).unwrap().id, "api");
        assert_eq!(plan.hit_test((1, 1)).unwrap().id, "docs");
    }

    #[test]
    fn hit_index_and_select_hit_cover_common_click_paths() {
        let ids = ["api", "docs"];
        let mut state = VirtualListState::default();
        let layout =
            VirtualList::new().layout(Rect::new(0, 0, 10, 2), &mut state, &Heights(vec![1, 1]));
        let mut selection = VisibleSelection::new();

        assert_eq!(layout.hit_index((1, 1)), Some(1));
        assert_eq!(layout.select_hit((1, 1), &mut selection, &ids), Some(1));
        assert_eq!(selection.selected_id(), Some("docs"));
        assert_eq!(layout.select_hit((1, 3), &mut selection, &ids), None);
        assert_eq!(selection.selected_id(), Some("docs"));
    }
}
