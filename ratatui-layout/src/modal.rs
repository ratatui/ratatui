//! Modal-shell geometry and outside-click routing.
//!
//! Dialog-like overlays usually need three pieces before any field-specific code runs: a centered
//! outer area, an inner content area, and an optional backdrop target that can route clicks outside
//! the dialog. [`ModalShell`](crate::modal::ModalShell) provides those pieces without rendering a
//! border, dimming the background, owning fields, or deciding whether outside clicks dismiss the
//! dialog.
//!
//! # Types
//!
//! - [`ModalShell`](crate::modal::ModalShell) stores size, padding, z-order, and optional backdrop
//!   id.
//! - [`ModalLayout`](crate::modal::ModalLayout) exposes the solved outer, inner, and backdrop areas
//!   plus frame-local routing.
//!
//! # Examples
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::container::Padding;
//! use ratatui_layout::modal::ModalShell;
//!
//! let layout = ModalShell::new(20, 5)
//!     .padding(Padding::all(1))
//!     .backdrop("outside")
//!     .layout(Rect::new(0, 0, 80, 24));
//!
//! assert_eq!(layout.outer().width, 20);
//! assert_eq!(layout.inner().width, 18);
//! assert_eq!(layout.frame().route_click((0, 0)).unwrap().id, "outside");
//! ```

use ratatui_core::layout::{Constraint, Flex, Layout, Rect};

use crate::container::{Container, Padding};
use crate::frame::FrameSnapshot;
use crate::pointer::{PointerTarget, PointerTargets};

/// Geometry policy for a centered modal surface.
///
/// [`ModalShell`](crate::modal::ModalShell) owns only the overlay shell: desired size, padding,
/// z-order, and optional backdrop routing. Child forms, command rows, focus state, validation, and
/// rendering remain app-owned. Use the returned
/// [`ModalLayout::inner`](crate::modal::ModalLayout::inner) as the area for the dialog contents.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ModalShell<Id = usize> {
    width: u16,
    height: u16,
    padding: Padding,
    backdrop: Option<Id>,
    z: u16,
}

impl ModalShell {
    /// Creates a modal shell with a desired width and height.
    ///
    /// The actual area is clamped by the screen area passed to
    /// [`ModalShell::layout`](crate::modal::ModalShell::layout).
    pub const fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            padding: Padding::new(0, 0, 0, 0),
            backdrop: None,
            z: 100,
        }
    }
}

impl<Id> ModalShell<Id> {
    /// Sets padding between the outer modal area and child content.
    #[must_use = "method returns the modified modal shell"]
    pub const fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Adds a pointer target over the screen area behind the modal.
    ///
    /// The backdrop id lets an app implement outside-click behavior explicitly: dismiss, ignore,
    /// flash validation errors, or move focus back to the dialog.
    #[must_use = "method returns the modified modal shell"]
    pub fn backdrop<Backdrop>(self, backdrop: Backdrop) -> ModalShell<Backdrop> {
        ModalShell {
            width: self.width,
            height: self.height,
            padding: self.padding,
            backdrop: Some(backdrop),
            z: self.z,
        }
    }

    /// Sets the z-order for backdrop routing.
    ///
    /// Child frames should usually use a higher z-order than the backdrop if their areas overlap.
    #[must_use = "method returns the modified modal shell"]
    pub const fn z(mut self, z: u16) -> Self {
        self.z = z;
        self
    }

    /// Solves the centered modal and optional backdrop target.
    ///
    /// The returned frame contains only backdrop routing. Merge child field and button frames into
    /// the returned frame after rendering dialog contents.
    pub fn layout(self, screen: Rect) -> ModalLayout<Id>
    where
        Id: Copy,
    {
        let outer = center(screen, self.width, self.height);
        let container = Container::<Id>::new().padding(self.padding).layout(outer);
        let frame = if let Some(backdrop) = self.backdrop {
            FrameSnapshot::new(screen)
                .mouse(PointerTargets::new().target(PointerTarget::new(backdrop, screen).z(self.z)))
        } else {
            FrameSnapshot::new(screen)
        };
        ModalLayout {
            screen,
            outer,
            inner: container.inner,
            backdrop: self.backdrop,
            frame,
        }
    }
}

/// Solved modal geometry for one frame.
///
/// Render the modal border or block into [`ModalLayout::outer`](crate::modal::ModalLayout::outer),
/// render dialog contents into [`ModalLayout::inner`](crate::modal::ModalLayout::inner), and store
/// or merge [`ModalLayout::frame`](crate::modal::ModalLayout::frame) so the next pointer event can
/// distinguish outside clicks from dialog controls.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ModalLayout<Id = usize> {
    screen: Rect,
    outer: Rect,
    inner: Rect,
    backdrop: Option<Id>,
    frame: FrameSnapshot<Id>,
}

impl<Id> ModalLayout<Id> {
    /// Returns the full screen area used for backdrop routing.
    pub const fn screen(&self) -> Rect {
        self.screen
    }

    /// Returns the centered modal area.
    pub const fn outer(&self) -> Rect {
        self.outer
    }

    /// Returns the child content area after padding.
    pub const fn inner(&self) -> Rect {
        self.inner
    }

    /// Returns the optional backdrop id.
    pub const fn backdrop(&self) -> Option<&Id> {
        self.backdrop.as_ref()
    }

    /// Returns the backdrop frame snapshot.
    pub const fn frame(&self) -> &FrameSnapshot<Id> {
        &self.frame
    }

    /// Merges a child frame into the modal frame.
    ///
    /// Use this after rendering fields and buttons so the stored frame routes controls before the
    /// backdrop when child targets use a higher z-order or later insertion order.
    pub fn merge_child(self, child: FrameSnapshot<Id>) -> FrameSnapshot<Id> {
        self.frame.merge(child)
    }
}

fn center(screen: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(screen.width);
    let height = height.min(screen.height);
    let vertical = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .split(screen);
    let horizontal = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .split(vertical[0]);
    horizontal[0]
}

#[cfg(test)]
mod tests {
    use ratatui_core::layout::Rect;

    use super::ModalShell;
    use crate::container::Padding;

    #[test]
    fn modal_centers_and_applies_padding() {
        let layout = ModalShell::new(20, 6)
            .padding(Padding::all(1))
            .layout(Rect::new(0, 0, 80, 24));

        assert_eq!(layout.outer(), Rect::new(30, 9, 20, 6));
        assert_eq!(layout.inner(), Rect::new(31, 10, 18, 4));
    }

    #[test]
    fn backdrop_routes_outside_clicks() {
        let layout = ModalShell::new(10, 3)
            .backdrop("outside")
            .layout(Rect::new(0, 0, 40, 10));

        assert_eq!(layout.frame().route_click((0, 0)).unwrap().id, "outside");
    }
}
