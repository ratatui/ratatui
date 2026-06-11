//! Focus targets and persistent focus state for app-owned controls.
//!
//! [`FocusTargets`](crate::focus::FocusTargets) is the keyboard counterpart to hit testing. A
//! layout can expose the regions that may receive focus, and
//! [`FocusState`](crate::focus::FocusState) can move between those targets without the container
//! owning child widgets.
//!
//! Keep focus handling local when one widget or one field owns the keyboard interaction. Use this
//! module when focus must move across app-owned controls that are visible, clipped, filtered, or
//! composed across component boundaries.
//!
//! # Common uses
//!
//! - Move through dialog fields with Tab, Shift-Tab, Up, or Down while the application owns the
//!   actual input state.
//! - Keep focus on a selected table cell or palette swatch as filtering, scrolling, or resizing
//!   changes which targets are visible.
//! - Focus a control under the pointer by using
//!   [`FocusState::focus_at`](crate::focus::FocusState::focus_at) with the previous frame's focus
//!   target collection.
//!
//! # Types
//!
//! - [`FocusTarget`](crate::focus::FocusTarget) describes one focusable region visible in the
//!   current frame.
//! - [`FocusTargets`](crate::focus::FocusTargets) stores current-frame focus targets in traversal
//!   order.
//! - [`FocusState`](crate::focus::FocusState) stores the app-owned focused id between events and
//!   frames.
//! - [`FocusFallback`](crate::focus::FocusFallback) names what should happen when stored focus no
//!   longer points at an enabled visible target.
//!
//! # Examples
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::focus::{FocusState, FocusTarget, FocusTargets};
//!
//! let targets = FocusTargets::from_targets([
//!     FocusTarget::new("title", Rect::new(0, 0, 20, 1), 0),
//!     FocusTarget::new("save", Rect::new(0, 2, 8, 1), 1),
//! ]);
//! let mut focus = FocusState::default();
//!
//! focus.next(&targets);
//! assert_eq!(focus.focused(), Some("title"));
//! focus.next(&targets);
//! assert_eq!(focus.focused(), Some("save"));
//! ```
//!
//! See [`crate::docs::interaction`] for the split between frame-local focus targets and persistent
//! app-owned focus state.

use alloc::vec::Vec;

use ratatui_core::layout::{Position, Rect};

use crate::regions::Region;

/// A focusable region in a rendered layout.
///
/// Use [`FocusTarget`](crate::focus::FocusTarget) when a button, input, tab, table cell, or row
/// should participate in keyboard traversal. The id belongs to the application; the target only
/// records where that id was rendered and whether traversal should currently skip it.
///
/// # Common uses
///
/// - A form can expose one target per field in visual order.
/// - A toolbar can disable targets for unavailable commands while leaving them visible.
/// - A table or grid can expose only visible cells so keyboard traversal matches the current frame.
///
/// # Constructors and geometry
///
/// - [`FocusTarget::new`](crate::focus::FocusTarget::new) creates an enabled focus target with a
///   traversal order.
/// - [`FocusTarget::from_region`](crate::focus::FocusTarget::from_region) derives one focus target
///   from a layout [`Region`](crate::regions::Region).
/// - [`FocusTarget::disabled`](crate::focus::FocusTarget::disabled) keeps a target visible but
///   skipped by traversal.
/// - [`FocusTarget::translate`](crate::focus::FocusTarget::translate) moves child-local target
///   geometry into parent coordinates.
/// - [`FocusTarget::clip_to`](crate::focus::FocusTarget::clip_to) clips target geometry to a
///   viewport and drops hidden targets.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::focus::{FocusState, FocusTarget, FocusTargets};
///
/// let targets = FocusTargets::from_targets([
///     FocusTarget::new("disabled", Rect::new(0, 0, 10, 1), 0).disabled(true),
///     FocusTarget::new("enabled", Rect::new(0, 1, 10, 1), 1),
/// ]);
/// let mut focus = FocusState::default();
///
/// focus.next(&targets);
/// assert_eq!(focus.focused(), Some("enabled"));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FocusTarget<Id = usize> {
    /// Application-owned target id.
    ///
    /// This should match the id used by the app to update the focused control, row, or cell. Use
    /// integers for generated indexed regions, string names for examples or diagnostics, enums for
    /// normal app controls, and stable record keys for reorderable data.
    pub id: Id,

    /// Current frame area for the focus target.
    ///
    /// This is used for pointer-to-focus routing and for drawing focus indicators.
    pub area: Rect,

    /// Traversal order. Lower values are visited first.
    ///
    /// Use visual order for predictable keyboard navigation. Targets with equal order keep their
    /// relative order after sorting unspecified by this API, so prefer distinct values.
    pub order: u16,

    /// Whether focus traversal should skip this target.
    ///
    /// Disabled targets stay in the target set for diagnostics and stable structure but are
    /// skipped by traversal and hit-based focus.
    pub disabled: bool,
}

impl<Id> FocusTarget<Id> {
    /// Creates an enabled focus target.
    ///
    /// Use this when a region should participate in traversal for the current frame.
    ///
    /// # Examples
    ///
    /// Add two rendered form fields to a traversal target set:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::from_targets([
    ///     FocusTarget::new("name", Rect::new(0, 0, 20, 1), 0),
    ///     FocusTarget::new("email", Rect::new(0, 1, 20, 1), 1),
    /// ]);
    ///
    /// assert_eq!(plan.first_enabled().unwrap().id, "name");
    /// ```
    pub const fn new(id: Id, area: Rect, order: u16) -> Self {
        Self {
            id,
            area,
            order,
            disabled: false,
        }
    }

    /// Sets whether the target is disabled.
    ///
    /// This supports visible-but-unavailable controls, such as a disabled Save button.
    ///
    /// # Examples
    ///
    /// Keep a command visible while skipping it during keyboard traversal:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusState, FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::from_targets([
    ///     FocusTarget::new("save", Rect::new(0, 0, 6, 1), 0).disabled(true),
    ///     FocusTarget::new("cancel", Rect::new(7, 0, 8, 1), 1),
    /// ]);
    /// let mut focus = FocusState::default();
    /// focus.next(&plan);
    ///
    /// assert_eq!(focus.focused(), Some("cancel"));
    /// ```
    #[must_use = "method returns the modified value"]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Creates a focus target from a layout region.
    ///
    /// This is the common bridge when every visible region is focusable and traversal order is
    /// known by the caller.
    ///
    /// # Examples
    ///
    /// Convert a single layout region into a focus target when the surrounding code owns the order:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::FocusTarget;
    /// use ratatui_layout::regions::Region;
    ///
    /// let region = Region::new("search", Rect::new(0, 0, 20, 1));
    /// let target = FocusTarget::from_region(region, 0);
    ///
    /// assert_eq!(target.id, "search");
    /// assert_eq!(target.area, Rect::new(0, 0, 20, 1));
    /// ```
    pub fn from_region(region: Region<Id>, order: u16) -> Self {
        Self::new(region.id, region.area, order)
    }

    /// Moves the target area by a signed offset.
    ///
    /// Use this when a child focus target collection was solved in local coordinates and then
    /// merged into a parent frame.
    ///
    /// # Examples
    ///
    /// Place a child-local field target into its parent dialog:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::FocusTarget;
    ///
    /// let placed = FocusTarget::new("field", Rect::new(0, 0, 10, 1), 0).translate(5, 3);
    ///
    /// assert_eq!(placed.area, Rect::new(5, 3, 10, 1));
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
    /// Drop focus targets that are outside a scroll viewport:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::FocusTarget;
    ///
    /// let target = FocusTarget::new("row", Rect::new(0, 5, 20, 1), 0);
    ///
    /// assert!(target.clip_to(Rect::new(0, 0, 20, 3)).is_none());
    /// ```
    pub fn clip_to(mut self, viewport: Rect) -> Option<Self> {
        self.area = intersect(self.area, viewport)?;
        Some(self)
    }

    fn contains<P: Into<Position>>(&self, position: P) -> bool {
        self.area.contains(position.into())
    }
}

/// Focusable targets for one frame.
///
/// Use [`FocusTargets`](crate::focus::FocusTargets) after solving layout when the app needs to move
/// focus or route keyboard input. The target set is rebuilt each frame from visible
/// [`FocusTarget`](crate::focus::FocusTarget) values. Persistent focus lives in
/// [`FocusState`](crate::focus::FocusState).
///
/// # Common uses
///
/// - Build from a dialog's visible fields and call
///   [`FocusState::next`](crate::focus::FocusState::next) on Tab.
/// - Build from a grid's visible cells and call
///   [`FocusState::focus_at`](crate::focus::FocusState::focus_at) after a mouse press.
/// - Clip or translate a child focus target collection before merging it into a parent
///   [`crate::frame::FrameSnapshot`].
///
/// # Constructors and builders
///
/// - [`FocusTargets::new`](crate::focus::FocusTargets::new) creates an empty target set.
/// - [`FocusTargets::from_targets`](crate::focus::FocusTargets::from_targets) creates a target set
///   and sorts by traversal order.
/// - [`FocusTargets::from_regions`](crate::focus::FocusTargets::from_regions) creates targets from
///   layout regions in iterator order.
/// - [`FocusTargets::target`](crate::focus::FocusTargets::target) appends one target and keeps
///   traversal sorted.
/// - [`FocusTargets::extend`](crate::focus::FocusTargets::extend) and
///   [`FocusTargets::merge`](crate::focus::FocusTargets::merge) combine child target lists.
///
/// # Composition and inspection
///
/// - [`FocusTargets::targets`](crate::focus::FocusTargets::targets) returns focus targets in
///   traversal order.
/// - [`FocusTargets::map_id`](crate::focus::FocusTargets::map_id) converts child ids into app-level
///   ids.
/// - [`FocusTargets::translate`](crate::focus::FocusTargets::translate) moves child-local targets
///   into parent coordinates.
/// - [`FocusTargets::clip_to`](crate::focus::FocusTargets::clip_to) removes hidden targets.
/// - [`FocusTargets::first_enabled`](crate::focus::FocusTargets::first_enabled) and
///   [`FocusTargets::last_enabled`](crate::focus::FocusTargets::last_enabled) find traversal
///   endpoints.
/// - [`FocusTargets::target_at`](crate::focus::FocusTargets::target_at) routes a terminal position
///   to an enabled focus target.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::focus::{FocusTarget, FocusTargets};
///
/// let local = FocusTargets::from_targets([FocusTarget::new(0, Rect::new(0, 0, 10, 1), 0)]);
/// let parent = local.translate(5, 2).map_id(|id| ("dialog", id));
///
/// assert_eq!(parent.targets()[0].id, ("dialog", 0));
/// assert_eq!(parent.targets()[0].area, Rect::new(5, 2, 10, 1));
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FocusTargets<Id = usize> {
    targets: Vec<FocusTarget<Id>>,
}

impl<Id> Default for FocusTargets<Id> {
    fn default() -> Self {
        Self {
            targets: Vec::new(),
        }
    }
}

impl<Id> FocusTargets<Id> {
    /// Creates an empty focus target collection.
    ///
    /// Use this before the first frame or for views with no keyboard-focusable controls.
    ///
    /// # Examples
    ///
    /// Store an empty target set before the first render has produced focus targets:
    ///
    /// ```rust
    /// use ratatui_layout::focus::FocusTargets;
    ///
    /// let previous_frame = FocusTargets::<()>::new();
    ///
    /// assert!(previous_frame.targets().is_empty());
    /// ```
    pub const fn new() -> Self {
        Self {
            targets: Vec::new(),
        }
    }

    /// Creates a focus target collection from targets.
    ///
    /// The targets are sorted by [`FocusTarget::order`](crate::focus::FocusTarget::order) so
    /// traversal follows the declared order rather than the input iterator.
    ///
    /// # Examples
    ///
    /// Declare traversal order independently from construction order:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::from_targets([
    ///     FocusTarget::new("second", Rect::new(0, 1, 10, 1), 1),
    ///     FocusTarget::new("first", Rect::new(0, 0, 10, 1), 0),
    /// ]);
    ///
    /// assert_eq!(plan.targets()[0].id, "first");
    /// ```
    pub fn from_targets(targets: impl Into<Vec<FocusTarget<Id>>>) -> Self {
        let mut targets = targets.into();
        targets.sort_by_key(|target| target.order);
        Self { targets }
    }

    /// Creates a focus target collection from layout regions.
    ///
    /// Iterator order becomes traversal order. Use this when every visible region should receive
    /// keyboard focus.
    ///
    /// # Examples
    ///
    /// Build dialog focus from the same regions used for rendering:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::FocusTargets;
    /// use ratatui_layout::regions::{Region, Regions};
    ///
    /// let fields = Regions::from_regions(
    ///     Rect::new(0, 0, 20, 2),
    ///     [
    ///         Region::new("title", Rect::new(0, 0, 20, 1)),
    ///         Region::new("owner", Rect::new(0, 1, 20, 1)),
    ///     ],
    /// );
    /// let focus = FocusTargets::from_regions(fields.regions().iter().copied());
    ///
    /// assert_eq!(focus.targets()[1].id, "owner");
    /// ```
    pub fn from_regions(regions: impl IntoIterator<Item = Region<Id>>) -> Self
    where
        Id: Copy,
    {
        Self::from_targets(
            regions
                .into_iter()
                .enumerate()
                .map(|(order, region)| FocusTarget::from_region(region, order as u16))
                .collect::<Vec<_>>(),
        )
    }

    /// Adds a target and returns the modified target set.
    ///
    /// # Examples
    ///
    /// Build a small dialog focus target collection as fields are rendered:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::new()
    ///     .target(FocusTarget::new("title", Rect::new(0, 0, 20, 1), 0))
    ///     .target(FocusTarget::new("body", Rect::new(0, 2, 20, 1), 1));
    ///
    /// assert_eq!(plan.last_enabled().unwrap().id, "body");
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn target(mut self, target: FocusTarget<Id>) -> Self {
        self.targets.push(target);
        self.targets.sort_by_key(|target| target.order);
        self
    }

    /// Returns all focus targets in traversal order.
    ///
    /// # Examples
    ///
    /// Render focus indicators in the same order keyboard traversal will use:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::from_targets([
    ///     FocusTarget::new("a", Rect::new(0, 0, 1, 1), 0),
    ///     FocusTarget::new("b", Rect::new(1, 0, 1, 1), 1),
    /// ]);
    /// let ids: Vec<_> = plan.targets().iter().map(|target| target.id).collect();
    ///
    /// assert_eq!(ids, ["a", "b"]);
    /// ```
    pub fn targets(&self) -> &[FocusTarget<Id>] {
        &self.targets
    }

    /// Extends the target set with additional targets and keeps traversal order sorted.
    ///
    /// Use this when a parent aggregates focus targets from multiple child components.
    ///
    /// # Examples
    ///
    /// Append footer controls to the current form targets:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusTarget, FocusTargets};
    ///
    /// let mut plan =
    ///     FocusTargets::from_targets([FocusTarget::new("field", Rect::new(0, 0, 20, 1), 0)]);
    /// plan.extend([FocusTarget::new("save", Rect::new(0, 2, 8, 1), 10)]);
    ///
    /// assert_eq!(plan.targets()[1].id, "save");
    /// ```
    pub fn extend<I>(&mut self, targets: I)
    where
        I: IntoIterator<Item = FocusTarget<Id>>,
    {
        self.targets.extend(targets);
        self.targets.sort_by_key(|target| target.order);
    }

    /// Returns a target set containing this set's targets and another set's targets.
    ///
    /// # Examples
    ///
    /// Merge sibling component focus target collections into one frame-local traversal set:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusTarget, FocusTargets};
    ///
    /// let form = FocusTargets::from_targets([FocusTarget::new("name", Rect::new(0, 0, 10, 1), 0)]);
    /// let footer = FocusTargets::from_targets([FocusTarget::new("save", Rect::new(0, 2, 6, 1), 1)]);
    ///
    /// assert_eq!(form.merge(footer).last_enabled().unwrap().id, "save");
    /// ```
    #[must_use = "method returns the merged plan"]
    pub fn merge(mut self, other: Self) -> Self {
        self.extend(other.targets);
        self
    }

    /// Maps target ids while preserving areas, order, and disabled state.
    ///
    /// This is useful when a child component uses local ids but the parent stores focus using a
    /// broader app-level id enum.
    ///
    /// # Examples
    ///
    /// Lift local child ids into an app-level enum before storing focus state:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusTarget, FocusTargets};
    ///
    /// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    /// enum Region {
    ///     Dialog(usize),
    /// }
    ///
    /// let child = FocusTargets::from_targets([FocusTarget::new(0, Rect::new(0, 0, 10, 1), 0)]);
    /// let parent = child.map_id(Region::Dialog);
    ///
    /// assert_eq!(parent.targets()[0].id, Region::Dialog(0));
    /// ```
    pub fn map_id<NextId, F>(self, mut map: F) -> FocusTargets<NextId>
    where
        F: FnMut(Id) -> NextId,
    {
        FocusTargets::from_targets(
            self.targets
                .into_iter()
                .map(|target| FocusTarget {
                    id: map(target.id),
                    area: target.area,
                    order: target.order,
                    disabled: target.disabled,
                })
                .collect::<Vec<_>>(),
        )
    }

    /// Moves all target areas by a signed offset.
    ///
    /// # Examples
    ///
    /// Place a child form solved at local origin inside a dialog:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusTarget, FocusTargets};
    ///
    /// let child = FocusTargets::from_targets([FocusTarget::new("field", Rect::new(0, 0, 12, 1), 0)]);
    /// let placed = child.translate(4, 2);
    ///
    /// assert_eq!(placed.targets()[0].area, Rect::new(4, 2, 12, 1));
    /// ```
    #[must_use = "method returns the translated plan"]
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
    /// # Examples
    ///
    /// Keep traversal aligned with rows visible inside a scroll viewport:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::from_targets([
    ///     FocusTarget::new("visible", Rect::new(0, 1, 20, 1), 0),
    ///     FocusTarget::new("hidden", Rect::new(0, 5, 20, 1), 1),
    /// ])
    /// .clip_to(Rect::new(0, 0, 20, 3));
    ///
    /// assert_eq!(plan.targets().len(), 1);
    /// ```
    #[must_use = "method returns the clipped plan"]
    pub fn clip_to(mut self, viewport: Rect) -> Self {
        self.targets = self
            .targets
            .into_iter()
            .filter_map(|target| target.clip_to(viewport))
            .collect();
        self
    }

    /// Returns the first enabled target.
    ///
    /// This is useful for initializing focus after a view first appears.
    ///
    /// # Examples
    ///
    /// Initialize dialog focus to the first enabled field:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::from_targets([
    ///     FocusTarget::new("disabled", Rect::new(0, 0, 10, 1), 0).disabled(true),
    ///     FocusTarget::new("name", Rect::new(0, 1, 10, 1), 1),
    /// ]);
    ///
    /// assert_eq!(plan.first_enabled().unwrap().id, "name");
    /// ```
    pub fn first_enabled(&self) -> Option<&FocusTarget<Id>> {
        self.targets.iter().find(|target| !target.disabled)
    }

    /// Returns the last enabled target.
    ///
    /// This is useful for reverse traversal when nothing is currently focused.
    ///
    /// # Examples
    ///
    /// Start reverse traversal at the last enabled target:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::from_targets([
    ///     FocusTarget::new("first", Rect::new(0, 0, 10, 1), 0),
    ///     FocusTarget::new("last", Rect::new(0, 1, 10, 1), 1),
    /// ]);
    ///
    /// assert_eq!(plan.last_enabled().unwrap().id, "last");
    /// ```
    pub fn last_enabled(&self) -> Option<&FocusTarget<Id>> {
        self.targets.iter().rev().find(|target| !target.disabled)
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

impl<Id: Copy + Eq> FocusTargets<Id> {
    /// Returns the enabled target at the given position.
    ///
    /// # Examples
    ///
    /// Move keyboard focus to the field under a mouse press:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::from_targets([FocusTarget::new("name", Rect::new(0, 0, 20, 1), 0)]);
    ///
    /// assert_eq!(plan.target_at((3, 0)).unwrap().id, "name");
    /// ```
    pub fn target_at<P: Into<Position>>(&self, position: P) -> Option<&FocusTarget<Id>> {
        let position = position.into();
        self.targets
            .iter()
            .rev()
            .find(|target| !target.disabled && target.contains(position))
    }

    fn target_index(&self, id: Id) -> Option<usize> {
        self.targets
            .iter()
            .position(|target| !target.disabled && target.id == id)
    }

    fn next_after(&self, index: usize) -> Option<&FocusTarget<Id>> {
        self.targets[index + 1..]
            .iter()
            .chain(self.targets[..=index].iter())
            .find(|target| !target.disabled)
    }

    fn previous_before(&self, index: usize) -> Option<&FocusTarget<Id>> {
        self.targets[..index]
            .iter()
            .rev()
            .chain(self.targets[index..].iter().rev())
            .find(|target| !target.disabled)
    }
}

/// Fallback policy for stale focus.
///
/// [`FocusState`](crate::focus::FocusState) persists between frames, while
/// [`FocusTargets`](crate::focus::FocusTargets) is rebuilt from visible targets. Filtering,
/// resizing, or disabling a control can make the stored id stale. This enum names the repair policy
/// used by [`FocusState::ensure_visible`](crate::focus::FocusState::ensure_visible).
///
/// Use [`FocusFallback::First`](crate::focus::FocusFallback::First) for ordinary forms, toolbars,
/// and menus where focus should remain usable after a target disappears. Use
/// [`FocusFallback::Last`](crate::focus::FocusFallback::Last) for reverse traversal or footer
/// controls that should keep focus near the end. Use
/// [`FocusFallback::Clear`](crate::focus::FocusFallback::Clear) when a view should stop receiving
/// keyboard input until the app explicitly focuses something else.
///
/// # Variants
///
/// - [`FocusFallback::Clear`](crate::focus::FocusFallback::Clear) removes focus when the stored id
///   is stale.
/// - [`FocusFallback::First`](crate::focus::FocusFallback::First) moves stale focus to the first
///   enabled target.
/// - [`FocusFallback::Last`](crate::focus::FocusFallback::Last) moves stale focus to the last
///   enabled target.
///
/// # Examples
///
/// Keep toolbar focus usable when the focused command becomes disabled:
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::focus::{FocusFallback, FocusState, FocusTarget, FocusTargets};
///
/// let targets = FocusTargets::from_targets([
///     FocusTarget::new("save", Rect::new(0, 0, 6, 1), 0).disabled(true),
///     FocusTarget::new("cancel", Rect::new(7, 0, 8, 1), 1),
/// ]);
/// let mut focus = FocusState::default();
/// focus.focus(Some("save"));
/// focus.ensure_visible(&targets, FocusFallback::First);
///
/// assert_eq!(focus.focused(), Some("cancel"));
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FocusFallback {
    /// Clear stale focus.
    ///
    /// Use this when focus should not jump to another target automatically.
    #[default]
    Clear,
    /// Move stale focus to the first enabled target.
    ///
    /// This is the common policy for forms, command strips, and menus.
    First,
    /// Move stale focus to the last enabled target.
    ///
    /// This supports reverse traversal and footer-like focus groups.
    Last,
}

/// Persistent focus state for an app or component.
///
/// Use [`FocusState`](crate::focus::FocusState) next to the data it controls. Each frame, pair it
/// with the current [`FocusTargets`](crate::focus::FocusTargets) to clamp stale focus and move to
/// the next visible enabled target.
///
/// # Common uses
///
/// - Store the focused form field while the form is redrawn every frame.
/// - Keep focus stable across resizes by clamping it to the current
///   [`FocusTargets`](crate::focus::FocusTargets).
/// - Move focus from a mouse click by pairing
///   [`FocusState::focus_at`](crate::focus::FocusState::focus_at) with the previous frame's target
///   set.
///
/// # Accessors and updates
///
/// - [`FocusState::focused`](crate::focus::FocusState::focused) returns the current focused id.
/// - [`FocusState::focus`](crate::focus::FocusState::focus) sets focus directly.
/// - [`FocusState::clear`](crate::focus::FocusState::clear) removes focus.
/// - [`FocusState::first`](crate::focus::FocusState::first) and
///   [`FocusState::last`](crate::focus::FocusState::last) jump to traversal endpoints.
/// - [`FocusState::next`](crate::focus::FocusState::next) and
///   [`FocusState::previous`](crate::focus::FocusState::previous) move through enabled targets.
/// - [`FocusState::focus_at`](crate::focus::FocusState::focus_at) moves focus to the enabled target
///   under a position.
/// - [`FocusState::ensure_visible`](crate::focus::FocusState::ensure_visible) repairs stale focus
///   with an explicit [`FocusFallback`](crate::focus::FocusFallback).
/// - [`FocusState::clamp_to`](crate::focus::FocusState::clamp_to) clears stale focus after
///   filtering, scrolling, or disabling targets.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::focus::{FocusState, FocusTarget, FocusTargets};
///
/// let targets =
///     FocusTargets::from_targets([FocusTarget::new("field", Rect::new(4, 2, 10, 1), 0)]);
/// let mut focus = FocusState::default();
///
/// focus.focus_at(&targets, (6, 2));
/// assert_eq!(focus.focused(), Some("field"));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FocusState<Id = usize> {
    focused: Option<Id>,
}

impl<Id> Default for FocusState<Id> {
    fn default() -> Self {
        Self { focused: None }
    }
}

impl<Id> FocusState<Id> {
    /// Returns the focused id.
    ///
    /// The id is app-owned; use it to decide which field receives keyboard input or which control
    /// should draw a focused style.
    ///
    /// # Examples
    ///
    /// Read the focused id while deciding which input receives a key event:
    ///
    /// ```rust
    /// use ratatui_layout::focus::FocusState;
    ///
    /// let mut focus = FocusState::default();
    /// focus.focus(Some("search"));
    ///
    /// assert_eq!(focus.focused(), Some("search"));
    /// ```
    pub const fn focused(&self) -> Option<Id>
    where
        Id: Copy,
    {
        self.focused
    }

    /// Sets the focused id.
    ///
    /// This does not validate against a target set. Call
    /// [`FocusState::clamp_to`](crate::focus::FocusState::clamp_to) after rendering
    /// if the focused id may have become hidden or disabled.
    ///
    /// # Examples
    ///
    /// Restore focus from app state before the next frame clamps it:
    ///
    /// ```rust
    /// use ratatui_layout::focus::FocusState;
    ///
    /// let mut focus = FocusState::default();
    /// focus.focus(Some("email"));
    ///
    /// assert_eq!(focus.focused(), Some("email"));
    /// ```
    pub fn focus(&mut self, focused: Option<Id>) {
        self.focused = focused;
    }

    /// Clears focus.
    ///
    /// Use this when closing a view or when keyboard input should stop targeting the current
    /// control.
    ///
    /// # Examples
    ///
    /// Clear stale focus when a modal closes:
    ///
    /// ```rust
    /// use ratatui_layout::focus::FocusState;
    ///
    /// let mut focus = FocusState::default();
    /// focus.focus(Some("modal-field"));
    /// focus.clear();
    ///
    /// assert_eq!(focus.focused(), None);
    /// ```
    pub fn clear(&mut self) {
        self.focused = None;
    }
}

impl<Id: Copy + Eq> FocusState<Id> {
    /// Moves focus to the first enabled target.
    ///
    /// This is the typical initialization step when a dialog opens and nothing is focused yet.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusState, FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::from_targets([FocusTarget::new("first", Rect::new(0, 0, 1, 1), 0)]);
    /// let mut focus = FocusState::default();
    /// focus.first(&plan);
    ///
    /// assert_eq!(focus.focused(), Some("first"));
    /// ```
    pub fn first(&mut self, plan: &FocusTargets<Id>) {
        self.focused = plan.first_enabled().map(|target| target.id);
    }

    /// Moves focus to the last enabled target.
    ///
    /// This supports reverse traversal from an empty focus state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusState, FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::from_targets([
    ///     FocusTarget::new("first", Rect::new(0, 0, 1, 1), 0),
    ///     FocusTarget::new("last", Rect::new(1, 0, 1, 1), 1),
    /// ]);
    /// let mut focus = FocusState::default();
    /// focus.last(&plan);
    ///
    /// assert_eq!(focus.focused(), Some("last"));
    /// ```
    pub fn last(&mut self, plan: &FocusTargets<Id>) {
        self.focused = plan.last_enabled().map(|target| target.id);
    }

    /// Moves focus to the next enabled target, wrapping at the end.
    ///
    /// Use this for Tab, Down, or Right depending on the view's interaction model.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusState, FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::from_targets([
    ///     FocusTarget::new("a", Rect::new(0, 0, 1, 1), 0),
    ///     FocusTarget::new("b", Rect::new(1, 0, 1, 1), 1),
    /// ]);
    /// let mut focus = FocusState::default();
    /// focus.next(&plan);
    /// focus.next(&plan);
    ///
    /// assert_eq!(focus.focused(), Some("b"));
    /// ```
    pub fn next(&mut self, plan: &FocusTargets<Id>) {
        self.focused = match self.focused.and_then(|id| plan.target_index(id)) {
            Some(index) => plan.next_after(index).map(|target| target.id),
            None => plan.first_enabled().map(|target| target.id),
        };
    }

    /// Moves focus to the previous enabled target, wrapping at the start.
    ///
    /// Use this for Shift-Tab, Up, or Left depending on the view's interaction model.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusState, FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::from_targets([
    ///     FocusTarget::new("a", Rect::new(0, 0, 1, 1), 0),
    ///     FocusTarget::new("b", Rect::new(1, 0, 1, 1), 1),
    /// ]);
    /// let mut focus = FocusState::default();
    /// focus.previous(&plan);
    ///
    /// assert_eq!(focus.focused(), Some("b"));
    /// ```
    pub fn previous(&mut self, plan: &FocusTargets<Id>) {
        self.focused = match self.focused.and_then(|id| plan.target_index(id)) {
            Some(index) => plan.previous_before(index).map(|target| target.id),
            None => plan.last_enabled().map(|target| target.id),
        };
    }

    /// Focuses the enabled target at a terminal position.
    ///
    /// This is useful after a mouse press when pointer routing and keyboard focus should converge
    /// on the same control.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusState, FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::from_targets([FocusTarget::new("field", Rect::new(2, 0, 10, 1), 0)]);
    /// let mut focus = FocusState::default();
    /// focus.focus_at(&plan, (3, 0));
    ///
    /// assert_eq!(focus.focused(), Some("field"));
    /// ```
    pub fn focus_at<P: Into<Position>>(&mut self, plan: &FocusTargets<Id>, position: P) {
        self.focused = plan.target_at(position).map(|target| target.id);
    }

    /// Keeps valid focus or applies a fallback.
    ///
    /// Call this after a render pass builds the current
    /// [`FocusTargets`](crate::focus::FocusTargets).
    ///
    /// # Examples
    ///
    /// Fall back to the first enabled command after the focused command becomes disabled:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusFallback, FocusState, FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::from_targets([
    ///     FocusTarget::new("run", Rect::new(0, 0, 5, 1), 0).disabled(true),
    ///     FocusTarget::new("help", Rect::new(6, 0, 6, 1), 1),
    /// ]);
    /// let mut focus = FocusState::default();
    /// focus.focus(Some("run"));
    /// focus.ensure_visible(&plan, FocusFallback::First);
    ///
    /// assert_eq!(focus.focused(), Some("help"));
    /// ```
    pub fn ensure_visible(&mut self, plan: &FocusTargets<Id>, fallback: FocusFallback) {
        if self.focused.and_then(|id| plan.target_index(id)).is_some() {
            return;
        }

        self.focused = match fallback {
            FocusFallback::Clear => None,
            FocusFallback::First => plan.first_enabled().map(|target| target.id),
            FocusFallback::Last => plan.last_enabled().map(|target| target.id),
        };
    }

    /// Clears stale focus.
    ///
    /// This is shorthand for
    /// [`FocusState::ensure_visible`](crate::focus::FocusState::ensure_visible) with
    /// [`FocusFallback::Clear`](crate::focus::FocusFallback::Clear).
    ///
    /// # Examples
    ///
    /// Clear focus when filtering removes the focused row from the visible target set:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::focus::{FocusState, FocusTarget, FocusTargets};
    ///
    /// let plan = FocusTargets::from_targets([FocusTarget::new("visible", Rect::new(0, 0, 1, 1), 0)]);
    /// let mut focus = FocusState::default();
    /// focus.focus(Some("hidden"));
    /// focus.clamp_to(&plan);
    ///
    /// assert_eq!(focus.focused(), None);
    /// ```
    pub fn clamp_to(&mut self, plan: &FocusTargets<Id>) {
        self.ensure_visible(plan, FocusFallback::Clear);
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use ratatui_core::layout::Rect;

    use super::*;

    #[test]
    fn traversal_skips_disabled_targets() {
        let plan = FocusTargets::from_targets([
            FocusTarget::new("first", Rect::new(0, 0, 1, 1), 0),
            FocusTarget::new("disabled", Rect::new(1, 0, 1, 1), 1).disabled(true),
            FocusTarget::new("last", Rect::new(2, 0, 1, 1), 2),
        ]);
        let mut state = FocusState::default();

        state.next(&plan);
        assert_eq!(state.focused(), Some("first"));

        state.next(&plan);
        assert_eq!(state.focused(), Some("last"));

        state.next(&plan);
        assert_eq!(state.focused(), Some("first"));
    }

    #[test]
    fn focus_at_uses_enabled_target_under_position() {
        let plan = FocusTargets::from_targets([
            FocusTarget::new("disabled", Rect::new(0, 0, 4, 1), 0).disabled(true),
            FocusTarget::new("enabled", Rect::new(0, 0, 4, 1), 1),
        ]);
        let mut state = FocusState::default();

        state.focus_at(&plan, (1, 0));

        assert_eq!(state.focused(), Some("enabled"));
    }

    #[test]
    fn from_regions_uses_iterator_order() {
        let plan = FocusTargets::from_regions([
            Region::new("title", Rect::new(0, 0, 10, 1)),
            Region::new("owner", Rect::new(0, 1, 10, 1)),
        ]);

        assert_eq!(plan.targets()[0].id, "title");
        assert_eq!(plan.targets()[0].order, 0);
        assert_eq!(plan.targets()[1].id, "owner");
        assert_eq!(plan.targets()[1].order, 1);
    }

    #[test]
    fn ensure_visible_keeps_valid_focus() {
        let plan = FocusTargets::from_targets([
            FocusTarget::new("first", Rect::new(0, 0, 1, 1), 0),
            FocusTarget::new("second", Rect::new(1, 0, 1, 1), 1),
        ]);
        let mut state = FocusState::default();

        state.focus(Some("second"));
        state.ensure_visible(&plan, FocusFallback::First);

        assert_eq!(state.focused(), Some("second"));
    }

    #[test]
    fn ensure_visible_uses_declared_fallback_for_stale_focus() {
        let plan = FocusTargets::from_targets([
            FocusTarget::new("disabled", Rect::new(0, 0, 1, 1), 0).disabled(true),
            FocusTarget::new("first", Rect::new(1, 0, 1, 1), 1),
            FocusTarget::new("last", Rect::new(2, 0, 1, 1), 2),
        ]);
        let mut state = FocusState::default();

        state.focus(Some("missing"));
        state.ensure_visible(&plan, FocusFallback::First);
        assert_eq!(state.focused(), Some("first"));

        state.focus(Some("missing"));
        state.ensure_visible(&plan, FocusFallback::Last);
        assert_eq!(state.focused(), Some("last"));

        state.focus(Some("missing"));
        state.ensure_visible(&plan, FocusFallback::Clear);
        assert_eq!(state.focused(), None);
    }
}
