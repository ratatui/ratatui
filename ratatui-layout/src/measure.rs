//! Measurement contracts for external layout participants.
//!
//! Measurement is the phase where externally owned content describes how much space it can use
//! before a container assigns final rectangles. The types in this module intentionally stay small:
//! terminal UIs usually need a width, a height, and enough bounds to explain whether a value is
//! fixed, preferred, or flexible.
//!
//! The most common terminal layout question is "how tall is this item at this width?" A virtual
//! list asks that question for every item it may need to show. More general participant-based
//! layouts use [`MeasureConstraint`](crate::measure::MeasureConstraint) and
//! [`SizeHint`](crate::measure::SizeHint) to keep that conversation explicit.
//!
//! # Types
//!
//! - [`MeasureConstraint`](crate::measure::MeasureConstraint) describes which dimensions the parent
//!   already knows before asking a child to measure itself.
//! - [`SizeHint`](crate::measure::SizeHint) reports minimum, preferred, and maximum useful sizes
//!   for externally owned content.
//!
//! See [`crate::docs::widget_contracts`] for how measurement fits into the future participant and
//! component direction. Use ordinary fixed `Rect` rendering when child content does not need to
//! negotiate size with a parent.
//!
//! # Examples
//!
//! Return a height hint from externally owned content before a parent assigns the final area:
//!
//! ```rust
//! use ratatui_core::layout::Size;
//! use ratatui_layout::measure::{MeasureConstraint, SizeHint};
//!
//! fn measure_label(text: &str, constraint: MeasureConstraint) -> SizeHint {
//!     match constraint {
//!         MeasureConstraint::Width(width) => {
//!             let height = text.len().div_ceil(width.max(1) as usize).max(1) as u16;
//!             SizeHint::exact(Size::new(width, height))
//!         }
//!         _ => SizeHint::preferred(Size::new(text.len() as u16, 1)),
//!     }
//! }
//!
//! assert_eq!(
//!     measure_label("abcdef", MeasureConstraint::Width(3)).preferred,
//!     Size::new(3, 2),
//! );
//! ```

use ratatui_core::layout::Size;

/// Constraints supplied when asking external content to measure itself.
///
/// Use [`MeasureConstraint`](crate::measure::MeasureConstraint) when a parent needs to ask a
/// child-sized concept a specific sizing question. A virtual list asks "how tall are you at this
/// width?" A grid-like parent might ask for a preferred size under an exact cell size. The
/// constraint records which dimensions are already known before the child returns a
/// [`SizeHint`](crate::measure::SizeHint).
///
/// A constraint does not force the child to return a particular value by itself; the child still
/// returns a hint that describes its useful range.
///
/// # Variants
///
/// - [`MeasureConstraint::Unbounded`](crate::measure::MeasureConstraint::Unbounded) asks for the
///   child's natural size.
/// - [`MeasureConstraint::Width`](crate::measure::MeasureConstraint::Width) fixes width so the
///   child can compute height, common for wrapped text or list rows.
/// - [`MeasureConstraint::Height`](crate::measure::MeasureConstraint::Height) fixes height so the
///   child can compute width.
/// - [`MeasureConstraint::Exact`](crate::measure::MeasureConstraint::Exact) fixes both dimensions
///   when a parent has already solved the cell.
///
/// # Examples
///
/// Match on the known dimension to answer the parent's sizing question:
///
/// ```rust
/// use ratatui_core::layout::Size;
/// use ratatui_layout::measure::{MeasureConstraint, SizeHint};
///
/// fn measure_wrapped(text: &str, constraint: MeasureConstraint) -> SizeHint {
///     match constraint {
///         MeasureConstraint::Width(width) => {
///             let height = text.len().div_ceil(width.max(1) as usize).max(1) as u16;
///             SizeHint::exact(Size::new(width, height))
///         }
///         MeasureConstraint::Exact(size) => SizeHint::exact(size),
///         _ => SizeHint::preferred(Size::new(text.len() as u16, 1)),
///     }
/// }
///
/// assert_eq!(
///     measure_wrapped("abcdefgh", MeasureConstraint::Width(4)).preferred,
///     Size::new(4, 2),
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MeasureConstraint {
    /// No explicit width or height is known.
    #[default]
    Unbounded,
    /// Width is fixed; height should be derived from that width.
    Width(u16),
    /// Height is fixed; width should be derived from that height.
    Height(u16),
    /// Both dimensions are fixed.
    Exact(Size),
}

/// Minimum, preferred, and maximum size information for external content.
///
/// Use [`SizeHint`](crate::measure::SizeHint) when external content has a natural size but the
/// parent still owns final layout. A toolbar button might have a preferred label width, a minimum
/// icon width, and a maximum useful width. The parent can compare those hints before assigning
/// concrete regions.
///
/// [`SizeHint`](crate::measure::SizeHint) is intentionally a hint, not a solved rectangle. The
/// final assigned area is still communicated later through a [`crate::regions::Region`] or render
/// callback.
///
/// # Constructors
///
/// - [`SizeHint::exact`](crate::measure::SizeHint::exact) creates a fixed-size hint where min,
///   preferred, and max match.
/// - [`SizeHint::preferred`](crate::measure::SizeHint::preferred) creates flexible bounds around a
///   natural size.
/// - [`SizeHint::bounded`](crate::measure::SizeHint::bounded) creates a fully specified range.
///
/// # Inspection
///
/// - [`SizeHint::clamped_preferred`](crate::measure::SizeHint::clamped_preferred) returns the
///   preferred size after applying the advertised minimum and maximum bounds.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Size;
/// use ratatui_layout::measure::SizeHint;
///
/// let fixed = SizeHint::exact(Size::new(8, 1));
/// assert_eq!(fixed.min, fixed.max);
///
/// let flexible = SizeHint::bounded(Size::new(4, 1), Size::new(12, 1), Size::new(20, 1));
/// assert_eq!(flexible.clamped_preferred(), Size::new(12, 1));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SizeHint {
    /// The smallest useful size.
    pub min: Size,
    /// The preferred size when enough space is available.
    pub preferred: Size,
    /// The largest useful size.
    pub max: Size,
}

impl SizeHint {
    /// Creates a size hint with the same min, preferred, and max size.
    ///
    /// Use this when an item must occupy one exact terminal-cell size, such as an icon, fixed
    /// badge, or already-solved cell.
    ///
    /// # Examples
    ///
    /// Return a fixed one-line height for a command palette row:
    ///
    /// ```rust
    /// use ratatui_core::layout::Size;
    /// use ratatui_layout::measure::SizeHint;
    ///
    /// let row = SizeHint::exact(Size::new(20, 1));
    ///
    /// assert_eq!(row.min, Size::new(20, 1));
    /// assert_eq!(row.max, Size::new(20, 1));
    /// ```
    pub const fn exact(size: Size) -> Self {
        Self {
            min: size,
            preferred: size,
            max: size,
        }
    }

    /// Creates an unconstrained hint with a preferred size.
    ///
    /// The minimum is zero and the maximum is [`Size::MAX`]. This is useful for content that can
    /// shrink or grow but has a natural size.
    ///
    /// # Examples
    ///
    /// Describe a label that prefers its text width but can be clipped by the parent:
    ///
    /// ```rust
    /// use ratatui_core::layout::Size;
    /// use ratatui_layout::measure::SizeHint;
    ///
    /// let label = SizeHint::preferred(Size::new(12, 1));
    ///
    /// assert_eq!(label.min, Size::ZERO);
    /// assert_eq!(label.preferred, Size::new(12, 1));
    /// ```
    pub const fn preferred(size: Size) -> Self {
        Self {
            min: Size::ZERO,
            preferred: size,
            max: Size::MAX,
        }
    }

    /// Creates a fully specified size hint.
    ///
    /// This constructor does not reorder the bounds. Callers should pass coherent values where
    /// `min <= preferred <= max` for each dimension, or use
    /// [`SizeHint::clamped_preferred`](crate::measure::SizeHint::clamped_preferred) before
    /// relying on the preferred size.
    ///
    /// # Examples
    ///
    /// Give a resizable panel a useful range while keeping its natural size explicit:
    ///
    /// ```rust
    /// use ratatui_core::layout::Size;
    /// use ratatui_layout::measure::SizeHint;
    ///
    /// let panel = SizeHint::bounded(Size::new(20, 5), Size::new(40, 10), Size::new(80, 20));
    ///
    /// assert_eq!(panel.preferred, Size::new(40, 10));
    /// ```
    pub const fn bounded(min: Size, preferred: Size, max: Size) -> Self {
        Self {
            min,
            preferred,
            max,
        }
    }

    /// Returns the preferred size clamped to the min and max bounds.
    ///
    /// This is a convenience for simple parents that want to honor the preferred size while still
    /// respecting the advertised bounds.
    ///
    /// # Examples
    ///
    /// Clamp an inconsistent or user-provided preferred size before assigning a region:
    ///
    /// ```rust
    /// use ratatui_core::layout::Size;
    /// use ratatui_layout::measure::SizeHint;
    ///
    /// let hint = SizeHint::bounded(Size::new(10, 1), Size::new(5, 4), Size::new(20, 3));
    ///
    /// assert_eq!(hint.clamped_preferred(), Size::new(10, 3));
    /// ```
    pub fn clamped_preferred(self) -> Size {
        Size::new(
            self.preferred.width.clamp(self.min.width, self.max.width),
            self.preferred
                .height
                .clamp(self.min.height, self.max.height),
        )
    }
}

impl Default for SizeHint {
    fn default() -> Self {
        Self::preferred(Size::ZERO)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use ratatui_core::layout::Size;

    use super::*;

    #[test]
    fn exact_sets_all_bounds() {
        let hint = SizeHint::exact(Size::new(3, 4));

        assert_eq!(hint.min, Size::new(3, 4));
        assert_eq!(hint.preferred, Size::new(3, 4));
        assert_eq!(hint.max, Size::new(3, 4));
    }

    #[test]
    fn clamps_preferred() {
        let hint = SizeHint::bounded(Size::new(2, 3), Size::new(1, 10), Size::new(5, 6));

        assert_eq!(hint.clamped_preferred(), Size::new(2, 6));
    }
}
