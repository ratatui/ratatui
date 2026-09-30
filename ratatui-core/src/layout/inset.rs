#![warn(missing_docs)]
use core::fmt;

use crate::layout::Margin;

/// Represents side-specific spacing inside rectangular areas.
///
/// `Inset` defines how much space to remove from each side of a rectangle. Unlike [`Margin`], which
/// applies uniform spacing horizontally and vertically, `Inset` lets you specify independent
/// amounts for the top, right, bottom, and left edges. The default constructor order is
/// top-right-bottom-left (often remembered as “trbl” or “trouble”), matching the CSS spec and
/// offering an easy clockwise mnemonic.
///
/// Use `Inset` when you need per-side control; choose [`Margin`](crate::layout::Margin) for the
/// common symmetric case.
///
/// # Construction
///
/// - [`trbl`](Self::trbl) - Create a new inset with top/right/bottom/left values
/// - [`default`](Default::default) - Create with zero inset on all sides
/// - [`symmetric`](Self::symmetric) - Create with shared horizontal and vertical values
/// - [`horizontal`](Self::horizontal) - Create with equal left and right values
/// - [`vertical`](Self::vertical) - Create with equal top and bottom values
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::{Inset, Rect};
///
/// let inset = Inset::trbl(1, 2, 3, 4);
/// let rect = Rect::new(0, 0, 10, 10).inner(inset);
/// assert_eq!(rect, Rect::new(4, 1, 4, 6));
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Inset {
    /// Space to remove from the top edge
    pub top: u16,
    /// Space to remove from the right edge
    pub right: u16,
    /// Space to remove from the bottom edge
    pub bottom: u16,
    /// Space to remove from the left edge
    pub left: u16,
}

impl Inset {
    /// Creates a new inset with explicit top/right/bottom/left values.
    pub const fn trbl(top: u16, right: u16, bottom: u16, left: u16) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Creates a new inset with shared horizontal and vertical values.
    ///
    /// The `horizontal` value is applied to `left` and `right`; the `vertical` value is applied to
    /// `top` and `bottom`. Note the order is `horizontal, vertical` (x then y), opposite of the CSS
    /// ordering; we keep a single ordering across helpers to avoid mixing patterns in code.
    pub const fn symmetric(horizontal: u16, vertical: u16) -> Self {
        Self {
            right: horizontal,
            left: horizontal,
            top: vertical,
            bottom: vertical,
        }
    }

    /// Creates a new inset with equal left and right values.
    pub const fn horizontal(horizontal: u16) -> Self {
        Self {
            top: 0,
            right: horizontal,
            bottom: 0,
            left: horizontal,
        }
    }

    /// Creates a new inset with equal top and bottom values.
    pub const fn vertical(vertical: u16) -> Self {
        Self {
            top: vertical,
            right: 0,
            bottom: vertical,
            left: 0,
        }
    }
}

impl fmt::Display for Inset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "t{} r{} b{} l{}",
            self.top, self.right, self.bottom, self.left
        )
    }
}

impl From<Margin> for Inset {
    fn from(margin: Margin) -> Self {
        Self {
            top: margin.vertical,
            right: margin.horizontal,
            bottom: margin.vertical,
            left: margin.horizontal,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn new() {
        assert_eq!(
            Inset::trbl(1, 2, 3, 4),
            Inset {
                top: 1,
                right: 2,
                bottom: 3,
                left: 4
            }
        );
    }

    #[test]
    fn display() {
        assert_eq!(Inset::trbl(1, 2, 3, 4).to_string(), "t1 r2 b3 l4");
    }

    #[test]
    fn symmetric() {
        assert_eq!(
            Inset::symmetric(2, 3),
            Inset {
                top: 3,
                right: 2,
                bottom: 3,
                left: 2
            }
        );
    }

    #[test]
    fn horizontal() {
        assert_eq!(
            Inset::horizontal(2),
            Inset {
                top: 0,
                right: 2,
                bottom: 0,
                left: 2
            }
        );
    }

    #[test]
    fn vertical() {
        assert_eq!(
            Inset::vertical(3),
            Inset {
                top: 3,
                right: 0,
                bottom: 3,
                left: 0
            }
        );
    }

    #[test]
    fn from_margin() {
        assert_eq!(
            Inset::from(Margin::new(2, 3)),
            Inset {
                top: 3,
                right: 2,
                bottom: 3,
                left: 2
            }
        );
    }
}
