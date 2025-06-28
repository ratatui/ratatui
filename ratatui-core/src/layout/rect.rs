#![warn(missing_docs)]
use core::array::TryFromSliceError;
use core::cmp::{max, min};
use core::fmt;

use crate::layout::{Margin, Position, Size};

mod iter;
pub use iter::*;

use super::{Constraint, Flex, Layout};

/// A rectangular area in the terminal.
///
/// A `Rect` represents a rectangular region in the terminal coordinate system, defined by its
/// top-left corner position and dimensions. This is the fundamental building block for all layout
/// operations and widget rendering in Ratatui.
///
/// Rectangles are used throughout the layout system to define areas where widgets can be rendered.
/// They are typically created by [`Layout`] operations that divide terminal space, but can also be
/// manually constructed for specific positioning needs.
///
/// The coordinate system uses the top-left corner as the origin (0, 0), with x increasing to the
/// right and y increasing downward. All measurements are in character cells.
///
/// # Construction and Conversion
///
/// - [`new`](Self::new) - Create a new rectangle from coordinates and dimensions
/// - [`as_position`](Self::as_position) - Convert to a position at the top-left corner
/// - [`as_size`](Self::as_size) - Convert to a size representing the dimensions
/// - [`from((Position, Size))`](Self::from) - Create from `(Position, Size)` tuple
/// - [`from(((u16, u16), (u16, u16)))`](Self::from) - Create from `((u16, u16), (u16, u16))`
///   coordinate and dimension tuples
/// - [`into((Position, Size))`] - Convert to `(Position, Size)` tuple
/// - [`default`](Self::default) - Create a zero-sized rectangle at origin
///
/// # Geometry and Properties
///
/// - [`area`](Self::area) - Calculate the total area in character cells
/// - [`is_empty`](Self::is_empty) - Check if the rectangle has zero area
/// - [`left`](Self::left), [`right`](Self::right), [`top`](Self::top), [`bottom`](Self::bottom) -
///   Get edge coordinates
///
/// # Spatial Operations
///
/// - [`inner`](Self::inner), [`outer`](Self::outer) - Apply margins to shrink or expand
/// - [`offset`](Self::offset) - Move the rectangle by a relative amount
/// - [`union`](Self::union) - Combine with another rectangle to create a bounding box
/// - [`intersection`](Self::intersection) - Find the overlapping area with another rectangle
/// - [`clamp`](Self::clamp) - Constrain the rectangle to fit within another
///
/// # Positioning and Centering
///
/// - [`centered_horizontally`](Self::centered_horizontally) - Center horizontally within a
///   constraint
/// - [`centered_vertically`](Self::centered_vertically) - Center vertically within a constraint
/// - [`centered`](Self::centered) - Center both horizontally and vertically
///
/// # Testing and Iteration
///
/// - [`contains`](Self::contains) - Check if a position is within the rectangle
/// - [`intersects`](Self::intersects) - Check if it overlaps with another rectangle
/// - [`rows`](Self::rows) - Iterate over horizontal rows within the rectangle
/// - [`columns`](Self::columns) - Iterate over vertical columns within the rectangle
/// - [`positions`](Self::positions) - Iterate over all positions within the rectangle
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::{Position, Rect, Size};
///
/// // Create a rectangle manually
/// let rect = Rect::new(10, 5, 80, 20);
/// assert_eq!(rect.x, 10);
/// assert_eq!(rect.y, 5);
/// assert_eq!(rect.width, 80);
/// assert_eq!(rect.height, 20);
///
/// // Create from position and size
/// let rect = Rect::from((Position::new(10, 5), Size::new(80, 20)));
/// ```
///
/// For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    /// The x coordinate of the top left corner of the `Rect`.
    pub x: u16,
    /// The y coordinate of the top left corner of the `Rect`.
    pub y: u16,
    /// The width of the `Rect`.
    pub width: u16,
    /// The height of the `Rect`.
    pub height: u16,
}

/// Amounts by which to move a [`Rect`](crate::layout::Rect).
///
/// Positive numbers move to the right/bottom and negative to the left/top.
///
/// See [`Rect::offset`]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Offset {
    /// How much to move on the X axis
    pub x: i32,
    /// How much to move on the Y axis
    pub y: i32,
}

impl Offset {
    /// A zero offset
    pub const ZERO: Self = Self { x: 0, y: 0 };

    /// Creates a new `Offset` with the given values.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}+{}+{}", self.width, self.height, self.x, self.y)
    }
}

impl Rect {
    /// A zero sized Rect at position 0,0
    pub const ZERO: Self = Self {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };

    /// Creates a new `Rect`, with width and height limited to keep both bounds within `u16`.
    ///
    /// If the width or height would cause the right or bottom coordinate to be larger than the
    /// maximum value of `u16`, the width or height will be clamped to keep the right or bottom
    /// coordinate within `u16`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::layout::Rect;
    ///
    /// let rect = Rect::new(1, 2, 3, 4);
    /// ```
    pub const fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        // these calculations avoid using min so that this function can be const
        let max_width = u16::MAX - x;
        let max_height = u16::MAX - y;
        let width = if width > max_width { max_width } else { width };
        let height = if height > max_height {
            max_height
        } else {
            height
        };
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// The area of the `Rect`. If the area is larger than the maximum value of `u16`, it will be
    /// clamped to `u16::MAX`.
    pub const fn area(self) -> u32 {
        (self.width as u32) * (self.height as u32)
    }

    /// Returns true if the `Rect` has no area.
    pub const fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Returns the left coordinate of the `Rect`.
    pub const fn left(self) -> u16 {
        self.x
    }

    /// Returns the right coordinate of the `Rect`. This is the first coordinate outside of the
    /// `Rect`.
    ///
    /// If the right coordinate is larger than the maximum value of u16, it will be clamped to
    /// `u16::MAX`.
    pub const fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Returns the top coordinate of the `Rect`.
    pub const fn top(self) -> u16 {
        self.y
    }

    /// Returns the bottom coordinate of the `Rect`. This is the first coordinate outside of the
    /// `Rect`.
    ///
    /// If the bottom coordinate is larger than the maximum value of u16, it will be clamped to
    /// `u16::MAX`.
    pub const fn bottom(self) -> u16 {
        self.y.saturating_add(self.height)
    }

    /// Returns a new `Rect` inside the current one, with the given margin on each side.
    ///
    /// If the margin is larger than the `Rect`, the returned `Rect` will have no area.
    #[must_use = "method returns the modified value"]
    pub const fn inner(self, margin: Margin) -> Self {
        let doubled_margin_horizontal = margin.horizontal.saturating_mul(2);
        let doubled_margin_vertical = margin.vertical.saturating_mul(2);

        if self.width < doubled_margin_horizontal || self.height < doubled_margin_vertical {
            Self::ZERO
        } else {
            Self {
                x: self.x.saturating_add(margin.horizontal),
                y: self.y.saturating_add(margin.vertical),
                width: self.width.saturating_sub(doubled_margin_horizontal),
                height: self.height.saturating_sub(doubled_margin_vertical),
            }
        }
    }

    /// Returns a new `Rect` outside the current one, with the given margin applied on each side.
    ///
    /// If the margin causes the `Rect`'s bounds to outsdie the range of a `u16`, the `Rect` will
    /// be truncated to keep the bounds within `u16`. This will cause the size of the `Rect` to
    /// change.
    ///
    /// The generated `Rect` may not fit inside the buffer or containing area, so it consider
    /// constraining the resulting `Rect` with [`Rect::clamp`] before using it.
    #[must_use = "method returns the modified value"]
    pub const fn outer(self, margin: Margin) -> Self {
        let x = self.x.saturating_sub(margin.horizontal);
        let y = self.y.saturating_sub(margin.vertical);
        let width = self
            .right()
            .saturating_add(margin.horizontal)
            .saturating_sub(x);
        let height = self
            .bottom()
            .saturating_add(margin.vertical)
            .saturating_sub(y);
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Moves the `Rect` without modifying its size.
    ///
    /// Moves the `Rect` according to the given offset without modifying its [`width`](Rect::width)
    /// or [`height`](Rect::height).
    /// - Positive `x` moves the whole `Rect` to the right, negative to the left.
    /// - Positive `y` moves the whole `Rect` to the bottom, negative to the top.
    ///
    /// See [`Offset`] for details.
    #[must_use = "method returns the modified value"]
    pub fn offset(self, offset: Offset) -> Self {
        Self {
            x: i32::from(self.x)
                .saturating_add(offset.x)
                .clamp(0, i32::from(u16::MAX - self.width)) as u16,
            y: i32::from(self.y)
                .saturating_add(offset.y)
                .clamp(0, i32::from(u16::MAX - self.height)) as u16,
            ..self
        }
    }

    /// Returns a new `Rect` that contains both the current one and the given one.
    #[must_use = "method returns the modified value"]
    pub fn union(self, other: Self) -> Self {
        let x1 = min(self.x, other.x);
        let y1 = min(self.y, other.y);
        let x2 = max(self.right(), other.right());
        let y2 = max(self.bottom(), other.bottom());
        Self {
            x: x1,
            y: y1,
            width: x2.saturating_sub(x1),
            height: y2.saturating_sub(y1),
        }
    }

    /// Returns a new `Rect` that is the intersection of the current one and the given one.
    ///
    /// If the two `Rect`s do not intersect, the returned `Rect` will have no area.
    #[must_use = "method returns the modified value"]
    pub fn intersection(self, other: Self) -> Self {
        let x1 = max(self.x, other.x);
        let y1 = max(self.y, other.y);
        let x2 = min(self.right(), other.right());
        let y2 = min(self.bottom(), other.bottom());
        Self {
            x: x1,
            y: y1,
            width: x2.saturating_sub(x1),
            height: y2.saturating_sub(y1),
        }
    }

    /// Returns true if the two `Rect`s intersect.
    pub const fn intersects(self, other: Self) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }

    /// Returns true if the given position is inside the `Rect`.
    ///
    /// The position is considered inside the `Rect` if it is on the `Rect`'s border.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Position, Rect};
    ///
    /// let rect = Rect::new(1, 2, 3, 4);
    /// assert!(rect.contains(Position { x: 1, y: 2 }));
    /// ````
    pub const fn contains(self, position: Position) -> bool {
        position.x >= self.x
            && position.x < self.right()
            && position.y >= self.y
            && position.y < self.bottom()
    }

    /// Clamp this `Rect` to fit inside the other `Rect`.
    ///
    /// If the width or height of this `Rect` is larger than the other `Rect`, it will be clamped to
    /// the other `Rect`'s width or height.
    ///
    /// If the left or top coordinate of this `Rect` is smaller than the other `Rect`, it will be
    /// clamped to the other `Rect`'s left or top coordinate.
    ///
    /// If the right or bottom coordinate of this `Rect` is larger than the other `Rect`, it will be
    /// clamped to the other `Rect`'s right or bottom coordinate.
    ///
    /// This is different from [`Rect::intersection`] because it will move this `Rect` to fit inside
    /// the other `Rect`, while [`Rect::intersection`] instead would keep this `Rect`'s position and
    /// truncate its size to only that which is inside the other `Rect`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    ///
    /// let area = Rect::new(0, 0, 100, 100);
    /// let rect = Rect::new(80, 80, 30, 30).clamp(area);
    /// assert_eq!(rect, Rect::new(70, 70, 30, 30));
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn clamp(self, other: Self) -> Self {
        let width = self.width.min(other.width);
        let height = self.height.min(other.height);
        let x = self.x.clamp(other.x, other.right().saturating_sub(width));
        let y = self.y.clamp(other.y, other.bottom().saturating_sub(height));
        Self::new(x, y, width, height)
    }

    /// An iterator over rows within the `Rect`.
    ///
    /// Each row is a full `Rect` region with height 1 that can be used for rendering widgets
    /// or as input to further layout methods.
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::{Constraint, Layout, Rect};
    /// use ratatui_core::widgets::Widget;
    ///
    /// fn render_list(area: Rect, buf: &mut Buffer) {
    ///     // Renders "Item 0", "Item 1", etc. in each row
    ///     for (i, row) in area.rows().enumerate() {
    ///         format!("Item {i}").render(row, buf);
    ///     }
    /// }
    ///
    /// fn render_with_nested_layout(area: Rect, buf: &mut Buffer) {
    ///     // Splits each row into left/right areas and renders labels and content
    ///     for (i, row) in area.rows().take(3).enumerate() {
    ///         let [left, right] =
    ///             Layout::horizontal([Constraint::Percentage(30), Constraint::Fill(1)]).areas(row);
    ///
    ///         format!("{i}:").render(left, buf);
    ///         "Content".render(right, buf);
    ///     }
    /// }
    /// ```
    pub const fn rows(self) -> Rows {
        Rows::new(self)
    }

    /// An iterator over columns within the `Rect`.
    ///
    /// Each column is a full `Rect` region with width 1 that can be used for rendering widgets
    /// or as input to further layout methods.
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_core::widgets::Widget;
    ///
    /// fn render_columns(area: Rect, buf: &mut Buffer) {
    ///     // Renders column indices (0-9 repeating) in each column
    ///     for (i, column) in area.columns().enumerate() {
    ///         format!("{}", i % 10).render(column, buf);
    ///     }
    /// }
    /// ```
    pub const fn columns(self) -> Columns {
        Columns::new(self)
    }

    /// An iterator over the positions within the `Rect`.
    ///
    /// The positions are returned in a row-major order (left-to-right, top-to-bottom).
    /// Each position is a `Position` that represents a single cell coordinate.
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::{Position, Rect};
    /// use ratatui_core::widgets::Widget;
    ///
    /// fn render_positions(area: Rect, buf: &mut Buffer) {
    ///     // Renders position indices (0-9 repeating) at each cell position
    ///     for (i, position) in area.positions().enumerate() {
    ///         buf[position].set_symbol(&format!("{}", i % 10));
    ///     }
    /// }
    /// ```
    pub const fn positions(self) -> Positions {
        Positions::new(self)
    }

    /// Returns a [`Position`] with the same coordinates as this `Rect`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::layout::Rect;
    ///
    /// let rect = Rect::new(1, 2, 3, 4);
    /// let position = rect.as_position();
    /// ````
    pub const fn as_position(self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }

    /// Converts the `Rect` into a size struct.
    pub const fn as_size(self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    /// Returns a new Rect, centered horizontally based on the provided constraint.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::layout::Constraint;
    /// use ratatui_core::terminal::Frame;
    ///
    /// fn render(frame: &mut Frame) {
    ///     let area = frame.area().centered_horizontally(Constraint::Ratio(1, 2));
    /// }
    /// ```
    #[must_use]
    pub fn centered_horizontally(self, constraint: Constraint) -> Self {
        let [area] = self.layout(&Layout::horizontal([constraint]).flex(Flex::Center));
        area
    }

    /// Returns a new Rect, centered vertically based on the provided constraint.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::layout::Constraint;
    /// use ratatui_core::terminal::Frame;
    ///
    /// fn render(frame: &mut Frame) {
    ///     let area = frame.area().centered_vertically(Constraint::Ratio(1, 2));
    /// }
    /// ```
    #[must_use]
    pub fn centered_vertically(self, constraint: Constraint) -> Self {
        let [area] = self.layout(&Layout::vertical([constraint]).flex(Flex::Center));
        area
    }

    /// Returns a new Rect, centered horizontally and vertically based on the provided constraints.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::layout::Constraint;
    /// use ratatui_core::terminal::Frame;
    ///
    /// fn render(frame: &mut Frame) {
    ///     let area = frame
    ///         .area()
    ///         .centered(Constraint::Ratio(1, 2), Constraint::Ratio(1, 3));
    /// }
    /// ```
    #[must_use]
    pub fn centered(
        self,
        horizontal_constraint: Constraint,
        vertical_constraint: Constraint,
    ) -> Self {
        self.centered_horizontally(horizontal_constraint)
            .centered_vertically(vertical_constraint)
    }

    /// Split the rect into a number of sub-rects according to the given [`Layout`].
    ///
    /// An ergonomic wrapper around [`Layout::split`] that returns an array of `Rect`s instead of
    /// `Rc<[Rect]>`.
    ///
    /// This method requires the number of constraints to be known at compile time. If you don't
    /// know the number of constraints at compile time, use [`Layout::split`] instead.
    ///
    /// # Panics
    ///
    /// Panics if the number of constraints is not equal to the length of the returned array.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::layout::{Constraint, Layout, Rect};
    ///
    /// let area = Rect::new(0, 0, 10, 10);
    /// let layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
    /// let [top, main] = area.layout(&layout);
    /// assert_eq!(top, Rect::new(0, 0, 10, 1));
    /// assert_eq!(main, Rect::new(0, 1, 10, 9));
    ///
    /// // or explicitly specify the number of constraints:
    /// let areas = area.layout::<2>(&layout);
    /// assert_eq!(areas, [Rect::new(0, 0, 10, 1), Rect::new(0, 1, 10, 9),]);
    /// ```
    #[must_use]
    pub fn layout<const N: usize>(self, layout: &Layout) -> [Self; N] {
        let areas = layout.split(self);
        areas.as_ref().try_into().unwrap_or_else(|_| {
            panic!(
                "invalid number of rects: expected {N}, found {}",
                areas.len()
            )
        })
    }

    /// Split the rect into a number of sub-rects according to the given [`Layout`].
    ///
    /// An ergonomic wrapper around [`Layout::split`] that returns a [`Vec`] of `Rect`s instead of
    /// `Rc<[Rect]>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::layout::{Constraint, Layout, Rect};
    ///
    /// let area = Rect::new(0, 0, 10, 10);
    /// let layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
    /// let areas = area.layout_vec(&layout);
    /// assert_eq!(areas, vec![Rect::new(0, 0, 10, 1), Rect::new(0, 1, 10, 9),]);
    /// ```
    ///
    /// [`Vec`]: alloc::vec::Vec
    #[must_use]
    pub fn layout_vec(self, layout: &Layout) -> alloc::vec::Vec<Self> {
        layout.split(self).as_ref().to_vec()
    }

    /// Try to split the rect into a number of sub-rects according to the given [`Layout`].
    ///
    /// An ergonomic wrapper around [`Layout::split`] that returns an array of `Rect`s instead of
    /// `Rc<[Rect]>`.
    ///
    /// # Errors
    ///
    /// Returns an error if the number of constraints is not equal to the length of the returned
    /// array.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::layout::{Constraint, Layout, Rect};
    ///
    /// let area = Rect::new(0, 0, 10, 10);
    /// let layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
    /// let [top, main] = area.try_layout(&layout)?;
    /// assert_eq!(top, Rect::new(0, 0, 10, 1));
    /// assert_eq!(main, Rect::new(0, 1, 10, 9));
    ///
    /// // or explicitly specify the number of constraints:
    /// let areas = area.try_layout::<2>(&layout)?;
    /// assert_eq!(areas, [Rect::new(0, 0, 10, 1), Rect::new(0, 1, 10, 9),]);
    /// # Ok::<(), core::array::TryFromSliceError>(())
    /// ``````
    pub fn try_layout<const N: usize>(
        self,
        layout: &Layout,
    ) -> Result<[Self; N], TryFromSliceError> {
        layout.split(self).as_ref().try_into()
    }

    /// indents the x value of the `Rect` by a given `offset`
    ///
    /// This is pub(crate) for now as we need to stabilize the naming / design of this API.
    #[must_use]
    pub(crate) const fn indent_x(self, offset: u16) -> Self {
        Self {
            x: self.x.saturating_add(offset),
            width: self.width.saturating_sub(offset),
            ..self
        }
    }
}

impl From<(Position, Size)> for Rect {
    fn from((position, size): (Position, Size)) -> Self {
        Self {
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use alloc::vec;
    use alloc::vec::Vec;

    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;
    use crate::layout::{Constraint, Layout};

    #[test]
    fn to_string() {
        assert_eq!(Rect::new(1, 2, 3, 4).to_string(), "3x4+1+2");
    }

    #[test]
    fn new() {
        assert_eq!(
            Rect::new(1, 2, 3, 4),
            Rect {
                x: 1,
                y: 2,
                width: 3,
                height: 4
            }
        );
    }

    #[test]
    fn area() {
        assert_eq!(Rect::new(1, 2, 3, 4).area(), 12);
    }

    #[test]
    fn is_empty() {
        assert!(!Rect::new(1, 2, 3, 4).is_empty());
        assert!(Rect::new(1, 2, 0, 4).is_empty());
        assert!(Rect::new(1, 2, 3, 0).is_empty());
    }

    #[test]
    fn left() {
        assert_eq!(Rect::new(1, 2, 3, 4).left(), 1);
    }

    #[test]
    fn right() {
        assert_eq!(Rect::new(1, 2, 3, 4).right(), 4);
    }

    #[test]
    fn top() {
        assert_eq!(Rect::new(1, 2, 3, 4).top(), 2);
    }

    #[test]
    fn bottom() {
        assert_eq!(Rect::new(1, 2, 3, 4).bottom(), 6);
    }

    #[test]
    fn inner() {
        assert_eq!(
            Rect::new(1, 2, 3, 4).inner(Margin::new(1, 2)),
            Rect::new(2, 4, 1, 0)
        );
    }

    #[test]
    fn outer() {
        // enough space to grow on all sides
        assert_eq!(
            Rect::new(100, 200, 10, 20).outer(Margin::new(20, 30)),
            Rect::new(80, 170, 50, 80)
        );

        // left / top saturation should truncate the size (10 less on left / top)
        assert_eq!(
            Rect::new(10, 20, 10, 20).outer(Margin::new(20, 30)),
            Rect::new(0, 0, 40, 70),
        );

        // right / bottom saturation should truncate the size (10 less on bottom / right)
        assert_eq!(
            Rect::new(u16::MAX - 20, u16::MAX - 40, 10, 20).outer(Margin::new(20, 30)),
            Rect::new(u16::MAX - 40, u16::MAX - 70, 40, 70),
        );
    }

    #[test]
    fn offset() {
        assert_eq!(
            Rect::new(1, 2, 3, 4).offset(Offset { x: 5, y: 6 }),
            Rect::new(6, 8, 3, 4),
        );
    }

    #[test]
    fn negative_offset() {
        assert_eq!(
            Rect::new(4, 3, 3, 4).offset(Offset { x: -2, y: -1 }),
            Rect::new(2, 2, 3, 4),
        );
    }

    #[test]
    fn negative_offset_saturate() {
        assert_eq!(
            Rect::new(1, 2, 3, 4).offset(Offset { x: -5, y: -6 }),
            Rect::new(0, 0, 3, 4),
        );
    }

    /// Offsets a [`Rect`] making it go outside [`u16::MAX`], it should keep its size.
    #[test]
    fn offset_saturate_max() {
        assert_eq!(
            Rect::new(u16::MAX - 500, u16::MAX - 500, 100, 100).offset(Offset { x: 1000, y: 1000 }),
            Rect::new(u16::MAX - 100, u16::MAX - 100, 100, 100),
        );
    }

    #[test]
    fn union() {
        assert_eq!(
            Rect::new(1, 2, 3, 4).union(Rect::new(2, 3, 4, 5)),
            Rect::new(1, 2, 5, 6)
        );
    }

    #[test]
    fn intersection() {
        assert_eq!(
            Rect::new(1, 2, 3, 4).intersection(Rect::new(2, 3, 4, 5)),
            Rect::new(2, 3, 2, 3)
        );
    }

    #[test]
    fn intersection_underflow() {
        assert_eq!(
            Rect::new(1, 1, 2, 2).intersection(Rect::new(4, 4, 2, 2)),
            Rect::new(4, 4, 0, 0)
        );
    }

    #[test]
    fn intersects() {
        assert!(Rect::new(1, 2, 3, 4).intersects(Rect::new(2, 3, 4, 5)));
        assert!(!Rect::new(1, 2, 3, 4).intersects(Rect::new(5, 6, 7, 8)));
    }

    // the bounds of this rect are x: [1..=3], y: [2..=5]
    #[rstest]
    #[case::inside_top_left(Rect::new(1, 2, 3, 4), Position { x: 1, y: 2 }, true)]
    #[case::inside_top_right(Rect::new(1, 2, 3, 4), Position { x: 3, y: 2 }, true)]
    #[case::inside_bottom_left(Rect::new(1, 2, 3, 4), Position { x: 1, y: 5 }, true)]
    #[case::inside_bottom_right(Rect::new(1, 2, 3, 4), Position { x: 3, y: 5 }, true)]
    #[case::outside_left(Rect::new(1, 2, 3, 4), Position { x: 0, y: 2 }, false)]
    #[case::outside_right(Rect::new(1, 2, 3, 4), Position { x: 4, y: 2 }, false)]
    #[case::outside_top(Rect::new(1, 2, 3, 4), Position { x: 1, y: 1 }, false)]
    #[case::outside_bottom(Rect::new(1, 2, 3, 4), Position { x: 1, y: 6 }, false)]
    #[case::outside_top_left(Rect::new(1, 2, 3, 4), Position { x: 0, y: 1 }, false)]
    #[case::outside_bottom_right(Rect::new(1, 2, 3, 4), Position { x: 4, y: 6 }, false)]
    fn contains(#[case] rect: Rect, #[case] position: Position, #[case] expected: bool) {
        assert_eq!(
            rect.contains(position),
            expected,
            "rect: {rect:?}, position: {position:?}",
        );
    }

    #[test]
    fn size_truncation() {
        assert_eq!(
            Rect::new(u16::MAX - 100, u16::MAX - 1000, 200, 2000),
            Rect {
                x: u16::MAX - 100,
                y: u16::MAX - 1000,
                width: 100,
                height: 1000
            }
        );
    }

    #[test]
    fn size_preservation() {
        assert_eq!(
            Rect::new(u16::MAX - 100, u16::MAX - 1000, 100, 1000),
            Rect {
                x: u16::MAX - 100,
                y: u16::MAX - 1000,
                width: 100,
                height: 1000
            }
        );
    }

    #[test]
    fn can_be_const() {
        const RECT: Rect = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };
        const _AREA: u32 = RECT.area();
        const _LEFT: u16 = RECT.left();
        const _RIGHT: u16 = RECT.right();
        const _TOP: u16 = RECT.top();
        const _BOTTOM: u16 = RECT.bottom();
        assert!(RECT.intersects(RECT));
    }

    #[test]
    fn split() {
        let [a, b] = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(Rect::new(0, 0, 2, 1));
        assert_eq!(a, Rect::new(0, 0, 1, 1));
        assert_eq!(b, Rect::new(1, 0, 1, 1));
    }

    #[test]
    #[should_panic(expected = "invalid number of rects")]
    fn split_invalid_number_of_recs() {
        let layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [_a, _b, _c] = layout.areas(Rect::new(0, 0, 2, 1));
    }

    #[rstest]
    #[case::inside(Rect::new(20, 20, 10, 10), Rect::new(20, 20, 10, 10))]
    #[case::up_left(Rect::new(5, 5, 10, 10), Rect::new(10, 10, 10, 10))]
    #[case::up(Rect::new(20, 5, 10, 10), Rect::new(20, 10, 10, 10))]
    #[case::up_right(Rect::new(105, 5, 10, 10), Rect::new(100, 10, 10, 10))]
    #[case::left(Rect::new(5, 20, 10, 10), Rect::new(10, 20, 10, 10))]
    #[case::right(Rect::new(105, 20, 10, 10), Rect::new(100, 20, 10, 10))]
    #[case::down_left(Rect::new(5, 105, 10, 10), Rect::new(10, 100, 10, 10))]
    #[case::down(Rect::new(20, 105, 10, 10), Rect::new(20, 100, 10, 10))]
    #[case::down_right(Rect::new(105, 105, 10, 10), Rect::new(100, 100, 10, 10))]
    #[case::too_wide(Rect::new(5, 20, 200, 10), Rect::new(10, 20, 100, 10))]
    #[case::too_tall(Rect::new(20, 5, 10, 200), Rect::new(20, 10, 10, 100))]
    #[case::too_large(Rect::new(0, 0, 200, 200), Rect::new(10, 10, 100, 100))]
    fn clamp(#[case] rect: Rect, #[case] expected: Rect) {
        let other = Rect::new(10, 10, 100, 100);
        assert_eq!(rect.clamp(other), expected);
    }

    #[test]
    fn rows() {
        let area = Rect::new(0, 0, 3, 2);
        let rows: Vec<Rect> = area.rows().collect();

        let expected_rows: Vec<Rect> = vec![Rect::new(0, 0, 3, 1), Rect::new(0, 1, 3, 1)];

        assert_eq!(rows, expected_rows);
    }

    #[test]
    fn columns() {
        let area = Rect::new(0, 0, 3, 2);
        let columns: Vec<Rect> = area.columns().collect();

        let expected_columns: Vec<Rect> = vec![
            Rect::new(0, 0, 1, 2),
            Rect::new(1, 0, 1, 2),
            Rect::new(2, 0, 1, 2),
        ];

        assert_eq!(columns, expected_columns);
    }

    #[test]
    fn as_position() {
        let rect = Rect::new(1, 2, 3, 4);
        let position = rect.as_position();
        assert_eq!(position.x, 1);
        assert_eq!(position.y, 2);
    }

    #[test]
    fn as_size() {
        assert_eq!(
            Rect::new(1, 2, 3, 4).as_size(),
            Size {
                width: 3,
                height: 4
            }
        );
    }

    #[test]
    fn from_position_and_size() {
        let position = Position { x: 1, y: 2 };
        let size = Size {
            width: 3,
            height: 4,
        };
        assert_eq!(
            Rect::from((position, size)),
            Rect {
                x: 1,
                y: 2,
                width: 3,
                height: 4
            }
        );
    }

    #[test]
    fn centered_horizontally() {
        let rect = Rect::new(0, 0, 5, 5);
        assert_eq!(
            rect.centered_horizontally(Constraint::Length(3)),
            Rect::new(1, 0, 3, 5)
        );
    }

    #[test]
    fn centered_vertically() {
        let rect = Rect::new(0, 0, 5, 5);
        assert_eq!(
            rect.centered_vertically(Constraint::Length(1)),
            Rect::new(0, 2, 5, 1)
        );
    }

    #[test]
    fn centered() {
        let rect = Rect::new(0, 0, 5, 5);
        assert_eq!(
            rect.centered(Constraint::Length(3), Constraint::Length(1)),
            Rect::new(1, 2, 3, 1)
        );
    }

    #[test]
    fn layout() {
        let layout = Layout::horizontal([Constraint::Length(3), Constraint::Min(0)]);

        let [a, b] = Rect::new(0, 0, 10, 10).layout(&layout);
        assert_eq!(a, Rect::new(0, 0, 3, 10));
        assert_eq!(b, Rect::new(3, 0, 7, 10));

        let areas = Rect::new(0, 0, 10, 10).layout::<2>(&layout);
        assert_eq!(areas[0], Rect::new(0, 0, 3, 10));
        assert_eq!(areas[1], Rect::new(3, 0, 7, 10));
    }

    #[test]
    #[should_panic(expected = "invalid number of rects: expected 3, found 1")]
    fn layout_invalid_number_of_rects() {
        let layout = Layout::horizontal([Constraint::Length(1)]);
        let [_, _, _] = Rect::new(0, 0, 10, 10).layout(&layout);
    }

    #[test]
    fn layout_vec() {
        let layout = Layout::horizontal([Constraint::Length(3), Constraint::Min(0)]);

        let areas = Rect::new(0, 0, 10, 10).layout_vec(&layout);
        assert_eq!(areas[0], Rect::new(0, 0, 3, 10));
        assert_eq!(areas[1], Rect::new(3, 0, 7, 10));
    }

    #[test]
    fn try_layout() {
        let layout = Layout::horizontal([Constraint::Length(3), Constraint::Min(0)]);

        let [a, b] = Rect::new(0, 0, 10, 10).try_layout(&layout).unwrap();
        assert_eq!(a, Rect::new(0, 0, 3, 10));
        assert_eq!(b, Rect::new(3, 0, 7, 10));

        let areas = Rect::new(0, 0, 10, 10).try_layout::<2>(&layout).unwrap();
        assert_eq!(areas[0], Rect::new(0, 0, 3, 10));
        assert_eq!(areas[1], Rect::new(3, 0, 7, 10));
    }

    #[test]
    fn try_layout_invalid_number_of_rects() {
        let layout = Layout::horizontal([Constraint::Length(1)]);
        Rect::new(0, 0, 10, 10)
            .try_layout::<3>(&layout)
            .unwrap_err();
    }
}
