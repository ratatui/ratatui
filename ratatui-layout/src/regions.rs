//! Layout values and hit testing.
//!
//! A region set is the reusable result of a container's geometry calculation. It contains the area
//! the container solved against and an ordered set of [`Region`] values for externally owned
//! content. The value is useful even when the caller does not render immediately: it can drive hit
//! testing, status text, scrollbars, pointer handling, debugging output, or custom render passes.
//!
//! This module uses "area" and "region" for related but different ideas. An area is a
//! [`ratatui_core::layout::Rect`]: geometry only. A region is a frame-local record built from an
//! area plus the app id, clipping metadata, and z-order needed for routing and composition. If a
//! rectangle is consumed immediately by `Widget::render`, call it an area. If it must be stored,
//! hit tested, merged, or mapped back to app state after rendering, call it a region.
//!
//! Region sets deliberately store identifiers instead of widgets. The identifier can be an index,
//! an enum, a stable application key, or any small value the caller can use to find its own data.
//!
//! A region set is not needed for every layout. When a render function can destructure rectangles
//! and render immediately, Ratatui's built-in layout APIs are simpler. Use a region set when the
//! rectangles need to cross a boundary: input handling, custom containers, tests, diagnostics, or
//! external rendering.
//!
//! # Common uses
//!
//! - **Route input from the previous frame.** Store a [`Regions`] after drawing, then call
//!   [`Regions::hit_test`] when the next pointer event arrives.
//! - **Render app-owned children.** A parent can solve a list of [`Region`] values while the
//!   application keeps ownership of rows, cells, or controls.
//! - **Compose child components.** A child can solve local geometry, then a parent can
//!   [`translate`](Regions::translate), [`clip`](Regions::clip_to), [`map`](Regions::map_id), and
//!   [`merge`](Regions::merge) it into a larger region set.
//! - **Make layout testable.** Tests can assert on assigned rectangles, clipping, and z-order
//!   without rendering a terminal buffer.
//!
//! # Types
//!
//! - [`Clip`] stores per-edge clipping metadata for partially visible regions.
//! - [`Region`] names one visible rectangle with an app-owned id, clip metadata, and z-order.
//! - [`Hit`] is returned by hit testing and includes the winning id plus local coordinates.
//! - [`Regions`] stores a parent area and ordered regions for one frame.
//!
//! A stored region set can route a later input event back to app data:
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::{Region, Regions};
//!
//! let previous_frame = Regions::from_regions(
//!     Rect::new(0, 0, 30, 1),
//!     [
//!         Region::new("open", Rect::new(0, 0, 10, 1)),
//!         Region::new("save", Rect::new(10, 0, 10, 1)),
//!     ],
//! );
//!
//! assert_eq!(previous_frame.hit_test((12, 0)).unwrap().id, "save");
//! ```
//!
//! Child values can also be composed without the parent owning child widgets:
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::{Region, Regions};
//!
//! #[derive(Debug, Clone, Copy, Eq, PartialEq)]
//! enum AppRegion {
//!     DialogField(usize),
//!     Help,
//! }
//!
//! let field_regions = [
//!     Region::new(0, Rect::new(0, 0, 20, 1)),
//!     Region::new(1, Rect::new(0, 1, 20, 1)),
//! ];
//! let fields = Regions::from_regions(Rect::new(0, 0, 20, 2), field_regions)
//!     .map_id(AppRegion::DialogField)
//!     .translate(5, 3);
//! let help = Regions::from_regions(
//!     Rect::new(0, 0, 40, 10),
//!     [Region::new(AppRegion::Help, Rect::new(0, 9, 40, 1))],
//! );
//!
//! let screen = help.merge(fields);
//! assert_eq!(
//!     screen.hit_test((6, 4)).unwrap().id,
//!     AppRegion::DialogField(1)
//! );
//! ```
//!
//! See [`crate::docs::regions`] for the broader explanation of values as frame-local geometry
//! data, and [`crate::docs::frame_snapshots`] for how region data compose with focus, pointer, and
//! cursor requests.

use alloc::vec::Vec;
use core::slice;

use ratatui_core::layout::{Position, Rect};

/// Clipping metadata for a partially visible region.
///
/// Use [`Clip`] when a renderer needs to know that its assigned rectangle is only part of a larger
/// logical item. A virtual list row, for example, may start above the viewport and receive only its
/// visible tail. The region area tells the renderer where to draw; the clip metadata explains which
/// edges were omitted.
///
/// Linear layouts usually leave this empty. Viewports and virtualized containers use it to preserve
/// enough information for continuation markers, clipped-line rendering, or hit-test adjustment.
///
/// # Common uses
///
/// - A virtual list can tell a row renderer that the first two logical lines were scrolled off the
///   top of the viewport.
/// - A clipped overlay can preserve that an item continues beyond the right edge, even though the
///   visible [`Region::area`] is smaller.
/// - Tests can assert that a container dropped hidden regions and correctly recorded the visible
///   edges of partially visible regions.
///
/// # Method
///
/// - [`Clip::is_empty`] reports whether any edge was clipped.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::{Clip, Region};
///
/// let region = Region::new("row", Rect::new(0, 0, 10, 3))
///     .clip_to(Rect::new(0, 1, 10, 1))
///     .unwrap();
///
/// assert_eq!(region.area, Rect::new(0, 1, 10, 1));
/// assert_eq!(region.clip.top, 1);
/// assert_eq!(region.clip.bottom, 1);
/// assert!(!region.clip.is_empty());
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Clip {
    /// The number of cells clipped from the left side.
    pub left: u16,
    /// The number of cells clipped from the top side.
    pub top: u16,
    /// The number of cells clipped from the right side.
    pub right: u16,
    /// The number of cells clipped from the bottom side.
    pub bottom: u16,
}

impl Clip {
    /// Returns true when no edge is clipped.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::Clip;
    ///
    /// assert!(Clip::default().is_empty());
    /// assert!(
    ///     !Clip {
    ///         top: 1,
    ///         ..Clip::default()
    ///     }
    ///     .is_empty()
    /// );
    /// ```
    pub const fn is_empty(self) -> bool {
        self.left == 0 && self.top == 0 && self.right == 0 && self.bottom == 0
    }
}

/// A frame-local region for externally owned content.
///
/// Use [`Region`] when the parent layout knows where something belongs but the application still
/// owns the thing itself. For example, a grid can return a region with id `row * column_count +
/// column`, and the app can use that id to look up and render the corresponding cell.
///
/// A [`Region`] is the smallest unit in a [`Regions`]. It answers: "render the external item with
/// this id in this area, at this z-order, with this clipping metadata." The region does not store
/// the external item or any widget state.
///
/// The [`Region::area`] field is still a plain [`ratatui_core::layout::Rect`]. That is intentional:
/// rendering code should pass the area to Ratatui widgets exactly as it would any other rectangle.
/// The surrounding [`Region`] exists only when the rectangle also needs an id, clipping metadata,
/// or z-order for later coordination. A local variable named `area` should usually be a `Rect`; a
/// value named `region` should usually be a `Region<Id>`.
///
/// # Type parameter
///
/// `Id` is chosen by the caller. The default is `usize`, which works well for row-major grids and
/// indexed collections. String names are useful in examples, tests, diagnostics, and small
/// prototypes because hit-test failures are easy to read. Enums are usually the best fit for app
/// controls because they give the compiler a closed set of routes. Use a stable record key when the
/// region identity should survive filtering, sorting, or reordering.
///
/// # Common uses
///
/// - Use `usize` ids for simple row-major grids, tabs, or toolbar segments where index order is the
///   stable contract.
/// - Use an enum id when a region represents a named control such as `Save`, `Cancel`, or `Search`.
/// - Use a stable application key when rows can be filtered or reordered but input should still
///   route to the same underlying item.
/// - Use z-order on overlay regions so a floating popup can win hit testing over the content below.
///
/// # Constructors and setters
///
/// - [`Region::new`] creates a fully visible z-zero region.
/// - [`Region::clip`] sets clipping metadata when a viewport has already computed it.
/// - [`Region::z`] sets hit-test priority for overlays and floating regions.
///
/// # Geometry helpers
///
/// - [`Region::contains`] checks geometry without z-order.
/// - [`Region::local_position`] converts terminal coordinates to region-local coordinates.
/// - [`Region::translate`] moves child-local geometry into parent coordinates.
/// - [`Region::clip_to`] clips a region to a viewport and records omitted edges.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::{Region, Regions};
///
/// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// enum Button {
///     Save,
///     Cancel,
/// }
///
/// let button_regions = Regions::from_regions(
///     Rect::new(0, 0, 20, 1),
///     [
///         Region::new(Button::Save, Rect::new(0, 0, 10, 1)),
///         Region::new(Button::Cancel, Rect::new(10, 0, 10, 1)),
///     ],
/// );
///
/// assert_eq!(button_regions.hit_test((12, 0)).unwrap().id, Button::Cancel);
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Region<Id = usize> {
    /// The external identifier this region belongs to.
    ///
    /// This is the value returned by hit testing and used by the app to find its own data.
    pub id: Id,
    /// The assigned area.
    ///
    /// Render app-owned content into this rectangle. The area is already clipped when the region
    /// came from [`Region::clip_to`] or [`Regions::clip_to`].
    pub area: Rect,
    /// Clipping metadata for the assigned area.
    ///
    /// Renderers can use this to decide whether to skip logical content, draw continuation
    /// markers, or adjust local hit coordinates.
    pub clip: Clip,
    /// Z ordering used for hit testing and overlays.
    ///
    /// Higher values win. When z-order ties, later regions win because they are usually rendered
    /// later.
    pub z: u16,
}

impl<Id> Region<Id> {
    /// Creates a region with no clipping and z-order zero.
    ///
    /// Use this for ordinary regions where render order is enough and the region is fully visible.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::Region;
    ///
    /// let region = Region::new("status", Rect::new(0, 0, 10, 1));
    /// assert_eq!(region.id, "status");
    /// assert_eq!(region.z, 0);
    /// ```
    pub const fn new(id: Id, area: Rect) -> Self {
        Self {
            id,
            area,
            clip: Clip {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            z: 0,
        }
    }

    /// Sets the region clipping metadata.
    ///
    /// This is a fluent setter that is most useful when a viewport or virtualized layout has
    /// already computed visible clipping and wants to preserve that information in the region.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{Clip, Region};
    ///
    /// let region = Region::new("item", Rect::new(0, 0, 5, 1)).clip(Clip {
    ///     left: 2,
    ///     ..Clip::default()
    /// });
    ///
    /// assert_eq!(region.clip.left, 2);
    /// ```
    #[must_use = "method returns the modified value"]
    pub const fn clip(mut self, clip: Clip) -> Self {
        self.clip = clip;
        self
    }

    /// Sets the region z-order.
    ///
    /// Higher z-order regions win hit testing. When z-order is equal, later regions win because
    /// they are usually rendered later.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{Region, Regions};
    ///
    /// let plan = Regions::from_regions(
    ///     Rect::new(0, 0, 10, 1),
    ///     [
    ///         Region::new("content", Rect::new(0, 0, 10, 1)),
    ///         Region::new("popup", Rect::new(0, 0, 10, 1)).z(10),
    ///     ],
    /// );
    ///
    /// assert_eq!(plan.hit_test((1, 0)).unwrap().id, "popup");
    /// ```
    #[must_use = "method returns the modified value"]
    pub const fn z(mut self, z: u16) -> Self {
        self.z = z;
        self
    }

    /// Returns true when the region area contains the position.
    ///
    /// This is a geometry-only check. It ignores z-order and disabled state. Use
    /// [`Regions::hit_test`] or [`crate::PointerTargets::hit_test`] when routing real input.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::Region;
    ///
    /// let region = Region::new("button", Rect::new(4, 2, 8, 1));
    /// assert!(region.contains((5, 2)));
    /// assert!(!region.contains((3, 2)));
    /// ```
    pub fn contains<P: Into<Position>>(&self, position: P) -> bool {
        self.area.contains(position.into())
    }

    /// Returns the position relative to the region area.
    ///
    /// Use this when routing pointer input to app-owned content that wants local coordinates
    /// instead of terminal coordinates.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Position, Rect};
    /// use ratatui_layout::Region;
    ///
    /// let region = Region::new("cell", Rect::new(10, 5, 4, 2));
    /// assert_eq!(region.local_position((12, 6)), Some(Position::new(2, 1)));
    /// ```
    pub fn local_position<P: Into<Position>>(&self, position: P) -> Option<Position> {
        let position = position.into();
        self.contains(position).then(|| {
            Position::new(
                position.x.saturating_sub(self.area.x),
                position.y.saturating_sub(self.area.y),
            )
        })
    }

    /// Moves the region area by a signed offset.
    ///
    /// This is useful when child regions were solved in local coordinates and then need to be moved
    /// into its parent area before being merged.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::Region;
    ///
    /// let region = Region::new("field", Rect::new(1, 1, 10, 1)).translate(20, 3);
    /// assert_eq!(region.area, Rect::new(21, 4, 10, 1));
    /// ```
    #[must_use = "method returns the translated region"]
    pub const fn translate(mut self, dx: i16, dy: i16) -> Self {
        self.area = translate_rect(self.area, dx, dy);
        self
    }

    /// Clips the region to a viewport area.
    ///
    /// Returns `None` when the region is entirely outside the viewport. The returned region
    /// preserves the id and z-order and adds clipping metadata for the omitted edges.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::Region;
    ///
    /// let visible = Region::new("row", Rect::new(0, 0, 10, 3))
    ///     .clip_to(Rect::new(0, 1, 10, 2))
    ///     .unwrap();
    ///
    /// assert_eq!(visible.area, Rect::new(0, 1, 10, 2));
    /// assert_eq!(visible.clip.top, 1);
    /// ```
    pub fn clip_to(mut self, viewport: Rect) -> Option<Self> {
        let clipped = intersect(self.area, viewport)?;
        self.clip.left = self
            .clip
            .left
            .saturating_add(clipped.x.saturating_sub(self.area.x));
        self.clip.top = self
            .clip
            .top
            .saturating_add(clipped.y.saturating_sub(self.area.y));
        self.clip.right = self
            .clip
            .right
            .saturating_add(self.area.right().saturating_sub(clipped.right()));
        self.clip.bottom = self
            .clip
            .bottom
            .saturating_add(self.area.bottom().saturating_sub(clipped.bottom()));
        self.area = clipped;
        Some(self)
    }
}

/// A hit-tested region with coordinates relative to the region area.
///
/// Use [`Hit`] when a pointer position needs to be routed back to application data. The id
/// identifies the external item, and the relative coordinates let the item interpret the event in
/// its own local coordinate system.
///
/// [`Hit`] is returned by [`Regions::hit_test`]. It carries the external id and both the absolute
/// region area and the pointer position relative to that area.
///
/// # Common uses
///
/// - Route a click to an app-owned row or cell using [`Hit::id`].
/// - Interpret pointer position inside a row using [`Hit::relative_x`] and [`Hit::relative_y`].
/// - Keep the absolute [`Hit::area`] around when drawing feedback such as hover or selection
///   styling on the next frame.
///
/// # Fields
///
/// - [`Hit::id`] is the app-owned id copied from the winning region or target.
/// - [`Hit::area`] is the terminal-space rectangle that was hit.
/// - [`Hit::relative_x`] and [`Hit::relative_y`] are coordinates inside [`Hit::area`].
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::{Region, Regions};
///
/// let row_regions = Regions::from_regions(
///     Rect::new(0, 0, 20, 2),
///     [Region::new("row-42", Rect::new(2, 0, 10, 2))],
/// );
/// let hit = row_regions.hit_test((5, 1)).unwrap();
///
/// assert_eq!(hit.id, "row-42");
/// assert_eq!((hit.relative_x, hit.relative_y), (3, 1));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hit<Id = usize> {
    /// The external identifier hit.
    ///
    /// This is copied from the winning [`Region`] and should be enough for the app to route the
    /// event.
    pub id: Id,
    /// The region area.
    ///
    /// This is the absolute terminal rectangle that was hit.
    pub area: Rect,
    /// X coordinate relative to the region.
    ///
    /// Use this when a child renderer wants local coordinates.
    pub relative_x: u16,
    /// Y coordinate relative to the region.
    ///
    /// Use this when a row or cell needs to know which local line was hit.
    pub relative_y: u16,
}

/// A reusable layout result with inspectable regions and hit testing.
///
/// Use [`Regions`] when a layout result needs to cross a boundary. A render function that
/// immediately destructures rectangles can use Ratatui's built-in `Layout` directly. A component
/// that needs to render app-owned items, route pointer events, test geometry, or report visible
/// regions benefits from carrying a region set.
///
/// A [`Regions`] is a value object. It does not borrow the layout that produced it and it does not
/// borrow the external content identified by its regions. Callers can pass it to render code,
/// inspect it in tests, or keep it around until the next frame to route input.
///
/// # Common uses
///
/// 1. **Hit-test app-owned content.** A virtual list can expose visible row regions, and the app
///    can route a pointer event to the row id returned by [`Regions::hit_test`].
/// 2. **Build parent regions from children.** A dialog can solve its fields locally, map field ids
///    into an app enum, translate them into the dialog area, and merge them with overlay regions.
/// 3. **Clip nested UI.** A scroll viewport can clip child regions so hidden rows do not receive
///    input or appear in diagnostics.
/// 4. **Assert layout behavior.** A test can compare [`Regions::regions`] against expected
///    rectangles without constructing widgets.
///
/// # Constructors and builders
///
/// - [`Regions::new`] creates an empty region set for a solved parent area.
/// - [`Regions::from_regions`] creates a region set from precomputed regions.
/// - [`Regions::push`] and [`Regions::region`] append regions.
/// - [`Regions::extend`] and [`Regions::merge`] combine child regions.
///
/// # Inspection and routing
///
/// - [`Regions::area`] returns the solved parent area.
/// - [`Regions::regions`] and [`Regions::iter`] expose regions in render order.
/// - [`Regions::ids`] exposes only region ids for selection traversal or diagnostics.
/// - [`Regions::is_empty`] reports whether the set has regions.
/// - [`Regions::hit_test`] routes a terminal position to the topmost region and returns a [`Hit`].
///
/// # Composition
///
/// - [`Regions::map_id`] converts local ids into app-level ids.
/// - [`Regions::translate`] moves child-local geometry into parent coordinates.
/// - [`Regions::clip_to`] removes hidden regions and records clipping metadata.
///
/// The following example shows the render-loop shape without requiring a terminal buffer: app data
/// stays in a slice, while the region set only stores ids and rectangles.
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::{Region, Regions};
///
/// let labels = ["open", "save"];
/// let label_regions = Regions::from_regions(
///     Rect::new(0, 0, 20, 1),
///     [
///         Region::new(0, Rect::new(0, 0, 10, 1)),
///         Region::new(1, Rect::new(10, 0, 10, 1)),
///     ],
/// );
///
/// let rendered_labels = label_regions
///     .regions()
///     .iter()
///     .map(|region| labels[region.id])
///     .collect::<Vec<_>>();
///
/// assert_eq!(rendered_labels, ["open", "save"]);
/// ```
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::{Region, Regions};
///
/// let field_regions = Regions::from_regions(
///     Rect::new(0, 0, 20, 1),
///     [Region::new("name", Rect::new(0, 0, 10, 1))],
/// );
///
/// assert_eq!(field_regions.hit_test((3, 0)).unwrap().id, "name");
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Regions<Id = usize> {
    area: Rect,
    regions: Vec<Region<Id>>,
}

impl<Id> Regions<Id> {
    /// Creates an empty region set for the given area.
    ///
    /// Use this when a container has no visible children but the solved area is still meaningful
    /// for diagnostics, hit testing, or later mutation with [`Regions::push`].
    ///
    /// # Examples
    ///
    /// Start with empty screen regions and append regions as components render:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{Region, Regions};
    ///
    /// let mut screen_regions = Regions::new(Rect::new(0, 0, 20, 1));
    /// screen_regions.push(Region::new("status", Rect::new(0, 0, 20, 1)));
    ///
    /// assert_eq!(screen_regions.hit_test((3, 0)).unwrap().id, "status");
    /// ```
    pub const fn new(area: Rect) -> Self {
        Self {
            area,
            regions: Vec::new(),
        }
    }

    /// Creates a region set from precomputed regions.
    ///
    /// This is the common constructor for layout solvers. The regions retain their input order, and
    /// that order is part of hit-test tie-breaking when z-order is equal.
    ///
    /// Use this when a layout helper has already solved every child rectangle and wants to return
    /// one inspectable [`Regions`] value to the caller.
    ///
    /// # Examples
    ///
    /// Return app-owned button regions from a small layout helper:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{Region, Regions};
    ///
    /// let button_regions = Regions::from_regions(
    ///     Rect::new(0, 0, 16, 1),
    ///     [
    ///         Region::new("ok", Rect::new(0, 0, 8, 1)),
    ///         Region::new("cancel", Rect::new(8, 0, 8, 1)),
    ///     ],
    /// );
    ///
    /// assert_eq!(button_regions.regions()[1].id, "cancel");
    /// ```
    pub fn from_regions(area: Rect, regions: impl Into<Vec<Region<Id>>>) -> Self {
        Self {
            area,
            regions: regions.into(),
        }
    }

    /// Returns the area the region set was solved for.
    ///
    /// This is the parent area, not the union of all regions. Empty space and clipped content may
    /// make those values differ.
    ///
    /// # Examples
    ///
    /// Keep the solved parent area for diagnostics even when there are no regions:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::Regions;
    ///
    /// let plan = Regions::<()>::new(Rect::new(0, 0, 80, 24));
    ///
    /// assert_eq!(plan.area(), Rect::new(0, 0, 80, 24));
    /// ```
    pub const fn area(&self) -> Rect {
        self.area
    }

    /// Returns all regions.
    ///
    /// Regions are returned in render order. Later regions are considered later-rendered for
    /// hit-test ties.
    ///
    /// This is the main render loop API: iterate the regions, look up app-owned data by
    /// [`Region::id`], and render into [`Region::area`].
    ///
    /// # Examples
    ///
    /// Use region ids to look up app-owned labels during rendering:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{Region, Regions};
    ///
    /// let labels = ["open", "save"];
    /// let plan = Regions::from_regions(
    ///     Rect::new(0, 0, 20, 1),
    ///     [
    ///         Region::new(0, Rect::new(0, 0, 10, 1)),
    ///         Region::new(1, Rect::new(10, 0, 10, 1)),
    ///     ],
    /// );
    /// let rendered: Vec<_> = plan
    ///     .regions()
    ///     .iter()
    ///     .map(|region| labels[region.id])
    ///     .collect();
    ///
    /// assert_eq!(rendered, ["open", "save"]);
    /// ```
    pub fn regions(&self) -> &[Region<Id>] {
        &self.regions
    }

    /// Returns region ids in render order.
    ///
    /// Use this when another state object needs only the visible identities, not the rectangles.
    /// The most common case is selection traversal: a region set already knows which controls or
    /// rows are visible, and [`crate::SelectionState`] can move through those ids without learning
    /// about geometry.
    ///
    /// # Examples
    ///
    /// Drive selection traversal from the ids visible in a frame-local region set:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{Region, Regions, SelectionMode, SelectionState};
    ///
    /// let plan = Regions::from_regions(
    ///     Rect::new(0, 0, 10, 2),
    ///     [
    ///         Region::new("first", Rect::new(0, 0, 10, 1)),
    ///         Region::new("second", Rect::new(0, 1, 10, 1)),
    ///     ],
    /// );
    /// let visible = plan.ids().copied().collect::<Vec<_>>();
    /// let mut selection = SelectionState::new(SelectionMode::Single);
    /// selection.select_next(&visible);
    ///
    /// assert_eq!(selection.primary(), Some("first"));
    /// ```
    pub fn ids(&self) -> impl Iterator<Item = &Id> {
        self.regions.iter().map(|region| &region.id)
    }

    /// Pairs regions with another id list in render order.
    ///
    /// Use this when a layout helper emits structural positions but rendering and routing should
    /// use application ids. A toolbar is the common case: a [`Grid`](crate::Grid) may produce
    /// [`GridPosition`](crate::GridPosition) regions, while the app wants to render and route
    /// `Command` enum values. Pairing regions and ids in one iterator keeps render code and
    /// frame-snapshot construction from each rebuilding the same index mapping.
    ///
    /// The iterator stops at the shorter input. That makes it useful for visible subsets, but fixed
    /// layouts should still keep tests around the expected count so missing controls are caught
    /// where the layout is defined.
    ///
    /// # Examples
    ///
    /// Render a toolbar with command ids instead of grid coordinates:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::{Grid, GridPosition};
    ///
    /// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    /// enum Command {
    ///     Edit,
    ///     Save,
    /// }
    ///
    /// let grid = Grid::new(
    ///     [Constraint::Length(1)],
    ///     [Constraint::Length(4), Constraint::Length(4)],
    /// )
    /// .layout(Rect::new(0, 0, 8, 1));
    /// let paired = grid
    ///     .cells()
    ///     .zip_ids(&[Command::Edit, Command::Save])
    ///     .map(|(region, command)| (region.id, command))
    ///     .collect::<Vec<_>>();
    ///
    /// assert_eq!(paired[0], (GridPosition::new(0, 0), Command::Edit));
    /// ```
    pub fn zip_ids<'a, OtherId: Copy>(
        &'a self,
        ids: &'a [OtherId],
    ) -> impl Iterator<Item = (&'a Region<Id>, OtherId)> + 'a {
        self.regions.iter().zip(ids.iter().copied())
    }

    /// Returns the first region with the requested id.
    ///
    /// This is the structural-layout lookup API. Use it when a region set represents named areas
    /// such as a page header, body, and footer. Iterating [`Regions::regions`] remains the
    /// better tool for repeated rows where duplicate ids are expected or where render order
    /// matters more than direct lookup.
    ///
    /// If a region set contains duplicate ids, this returns the first matching region in render
    /// order. For fixed page structure, prefer unique ids so the lookup reads like a total map
    /// from region name to rectangle.
    ///
    /// # Examples
    ///
    /// Look up a named body region after solving a page column:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::Column;
    ///
    /// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    /// enum PageSlot {
    ///     Header,
    ///     Body,
    /// }
    ///
    /// let page_slots = [
    ///     (PageSlot::Header, Constraint::Length(1)),
    ///     (PageSlot::Body, Constraint::Fill(1)),
    /// ];
    /// let page = Column::named(page_slots).regions(Rect::new(0, 0, 20, 5));
    ///
    /// assert_eq!(page.region_for(PageSlot::Body).unwrap().area.height, 4);
    /// ```
    #[allow(
        clippy::needless_pass_by_value,
        reason = "enum ids are the normal lookup case, and value lookup keeps call sites readable"
    )]
    pub fn region_for(&self, id: Id) -> Option<&Region<Id>>
    where
        Id: Eq,
    {
        self.regions.iter().find(|region| region.id == id)
    }

    /// Returns the area of the first region with the requested id.
    ///
    /// This is a convenience wrapper around [`Regions::region_for`] for the common case where a
    /// parent layout only needs a child region's rectangle. Use [`Regions::region_for`] when code
    /// also needs clipping or z-order metadata.
    ///
    /// # Examples
    ///
    /// Pass a named body area to a nested row:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::{Column, Row};
    ///
    /// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    /// enum PageSlot {
    ///     Header,
    ///     Body,
    /// }
    ///
    /// let page_slots = [
    ///     (PageSlot::Header, Constraint::Length(1)),
    ///     (PageSlot::Body, Constraint::Fill(1)),
    /// ];
    /// let page = Column::named(page_slots).regions(Rect::new(0, 0, 20, 5));
    /// let body_area = page.area_for(PageSlot::Body).unwrap();
    /// let body = Row::new([Constraint::Fill(1)]).regions(body_area);
    ///
    /// assert_eq!(body.regions()[0].area, Rect::new(0, 1, 20, 4));
    /// ```
    pub fn area_for(&self, id: Id) -> Option<Rect>
    where
        Id: Eq,
    {
        self.region_for(id).map(|region| region.area)
    }

    /// Returns an iterator over regions.
    ///
    /// This is equivalent to iterating over `&plan`.
    ///
    /// # Examples
    ///
    /// Collect visible ids for selection traversal:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{Region, Regions};
    ///
    /// let plan = Regions::from_regions(
    ///     Rect::new(0, 0, 10, 2),
    ///     [
    ///         Region::new("first", Rect::new(0, 0, 10, 1)),
    ///         Region::new("second", Rect::new(0, 1, 10, 1)),
    ///     ],
    /// );
    /// let visible: Vec<_> = plan.iter().map(|region| region.id).collect();
    ///
    /// assert_eq!(visible, ["first", "second"]);
    /// ```
    pub fn iter(&self) -> slice::Iter<'_, Region<Id>> {
        self.regions.iter()
    }

    /// Adds a region to the region set.
    ///
    /// Pushed regions are appended after existing regions and therefore win same-z hit-test ties
    /// over earlier regions.
    ///
    /// This is useful for incremental builders, overlays, or tests that want to start from
    /// [`Regions::new`] and add regions one at a time.
    ///
    /// # Examples
    ///
    /// Append a popup region after the base content so it wins same-z ties:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{Region, Regions};
    ///
    /// let mut plan = Regions::from_regions(
    ///     Rect::new(0, 0, 10, 2),
    ///     [Region::new("content", Rect::new(0, 0, 10, 2))],
    /// );
    /// plan.push(Region::new("popup", Rect::new(0, 0, 10, 2)));
    ///
    /// assert_eq!(plan.hit_test((1, 1)).unwrap().id, "popup");
    /// ```
    pub fn push(&mut self, region: Region<Id>) {
        self.regions.push(region);
    }

    /// Adds a region and returns the modified region set.
    ///
    /// Use this fluent form when building values inline, especially overlays or small test values.
    ///
    /// # Examples
    ///
    /// Build a compact previous-frame snapshot inline:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{Region, Regions};
    ///
    /// let plan =
    ///     Regions::new(Rect::new(0, 0, 8, 1)).region(Region::new("button", Rect::new(0, 0, 8, 1)));
    ///
    /// assert_eq!(plan.hit_test((2, 0)).unwrap().id, "button");
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn region(mut self, region: Region<Id>) -> Self {
        self.push(region);
        self
    }

    /// Extends the region set with regions from another iterator.
    ///
    /// Use this when a layout solver already has an iterator of regions and wants to append them
    /// without allocating an intermediate child region set.
    ///
    /// # Examples
    ///
    /// Append row regions produced by an iterator:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{Region, Regions};
    ///
    /// let mut plan = Regions::new(Rect::new(0, 0, 10, 2));
    /// plan.extend((0..2).map(|row| Region::new(row, Rect::new(0, row as u16, 10, 1))));
    ///
    /// assert_eq!(plan.regions().len(), 2);
    /// ```
    pub fn extend<I>(&mut self, regions: I)
    where
        I: IntoIterator<Item = Region<Id>>,
    {
        self.regions.extend(regions);
    }

    /// Returns a region set containing this set's regions followed by another set's regions.
    ///
    /// The merged region set keeps this set's parent area. Later regions preserve their normal
    /// hit-test tie-breaking behavior.
    ///
    /// Use this when a parent aggregates child component values. If the child was solved in local
    /// coordinates, call [`Regions::translate`] before merging.
    ///
    /// # Examples
    ///
    /// Merge footer regions with content regions:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{Region, Regions};
    ///
    /// let content = Regions::from_regions(
    ///     Rect::new(0, 0, 20, 3),
    ///     [Region::new("body", Rect::new(0, 0, 20, 2))],
    /// );
    /// let footer = Regions::from_regions(
    ///     Rect::new(0, 0, 20, 3),
    ///     [Region::new("status", Rect::new(0, 2, 20, 1))],
    /// );
    /// let frame = content.merge(footer);
    ///
    /// assert_eq!(frame.hit_test((1, 2)).unwrap().id, "status");
    /// ```
    #[must_use = "method returns the merged plan"]
    pub fn merge(mut self, other: Self) -> Self {
        self.extend(other.regions);
        self
    }

    /// Maps region ids while preserving areas, clipping, and z-order.
    ///
    /// This lets a child component solve a local region set with simple indexes and then lift those
    /// ids into an application enum or stable key before merging with a parent region set.
    ///
    /// Common examples include mapping row indexes to app record ids, mapping dialog field indexes
    /// to a `Field` enum, or wrapping child ids in a parent enum variant.
    ///
    /// # Examples
    ///
    /// Convert child-local integer ids into an enum used by the app event router:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{Region, Regions};
    ///
    /// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    /// enum AppRegion {
    ///     Field(usize),
    /// }
    ///
    /// let child = Regions::from_regions(
    ///     Rect::new(0, 0, 10, 1),
    ///     [Region::new(0, Rect::new(0, 0, 10, 1))],
    /// );
    /// let parent = child.map_id(AppRegion::Field);
    ///
    /// assert_eq!(parent.hit_test((1, 0)).unwrap().id, AppRegion::Field(0));
    /// ```
    pub fn map_id<NextId, F>(self, mut map: F) -> Regions<NextId>
    where
        F: FnMut(Id) -> NextId,
    {
        let regions: Vec<_> = self
            .regions
            .into_iter()
            .map(|region| Region {
                id: map(region.id),
                area: region.area,
                clip: region.clip,
                z: region.z,
            })
            .collect();
        Regions::from_regions(self.area, regions)
    }

    /// Moves the parent area and all regions by a signed offset.
    ///
    /// Use this when composing child regions that were solved relative to a local origin.
    ///
    /// A component can solve itself against `Rect::new(0, 0, width, height)`, then the parent can
    /// translate the result into the actual terminal area where the component was drawn.
    ///
    /// # Examples
    ///
    /// Place child-local regions inside a parent panel:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{Region, Regions};
    ///
    /// let child = Regions::from_regions(
    ///     Rect::new(0, 0, 8, 1),
    ///     [Region::new("field", Rect::new(0, 0, 8, 1))],
    /// );
    /// let placed = child.translate(10, 4);
    ///
    /// assert_eq!(placed.area(), Rect::new(10, 4, 8, 1));
    /// assert_eq!(placed.regions()[0].area, Rect::new(10, 4, 8, 1));
    /// ```
    #[must_use = "method returns the translated plan"]
    pub fn translate(mut self, dx: i16, dy: i16) -> Self {
        self.area = translate_rect(self.area, dx, dy);
        self.regions = self
            .regions
            .into_iter()
            .map(|region| region.translate(dx, dy))
            .collect();
        self
    }

    /// Clips all regions to the viewport area and drops regions outside it.
    ///
    /// The parent area is changed to the viewport so downstream code can treat the returned value
    /// as visible child regions.
    ///
    /// Use this for scroll views, containers with inner clipping, and popups that should hide child
    /// targets outside their visible boundary.
    ///
    /// # Examples
    ///
    /// Clip hidden rows before storing previous-frame hit-test regions:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{Region, Regions};
    ///
    /// let child = Regions::from_regions(
    ///     Rect::new(0, 0, 10, 5),
    ///     [
    ///         Region::new("visible", Rect::new(0, 1, 10, 1)),
    ///         Region::new("hidden", Rect::new(0, 4, 10, 1)),
    ///     ],
    /// );
    /// let visible = child.clip_to(Rect::new(0, 0, 10, 3));
    ///
    /// assert_eq!(visible.regions().len(), 1);
    /// assert_eq!(visible.regions()[0].id, "visible");
    /// ```
    #[must_use = "method returns the clipped plan"]
    pub fn clip_to(mut self, viewport: Rect) -> Self {
        self.area = viewport;
        self.regions = self
            .regions
            .into_iter()
            .filter_map(|region| region.clip_to(viewport))
            .collect();
        self
    }

    /// Returns true when the region set has no regions.
    ///
    /// Empty values are normal for empty collections, fully clipped children, and components that
    /// expose an area but no externally addressable regions.
    ///
    /// # Examples
    ///
    /// Check whether an empty collection exposed any visible rows:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::Regions;
    ///
    /// let plan = Regions::<usize>::new(Rect::new(0, 0, 20, 4));
    ///
    /// assert!(plan.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }
}

const fn translate_rect(rect: Rect, dx: i16, dy: i16) -> Rect {
    Rect::new(
        translate_coordinate(rect.x, dx),
        translate_coordinate(rect.y, dy),
        rect.width,
        rect.height,
    )
}

const fn translate_coordinate(value: u16, delta: i16) -> u16 {
    if delta.is_negative() {
        value.saturating_sub(delta.unsigned_abs())
    } else {
        value.saturating_add(delta as u16)
    }
}

fn intersect(a: Rect, b: Rect) -> Option<Rect> {
    let x = a.x.max(b.x);
    let y = a.y.max(b.y);
    let right = a.right().min(b.right());
    let bottom = a.bottom().min(b.bottom());

    (x < right && y < bottom).then(|| Rect::new(x, y, right - x, bottom - y))
}

impl<'a, Id> IntoIterator for &'a Regions<Id> {
    type Item = &'a Region<Id>;
    type IntoIter = slice::Iter<'a, Region<Id>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<Id: Copy> Regions<Id> {
    /// Returns the topmost region containing the position.
    ///
    /// Higher z-order wins. For matching z-order values, later regions win because they are usually
    /// rendered later.
    ///
    /// Use this when layout geometry is enough to route input. If pointer behavior has disabled
    /// state or pointer-only regions, build a [`crate::PointerTargets`] and route through that
    /// instead.
    ///
    /// # Examples
    ///
    /// Route a previous-frame click to the topmost visible region:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{Region, Regions};
    ///
    /// let plan = Regions::from_regions(
    ///     Rect::new(0, 0, 20, 2),
    ///     [
    ///         Region::new("body", Rect::new(0, 0, 20, 2)),
    ///         Region::new("popup", Rect::new(4, 0, 8, 2)).z(10),
    ///     ],
    /// );
    /// let hit = plan.hit_test((5, 1)).unwrap();
    ///
    /// assert_eq!(hit.id, "popup");
    /// assert_eq!((hit.relative_x, hit.relative_y), (1, 1));
    /// ```
    pub fn hit_test<P: Into<Position>>(&self, position: P) -> Option<Hit<Id>> {
        let position = position.into();
        let mut hit = None;
        for region in self
            .regions
            .iter()
            .filter(|region| region.area.contains(position))
        {
            if hit.is_none_or(|current: &Region<Id>| region.z >= current.z) {
                hit = Some(region);
            }
        }

        hit.map(|region| Hit {
            id: region.id,
            area: region.area,
            relative_x: position.x.saturating_sub(region.area.x),
            relative_y: position.y.saturating_sub(region.area.y),
        })
    }
}

impl<Id> IntoIterator for Regions<Id> {
    type Item = Region<Id>;
    type IntoIter = alloc::vec::IntoIter<Region<Id>>;

    fn into_iter(self) -> Self::IntoIter {
        self.regions.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use ratatui_core::layout::Rect;

    use super::*;

    #[test]
    fn hit_test_returns_relative_coordinates() {
        let plan = Regions::from_regions(
            Rect::new(0, 0, 20, 10),
            [Region::new("row", Rect::new(2, 3, 5, 4))],
        );

        assert_eq!(
            plan.hit_test((4, 5)),
            Some(Hit {
                id: "row",
                area: Rect::new(2, 3, 5, 4),
                relative_x: 2,
                relative_y: 2,
            })
        );
    }

    #[test]
    fn hit_test_prefers_higher_z_then_later_regions() {
        let plan = Regions::from_regions(
            Rect::new(0, 0, 20, 10),
            [
                Region::new("first", Rect::new(0, 0, 10, 10)).z(2),
                Region::new("second", Rect::new(0, 0, 10, 10)).z(1),
                Region::new("third", Rect::new(0, 0, 10, 10)).z(2),
            ],
        );

        assert_eq!(plan.hit_test((1, 1)).map(|hit| hit.id), Some("third"));
    }

    #[test]
    fn clips_regions_to_viewport() {
        let plan = Regions::from_regions(
            Rect::new(0, 0, 10, 10),
            [
                Region::new("visible", Rect::new(2, 2, 4, 4)),
                Region::new("clipped", Rect::new(8, 8, 4, 4)),
                Region::new("hidden", Rect::new(20, 20, 2, 2)),
            ],
        )
        .clip_to(Rect::new(0, 0, 10, 10));

        assert_eq!(plan.regions().len(), 2);
        assert_eq!(plan.regions()[1].area, Rect::new(8, 8, 2, 2));
        assert_eq!(plan.regions()[1].clip.right, 2);
        assert_eq!(plan.regions()[1].clip.bottom, 2);
    }

    #[test]
    fn maps_and_translates_child_plans() {
        let plan = Regions::from_regions(
            Rect::new(0, 0, 4, 1),
            [Region::new(0, Rect::new(1, 0, 2, 1))],
        )
        .map_id(|id| if id == 0 { "label" } else { "other" })
        .translate(4, 2);

        assert_eq!(plan.area(), Rect::new(4, 2, 4, 1));
        assert_eq!(plan.regions()[0].id, "label");
        assert_eq!(plan.regions()[0].area, Rect::new(5, 2, 2, 1));
    }
}
