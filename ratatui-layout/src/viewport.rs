//! Generic viewport layout.
//!
//! A [`Viewport`](crate::viewport::Viewport) maps a visible terminal rectangle onto a larger
//! content size. It clamps the caller's desired offset and returns the visible content-space
//! rectangle. It does not render content and does not create scrollbars; callers use the returned
//! [`ViewportLayout`](crate::viewport::ViewportLayout) to drive their own rendering and scrollbar
//! state.
//!
//! Use direct rendering when all content fits. Use this module when a render pass needs clamped
//! content-space coordinates that can also drive app-owned scrollbars or status text.
//! Ratatui widgets or direct buffer rendering remain simpler when there is no larger content-space
//! model to preserve between input events.
//!
//! # Types
//!
//! - [`ViewportState`](crate::viewport::ViewportState) stores the desired content offset between
//!   frames.
//! - [`Viewport`](crate::viewport::Viewport) stores the content size for one layout pass and clamps
//!   the state.
//! - [`ViewportLayout`](crate::viewport::ViewportLayout) is the solved frame-local value: screen
//!   viewport, content size, clamped offset, and visible content-space rectangle.
//!
//! See [`crate::docs::virtualization`] for the broader viewport, list, table, and scroll-metrics
//! model.
//!
//! # Examples
//!
//! ```rust
//! use ratatui_core::layout::{Position, Rect, Size};
//! use ratatui_layout::viewport::{Viewport, ViewportState};
//!
//! let mut state = ViewportState::new(Position::new(50, 10));
//! let layout = Viewport::new(Size::new(100, 40)).layout(Rect::new(0, 0, 20, 5), &mut state);
//!
//! assert_eq!(layout.offset, Position::new(50, 10));
//! assert_eq!(layout.visible_content, Rect::new(50, 10, 20, 5));
//! ```

use ratatui_core::layout::{Position, Rect, Size};

/// Scroll state for a rectangular viewport.
///
/// Use [`ViewportState`](crate::viewport::ViewportState) when user input can request scroll
/// positions that may later become invalid. The user can press right or down before content size is
/// known; [`Viewport::layout`](crate::viewport::Viewport::layout) clamps the stored offset once the
/// current viewport and content size are available.
///
/// # Constructor
///
/// - [`ViewportState::new`](crate::viewport::ViewportState::new) creates state with a desired
///   content offset.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::{Position, Rect, Size};
/// use ratatui_layout::viewport::{Viewport, ViewportState};
///
/// let mut state = ViewportState::new(Position::new(99, 99));
/// Viewport::new(Size::new(20, 10)).layout(Rect::new(0, 0, 5, 5), &mut state);
///
/// assert_eq!(state.offset, Position::new(15, 5));
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewportState {
    /// The desired content offset.
    pub offset: Position,
}

impl ViewportState {
    /// Creates viewport state with the given offset.
    ///
    /// # Examples
    ///
    /// Store a user-requested offset before the next frame knows current content bounds:
    ///
    /// ```rust
    /// use ratatui_core::layout::Position;
    /// use ratatui_layout::viewport::ViewportState;
    ///
    /// let state = ViewportState::new(Position::new(10, 3));
    ///
    /// assert_eq!(state.offset, Position::new(10, 3));
    /// ```
    pub const fn new(offset: Position) -> Self {
        Self { offset }
    }
}

/// A generic scroll viewport layout helper.
///
/// Use [`Viewport`](crate::viewport::Viewport) when a widget has larger logical content than
/// visible screen space but the widget still renders that content itself.
/// [`Viewport`](crate::viewport::Viewport) does not draw text, scrollbars, or clipping buffers; it
/// answers which content-space rectangle should be visible and clamps the
/// [`ViewportState`](crate::viewport::ViewportState).
///
/// # Constructors and solving
///
/// - [`Viewport::new`](crate::viewport::Viewport::new) creates a viewport helper for a full logical
///   content size.
/// - [`Viewport::layout`](crate::viewport::Viewport::layout) clamps
///   [`ViewportState`](crate::viewport::ViewportState) and returns
///   [`ViewportLayout`](crate::viewport::ViewportLayout) for rendering and scroll metrics.
///
/// # Examples
///
/// Use a viewport when input changes a logical content offset before rendering:
///
/// ```rust
/// use ratatui_core::layout::{Position, Rect, Size};
/// use ratatui_layout::scroll::ScrollMetrics;
/// use ratatui_layout::viewport::{Viewport, ViewportState};
///
/// let mut state = ViewportState::new(Position::new(30, 10));
/// let layout = Viewport::new(Size::new(100, 40)).layout(Rect::new(0, 0, 20, 5), &mut state);
/// let horizontal = ScrollMetrics::horizontal(&layout);
///
/// assert_eq!(layout.visible_content, Rect::new(30, 10, 20, 5));
/// assert_eq!(horizontal.offset, 30);
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Viewport {
    content_size: Size,
}

impl Viewport {
    /// Creates a viewport for content of the given size.
    ///
    /// # Examples
    ///
    /// Describe a content canvas larger than the terminal viewport:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Position, Rect, Size};
    /// use ratatui_layout::viewport::{Viewport, ViewportState};
    ///
    /// let mut state = ViewportState::new(Position::new(5, 0));
    /// let layout = Viewport::new(Size::new(100, 20)).layout(Rect::new(0, 0, 20, 5), &mut state);
    ///
    /// assert_eq!(layout.visible_content.x, 5);
    /// ```
    pub const fn new(content_size: Size) -> Self {
        Self { content_size }
    }

    /// Solves the viewport and clamps the state offset.
    ///
    /// If the content is smaller than the viewport on an axis, that axis is clamped to zero. The
    /// returned [`ViewportLayout::visible_content`](crate::viewport::ViewportLayout::visible_content) is expressed in content coordinates, not screen
    /// coordinates.
    ///
    /// # Examples
    ///
    /// Convert a terminal viewport into the content-space rectangle to render:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Position, Rect, Size};
    /// use ratatui_layout::viewport::{Viewport, ViewportState};
    ///
    /// let mut state = ViewportState::new(Position::new(30, 5));
    /// let layout = Viewport::new(Size::new(80, 30)).layout(Rect::new(0, 0, 20, 10), &mut state);
    ///
    /// assert_eq!(layout.viewport, Rect::new(0, 0, 20, 10));
    /// assert_eq!(layout.visible_content, Rect::new(30, 5, 20, 10));
    /// ```
    pub fn layout(self, area: Rect, state: &mut ViewportState) -> ViewportLayout {
        let max_x = self.content_size.width.saturating_sub(area.width);
        let max_y = self.content_size.height.saturating_sub(area.height);
        let offset = Position::new(state.offset.x.min(max_x), state.offset.y.min(max_y));
        state.offset = offset;

        let visible_width = area
            .width
            .min(self.content_size.width.saturating_sub(offset.x));
        let visible_height = area
            .height
            .min(self.content_size.height.saturating_sub(offset.y));
        let visible_content = Rect::new(offset.x, offset.y, visible_width, visible_height);

        ViewportLayout {
            viewport: area,
            content_size: self.content_size,
            offset,
            visible_content,
        }
    }
}

/// Solved viewport metadata.
///
/// Use [`ViewportLayout`](crate::viewport::ViewportLayout) after solving a
/// [`Viewport`](crate::viewport::Viewport) to render only the visible content slice or
/// to report scroll state. For example, a custom canvas can use
/// [`ViewportLayout::visible_content`](crate::viewport::ViewportLayout::visible_content) to decide
/// which world rows to draw while a separate scrollbar uses
/// [`ViewportLayout::content_size`](crate::viewport::ViewportLayout::content_size) and
/// [`ViewportLayout::offset`](crate::viewport::ViewportLayout::offset).
///
/// # Fields
///
/// - [`ViewportLayout::viewport`](crate::viewport::ViewportLayout::viewport) is the terminal-space
///   area being drawn.
/// - [`ViewportLayout::content_size`](crate::viewport::ViewportLayout::content_size) is the full
///   logical content size.
/// - [`ViewportLayout::offset`](crate::viewport::ViewportLayout::offset) is the clamped
///   content-space origin.
/// - [`ViewportLayout::visible_content`](crate::viewport::ViewportLayout::visible_content) is the
///   content-space rectangle visible through the viewport.
///
/// # Examples
///
/// Render code can compare screen-space and content-space rectangles from one value:
///
/// ```rust
/// use ratatui_core::layout::{Position, Rect, Size};
/// use ratatui_layout::viewport::{Viewport, ViewportState};
///
/// let mut state = ViewportState::new(Position::new(5, 2));
/// let layout = Viewport::new(Size::new(30, 10)).layout(Rect::new(1, 1, 10, 4), &mut state);
///
/// assert_eq!(layout.viewport, Rect::new(1, 1, 10, 4));
/// assert_eq!(layout.visible_content, Rect::new(5, 2, 10, 4));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewportLayout {
    /// The visible viewport area.
    pub viewport: Rect,
    /// The full scrollable content size.
    pub content_size: Size,
    /// The clamped content offset.
    pub offset: Position,
    /// The content-space rectangle visible in the viewport.
    pub visible_content: Rect,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use ratatui_core::layout::{Position, Rect, Size};

    use super::*;

    #[test]
    fn clamps_offset_to_content_bounds() {
        let mut state = ViewportState::new(Position::new(99, 99));
        let layout = Viewport::new(Size::new(10, 8)).layout(Rect::new(0, 0, 4, 3), &mut state);

        assert_eq!(state.offset, Position::new(6, 5));
        assert_eq!(layout.visible_content, Rect::new(6, 5, 4, 3));
    }

    #[test]
    fn clamps_offset_to_zero_when_content_fits() {
        let mut state = ViewportState::new(Position::new(1, 1));
        Viewport::new(Size::new(2, 2)).layout(Rect::new(0, 0, 4, 3), &mut state);

        assert_eq!(state.offset, Position::ORIGIN);
    }
}
