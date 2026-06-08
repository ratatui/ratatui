//! Grid regions and typed cell coordinates.
//!
//! A [`Grid`] composes Ratatui row and column constraints into a row-major [`Regions`]. It is a
//! geometry helper, not a table widget: it does not own rows, cells, headers, styles, or selection.
//! Callers use the resulting regions to render their own content.
//!
//! For a static grid that is rendered immediately, nested Ratatui [`Layout`] calls are usually
//! enough. `Grid` is useful when cells need row-major ids, hit testing, or one [`Regions`] value
//! that can be inspected before rendering.
//! Use Ratatui's built-in table widget when the content is genuinely tabular data with row/column
//! styling handled by the widget; this module solves cell regions for app-owned content.
//!
//! # Common uses
//!
//! - Render a palette, dashboard, or toolbar where each cell is app-owned content.
//! - Use row-major [`Grid::regions`] when simple indexed ids are enough.
//! - Use typed [`Grid::layout`] when focus, pointer, or selection state should talk in
//!   [`GridPosition`] values instead of encoded indexes.
//! - Inspect row and column areas for decorations that span an entire row or column.
//!
//! # Types
//!
//! - [`GridPosition`] is a typed `{ row, column }` cell id for focus, pointer, and selection state.
//! - [`GridLayout`] is the solved typed-grid layout with row areas, column areas, and cell regions.
//! - [`Grid`] stores row and column constraints and can solve either indexed or typed cell values.
//!
//! # Examples
//!
//! ```rust
//! use ratatui_core::layout::{Constraint, Rect};
//! use ratatui_layout::{Grid, GridPosition, SelectionMode, SelectionState};
//!
//! let row_heights = [Constraint::Length(1), Constraint::Length(1)];
//! let column_widths = [Constraint::Length(4), Constraint::Length(4)];
//! let layout = Grid::new(row_heights, column_widths).layout(Rect::new(0, 0, 8, 2));
//! let visible_cells = layout
//!     .cells()
//!     .regions()
//!     .iter()
//!     .map(|region| region.id)
//!     .collect::<Vec<_>>();
//! let mut selection = SelectionState::new(SelectionMode::Single);
//!
//! selection.select_next(&visible_cells);
//! assert_eq!(selection.primary(), Some(GridPosition::new(0, 0)));
//! ```
//!
//! See [`crate::docs::containers`] for how grids fit with rows, columns, overlays, and nested
//! composition. See [`crate::docs::interaction`] for examples of pairing typed grid cells with
//! focus, pointer, and selection state.
//!
//! [`Layout`]: ratatui_core::layout::Layout
//! [`Regions`]: crate::regions::Regions

use alloc::vec::Vec;

use ratatui_core::layout::{Constraint, Layout, Position, Rect};

use crate::regions::{Region, Regions};

/// Typed row-column identity for a grid cell.
///
/// [`GridPosition`] exists when a cell id should say what it means instead of encoding row-major
/// math in a `usize`. It owns no cell data; it is only the frame-local identity used by
/// [`GridLayout`].
///
/// # Common uses
///
/// - Store selected or focused palette cells without decoding `usize` math.
/// - Route pointer hits from [`GridLayout::cell_at`] to app state using row and column fields.
/// - Use the same id type across [`crate::FocusTargets`], [`crate::PointerTargets`], and
///   [`crate::SelectionState`].
///
/// # Constructor
///
/// - [`GridPosition::new`] creates a typed cell id from zero-based row and column indexes.
///
/// # Examples
///
/// Use typed cell ids with selection and pointer routing instead of encoding row-major indexes:
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::{
///     GridPosition, PointerTarget, PointerTargets, SelectionMode, SelectionState,
/// };
///
/// let cell = GridPosition::new(1, 2);
/// let mouse = PointerTargets::new().target(PointerTarget::new(cell, Rect::new(8, 1, 4, 1)));
/// let mut selection = SelectionState::new(SelectionMode::Single);
///
/// selection.select(mouse.hit_test((9, 1)).unwrap().id);
///
/// assert_eq!(
///     selection.primary(),
///     Some(GridPosition { row: 1, column: 2 })
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GridPosition {
    /// Zero-based row index.
    ///
    /// This indexes the row constraints passed to [`Grid::new`].
    pub row: usize,
    /// Zero-based column index.
    ///
    /// This indexes the column constraints passed to [`Grid::new`].
    pub column: usize,
}

impl GridPosition {
    /// Creates a grid position from a zero-based row and column.
    ///
    /// Use this when tests or app code need to name a typed cell explicitly.
    ///
    /// # Examples
    ///
    /// Store selection using a typed cell identity instead of row-major math:
    ///
    /// ```rust
    /// use ratatui_layout::{GridPosition, SelectionMode, SelectionState};
    ///
    /// let mut selection = SelectionState::new(SelectionMode::Single);
    /// selection.select(GridPosition::new(1, 2));
    ///
    /// assert_eq!(
    ///     selection.primary(),
    ///     Some(GridPosition { row: 1, column: 2 })
    /// );
    /// ```
    pub const fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

/// Solved grid geometry for one frame.
///
/// [`GridLayout`] owns row areas, column areas, and a typed cell [`Regions`]. It does not own
/// table data, headers, selection, or rendering state. Use it when app logic wants to talk about
/// cells as `{ row, column }` rather than row-major indexes.
///
/// # Common uses
///
/// - Build a [`crate::FocusTargets`] or [`crate::PointerTargets`] from [`GridLayout::cells`].
/// - Draw row or column decorations using [`GridLayout::row_areas`] and
///   [`GridLayout::column_areas`].
/// - Hit test cells with [`GridLayout::cell_at`] and update app-owned selection.
///
/// # Inspection and routing
///
/// - [`GridLayout::area`] returns the area the grid was solved against.
/// - [`GridLayout::row_areas`] returns row-spanning rectangles.
/// - [`GridLayout::column_areas`] returns column-spanning rectangles.
/// - [`GridLayout::cells`] returns the typed [`Regions`] of cell regions.
/// - [`GridLayout::cell_at`] hit-tests a terminal position and returns a [`GridPosition`].
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::{Constraint, Rect};
/// use ratatui_layout::{Grid, GridPosition};
///
/// let row_heights = [Constraint::Length(1), Constraint::Length(1)];
/// let column_widths = [Constraint::Length(3), Constraint::Length(3)];
/// let layout = Grid::new(row_heights, column_widths).layout(Rect::new(0, 0, 6, 2));
///
/// assert_eq!(layout.cell_at((4, 1)), Some(GridPosition::new(1, 1)));
/// assert_eq!(layout.row_areas()[1], Rect::new(0, 1, 6, 1));
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GridLayout {
    area: Rect,
    row_areas: Vec<Rect>,
    column_areas: Vec<Rect>,
    cells: Regions<GridPosition>,
}

impl GridLayout {
    /// Returns the area the grid was solved for.
    ///
    /// This is the parent area, not the union of visible cells.
    ///
    /// # Examples
    ///
    /// Keep the full grid area for drawing a shared background:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::Grid;
    ///
    /// let layout =
    ///     Grid::new([Constraint::Length(1)], [Constraint::Length(4)]).layout(Rect::new(2, 3, 10, 4));
    ///
    /// assert_eq!(layout.area(), Rect::new(2, 3, 10, 4));
    /// ```
    pub const fn area(&self) -> Rect {
        self.area
    }

    /// Returns the solved row areas.
    ///
    /// These are useful for row backgrounds, row separators, or diagnostics that span all columns.
    ///
    /// # Examples
    ///
    /// Draw row-spanning selection or separator regions:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::Grid;
    ///
    /// let row_heights = [Constraint::Length(1), Constraint::Length(2)];
    /// let column_widths = [Constraint::Fill(1)];
    /// let layout = Grid::new(row_heights, column_widths).layout(Rect::new(0, 0, 10, 3));
    ///
    /// assert_eq!(layout.row_areas()[1], Rect::new(0, 1, 10, 2));
    /// ```
    pub fn row_areas(&self) -> &[Rect] {
        &self.row_areas
    }

    /// Returns the solved column areas.
    ///
    /// These are useful for column headers, guides, or pointer affordances that span all rows.
    ///
    /// # Examples
    ///
    /// Use column areas for column-wide hover guides or headers:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::Grid;
    ///
    /// let row_heights = [Constraint::Fill(1)];
    /// let column_widths = [Constraint::Length(3), Constraint::Length(5)];
    /// let layout = Grid::new(row_heights, column_widths).layout(Rect::new(0, 0, 8, 4));
    ///
    /// assert_eq!(layout.column_areas()[1], Rect::new(3, 0, 5, 4));
    /// ```
    pub fn column_areas(&self) -> &[Rect] {
        &self.column_areas
    }

    /// Returns the typed cell regions.
    ///
    /// Iterate these regions to render app-owned cell content or derive focus and pointer targets.
    ///
    /// # Examples
    ///
    /// Build visible ids for selection traversal from the typed cell regions:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::{Grid, GridPosition};
    ///
    /// let row_heights = [Constraint::Length(1)];
    /// let column_widths = [Constraint::Length(2), Constraint::Length(2)];
    /// let layout = Grid::new(row_heights, column_widths).layout(Rect::new(0, 0, 4, 1));
    /// let ids: Vec<_> = layout
    ///     .cells()
    ///     .regions()
    ///     .iter()
    ///     .map(|region| region.id)
    ///     .collect();
    ///
    /// assert_eq!(ids, [GridPosition::new(0, 0), GridPosition::new(0, 1)]);
    /// ```
    pub const fn cells(&self) -> &Regions<GridPosition> {
        &self.cells
    }

    /// Returns the topmost cell at a terminal position.
    ///
    /// This is a convenience wrapper around [`Regions::hit_test`] for callers that only need the
    /// typed cell id.
    ///
    /// # Examples
    ///
    /// Route a pointer position to the typed grid cell under the pointer:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::{Grid, GridPosition};
    ///
    /// let row_heights = [Constraint::Length(1), Constraint::Length(1)];
    /// let column_widths = [Constraint::Length(4), Constraint::Length(4)];
    /// let layout = Grid::new(row_heights, column_widths).layout(Rect::new(0, 0, 8, 2));
    ///
    /// assert_eq!(layout.cell_at((5, 1)), Some(GridPosition::new(1, 1)));
    /// ```
    pub fn cell_at<P: Into<Position>>(&self, position: P) -> Option<GridPosition> {
        self.cells.hit_test(position).map(|hit| hit.id)
    }
}

/// Grid configuration that assigns one region for each row-column cell.
///
/// Use [`Grid`] when a cell layout needs ids and hit testing but not a full table widget. For
/// example, a dashboard of app-owned tiles can use `Grid` to assign one region per tile, then
/// render each tile from application data.
///
/// Grid regions are assigned in row-major order. For `r` rows and `c` columns, the id for
/// `(row, column)` is `row * c + column`.
///
/// Use [`Grid::layout`] instead when the caller would otherwise need to repeatedly encode and
/// decode row/column pairs.
///
/// # Constructors and solving
///
/// - [`Grid::new`] stores row and column constraints.
/// - [`Grid::regions`] returns a row-major [`Regions`] with integer ids.
/// - [`Grid::layout`] returns [`GridLayout`] with typed [`GridPosition`] ids plus row/column areas.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::{Constraint, Rect};
/// use ratatui_layout::Grid;
///
/// let row_heights = [Constraint::Length(1), Constraint::Length(1)];
/// let column_widths = [Constraint::Length(4), Constraint::Fill(1)];
/// let grid = Grid::new(row_heights, column_widths);
/// let cell_regions = grid.regions(Rect::new(0, 0, 10, 2));
///
/// assert_eq!(cell_regions.regions()[0].id, 0);
/// assert_eq!(cell_regions.regions()[1].id, 1);
/// assert_eq!(cell_regions.regions()[2].id, 2);
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Grid {
    rows: Vec<Constraint>,
    columns: Vec<Constraint>,
}

impl Grid {
    /// Creates a grid from row and column constraints.
    ///
    /// The row constraints are solved vertically first. Each solved row is then split horizontally
    /// with the column constraints.
    ///
    /// # Examples
    ///
    /// Create a dashboard grid from ordinary Ratatui constraints:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::Grid;
    ///
    /// let row_heights = [Constraint::Length(1)];
    /// let column_widths = [Constraint::Length(4), Constraint::Fill(1)];
    /// let grid = Grid::new(row_heights, column_widths);
    /// let plan = grid.regions(Rect::new(0, 0, 10, 1));
    ///
    /// assert_eq!(plan.regions()[0].area.width, 4);
    /// ```
    pub fn new<R, C>(rows: R, columns: C) -> Self
    where
        R: IntoIterator,
        R::Item: Into<Constraint>,
        C: IntoIterator,
        C::Item: Into<Constraint>,
    {
        Self {
            rows: rows.into_iter().map(Into::into).collect(),
            columns: columns.into_iter().map(Into::into).collect(),
        }
    }

    /// Solves the grid into row-major regions.
    ///
    /// Empty row or column constraint lists produce empty regions.
    ///
    /// Use this when simple index ids are enough and row/column metadata is not needed.
    ///
    /// # Examples
    ///
    /// Use row-major integer ids for a simple generated grid:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::Grid;
    ///
    /// let row_heights = [Constraint::Length(1), Constraint::Length(1)];
    /// let column_widths = [Constraint::Length(3), Constraint::Length(3)];
    /// let plan = Grid::new(row_heights, column_widths).regions(Rect::new(0, 0, 6, 2));
    ///
    /// assert_eq!(plan.hit_test((4, 1)).unwrap().id, 3);
    /// ```
    pub fn regions(&self, area: Rect) -> Regions {
        let row_areas = Layout::vertical(self.rows.clone()).split(area);
        let mut regions = Vec::with_capacity(row_areas.len().saturating_mul(self.columns.len()));

        for (row_index, row_area) in row_areas.iter().copied().enumerate() {
            for (column_index, cell_area) in Layout::horizontal(self.columns.clone())
                .split(row_area)
                .iter()
                .copied()
                .enumerate()
            {
                let id = row_index * self.columns.len() + column_index;
                regions.push(Region::new(id, cell_area));
            }
        }

        Regions::from_regions(area, regions)
    }

    /// Solves the grid into row areas, column areas, and typed cell ids.
    ///
    /// Empty row or column constraint lists produce empty cell regions. Column areas are solved
    /// against the full grid height so callers can render column affordances that span all rows.
    ///
    /// # Examples
    ///
    /// Use typed cell ids when focus, mouse, and selection should share one coordinate type:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::{Grid, GridPosition, SelectionMode, SelectionState};
    ///
    /// let row_heights = [Constraint::Length(1), Constraint::Length(1)];
    /// let column_widths = [Constraint::Length(2), Constraint::Length(2)];
    /// let layout = Grid::new(row_heights, column_widths).layout(Rect::new(0, 0, 4, 2));
    /// let visible: Vec<_> = layout
    ///     .cells()
    ///     .regions()
    ///     .iter()
    ///     .map(|region| region.id)
    ///     .collect();
    /// let mut selection = SelectionState::new(SelectionMode::Single);
    /// selection.select_next(&visible);
    ///
    /// assert_eq!(selection.primary(), Some(GridPosition::new(0, 0)));
    /// ```
    pub fn layout(&self, area: Rect) -> GridLayout {
        let row_areas = Layout::vertical(self.rows.clone()).split(area).to_vec();
        let column_areas = Layout::horizontal(self.columns.clone())
            .split(area)
            .to_vec();
        let mut regions = Vec::with_capacity(row_areas.len().saturating_mul(self.columns.len()));

        for (row_index, row_area) in row_areas.iter().copied().enumerate() {
            for (column_index, cell_area) in Layout::horizontal(self.columns.clone())
                .split(row_area)
                .iter()
                .copied()
                .enumerate()
            {
                regions.push(Region::new(
                    GridPosition::new(row_index, column_index),
                    cell_area,
                ));
            }
        }

        GridLayout {
            area,
            row_areas,
            column_areas,
            cells: Regions::from_regions(area, regions),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use ratatui_core::layout::{Constraint, Rect};

    use super::*;

    #[test]
    fn grid_uses_row_major_ids() {
        let row_heights = [Constraint::Length(1), Constraint::Length(1)];
        let column_widths = [Constraint::Length(2), Constraint::Length(3)];
        let plan = Grid::new(row_heights, column_widths).regions(Rect::new(0, 0, 5, 2));

        assert_eq!(plan.regions()[0].area, Rect::new(0, 0, 2, 1));
        assert_eq!(plan.regions()[1].area, Rect::new(2, 0, 3, 1));
        assert_eq!(plan.regions()[2].area, Rect::new(0, 1, 2, 1));
        assert_eq!(plan.regions()[3].area, Rect::new(2, 1, 3, 1));
    }

    #[test]
    fn grid_layout_uses_typed_cell_ids() {
        let row_heights = [Constraint::Length(1), Constraint::Length(1)];
        let column_widths = [Constraint::Length(2), Constraint::Length(3)];
        let layout = Grid::new(row_heights, column_widths).layout(Rect::new(0, 0, 5, 2));

        assert_eq!(
            layout.row_areas(),
            &[Rect::new(0, 0, 5, 1), Rect::new(0, 1, 5, 1)]
        );
        assert_eq!(
            layout.column_areas(),
            &[Rect::new(0, 0, 2, 2), Rect::new(2, 0, 3, 2)]
        );
        assert_eq!(layout.cells().regions()[3].id, GridPosition::new(1, 1));
        assert_eq!(layout.cell_at((3, 1)), Some(GridPosition::new(1, 1)));
    }

    #[test]
    fn empty_grid_has_no_cells() {
        let rows: [Constraint; 0] = [];
        let layout = Grid::new(rows, [Constraint::Length(1)]).layout(Rect::new(0, 0, 5, 2));

        assert!(layout.row_areas().is_empty());
        assert!(layout.cells().is_empty());
    }
}
