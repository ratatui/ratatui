//! Container geometry without retained child widgets.
//!
//! Dialogs, panels, and tool surfaces often need two related rectangles: the outer rectangle where
//! a border or background is drawn, and the inner rectangle where application content is rendered.
//! Ratatui already has good widgets for the visual part, so this module only records the geometry
//! when that geometry must be reused after rendering.
//!
//! The usual flow is higher level than a single rectangle calculation:
//!
//! 1. Configure a [`Container`](crate::container::Container) with
//!    [`Padding`](crate::container::Padding) and, when the inner area should be named, a child id.
//! 2. Call [`Container::layout`](crate::container::Container::layout) during rendering to get a
//!    [`ContainerLayout`](crate::container::ContainerLayout) for that frame.
//! 3. Render ordinary Ratatui widgets into
//!    [`ContainerLayout::outer`](crate::container::ContainerLayout::outer) and
//!    [`ContainerLayout::inner`](crate::container::ContainerLayout::inner).
//! 4. Convert the optional child region into a [`Regions`](crate::regions::Regions) with
//!    [`ContainerLayout::regions`](crate::container::ContainerLayout::regions), or clip child
//!    regions with
//!    [`ContainerLayout::clip_child_regions`](crate::container::ContainerLayout::clip_child_regions)
//!    before merging them into a parent [`crate::frame::FrameSnapshot`].
//!
//! This separation keeps the container useful for common UI shapes without asking it to own child
//! widgets. A dialog can use a `Block` for its border, a form widget for its body, a
//! [`crate::focus::FocusTargets`] for tab order, and this module only for the outer/inner/clipping
//! geometry that later input events need.
//!
//! # Types
//!
//! - [`Padding`](crate::container::Padding) stores per-edge cell counts and can derive the inner
//!   rectangle for any [`ratatui_core::layout::Rect`].
//! - [`Container`](crate::container::Container) is reusable configuration: padding plus an optional
//!   app-owned child id.
//! - [`ContainerLayout`](crate::container::ContainerLayout) is the solved frame-local result: outer
//!   area, inner area, clip boundary, and optional child [`Region`](crate::regions::Region).
//!
//! Use this module when padding, child regions, or clipping boundaries need to become frame-local
//! data that can be merged into a [`crate::frame::FrameSnapshot`] or inspected in tests. For a
//! one-off split inside a render function, Ratatui's `Block::inner` or `Rect::inner` is usually
//! simpler because there is no need to name or store the geometry.
//!
//! See [`crate::docs::containers`] for the broader composition model and examples that combine
//! containers with ordinary Ratatui widgets.
//!
//! # Examples
//!
//! Solve dialog geometry, render chrome in the outer area, and render content in the inner area:
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::container::{Container, Padding};
//!
//! let dialog = Container::<()>::new()
//!     .padding(Padding::symmetric(2, 1))
//!     .layout(Rect::new(10, 5, 40, 8));
//!
//! assert_eq!(dialog.outer, Rect::new(10, 5, 40, 8));
//! assert_eq!(dialog.inner, Rect::new(12, 6, 36, 6));
//! ```
//!
//! When the child area needs to become a parent-visible region, give it an id:
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::container::{Container, Padding};
//!
//! let layout = Container::new()
//!     .padding(Padding::all(1))
//!     .child("body")
//!     .layout(Rect::new(0, 0, 10, 4));
//! let body_regions = layout.regions();
//!
//! assert_eq!(body_regions.hit_test((2, 2)).unwrap().id, "body");
//! ```

use ratatui_core::layout::Rect;

use crate::regions::{Region, Regions};

/// Per-edge padding used to derive an inner content rectangle.
///
/// [`Padding`](crate::container::Padding) exists when horizontal/vertical margins are not
/// expressive enough. It owns only cell counts for the four edges and does not render any visual
/// decoration. Use it with [`Container::padding`](crate::container::Container::padding) or
/// [`Padding::inner`](crate::container::Padding::inner) when edge-specific spacing should be part
/// of the frame-local geometry.
///
/// # Methods
///
/// - [`Padding::new`](crate::container::Padding::new) creates edge-specific padding for asymmetric
///   chrome.
/// - [`Padding::all`](crate::container::Padding::all) creates a uniform inset for simple panels.
/// - [`Padding::symmetric`](crate::container::Padding::symmetric) creates a common terminal inset
///   with separate horizontal and vertical values.
/// - [`Padding::inner`](crate::container::Padding::inner) applies the padding to a [`Rect`] with
///   saturating resize behavior.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::container::Padding;
///
/// let inner = Padding::new(1, 2, 3, 4).inner(Rect::new(0, 0, 20, 10));
/// assert_eq!(inner, Rect::new(1, 2, 16, 4));
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Padding {
    /// Cells reserved on the left edge before child content begins.
    pub left: u16,
    /// Cells reserved on the top edge before child content begins.
    pub top: u16,
    /// Cells reserved on the right edge after child content ends.
    pub right: u16,
    /// Cells reserved on the bottom edge after child content ends.
    pub bottom: u16,
}

impl Padding {
    /// Creates padding with explicit edge values.
    ///
    /// Use this when the visual treatment is asymmetric, such as a title row at the top and a
    /// narrow inset on the remaining edges.
    ///
    /// # Examples
    ///
    /// Reserve extra space below a title while keeping the left and right insets narrow:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::container::Padding;
    ///
    /// let area = Rect::new(0, 0, 30, 8);
    /// let content = Padding::new(1, 2, 1, 1).inner(area);
    ///
    /// assert_eq!(content, Rect::new(1, 2, 28, 5));
    /// ```
    pub const fn new(left: u16, top: u16, right: u16, bottom: u16) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Creates equal padding on all edges.
    ///
    /// This is the common case for simple panels where content should be inset uniformly.
    ///
    /// # Examples
    ///
    /// Give a bordered panel one cell of content padding on every side:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::container::{Container, Padding};
    ///
    /// let panel = Container::<()>::new()
    ///     .padding(Padding::all(1))
    ///     .layout(Rect::new(5, 3, 20, 6));
    ///
    /// assert_eq!(panel.inner, Rect::new(6, 4, 18, 4));
    /// ```
    pub const fn all(value: u16) -> Self {
        Self::new(value, value, value, value)
    }

    /// Creates symmetric horizontal and vertical padding.
    ///
    /// This matches the common terminal pattern of wider horizontal breathing room and tighter
    /// vertical spacing.
    ///
    /// # Examples
    ///
    /// Use wider horizontal padding for dialog text while keeping the dialog compact vertically:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::container::{Container, Padding};
    ///
    /// let dialog = Container::<()>::new()
    ///     .padding(Padding::symmetric(2, 1))
    ///     .layout(Rect::new(0, 0, 24, 5));
    ///
    /// assert_eq!(dialog.inner, Rect::new(2, 1, 20, 3));
    /// ```
    pub const fn symmetric(horizontal: u16, vertical: u16) -> Self {
        Self::new(horizontal, vertical, horizontal, vertical)
    }

    /// Returns the rectangle inside this padding, saturating when the area is too small.
    ///
    /// Saturation matters for resize handling. A dialog may temporarily be smaller than its
    /// padding; returning a zero-sized inner rectangle lets callers skip rendering child content
    /// without panicking or producing underflowed coordinates.
    ///
    /// # Examples
    ///
    /// A very small terminal resize produces a safe empty content area:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::container::Padding;
    ///
    /// let content = Padding::all(3).inner(Rect::new(0, 0, 4, 2));
    ///
    /// assert_eq!(content, Rect::new(3, 2, 0, 0));
    /// ```
    pub const fn inner(self, area: Rect) -> Rect {
        let horizontal = self.left.saturating_add(self.right);
        let vertical = self.top.saturating_add(self.bottom);
        let left = if self.left > area.width {
            area.width
        } else {
            self.left
        };
        let top = if self.top > area.height {
            area.height
        } else {
            self.top
        };
        Rect::new(
            area.x.saturating_add(left),
            area.y.saturating_add(top),
            area.width.saturating_sub(horizontal),
            area.height.saturating_sub(vertical),
        )
    }
}

/// A geometry-only container configuration for one child region.
///
/// [`Container`](crate::container::Container) owns [`Padding`](crate::container::Padding) and an
/// optional child id. It does not own child widgets, blocks, titles, focus, or input state. A
/// render pass asks it to produce [`ContainerLayout`](crate::container::ContainerLayout), then
/// renders normal Ratatui widgets into the outer or inner areas as needed.
///
/// The optional child id is useful when the container is part of a larger screen snapshot and the
/// inner area should be hit-testable or mapped to app-owned content. Leave it unset when the caller
/// only needs the outer and inner rectangles.
///
/// # Constructors and setters
///
/// - [`Container::new`](crate::container::Container::new) creates reusable geometry configuration
///   with no padding or child region.
/// - [`Container::padding`](crate::container::Container::padding) sets the
///   [`Padding`](crate::container::Padding) applied when solving a frame.
/// - [`Container::child`](crate::container::Container::child) names the inner area so it can become
///   a [`Region`](crate::regions::Region) in a [`Regions`](crate::regions::Regions).
/// - [`Container::padding_value`](crate::container::Container::padding_value) returns the
///   configured padding for diagnostics or aligned decoration.
///
/// # Solving
///
/// - [`Container::layout`](crate::container::Container::layout) solves the configuration against a
///   frame area and returns [`ContainerLayout`](crate::container::ContainerLayout).
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::container::{Container, Padding};
///
/// let layout = Container::new()
///     .padding(Padding::all(1))
///     .child("form")
///     .layout(Rect::new(0, 0, 12, 5));
///
/// assert_eq!(layout.child.unwrap().id, "form");
/// assert_eq!(layout.inner, Rect::new(1, 1, 10, 3));
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Container<Id = usize> {
    padding: Padding,
    child: Option<Id>,
}

impl<Id> Container<Id> {
    /// Creates a container with no padding and no child region.
    ///
    /// This is a neutral starting point for builder-style configuration.
    ///
    /// # Examples
    ///
    /// Start from an empty container and add only the frame-local data the view needs:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::container::{Container, Padding};
    ///
    /// let layout = Container::new()
    ///     .padding(Padding::all(1))
    ///     .child("body")
    ///     .layout(Rect::new(0, 0, 10, 4));
    ///
    /// assert_eq!(layout.child.unwrap().id, "body");
    /// ```
    pub const fn new() -> Self {
        Self {
            padding: Padding::new(0, 0, 0, 0),
            child: None,
        }
    }

    /// Sets per-edge padding.
    ///
    /// Padding is applied when [`Container::layout`](crate::container::Container::layout) is
    /// called; it does not affect the supplied outer area itself.
    ///
    /// # Examples
    ///
    /// Keep the assigned outer area for chrome while deriving an inset content area:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::container::{Container, Padding};
    ///
    /// let area = Rect::new(2, 1, 16, 5);
    /// let layout = Container::<()>::new()
    ///     .padding(Padding::symmetric(2, 1))
    ///     .layout(area);
    ///
    /// assert_eq!(layout.outer, area);
    /// assert_eq!(layout.inner, Rect::new(4, 2, 12, 3));
    /// ```
    #[must_use = "method returns the modified value"]
    pub const fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Sets an optional child id for the inner area.
    ///
    /// The id is copied into the [`Region`](crate::regions::Region) stored in
    /// [`ContainerLayout::child`](crate::container::ContainerLayout::child). This lets a parent
    /// merge the container into a larger [`Regions`](crate::regions::Regions) without requiring the
    /// container to know what the child actually renders.
    ///
    /// # Examples
    ///
    /// Name a dialog body so later hit testing can route a click back to application state:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::container::{Container, Padding};
    ///
    /// let plan = Container::new()
    ///     .padding(Padding::all(1))
    ///     .child("dialog-body")
    ///     .layout(Rect::new(0, 0, 12, 5))
    ///     .regions();
    ///
    /// assert_eq!(plan.hit_test((2, 2)).unwrap().id, "dialog-body");
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn child(mut self, child: Id) -> Self {
        self.child = Some(child);
        self
    }

    /// Returns the configured padding.
    ///
    /// This is mainly useful for diagnostics or for code that wants to align decoration with the
    /// same inset policy used by the container.
    ///
    /// # Examples
    ///
    /// Reuse the configured inset when aligning a title or status line with the content area:
    ///
    /// ```rust
    /// use ratatui_layout::container::{Container, Padding};
    ///
    /// let container = Container::<()>::new().padding(Padding::symmetric(2, 1));
    /// let padding = container.padding_value();
    ///
    /// assert_eq!(padding.left, 2);
    /// assert_eq!(padding.top, 1);
    /// ```
    pub const fn padding_value(&self) -> Padding {
        self.padding
    }

    /// Solves the container against an outer area.
    ///
    /// The returned [`ContainerLayout`](crate::container::ContainerLayout) is the frame-local value
    /// to keep. The [`Container`](crate::container::Container) itself is just reusable
    /// configuration and can be rebuilt each frame.
    ///
    /// # Examples
    ///
    /// Produce the geometry a render pass can use for chrome, content, and later input routing:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::container::{Container, Padding};
    ///
    /// let layout = Container::new()
    ///     .padding(Padding::all(1))
    ///     .child("form")
    ///     .layout(Rect::new(10, 4, 30, 8));
    ///
    /// assert_eq!(layout.outer, Rect::new(10, 4, 30, 8));
    /// assert_eq!(layout.inner, Rect::new(11, 5, 28, 6));
    /// assert_eq!(layout.regions().hit_test((12, 6)).unwrap().id, "form");
    /// ```
    pub fn layout(&self, area: Rect) -> ContainerLayout<Id>
    where
        Id: Copy,
    {
        let inner = self.padding.inner(area);
        let child = self.child.map(|id| Region::new(id, inner));
        ContainerLayout {
            outer: area,
            inner,
            clip: inner,
            child,
        }
    }
}

/// Solved container geometry for one frame.
///
/// [`ContainerLayout`](crate::container::ContainerLayout) is the frame-local artifact returned by
/// [`Container::layout`](crate::container::Container::layout). It records the outer area, inner
/// area, clipping boundary, and optional child [`Region`](crate::regions::Region). It does not
/// remember the container configuration or any rendered widget.
///
/// # Fields and methods
///
/// - [`ContainerLayout::outer`](crate::container::ContainerLayout::outer) is the area for chrome
///   such as a border, background, or clear pass.
/// - [`ContainerLayout::inner`](crate::container::ContainerLayout::inner) is the padded content
///   area for ordinary widgets.
/// - [`ContainerLayout::clip`](crate::container::ContainerLayout::clip) is the boundary used when
///   child values must not expose hidden regions.
/// - [`ContainerLayout::child`](crate::container::ContainerLayout::child) is the optional named
///   inner [`Region`](crate::regions::Region).
/// - [`ContainerLayout::regions`](crate::container::ContainerLayout::regions) converts the child
///   region into a [`Regions`](crate::regions::Regions) for parent composition.
/// - [`ContainerLayout::clip_child_regions`](crate::container::ContainerLayout::clip_child_regions)
///   clips child regions to the inner area.
///
/// # Examples
///
/// Clip child regions to the content area so hidden children do not route input:
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::container::{Container, Padding};
/// use ratatui_layout::regions::{Region, Regions};
///
/// let container = Container::<()>::new()
///     .padding(Padding::all(1))
///     .layout(Rect::new(0, 0, 6, 4));
/// let child = Regions::from_regions(
///     Rect::new(0, 0, 6, 4),
///     [Region::new("row", Rect::new(0, 0, 6, 2))],
/// );
///
/// let visible = container.clip_child_regions(child);
/// assert_eq!(visible.regions()[0].area, Rect::new(1, 1, 4, 1));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContainerLayout<Id = usize> {
    /// Area assigned to the container itself.
    ///
    /// Render borders, backgrounds, or clearing widgets here.
    pub outer: Rect,
    /// Area inside container padding.
    ///
    /// Render child content here when it does not need a separate child region.
    pub inner: Rect,
    /// Boundary used to clip child values.
    ///
    /// This currently matches
    /// [`ContainerLayout::inner`](crate::container::ContainerLayout::inner). It is named
    /// separately because clipping is the behavior downstream code cares about when composing
    /// child values.
    pub clip: Rect,
    /// Optional child region covering the inner area.
    ///
    /// This is present only when the source [`Container`](crate::container::Container) was
    /// configured with [`Container::child`](crate::container::Container::child).
    pub child: Option<Region<Id>>,
}

impl<Id> ContainerLayout<Id> {
    /// Returns a [`Regions`](crate::regions::Regions) containing the child region when one exists.
    ///
    /// Use this when the container itself should contribute a child region to a parent region set.
    /// If there is no child id, the returned region set still records the outer area but has no
    /// regions.
    ///
    /// # Examples
    ///
    /// Convert a named inner area into a region set that can be merged with sibling regions:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::container::{Container, Padding};
    ///
    /// let dialog_plan = Container::new()
    ///     .padding(Padding::all(1))
    ///     .child("body")
    ///     .layout(Rect::new(0, 0, 20, 6))
    ///     .regions();
    ///
    /// assert_eq!(dialog_plan.regions()[0].id, "body");
    /// assert_eq!(dialog_plan.regions()[0].area, Rect::new(1, 1, 18, 4));
    /// ```
    pub fn regions(self) -> Regions<Id> {
        match self.child {
            Some(child) => Regions::from_regions(self.outer, [child]),
            None => Regions::new(self.outer),
        }
    }

    /// Clips a child [`Regions`](crate::regions::Regions) to the inner clipping boundary.
    ///
    /// Use this when a child was solved against a larger logical area but should only expose
    /// visible targets inside the container.
    ///
    /// # Examples
    ///
    /// Clip scrollable child regions before storing them for the next pointer event:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::container::{Container, Padding};
    /// use ratatui_layout::regions::{Region, Regions};
    ///
    /// let container = Container::<()>::new()
    ///     .padding(Padding::all(1))
    ///     .layout(Rect::new(0, 0, 8, 4));
    /// let child_plan = Regions::from_regions(
    ///     Rect::new(0, 0, 8, 8),
    ///     [
    ///         Region::new("visible-row", Rect::new(0, 1, 8, 1)),
    ///         Region::new("hidden-row", Rect::new(0, 5, 8, 1)),
    ///     ],
    /// );
    ///
    /// let visible = container.clip_child_regions(child_plan);
    ///
    /// assert_eq!(visible.regions().len(), 1);
    /// assert_eq!(visible.regions()[0].id, "visible-row");
    /// assert_eq!(visible.regions()[0].area, Rect::new(1, 1, 6, 1));
    /// ```
    pub fn clip_child_regions<ChildId>(&self, regions: Regions<ChildId>) -> Regions<ChildId> {
        regions.clip_to(self.clip)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn padding_calculates_inner_area() {
        let layout = Container::new()
            .padding(Padding::new(1, 2, 3, 4))
            .child("body")
            .layout(Rect::new(10, 20, 30, 40));

        assert_eq!(layout.outer, Rect::new(10, 20, 30, 40));
        assert_eq!(layout.inner, Rect::new(11, 22, 26, 34));
        assert_eq!(layout.child.unwrap().area, layout.inner);
    }

    #[test]
    fn padding_saturates_small_areas() {
        let layout = Container::<()>::new()
            .padding(Padding::all(4))
            .layout(Rect::new(0, 0, 3, 2));

        assert_eq!(layout.inner, Rect::new(3, 2, 0, 0));
    }

    #[test]
    fn clips_child_plan_to_inner_area() {
        let layout = Container::<()>::new()
            .padding(Padding::all(1))
            .layout(Rect::new(0, 0, 4, 4));
        let child = Regions::from_regions(
            Rect::new(0, 0, 4, 4),
            [Region::new("row", Rect::new(0, 0, 4, 2))],
        );

        let clipped = layout.clip_child_regions(child);

        assert_eq!(clipped.area(), Rect::new(1, 1, 2, 2));
        assert_eq!(clipped.regions()[0].area, Rect::new(1, 1, 2, 1));
    }
}
