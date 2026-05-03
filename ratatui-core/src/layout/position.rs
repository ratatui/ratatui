#![warn(missing_docs)]
use core::fmt;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::layout::{Offset, Rect};

/// Position in the terminal coordinate system.
///
/// The position is relative to the top left corner of the terminal window, with the top left corner
/// being (0, 0). The x axis is horizontal increasing to the right, and the y axis is vertical
/// increasing downwards.
///
/// `Position` is used throughout the layout system to represent specific points in the terminal.
/// It can be created from coordinates, tuples, or extracted from rectangular areas.
///
/// # Construction
///
/// - [`new`](Self::new) - Create a new position from x and y coordinates
/// - [`default`](Default::default) - Create at origin (0, 0)
///
/// # Conversion
///
/// - [`from((u16, u16))`](Self::from) - Create from `(u16, u16)` tuple
/// - [`from(Rect)`](Self::from) - Create from [`Rect`] (uses top-left corner)
/// - [`into((u16, u16))`] - Convert to `(u16, u16)` tuple
///
/// # Movement
///
/// - [`offset`](Self::offset) - Move by an [`Offset`]
/// - [`Add<Offset>`](core::ops::Add) and [`Sub<Offset>`](core::ops::Sub) - Shift by offsets with
///   clamping
/// - [`AddAssign<Offset>`](core::ops::AddAssign) and [`SubAssign<Offset>`](core::ops::SubAssign) -
///   In-place shifting
///
/// # Examples
///
/// ```
/// use ratatui_core::layout::{Offset, Position, Rect};
///
/// // the following are all equivalent
/// let position = Position { x: 1, y: 2 };
/// let position = Position::new(1, 2);
/// let position = Position::from((1, 2));
/// let position = Position::from(Rect::new(1, 2, 3, 4));
///
/// // position can be converted back into the components when needed
/// let (x, y) = position.into();
///
/// // movement by offsets
/// let position = Position::new(5, 5) + Offset::new(2, -3);
/// assert_eq!(position, Position::new(7, 2));
/// ```
///
/// For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position {
    /// The x coordinate of the position
    ///
    /// The x coordinate is relative to the left edge of the terminal window, with the left edge
    /// being 0.
    pub x: u16,

    /// The y coordinate of the position
    ///
    /// The y coordinate is relative to the top edge of the terminal window, with the top edge
    /// being 0.
    pub y: u16,
}

impl Position {
    /// Position at the origin, the top left edge at 0,0
    pub const ORIGIN: Self = Self::new(0, 0);

    /// Position at the minimum x and y values
    pub const MIN: Self = Self::ORIGIN;

    /// Position at the maximum x and y values
    pub const MAX: Self = Self::new(u16::MAX, u16::MAX);

    /// Create a new position
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// Moves the position by the given offset.
    ///
    /// Positive offsets move right and down, negative offsets move left and up. Values that would
    /// move the position outside the `u16` range are clamped to the nearest edge.
    #[must_use = "method returns the modified value"]
    pub fn offset(self, offset: Offset) -> Self {
        self + offset
    }
}

impl From<(u16, u16)> for Position {
    fn from((x, y): (u16, u16)) -> Self {
        Self { x, y }
    }
}

impl From<Position> for (u16, u16) {
    fn from(position: Position) -> Self {
        (position.x, position.y)
    }
}

impl From<Rect> for Position {
    fn from(rect: Rect) -> Self {
        rect.as_position()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<Offset> for Position {
    type Output = Self;

    /// Moves the position by the given offset.
    ///
    /// Values that would move the position outside the `u16` range are clamped to the nearest
    /// edge.
    fn add(self, offset: Offset) -> Self {
        let max = i32::from(u16::MAX);
        let x = i32::from(self.x).saturating_add(offset.x).clamp(0, max) as u16;
        let y = i32::from(self.y).saturating_add(offset.y).clamp(0, max) as u16;
        Self { x, y }
    }
}

impl Add<Position> for Offset {
    type Output = Position;

    /// Moves the position by the given offset.
    ///
    /// Values that would move the position outside the `u16` range are clamped to the nearest
    /// edge.
    fn add(self, position: Position) -> Position {
        position + self
    }
}

impl Sub<Offset> for Position {
    type Output = Self;

    /// Moves the position by the inverse of the given offset.
    ///
    /// Values that would move the position outside the `u16` range are clamped to the nearest
    /// edge.
    fn sub(self, offset: Offset) -> Self {
        let max = i32::from(u16::MAX);
        let x = i32::from(self.x).saturating_sub(offset.x).clamp(0, max) as u16;
        let y = i32::from(self.y).saturating_sub(offset.y).clamp(0, max) as u16;
        Self { x, y }
    }
}

impl AddAssign<Offset> for Position {
    /// Moves the position in place by the given offset.
    ///
    /// Values that would move the position outside the `u16` range are clamped to the nearest
    /// edge.
    fn add_assign(&mut self, offset: Offset) {
        *self = *self + offset;
    }
}

impl SubAssign<Offset> for Position {
    /// Moves the position in place by the inverse of the given offset.
    ///
    /// Values that would move the position outside the `u16` range are clamped to the nearest
    /// edge.
    fn sub_assign(&mut self, offset: Offset) {
        *self = *self - offset;
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn new() {
        let position = Position::new(1, 2);

        assert_eq!(position, Position { x: 1, y: 2 });
    }

    #[test]
    fn from_tuple() {
        let position = Position::from((1, 2));

        assert_eq!(position, Position { x: 1, y: 2 });
    }

    #[test]
    fn into_tuple() {
        let position = Position::new(1, 2);
        let (x, y) = position.into();
        assert_eq!(x, 1);
        assert_eq!(y, 2);
    }

    #[test]
    fn from_rect() {
        let rect = Rect::new(1, 2, 3, 4);
        let position = Position::from(rect);

        assert_eq!(position, Position { x: 1, y: 2 });
    }

    #[test]
    fn to_string() {
        let position = Position::new(1, 2);
        assert_eq!(position.to_string(), "(1, 2)");
    }

    #[test]
    fn offset_moves_position() {
        let position = Position::new(2, 3).offset(Offset::new(5, 7));

        assert_eq!(position, Position::new(7, 10));
    }

    #[test]
    fn offset_clamps_to_bounds() {
        let position = Position::new(1, 1).offset(Offset::MAX);

        assert_eq!(position, Position::MAX);
    }

    #[test]
    fn add_and_subtract_offset() {
        let position = Position::new(10, 10) + Offset::new(-3, 4) - Offset::new(5, 20);

        assert_eq!(position, Position::new(2, 0));
    }

    #[test]
    fn add_assign_and_sub_assign_offset() {
        let mut position = Position::new(5, 5);
        position += Offset::new(2, 3);
        position -= Offset::new(10, 1);

        assert_eq!(position, Position::new(0, 7));
    }
}
