#![warn(missing_docs)]
use std::{
    cmp::{max, min},
    fmt,
};

use super::{Position, Size};
use crate::prelude::*;

mod iter;
pub use iter::*;

/// A Rectangular area.
///
/// A simple rectangle used in the computation of the layout and to give widgets a hint about the
/// area they are supposed to render to.
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

/// Amounts by which to move a [`Rect`](super::Rect).
///
/// Positive numbers move to the right/bottom and negative to the left/top.
///
/// See [`Rect::offset`]
#[derive(Debug, Default, Clone, Copy)]
pub struct Offset {
    /// How much to move on the X axis
    pub x: i32,
    /// How much to move on the Y axis
    pub y: i32,
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

    /// Creates a new `Rect`, with width and height limited to keep the area under max `u16`. If
    /// clipped, aspect ratio will be preserved.
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        let max_area = u16::MAX;
        let (clipped_width, clipped_height) =
            if u32::from(width) * u32::from(height) > u32::from(max_area) {
                let aspect_ratio = f64::from(width) / f64::from(height);
                let max_area_f = f64::from(max_area);
                let height_f = (max_area_f / aspect_ratio).sqrt();
                let width_f = height_f * aspect_ratio;
                (width_f as u16, height_f as u16)
            } else {
                (width, height)
            };
        Self {
            x,
            y,
            width: clipped_width,
            height: clipped_height,
        }
    }

    /// The area of the `Rect`. If the area is larger than the maximum value of `u16`, it will be
    /// clamped to `u16::MAX`.
    pub const fn area(self) -> u16 {
        self.width.saturating_mul(self.height)
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
    /// # use ratatui::{prelude::*, layout::Position};
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
    /// # use ratatui::prelude::*;
    /// # fn render(frame: &mut Frame) {
    /// let area = frame.size();
    /// let rect = Rect::new(0, 0, 100, 100).clamp(area);
    /// # }
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
    /// # Example
    ///
    /// ```
    /// # use ratatui::prelude::*;
    /// fn render(area: Rect, buf: &mut Buffer) {
    ///     for row in area.rows() {
    ///         Line::raw("Hello, world!").render(row, buf);
    ///     }
    /// }
    /// ```
    pub const fn rows(self) -> Rows {
        Rows::new(self)
    }

    /// An iterator over columns within the `Rect`.
    ///
    /// # Example
    ///
    /// ```
    /// # use ratatui::{prelude::*, widgets::*};
    /// fn render(area: Rect, buf: &mut Buffer) {
    ///     if let Some(left) = area.columns().next() {
    ///         Block::new().borders(Borders::LEFT).render(left, buf);
    ///     }
    /// }
    /// ```
    pub const fn columns(self) -> Columns {
        Columns::new(self)
    }

    /// An iterator over the positions within the `Rect`.
    ///
    /// The positions are returned in a row-major order (left-to-right, top-to-bottom).
    ///
    /// # Example
    ///
    /// ```
    /// # use ratatui::prelude::*;
    /// fn render(area: Rect, buf: &mut Buffer) {
    ///     for position in area.positions() {
    ///         buf.get_mut(position.x, position.y).set_symbol("x");
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
    /// # use ratatui::prelude::*;
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
    use rstest::rstest;

    use super::*;

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
        for width in 256u16..300u16 {
            for height in 256u16..300u16 {
                let rect = Rect::new(0, 0, width, height);
                rect.area(); // Should not panic.
                assert!(rect.width < width || rect.height < height);
                // The target dimensions are rounded down so the math will not be too precise
                // but let's make sure the ratios don't diverge crazily.
                assert!(
                    (f64::from(rect.width) / f64::from(rect.height)
                        - f64::from(width) / f64::from(height))
                    .abs()
                        < 1.0
                );
            }
        }

        // One dimension below 255, one above. Area above max u16.
        let width = 900;
        let height = 100;
        let rect = Rect::new(0, 0, width, height);
        assert_ne!(rect.width, 900);
        assert_ne!(rect.height, 100);
        assert!(rect.width < width || rect.height < height);
    }

    #[test]
    fn size_preservation() {
        for width in 0..256u16 {
            for height in 0..256u16 {
                let rect = Rect::new(0, 0, width, height);
                rect.area(); // Should not panic.
                assert_eq!(rect.width, width);
                assert_eq!(rect.height, height);
            }
        }

        // One dimension below 255, one above. Area below max u16.
        let rect = Rect::new(0, 0, 300, 100);
        assert_eq!(rect.width, 300);
        assert_eq!(rect.height, 100);
    }

    #[test]
    fn can_be_const() {
        const RECT: Rect = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };
        const _AREA: u16 = RECT.area();
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
}
