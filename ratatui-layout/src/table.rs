//! Virtual table layout and app-owned cell rendering.
//!
//! `VirtualTable` is a two-dimensional counterpart to [`crate::list::VirtualList`]. It does not
//! store rows, cells, styles, or cell widget state. The application owns the data and renders
//! visible cells by row and column after the table computes the current visible regions.
//!
//! Use Ratatui's built-in table widget when rows are small, fully visible, and can be rendered
//! directly. Use this module when pinned headers, two-axis scrolling, hit testing, or app-owned
//! cell rendering need explicit frame-local data.
//!
//! # Types and traits
//!
//! - [`CellPosition`] is the frame-local id for header and body cells.
//! - [`VirtualTableState`] stores selected cell plus row and column scroll offsets.
//! - [`TableItems`] is the trait for app-owned table data that can render visible cells.
//! - [`TableCellContext`] is passed to cell renderers with interaction and visible-position
//!   metadata.
//! - [`VisibleCell`] describes one header or body cell visible in the current frame.
//! - [`TableLayout`] is the solved frame-local layout for visible cells, hit testing, and scroll
//!   metrics.
//! - [`VirtualTable`] is the reusable table configuration that computes or renders a
//!   [`TableLayout`].
//!
//! See [`crate::docs::virtualization`] for the broader virtualization model and
//! [`crate::docs::interaction`] for how table cell ids can feed focus, pointer, and selection
//! state.
//!
//! # Examples
//!
//! Render visible cells from app-owned data and keep the solved layout for hit testing:
//!
//! ```rust
//! use ratatui_core::buffer::Buffer;
//! use ratatui_core::layout::{Constraint, Rect};
//! use ratatui_layout::table::{
//!     CellPosition, TableCellContext, TableItems, VirtualTable, VirtualTableState,
//! };
//!
//! struct Rows(&'static [[&'static str; 2]]);
//! impl TableItems for Rows {
//!     fn row_count(&self) -> usize {
//!         self.0.len()
//!     }
//!     fn column_count(&self) -> usize {
//!         2
//!     }
//!     fn render_cell(&mut self, _: CellPosition, _: Rect, _: &mut Buffer, _: TableCellContext) {}
//! }
//!
//! let mut rows = Rows(&[["id", "name"], ["1", "Ada"]]);
//! let mut state = VirtualTableState::default();
//! let mut buffer = Buffer::empty(Rect::new(0, 0, 12, 3));
//! let layout = VirtualTable::new([Constraint::Length(6), Constraint::Length(6)]).render(
//!     buffer.area,
//!     &mut buffer,
//!     &mut state,
//!     &mut rows,
//! );
//!
//! assert_eq!(
//!     layout.hit_test((7, 1)).unwrap().id,
//!     CellPosition::body(0, 1)
//! );
//! ```

use alloc::vec::Vec;

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Constraint, Layout, Position, Rect};

use crate::participant::{RenderContext, RenderState};
use crate::regions::{Hit, Region, Regions};
use crate::scroll::ScrollMetrics;

/// A table cell coordinate.
///
/// Use [`CellPosition`] as the stable frame-local id for a virtual table cell. Header cells use
/// `row: None`; body cells use `row: Some(index)`.
///
/// # Constructors
///
/// - [`CellPosition::body`] creates a source body-cell id.
/// - [`CellPosition::header`] creates a pinned header-cell id.
///
/// # Examples
///
/// Use one id type for both pinned headers and scrollable body cells:
///
/// ```rust
/// use ratatui_layout::table::{CellPosition, VirtualTableState};
///
/// let header = CellPosition::header(0);
/// let body = CellPosition::body(3, 0);
/// let mut state = VirtualTableState::default();
/// state.select(Some(body));
///
/// assert_eq!(header.row, None);
/// assert_eq!(
///     state.selected(),
///     Some(CellPosition {
///         row: Some(3),
///         column: 0
///     })
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CellPosition {
    /// Body row index, or `None` for a header cell.
    pub row: Option<usize>,

    /// Column index.
    pub column: usize,
}

impl CellPosition {
    /// Creates a body cell position.
    ///
    /// # Examples
    ///
    /// Store a selected source cell in table state:
    ///
    /// ```rust
    /// use ratatui_layout::table::{CellPosition, VirtualTableState};
    ///
    /// let mut state = VirtualTableState::default();
    /// state.select(Some(CellPosition::body(3, 2)));
    ///
    /// assert_eq!(
    ///     state.selected(),
    ///     Some(CellPosition {
    ///         row: Some(3),
    ///         column: 2
    ///     })
    /// );
    /// ```
    pub const fn body(row: usize, column: usize) -> Self {
        Self {
            row: Some(row),
            column,
        }
    }

    /// Creates a header cell position.
    ///
    /// # Examples
    ///
    /// Distinguish pinned header hits from body-cell hits:
    ///
    /// ```rust
    /// use ratatui_layout::table::CellPosition;
    ///
    /// let header = CellPosition::header(1);
    ///
    /// assert_eq!(header.row, None);
    /// assert_eq!(header.column, 1);
    /// ```
    pub const fn header(column: usize) -> Self {
        Self { row: None, column }
    }
}

/// Persistent state for a virtual table.
///
/// Use [`VirtualTableState`] next to the table's data. It stores selection plus vertical and
/// horizontal scroll positions. [`VirtualTable`] clamps this state during layout.
///
/// # Selection
///
/// - [`VirtualTableState::selected`] reads the selected cell.
/// - [`VirtualTableState::select`] sets or clears the selected cell and asks layout to reveal it.
/// - [`VirtualTableState::select_relative`] moves selection by row and column deltas and asks
///   layout to reveal it.
/// - [`VirtualTableState::select_without_scrolling`] sets selection for styling or commands without
///   moving the viewport.
/// - [`VirtualTableState::scroll_to_selected`] asks the next layout to reveal the selected cell.
/// - [`VirtualTableState::scrolls_selected_into_view`] reports which selection policy the next
///   layout pass will use.
///
/// # Scrolling
///
/// - [`VirtualTableState::row_scroll`] and [`VirtualTableState::set_row_scroll`] read and write the
///   first visible body row.
/// - [`VirtualTableState::column_scroll`] and [`VirtualTableState::set_column_scroll`] read and
///   write the first visible column.
/// - [`VirtualTableState::scroll_rows_by`], [`VirtualTableState::scroll_columns_by`], and
///   [`VirtualTableState::scroll_viewport_by`] move the viewport without changing selection.
///
/// # Examples
///
/// Store two-axis table state between frames while rebuilding [`VirtualTable`] as a value:
///
/// ```rust
/// use ratatui_layout::table::{CellPosition, VirtualTableState};
///
/// let mut state = VirtualTableState::default();
/// state.select(Some(CellPosition::body(8, 3)));
/// state.set_row_scroll(6);
/// state.set_column_scroll(2);
///
/// assert_eq!(state.selected(), Some(CellPosition::body(8, 3)));
/// assert_eq!((state.row_scroll(), state.column_scroll()), (6, 2));
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VirtualTableState {
    selected: Option<CellPosition>,
    row_scroll: usize,
    column_scroll: usize,
    #[cfg_attr(feature = "serde", serde(default))]
    scroll_selected_into_view: bool,
}

impl VirtualTableState {
    /// Returns the selected body cell.
    ///
    /// # Examples
    ///
    /// Read the selected cell when routing a keyboard command:
    ///
    /// ```rust
    /// use ratatui_layout::table::{CellPosition, VirtualTableState};
    ///
    /// let mut state = VirtualTableState::default();
    /// state.select(Some(CellPosition::body(1, 0)));
    ///
    /// assert_eq!(state.selected(), Some(CellPosition::body(1, 0)));
    /// ```
    pub const fn selected(&self) -> Option<CellPosition> {
        self.selected
    }

    /// Returns whether layout should keep the selected body cell visible.
    ///
    /// This flag lets callers distinguish keyboard/click selection from viewport-only scrolling.
    /// Calling [`VirtualTableState::select`] turns it on. Calling
    /// [`VirtualTableState::scroll_rows_by`], [`VirtualTableState::scroll_columns_by`], or
    /// [`VirtualTableState::scroll_viewport_by`] turns it off so wheel input can move the viewport
    /// without snapping back to the selected cell.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::table::{CellPosition, VirtualTableState};
    ///
    /// let mut state = VirtualTableState::default();
    /// state.select(Some(CellPosition::body(4, 1)));
    /// assert!(state.scrolls_selected_into_view());
    ///
    /// state.scroll_rows_by(1);
    /// assert!(!state.scrolls_selected_into_view());
    /// ```
    pub const fn scrolls_selected_into_view(&self) -> bool {
        self.scroll_selected_into_view
    }

    /// Sets the selected body cell.
    ///
    /// Selecting a body cell also asks the next layout pass to keep that cell visible. Use
    /// [`VirtualTableState::select_without_scrolling`] when selection should be retained for
    /// styling or commands but viewport-only input should not move back to it.
    ///
    /// # Examples
    ///
    /// Select the clicked cell after hit testing a previous table layout:
    ///
    /// ```rust
    /// use ratatui_layout::table::{CellPosition, VirtualTableState};
    ///
    /// let mut state = VirtualTableState::default();
    /// state.select(Some(CellPosition::body(2, 3)));
    ///
    /// assert_eq!(state.selected().unwrap().column, 3);
    /// ```
    pub const fn select(&mut self, selected: Option<CellPosition>) {
        self.selected = selected;
        self.scroll_selected_into_view = selected.is_some();
    }

    /// Moves selected body cell by signed row and column deltas.
    ///
    /// Use this for keyboard movement in source-index tables. The caller passes current row and
    /// column counts because table state intentionally does not own app data or column policy.
    /// Empty rows or columns clear selection. If no body cell is selected, the first movement
    /// selects the top-left body cell. Movement is clamped to the table bounds and the next layout
    /// pass will reveal the selected cell.
    ///
    /// This changes selection. Use [`VirtualTableState::scroll_viewport_by`] for wheel or page
    /// scrolling that should move the viewport without changing the selected cell.
    ///
    /// # Examples
    ///
    /// Move a selected cell without hand-writing row and column clamping:
    ///
    /// ```rust
    /// use ratatui_layout::table::{CellPosition, VirtualTableState};
    ///
    /// let mut state = VirtualTableState::default();
    ///
    /// assert_eq!(
    ///     state.select_relative(1, 0, 3, 2),
    ///     Some(CellPosition::body(0, 0))
    /// );
    /// assert_eq!(
    ///     state.select_relative(1, 1, 3, 2),
    ///     Some(CellPosition::body(1, 1))
    /// );
    /// assert_eq!(
    ///     state.select_relative(99, 99, 3, 2),
    ///     Some(CellPosition::body(2, 1))
    /// );
    /// ```
    pub const fn select_relative(
        &mut self,
        row_delta: isize,
        column_delta: isize,
        row_count: usize,
        column_count: usize,
    ) -> Option<CellPosition> {
        if row_count == 0 || column_count == 0 {
            self.select(None);
            return None;
        }

        let Some(current) = self.selected else {
            let selected = CellPosition::body(0, 0);
            self.select(Some(selected));
            return Some(selected);
        };
        let row = match current.row {
            Some(row) => clamp_index(offset_index(row, row_delta), row_count),
            None => 0,
        };
        let column = clamp_index(offset_index(current.column, column_delta), column_count);
        let selected = CellPosition::body(row, column);
        self.select(Some(selected));
        Some(selected)
    }

    /// Sets the selected body cell without asking layout to move the viewport.
    ///
    /// Use this when the selected record remains app state, but the user just scrolled the table
    /// viewport. If the selected cell is still visible, renderers receive selected state. If it is
    /// off-screen, the viewport stays where the scroll command put it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::table::{CellPosition, VirtualTableState};
    ///
    /// let mut state = VirtualTableState::default();
    /// state.select_without_scrolling(Some(CellPosition::body(8, 2)));
    ///
    /// assert_eq!(state.selected(), Some(CellPosition::body(8, 2)));
    /// assert!(!state.scrolls_selected_into_view());
    /// ```
    pub const fn select_without_scrolling(&mut self, selected: Option<CellPosition>) {
        self.selected = selected;
        self.scroll_selected_into_view = false;
    }

    /// Asks the next layout pass to bring the current selected cell into view.
    ///
    /// This is useful after restoring selection without scrolling and then receiving keyboard input
    /// that should reveal the selected cell. The exact row and column offsets are still solved by
    /// [`VirtualTable::layout`] because they depend on viewport size and column constraints.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::table::{CellPosition, VirtualTableState};
    ///
    /// let mut state = VirtualTableState::default();
    /// state.select_without_scrolling(Some(CellPosition::body(12, 3)));
    /// state.scroll_to_selected();
    ///
    /// assert!(state.scrolls_selected_into_view());
    /// ```
    pub const fn scroll_to_selected(&mut self) {
        self.scroll_selected_into_view = self.selected.is_some();
    }

    /// Returns the first visible body row index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::table::VirtualTableState;
    ///
    /// let mut state = VirtualTableState::default();
    /// state.set_row_scroll(10);
    ///
    /// assert_eq!(state.row_scroll(), 10);
    /// ```
    pub const fn row_scroll(&self) -> usize {
        self.row_scroll
    }

    /// Sets the first visible body row index.
    ///
    /// # Examples
    ///
    /// Apply a page-down command before the next layout clamps the row offset:
    ///
    /// ```rust
    /// use ratatui_layout::table::VirtualTableState;
    ///
    /// let mut state = VirtualTableState::default();
    /// state.set_row_scroll(20);
    ///
    /// assert_eq!(state.row_scroll(), 20);
    /// ```
    pub const fn set_row_scroll(&mut self, row_scroll: usize) {
        self.row_scroll = row_scroll;
    }

    /// Returns the first visible column index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::table::VirtualTableState;
    ///
    /// let mut state = VirtualTableState::default();
    /// state.set_column_scroll(4);
    ///
    /// assert_eq!(state.column_scroll(), 4);
    /// ```
    pub const fn column_scroll(&self) -> usize {
        self.column_scroll
    }

    /// Sets the first visible column index.
    ///
    /// # Examples
    ///
    /// Apply a horizontal scroll command before the next table layout clamps it:
    ///
    /// ```rust
    /// use ratatui_layout::table::VirtualTableState;
    ///
    /// let mut state = VirtualTableState::default();
    /// state.set_column_scroll(5);
    ///
    /// assert_eq!(state.column_scroll(), 5);
    /// ```
    pub const fn set_column_scroll(&mut self, column_scroll: usize) {
        self.column_scroll = column_scroll;
    }

    /// Scrolls the body rows without changing the selected cell.
    ///
    /// This is the canonical vertical mouse-wheel helper for tables. It updates the desired first
    /// body row and lets the next layout pass clamp the value to the current row count. It also
    /// disables automatic selection reveal so the wheel moves only the viewport.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::table::{CellPosition, VirtualTableState};
    ///
    /// let mut state = VirtualTableState::default();
    /// state.select(Some(CellPosition::body(0, 0)));
    /// state.scroll_rows_by(5);
    ///
    /// assert_eq!(state.selected(), Some(CellPosition::body(0, 0)));
    /// assert_eq!(state.row_scroll(), 5);
    /// assert!(!state.scrolls_selected_into_view());
    /// ```
    pub const fn scroll_rows_by(&mut self, delta: isize) {
        self.row_scroll = offset_index(self.row_scroll, delta);
        self.scroll_selected_into_view = false;
    }

    /// Scrolls visible columns without changing the selected cell.
    ///
    /// Use this for table UIs that support horizontal movement independent of selection. Callers
    /// that do not want horizontal scroll can simply avoid routing horizontal wheel events to this
    /// method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::table::{CellPosition, VirtualTableState};
    ///
    /// let mut state = VirtualTableState::default();
    /// state.select(Some(CellPosition::body(2, 0)));
    /// state.scroll_columns_by(2);
    ///
    /// assert_eq!(state.column_scroll(), 2);
    /// assert!(!state.scrolls_selected_into_view());
    /// ```
    pub const fn scroll_columns_by(&mut self, delta: isize) {
        self.column_scroll = offset_index(self.column_scroll, delta);
        self.scroll_selected_into_view = false;
    }

    /// Scrolls rows and columns as a viewport operation without changing selection.
    ///
    /// This is a convenience for backends or widgets that report two-axis wheel deltas together.
    /// It intentionally does not select a new cell.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::table::{CellPosition, VirtualTableState};
    ///
    /// let mut state = VirtualTableState::default();
    /// state.select(Some(CellPosition::body(3, 1)));
    /// state.scroll_viewport_by(4, 2);
    ///
    /// assert_eq!((state.row_scroll(), state.column_scroll()), (4, 2));
    /// assert_eq!(state.selected(), Some(CellPosition::body(3, 1)));
    /// ```
    pub const fn scroll_viewport_by(&mut self, row_delta: isize, column_delta: isize) {
        self.row_scroll = offset_index(self.row_scroll, row_delta);
        self.column_scroll = offset_index(self.column_scroll, column_delta);
        self.scroll_selected_into_view = false;
    }
}

/// Externally owned cells rendered by a [`VirtualTable`].
///
/// Implement [`TableItems`] for application data or a small adapter. [`VirtualTable`] asks for
/// dimensions and renders only visible header/body cells into assigned rectangles.
///
/// # Required methods
///
/// - [`TableItems::row_count`] returns the current source body-row count.
/// - [`TableItems::column_count`] returns the current source column count.
/// - [`TableItems::render_cell`] renders one visible header or body cell into its assigned area.
///
/// # Examples
///
/// Implement the trait on app-owned records and render headers from `CellPosition::header`:
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::Rect;
/// use ratatui_core::text::Line;
/// use ratatui_core::widgets::Widget;
/// use ratatui_layout::table::{CellPosition, TableCellContext, TableItems};
///
/// struct Rows(&'static [[&'static str; 2]]);
///
/// impl TableItems for Rows {
///     fn row_count(&self) -> usize {
///         self.0.len()
///     }
///
///     fn column_count(&self) -> usize {
///         2
///     }
///
///     fn render_cell(
///         &mut self,
///         position: CellPosition,
///         area: Rect,
///         buf: &mut Buffer,
///         _: TableCellContext,
///     ) {
///         let text = position
///             .row
///             .map(|row| self.0[row][position.column])
///             .unwrap_or(["id", "name"][position.column]);
///         Line::from(text).render(area, buf);
///     }
/// }
/// ```
pub trait TableItems {
    /// Returns the number of body rows.
    ///
    /// # Examples
    ///
    /// Back a table with app-owned data rather than row widgets:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::table::{CellPosition, TableCellContext, TableItems};
    ///
    /// struct Rows(&'static [[&'static str; 2]]);
    ///
    /// impl TableItems for Rows {
    ///     fn row_count(&self) -> usize {
    ///         self.0.len()
    ///     }
    ///     fn column_count(&self) -> usize {
    ///         2
    ///     }
    ///     fn render_cell(&mut self, _: CellPosition, _: Rect, _: &mut Buffer, _: TableCellContext) {}
    /// }
    ///
    /// assert_eq!(Rows(&[["a", "b"]]).row_count(), 1);
    /// ```
    fn row_count(&self) -> usize;

    /// Returns the number of columns.
    ///
    /// # Examples
    ///
    /// Report the source column count separately from the visible columns:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::table::{CellPosition, TableCellContext, TableItems};
    ///
    /// struct Rows;
    ///
    /// impl TableItems for Rows {
    ///     fn row_count(&self) -> usize {
    ///         10
    ///     }
    ///     fn column_count(&self) -> usize {
    ///         4
    ///     }
    ///     fn render_cell(&mut self, _: CellPosition, _: Rect, _: &mut Buffer, _: TableCellContext) {}
    /// }
    ///
    /// assert_eq!(Rows.column_count(), 4);
    /// ```
    fn column_count(&self) -> usize;

    /// Renders a visible table cell.
    ///
    /// # Examples
    ///
    /// Render only the cells that [`VirtualTable::render`] determined are visible:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_core::text::Line;
    /// use ratatui_core::widgets::Widget;
    /// use ratatui_layout::table::{CellPosition, TableCellContext, TableItems};
    ///
    /// struct Rows(&'static [[&'static str; 2]]);
    ///
    /// impl TableItems for Rows {
    ///     fn row_count(&self) -> usize {
    ///         self.0.len()
    ///     }
    ///     fn column_count(&self) -> usize {
    ///         2
    ///     }
    ///     fn render_cell(
    ///         &mut self,
    ///         position: CellPosition,
    ///         area: Rect,
    ///         buf: &mut Buffer,
    ///         _: TableCellContext,
    ///     ) {
    ///         let text = position
    ///             .row
    ///             .map(|row| self.0[row][position.column])
    ///             .unwrap_or("header");
    ///         Line::from(text).render(area, buf);
    ///     }
    /// }
    /// ```
    fn render_cell(
        &mut self,
        position: CellPosition,
        area: Rect,
        buf: &mut Buffer,
        ctx: TableCellContext,
    );
}

/// Render context for a visible table cell.
///
/// [`TableCellContext`] combines common [`RenderContext`] state with table-specific coordinates.
/// It is passed to [`TableItems::render_cell`] for visible cells only.
///
/// # Fields
///
/// - [`TableCellContext::render`] carries shared interaction flags such as selection.
/// - [`TableCellContext::position`] is the source cell being rendered.
/// - [`TableCellContext::visible_column`] is the column's visible index after horizontal scrolling.
/// - [`TableCellContext::visible_row`] is the visible body row index, or `None` for headers.
///
/// # Examples
///
/// Render headers, selected cells, and scrolled body cells from the same context:
///
/// ```rust
/// use ratatui_layout::RenderContext;
/// use ratatui_layout::table::{CellPosition, TableCellContext};
///
/// let context = TableCellContext {
///     render: RenderContext::selected(true),
///     position: CellPosition::body(12, 3),
///     visible_column: 1,
///     visible_row: Some(0),
/// };
///
/// assert!(context.render.state.selected);
/// assert_eq!(context.position, CellPosition::body(12, 3));
/// assert_eq!(context.visible_row, Some(0));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TableCellContext {
    /// Common render context.
    pub render: RenderContext,

    /// Position of the cell in the source table.
    pub position: CellPosition,

    /// Column index among visible columns.
    pub visible_column: usize,

    /// Row index among visible body rows. Header cells use `None`.
    pub visible_row: Option<usize>,
}

/// Metadata for a visible table cell.
///
/// Use [`VisibleCell`] for hit testing, diagnostics, focus target collections, or custom rendering
/// outside the high-level [`VirtualTable::render`] method.
///
/// # Fields
///
/// - [`VisibleCell::position`] is the source header or body cell id.
/// - [`VisibleCell::area`] is the terminal-space rectangle assigned to the cell.
/// - [`VisibleCell::visible_column`] is the visible column index after horizontal scrolling.
/// - [`VisibleCell::visible_row`] is the visible body row index, or `None` for headers.
///
/// # Examples
///
/// Build focus or pointer targets from the table cells visible in the current frame:
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
/// use ratatui_layout::table::{CellPosition, VisibleCell};
///
/// let cell = VisibleCell {
///     position: CellPosition::body(4, 2),
///     area: Rect::new(10, 3, 8, 1),
///     visible_column: 0,
///     visible_row: Some(1),
/// };
/// let mouse = PointerTargets::new().target(PointerTarget::new(cell.position, cell.area));
///
/// assert_eq!(
///     mouse.hit_test((11, 3)).unwrap().id,
///     CellPosition::body(4, 2)
/// );
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VisibleCell {
    /// Source cell position.
    pub position: CellPosition,

    /// Assigned screen area.
    pub area: Rect,

    /// Column index among visible columns.
    pub visible_column: usize,

    /// Row index among visible body rows. Header cells use `None`.
    pub visible_row: Option<usize>,
}

impl VisibleCell {
    fn context(self, selected: bool) -> TableCellContext {
        TableCellContext {
            render: RenderContext {
                state: RenderState {
                    selected,
                    ..RenderState::default()
                },
            },
            position: self.position,
            visible_column: self.visible_column,
            visible_row: self.visible_row,
        }
    }
}

/// Solved table layout.
///
/// Use [`TableLayout`] when code needs to inspect what a [`VirtualTable`] did during a frame:
/// visible [`VisibleCell`] values, selected cell area, hit testing, and [`ScrollMetrics`].
///
/// # Fields and methods
///
/// - [`TableLayout::area`] is the full terminal area assigned to the table.
/// - [`TableLayout::header_area`] is the pinned header area when headers are enabled and visible.
/// - [`TableLayout::body_area`] is the body viewport.
/// - [`TableLayout::row_count`] and [`TableLayout::column_count`] are source dimensions.
/// - [`TableLayout::row_scroll`] and [`TableLayout::column_scroll`] are clamped scroll offsets.
/// - [`TableLayout::visible_cells`] stores visible header and body cells in render order.
/// - [`TableLayout::selected`] is the selected body cell when visible.
/// - [`TableLayout::visible_positions`] returns only visible cell ids for traversal or diagnostics.
/// - [`TableLayout::cell_regions`] converts visible cells into a generic [`Regions`].
/// - [`TableLayout::hit_test`] maps terminal positions to [`CellPosition`] values.
/// - [`TableLayout::hit_position`] returns only the clicked cell position.
/// - [`TableLayout::select_hit`] selects a clicked body cell in [`VirtualTableState`].
/// - [`TableLayout::vertical_metrics`] and [`TableLayout::horizontal_metrics`] compute scrollbar
///   data for app-owned scrollbars or status displays.
///
/// # Examples
///
/// Use the solved layout for routing and status chrome after rendering:
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::{Constraint, Rect};
/// use ratatui_layout::table::{
///     CellPosition, TableCellContext, TableItems, VirtualTable, VirtualTableState,
/// };
///
/// struct Rows;
/// impl TableItems for Rows {
///     fn row_count(&self) -> usize {
///         20
///     }
///     fn column_count(&self) -> usize {
///         4
///     }
///     fn render_cell(&mut self, _: CellPosition, _: Rect, _: &mut Buffer, _: TableCellContext) {}
/// }
///
/// let mut rows = Rows;
/// let mut state = VirtualTableState::default();
/// state.set_row_scroll(5);
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 16, 4));
/// let layout = VirtualTable::new([Constraint::Length(4); 4]).render(
///     buffer.area,
///     &mut buffer,
///     &mut state,
///     &mut rows,
/// );
///
/// assert_eq!(layout.vertical_metrics().offset, 5);
/// assert_eq!(
///     layout.hit_test((5, 1)).unwrap().id,
///     CellPosition::body(5, 1)
/// );
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TableLayout {
    /// Full table area.
    pub area: Rect,

    /// Header area, if headers are enabled and visible.
    pub header_area: Option<Rect>,

    /// Body viewport area.
    pub body_area: Rect,

    /// Total body rows.
    pub row_count: usize,

    /// Total columns.
    pub column_count: usize,

    /// First visible body row.
    pub row_scroll: usize,

    /// First visible column.
    pub column_scroll: usize,

    /// Visible header and body cells.
    pub visible_cells: Vec<VisibleCell>,

    /// Visible selected cell, if any.
    pub selected: Option<VisibleCell>,
}

impl TableLayout {
    /// Returns source cell positions visible in this layout.
    ///
    /// Positions are returned in render order and include header cells when headers are enabled.
    /// Use this when selection, diagnostics, or target construction only needs the ids that the
    /// table made visible, not the full [`VisibleCell`] metadata.
    ///
    /// # Examples
    ///
    /// Build a visible-position list for keyboard traversal or diagnostics:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::table::{CellPosition, TableLayout, VisibleCell};
    ///
    /// let layout = TableLayout {
    ///     area: Rect::new(0, 0, 8, 2),
    ///     header_area: Some(Rect::new(0, 0, 8, 1)),
    ///     body_area: Rect::new(0, 1, 8, 1),
    ///     row_count: 1,
    ///     column_count: 2,
    ///     row_scroll: 0,
    ///     column_scroll: 0,
    ///     visible_cells: vec![
    ///         VisibleCell {
    ///             position: CellPosition::header(0),
    ///             area: Rect::new(0, 0, 4, 1),
    ///             visible_column: 0,
    ///             visible_row: None,
    ///         },
    ///         VisibleCell {
    ///             position: CellPosition::body(0, 0),
    ///             area: Rect::new(0, 1, 4, 1),
    ///             visible_column: 0,
    ///             visible_row: Some(0),
    ///         },
    ///     ],
    ///     selected: None,
    /// };
    ///
    /// assert_eq!(
    ///     layout.visible_positions().collect::<Vec<_>>(),
    ///     [CellPosition::header(0), CellPosition::body(0, 0)]
    /// );
    /// ```
    pub fn visible_positions(&self) -> impl Iterator<Item = CellPosition> + '_ {
        self.visible_cells.iter().map(|cell| cell.position)
    }

    /// Converts visible cells into a generic region set keyed by [`CellPosition`].
    ///
    /// Use this when a virtual table needs to participate in APIs that understand
    /// [`Regions`] rather than [`TableLayout`]. The table layout remains the richer source of
    /// truth for pinned headers, scroll offsets, visible row/column indexes, and scroll metrics;
    /// the returned [`Regions`] value is the geometry projection used for generic
    /// composition, pointer routing, or diagnostics.
    ///
    /// Use [`TableLayout::cells_regions`] when outer coordination code should route through domain
    /// ids instead of table coordinates.
    ///
    /// # Examples
    ///
    /// Convert visible cells into regions that can be merged into a frame snapshot:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::table::{CellPosition, TableLayout, VisibleCell};
    ///
    /// let layout = TableLayout {
    ///     area: Rect::new(0, 0, 8, 2),
    ///     header_area: Some(Rect::new(0, 0, 8, 1)),
    ///     body_area: Rect::new(0, 1, 8, 1),
    ///     row_count: 1,
    ///     column_count: 2,
    ///     row_scroll: 0,
    ///     column_scroll: 0,
    ///     visible_cells: vec![
    ///         VisibleCell {
    ///             position: CellPosition::header(0),
    ///             area: Rect::new(0, 0, 4, 1),
    ///             visible_column: 0,
    ///             visible_row: None,
    ///         },
    ///         VisibleCell {
    ///             position: CellPosition::body(0, 0),
    ///             area: Rect::new(0, 1, 4, 1),
    ///             visible_column: 0,
    ///             visible_row: Some(0),
    ///         },
    ///     ],
    ///     selected: None,
    /// };
    /// let plan = layout.cell_regions();
    ///
    /// assert_eq!(plan.hit_test((1, 1)).unwrap().id, CellPosition::body(0, 0));
    /// ```
    pub fn cell_regions(&self) -> Regions<CellPosition> {
        self.cells_regions(core::convert::identity)
    }

    /// Converts visible cells into a generic region set with caller-chosen ids.
    ///
    /// Use this when a table renders by [`CellPosition`] but the rest of the app routes input
    /// through semantic ids. A release board might map `CellPosition::body(row, column)` to a
    /// `(TaskId, FieldId)` pair, while keeping header cells as sort actions. The mapper receives
    /// each visible source cell position in render order.
    ///
    /// # Examples
    ///
    /// Project visible body cells into domain-specific ids:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::table::{CellPosition, TableLayout, VisibleCell};
    ///
    /// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    /// enum Target {
    ///     Header(usize),
    ///     Cell { row: usize, column: usize },
    /// }
    ///
    /// let layout = TableLayout {
    ///     area: Rect::new(0, 0, 8, 2),
    ///     header_area: Some(Rect::new(0, 0, 8, 1)),
    ///     body_area: Rect::new(0, 1, 8, 1),
    ///     row_count: 1,
    ///     column_count: 2,
    ///     row_scroll: 0,
    ///     column_scroll: 0,
    ///     visible_cells: vec![VisibleCell {
    ///         position: CellPosition::body(0, 1),
    ///         area: Rect::new(4, 1, 4, 1),
    ///         visible_column: 1,
    ///         visible_row: Some(0),
    ///     }],
    ///     selected: None,
    /// };
    /// let plan = layout.cells_regions(|position| match position.row {
    ///     Some(row) => Target::Cell {
    ///         row,
    ///         column: position.column,
    ///     },
    ///     None => Target::Header(position.column),
    /// });
    ///
    /// assert_eq!(
    ///     plan.hit_test((5, 1)).unwrap().id,
    ///     Target::Cell { row: 0, column: 1 }
    /// );
    /// ```
    pub fn cells_regions<Id>(&self, mut id_for: impl FnMut(CellPosition) -> Id) -> Regions<Id> {
        let regions: Vec<_> = self
            .visible_cells
            .iter()
            .map(|cell| Region::new(id_for(cell.position), cell.area))
            .collect();
        Regions::from_regions(self.area, regions)
    }

    /// Returns the visible cell hit by the position.
    ///
    /// # Examples
    ///
    /// Route a previous-frame pointer position to a source table cell:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::table::{CellPosition, TableLayout, VisibleCell};
    ///
    /// let layout = TableLayout {
    ///     area: Rect::new(0, 0, 8, 2),
    ///     header_area: Some(Rect::new(0, 0, 8, 1)),
    ///     body_area: Rect::new(0, 1, 8, 1),
    ///     row_count: 1,
    ///     column_count: 2,
    ///     row_scroll: 0,
    ///     column_scroll: 0,
    ///     visible_cells: vec![VisibleCell {
    ///         position: CellPosition::body(0, 1),
    ///         area: Rect::new(4, 1, 4, 1),
    ///         visible_column: 1,
    ///         visible_row: Some(0),
    ///     }],
    ///     selected: None,
    /// };
    ///
    /// assert_eq!(
    ///     layout.hit_test((5, 1)).unwrap().id,
    ///     CellPosition::body(0, 1)
    /// );
    /// ```
    pub fn hit_test<P: Into<Position>>(&self, position: P) -> Option<Hit<CellPosition>> {
        let position = position.into();
        self.visible_cells
            .iter()
            .rev()
            .find(|cell| cell.area.contains(position))
            .map(|cell| Hit {
                id: cell.position,
                area: cell.area,
                relative_x: position.x.saturating_sub(cell.area.x),
                relative_y: position.y.saturating_sub(cell.area.y),
            })
    }

    /// Returns the cell position hit by the terminal position.
    ///
    /// This is the common click path when an app only needs the header or body cell coordinate.
    /// Use [`TableLayout::hit_test`] when the cell renderer also needs local coordinates.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::table::{CellPosition, TableLayout, VisibleCell};
    ///
    /// let layout = TableLayout {
    ///     area: Rect::new(0, 0, 8, 2),
    ///     header_area: Some(Rect::new(0, 0, 8, 1)),
    ///     body_area: Rect::new(0, 1, 8, 1),
    ///     row_count: 1,
    ///     column_count: 2,
    ///     row_scroll: 0,
    ///     column_scroll: 0,
    ///     visible_cells: vec![VisibleCell {
    ///         position: CellPosition::body(0, 1),
    ///         area: Rect::new(4, 1, 4, 1),
    ///         visible_column: 1,
    ///         visible_row: Some(0),
    ///     }],
    ///     selected: None,
    /// };
    ///
    /// assert_eq!(layout.hit_position((5, 1)), Some(CellPosition::body(0, 1)));
    /// ```
    pub fn hit_position<P: Into<Position>>(&self, position: P) -> Option<CellPosition> {
        self.hit_test(position).map(|hit| hit.id)
    }

    /// Selects the body cell hit by the terminal position.
    ///
    /// Header hits and blank-space hits leave state unchanged and return `None`. This keeps
    /// [`VirtualTableState`] focused on body-cell selection while still letting apps use
    /// [`TableLayout::hit_position`] for header actions such as sorting.
    ///
    /// # Examples
    ///
    /// Click a visible body cell and ask the next layout to reveal it:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::table::{CellPosition, TableLayout, VirtualTableState, VisibleCell};
    ///
    /// let layout = TableLayout {
    ///     area: Rect::new(0, 0, 8, 2),
    ///     header_area: Some(Rect::new(0, 0, 8, 1)),
    ///     body_area: Rect::new(0, 1, 8, 1),
    ///     row_count: 1,
    ///     column_count: 2,
    ///     row_scroll: 0,
    ///     column_scroll: 0,
    ///     visible_cells: vec![
    ///         VisibleCell {
    ///             position: CellPosition::header(1),
    ///             area: Rect::new(4, 0, 4, 1),
    ///             visible_column: 1,
    ///             visible_row: None,
    ///         },
    ///         VisibleCell {
    ///             position: CellPosition::body(0, 1),
    ///             area: Rect::new(4, 1, 4, 1),
    ///             visible_column: 1,
    ///             visible_row: Some(0),
    ///         },
    ///     ],
    ///     selected: None,
    /// };
    /// let mut state = VirtualTableState::default();
    ///
    /// assert_eq!(layout.select_hit((5, 0), &mut state), None);
    /// assert_eq!(
    ///     layout.select_hit((5, 1), &mut state),
    ///     Some(CellPosition::body(0, 1))
    /// );
    /// assert_eq!(state.selected(), Some(CellPosition::body(0, 1)));
    /// ```
    pub fn select_hit<P: Into<Position>>(
        &self,
        position: P,
        state: &mut VirtualTableState,
    ) -> Option<CellPosition> {
        let position = self.hit_position(position)?;
        position.row?;
        state.select(Some(position));
        Some(position)
    }

    /// Returns vertical body scroll metrics.
    ///
    /// # Examples
    ///
    /// Build scrollbar data from the solved table body viewport:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::table::TableLayout;
    ///
    /// let layout = TableLayout {
    ///     area: Rect::new(0, 0, 10, 5),
    ///     header_area: None,
    ///     body_area: Rect::new(0, 0, 10, 5),
    ///     row_count: 20,
    ///     column_count: 1,
    ///     row_scroll: 4,
    ///     column_scroll: 0,
    ///     visible_cells: vec![],
    ///     selected: None,
    /// };
    ///
    /// assert_eq!(layout.vertical_metrics().offset, 4);
    /// ```
    pub fn vertical_metrics(&self) -> ScrollMetrics {
        ScrollMetrics::new(
            self.row_count as u32,
            self.body_area.height,
            self.row_scroll as u32,
        )
    }

    /// Returns horizontal column scroll metrics.
    ///
    /// # Examples
    ///
    /// Build horizontal scrollbar data from visible column capacity:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::table::{CellPosition, TableLayout, VisibleCell};
    ///
    /// let layout = TableLayout {
    ///     area: Rect::new(0, 0, 10, 2),
    ///     header_area: Some(Rect::new(0, 0, 10, 1)),
    ///     body_area: Rect::new(0, 1, 10, 1),
    ///     row_count: 1,
    ///     column_count: 5,
    ///     row_scroll: 0,
    ///     column_scroll: 2,
    ///     visible_cells: vec![
    ///         VisibleCell {
    ///             position: CellPosition::header(2),
    ///             area: Rect::new(0, 0, 5, 1),
    ///             visible_column: 0,
    ///             visible_row: None,
    ///         },
    ///         VisibleCell {
    ///             position: CellPosition::header(3),
    ///             area: Rect::new(5, 0, 5, 1),
    ///             visible_column: 1,
    ///             visible_row: None,
    ///         },
    ///     ],
    ///     selected: None,
    /// };
    ///
    /// assert_eq!(layout.horizontal_metrics().offset, 2);
    /// ```
    pub fn horizontal_metrics(&self) -> ScrollMetrics {
        ScrollMetrics::new(
            self.column_count as u32,
            self.visible_column_capacity() as u16,
            self.column_scroll as u32,
        )
    }

    fn visible_column_capacity(&self) -> usize {
        self.visible_cells
            .iter()
            .filter(|cell| cell.position.row.is_none())
            .count()
            .max(
                self.visible_cells
                    .iter()
                    .filter_map(|cell| cell.visible_row.map(|_| cell.visible_column + 1))
                    .max()
                    .unwrap_or_default(),
            )
    }
}

/// A virtualized table with app-owned cells.
///
/// Use `VirtualTable` for large or custom-rendered tables where the built-in table's ownership
/// shape is too restrictive. It supports pinned headers, selected body cells, column-based
/// horizontal scrolling, row virtualization, and visible-cell hit testing.
///
/// # Constructors and setters
///
/// - [`VirtualTable::new`] creates a table layout configuration from column constraints.
/// - [`VirtualTable::row_height`] sets fixed body-row height.
/// - [`VirtualTable::header_height`] sets pinned header height or hides headers with zero.
/// - [`VirtualTable::scroll_padding`] keeps space around the selected cell when possible.
///
/// # Layout and rendering
///
/// - [`VirtualTable::layout`] computes [`TableLayout`] and clamps [`VirtualTableState`] without
///   drawing.
/// - [`VirtualTable::render`] computes layout, renders visible cells, and returns the layout.
///
/// # Examples
///
/// Rebuild the table layout each frame while storing only app-owned state:
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::{Constraint, Rect};
/// use ratatui_layout::table::{
///     CellPosition, TableCellContext, TableItems, VirtualTable, VirtualTableState,
/// };
///
/// struct Rows;
/// impl TableItems for Rows {
///     fn row_count(&self) -> usize {
///         3
///     }
///     fn column_count(&self) -> usize {
///         2
///     }
///     fn render_cell(&mut self, _: CellPosition, _: Rect, _: &mut Buffer, _: TableCellContext) {}
/// }
///
/// let mut rows = Rows;
/// let mut state = VirtualTableState::default();
/// state.select(Some(CellPosition::body(2, 1)));
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 3));
/// let layout = VirtualTable::new([Constraint::Length(4), Constraint::Length(4)])
///     .scroll_padding(1)
///     .render(buffer.area, &mut buffer, &mut state, &mut rows);
///
/// assert_eq!(layout.selected.unwrap().position, CellPosition::body(2, 1));
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct VirtualTable {
    columns: Vec<Constraint>,
    row_height: u16,
    header_height: u16,
    scroll_padding: usize,
}

impl VirtualTable {
    /// Creates a virtual table from column constraints.
    ///
    /// # Examples
    ///
    /// Compute visible table cells from ordinary Ratatui column constraints:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::table::{
    ///     CellPosition, TableCellContext, TableItems, VirtualTable, VirtualTableState,
    /// };
    ///
    /// struct Rows;
    /// impl TableItems for Rows {
    ///     fn row_count(&self) -> usize {
    ///         2
    ///     }
    ///     fn column_count(&self) -> usize {
    ///         2
    ///     }
    ///     fn render_cell(&mut self, _: CellPosition, _: Rect, _: &mut Buffer, _: TableCellContext) {}
    /// }
    ///
    /// let mut state = VirtualTableState::default();
    /// let layout = VirtualTable::new([Constraint::Length(4), Constraint::Length(4)]).layout(
    ///     Rect::new(0, 0, 8, 2),
    ///     &mut state,
    ///     &Rows,
    /// );
    ///
    /// assert_eq!(layout.visible_cells[0].position, CellPosition::header(0));
    /// ```
    pub fn new<C>(columns: C) -> Self
    where
        C: IntoIterator,
        C::Item: Into<Constraint>,
    {
        Self {
            columns: columns.into_iter().map(Into::into).collect(),
            row_height: 1,
            header_height: 1,
            scroll_padding: 0,
        }
    }

    /// Sets the body row height.
    ///
    /// # Examples
    ///
    /// Use taller body rows when each record renders multiple terminal lines:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::table::{
    ///     CellPosition, TableCellContext, TableItems, VirtualTable, VirtualTableState,
    /// };
    ///
    /// struct Rows;
    /// impl TableItems for Rows {
    ///     fn row_count(&self) -> usize {
    ///         1
    ///     }
    ///     fn column_count(&self) -> usize {
    ///         1
    ///     }
    ///     fn render_cell(&mut self, _: CellPosition, _: Rect, _: &mut Buffer, _: TableCellContext) {}
    /// }
    ///
    /// let mut state = VirtualTableState::default();
    /// let layout = VirtualTable::new([Constraint::Length(8)])
    ///     .row_height(2)
    ///     .layout(Rect::new(0, 0, 8, 3), &mut state, &Rows);
    ///
    /// assert_eq!(layout.visible_cells[1].area.height, 2);
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn row_height(mut self, row_height: u16) -> Self {
        self.row_height = row_height.max(1);
        self
    }

    /// Sets the pinned header height. Use zero to hide headers.
    ///
    /// # Examples
    ///
    /// Hide header cells for a body-only data grid:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::table::{
    ///     CellPosition, TableCellContext, TableItems, VirtualTable, VirtualTableState,
    /// };
    ///
    /// struct Rows;
    /// impl TableItems for Rows {
    ///     fn row_count(&self) -> usize {
    ///         1
    ///     }
    ///     fn column_count(&self) -> usize {
    ///         1
    ///     }
    ///     fn render_cell(&mut self, _: CellPosition, _: Rect, _: &mut Buffer, _: TableCellContext) {}
    /// }
    ///
    /// let mut state = VirtualTableState::default();
    /// let layout = VirtualTable::new([Constraint::Length(8)])
    ///     .header_height(0)
    ///     .layout(Rect::new(0, 0, 8, 1), &mut state, &Rows);
    ///
    /// assert_eq!(layout.header_area, None);
    /// assert_eq!(layout.visible_cells[0].position, CellPosition::body(0, 0));
    /// ```
    #[must_use = "method returns the modified value"]
    pub const fn header_height(mut self, header_height: u16) -> Self {
        self.header_height = header_height;
        self
    }

    /// Sets row/column padding kept around the selected cell when possible.
    ///
    /// # Examples
    ///
    /// Keep context around a selected body cell when scrolling in both axes:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::table::{
    ///     CellPosition, TableCellContext, TableItems, VirtualTable, VirtualTableState,
    /// };
    ///
    /// struct Rows;
    /// impl TableItems for Rows {
    ///     fn row_count(&self) -> usize {
    ///         10
    ///     }
    ///     fn column_count(&self) -> usize {
    ///         5
    ///     }
    ///     fn render_cell(&mut self, _: CellPosition, _: Rect, _: &mut Buffer, _: TableCellContext) {}
    /// }
    ///
    /// let mut state = VirtualTableState::default();
    /// state.select(Some(CellPosition::body(8, 4)));
    /// VirtualTable::new([Constraint::Length(2); 5])
    ///     .scroll_padding(1)
    ///     .layout(Rect::new(0, 0, 4, 4), &mut state, &Rows);
    ///
    /// assert_eq!(state.row_scroll(), 7);
    /// assert_eq!(state.column_scroll(), 2);
    /// ```
    #[must_use = "method returns the modified value"]
    pub const fn scroll_padding(mut self, scroll_padding: usize) -> Self {
        self.scroll_padding = scroll_padding;
        self
    }

    /// Computes the table layout and clamps state.
    ///
    /// # Examples
    ///
    /// Compute a frame-local table layout for hit testing without rendering cells:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::table::{
    ///     CellPosition, TableCellContext, TableItems, VirtualTable, VirtualTableState,
    /// };
    ///
    /// struct Rows;
    /// impl TableItems for Rows {
    ///     fn row_count(&self) -> usize {
    ///         1
    ///     }
    ///     fn column_count(&self) -> usize {
    ///         2
    ///     }
    ///     fn render_cell(&mut self, _: CellPosition, _: Rect, _: &mut Buffer, _: TableCellContext) {}
    /// }
    ///
    /// let mut state = VirtualTableState::default();
    /// let layout = VirtualTable::new([Constraint::Length(4), Constraint::Length(4)]).layout(
    ///     Rect::new(0, 0, 8, 2),
    ///     &mut state,
    ///     &Rows,
    /// );
    ///
    /// assert_eq!(
    ///     layout.hit_test((5, 1)).unwrap().id,
    ///     CellPosition::body(0, 1)
    /// );
    /// ```
    pub fn layout<I: TableItems>(
        self,
        area: Rect,
        state: &mut VirtualTableState,
        items: &I,
    ) -> TableLayout {
        let row_count = items.row_count();
        let column_count = items.column_count().min(self.columns.len());
        Self::clamp_selection(state, row_count, column_count);
        if state.scroll_selected_into_view {
            self.scroll_to_selection(state, row_count, column_count, area);
        }

        let areas = self.areas(area);
        state.row_scroll = clamp_start(
            state.row_scroll,
            row_count,
            visible_rows(areas.body, self.row_height),
        );
        let column_scroll = state.column_scroll.min(column_count);
        state.column_scroll = clamp_start(
            column_scroll,
            column_count,
            visible_columns(area, &self.columns[column_scroll..column_count]),
        );

        let visible_cells = self.visible_cells(areas, state, row_count, column_count);
        let selected = state.selected.and_then(|selected| {
            visible_cells
                .iter()
                .find(|cell| cell.position == selected)
                .copied()
        });

        TableLayout {
            area,
            header_area: areas.header,
            body_area: areas.body,
            row_count,
            column_count,
            row_scroll: state.row_scroll,
            column_scroll: state.column_scroll,
            visible_cells,
            selected,
        }
    }

    /// Computes and renders the table.
    ///
    /// # Examples
    ///
    /// Render visible cells and keep the layout for the next input event:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_core::text::Line;
    /// use ratatui_core::widgets::Widget;
    /// use ratatui_layout::table::{
    ///     CellPosition, TableCellContext, TableItems, VirtualTable, VirtualTableState,
    /// };
    ///
    /// struct Rows(&'static [[&'static str; 2]]);
    /// impl TableItems for Rows {
    ///     fn row_count(&self) -> usize {
    ///         self.0.len()
    ///     }
    ///     fn column_count(&self) -> usize {
    ///         2
    ///     }
    ///     fn render_cell(
    ///         &mut self,
    ///         position: CellPosition,
    ///         area: Rect,
    ///         buf: &mut Buffer,
    ///         _: TableCellContext,
    ///     ) {
    ///         let text = position
    ///             .row
    ///             .map(|row| self.0[row][position.column])
    ///             .unwrap_or("header");
    ///         Line::from(text).render(area, buf);
    ///     }
    /// }
    ///
    /// let mut rows = Rows(&[["a", "b"]]);
    /// let mut state = VirtualTableState::default();
    /// let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 2));
    /// let layout = VirtualTable::new([Constraint::Length(4), Constraint::Length(4)]).render(
    ///     buffer.area,
    ///     &mut buffer,
    ///     &mut state,
    ///     &mut rows,
    /// );
    ///
    /// assert_eq!(layout.visible_cells.len(), 4);
    /// ```
    pub fn render<I: TableItems>(
        self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut VirtualTableState,
        items: &mut I,
    ) -> TableLayout {
        let layout = self.layout(area, state, items);
        for cell in layout.visible_cells.iter().copied() {
            let selected = state.selected == Some(cell.position);
            items.render_cell(cell.position, cell.area, buf, cell.context(selected));
        }
        layout
    }

    fn clamp_selection(state: &mut VirtualTableState, row_count: usize, column_count: usize) {
        if let Some(selected) = state.selected {
            state.selected = selected.row.filter(|row| *row < row_count).and_then(|row| {
                (selected.column < column_count).then_some(CellPosition::body(row, selected.column))
            });
            if state.selected.is_none() {
                state.scroll_selected_into_view = false;
            }
        }
    }

    fn scroll_to_selection(
        &self,
        state: &mut VirtualTableState,
        row_count: usize,
        column_count: usize,
        area: Rect,
    ) {
        let Some(selected) = state
            .selected
            .and_then(|position| position.row.map(|row| (row, position.column)))
        else {
            return;
        };
        let areas = self.areas(area);
        let rows = visible_rows(areas.body, self.row_height);
        let columns = visible_columns(
            area,
            &self.columns[state.column_scroll.min(column_count)..column_count],
        );
        state.row_scroll = scroll_to_index(
            state.row_scroll,
            selected.0,
            row_count,
            rows,
            self.scroll_padding,
        );
        state.column_scroll = scroll_to_index(
            state.column_scroll,
            selected.1,
            column_count,
            columns,
            self.scroll_padding,
        );
    }

    fn areas(&self, area: Rect) -> TableAreas {
        if self.header_height == 0 || area.height == 0 {
            return TableAreas {
                header: None,
                body: area,
            };
        }

        let header_height = self.header_height.min(area.height);
        let header = Rect::new(area.x, area.y, area.width, header_height);
        let body = Rect::new(
            area.x,
            area.y.saturating_add(header_height),
            area.width,
            area.height.saturating_sub(header_height),
        );
        TableAreas {
            header: Some(header),
            body,
        }
    }

    fn visible_cells(
        &self,
        areas: TableAreas,
        state: &VirtualTableState,
        row_count: usize,
        column_count: usize,
    ) -> Vec<VisibleCell> {
        let mut cells = Vec::new();
        let columns = self.visible_column_areas(areas.body, state.column_scroll, column_count);
        if let Some(header) = areas.header {
            cells.extend(columns.iter().copied().map(|column| VisibleCell {
                position: CellPosition::header(column.index),
                area: Rect::new(column.area.x, header.y, column.area.width, header.height),
                visible_column: column.visible_index,
                visible_row: None,
            }));
        }

        let rows = visible_rows(areas.body, self.row_height);
        let last_row = state.row_scroll.saturating_add(rows).min(row_count);
        for (visible_row, row) in (state.row_scroll..last_row).enumerate() {
            let y = areas
                .body
                .y
                .saturating_add((visible_row as u16).saturating_mul(self.row_height));
            let height = self.row_height.min(areas.body.bottom().saturating_sub(y));
            cells.extend(columns.iter().copied().map(|column| VisibleCell {
                position: CellPosition::body(row, column.index),
                area: Rect::new(column.area.x, y, column.area.width, height),
                visible_column: column.visible_index,
                visible_row: Some(visible_row),
            }));
        }
        cells
    }

    fn visible_column_areas(
        &self,
        area: Rect,
        column_scroll: usize,
        column_count: usize,
    ) -> Vec<VisibleColumn> {
        let constraints = &self.columns[column_scroll..column_count];
        Layout::horizontal(constraints.iter().copied())
            .split(area)
            .iter()
            .copied()
            .enumerate()
            .take_while(|(_, area)| area.width > 0)
            .map(|(visible_index, area)| VisibleColumn {
                index: column_scroll + visible_index,
                visible_index,
                area,
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
struct TableAreas {
    header: Option<Rect>,
    body: Rect,
}

#[derive(Debug, Clone, Copy)]
struct VisibleColumn {
    index: usize,
    visible_index: usize,
    area: Rect,
}

fn visible_rows(area: Rect, row_height: u16) -> usize {
    if row_height == 0 {
        return 0;
    }
    usize::from(area.height / row_height)
}

fn visible_columns(area: Rect, constraints: &[Constraint]) -> usize {
    Layout::horizontal(constraints.iter().copied())
        .split(area)
        .iter()
        .filter(|area| area.width > 0)
        .count()
}

fn clamp_start(start: usize, len: usize, visible: usize) -> usize {
    if len <= visible {
        0
    } else {
        start.min(len - visible)
    }
}

fn scroll_to_index(
    start: usize,
    selected: usize,
    len: usize,
    visible: usize,
    padding: usize,
) -> usize {
    if visible == 0 || len <= visible {
        return 0;
    }

    let padding = padding.min(visible.saturating_sub(1) / 2);
    if selected < start.saturating_add(padding) {
        return selected.saturating_sub(padding);
    }

    let padded_end = start.saturating_add(visible).saturating_sub(padding + 1);
    if selected > padded_end {
        return selected.saturating_add(padding + 1).saturating_sub(visible);
    }

    start
}

const fn offset_index(index: usize, delta: isize) -> usize {
    if delta.is_negative() {
        index.saturating_sub(delta.unsigned_abs())
    } else {
        index.saturating_add(delta as usize)
    }
}

const fn clamp_index(index: usize, len: usize) -> usize {
    if index >= len { len - 1 } else { index }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use ratatui_core::layout::Constraint;

    use super::*;

    struct EmptyTable {
        rows: usize,
        columns: usize,
    }

    impl TableItems for EmptyTable {
        fn row_count(&self) -> usize {
            self.rows
        }

        fn column_count(&self) -> usize {
            self.columns
        }

        fn render_cell(&mut self, _: CellPosition, _: Rect, _: &mut Buffer, _: TableCellContext) {}
    }

    #[test]
    fn lays_out_pinned_headers_and_body_cells() {
        let table = VirtualTable::new([Constraint::Length(4), Constraint::Length(6)]);
        let mut state = VirtualTableState::default();
        let layout = table.layout(
            Rect::new(0, 0, 10, 3),
            &mut state,
            &EmptyTable {
                rows: 2,
                columns: 2,
            },
        );

        assert_eq!(layout.header_area, Some(Rect::new(0, 0, 10, 1)));
        assert_eq!(layout.visible_cells.len(), 6);
        assert_eq!(layout.visible_cells[0].position, CellPosition::header(0));
        assert_eq!(layout.visible_cells[2].position, CellPosition::body(0, 0));
    }

    #[test]
    fn keeps_selected_cell_visible() {
        let table = VirtualTable::new([Constraint::Length(2); 5]).scroll_padding(1);
        let mut state = VirtualTableState::default();
        state.select(Some(CellPosition::body(8, 4)));

        let layout = table.layout(
            Rect::new(0, 0, 4, 4),
            &mut state,
            &EmptyTable {
                rows: 10,
                columns: 5,
            },
        );

        assert_eq!(state.row_scroll(), 7);
        assert_eq!(state.column_scroll(), 2);
        assert_eq!(layout.selected.unwrap().position, CellPosition::body(8, 4));
    }

    #[test]
    fn relative_selection_clamps_to_table_bounds() {
        let mut state = VirtualTableState::default();

        assert_eq!(
            state.select_relative(1, 0, 3, 2),
            Some(CellPosition::body(0, 0))
        );
        assert!(state.scrolls_selected_into_view());
        assert_eq!(
            state.select_relative(1, 1, 3, 2),
            Some(CellPosition::body(1, 1))
        );
        assert_eq!(
            state.select_relative(99, 99, 3, 2),
            Some(CellPosition::body(2, 1))
        );
        assert_eq!(
            state.select_relative(-99, -99, 3, 2),
            Some(CellPosition::body(0, 0))
        );
        assert_eq!(state.select_relative(1, 0, 0, 2), None);
        assert_eq!(state.selected(), None);
    }

    #[test]
    fn row_scroll_does_not_snap_back_to_selection() {
        let table = VirtualTable::new([Constraint::Length(2); 5]).scroll_padding(1);
        let mut state = VirtualTableState::default();
        state.select(Some(CellPosition::body(0, 0)));
        state.scroll_rows_by(6);

        let layout = table.layout(
            Rect::new(0, 0, 4, 4),
            &mut state,
            &EmptyTable {
                rows: 10,
                columns: 5,
            },
        );

        assert_eq!(state.selected(), Some(CellPosition::body(0, 0)));
        assert_eq!(state.row_scroll(), 6);
        assert_eq!(layout.selected, None);
    }

    #[test]
    fn hit_test_returns_cell_position() {
        let table = VirtualTable::new([Constraint::Length(4), Constraint::Length(4)]);
        let mut state = VirtualTableState::default();
        let layout = table.layout(
            Rect::new(0, 0, 8, 2),
            &mut state,
            &EmptyTable {
                rows: 1,
                columns: 2,
            },
        );

        assert_eq!(
            layout.hit_test((5, 1)).unwrap().id,
            CellPosition::body(0, 1)
        );
    }

    #[test]
    fn cell_plan_preserves_visible_cell_geometry() {
        let table = VirtualTable::new([Constraint::Length(4), Constraint::Length(4)]);
        let mut state = VirtualTableState::default();
        let layout = table.layout(
            Rect::new(0, 0, 8, 2),
            &mut state,
            &EmptyTable {
                rows: 1,
                columns: 2,
            },
        );
        let plan = layout.cell_regions();

        assert_eq!(plan.area(), Rect::new(0, 0, 8, 2));
        assert_eq!(plan.hit_test((1, 0)).unwrap().id, CellPosition::header(0));
        assert_eq!(plan.hit_test((5, 1)).unwrap().id, CellPosition::body(0, 1));
    }

    #[test]
    fn cells_plan_maps_positions_to_app_ids() {
        let table = VirtualTable::new([Constraint::Length(4), Constraint::Length(4)]);
        let mut state = VirtualTableState::default();
        let layout = table.layout(
            Rect::new(0, 0, 8, 2),
            &mut state,
            &EmptyTable {
                rows: 1,
                columns: 2,
            },
        );
        let plan = layout.cells_regions(|position| match position.row {
            Some(row) => ("body", row, position.column),
            None => ("header", 0, position.column),
        });

        assert_eq!(plan.hit_test((1, 0)).unwrap().id, ("header", 0, 0));
        assert_eq!(plan.hit_test((5, 1)).unwrap().id, ("body", 0, 1));
    }

    #[test]
    fn select_hit_selects_body_cells_but_not_headers() {
        let table = VirtualTable::new([Constraint::Length(4), Constraint::Length(4)]);
        let mut state = VirtualTableState::default();
        let layout = table.layout(
            Rect::new(0, 0, 8, 2),
            &mut state,
            &EmptyTable {
                rows: 1,
                columns: 2,
            },
        );

        assert_eq!(layout.select_hit((5, 0), &mut state), None);
        assert_eq!(state.selected(), None);
        assert_eq!(
            layout.select_hit((5, 1), &mut state),
            Some(CellPosition::body(0, 1))
        );
        assert_eq!(state.selected(), Some(CellPosition::body(0, 1)));
    }
}
