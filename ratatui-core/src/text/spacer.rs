#![warn(clippy::pedantic, clippy::nursery, clippy::arithmetic_side_effects)]
use core::fmt;

use crate::layout::Rect;

/// Represents a horizontal gap in a line of text for alignment or spacing purposes.
///
/// `Spacer` is used within a [`InlineText`] to reserve horizontal space without rendering any
/// visible characters. Unlike a [`Span`], which displays styled text, a `Spacer` affects only the
/// rendering position by shifting the cursor horizontally by a specified width. This is useful when
/// aligning text elements manually or inserting visual gaps between components.
///
/// # Constructor Methods
///
/// - [`Spacer::new`] creates a `Spacer` with the given width.
/// - [`Spacer::default`] creates an empty `Spacer` (i.e. zero width).
///
/// # Conversion Methods
///
/// - [`From<usize>`] creates a `Spacer` from a [`usize`].
///
/// # Setter Methods
///
/// These methods are fluent setters. They return a `Spacer` with the property set.
///
/// - [`Spacer::width`] sets the width of the `Spacer`.
///
/// # Other Methods
///
/// - [`Spacer::apply`] applies the `Spacer` by advancing the `x` position of a [`Rect`] by its
///   width.
///
/// [`Rect`]: crate::layout::Rect
/// [`Span`]: crate::text::Span
/// [`Line`]: crate::text::Line
#[doc(hidden)]
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Spacer {
    /// The width in terminal cells, used to shift the cursor horizontally without rendering any
    /// content.
    pub width: usize,
}

impl fmt::Debug for Spacer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.width == 0 {
            f.write_str("Spacer::default()")
        } else {
            write!(f, "Spacer::from({})", self.width)
        }
    }
}

impl Spacer {
    /// Creates a `Spacer` that occupies the specified number of horizontal cells when rendered.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::text::Spacer;
    ///
    /// let spacer = Spacer::new(4);
    /// assert_eq!(spacer.width, 4);
    /// ```
    #[must_use = "function returns a new value and must be used to avoid discarding it"]
    pub const fn new(width: usize) -> Self {
        Self { width }
    }

    /// Returns a new `Spacer` with the specified width.
    ///
    /// The width determines how many terminal cells the `Spacer` occupies during rendering.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::text::Spacer;
    ///
    /// let spacer = Spacer::default().width(4);
    /// assert_eq!(spacer.width, 4);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Applies the `Spacer` by advancing the horizontal position of the given layout area.
    ///
    /// This method shifts the `x` coordinate of the provided [`Rect`] by the `Spacer`'s width,
    /// using saturating addition to prevent overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::layout::Rect;
    /// use ratatui_core::text::Spacer;
    ///
    /// let mut area = Rect::new(0, 0, 10, 1);
    /// let spacer = Spacer::new(4);
    /// spacer.apply(&mut area);
    /// assert_eq!(area.x, 4);
    /// ```
    ///
    /// [`Rect`]: crate::layout::Rect
    pub fn apply(&self, area: &mut Rect) {
        area.x = area
            .x
            .saturating_add(u16::try_from(self.width).unwrap_or(u16::MAX));
    }
}

impl From<usize> for Spacer {
    fn from(width: usize) -> Self {
        Self { width }
    }
}

/// Adds a number of units to the width of the `Spacer`, returning a new `Spacer`.
impl core::ops::Add<usize> for Spacer {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::new(self.width.saturating_add(rhs))
    }
}

/// Adds a number of units to the width of the `Spacer` in place.
impl core::ops::AddAssign<usize> for Spacer {
    fn add_assign(&mut self, rhs: usize) {
        self.width = self.width.saturating_add(rhs);
    }
}

impl fmt::Display for Spacer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if cfg!(debug_assertions) {
            let width = f.precision().map_or(self.width, |p| self.width.min(p));
            for _ in 0..width {
                f.write_str(".")?;
            }
            return Ok(());
        }
        let width = f.precision().map_or(self.width, |p| self.width.min(p));
        for _ in 0..width {
            f.write_str(" ")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use rstest::rstest;

    use super::*;
    use crate::layout::Rect;

    #[test]
    fn default() {
        let spacer = Spacer::default();
        assert_eq!(spacer.width, 0);
    }

    #[test]
    fn new() {
        let spacer = Spacer::new(4);
        assert_eq!(spacer.width, 4);
    }

    #[test]
    fn width() {
        let spacer = Spacer::default().width(4);
        assert_eq!(spacer.width, 4);
    }

    #[test]
    fn apply() {
        let mut area = Rect::new(0, 0, 10, 1);
        let spacer = Spacer::new(4);
        spacer.apply(&mut area);
        assert_eq!(area.x, 4);
    }

    #[test]
    fn from_usize() {
        let spacer: Spacer = 4usize.into();
        assert_eq!(spacer.width, 4);
    }

    #[test]
    fn add_usize() {
        let spacer = Spacer::new(4);
        let spacer = spacer + 1;
        assert_eq!(spacer.width, 5);
    }

    #[test]
    fn add_assign_usize() {
        let mut spacer = Spacer::new(4);
        spacer += 1;
        assert_eq!(spacer.width, 5);
    }

    #[rstest]
    #[case::default(Spacer::default(), "Spacer::default()")]
    #[case::from(Spacer::from(4), "Spacer::from(4)")]
    fn debug(#[case] spacer: Spacer, #[case] expected: &str) {
        assert_eq!(format!("{spacer:?}"), expected);
    }

    #[rstest]
    #[case::zero(Spacer::default(), "", Some(0))]
    #[case::full(Spacer::new(4), "....", None)]
    #[case::truncated(Spacer::new(4), "...", Some(3))]
    #[cfg(debug_assertions)]
    fn display(#[case] spacer: Spacer, #[case] expected: &str, #[case] precision: Option<usize>) {
        let output = precision.map_or_else(|| format!("{spacer}"), |p| format!("{spacer:.p$}"));
        assert_eq!(output, expected);
    }

    #[rstest]
    #[case::zero(Spacer::default(), "", Some(0))]
    #[case::full(Spacer::new(4), "    ", None)]
    #[case::truncated(Spacer::new(4), "   ", Some(3))]
    #[cfg(not(debug_assertions))]
    fn display(#[case] spacer: Spacer, #[case] expected: &str, #[case] precision: Option<usize>) {
        let output = precision.map_or_else(|| format!("{spacer}"), |p| format!("{spacer:.p$}"));
        assert_eq!(output, expected);
    }
}
