#![warn(missing_docs)]
use std::fmt;

use crate::layout::Rect;

/// Position in the terminal
///
/// The position is relative to the top left corner of the terminal window, with the top left corner
/// being (0, 0). The x axis is horizontal increasing to the right, and the y axis is vertical
/// increasing downwards.
///
/// # Examples
///
/// ```
/// use ratatui::layout::{Position, Rect};
///
/// // the following are all equivalent
/// let position = Position { x: 1, y: 2 };
/// let position = Position::new(1, 2);
/// let position = Position::from((1, 2));
/// let position = Position::from(Rect::new(1, 2, 3, 4));
///
/// // position can be converted back into the components when needed
/// let (x, y) = position.into();
/// ```
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
    pub const ORIGIN: Self = Self { x: 0, y: 0 };

    /// Create a new position
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let position = Position::new(1, 2);
        assert_eq!(position.x, 1);
        assert_eq!(position.y, 2);
    }

    #[test]
    fn from_tuple() {
        let position = Position::from((1, 2));
        assert_eq!(position.x, 1);
        assert_eq!(position.y, 2);
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
        assert_eq!(position.x, 1);
        assert_eq!(position.y, 2);
    }

    #[test]
    fn to_string() {
        let position = Position::new(1, 2);
        assert_eq!(position.to_string(), "(1, 2)");
    }
}
