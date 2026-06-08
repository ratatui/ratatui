//! Scrollable pane coordination.
//!
//! A pane often needs to scroll even when the pointer is over blank space below the last visible
//! row. [`ScrollablePane`] solves that small but common problem: it computes scroll metrics for a
//! viewport and creates a whole-pane pointer target for wheel routing. Rendering remains app-owned.
//!
//! # Types
//!
//! - [`ScrollablePane`] stores the pane id and optional status-line reservation.
//! - [`ScrollablePaneLayout`] exposes the content area, optional status area, metrics, and frame.
//!
//! # Examples
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::ScrollablePane;
//!
//! let layout =
//!     ScrollablePane::new("log")
//!         .status_line(true)
//!         .layout(Rect::new(0, 0, 20, 5), 30, 10);
//!
//! assert_eq!(layout.content_area().height, 4);
//! assert_eq!(layout.metrics().visible_range(), 10..14);
//! assert_eq!(layout.frame().route_scroll((2, 4)).unwrap().id, "log");
//! ```

use ratatui_core::layout::Rect;

use crate::frame::FrameSnapshot;
use crate::pointer::PointerTargets;
use crate::scroll::ScrollMetrics;

/// Pointer and metrics policy for one scrollable pane.
///
/// [`ScrollablePane`] does not know what content is drawn. It only records which id should receive
/// wheel events across the pane and how a logical content length maps into the visible viewport.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ScrollablePane<Id = usize> {
    id: Id,
    status_line: bool,
    z: u16,
}

impl<Id> ScrollablePane<Id> {
    /// Creates a pane for an app-owned id.
    pub const fn new(id: Id) -> Self {
        Self {
            id,
            status_line: false,
            z: 0,
        }
    }

    /// Reserves the last row for scroll status text.
    ///
    /// The content viewport uses the remaining rows while wheel routing still covers the full pane.
    #[must_use = "method returns the modified pane"]
    pub const fn status_line(mut self, status_line: bool) -> Self {
        self.status_line = status_line;
        self
    }

    /// Sets the z-order for the whole-pane pointer target.
    #[must_use = "method returns the modified pane"]
    pub const fn z(mut self, z: u16) -> Self {
        self.z = z;
        self
    }

    /// Computes scroll metrics and a whole-pane pointer target.
    ///
    /// `content_length` is the logical number of rows or records, and `offset` is the desired
    /// starting index. The returned [`ScrollMetrics`] clamps the offset to the visible range.
    pub fn layout(
        self,
        area: Rect,
        content_length: usize,
        offset: usize,
    ) -> ScrollablePaneLayout<Id>
    where
        Id: Copy,
    {
        let content_height = if self.status_line {
            area.height.saturating_sub(1)
        } else {
            area.height
        };
        let content_area = Rect {
            height: content_height,
            ..area
        };
        let status_area = self.status_line.then(|| Rect {
            y: area.y.saturating_add(content_height),
            height: area.height.saturating_sub(content_height),
            ..area
        });
        let content_length = content_length.min(u32::MAX as usize) as u32;
        let offset = offset.min(u32::MAX as usize) as u32;
        let metrics = ScrollMetrics::new(content_length, content_height, offset);
        let pointer = PointerTargets::new().region(self.id, area);
        let frame = FrameSnapshot::new(area).mouse(pointer);
        ScrollablePaneLayout {
            area,
            content_area,
            status_area,
            metrics,
            frame,
        }
    }
}

/// Solved scrollable-pane data for one frame.
///
/// Use the content area for drawing records, the metrics for scrollbar or status text, and the
/// frame snapshot for wheel routing over both content and blank pane space.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ScrollablePaneLayout<Id = usize> {
    area: Rect,
    content_area: Rect,
    status_area: Option<Rect>,
    metrics: ScrollMetrics,
    frame: FrameSnapshot<Id>,
}

impl<Id> ScrollablePaneLayout<Id> {
    /// Returns the full pane area.
    pub const fn area(&self) -> Rect {
        self.area
    }

    /// Returns the area available for scrollable content.
    pub const fn content_area(&self) -> Rect {
        self.content_area
    }

    /// Returns the optional status-line area.
    pub const fn status_area(&self) -> Option<Rect> {
        self.status_area
    }

    /// Returns clamped scroll metrics for the content viewport.
    pub const fn metrics(&self) -> ScrollMetrics {
        self.metrics
    }

    /// Returns the frame snapshot used for wheel routing.
    pub const fn frame(&self) -> &FrameSnapshot<Id> {
        &self.frame
    }
}

#[cfg(test)]
mod tests {
    use ratatui_core::layout::Rect;

    use super::ScrollablePane;

    #[test]
    fn status_line_reduces_content_height() {
        let layout =
            ScrollablePane::new("pane")
                .status_line(true)
                .layout(Rect::new(0, 0, 20, 5), 20, 0);

        assert_eq!(layout.content_area(), Rect::new(0, 0, 20, 4));
        assert_eq!(layout.status_area(), Some(Rect::new(0, 4, 20, 1)));
    }

    #[test]
    fn wheel_routes_across_blank_pane_space() {
        let layout = ScrollablePane::new("pane").layout(Rect::new(0, 0, 20, 5), 2, 0);

        assert_eq!(layout.frame().route_scroll((10, 4)).unwrap().id, "pane");
    }
}
