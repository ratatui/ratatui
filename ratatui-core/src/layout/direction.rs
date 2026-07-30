use strum::{Display, EnumString};

/// Defines the direction of a layout.
///
/// This enumeration is used with [`Layout`](crate::layout::Layout) to specify whether layout
/// segments should be arranged horizontally or vertically.
///
/// - `Horizontal`: Layout segments are arranged side by side (left to right)
/// - `Vertical`: Layout segments are arranged top to bottom (default)
///
/// For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction {
    /// Layout segments are arranged side by side (left to right).
    Horizontal,
    /// Layout segments are arranged top to bottom (default).
    #[default]
    Vertical,
}

impl Direction {
    /// The perpendicular direction to this direction.
    ///
    /// `Horizontal` returns `Vertical`, and `Vertical` returns `Horizontal`.
    #[inline]
    #[must_use = "returns the perpendicular direction"]
    pub const fn perpendicular(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use strum::ParseError;

    use super::*;

    #[test]
    fn direction_to_string() {
        assert_eq!(Direction::Horizontal.to_string(), "Horizontal");
        assert_eq!(Direction::Vertical.to_string(), "Vertical");
    }

    #[test]
    fn direction_from_str() {
        assert_eq!("Horizontal".parse::<Direction>(), Ok(Direction::Horizontal));
        assert_eq!("Vertical".parse::<Direction>(), Ok(Direction::Vertical));
        assert_eq!("".parse::<Direction>(), Err(ParseError::VariantNotFound));
    }

    #[test]
    fn other() {
        use Direction::*;
        assert_eq!(Horizontal.perpendicular(), Vertical);
        assert_eq!(Vertical.perpendicular(), Horizontal);
    }
}
