//! Scroll metrics for app-owned scrollbars and status displays.
//!
//! Layout primitives such as [`crate::viewport::Viewport`] and [`crate::list::VirtualList`] expose
//! content length and offset, but applications often need the same derived scrollbar values.
//! [`ScrollMetrics`](crate::scroll::ScrollMetrics) keeps that math in one inspectable value without
//! making this crate render a scrollbar widget.
//!
//! Use a fixed status string or no scrollbar at all when the content always fits. Use this module
//! when the app renders its own scrollbar, minimap, or scroll status from frame-local viewport
//! data.
//!
//! # Type
//!
//! - [`ScrollMetrics`](crate::scroll::ScrollMetrics) stores one-axis content length, viewport
//!   length, clamped offset, maximum offset, and scrollbar thumb geometry.
//!
//! See [`crate::docs::virtualization`] for how scroll metrics fit with viewports, virtual lists,
//! and virtual tables.
//!
//! # Examples
//!
//! Compute scrollbar geometry from a virtualized layout and let the app decide how to render it:
//!
//! ```rust
//! use ratatui_core::buffer::Buffer;
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::list::{ListItemContext, ListItems, VirtualList, VirtualListState};
//! use ratatui_layout::participant::MeasureContext;
//! use ratatui_layout::scroll::ScrollMetrics;
//!
//! struct Rows;
//! impl ListItems for Rows {
//!     fn len(&self) -> usize {
//!         20
//!     }
//!     fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
//!         1
//!     }
//!     fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
//! }
//!
//! let mut state = VirtualListState::default();
//! state.set_scroll(ratatui_layout::list::ScrollPosition::new(5, 0));
//! let layout = VirtualList::new().layout(Rect::new(0, 0, 10, 5), &mut state, &Rows);
//! let metrics = ScrollMetrics::from_list(&layout);
//!
//! assert_eq!(metrics.offset, 5);
//! assert!(metrics.thumb_length > 0);
//! ```

use ratatui_core::layout::Size;

use crate::list::ListLayout;
use crate::viewport::ViewportLayout;

/// Derived scrollbar geometry for one scroll axis.
///
/// Use [`ScrollMetrics`](crate::scroll::ScrollMetrics) when an app-owned scrollbar, minimap, or
/// status line needs to describe a scroll position. It stores the full logical content length,
/// visible viewport length, current offset, and the thumb range within the viewport.
///
/// # Constructors
///
/// - [`ScrollMetrics::new`](crate::scroll::ScrollMetrics::new) computes metrics directly for one
///   scroll axis.
/// - [`ScrollMetrics::from_list`](crate::scroll::ScrollMetrics::from_list) derives vertical metrics
///   from a [`ListLayout`].
/// - [`ScrollMetrics::horizontal`](crate::scroll::ScrollMetrics::horizontal) and
///   [`ScrollMetrics::vertical`](crate::scroll::ScrollMetrics::vertical) derive metrics from a
///   [`ViewportLayout`](crate::viewport::ViewportLayout).
///
/// # Inspection
///
/// - [`ScrollMetrics::fits`](crate::scroll::ScrollMetrics::fits) reports whether the content fits
///   without scrolling.
/// - [`ScrollMetrics::visible_range`](crate::scroll::ScrollMetrics::visible_range) returns the
///   clamped content indexes visible in a simple fixed-height pane.
///
/// # Examples
///
/// ```rust
/// use ratatui_layout::scroll::ScrollMetrics;
///
/// let metrics = ScrollMetrics::new(100, 10, 30);
///
/// assert_eq!(metrics.offset, 30);
/// assert_eq!(metrics.max_offset, 90);
/// assert!(metrics.thumb_length > 0);
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScrollMetrics {
    /// Full scrollable content length on this axis.
    pub content_length: u32,

    /// Visible viewport length on this axis.
    pub viewport_length: u16,

    /// Clamped content offset on this axis.
    pub offset: u32,

    /// Maximum valid offset on this axis.
    pub max_offset: u32,

    /// Thumb start within the viewport axis.
    pub thumb_start: u16,

    /// Thumb length within the viewport axis.
    pub thumb_length: u16,
}

impl ScrollMetrics {
    /// Creates metrics for one scroll axis.
    ///
    /// # Examples
    ///
    /// Compute the scrollbar thumb for a list with more rows than the viewport can show:
    ///
    /// ```rust
    /// use ratatui_layout::scroll::ScrollMetrics;
    ///
    /// let metrics = ScrollMetrics::new(50, 10, 15);
    ///
    /// assert_eq!(metrics.offset, 15);
    /// assert_eq!(metrics.max_offset, 40);
    /// assert_eq!(metrics.thumb_length, 2);
    /// ```
    pub fn new(content_length: u32, viewport_length: u16, offset: u32) -> Self {
        let viewport = u32::from(viewport_length);
        let max_offset = content_length.saturating_sub(viewport);
        let offset = offset.min(max_offset);
        let thumb_length = thumb_length(content_length, viewport_length);
        let thumb_start = thumb_start(offset, max_offset, viewport_length, thumb_length);

        Self {
            content_length,
            viewport_length,
            offset,
            max_offset,
            thumb_start,
            thumb_length,
        }
    }

    /// Returns true when the content fits without scrolling.
    ///
    /// # Examples
    ///
    /// Hide scrollbar chrome when all content is visible:
    ///
    /// ```rust
    /// use ratatui_layout::scroll::ScrollMetrics;
    ///
    /// let metrics = ScrollMetrics::new(5, 10, 0);
    ///
    /// assert!(metrics.fits());
    /// ```
    pub const fn fits(self) -> bool {
        self.max_offset == 0
    }

    /// Returns the visible content range for fixed-height line or row panes.
    ///
    /// Use this when the content is already a slice of equally sized rows and the app only needs to
    /// render the rows visible in the current viewport. Virtualized lists and tables expose richer
    /// visible item metadata, but simple log panes and status panes often only need this range.
    ///
    /// # Examples
    ///
    /// Render the lines visible in a simple scroll pane:
    ///
    /// ```rust
    /// use ratatui_layout::scroll::ScrollMetrics;
    ///
    /// let rows = ["zero", "one", "two", "three", "four"];
    /// let metrics = ScrollMetrics::new(rows.len() as u32, 2, 3);
    ///
    /// assert_eq!(&rows[metrics.visible_range()], &["three", "four"]);
    /// ```
    pub fn visible_range(self) -> core::ops::Range<usize> {
        let start = self.offset as usize;
        let end = self
            .offset
            .saturating_add(u32::from(self.viewport_length))
            .min(self.content_length) as usize;

        start..end
    }

    /// Builds vertical metrics from a virtual list layout.
    ///
    /// Use this when [`crate::list::VirtualList`] already computed content height and scroll
    /// offset, and the app wants a scrollbar or status indicator beside the list.
    ///
    /// # Examples
    ///
    /// Derive scrollbar values from the same layout used for list hit testing:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::list::{
    ///     ListItemContext, ListItems, ScrollPosition, VirtualList, VirtualListState,
    /// };
    /// use ratatui_layout::participant::MeasureContext;
    /// use ratatui_layout::scroll::ScrollMetrics;
    ///
    /// struct Rows;
    /// impl ListItems for Rows {
    ///     fn len(&self) -> usize {
    ///         10
    ///     }
    ///     fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
    ///         1
    ///     }
    ///     fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
    /// }
    ///
    /// let mut state = VirtualListState::default();
    /// state.set_scroll(ScrollPosition::new(3, 0));
    /// let layout = VirtualList::new().layout(Rect::new(0, 0, 10, 4), &mut state, &Rows);
    /// let metrics = ScrollMetrics::from_list(&layout);
    ///
    /// assert_eq!(metrics.viewport_length, 4);
    /// assert_eq!(metrics.offset, 3);
    /// ```
    pub fn from_list(layout: &ListLayout) -> Self {
        Self::new(
            layout.content_height,
            layout.viewport.height,
            layout.scroll_offset,
        )
    }

    /// Builds horizontal metrics from a rectangular viewport layout.
    ///
    /// # Examples
    ///
    /// Derive horizontal scrollbar values from a two-axis viewport:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Position, Rect, Size};
    /// use ratatui_layout::scroll::ScrollMetrics;
    /// use ratatui_layout::viewport::{Viewport, ViewportState};
    ///
    /// let mut state = ViewportState::new(Position::new(20, 0));
    /// let layout = Viewport::new(Size::new(100, 10)).layout(Rect::new(0, 0, 20, 5), &mut state);
    /// let metrics = ScrollMetrics::horizontal(&layout);
    ///
    /// assert_eq!(metrics.offset, 20);
    /// ```
    pub fn horizontal(viewport: &ViewportLayout) -> Self {
        let viewport_size = Size::new(viewport.viewport.width, viewport.viewport.height);
        Self::from_viewport(
            viewport.content_size,
            viewport_size,
            viewport.offset.x,
            Axis::X,
        )
    }

    /// Builds vertical metrics from a rectangular viewport layout.
    ///
    /// # Examples
    ///
    /// Derive vertical scrollbar values from a viewport layout:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Position, Rect, Size};
    /// use ratatui_layout::scroll::ScrollMetrics;
    /// use ratatui_layout::viewport::{Viewport, ViewportState};
    ///
    /// let mut state = ViewportState::new(Position::new(0, 30));
    /// let layout = Viewport::new(Size::new(20, 100)).layout(Rect::new(0, 0, 20, 10), &mut state);
    /// let metrics = ScrollMetrics::vertical(&layout);
    ///
    /// assert_eq!(metrics.offset, 30);
    /// ```
    pub fn vertical(viewport: &ViewportLayout) -> Self {
        let viewport_size = Size::new(viewport.viewport.width, viewport.viewport.height);
        Self::from_viewport(
            viewport.content_size,
            viewport_size,
            viewport.offset.y,
            Axis::Y,
        )
    }

    fn from_viewport(content: Size, viewport: Size, offset: u16, axis: Axis) -> Self {
        match axis {
            Axis::X => Self::new(u32::from(content.width), viewport.width, u32::from(offset)),
            Axis::Y => Self::new(
                u32::from(content.height),
                viewport.height,
                u32::from(offset),
            ),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
}

fn thumb_length(content_length: u32, viewport_length: u16) -> u16 {
    if viewport_length == 0 || content_length == 0 {
        return 0;
    }

    let viewport = u32::from(viewport_length);
    if content_length <= viewport {
        return viewport_length;
    }

    ((viewport * viewport) / content_length)
        .max(1)
        .min(viewport) as u16
}

fn thumb_start(offset: u32, max_offset: u32, viewport_length: u16, thumb_length: u16) -> u16 {
    if max_offset == 0 || viewport_length <= thumb_length {
        return 0;
    }

    let track = u32::from(viewport_length - thumb_length);
    ((offset * track) / max_offset).min(track) as u16
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn content_that_fits_uses_full_thumb() {
        let metrics = ScrollMetrics::new(5, 10, 0);

        assert!(metrics.fits());
        assert_eq!(metrics.thumb_start, 0);
        assert_eq!(metrics.thumb_length, 10);
    }

    #[test]
    fn oversized_content_computes_thumb_range() {
        let metrics = ScrollMetrics::new(100, 10, 45);

        assert_eq!(metrics.max_offset, 90);
        assert_eq!(metrics.thumb_length, 1);
        assert_eq!(metrics.thumb_start, 4);
    }

    #[test]
    fn visible_range_uses_clamped_offset_and_content_end() {
        let metrics = ScrollMetrics::new(5, 2, 99);

        assert_eq!(metrics.offset, 3);
        assert_eq!(metrics.visible_range(), 3..5);
    }
}
