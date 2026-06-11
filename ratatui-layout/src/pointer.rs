//! Pointer targets for backend-agnostic routing.
//!
//! Layout hit testing can answer "which rectangle contains this position?" but pointer handling
//! usually needs a more specific contract: some visible regions are disabled, some are not pointer
//! targets at all, and overlapping popups should route events according to z-order. This module
//! records those pointer-specific data without depending on crossterm, termion, or any other event
//! backend.
//!
//! [`PointerTargets`](crate::pointer::PointerTargets) is the pointer counterpart to
//! [`crate::regions::Regions`]. It records the visible regions that can receive pointer events
//! during the current frame, while persistent pressed and hover state stays in
//! [`PointerState`](crate::pointer::PointerState). Use it when a previous render pass should
//! route the next pointer event back to app-owned data.
//!
//! The module separates the pointer workflow into three levels:
//!
//! - [`PointerTarget`](crate::pointer::PointerTarget) names one rectangular region that can receive
//!   pointer input. It adds pointer-specific data, such as disabled state and z-order, that a plain
//!   [`crate::regions::Region`] does not carry.
//! - [`PointerTargets`](crate::pointer::PointerTargets) is rebuilt during rendering from the
//!   targets that are actually visible. It can merge child values, translate child-local
//!   coordinates into parent coordinates, clip targets to a viewport, and hit test terminal
//!   positions.
//! - [`PointerState`](crate::pointer::PointerState) is application state that survives between
//!   events. It remembers hover and press state so an app can distinguish a click from a press that
//!   was released somewhere else.
//!
//! This is enough for backend-agnostic routing without making the target set know about crossterm,
//! termion, mouse buttons, or callbacks. The backend converts its event position into
//! [`ratatui_core::layout::Position`], routes through the previous frame's targets, and then the
//! app decides what action the hit id means.
//!
//! # Types
//!
//! - [`PointerTarget`](crate::pointer::PointerTarget) describes one pointer-sensitive rectangle.
//! - [`PointerTargets`](crate::pointer::PointerTargets) stores the visible targets from one frame
//!   and performs hit testing.
//! - [`PointerState`](crate::pointer::PointerState) stores hover and pressed ids across input
//!   events.
//! - [`PointerPhase`](crate::pointer::PointerPhase) describes backend-agnostic hover, press, and
//!   release phases.
//!
//! Normal Ratatui widgets remain the better tool when pointer input is not needed, or when the
//! input can be handled by one known widget without hit testing a set of visible targets.
//!
//! See [`crate::docs::interaction`] for how pointer targets, focus targets, selection, and cursor
//! requests stay separate while sharing ids.
//!
//! # Examples
//!
//! Disabled targets remain in the target set but do not win hit testing:
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::pointer::{PointerTarget, PointerTargets};
//!
//! let targets = PointerTargets::from_targets([
//!     PointerTarget::new("content", Rect::new(0, 0, 10, 1)),
//!     PointerTarget::new("disabled-popup", Rect::new(0, 0, 10, 1))
//!         .z(10)
//!         .disabled(true),
//! ]);
//!
//! assert_eq!(targets.hit_test((1, 0)).unwrap().id, "content");
//! ```
//!
//! Press/release matching helps distinguish a click from a press that moved away:
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::pointer::{PointerState, PointerTarget, PointerTargets};
//!
//! let targets = PointerTargets::from_targets([
//!     PointerTarget::new("left", Rect::new(0, 0, 5, 1)),
//!     PointerTarget::new("right", Rect::new(5, 0, 5, 1)),
//! ]);
//! let mut mouse = PointerState::default();
//!
//! mouse.press(&targets, (1, 0));
//! assert!(mouse.release(&targets, (7, 0)).is_none());
//! ```

use alloc::vec::Vec;
use core::slice;

use ratatui_core::layout::{Position, Rect};

use crate::regions::{Hit, Region};

/// A rectangular pointer target for app-owned content.
///
/// [`PointerTarget`](crate::pointer::PointerTarget) exists so pointer routing can be described
/// without depending on a terminal backend's event type. It owns only frame-local data: the app id,
/// area, disabled state, and z-order. It does not store callbacks, widgets, or persistent
/// hover/pressed state; that belongs in [`PointerState`](crate::pointer::PointerState).
///
/// Use a [`PointerTarget`](crate::pointer::PointerTarget) instead of relying only on
/// [`crate::regions::Region`] when a visible region has pointer-specific behavior: disabled state,
/// different z-order, or an area that should accept pointer events even though it is not a render
/// region.
///
/// # Constructors and setters
///
/// - [`PointerTarget::new`](crate::pointer::PointerTarget::new) creates an enabled target at
///   z-order zero.
/// - [`PointerTarget::from_region`](crate::pointer::PointerTarget::from_region) converts a
///   [`Region`](crate::regions::Region) when layout regions and pointer targets match.
/// - [`PointerTarget::z`](crate::pointer::PointerTarget::z) changes hit-test priority for overlays
///   or floating controls.
/// - [`PointerTarget::disabled`](crate::pointer::PointerTarget::disabled) keeps a target visible
///   while skipping real routing.
///
/// # Geometry helpers
///
/// - [`PointerTarget::contains`](crate::pointer::PointerTarget::contains) checks whether a terminal
///   position falls inside the target.
/// - [`PointerTarget::local_position`](crate::pointer::PointerTarget::local_position) converts
///   terminal coordinates into target-local coordinates.
/// - [`PointerTarget::translate`](crate::pointer::PointerTarget::translate) places a child-local
///   target in parent coordinates.
/// - [`PointerTarget::clip_to`](crate::pointer::PointerTarget::clip_to) trims a target to a
///   viewport and drops it when fully hidden.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::pointer::PointerTarget;
///
/// let target = PointerTarget::new("save", Rect::new(10, 2, 8, 1)).z(2);
///
/// assert!(target.contains((11, 2)));
/// assert_eq!(target.local_position((11, 2)).unwrap().x, 1);
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PointerTarget<Id = usize> {
    /// Application-owned target id.
    ///
    /// The id should be something the app can route back to its own state. Use integers for
    /// generated indexed regions, string names for examples or diagnostics, enums for normal app
    /// controls, and stable record keys for reorderable rows or cells.
    pub id: Id,
    /// Current frame area for the pointer target.
    ///
    /// Hit testing uses terminal coordinates against this rectangle.
    pub area: Rect,
    /// Z ordering used for hit testing.
    ///
    /// Higher values win. Equal z values fall back to insertion order, where later targets win.
    pub z: u16,
    /// Whether pointer routing should skip this target.
    ///
    /// Disabled targets remain in the target set for diagnostics and stable structure, but
    /// [`PointerTargets::hit_test`](crate::pointer::PointerTargets::hit_test) ignores them.
    pub disabled: bool,
}

impl<Id> PointerTarget<Id> {
    /// Creates an enabled pointer target with z-order zero.
    ///
    /// This is the normal constructor when render order alone is enough for hit-test tie breaking.
    ///
    /// # Examples
    ///
    /// Build a target for a rendered button and route a click through a target set:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
    ///
    /// let plan =
    ///     PointerTargets::new().target(PointerTarget::new("save-button", Rect::new(10, 2, 6, 1)));
    ///
    /// assert_eq!(plan.hit_test((12, 2)).unwrap().id, "save-button");
    /// ```
    pub const fn new(id: Id, area: Rect) -> Self {
        Self {
            id,
            area,
            z: 0,
            disabled: false,
        }
    }

    /// Sets the target z-order.
    ///
    /// Use this for overlays, popups, and floating controls that should receive pointer events
    /// before lower visual layers.
    ///
    /// # Examples
    ///
    /// Let a popup route clicks before the content underneath it:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
    ///
    /// let plan = PointerTargets::from_targets([
    ///     PointerTarget::new("editor", Rect::new(0, 0, 20, 5)),
    ///     PointerTarget::new("palette", Rect::new(4, 1, 10, 3)).z(10),
    /// ]);
    ///
    /// assert_eq!(plan.hit_test((5, 2)).unwrap().id, "palette");
    /// ```
    #[must_use = "method returns the modified value"]
    pub const fn z(mut self, z: u16) -> Self {
        self.z = z;
        self
    }

    /// Sets whether the target is disabled.
    ///
    /// Disabled targets are useful when a control should remain visible but should not react to
    /// hover, press, or release.
    ///
    /// # Examples
    ///
    /// Keep a disabled command visible while allowing the enabled target below to receive input:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
    ///
    /// let plan = PointerTargets::from_targets([
    ///     PointerTarget::new("fallback", Rect::new(0, 0, 10, 1)),
    ///     PointerTarget::new("disabled-save", Rect::new(0, 0, 10, 1))
    ///         .z(5)
    ///         .disabled(true),
    /// ]);
    ///
    /// assert_eq!(plan.hit_test((1, 0)).unwrap().id, "fallback");
    /// ```
    #[must_use = "method returns the modified value"]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Creates a pointer target from a layout region.
    ///
    /// This is the quickest path when every rendered region is also interactive. The target copies
    /// the region id, area, and z-order, but starts enabled because [`crate::regions::Region`] has
    /// no pointer disabled state.
    ///
    /// # Examples
    ///
    /// Derive pointer targets from a region set when every region behaves like a button:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
    /// use ratatui_layout::regions::{Region, Regions};
    ///
    /// let layout = Regions::from_regions(
    ///     Rect::new(0, 0, 12, 1),
    ///     [
    ///         Region::new("open", Rect::new(0, 0, 6, 1)),
    ///         Region::new("save", Rect::new(6, 0, 6, 1)),
    ///     ],
    /// );
    /// let plan = PointerTargets::from_targets(
    ///     layout
    ///         .regions()
    ///         .iter()
    ///         .copied()
    ///         .map(PointerTarget::from_region)
    ///         .collect::<Vec<_>>(),
    /// );
    ///
    /// assert_eq!(plan.hit_test((7, 0)).unwrap().id, "save");
    /// ```
    pub fn from_region(region: Region<Id>) -> Self {
        Self {
            id: region.id,
            area: region.area,
            z: region.z,
            disabled: false,
        }
    }

    /// Returns true when the target area contains the position.
    ///
    /// This checks geometry only. It does not consider
    /// [`PointerTarget::disabled`](crate::pointer::PointerTarget::disabled); use
    /// [`PointerTargets::hit_test`](crate::pointer::PointerTargets::hit_test) when routing real
    /// events.
    ///
    /// # Examples
    ///
    /// Use geometry checks for diagnostics or pointer previews before full routing:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::PointerTarget;
    ///
    /// let tab = PointerTarget::new("settings", Rect::new(8, 0, 10, 1)).disabled(true);
    ///
    /// assert!(tab.contains((9, 0)));
    /// assert!(tab.disabled);
    /// ```
    pub fn contains<P: Into<Position>>(&self, position: P) -> bool {
        self.area.contains(position.into())
    }

    /// Returns the position relative to the target area.
    ///
    /// Local coordinates let a row, cell, or control interpret the event without re-subtracting its
    /// terminal origin.
    ///
    /// # Examples
    ///
    /// Convert a table-row hit into a column offset inside that row:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Position, Rect};
    /// use ratatui_layout::pointer::PointerTarget;
    ///
    /// let row = PointerTarget::new("row-42", Rect::new(5, 10, 30, 1));
    ///
    /// assert_eq!(row.local_position((12, 10)), Some(Position::new(7, 0)));
    /// assert_eq!(row.local_position((12, 11)), None);
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

    /// Moves the target area by a signed offset.
    ///
    /// This supports component composition where a child produced local coordinates and a parent
    /// later places that child in the terminal.
    ///
    /// # Examples
    ///
    /// Move a child component's local button target into its parent panel:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::PointerTarget;
    ///
    /// let local_button = PointerTarget::new("ok", Rect::new(0, 0, 4, 1));
    /// let placed = local_button.translate(10, 3);
    ///
    /// assert_eq!(placed.area, Rect::new(10, 3, 4, 1));
    /// ```
    #[must_use = "method returns the translated target"]
    pub const fn translate(mut self, dx: i16, dy: i16) -> Self {
        self.area = translate_rect(self.area, dx, dy);
        self
    }

    /// Clips the target to a visible viewport.
    ///
    /// Returns `None` when the target is entirely outside the viewport.
    ///
    /// # Examples
    ///
    /// Clip an oversized row target to the visible viewport before hit testing:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::PointerTarget;
    ///
    /// let row = PointerTarget::new("row", Rect::new(0, 2, 20, 1));
    /// let visible = row.clip_to(Rect::new(4, 0, 8, 4)).unwrap();
    ///
    /// assert_eq!(visible.area, Rect::new(4, 2, 8, 1));
    /// assert!(row.clip_to(Rect::new(0, 4, 8, 2)).is_none());
    /// ```
    pub fn clip_to(mut self, viewport: Rect) -> Option<Self> {
        self.area = intersect(self.area, viewport)?;
        Some(self)
    }
}

/// Pointer targets produced by one render pass.
///
/// [`PointerTargets`](crate::pointer::PointerTargets) owns no application state and has no
/// backend-specific event knowledge. Store it after rendering, then route the next event by
/// converting its coordinates to a [`ratatui_core::layout::Position`].
///
/// A target set is rebuilt every frame from visible targets. The app should keep long-lived
/// interaction data, such as the target currently pressed, in
/// [`PointerState`](crate::pointer::PointerState).
///
/// # Constructors and builders
///
/// - [`PointerTargets::new`](crate::pointer::PointerTargets::new) creates an empty target set for
///   incremental construction.
/// - [`PointerTargets::from_targets`](crate::pointer::PointerTargets::from_targets) creates a
///   target set from targets already collected during rendering.
/// - [`PointerTargets::target`](crate::pointer::PointerTargets::target) appends one target in
///   builder style.
/// - [`PointerTargets::region`](crate::pointer::PointerTargets::region) appends a whole-region
///   target, which is useful for scrollable pane backgrounds and blank viewport space.
/// - [`PointerTargets::extend`](crate::pointer::PointerTargets::extend) appends many targets to an
///   existing target set.
/// - [`PointerTargets::merge`](crate::pointer::PointerTargets::merge) combines child values while
///   preserving ordering.
///
/// # Inspection and routing
///
/// - [`PointerTargets::targets`](crate::pointer::PointerTargets::targets) and
///   [`PointerTargets::iter`](crate::pointer::PointerTargets::iter) expose the visible targets for
///   diagnostics or derived state.
/// - [`PointerTargets::hit_test`](crate::pointer::PointerTargets::hit_test) routes a terminal
///   position to the topmost enabled target and returns a [`crate::regions::Hit`] with local
///   coordinates.
///
/// # Composition
///
/// - [`PointerTargets::map_id`](crate::pointer::PointerTargets::map_id) converts child ids into
///   parent app ids.
/// - [`PointerTargets::translate`](crate::pointer::PointerTargets::translate) places child-local
///   targets in parent coordinates.
/// - [`PointerTargets::clip_to`](crate::pointer::PointerTargets::clip_to) removes hidden target
///   regions before storing targets for input.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
///
/// let local = PointerTargets::from_targets([PointerTarget::new(0, Rect::new(0, 0, 10, 1))]);
/// let parent = local.translate(5, 2).map_id(|id| ("toolbar", id));
///
/// assert_eq!(parent.hit_test((6, 2)).unwrap().id, ("toolbar", 0));
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PointerTargets<Id = usize> {
    targets: Vec<PointerTarget<Id>>,
}

impl<Id> Default for PointerTargets<Id> {
    fn default() -> Self {
        Self {
            targets: Vec::new(),
        }
    }
}

impl<Id> PointerTargets<Id> {
    /// Creates an empty pointer target collection.
    ///
    /// Use this before the first frame or for components with no pointer-interactive regions.
    ///
    /// # Examples
    ///
    /// Build toolbar targets as each command is rendered:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
    ///
    /// let plan = PointerTargets::new()
    ///     .target(PointerTarget::new("open", Rect::new(0, 0, 6, 1)))
    ///     .target(PointerTarget::new("save", Rect::new(6, 0, 6, 1)));
    ///
    /// assert_eq!(plan.hit_test((7, 0)).unwrap().id, "save");
    /// ```
    pub const fn new() -> Self {
        Self {
            targets: Vec::new(),
        }
    }

    /// Creates a pointer target collection from targets.
    ///
    /// Targets keep their input order. That order matters when two enabled targets have the same
    /// z-order and both contain a position.
    ///
    /// # Examples
    ///
    /// Build a target set from the targets returned by a component render function:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
    ///
    /// let rendered_targets = vec![
    ///     PointerTarget::new("row-1", Rect::new(0, 0, 20, 1)),
    ///     PointerTarget::new("row-2", Rect::new(0, 1, 20, 1)),
    /// ];
    /// let plan = PointerTargets::from_targets(rendered_targets);
    ///
    /// assert_eq!(plan.targets().len(), 2);
    /// ```
    pub fn from_targets(targets: impl Into<Vec<PointerTarget<Id>>>) -> Self {
        Self {
            targets: targets.into(),
        }
    }

    /// Adds a target and returns the modified target set.
    ///
    /// This builder-style method appends in render order.
    ///
    /// # Examples
    ///
    /// Add a row target only after the row is known to be visible:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
    ///
    /// let row_area = Rect::new(0, 3, 40, 1);
    /// let plan = PointerTargets::new().target(PointerTarget::new(3, row_area));
    ///
    /// assert_eq!(plan.hit_test((2, 3)).unwrap().id, 3);
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn target(mut self, target: PointerTarget<Id>) -> Self {
        self.targets.push(target);
        self
    }

    /// Adds an enabled target for a whole region and returns the modified plan.
    ///
    /// Use this when the pointer-sensitive area is the region itself rather than a specific region.
    /// The most common case is mouse-wheel routing over blank space in a scrollable pane: visible
    /// rows may not fill the whole pane, but the pane should still receive scroll input anywhere
    /// inside its viewport.
    ///
    /// # Examples
    ///
    /// Route wheel input over a list viewport even when the pointer is below the last visible row:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::PointerTargets;
    ///
    /// let plan = PointerTargets::new().region("queue-pane", Rect::new(0, 0, 30, 10));
    ///
    /// assert_eq!(plan.hit_test((2, 8)).unwrap().id, "queue-pane");
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn region(self, id: Id, area: Rect) -> Self {
        self.target(PointerTarget::new(id, area))
    }

    /// Returns all pointer targets in render order.
    ///
    /// # Examples
    ///
    /// Inspect visible targets when deriving hoverable ids for a status view:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
    ///
    /// let plan = PointerTargets::from_targets([
    ///     PointerTarget::new("open", Rect::new(0, 0, 4, 1)),
    ///     PointerTarget::new("save", Rect::new(5, 0, 4, 1)),
    /// ]);
    /// let ids: Vec<_> = plan.targets().iter().map(|target| target.id).collect();
    ///
    /// assert_eq!(ids, ["open", "save"]);
    /// ```
    pub fn targets(&self) -> &[PointerTarget<Id>] {
        &self.targets
    }

    /// Returns an iterator over targets.
    ///
    /// # Examples
    ///
    /// Count enabled targets before deciding whether to enable mouse capture:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
    ///
    /// let plan = PointerTargets::from_targets([
    ///     PointerTarget::new("enabled", Rect::new(0, 0, 8, 1)),
    ///     PointerTarget::new("disabled", Rect::new(0, 1, 8, 1)).disabled(true),
    /// ]);
    ///
    /// assert_eq!(plan.iter().filter(|target| !target.disabled).count(), 1);
    /// ```
    pub fn iter(&self) -> slice::Iter<'_, PointerTarget<Id>> {
        self.targets.iter()
    }

    /// Extends the target set with additional targets.
    ///
    /// Appended targets win same-z hit-test ties over earlier targets.
    ///
    /// # Examples
    ///
    /// Append a modal target after base content so it wins same-z overlap by render order:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
    ///
    /// let mut plan =
    ///     PointerTargets::from_targets([PointerTarget::new("content", Rect::new(0, 0, 10, 3))]);
    /// plan.extend([PointerTarget::new("modal", Rect::new(2, 1, 6, 1))]);
    ///
    /// assert_eq!(plan.hit_test((3, 1)).unwrap().id, "modal");
    /// ```
    pub fn extend<I>(&mut self, targets: I)
    where
        I: IntoIterator<Item = PointerTarget<Id>>,
    {
        self.targets.extend(targets);
    }

    /// Returns a target set containing this set's targets followed by another set's targets.
    ///
    /// # Examples
    ///
    /// Merge child component targets into the parent frame snapshot:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
    ///
    /// let list =
    ///     PointerTargets::from_targets([PointerTarget::new("list-row", Rect::new(0, 0, 20, 1))]);
    /// let footer = PointerTargets::from_targets([PointerTarget::new("help", Rect::new(0, 3, 20, 1))]);
    ///
    /// let plan = list.merge(footer);
    /// assert_eq!(plan.hit_test((1, 3)).unwrap().id, "help");
    /// ```
    #[must_use = "method returns the merged target set"]
    pub fn merge(mut self, other: Self) -> Self {
        self.extend(other.targets);
        self
    }

    /// Maps target ids while preserving areas, z-order, and disabled state.
    ///
    /// Use this when a child component exposes local ids and the parent needs to route events using
    /// a larger application id type.
    ///
    /// # Examples
    ///
    /// Wrap child ids in an app-level enum before storing the plan:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
    ///
    /// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    /// enum Region {
    ///     Palette(usize),
    /// }
    ///
    /// let child = PointerTargets::from_targets([PointerTarget::new(0, Rect::new(0, 0, 5, 1))]);
    /// let plan = child.map_id(Region::Palette);
    ///
    /// assert_eq!(plan.hit_test((1, 0)).unwrap().id, Region::Palette(0));
    /// ```
    pub fn map_id<NextId, F>(self, mut map: F) -> PointerTargets<NextId>
    where
        F: FnMut(Id) -> NextId,
    {
        PointerTargets::from_targets(
            self.targets
                .into_iter()
                .map(|target| PointerTarget {
                    id: map(target.id),
                    area: target.area,
                    z: target.z,
                    disabled: target.disabled,
                })
                .collect::<Vec<_>>(),
        )
    }

    /// Moves all target areas by a signed offset.
    ///
    /// # Examples
    ///
    /// Place child targets solved in local coordinates into a parent area:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
    ///
    /// let child = PointerTargets::from_targets([PointerTarget::new("item", Rect::new(0, 0, 8, 1))]);
    /// let placed = child.translate(10, 4);
    ///
    /// assert_eq!(placed.hit_test((11, 4)).unwrap().id, "item");
    /// ```
    #[must_use = "method returns the translated target set"]
    pub fn translate(mut self, dx: i16, dy: i16) -> Self {
        self.targets = self
            .targets
            .into_iter()
            .map(|target| target.translate(dx, dy))
            .collect();
        self
    }

    /// Clips all target areas to a viewport and drops targets outside it.
    ///
    /// This keeps hidden child controls from receiving pointer events after a parent applies
    /// viewport or container clipping.
    ///
    /// # Examples
    ///
    /// Drop targets outside a scroll viewport before saving the previous-frame snapshot:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
    ///
    /// let plan = PointerTargets::from_targets([
    ///     PointerTarget::new("visible", Rect::new(0, 1, 10, 1)),
    ///     PointerTarget::new("hidden", Rect::new(0, 5, 10, 1)),
    /// ])
    /// .clip_to(Rect::new(0, 0, 10, 3));
    ///
    /// assert_eq!(plan.targets().len(), 1);
    /// assert_eq!(plan.hit_test((1, 1)).unwrap().id, "visible");
    /// ```
    #[must_use = "method returns the clipped target set"]
    pub fn clip_to(mut self, viewport: Rect) -> Self {
        self.targets = self
            .targets
            .into_iter()
            .filter_map(|target| target.clip_to(viewport))
            .collect();
        self
    }
}

impl<Id: Copy> PointerTargets<Id> {
    /// Returns the topmost enabled target containing the position.
    ///
    /// Higher z-order wins. For matching z-order values, later targets win because they are
    /// usually rendered later.
    ///
    /// # Examples
    ///
    /// Route a backend mouse event position to an app command and local x/y coordinates:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerTarget, PointerTargets};
    ///
    /// let plan = PointerTargets::from_targets([PointerTarget::new(
    ///     "status-command",
    ///     Rect::new(20, 10, 12, 1),
    /// )]);
    ///
    /// let hit = plan.hit_test((24, 10)).unwrap();
    /// assert_eq!(hit.id, "status-command");
    /// assert_eq!((hit.relative_x, hit.relative_y), (4, 0));
    /// ```
    pub fn hit_test<P: Into<Position>>(&self, position: P) -> Option<Hit<Id>> {
        let position = position.into();
        let mut hit = None;
        for target in self
            .targets
            .iter()
            .filter(|target| !target.disabled && target.area.contains(position))
        {
            if hit.is_none_or(|current: &PointerTarget<Id>| target.z >= current.z) {
                hit = Some(target);
            }
        }

        hit.map(|target| Hit {
            id: target.id,
            area: target.area,
            relative_x: position.x.saturating_sub(target.area.x),
            relative_y: position.y.saturating_sub(target.area.y),
        })
    }
}

impl<'a, Id> IntoIterator for &'a PointerTargets<Id> {
    type Item = &'a PointerTarget<Id>;
    type IntoIter = slice::Iter<'a, PointerTarget<Id>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Backend-agnostic pointer event phase.
///
/// [`PointerPhase`](crate::pointer::PointerPhase) is the small piece of event information that
/// [`PointerState::route`](crate::pointer::PointerState::route) needs to update hover and press
/// state. Terminal backends still own their full event types, button data, modifiers, drag policy,
/// and scroll events. Convert only the phases that should use ordinary hover/press/release routing
/// into this enum.
///
/// # Variants
///
/// - [`PointerPhase::Hover`](crate::pointer::PointerPhase::Hover) updates the hovered target.
/// - [`PointerPhase::Press`](crate::pointer::PointerPhase::Press) records the target where a click
///   or drag began.
/// - [`PointerPhase::Release`](crate::pointer::PointerPhase::Release) clears the pressed target and
///   returns a hit only when release lands on the same target.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::pointer::{PointerPhase, PointerState, PointerTarget, PointerTargets};
///
/// let targets =
///     PointerTargets::from_targets([PointerTarget::new("button", Rect::new(0, 0, 8, 1))]);
/// let mut mouse = PointerState::default();
///
/// mouse.route(&targets, (2, 0), PointerPhase::Press);
/// assert_eq!(
///     mouse
///         .route(&targets, (2, 0), PointerPhase::Release)
///         .unwrap()
///         .id,
///     "button"
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PointerPhase {
    /// Pointer moved over the terminal without starting or completing an activation.
    #[default]
    Hover,
    /// Pointer button was pressed.
    Press,
    /// Pointer button was released.
    Release,
}

/// Persistent pointer state for an app or component.
///
/// [`PointerState`](crate::pointer::PointerState) stores event-to-event data: the currently hovered
/// target and the target that was pressed. It does not own target geometry; pair it with the
/// current or previous [`PointerTargets`](crate::pointer::PointerTargets) to update that data from
/// terminal coordinates.
///
/// The state is intentionally small because different backends report different event details. Apps
/// can keep backend-specific button, modifier, or drag information next to this state when needed.
///
/// # Accessors and updates
///
/// - [`PointerState::hovered`](crate::pointer::PointerState::hovered) returns the id most recently
///   hit by hover, press, or release.
/// - [`PointerState::pressed`](crate::pointer::PointerState::pressed) returns the id where the
///   current press began.
/// - [`PointerState::clear`](crate::pointer::PointerState::clear) resets state when a view closes
///   or the pointer leaves the terminal.
/// - [`PointerState::hover`](crate::pointer::PointerState::hover) updates hover from a
///   [`PointerTargets`](crate::pointer::PointerTargets) and terminal position.
/// - [`PointerState::press`](crate::pointer::PointerState::press) records where a click or drag
///   began.
/// - [`PointerState::release`](crate::pointer::PointerState::release) clears the press and returns
///   a hit only when press and release match.
/// - [`PointerState::route`](crate::pointer::PointerState::route) applies a backend-agnostic
///   [`PointerPhase`](crate::pointer::PointerPhase) to the state.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::pointer::{PointerState, PointerTarget, PointerTargets};
///
/// let targets =
///     PointerTargets::from_targets([PointerTarget::new("button", Rect::new(0, 0, 6, 1))]);
/// let mut state = PointerState::default();
///
/// state.hover(&targets, (2, 0));
/// assert_eq!(state.hovered(), Some("button"));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PointerState<Id = usize> {
    hovered: Option<Id>,
    pressed: Option<Id>,
}

impl<Id> Default for PointerState<Id> {
    fn default() -> Self {
        Self {
            hovered: None,
            pressed: None,
        }
    }
}

impl<Id> PointerState<Id> {
    /// Returns the currently hovered target id.
    ///
    /// This is updated by [`PointerState::hover`](crate::pointer::PointerState::hover),
    /// [`PointerState::press`](crate::pointer::PointerState::press), and
    /// [`PointerState::release`](crate::pointer::PointerState::release).
    ///
    /// # Examples
    ///
    /// Drive hover styling from the previous frame's pointer target collection:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerState, PointerTarget, PointerTargets};
    ///
    /// let plan = PointerTargets::from_targets([PointerTarget::new("delete", Rect::new(0, 0, 8, 1))]);
    /// let mut mouse = PointerState::default();
    ///
    /// mouse.hover(&plan, (2, 0));
    /// assert_eq!(mouse.hovered(), Some("delete"));
    /// ```
    pub const fn hovered(&self) -> Option<Id>
    where
        Id: Copy,
    {
        self.hovered
    }

    /// Returns the target id that was pressed.
    ///
    /// A later release only counts as activation when it lands on the same target.
    ///
    /// # Examples
    ///
    /// Remember which target began a click while the pointer is still down:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerState, PointerTarget, PointerTargets};
    ///
    /// let plan =
    ///     PointerTargets::from_targets([PointerTarget::new("drag-handle", Rect::new(0, 0, 4, 1))]);
    /// let mut mouse = PointerState::default();
    ///
    /// mouse.press(&plan, (1, 0));
    /// assert_eq!(mouse.pressed(), Some("drag-handle"));
    /// ```
    pub const fn pressed(&self) -> Option<Id>
    where
        Id: Copy,
    {
        self.pressed
    }

    /// Clears hover and pressed state.
    ///
    /// Use this when a view is closed, the pointer leaves the terminal, or the previous frame is no
    /// longer relevant.
    ///
    /// # Examples
    ///
    /// Clear stale state when closing a popup that owned the pressed target:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerState, PointerTarget, PointerTargets};
    ///
    /// let plan =
    ///     PointerTargets::from_targets([PointerTarget::new("popup-ok", Rect::new(0, 0, 8, 1))]);
    /// let mut mouse = PointerState::default();
    ///
    /// mouse.press(&plan, (1, 0));
    /// mouse.clear();
    ///
    /// assert_eq!(mouse.hovered(), None);
    /// assert_eq!(mouse.pressed(), None);
    /// ```
    pub fn clear(&mut self) {
        self.hovered = None;
        self.pressed = None;
    }
}

impl<Id: Copy + Eq> PointerState<Id> {
    /// Routes a backend-agnostic pointer phase through a pointer target collection.
    ///
    /// Use this after converting a terminal backend event into
    /// [`PointerPhase`](crate::pointer::PointerPhase) plus terminal coordinates. The method
    /// centralizes the ordinary hover, press, and release state transitions while leaving
    /// scroll, drag, buttons, and modifiers to the application.
    ///
    /// # Examples
    ///
    /// Convert a press/release pair into an activation only when both phases hit the same target:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerPhase, PointerState, PointerTarget, PointerTargets};
    ///
    /// let plan = PointerTargets::from_targets([
    ///     PointerTarget::new("open", Rect::new(0, 0, 6, 1)),
    ///     PointerTarget::new("save", Rect::new(6, 0, 6, 1)),
    /// ]);
    /// let mut mouse = PointerState::default();
    ///
    /// mouse.route(&plan, (1, 0), PointerPhase::Press);
    /// assert!(mouse.route(&plan, (7, 0), PointerPhase::Release).is_none());
    ///
    /// mouse.route(&plan, (7, 0), PointerPhase::Press);
    /// assert_eq!(
    ///     mouse
    ///         .route(&plan, (7, 0), PointerPhase::Release)
    ///         .unwrap()
    ///         .id,
    ///     "save"
    /// );
    /// ```
    pub fn route<P: Into<Position>>(
        &mut self,
        plan: &PointerTargets<Id>,
        position: P,
        phase: PointerPhase,
    ) -> Option<Hit<Id>> {
        match phase {
            PointerPhase::Hover => self.hover(plan, position),
            PointerPhase::Press => self.press(plan, position),
            PointerPhase::Release => self.release(plan, position),
        }
    }

    /// Updates hover state from a position and returns the hit target.
    ///
    /// # Examples
    ///
    /// Update app hover state from a mouse-move event:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerState, PointerTarget, PointerTargets};
    ///
    /// let plan = PointerTargets::from_targets([
    ///     PointerTarget::new("row-0", Rect::new(0, 0, 20, 1)),
    ///     PointerTarget::new("row-1", Rect::new(0, 1, 20, 1)),
    /// ]);
    /// let mut mouse = PointerState::default();
    ///
    /// let hit = mouse.hover(&plan, (3, 1)).unwrap();
    ///
    /// assert_eq!(hit.id, "row-1");
    /// assert_eq!(mouse.hovered(), Some("row-1"));
    /// ```
    pub fn hover<P: Into<Position>>(
        &mut self,
        plan: &PointerTargets<Id>,
        position: P,
    ) -> Option<Hit<Id>> {
        let hit = plan.hit_test(position);
        self.hovered = hit.map(|hit| hit.id);
        hit
    }

    /// Records a press from a position and returns the hit target.
    ///
    /// The pressed id is stored so [`PointerState::release`](crate::pointer::PointerState::release)
    /// can distinguish a click from a press that moved to another target before release.
    ///
    /// # Examples
    ///
    /// Start a click only when the press begins on an enabled target:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerState, PointerTarget, PointerTargets};
    ///
    /// let plan = PointerTargets::from_targets([PointerTarget::new("save", Rect::new(0, 0, 6, 1))]);
    /// let mut mouse = PointerState::default();
    ///
    /// assert_eq!(mouse.press(&plan, (2, 0)).unwrap().id, "save");
    /// assert_eq!(mouse.pressed(), Some("save"));
    /// ```
    pub fn press<P: Into<Position>>(
        &mut self,
        plan: &PointerTargets<Id>,
        position: P,
    ) -> Option<Hit<Id>> {
        let hit = self.hover(plan, position);
        self.pressed = hit.map(|hit| hit.id);
        hit
    }

    /// Releases the current press and returns the released target when press and release match.
    ///
    /// # Examples
    ///
    /// Treat a press/release on the same target as activation, and a moved release as cancellation:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::pointer::{PointerState, PointerTarget, PointerTargets};
    ///
    /// let plan = PointerTargets::from_targets([
    ///     PointerTarget::new("open", Rect::new(0, 0, 6, 1)),
    ///     PointerTarget::new("save", Rect::new(6, 0, 6, 1)),
    /// ]);
    /// let mut mouse = PointerState::default();
    ///
    /// mouse.press(&plan, (1, 0));
    /// assert!(mouse.release(&plan, (7, 0)).is_none());
    ///
    /// mouse.press(&plan, (7, 0));
    /// assert_eq!(mouse.release(&plan, (7, 0)).unwrap().id, "save");
    /// ```
    pub fn release<P: Into<Position>>(
        &mut self,
        plan: &PointerTargets<Id>,
        position: P,
    ) -> Option<Hit<Id>> {
        let hit = self.hover(plan, position);
        let released = hit.filter(|hit| Some(hit.id) == self.pressed);
        self.pressed = None;
        released
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn hit_test_prefers_topmost_enabled_target() {
        let plan = PointerTargets::from_targets([
            PointerTarget::new("bottom", Rect::new(0, 0, 4, 4)).z(1),
            PointerTarget::new("disabled", Rect::new(0, 0, 4, 4))
                .z(3)
                .disabled(true),
            PointerTarget::new("top", Rect::new(0, 0, 4, 4)).z(2),
        ]);

        assert_eq!(plan.hit_test((1, 1)).map(|hit| hit.id), Some("top"));
        assert_eq!(
            plan.hit_test((1, 1))
                .map(|hit| (hit.relative_x, hit.relative_y)),
            Some((1, 1))
        );
    }

    #[test]
    fn press_release_requires_same_target() {
        let plan = PointerTargets::from_targets([
            PointerTarget::new("first", Rect::new(0, 0, 2, 1)),
            PointerTarget::new("second", Rect::new(2, 0, 2, 1)),
        ]);
        let mut state = PointerState::default();

        assert_eq!(state.press(&plan, (0, 0)).map(|hit| hit.id), Some("first"));
        assert_eq!(state.release(&plan, (2, 0)).map(|hit| hit.id), None);
        assert_eq!(state.press(&plan, (2, 0)).map(|hit| hit.id), Some("second"));
        assert_eq!(
            state.release(&plan, (2, 0)).map(|hit| hit.id),
            Some("second")
        );
    }

    #[test]
    fn route_applies_pointer_phase_state_transitions() {
        let plan = PointerTargets::from_targets([
            PointerTarget::new("first", Rect::new(0, 0, 2, 1)),
            PointerTarget::new("second", Rect::new(2, 0, 2, 1)),
        ]);
        let mut state = PointerState::default();

        assert_eq!(
            state
                .route(&plan, (0, 0), PointerPhase::Hover)
                .map(|hit| hit.id),
            Some("first")
        );
        assert_eq!(state.hovered(), Some("first"));

        assert_eq!(
            state
                .route(&plan, (0, 0), PointerPhase::Press)
                .map(|hit| hit.id),
            Some("first")
        );
        assert_eq!(state.pressed(), Some("first"));
        assert_eq!(
            state
                .route(&plan, (2, 0), PointerPhase::Release)
                .map(|hit| hit.id),
            None
        );
        assert_eq!(state.pressed(), None);
    }

    #[test]
    fn clips_and_translates_targets() {
        let plan = PointerTargets::from_targets([
            PointerTarget::new("kept", Rect::new(1, 1, 4, 4)),
            PointerTarget::new("dropped", Rect::new(10, 10, 1, 1)),
        ])
        .translate(1, 0)
        .clip_to(Rect::new(0, 0, 4, 3));

        assert_eq!(
            plan.targets(),
            &[PointerTarget::new("kept", Rect::new(2, 1, 2, 2))]
        );
    }

    #[test]
    fn region_adds_whole_area_target() {
        let plan = PointerTargets::new().region("pane", Rect::new(0, 0, 10, 5));

        assert_eq!(plan.hit_test((1, 4)).map(|hit| hit.id), Some("pane"));
    }
}
