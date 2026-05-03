use crate::layout::Position;

/// Amounts by which to move a [`Rect`](crate::layout::Rect).
///
/// Positive numbers move to the right/bottom and negative to the left/top.
///
/// See [`Rect::offset`](crate::layout::Rect::offset) for usage.
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
    pub const ZERO: Self = Self::new(0, 0);

    /// The minimum offset
    pub const MIN: Self = Self::new(i32::MIN, i32::MIN);

    /// The maximum offset
    pub const MAX: Self = Self::new(i32::MAX, i32::MAX);

    /// Creates a new `Offset` with the given values.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<Position> for Offset {
    fn from(position: Position) -> Self {
        Self {
            x: i32::from(position.x),
            y: i32::from(position.y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_components() {
        assert_eq!(Offset::new(-3, 7), Offset { x: -3, y: 7 });
    }

    #[test]
    fn constants_match_expected_values() {
        assert_eq!(Offset::ZERO, Offset::new(0, 0));
        assert_eq!(Offset::MIN, Offset::new(i32::MIN, i32::MIN));
        assert_eq!(Offset::MAX, Offset::new(i32::MAX, i32::MAX));
    }

    #[test]
    fn from_position_converts_coordinates() {
        let position = Position::new(4, 9);
        let offset = Offset::from(position);

        assert_eq!(offset, Offset::new(4, 9));
    }
}
