//! Aggregate frame-local UI data.
//!
//! Immediate-mode apps usually handle input after the previous frame was drawn. That means a click,
//! focus movement, or cursor decision often needs the data produced by the last render pass, not a
//! retained widget tree. [`FrameSnapshot`] is the value a component can return when it wants to
//! expose that data as data.
//!
//! A [`FrameSnapshot`] is the optional coordination layer above individual [`Regions`],
//! [`FocusTargets`], [`PointerTargets`], and [`CursorRequests`] values. A render pass builds it
//! from visible UI data, the app stores it, and the next input event can route through the previous
//! frame without requiring a retained widget tree.
//!
//! Use the smaller values directly when only one concern is needed. [`FrameSnapshot`] is useful
//! when a component boundary needs to return several concerns together.
//!
//! # Type
//!
//! - [`FrameSnapshot`] aggregates the frame-local data produced by rendering: geometry, keyboard
//!   focus targets, pointer targets, and cursor requests.
//! - [`FrameTargets`] builds a [`FrameSnapshot`] from visible regions when layout, pointer, and
//!   focus target data should stay aligned.
//! - [`RegionTargets`] starts from an existing [`Regions`] and adds focus, pointer, disabled, and
//!   z-order policy before building a [`FrameSnapshot`].
//!
//! # Common uses
//!
//! - Store a previous frame so the next pointer event can route through the regions that were
//!   actually visible.
//! - Return one value from a component that produced geometry, keyboard focus targets, pointer
//!   targets, and cursor requests.
//! - Compose child components by mapping local ids into parent ids, translating local coordinates,
//!   clipping hidden regions, and merging the results.
//! - Preserve render-order cursor behavior across children, where later visible cursor requests can
//!   win.
//!
//! # Examples
//!
//! Store previous-frame-local data and route the next event through them:
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::{FrameSnapshot, Region, Regions};
//!
//! let previous_frame = FrameSnapshot::from_layout(Regions::from_regions(
//!     Rect::new(0, 0, 20, 1),
//!     [Region::new("save", Rect::new(10, 0, 10, 1))],
//! ));
//!
//! assert_eq!(previous_frame.route_position((12, 0)).unwrap().id, "save");
//! ```
//!
//! Pointer-specific targets can override layout fallback, which is useful for disabled controls or
//! pointer-only regions:
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::{FrameSnapshot, PointerTarget, PointerTargets, Region, Regions};
//!
//! let layout_regions = Regions::from_regions(
//!     Rect::new(0, 0, 10, 1),
//!     [Region::new("layout", Rect::new(0, 0, 10, 1))],
//! );
//! let pointer_targets =
//!     PointerTargets::from_targets([PointerTarget::new("mouse", Rect::new(0, 0, 10, 1))]);
//! let frame = FrameSnapshot::from_layout(layout_regions).mouse(pointer_targets);
//!
//! assert_eq!(frame.route_position((1, 0)).unwrap().id, "mouse");
//! ```
//!
//! See [`crate::docs::frame_snapshots`] for the full render-then-route model and
//! [`crate::docs::widget_contracts`] for the future component/action direction that this type is
//! intended to leave room for.

use alloc::vec::Vec;

use ratatui_core::layout::{Position, Rect};

use crate::cursor::CursorRequests;
use crate::focus::{FocusTarget, FocusTargets};
use crate::pointer::{PointerTarget, PointerTargets};
use crate::regions::{Hit, Region, Regions};

/// UI coordination data produced by one render pass.
///
/// [`FrameSnapshot`] owns frame-local geometry, focus targets, pointer targets, and cursor
/// requests. It does not own app data, widgets, event callbacks, selection, or retained children.
/// Store it after rendering when the next event should use the previous frame's visible data.
///
/// The type is intentionally an aggregate, not a replacement for the smaller values. A component
/// that only needs hit testing should return [`Regions`] or [`PointerTargets`] directly. Use
/// [`FrameSnapshot`] when a boundary would otherwise need to return several independent region and
/// target values and the caller must keep their ids aligned.
///
/// # Constructors and setters
///
/// - [`FrameSnapshot::new`] creates an empty aggregate for a solved area.
/// - [`FrameSnapshot::from_layout`] starts from geometry and adds interaction data later.
/// - [`FrameSnapshot::focus`] attaches a [`FocusTargets`] produced by the same render pass.
/// - [`FrameSnapshot::mouse`] attaches a [`PointerTargets`] for pointer routing.
/// - [`FrameSnapshot::mouse_target`] adds one pointer target to an existing frame.
/// - [`FrameSnapshot::scroll_region`] adds a whole-region pointer target, commonly for wheel
///   routing over blank pane space.
/// - [`FrameSnapshot::cursor`] attaches a [`CursorRequests`] for terminal cursor placement.
///
/// # Composition
///
/// - [`FrameSnapshot::merge`] appends a child frame's data to a parent frame.
/// - [`FrameSnapshot::merge_child`] translates, clips, and merges a local child frame in one call.
/// - [`FrameSnapshot::place_child`] places a local child frame in an already solved screen area.
/// - [`FrameSnapshot::translate`] moves child-local layout, focus, pointer, and cursor requests
///   into parent coordinates.
/// - [`FrameSnapshot::clip_to`] removes hidden child data before the snapshot is stored for input.
/// - [`FrameSnapshot::map_id`] converts child ids into application-level ids while preserving
///   geometry and interaction behavior.
///
/// # Routing
///
/// - [`FrameSnapshot::route_position`] routes broad position queries through pointer targets first
///   and layout regions second, returning local coordinates in a [`crate::Hit`].
/// - [`FrameSnapshot::route_click`], [`FrameSnapshot::route_hover`], and
///   [`FrameSnapshot::route_scroll`] are intent-named routes that use explicit pointer policy when
///   a pointer target collection exists.
///
/// # Examples
///
/// Compose a child frame that solved local coordinates into a parent frame that uses app-level ids:
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::{FrameSnapshot, Region, Regions};
///
/// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// enum AppRegion {
///     Child(usize),
///     Status,
/// }
///
/// let child_regions = Regions::from_regions(
///     Rect::new(0, 0, 8, 1),
///     [Region::new(0, Rect::new(0, 0, 8, 1))],
/// );
/// let child_frame = FrameSnapshot::from_layout(child_regions)
///     .map_id(AppRegion::Child)
///     .translate(4, 2);
/// let status = FrameSnapshot::from_layout(Regions::from_regions(
///     Rect::new(0, 0, 20, 5),
///     [Region::new(AppRegion::Status, Rect::new(0, 4, 20, 1))],
/// ));
///
/// let frame = status.merge(child_frame);
/// assert_eq!(
///     frame.route_position((5, 2)).unwrap().id,
///     AppRegion::Child(0)
/// );
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameSnapshot<Id = usize> {
    /// Geometry data for visible regions.
    ///
    /// This is the broadest hit-test fallback and the source of renderable [`crate::Region`]
    /// values.
    pub layout: Regions<Id>,
    /// Keyboard traversal targets for visible controls.
    ///
    /// Persistent focused id remains in [`crate::FocusState`]; this field records only the targets
    /// visible in this frame.
    pub focus: FocusTargets<Id>,
    /// Pointer routing targets for visible controls.
    ///
    /// Persistent hover and pressed ids remain in [`crate::PointerState`]; this field records only
    /// where pointer events can route for this frame.
    pub mouse: PointerTargets<Id>,
    /// Cursor requests emitted during rendering.
    ///
    /// The final cursor can be derived after all children have contributed requests.
    pub cursor: CursorRequests,
}

/// Builder that turns visible regions into frame-local routing data.
///
/// Many components produce the same three data from the same solved rectangles: layout regions for
/// geometry, pointer targets for pointer routing, and focus targets for keyboard traversal.
/// [`FrameTargets`] packages that pattern so component code can describe policy instead of
/// manually keeping three values aligned.
///
/// The builder does not render and does not own application state. It consumes regions produced by
/// a layout helper, maps local region ids into app ids, and records which regions are focusable or
/// disabled for this frame. Use [`FrameSnapshot`] directly when a component has only one or two
/// ad-hoc targets; use [`FrameTargets`] when a repeated list, grid, toolbar, or dialog field set
/// would otherwise build [`Regions`], [`PointerTargets`], and [`FocusTargets`] in parallel.
///
/// # Common Uses
///
/// - Turn virtual-list rows into layout, pointer, and focus targets.
/// - Keep table header cells pointer-routable while making only body cells focusable.
/// - Mark disabled toolbar buttons as visible layout regions while excluding them from focus and
///   activation.
/// - Assign higher z-order to overlay targets so hit testing follows render order.
///
/// # Examples
///
/// Build focus and pointer data for visible dialog fields:
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::{FrameTargets, Region};
///
/// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// enum Target {
///     Field(&'static str),
/// }
///
/// let regions = [
///     Region::new("title", Rect::new(0, 0, 20, 1)),
///     Region::new("owner", Rect::new(0, 1, 20, 1)),
/// ];
/// let frame =
///     FrameTargets::new(Rect::new(0, 0, 20, 2), 10).build_focusable(regions, Target::Field);
///
/// assert_eq!(
///     frame.route_click((1, 1)).unwrap().id,
///     Target::Field("owner")
/// );
/// assert_eq!(frame.focus.targets()[0].order, 10);
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct FrameTargets<Id = usize> {
    area: Rect,
    z: u16,
    focus_start: u16,
    mouse_region: Option<(Id, Rect)>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct FrameTargetSlot<Id> {
    region: Region<Id>,
    focusable: bool,
    mouseable: bool,
    disabled: bool,
}

impl<Id> FrameTargets<Id> {
    /// Starts a target builder for a component area.
    ///
    /// `focus_start` is the first focus order assigned to focusable regions. Parent components can
    /// reserve ranges for children so traversal follows page order after values are merged.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::FrameTargets;
    ///
    /// let targets = FrameTargets::<&str>::new(Rect::new(0, 0, 10, 1), 20);
    ///
    /// assert_eq!(targets.focus_start(), 20);
    /// ```
    pub const fn new(area: Rect, focus_start: u16) -> Self {
        Self {
            area,
            z: 0,
            focus_start,
            mouse_region: None,
        }
    }

    /// Returns the component area used for the produced region set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::FrameTargets;
    ///
    /// let area = Rect::new(0, 0, 10, 1);
    /// let targets = FrameTargets::<()>::new(area, 0);
    ///
    /// assert_eq!(targets.area(), area);
    /// ```
    pub const fn area(&self) -> Rect {
        self.area
    }

    /// Returns the first focus order assigned by this builder.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::FrameTargets;
    ///
    /// let targets = FrameTargets::<()>::new(Rect::new(0, 0, 1, 1), 7);
    ///
    /// assert_eq!(targets.focus_start(), 7);
    /// ```
    pub const fn focus_start(&self) -> u16 {
        self.focus_start
    }

    /// Applies z-order to generated layout regions and pointer targets.
    ///
    /// Use this for overlays and dialogs that are rendered above the base page. Higher z values
    /// win hit testing when areas overlap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameTargets, Region};
    ///
    /// let frame = FrameTargets::new(Rect::new(0, 0, 4, 1), 0)
    ///     .z(10)
    ///     .build_focusable([Region::new("popup", Rect::new(0, 0, 4, 1))], |id| id);
    ///
    /// assert_eq!(frame.layout.regions()[0].z, 10);
    /// ```
    #[must_use = "method returns the modified builder"]
    pub const fn z(mut self, z: u16) -> Self {
        self.z = z;
        self
    }

    /// Adds a whole-region pointer target before region-specific targets.
    ///
    /// Scrollable panes use this so wheel events route over blank viewport space as well as over
    /// visible rows or cells.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::FrameTargets;
    ///
    /// let area = Rect::new(0, 0, 20, 5);
    /// let frame = FrameTargets::new(area, 0).mouse_region("pane", area).build(
    ///     core::iter::empty::<ratatui_layout::Region<&str>>(),
    ///     |id| id,
    ///     |_| false,
    ///     |_| false,
    /// );
    ///
    /// assert_eq!(frame.route_scroll((1, 1)).unwrap().id, "pane");
    /// ```
    #[must_use = "method returns the modified builder"]
    pub fn mouse_region(mut self, id: Id, area: Rect) -> Self {
        self.mouse_region = Some((id, area));
        self
    }

    /// Starts a target builder from a solved region set.
    ///
    /// Use this after [`Row`](crate::Row), [`Column`](crate::Column), [`Grid`](crate::Grid),
    /// [`Overlay`](crate::Overlay), a viewport, or a custom component has already produced
    /// geometry. The returned [`RegionTargets`] adds policy that [`Regions`] does not store:
    /// disabled, focusable, pointer routing, whole-region pointer routing, and z-order.
    ///
    /// # Examples
    ///
    /// Derive frame-local data from a toolbar layout:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::{FocusFallback, FocusState, FrameTargets, Row};
    ///
    /// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    /// enum Command {
    ///     Open,
    ///     Save,
    /// }
    ///
    /// let command_slots = [
    ///     (Command::Open, Constraint::Length(6)),
    ///     (Command::Save, Constraint::Length(6)),
    /// ];
    /// let plan = Row::named(command_slots)
    ///     .spacing(1)
    ///     .regions(Rect::new(0, 0, 13, 1));
    /// let frame = FrameTargets::from_regions(plan, 0)
    ///     .disabled(|command| command == Command::Save)
    ///     .build();
    /// let mut focus = FocusState::default();
    ///
    /// focus.ensure_visible(&frame.focus, FocusFallback::First);
    ///
    /// assert_eq!(focus.focused(), Some(Command::Open));
    /// assert!(frame.route_click((8, 0)).is_none());
    /// assert_eq!(frame.layout.hit_test((8, 0)).unwrap().id, Command::Save);
    /// ```
    pub fn from_regions(plan: Regions<Id>, focus_start: u16) -> RegionTargets<Id> {
        RegionTargets::new(plan, focus_start)
    }
}

/// Builder that derives frame-local targets from solved regions.
///
/// [`Regions`] stores visible ids and rectangles. [`RegionTargets`] adds interaction policy
/// while keeping layout, focus, and pointer targets aligned on the same ids.
///
/// Use this builder when rendering and routing share solved regions:
///
/// - a toolbar or command strip where disabled commands remain visible;
/// - a table with mouseable headers but focusable body cells;
/// - a segmented control that is mouseable but intentionally skipped by page-level focus;
/// - a scrollable pane that needs a whole-region pointer target in addition to item regions.
///
/// Labels, styles, shortcuts, command effects, widget state, and app data stay outside this type.
///
/// # Methods
///
/// - [`RegionTargets::disabled`] marks visible regions unavailable for focus traversal and pointer
///   activation.
/// - [`RegionTargets::focusable`] chooses which regions join keyboard traversal.
/// - [`RegionTargets::mouseable`] chooses which regions receive pointer routing.
/// - [`RegionTargets::mouse_region`] adds a whole-region pointer target for blank pane space or
///   wheel routing.
/// - [`RegionTargets::z`] assigns one z-order to generated layout and pointer targets.
/// - [`RegionTargets::build`] produces the [`FrameSnapshot`] stored after rendering.
///
/// # Examples
///
/// Build aligned frame-local data from a row of app-owned commands:
///
/// ```rust
/// use ratatui_core::layout::{Constraint, Rect};
/// use ratatui_layout::{FrameTargets, Row};
///
/// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// enum Command {
///     Edit,
///     Save,
/// }
///
/// let command_slots = [
///     (Command::Edit, Constraint::Length(6)),
///     (Command::Save, Constraint::Length(6)),
/// ];
/// let row = Row::named(command_slots)
///     .spacing(1)
///     .regions(Rect::new(0, 0, 13, 1));
/// let frame = FrameTargets::from_regions(row, 10)
///     .disabled(|command| command == Command::Save)
///     .build();
///
/// assert_eq!(frame.focus.targets()[0].id, Command::Edit);
/// assert!(frame.focus.targets()[1].disabled);
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RegionTargets<Id = usize> {
    area: Rect,
    focus_start: u16,
    mouse_region: Option<(Id, Rect)>,
    targets: Vec<FrameTargetSlot<Id>>,
}

impl<Id> RegionTargets<Id> {
    fn new(plan: Regions<Id>, focus_start: u16) -> Self {
        let area = plan.area();
        let targets = plan
            .into_iter()
            .map(|region| FrameTargetSlot {
                region,
                focusable: true,
                mouseable: true,
                disabled: false,
            })
            .collect();
        Self {
            area,
            focus_start,
            mouse_region: None,
            targets,
        }
    }

    /// Marks regions disabled for focus traversal and pointer routing.
    ///
    /// Disabled regions remain renderable in the region set. Focus traversal and pointer hit
    /// testing skip their generated targets.
    ///
    /// # Examples
    ///
    /// Keep a `Save` command visible while preventing activation when the document is clean:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::{FrameTargets, Row};
    ///
    /// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    /// enum Command {
    ///     Open,
    ///     Save,
    /// }
    ///
    /// let dirty = false;
    /// let command_slots = [
    ///     (Command::Open, Constraint::Length(6)),
    ///     (Command::Save, Constraint::Length(6)),
    /// ];
    /// let plan = Row::named(command_slots).regions(Rect::new(0, 0, 12, 1));
    /// let frame = FrameTargets::from_regions(plan, 0)
    ///     .disabled(|command| command == Command::Save && !dirty)
    ///     .build();
    ///
    /// assert!(frame.focus.targets()[1].disabled);
    /// assert!(frame.route_click((7, 0)).is_none());
    /// ```
    #[must_use = "method returns the modified builder"]
    pub fn disabled(mut self, disabled: impl Fn(Id) -> bool) -> Self
    where
        Id: Copy,
    {
        for target in &mut self.targets {
            target.disabled = disabled(target.region.id);
        }
        self
    }

    /// Chooses which regions participate in keyboard traversal.
    ///
    /// Use this for headers, labels, pointer-only segments, or any region that should be visible
    /// without becoming a page-level focus target.
    ///
    /// # Examples
    ///
    /// Make only body rows focusable while keeping the header in the layout:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameTargets, Region, Regions};
    ///
    /// let plan = Regions::from_regions(
    ///     Rect::new(0, 0, 20, 2),
    ///     [
    ///         Region::new(None, Rect::new(0, 0, 20, 1)),
    ///         Region::new(Some(0), Rect::new(0, 1, 20, 1)),
    ///     ],
    /// );
    /// let frame = FrameTargets::from_regions(plan, 0)
    ///     .focusable(|row| row.is_some())
    ///     .build();
    ///
    /// assert_eq!(frame.layout.regions().len(), 2);
    /// assert_eq!(frame.focus.targets().len(), 1);
    /// ```
    #[must_use = "method returns the modified builder"]
    pub fn focusable(mut self, focusable: impl Fn(Id) -> bool) -> Self
    where
        Id: Copy,
    {
        for target in &mut self.targets {
            target.focusable = focusable(target.region.id);
        }
        self
    }

    /// Chooses which regions receive pointer routing.
    ///
    /// Non-pointer-routable regions remain renderable and can still be found by broad
    /// [`FrameSnapshot::route_position`] geometry queries.
    ///
    /// # Examples
    ///
    /// Keep a field focusable while routing clicks only to its explicit edit button:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameTargets, Region, Regions};
    ///
    /// let plan = Regions::from_regions(
    ///     Rect::new(0, 0, 20, 1),
    ///     [
    ///         Region::new("field", Rect::new(0, 0, 12, 1)),
    ///         Region::new("edit", Rect::new(13, 0, 6, 1)),
    ///     ],
    /// );
    /// let frame = FrameTargets::from_regions(plan, 0)
    ///     .mouseable(|id| id == "edit")
    ///     .build();
    ///
    /// assert_eq!(frame.focus.targets().len(), 2);
    /// assert_eq!(frame.route_click((14, 0)).unwrap().id, "edit");
    /// ```
    #[must_use = "method returns the modified builder"]
    pub fn mouseable(mut self, mouseable: impl Fn(Id) -> bool) -> Self
    where
        Id: Copy,
    {
        for target in &mut self.targets {
            target.mouseable = mouseable(target.region.id);
        }
        self
    }

    /// Adds a whole-region pointer target before region targets.
    ///
    /// Scrollable panes use this so wheel events route over blank viewport space.
    ///
    /// # Examples
    ///
    /// Route wheel events to a pane even when the pointer is below the last visible row:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameTargets, Regions};
    ///
    /// let pane = Rect::new(0, 0, 20, 5);
    /// let frame = FrameTargets::from_regions(Regions::new(pane), 0)
    ///     .mouse_region("details", pane)
    ///     .build();
    ///
    /// assert_eq!(frame.route_scroll((10, 4)).unwrap().id, "details");
    /// ```
    #[must_use = "method returns the modified builder"]
    pub fn mouse_region(mut self, id: Id, area: Rect) -> Self {
        self.mouse_region = Some((id, area));
        self
    }

    /// Assigns one z-order to generated layout regions and pointer targets.
    ///
    /// Use this for dialogs, overlays, popups, and command palettes.
    ///
    /// # Examples
    ///
    /// Make a popup button route before base content at the same position:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameTargets, Region, Regions};
    ///
    /// let popup = Regions::from_regions(
    ///     Rect::new(0, 0, 8, 1),
    ///     [Region::new("close", Rect::new(0, 0, 8, 1))],
    /// );
    /// let frame = FrameTargets::from_regions(popup, 0).z(20).build();
    ///
    /// assert_eq!(frame.layout.regions()[0].z, 20);
    /// assert_eq!(frame.mouse.targets()[0].z, 20);
    /// ```
    #[must_use = "method returns the modified builder"]
    pub fn z(mut self, z: u16) -> Self {
        for target in &mut self.targets {
            target.region.z = z;
        }
        self
    }

    /// Builds a frame snapshot from the configured regions and policies.
    ///
    /// The layout keeps every region. Focus and pointer target collections include only regions
    /// enabled by their policies, and disabled generated targets are skipped by traversal and
    /// hit testing.
    ///
    /// # Examples
    ///
    /// Build a frame snapshot for a pointer-only segmented control:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Constraint, Rect};
    /// use ratatui_layout::{FrameTargets, Row};
    ///
    /// let segment_slots = [
    ///     ("queued", Constraint::Length(8)),
    ///     ("done", Constraint::Length(6)),
    /// ];
    /// let plan = Row::named(segment_slots).regions(Rect::new(0, 0, 14, 1));
    /// let frame = FrameTargets::from_regions(plan, 0)
    ///     .focusable(|_| false)
    ///     .build();
    ///
    /// assert!(frame.focus.targets().is_empty());
    /// assert_eq!(frame.route_click((1, 0)).unwrap().id, "queued");
    /// ```
    pub fn build(self) -> FrameSnapshot<Id>
    where
        Id: Copy,
    {
        let mut layout = Regions::new(self.area);
        let mut mouse = if let Some((id, area)) = self.mouse_region {
            PointerTargets::new().region(id, area)
        } else {
            PointerTargets::new()
        };
        let mut focus = FocusTargets::new();

        for (order, target) in self.targets.into_iter().enumerate() {
            let region = target.region;
            layout.push(region);
            if target.mouseable {
                mouse = mouse.target(PointerTarget::from_region(region).disabled(target.disabled));
            }
            if target.focusable {
                let order = self.focus_start + order as u16;
                focus =
                    focus.target(FocusTarget::from_region(region, order).disabled(target.disabled));
            }
        }

        FrameSnapshot::from_layout(layout).mouse(mouse).focus(focus)
    }
}

impl<Out> FrameTargets<Out>
where
    Out: Copy,
{
    /// Builds a frame snapshot for one focusable and mouseable region.
    ///
    /// This is useful for panes whose whole viewport is one target: keyboard focus scrolls the pane
    /// and wheel events route over the same rectangle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::FrameTargets;
    ///
    /// let frame =
    ///     FrameTargets::new(Rect::new(0, 0, 10, 3), 0).region("details", Rect::new(0, 0, 10, 3));
    ///
    /// assert_eq!(frame.route_click((2, 2)).unwrap().id, "details");
    /// ```
    pub fn region(self, id: Out, area: Rect) -> FrameSnapshot<Out> {
        self.build_focusable([Region::new(id, area)], |id| id)
    }

    /// Builds a frame snapshot when every region is enabled and focusable.
    ///
    /// Use this for dialog fields, menu rows, and ordinary button groups where every visible region
    /// participates in keyboard traversal and pointer routing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameTargets, Region};
    ///
    /// let frame = FrameTargets::new(Rect::new(0, 0, 8, 1), 0)
    ///     .build_focusable([Region::new("save", Rect::new(0, 0, 8, 1))], |id| id);
    ///
    /// assert_eq!(frame.focus.targets()[0].id, "save");
    /// ```
    pub fn build_focusable<Id>(
        self,
        regions: impl IntoIterator<Item = Region<Id>>,
        map_id: impl Fn(Id) -> Out,
    ) -> FrameSnapshot<Out>
    where
        Id: Copy,
    {
        self.build(regions, map_id, |_| true, |_| false)
    }

    /// Builds a frame snapshot with a custom focus predicate and no disabled regions.
    ///
    /// Tables use this when header cells should remain pointer targets but body cells are the only
    /// keyboard traversal targets.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameTargets, Region};
    ///
    /// let regions = [
    ///     Region::new(None, Rect::new(0, 0, 10, 1)),
    ///     Region::new(Some(0), Rect::new(0, 1, 10, 1)),
    /// ];
    /// let frame = FrameTargets::new(Rect::new(0, 0, 10, 2), 0).build_with_focus(
    ///     regions,
    ///     |row| row,
    ///     |row| row.is_some(),
    /// );
    ///
    /// assert_eq!(frame.mouse.targets().len(), 2);
    /// assert_eq!(frame.focus.targets().len(), 1);
    /// ```
    pub fn build_with_focus<Id>(
        self,
        regions: impl IntoIterator<Item = Region<Id>>,
        map_id: impl Fn(Id) -> Out,
        focusable: impl Fn(Id) -> bool,
    ) -> FrameSnapshot<Out>
    where
        Id: Copy,
    {
        self.build(regions, map_id, focusable, |_| false)
    }

    /// Builds a frame snapshot when every region is focusable but some regions are disabled.
    ///
    /// Disabled regions remain in the region set so they can still be rendered, but their pointer
    /// and focus targets are marked disabled for routing and traversal.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameTargets, Region};
    ///
    /// let frame = FrameTargets::new(Rect::new(0, 0, 8, 1), 0).build_with_disabled(
    ///     [Region::new("save", Rect::new(0, 0, 8, 1))],
    ///     |id| id,
    ///     |_| true,
    /// );
    ///
    /// assert!(frame.focus.targets()[0].disabled);
    /// assert!(frame.mouse.hit_test((1, 0)).is_none());
    /// assert_eq!(frame.layout.hit_test((1, 0)).unwrap().id, "save");
    /// ```
    pub fn build_with_disabled<Id>(
        self,
        regions: impl IntoIterator<Item = Region<Id>>,
        map_id: impl Fn(Id) -> Out,
        disabled: impl Fn(Id) -> bool,
    ) -> FrameSnapshot<Out>
    where
        Id: Copy,
    {
        self.build(regions, map_id, |_| true, disabled)
    }

    /// Builds a frame snapshot from local regions.
    ///
    /// `map_id` turns component-local region ids into app-level routing ids. `focusable` decides
    /// whether the region joins keyboard traversal. `disabled` keeps disabled controls visible
    /// while removing them from activation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameTargets, Region};
    ///
    /// let frame = FrameTargets::new(Rect::new(0, 0, 10, 2), 5).build(
    ///     [
    ///         Region::new(0, Rect::new(0, 0, 10, 1)),
    ///         Region::new(1, Rect::new(0, 1, 10, 1)),
    ///     ],
    ///     |row| ("row", row),
    ///     |row| row == 1,
    ///     |_| false,
    /// );
    ///
    /// assert_eq!(frame.layout.regions().len(), 2);
    /// assert_eq!(frame.focus.targets()[0].id, ("row", 1));
    /// ```
    pub fn build<Id>(
        self,
        regions: impl IntoIterator<Item = Region<Id>>,
        map_id: impl Fn(Id) -> Out,
        focusable: impl Fn(Id) -> bool,
        disabled: impl Fn(Id) -> bool,
    ) -> FrameSnapshot<Out>
    where
        Id: Copy,
    {
        let mut layout = Regions::new(self.area);
        let mut mouse = if let Some((id, area)) = self.mouse_region {
            PointerTargets::new().region(id, area)
        } else {
            PointerTargets::new()
        };
        let mut focus = FocusTargets::new();

        for (order, region) in regions.into_iter().enumerate() {
            let id = map_id(region.id);
            let disabled = disabled(region.id);
            layout.push(Region::new(id, region.area).z(self.z));
            mouse = mouse.target(
                PointerTarget::new(id, region.area)
                    .z(self.z)
                    .disabled(disabled),
            );
            if focusable(region.id) {
                let order = self.focus_start + order as u16;
                focus = focus.target(FocusTarget::new(id, region.area, order).disabled(disabled));
            }
        }

        FrameSnapshot::from_layout(layout).mouse(mouse).focus(focus)
    }
}

impl<Id> FrameSnapshot<Id> {
    /// Creates an empty frame snapshot for a solved area.
    ///
    /// Use this as a neutral previous-frame value before the first draw, or when a component has an
    /// area but no visible children.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::FrameSnapshot;
    ///
    /// let frame = FrameSnapshot::<()>::new(Rect::new(0, 0, 80, 24));
    /// assert!(frame.layout.is_empty());
    /// assert!(frame.route_position((0, 0)).is_none());
    /// ```
    pub const fn new(area: Rect) -> Self {
        Self {
            layout: Regions::new(area),
            focus: FocusTargets::new(),
            mouse: PointerTargets::new(),
            cursor: CursorRequests::new(),
        }
    }

    /// Creates a frame snapshot from a region set.
    ///
    /// This is the common incremental path: start with geometry, then attach focus, pointer, or
    /// cursor requests only when the component produces them.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameSnapshot, Region, Regions};
    ///
    /// let layout = Regions::from_regions(
    ///     Rect::new(0, 0, 10, 1),
    ///     [Region::new("tab", Rect::new(0, 0, 10, 1))],
    /// );
    /// let frame = FrameSnapshot::from_layout(layout);
    ///
    /// assert_eq!(frame.route_position((1, 0)).unwrap().id, "tab");
    /// ```
    pub const fn from_layout(layout: Regions<Id>) -> Self {
        Self {
            layout,
            focus: FocusTargets::new(),
            mouse: PointerTargets::new(),
            cursor: CursorRequests::new(),
        }
    }

    /// Sets the focus target collection.
    ///
    /// This replaces the current focus target collection rather than merging. Use
    /// [`FrameSnapshot::merge`] when a child frame should be appended to existing data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FocusTarget, FocusTargets, FrameSnapshot};
    ///
    /// let frame = FrameSnapshot::new(Rect::new(0, 0, 10, 1)).focus(FocusTargets::from_targets([
    ///     FocusTarget::new("field", Rect::new(0, 0, 10, 1), 0),
    /// ]));
    ///
    /// assert_eq!(frame.focus.first_enabled().unwrap().id, "field");
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn focus(mut self, focus: FocusTargets<Id>) -> Self {
        self.focus = focus;
        self
    }

    /// Sets the pointer target collection.
    ///
    /// This replaces the current pointer target collection rather than deriving one from layout
    /// regions. Keeping the two separate lets pointer routing skip disabled or non-interactive
    /// regions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameSnapshot, PointerTarget, PointerTargets};
    ///
    /// let frame = FrameSnapshot::new(Rect::new(0, 0, 10, 1)).mouse(PointerTargets::from_targets([
    ///     PointerTarget::new("button", Rect::new(0, 0, 10, 1)),
    /// ]));
    ///
    /// assert_eq!(frame.route_position((1, 0)).unwrap().id, "button");
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn mouse(mut self, mouse: PointerTargets<Id>) -> Self {
        self.mouse = mouse;
        self
    }

    /// Sets the cursor request list.
    ///
    /// This replaces the current cursor request list. During composition, [`FrameSnapshot::merge`]
    /// preserves render order so later visible cursor requests can win.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::{Position, Rect};
    /// use ratatui_layout::{CursorRequest, CursorRequests, FrameSnapshot};
    ///
    /// let frame = FrameSnapshot::<()>::new(Rect::default())
    ///     .cursor(CursorRequests::new().request(CursorRequest::visible(Position::new(4, 2))));
    ///
    /// assert_eq!(
    ///     frame.cursor.final_cursor(),
    ///     Some(CursorRequest::visible(Position::new(4, 2)))
    /// );
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn cursor(mut self, cursor: CursorRequests) -> Self {
        self.cursor = cursor;
        self
    }

    /// Adds one pointer target to the frame's pointer target collection.
    ///
    /// This is the aggregate equivalent of [`PointerTargets::target`]. It is useful when a
    /// component already has a [`FrameSnapshot`] and wants to add a pointer-only region without
    /// constructing a temporary pointer target collection.
    ///
    /// # Examples
    ///
    /// Add a pane-level target after building layout regions for visible rows:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameSnapshot, PointerTarget};
    ///
    /// let frame = FrameSnapshot::new(Rect::new(0, 0, 20, 5))
    ///     .mouse_target(PointerTarget::new("pane", Rect::new(0, 0, 20, 5)));
    ///
    /// assert_eq!(frame.route_position((2, 4)).unwrap().id, "pane");
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn mouse_target(mut self, target: PointerTarget<Id>) -> Self {
        self.mouse = self.mouse.target(target);
        self
    }

    /// Adds a whole-region pointer target to the frame.
    ///
    /// Use this for scrollable panes, blank viewport space, or any region where pointer input
    /// belongs to the container rather than a visible child region. It records ordinary pointer
    /// routing data; the app still decides whether a routed hit means scroll, click, hover, or
    /// another semantic action.
    ///
    /// # Examples
    ///
    /// Register a scrollable list viewport before adding row targets:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::FrameSnapshot;
    ///
    /// let frame = FrameSnapshot::new(Rect::new(0, 0, 20, 5))
    ///     .scroll_region("list-pane", Rect::new(0, 0, 20, 5));
    ///
    /// assert_eq!(frame.route_scroll((1, 4)).unwrap().id, "list-pane");
    /// ```
    #[must_use = "method returns the modified value"]
    pub fn scroll_region(mut self, id: Id, area: Rect) -> Self {
        self.mouse = self.mouse.region(id, area);
        self
    }

    /// Merges another frame snapshot into this one.
    ///
    /// Later merged layout and pointer targets preserve normal z-order tie-breaking because their
    /// regions and targets are appended after existing ones.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameSnapshot, Region, Regions};
    ///
    /// let base = FrameSnapshot::from_layout(Regions::from_regions(
    ///     Rect::new(0, 0, 10, 1),
    ///     [Region::new("base", Rect::new(0, 0, 10, 1))],
    /// ));
    /// let overlay = FrameSnapshot::from_layout(Regions::from_regions(
    ///     Rect::new(0, 0, 10, 1),
    ///     [Region::new("overlay", Rect::new(0, 0, 10, 1)).z(1)],
    /// ));
    ///
    /// assert_eq!(
    ///     base.merge(overlay).route_position((0, 0)).unwrap().id,
    ///     "overlay"
    /// );
    /// ```
    #[must_use = "method returns the merged plan"]
    pub fn merge(mut self, other: Self) -> Self {
        self.layout = self.layout.merge(other.layout);
        self.focus = self.focus.merge(other.focus);
        self.mouse = self.mouse.merge(other.mouse);
        self.cursor = self.cursor.merge(other.cursor);
        self
    }

    /// Places a child frame and merges its visible data into this frame.
    ///
    /// This is a convenience for the common parent/child composition sequence: a child solves local
    /// coordinates, the parent translates those coordinates to the child's screen position, clips
    /// them to a viewport, and then merges them into the parent aggregate. Use the lower-level
    /// [`FrameSnapshot::translate`], [`FrameSnapshot::clip_to`], and [`FrameSnapshot::merge`]
    /// methods when each step needs to be inspected separately.
    ///
    /// # Examples
    ///
    /// Place a child-local plan inside a parent viewport:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameSnapshot, Region, Regions};
    ///
    /// let child = FrameSnapshot::from_layout(Regions::from_regions(
    ///     Rect::new(0, 0, 10, 1),
    ///     [Region::new("row", Rect::new(0, 0, 10, 1))],
    /// ));
    /// let frame =
    ///     FrameSnapshot::new(Rect::new(0, 0, 20, 5)).merge_child(child, 4, 2, Rect::new(0, 0, 20, 5));
    ///
    /// assert_eq!(frame.route_position((5, 2)).unwrap().id, "row");
    /// ```
    #[must_use = "method returns the merged plan"]
    pub fn merge_child(self, child: Self, dx: i16, dy: i16, clip: Rect) -> Self {
        self.merge(child.translate(dx, dy).clip_to(clip))
    }

    /// Places a child frame in an absolute screen area and merges it into this frame.
    ///
    /// This is the common form of [`FrameSnapshot::merge_child`] when the parent has already solved
    /// the child area. The child is translated by `area.x` and `area.y`, then clipped to
    /// `area`, so layout, focus, pointer, and cursor requests match the region the child actually
    /// rendered into.
    ///
    /// # Examples
    ///
    /// Compose a local child plan after solving page areas:
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameSnapshot, Region, Regions};
    ///
    /// let child_area = Rect::new(4, 2, 10, 1);
    /// let child = FrameSnapshot::from_layout(Regions::from_regions(
    ///     Rect::new(0, 0, child_area.width, child_area.height),
    ///     [Region::new("field", Rect::new(0, 0, 10, 1))],
    /// ));
    /// let frame = FrameSnapshot::new(Rect::new(0, 0, 20, 5)).place_child(child, child_area);
    ///
    /// assert_eq!(frame.route_position((5, 2)).unwrap().id, "field");
    /// ```
    #[must_use = "method returns the merged plan"]
    pub fn place_child(self, child: Self, area: Rect) -> Self {
        self.merge_child(child, area.x as i16, area.y as i16, area)
    }

    /// Moves child-local frame-local data by a signed offset.
    ///
    /// Use this when a child solved layout, focus, pointer, and cursor requests in local
    /// coordinates and the parent knows where that child was placed on the terminal. Cursor
    /// requests move with the geometry data so focused child inputs can report local cursor
    /// positions during render.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameSnapshot, Region, Regions};
    ///
    /// let frame = FrameSnapshot::from_layout(Regions::from_regions(
    ///     Rect::new(0, 0, 5, 1),
    ///     [Region::new("field", Rect::new(0, 0, 5, 1))],
    /// ))
    /// .translate(10, 3);
    ///
    /// assert_eq!(frame.route_position((11, 3)).unwrap().id, "field");
    /// ```
    #[must_use = "method returns the translated plan"]
    pub fn translate(mut self, dx: i16, dy: i16) -> Self {
        self.layout = self.layout.translate(dx, dy);
        self.focus = self.focus.translate(dx, dy);
        self.mouse = self.mouse.translate(dx, dy);
        self.cursor = self.cursor.translate(dx, dy);
        self
    }

    /// Clips frame-local data to a viewport.
    ///
    /// Use this when a child plan should expose only the regions visible through a parent
    /// container or scroll viewport. Layout, focus, and pointer regions are clipped or removed.
    /// Cursor requests outside the viewport are hidden so a scrolled-out input does not move the
    /// terminal cursor.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameSnapshot, Region, Regions};
    ///
    /// let frame = FrameSnapshot::from_layout(Regions::from_regions(
    ///     Rect::new(0, 0, 10, 3),
    ///     [Region::new("row", Rect::new(0, 0, 10, 3))],
    /// ))
    /// .clip_to(Rect::new(0, 1, 10, 1));
    ///
    /// assert_eq!(frame.layout.regions()[0].area, Rect::new(0, 1, 10, 1));
    /// ```
    #[must_use = "method returns the clipped plan"]
    pub fn clip_to(mut self, viewport: Rect) -> Self {
        self.layout = self.layout.clip_to(viewport);
        self.focus = self.focus.clip_to(viewport);
        self.mouse = self.mouse.clip_to(viewport);
        self.cursor = self.cursor.clip_to(viewport);
        self
    }

    /// Maps ids for layout, focus, and pointer target collections while preserving cursor requests.
    ///
    /// This is how a child component can use small local ids internally and then lift them into a
    /// parent enum or stable app id before merging.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameSnapshot, Region, Regions};
    ///
    /// let frame = FrameSnapshot::from_layout(Regions::from_regions(
    ///     Rect::new(0, 0, 5, 1),
    ///     [Region::new(0, Rect::new(0, 0, 5, 1))],
    /// ))
    /// .map_id(|id| ("child", id));
    ///
    /// assert_eq!(frame.route_position((1, 0)).unwrap().id, ("child", 0));
    /// ```
    pub fn map_id<NextId, F>(self, mut map: F) -> FrameSnapshot<NextId>
    where
        F: FnMut(Id) -> NextId,
    {
        FrameSnapshot {
            layout: self.layout.map_id(&mut map),
            focus: self.focus.map_id(&mut map),
            mouse: self.mouse.map_id(&mut map),
            cursor: self.cursor,
        }
    }
}

impl<Id: Copy> FrameSnapshot<Id> {
    /// Routes a position through the pointer target collection, falling back to layout hit testing.
    ///
    /// Pointer targets are the primary routing surface because they can represent disabled state
    /// and pointer-specific z-order. The layout fallback keeps simple examples useful when no
    /// explicit pointer target collection was built.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameSnapshot, PointerTarget, PointerTargets, Region, Regions};
    ///
    /// let frame = FrameSnapshot::from_layout(Regions::from_regions(
    ///     Rect::new(0, 0, 10, 1),
    ///     [Region::new("fallback", Rect::new(0, 0, 10, 1))],
    /// ))
    /// .mouse(PointerTargets::from_targets([PointerTarget::new(
    ///     "explicit",
    ///     Rect::new(0, 0, 10, 1),
    /// )]));
    ///
    /// assert_eq!(frame.route_position((1, 0)).unwrap().id, "explicit");
    /// ```
    pub fn route_position<P: Into<Position>>(&self, position: P) -> Option<Hit<Id>> {
        let position = position.into();
        self.mouse
            .hit_test(position)
            .or_else(|| self.layout.hit_test(position))
    }

    fn route_mouse_event<P: Into<Position>>(&self, position: P) -> Option<Hit<Id>> {
        let position = position.into();
        if self.mouse.targets().is_empty() {
            self.layout.hit_test(position)
        } else {
            self.mouse.hit_test(position)
        }
    }

    /// Routes a wheel position through the previous frame.
    ///
    /// When the frame has explicit pointer targets, wheel routing uses those targets so disabled or
    /// non-pointer regions do not fall through to layout geometry. When the frame has no pointer
    /// targets, it falls back to [`FrameSnapshot::route_position`] behavior and routes through
    /// layout regions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::FrameSnapshot;
    ///
    /// let frame =
    ///     FrameSnapshot::new(Rect::new(0, 0, 20, 5)).scroll_region("details", Rect::new(0, 0, 20, 5));
    ///
    /// assert_eq!(frame.route_scroll((10, 4)).unwrap().id, "details");
    /// ```
    pub fn route_scroll<P: Into<Position>>(&self, position: P) -> Option<Hit<Id>> {
        self.route_mouse_event(position)
    }

    /// Routes a click position through the previous frame.
    ///
    /// When the frame has explicit pointer targets, click routing uses those targets so disabled or
    /// non-pointer regions do not fall through to layout geometry. When the frame has no pointer
    /// targets, it falls back to [`FrameSnapshot::route_position`] behavior and routes through
    /// layout regions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::{FrameSnapshot, Region, Regions};
    ///
    /// let frame = FrameSnapshot::from_layout(Regions::from_regions(
    ///     Rect::new(0, 0, 10, 1),
    ///     [Region::new("save", Rect::new(0, 0, 10, 1))],
    /// ));
    ///
    /// assert_eq!(frame.route_click((4, 0)).unwrap().id, "save");
    /// ```
    pub fn route_click<P: Into<Position>>(&self, position: P) -> Option<Hit<Id>> {
        self.route_mouse_event(position)
    }

    /// Routes a hover position through the previous frame.
    ///
    /// When the frame has explicit pointer targets, hover routing uses those targets so disabled or
    /// non-pointer regions do not fall through to layout geometry. When the frame has no pointer
    /// targets, it falls back to [`FrameSnapshot::route_position`] behavior and routes through
    /// layout regions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::layout::Rect;
    /// use ratatui_layout::FrameSnapshot;
    ///
    /// let frame =
    ///     FrameSnapshot::new(Rect::new(0, 0, 10, 3)).scroll_region("pane", Rect::new(0, 0, 10, 3));
    ///
    /// assert_eq!(frame.route_hover((5, 2)).unwrap().id, "pane");
    /// ```
    pub fn route_hover<P: Into<Position>>(&self, position: P) -> Option<Hit<Id>> {
        self.route_mouse_event(position)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::cursor::CursorRequest;
    use crate::focus::FocusTarget;
    use crate::pointer::PointerTarget;
    use crate::regions::Region;

    #[test]
    fn merge_child_plans_preserves_z_order() {
        let parent = FrameSnapshot::from_layout(Regions::from_regions(
            Rect::new(0, 0, 5, 1),
            [Region::new("parent", Rect::new(0, 0, 5, 1)).z(1)],
        ))
        .mouse(PointerTargets::from_targets([PointerTarget::new(
            "parent",
            Rect::new(0, 0, 5, 1),
        )
        .z(1)]));
        let child = FrameSnapshot::from_layout(Regions::from_regions(
            Rect::new(0, 0, 5, 1),
            [Region::new("child", Rect::new(0, 0, 5, 1)).z(2)],
        ))
        .mouse(PointerTargets::from_targets([PointerTarget::new(
            "child",
            Rect::new(0, 0, 5, 1),
        )
        .z(2)]));

        let merged = parent.merge(child);

        assert_eq!(
            merged.route_position((0, 0)).map(|hit| hit.id),
            Some("child")
        );
    }

    #[test]
    fn maps_translates_and_clips_child_regions() {
        let frame = FrameSnapshot::from_layout(Regions::from_regions(
            Rect::new(0, 0, 4, 2),
            [Region::new(1, Rect::new(1, 0, 3, 2))],
        ))
        .focus(FocusTargets::from_targets([FocusTarget::new(
            1,
            Rect::new(1, 0, 3, 2),
            0,
        )]))
        .mouse(PointerTargets::from_targets([PointerTarget::new(
            1,
            Rect::new(1, 0, 3, 2),
        )]))
        .translate(2, 1)
        .clip_to(Rect::new(0, 0, 5, 3))
        .map_id(|id| if id == 1 { "field" } else { "other" });

        assert_eq!(frame.layout.regions()[0].id, "field");
        assert_eq!(frame.layout.regions()[0].area, Rect::new(3, 1, 2, 2));
        assert_eq!(frame.focus.targets()[0].area, Rect::new(3, 1, 2, 2));
        assert_eq!(frame.mouse.targets()[0].area, Rect::new(3, 1, 2, 2));
    }

    #[test]
    fn merged_cursor_requests_keep_last_visible() {
        let first = FrameSnapshot::<()>::new(Rect::default())
            .cursor(CursorRequests::new().request(CursorRequest::visible(Position::new(1, 1))));
        let second = FrameSnapshot::<()>::new(Rect::default())
            .cursor(CursorRequests::new().request(CursorRequest::visible(Position::new(2, 2))));

        assert_eq!(
            first.merge(second).cursor.final_cursor(),
            Some(CursorRequest::visible(Position::new(2, 2)))
        );
    }

    #[test]
    fn translated_frame_moves_cursor_requests() {
        let frame = FrameSnapshot::<()>::new(Rect::default())
            .cursor(CursorRequests::new().request(CursorRequest::visible(Position::new(2, 1))))
            .translate(10, 3);

        assert_eq!(
            frame.cursor.final_cursor(),
            Some(CursorRequest::visible(Position::new(12, 4)))
        );
    }

    #[test]
    fn clipped_frame_hides_cursor_requests_outside_viewport() {
        let frame = FrameSnapshot::<()>::new(Rect::default())
            .cursor(CursorRequests::new().request(CursorRequest::visible(Position::new(8, 1))))
            .clip_to(Rect::new(0, 0, 5, 3));

        assert_eq!(frame.cursor.final_cursor(), None);
    }

    #[test]
    fn place_child_uses_area_offset_and_clip() {
        let child = FrameSnapshot::from_layout(Regions::from_regions(
            Rect::new(0, 0, 10, 3),
            [Region::new("row", Rect::new(0, 0, 10, 3))],
        ))
        .cursor(CursorRequests::new().request(CursorRequest::visible(Position::new(2, 1))));

        let frame =
            FrameSnapshot::new(Rect::new(0, 0, 20, 5)).place_child(child, Rect::new(4, 2, 10, 2));

        assert_eq!(frame.route_position((5, 2)).map(|hit| hit.id), Some("row"));
        assert_eq!(
            frame.cursor.final_cursor(),
            Some(CursorRequest::visible(Position::new(6, 3)))
        );
    }

    #[test]
    fn scroll_region_routes_blank_pane_space() {
        let frame = FrameSnapshot::new(Rect::new(0, 0, 20, 5))
            .scroll_region("details", Rect::new(0, 0, 20, 5));

        assert_eq!(
            frame.route_scroll((10, 4)).map(|hit| hit.id),
            Some("details")
        );
    }

    #[test]
    fn frame_targets_build_layout_mouse_and_focus_from_regions() {
        let frame = FrameTargets::new(Rect::new(0, 0, 10, 2), 40).build_with_focus(
            [
                Region::new(None, Rect::new(0, 0, 10, 1)),
                Region::new(Some(0), Rect::new(0, 1, 10, 1)),
            ],
            |row| row,
            |row| row.is_some(),
        );

        assert_eq!(frame.layout.regions().len(), 2);
        assert_eq!(frame.mouse.targets().len(), 2);
        assert_eq!(frame.focus.targets().len(), 1);
        assert_eq!(frame.focus.targets()[0].order, 41);
    }

    #[test]
    fn frame_targets_preserve_disabled_visual_regions() {
        let frame = FrameTargets::new(Rect::new(0, 0, 10, 1), 0).build_with_disabled(
            [Region::new("save", Rect::new(0, 0, 10, 1))],
            |id| id,
            |_| true,
        );

        assert_eq!(
            frame.layout.hit_test((1, 0)).map(|hit| hit.id),
            Some("save")
        );
        assert!(frame.mouse.hit_test((1, 0)).is_none());
        assert!(frame.focus.first_enabled().is_none());
    }

    #[test]
    fn frame_target_plan_builds_from_layout_plan() {
        let plan = Regions::from_regions(
            Rect::new(0, 0, 12, 1),
            [
                Region::new("open", Rect::new(0, 0, 6, 1)),
                Region::new("save", Rect::new(6, 0, 6, 1)),
            ],
        );

        let frame = FrameTargets::from_regions(plan, 20)
            .disabled(|id| id == "save")
            .build();

        assert_eq!(
            frame.layout.hit_test((7, 0)).map(|hit| hit.id),
            Some("save")
        );
        assert_eq!(frame.focus.targets()[0].id, "open");
        assert_eq!(frame.focus.targets()[0].order, 20);
        assert!(frame.focus.targets()[1].disabled);
        assert!(frame.mouse.hit_test((7, 0)).is_none());
    }

    #[test]
    fn frame_target_plan_separates_focusable_and_mouseable_policy() {
        let plan = Regions::from_regions(
            Rect::new(0, 0, 20, 2),
            [
                Region::new("header", Rect::new(0, 0, 20, 1)),
                Region::new("row", Rect::new(0, 1, 20, 1)),
            ],
        );

        let frame = FrameTargets::from_regions(plan, 0)
            .focusable(|id| id == "row")
            .mouseable(|id| id == "header")
            .build();

        assert_eq!(frame.focus.targets().len(), 1);
        assert_eq!(frame.focus.targets()[0].id, "row");
        assert_eq!(frame.route_click((1, 0)).map(|hit| hit.id), Some("header"));
        assert_eq!(frame.route_click((1, 1)).map(|hit| hit.id), None);
        assert_eq!(frame.route_position((1, 1)).map(|hit| hit.id), Some("row"));
        assert!(frame.mouse.hit_test((1, 1)).is_none());
    }

    #[test]
    fn frame_target_plan_applies_z_and_mouse_region() {
        let pane = Rect::new(0, 0, 20, 5);
        let plan = Regions::from_regions(pane, [Region::new("button", Rect::new(0, 0, 6, 1))]);

        let frame = FrameTargets::from_regions(plan, 0)
            .mouse_region("pane", pane)
            .z(10)
            .build();

        assert_eq!(frame.layout.regions()[0].z, 10);
        assert_eq!(frame.mouse.targets()[1].z, 10);
        assert_eq!(frame.route_scroll((10, 4)).map(|hit| hit.id), Some("pane"));
        assert_eq!(frame.route_click((1, 0)).map(|hit| hit.id), Some("button"));
    }

    #[test]
    fn merge_child_places_and_clips_local_plan() {
        let child = FrameSnapshot::from_layout(Regions::from_regions(
            Rect::new(0, 0, 10, 3),
            [Region::new("row", Rect::new(0, 0, 10, 3))],
        ));

        let frame = FrameSnapshot::new(Rect::new(0, 0, 20, 5)).merge_child(
            child,
            4,
            2,
            Rect::new(0, 0, 20, 4),
        );

        assert_eq!(
            frame.layout.regions(),
            &[
                Region::new("row", Rect::new(4, 2, 10, 2)).clip(crate::regions::Clip {
                    bottom: 1,
                    ..crate::regions::Clip::default()
                })
            ]
        );
    }
}
