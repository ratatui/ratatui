//! Cursor placement metadata for focused app-owned content.
//!
//! Terminal cursor placement is layout-related but not widget ownership. These types let a focused
//! participant report where the cursor should appear after layout without requiring a retained
//! widget tree or a custom render trait.
//!
//! The shared rule is render order: code can collect cursor requests while rendering child
//! components, then apply the final visible request after the frame is complete. Hiding the cursor
//! is a request too, but hidden requests do not become the final cursor position; they are useful
//! as explicit data that a component chose not to show a cursor.
//!
//! Use direct `Frame::set_cursor_position` style code when one known widget owns the cursor
//! decision. Use this module when several app-owned children may request cursor placement and the
//! final position should follow render-order composition.
//!
//! # Types
//!
//! - [`CursorRequest`](crate::cursor::CursorRequest) is one frame-local request to show or hide the
//!   terminal cursor at a [`ratatui_core::layout::Position`].
//! - [`CursorRequests`](crate::cursor::CursorRequests) stores requests in render order and reports
//!   the final visible cursor.
//!
//! See [`crate::docs::interaction`] for how cursor requests fit with focus, selection, hover, and
//! disabled state in the frame-local model.
//!
//! # Examples
//!
//! Collect requests from independently rendered children and apply the last visible one:
//!
//! ```rust
//! use ratatui_core::layout::Position;
//! use ratatui_layout::cursor::{CursorRequest, CursorRequests};
//!
//! let input = CursorRequests::new().request(CursorRequest::visible(Position::new(3, 1)));
//! let popup = CursorRequests::new().request(CursorRequest::visible(Position::new(12, 4)));
//! let frame = input.merge(popup);
//!
//! assert_eq!(frame.final_cursor().unwrap().position, Position::new(12, 4));
//! ```

use alloc::vec::Vec;

use ratatui_core::layout::{Position, Rect};

/// A request to place or hide the terminal cursor.
///
/// Use [`CursorRequest`](crate::cursor::CursorRequest) when a focused input, editor, or table cell
/// computes a local cursor position during rendering. The app can collect these requests in a
/// [`CursorRequests`](crate::cursor::CursorRequests) and apply the final one to the frame.
///
/// # Constructors
///
/// - [`CursorRequest::visible`](crate::cursor::CursorRequest::visible) records the terminal
///   position that should show the cursor.
/// - [`CursorRequest::hidden`](crate::cursor::CursorRequest::hidden) records that a component
///   considered cursor placement but should not show a cursor.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Position;
/// use ratatui_layout::cursor::{CursorRequest, CursorRequests};
///
/// let cursor_requests = CursorRequests::new()
///     .request(CursorRequest::visible(Position::new(4, 2)))
///     .request(CursorRequest::hidden(Position::new(8, 2)));
///
/// assert_eq!(
///     cursor_requests.final_cursor(),
///     Some(CursorRequest::visible(Position::new(4, 2)))
/// );
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CursorRequest {
    /// Terminal position for the cursor.
    pub position: Position,

    /// Whether the cursor should be visible.
    pub visible: bool,
}

impl CursorRequest {
    /// Creates a visible cursor request.
    ///
    /// # Examples
    ///
    /// Record the cursor position computed by a focused input field:
    ///
    /// ```rust
    /// use ratatui_core::layout::Position;
    /// use ratatui_layout::cursor::{CursorRequest, CursorRequests};
    ///
    /// let plan = CursorRequests::new().request(CursorRequest::visible(Position::new(12, 3)));
    ///
    /// assert_eq!(plan.final_cursor().unwrap().position, Position::new(12, 3));
    /// ```
    pub const fn visible(position: Position) -> Self {
        Self {
            position,
            visible: true,
        }
    }

    /// Creates a hidden cursor request.
    ///
    /// # Examples
    ///
    /// A read-only focused view can explicitly avoid showing a terminal cursor:
    ///
    /// ```rust
    /// use ratatui_core::layout::Position;
    /// use ratatui_layout::cursor::{CursorRequest, CursorRequests};
    ///
    /// let plan = CursorRequests::new().request(CursorRequest::hidden(Position::new(0, 0)));
    ///
    /// assert_eq!(plan.final_cursor(), None);
    /// assert!(!plan.requests()[0].visible);
    /// ```
    pub const fn hidden(position: Position) -> Self {
        Self {
            position,
            visible: false,
        }
    }
}

/// Cursor requests produced during a render pass.
///
/// Use [`CursorRequests`](crate::cursor::CursorRequests) when multiple app-owned participants may
/// request cursor placement. The last visible [`CursorRequest`](crate::cursor::CursorRequest) wins,
/// matching normal render-order expectations.
///
/// # Constructors and builders
///
/// - [`CursorRequests::new`](crate::cursor::CursorRequests::new) creates an empty request list.
/// - [`CursorRequests::push`](crate::cursor::CursorRequests::push) appends a request to an existing
///   request list.
/// - [`CursorRequests::request`](crate::cursor::CursorRequests::request) appends a request in
///   builder style.
///
/// # Composition and inspection
///
/// - [`CursorRequests::requests`](crate::cursor::CursorRequests::requests) returns all requests in
///   render order for diagnostics or custom policies.
/// - [`CursorRequests::translate`](crate::cursor::CursorRequests::translate) moves child-local
///   cursor positions into parent coordinates.
/// - [`CursorRequests::clip_to`](crate::cursor::CursorRequests::clip_to) hides cursor requests
///   outside a viewport.
/// - [`CursorRequests::extend`](crate::cursor::CursorRequests::extend) appends another request
///   list.
/// - [`CursorRequests::merge`](crate::cursor::CursorRequests::merge) returns a combined request
///   list.
/// - [`CursorRequests::final_cursor`](crate::cursor::CursorRequests::final_cursor) returns the last
///   visible request to apply to the terminal frame.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Position;
/// use ratatui_layout::cursor::{CursorRequest, CursorRequests};
///
/// let form = CursorRequests::new().request(CursorRequest::visible(Position::new(2, 1)));
/// let popup = CursorRequests::new().request(CursorRequest::visible(Position::new(10, 4)));
/// let frame = form.merge(popup);
///
/// assert_eq!(frame.final_cursor().unwrap().position, Position::new(10, 4));
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CursorRequests {
    requests: Vec<CursorRequest>,
}

impl CursorRequests {
    /// Creates an empty cursor request list.
    ///
    /// # Examples
    ///
    /// Use an empty request list before the first draw or for screens without focused text input:
    ///
    /// ```rust
    /// use ratatui_layout::cursor::CursorRequests;
    ///
    /// let cursor_requests = CursorRequests::new();
    /// assert!(cursor_requests.requests().is_empty());
    /// assert_eq!(cursor_requests.final_cursor(), None);
    /// ```
    pub const fn new() -> Self {
        Self {
            requests: Vec::new(),
        }
    }

    /// Adds a cursor request.
    ///
    /// # Examples
    ///
    /// Append requests as focused child widgets render:
    ///
    /// ```rust
    /// use ratatui_core::layout::Position;
    /// use ratatui_layout::cursor::{CursorRequest, CursorRequests};
    ///
    /// let mut plan = CursorRequests::new();
    /// plan.push(CursorRequest::visible(Position::new(6, 1)));
    ///
    /// assert_eq!(plan.requests().len(), 1);
    /// ```
    pub fn push(&mut self, request: CursorRequest) {
        self.requests.push(request);
    }

    /// Adds a request and returns the modified request list.
    ///
    /// # Examples
    ///
    /// Build a small child request list inline:
    ///
    /// ```rust
    /// use ratatui_core::layout::Position;
    /// use ratatui_layout::cursor::{CursorRequest, CursorRequests};
    ///
    /// let plan = CursorRequests::new().request(CursorRequest::visible(Position::new(1, 1)));
    /// assert_eq!(plan.final_cursor().unwrap().position, Position::new(1, 1));
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn request(mut self, request: CursorRequest) -> Self {
        self.push(request);
        self
    }

    /// Returns all cursor requests in render order.
    ///
    /// # Examples
    ///
    /// Inspect child requests when debugging why the final cursor moved:
    ///
    /// ```rust
    /// use ratatui_core::layout::Position;
    /// use ratatui_layout::cursor::{CursorRequest, CursorRequests};
    ///
    /// let plan = CursorRequests::new()
    ///     .request(CursorRequest::visible(Position::new(1, 1)))
    ///     .request(CursorRequest::visible(Position::new(2, 1)));
    ///
    /// assert_eq!(plan.requests()[0].position, Position::new(1, 1));
    /// ```
    pub fn requests(&self) -> &[CursorRequest] {
        &self.requests
    }

    /// Moves all cursor requests by a signed offset.
    ///
    /// Use this when a child component computed cursor positions in local coordinates and the
    /// parent places that child at a screen offset. This mirrors
    /// [`crate::frame::FrameSnapshot::translate`], which translates layout, focus, pointer, and
    /// cursor requests together.
    ///
    /// # Examples
    ///
    /// Translate a field-local cursor request into terminal coordinates:
    ///
    /// ```rust
    /// use ratatui_core::layout::Position;
    /// use ratatui_layout::cursor::{CursorRequest, CursorRequests};
    ///
    /// let local = CursorRequests::new().request(CursorRequest::visible(Position::new(2, 1)));
    /// let screen = local.translate(10, 3);
    ///
    /// assert_eq!(
    ///     screen.final_cursor(),
    ///     Some(CursorRequest::visible(Position::new(12, 4)))
    /// );
    /// ```
    #[must_use = "method returns the translated request list"]
    pub fn translate(mut self, dx: i16, dy: i16) -> Self {
        for request in &mut self.requests {
            request.position = translate_position(request.position, dx, dy);
        }
        self
    }

    /// Hides cursor requests outside a viewport.
    ///
    /// Use this when a child component reports cursor positions inside a clipped pane or scrolling
    /// viewport. Requests outside the viewport stay in render order for diagnostics, but they no
    /// longer count as visible final cursor requests.
    ///
    /// # Examples
    ///
    /// Keep an earlier visible cursor when a later child cursor is clipped away:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Position, Rect};
    /// use ratatui_layout::cursor::{CursorRequest, CursorRequests};
    ///
    /// let plan = CursorRequests::new()
    ///     .request(CursorRequest::visible(Position::new(2, 1)))
    ///     .request(CursorRequest::visible(Position::new(12, 1)))
    ///     .clip_to(Rect::new(0, 0, 10, 3));
    ///
    /// assert_eq!(
    ///     plan.final_cursor(),
    ///     Some(CursorRequest::visible(Position::new(2, 1)))
    /// );
    /// ```
    #[must_use = "method returns the clipped request list"]
    pub fn clip_to(mut self, viewport: Rect) -> Self {
        for request in &mut self.requests {
            if !viewport.contains(request.position) {
                request.visible = false;
            }
        }
        self
    }

    /// Extends this request list with another request list.
    ///
    /// Requests keep render order. Requests from `other` are appended, so a visible request in the
    /// child request can become the final cursor for the aggregate frame.
    ///
    /// # Examples
    ///
    /// Append a child component's cursor requests after the parent request:
    ///
    /// ```rust
    /// use ratatui_core::layout::Position;
    /// use ratatui_layout::cursor::{CursorRequest, CursorRequests};
    ///
    /// let mut parent = CursorRequests::new().request(CursorRequest::visible(Position::new(1, 1)));
    /// let child = CursorRequests::new().request(CursorRequest::visible(Position::new(5, 5)));
    /// parent.extend(child);
    ///
    /// assert_eq!(parent.final_cursor().unwrap().position, Position::new(5, 5));
    /// ```
    pub fn extend(&mut self, other: Self) {
        self.requests.extend(other.requests);
    }

    /// Returns a cursor request list containing this list's requests followed by another list's
    /// requests.
    ///
    /// # Examples
    ///
    /// Combine sibling component values while preserving render-order cursor priority:
    ///
    /// ```rust
    /// use ratatui_core::layout::Position;
    /// use ratatui_layout::cursor::{CursorRequest, CursorRequests};
    ///
    /// let left = CursorRequests::new().request(CursorRequest::visible(Position::new(2, 0)));
    /// let right = CursorRequests::new().request(CursorRequest::visible(Position::new(20, 0)));
    ///
    /// assert_eq!(
    ///     left.merge(right).final_cursor().unwrap().position,
    ///     Position::new(20, 0)
    /// );
    /// ```
    #[must_use = "method returns the merged request list"]
    pub fn merge(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }

    /// Returns the final visible cursor request.
    ///
    /// # Examples
    ///
    /// Apply only the last visible request after all children have rendered:
    ///
    /// ```rust
    /// use ratatui_core::layout::Position;
    /// use ratatui_layout::cursor::{CursorRequest, CursorRequests};
    ///
    /// let plan = CursorRequests::new()
    ///     .request(CursorRequest::visible(Position::new(1, 1)))
    ///     .request(CursorRequest::hidden(Position::new(3, 1)))
    ///     .request(CursorRequest::visible(Position::new(5, 1)));
    ///
    /// assert_eq!(plan.final_cursor().unwrap().position, Position::new(5, 1));
    /// ```
    pub fn final_cursor(&self) -> Option<CursorRequest> {
        self.requests
            .iter()
            .rev()
            .copied()
            .find(|request| request.visible)
    }
}

const fn translate_position(position: Position, dx: i16, dy: i16) -> Position {
    Position::new(
        translate_coordinate(position.x, dx),
        translate_coordinate(position.y, dy),
    )
}

const fn translate_coordinate(coordinate: u16, delta: i16) -> u16 {
    if delta.is_negative() {
        coordinate.saturating_sub(delta.unsigned_abs())
    } else {
        coordinate.saturating_add(delta as u16)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use ratatui_core::layout::{Position, Rect};

    use super::*;

    #[test]
    fn final_cursor_uses_last_visible_request() {
        let plan = CursorRequests::new()
            .request(CursorRequest::visible(Position::new(1, 1)))
            .request(CursorRequest::hidden(Position::new(2, 2)))
            .request(CursorRequest::visible(Position::new(3, 3)));

        assert_eq!(
            plan.final_cursor(),
            Some(CursorRequest::visible(Position::new(3, 3)))
        );
    }

    #[test]
    fn translate_moves_cursor_requests() {
        let plan = CursorRequests::new()
            .request(CursorRequest::visible(Position::new(2, 1)))
            .request(CursorRequest::hidden(Position::new(0, 0)))
            .translate(10, 3);

        assert_eq!(
            plan.requests(),
            &[
                CursorRequest::visible(Position::new(12, 4)),
                CursorRequest::hidden(Position::new(10, 3)),
            ]
        );
    }

    #[test]
    fn clip_hides_cursor_requests_outside_viewport() {
        let plan = CursorRequests::new()
            .request(CursorRequest::visible(Position::new(2, 1)))
            .request(CursorRequest::visible(Position::new(12, 1)))
            .clip_to(Rect::new(0, 0, 10, 3));

        assert_eq!(
            plan.requests(),
            &[
                CursorRequest::visible(Position::new(2, 1)),
                CursorRequest::hidden(Position::new(12, 1)),
            ]
        );
        assert_eq!(
            plan.final_cursor(),
            Some(CursorRequest::visible(Position::new(2, 1)))
        );
    }
}
