use core::fmt;

/// Represents spacing around rectangular areas.
///
/// `Margin` defines the horizontal and vertical spacing that should be applied around a rectangular
/// area. It's commonly used with [`Layout`](crate::layout::Layout) to add space between the
/// layout's boundaries and its contents, or with [`Rect::inner`](crate::layout::Rect::inner) and
/// [`Rect::outer`](crate::layout::Rect::outer) to create padded areas.
///
/// The margin values represent the number of character cells to add on each side. For horizontal
/// margin, the space is applied to both the left and right sides. For vertical margin, the space
/// is applied to both the top and bottom sides.
///
/// # Construction
///
/// - [`new`](Self::new) - Create a new margin with horizontal and vertical spacing
/// - [`default`](Default::default) - Create with zero margin
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::{Constraint, Layout, Margin, Rect};
///
/// // Create a margin of 2 cells horizontally and 1 cell vertically
/// let margin = Margin::new(2, 1);
///
/// // Apply directly to a rectangle
/// let area = Rect::new(0, 0, 80, 24);
/// let inner_area = area.inner(margin);
///
/// // Or use with a layout (which only accepts uniform margins)
/// let layout = Layout::vertical([Constraint::Fill(1)]).margin(2);
/// ```
///
/// For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Margin {
    pub horizontal: u16,
    pub vertical: u16,
}

impl Margin {
    pub const fn new(horizontal: u16, vertical: u16) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}

impl fmt::Display for Margin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.horizontal, self.vertical)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn margin_to_string() {
        assert_eq!(Margin::new(1, 2).to_string(), "1x2");
    }

    #[test]
    fn margin_new() {
        assert_eq!(
            Margin::new(1, 2),
            Margin {
                horizontal: 1,
                vertical: 2
            }
        );
    }
}
