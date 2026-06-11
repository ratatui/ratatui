//! Overlay regions for overlapping UI.
//!
//! An [`Overlay`](crate::overlay::Overlay) collects explicit regions that may overlap. It is useful
//! for popups, floating controls, drag previews, or debug layers where the caller already knows
//! each layer's rectangle. The overlay does not sort or clip regions; it preserves insertion order
//! and z-order metadata for the returned [`Regions`](crate::regions::Regions).
//!
//! Use ordinary direct rendering when an overlay has one known rectangle and no need for hit
//! testing or later inspection. Use this module when several explicit layers need to become one
//! frame-local region set.
//!
//! See [`crate::docs::containers`] for how overlays fit with panels, dialogs, flex layouts, grids,
//! and nested composition.
//!
//! # Type
//!
//! - [`Overlay`](crate::overlay::Overlay) collects explicit [`Region`](crate::regions::Region)
//!   values and returns one [`Regions`](crate::regions::Regions) for hit testing, diagnostics, or
//!   parent composition.
//!
//! [`Regions`](crate::regions::Regions): crate::regions::Regions
//!
//! # Examples
//!
//! Layer a popup over content and keep the topmost layer available for input routing:
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::overlay::Overlay;
//! use ratatui_layout::regions::Region;
//!
//! let overlay_regions = Overlay::new()
//!     .region(Region::new("content", Rect::new(0, 0, 40, 10)))
//!     .region(Region::new("dialog", Rect::new(8, 2, 24, 5)).z(10))
//!     .regions(Rect::new(0, 0, 40, 10));
//!
//! assert_eq!(overlay_regions.hit_test((10, 3)).unwrap().id, "dialog");
//! ```

use alloc::vec::Vec;

use ratatui_core::layout::Rect;

use crate::regions::{Region, Regions};

/// Z-ordered region builder for explicit overlay layers.
///
/// Use [`Overlay`](crate::overlay::Overlay) when the caller already knows each layer's rectangle
/// and needs those layers in one hit-testable [`Regions`](crate::regions::Regions). Popups,
/// floating command palettes, drag previews, and debug overlays all have positioning policy outside
/// the overlay itself.
///
/// [`Overlay`](crate::overlay::Overlay) is intentionally small. It provides a fluent way to build a
/// [`Regions`](crate::regions::Regions) from already-solved regions and leaves all positioning
/// policy to the caller.
///
/// # Constructors and solving
///
/// - [`Overlay::new`](crate::overlay::Overlay::new) creates an empty overlay.
/// - [`Overlay::region`](crate::overlay::Overlay::region) appends an explicit layer in render
///   order.
/// - [`Overlay::regions`](crate::overlay::Overlay::regions) returns the collected regions as a
///   [`Regions`](crate::regions::Regions).
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::overlay::Overlay;
/// use ratatui_layout::regions::Region;
///
/// let overlay_regions = Overlay::new()
///     .region(Region::new("content", Rect::new(0, 0, 20, 5)))
///     .region(Region::new("popup", Rect::new(4, 1, 10, 3)).z(10))
///     .regions(Rect::new(0, 0, 20, 5));
///
/// assert_eq!(overlay_regions.hit_test((5, 2)).unwrap().id, "popup");
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Overlay<Id = usize> {
    regions: Vec<Region<Id>>,
}

impl<Id> Default for Overlay<Id> {
    fn default() -> Self {
        Self {
            regions: Vec::new(),
        }
    }
}

impl<Id> Overlay<Id> {
    /// Creates an empty overlay.
    ///
    /// # Examples
    ///
    /// Start an overlay when popup placement is computed elsewhere:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::overlay::Overlay;
    /// use ratatui_layout::regions::Region;
    ///
    /// let overlay = Overlay::new().region(Region::new("tooltip", Rect::new(2, 1, 8, 1)));
    ///
    /// assert_eq!(overlay.regions(Rect::new(0, 0, 20, 4)).regions().len(), 1);
    /// ```
    pub const fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }

    /// Adds a region to the overlay.
    ///
    /// Regions are appended in render order. Hit testing later uses z-order first and insertion
    /// order as the tie-breaker.
    ///
    /// # Examples
    ///
    /// Add a floating command palette above a base content layer:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::overlay::Overlay;
    /// use ratatui_layout::regions::Region;
    ///
    /// let overlay = Overlay::new()
    ///     .region(Region::new("base", Rect::new(0, 0, 30, 10)))
    ///     .region(Region::new("palette", Rect::new(5, 2, 20, 4)).z(5));
    ///
    /// assert_eq!(
    ///     overlay
    ///         .regions(Rect::new(0, 0, 30, 10))
    ///         .hit_test((6, 3))
    ///         .unwrap()
    ///         .id,
    ///     "palette"
    /// );
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn region(mut self, region: Region<Id>) -> Self {
        self.regions.push(region);
        self
    }

    /// Solves the overlay into regions.
    ///
    /// The supplied `area` is the parent overlay area. It does not clamp region rectangles.
    ///
    /// # Examples
    ///
    /// Store explicit overlay regions for previous-frame hit testing:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::overlay::Overlay;
    /// use ratatui_layout::regions::Region;
    ///
    /// let previous_frame = Overlay::new()
    ///     .region(Region::new("drag-preview", Rect::new(10, 4, 6, 2)).z(20))
    ///     .regions(Rect::new(0, 0, 40, 12));
    ///
    /// assert_eq!(previous_frame.area(), Rect::new(0, 0, 40, 12));
    /// ```
    pub fn regions(self, area: Rect) -> Regions<Id> {
        Regions::from_regions(area, self.regions)
    }
}
