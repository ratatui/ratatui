//! Guide pages for the frame-local UI coordination model.
//!
//! These modules are documentation-only. They describe how the experimental primitives fit
//! together, where normal Ratatui APIs remain simpler, and which ideas are current behavior versus
//! future direction.
//!
//! Maintainers updating these docs should also read [`crate::docs::documentation_review`]. It
//! records the standard used for this experimental crate: links should help navigation, but each
//! page and type should still explain the local problem well enough that readers understand why the
//! linked concept matters before they click away.
//!
//! # Examples
//!
//! The guides describe why a render pass can return data that the next input event uses:
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::frame::FrameSnapshot;
//! use ratatui_layout::pointer::{PointerTarget, PointerTargets};
//!
//! let frame = FrameSnapshot::new(Rect::new(0, 0, 20, 1))
//!     .mouse(PointerTargets::new().target(PointerTarget::new("save", Rect::new(0, 0, 6, 1))));
//!
//! assert_eq!(frame.route_position((2, 0)).unwrap().id, "save");
//! ```

/// Visible regions as frame-local geometry.
///
/// # Problem
///
/// Many Ratatui render functions split an area and immediately draw widgets. That is still the
/// simplest path when geometry is only a local implementation detail. The problem starts when the
/// rectangles need to survive the render function: a later pointer event must hit a row, a test
/// wants to inspect visible cells, or a parent container needs to merge child regions.
///
/// [`crate::regions::Regions`] turns solved rectangles into a region set. It stores the parent area
/// plus ordered [`crate::regions::Region`] values with ids, clipping metadata, and z-order. It does
/// not store widgets or application data.
///
/// The area/region distinction is deliberate. An area is a [`ratatui_core::layout::Rect`], so it
/// answers "where can I draw?" A region answers "what app-owned thing was drawn there, and how
/// should later coordination treat it?" Use areas for local rendering. Use regions when the same
/// rectangle needs identity, clipping metadata, z-order, hit testing, or composition across a
/// component boundary.
///
/// # Current behavior
///
/// Region sets can be built directly, returned by [`crate::linear::Row`],
/// [`crate::linear::Column`], or [`crate::grid::Grid`], translated, clipped, merged, mapped to
/// another id type, and hit tested. Generic `Id` parameters are the bridge back to app state:
/// integers work for generated order, strings make examples and diagnostics readable, enums are
/// usually best for named app controls, and stable record keys fit filtered or reordered data.
///
/// ```rust
/// use ratatui_core::layout::{Constraint, Rect};
/// use ratatui_layout::linear::Row;
///
/// let row_regions = Row::new([Constraint::Length(8), Constraint::Fill(1)])
///     .spacing(1)
///     .regions(Rect::new(0, 0, 20, 1));
///
/// assert_eq!(row_regions.hit_test((2, 0)).unwrap().id, 0);
/// ```
///
/// # Tradeoff
///
/// A region set is extra ceremony. Prefer `Layout::areas` when the result is consumed immediately
/// and never routed, stored, merged, clipped, or tested.
pub mod regions {}

/// Containers, panels, dialogs, flex, grid, overlays, and nested composition.
///
/// # Problem
///
/// Container-like UI often needs an outer region, an inner content region, and child geometry that
/// can be clipped or translated. Ratatui already renders blocks and borders well; the missing piece
/// is a small value that records the geometry without taking ownership of children.
///
/// # Current behavior
///
/// [`crate::container::Container`] computes outer and inner areas with per-edge
/// [`crate::container::Padding`]. [`crate::overlay::Overlay`] stacks explicit regions.
/// [`crate::linear::Row`], [`crate::linear::Column`], and [`crate::grid::Grid`] delegate to
/// Ratatui's layout solver and expose the solved rectangles as values.
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::container::{Container, Padding};
///
/// let layout = Container::new()
///     .padding(Padding::symmetric(2, 1))
///     .child("body")
///     .layout(Rect::new(0, 0, 20, 5));
///
/// assert_eq!(layout.inner, Rect::new(2, 1, 16, 3));
/// ```
///
/// # Tradeoff
///
/// Use ordinary `Block`, `Layout`, and `Rect::inner` directly when the container has no need to
/// expose its geometry as a reusable value.
pub mod containers {}

/// Pointer, focus, selection, cursor, disabled state, hover, and local coordinates.
///
/// # Problem
///
/// Immediate-mode rendering means the current event usually arrives after the previous frame was
/// drawn. Pointer and keyboard coordination therefore need a record of what was visible last frame,
/// but that record should not become a retained widget tree.
///
/// # Current behavior
///
/// [`crate::focus::FocusTargets`] stores keyboard targets while [`crate::focus::FocusState`] stores
/// the persistent focused id. [`crate::focus::FocusFallback`] repairs stale focus after filtering
/// or disabling controls. [`crate::pointer::PointerTargets`] stores pointer targets while
/// [`crate::pointer::PointerState`] stores hover and pressed ids. [`crate::pointer::PointerPhase`]
/// lets apps convert backend mouse events into backend-agnostic hover, press, and release
/// transitions without putting crossterm or another backend in the crate.
/// [`crate::selection::SelectionState`] stores selection separately from geometry.
/// [`crate::cursor::CursorRequests`] collects cursor requests produced during rendering.
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::pointer::{PointerPhase, PointerState, PointerTarget, PointerTargets};
///
/// let targets = PointerTargets::from_targets([PointerTarget::new("save", Rect::new(0, 0, 6, 1))]);
/// let mut state = PointerState::default();
///
/// state.route(&targets, (2, 0), PointerPhase::Press);
/// assert_eq!(
///     state
///         .route(&targets, (2, 0), PointerPhase::Release)
///         .unwrap()
///         .id,
///     "save"
/// );
/// ```
///
/// # Tradeoff
///
/// Keep input handling local when there is only one known target. Use these values when routing
/// depends on a set of visible, possibly clipped or overlapping regions.
pub mod interaction {}

/// Viewports, lists, tables, visible rows/cells, and scroll metrics.
///
/// # Problem
///
/// Scrollable UI needs two coordinate systems: content-space positions owned by the app and
/// viewport-space rectangles visible in the terminal. It also needs status math for scrollbars and
/// efficient rendering of only visible rows or cells.
///
/// # Current behavior
///
/// [`crate::viewport::Viewport`] clamps a content offset against a viewport.
/// [`crate::scroll::ScrollMetrics`] reports thumb size and position for status displays.
/// [`crate::list::VirtualList`] and [`crate::table::VirtualTable`] compute visible app-owned rows
/// and cells without storing the row or cell widgets. Their state types distinguish selection
/// reveal from viewport movement: selecting a row or cell asks layout to keep it visible, while
/// viewport-scroll helpers move the viewport without changing selection. That split is important
/// for mouse wheels, where users expect the pane under the pointer to scroll without changing the
/// selected item that commands operate on.
///
/// ```rust
/// use ratatui_core::layout::{Position, Rect, Size};
/// use ratatui_layout::scroll::ScrollMetrics;
/// use ratatui_layout::viewport::{Viewport, ViewportState};
///
/// let mut state = ViewportState::new(Position::new(0, 10));
/// let layout = Viewport::new(Size::new(20, 100)).layout(Rect::new(0, 0, 20, 10), &mut state);
/// let metrics = ScrollMetrics::vertical(&layout);
///
/// assert_eq!(metrics.offset, 10);
/// ```
///
/// A virtual table can keep a selected cell for commands while wheel input scrolls only the visible
/// rows:
///
/// ```rust
/// use ratatui_layout::table::{CellPosition, VirtualTableState};
///
/// let mut state = VirtualTableState::default();
/// state.select(Some(CellPosition::body(0, 0)));
/// state.scroll_rows_by(4);
///
/// assert_eq!(state.selected(), Some(CellPosition::body(0, 0)));
/// assert_eq!(state.row_scroll(), 4);
/// assert!(!state.scrolls_selected_into_view());
/// ```
///
/// # Tradeoff
///
/// For short, fully visible collections, render the rows directly. Virtualization becomes useful
/// when clipping, scroll metrics, or app-owned row rendering need a shared contract.
pub mod virtualization {}

/// How a render pass produces data used by the next event.
///
/// # Problem
///
/// A complete screen can produce several kinds of visible data at once: regions, keyboard targets,
/// pointer targets, and cursor requests. Returning each concern separately works, but component
/// boundaries often want one value to merge into a parent. The important choice is the boundary:
/// a pane that only routes wheel input can store a [`crate::pointer::PointerTargets`], while a
/// child component that returns layout, pointer, focus, and cursor requests together is a better
/// fit for [`crate::frame::FrameSnapshot`].
///
/// # Current behavior
///
/// [`crate::frame::FrameSnapshot`] aggregates [`crate::regions::Regions`],
/// [`crate::focus::FocusTargets`], [`crate::pointer::PointerTargets`], and
/// [`crate::cursor::CursorRequests`]. It can merge child values and map, translate, or clip their
/// ids and rectangles. [`crate::frame::FrameTargets::from_regions`] adds disabled, focusable,
/// mouseable, and z-order policy to solved regions. [`crate::frame::FrameSnapshot::route_position`]
/// is the broad geometry query; click, hover, and wheel helpers use explicit pointer targets when
/// present.
///
/// Use smaller values directly when they describe the whole coordination problem:
///
/// - [`crate::regions::Regions`] when geometry and hit testing are enough;
/// - [`crate::focus::FocusTargets`] when only keyboard traversal needs previous-frame targets;
/// - [`crate::pointer::PointerTargets`] when pointer routing is independent of focus or cursor
///   placement;
/// - [`crate::cursor::CursorRequests`] when rendering only needs to negotiate the terminal cursor.
///
/// Use [`crate::frame::FrameSnapshot`] when regions, focus targets, pointer targets, and cursor
/// requests must be mapped, translated, clipped, or merged as one component result. That keeps
/// aggregation useful without making it the default return type for every surface.
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::frame::FrameSnapshot;
/// use ratatui_layout::regions::{Region, Regions};
///
/// let frame = FrameSnapshot::from_layout(Regions::from_regions(
///     Rect::new(0, 0, 10, 1),
///     [Region::new("tab", Rect::new(0, 0, 5, 1))],
/// ));
///
/// assert_eq!(frame.route_position((1, 0)).unwrap().id, "tab");
/// ```
///
/// A pointer-only pane can stay smaller and store only pointer targets:
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::pointer::PointerTargets;
///
/// let previous_mouse = PointerTargets::new()
///     .region("queue", Rect::new(0, 0, 20, 10))
///     .region("log", Rect::new(21, 0, 20, 10));
///
/// assert_eq!(previous_mouse.hit_test((25, 4)).unwrap().id, "log");
/// ```
///
/// Event handlers can use intent-named routing helpers while still relying on the same stored
/// frame-local data:
///
/// ```rust
/// use ratatui_core::layout::Rect;
/// use ratatui_layout::frame::FrameSnapshot;
///
/// let frame = FrameSnapshot::new(Rect::new(0, 0, 20, 5))
///     .scroll_region("log-pane", Rect::new(0, 0, 20, 5));
///
/// assert_eq!(frame.route_scroll((3, 4)).unwrap().id, "log-pane");
/// ```
///
/// The examples show both sides of this choice. `pointer_only_scroll_region` routes wheel input
/// with only [`crate::pointer::PointerTargets`]. `component_frames` returns child
/// [`crate::frame::FrameSnapshot`] values because the parent needs to map ids, place children, clip
/// hidden data, and merge cursor requests.
///
/// # Future direction
///
/// Semantic actions and component outcomes are intentionally not part of the first `FrameSnapshot`.
/// They need examples that prove the shape before becoming API.
pub mod frame_snapshots {}

/// Future participant, outcome, action, and component contracts.
///
/// # Problem
///
/// Ratatui's existing `Widget` and `StatefulWidget` traits are intentionally simple render
/// contracts. Framework-like composition sometimes wants measurement, child rendering,
/// interaction data, cursor placement, and semantic actions to travel together.
///
/// # Current behavior
///
/// This crate does not change `ratatui-core` or `ratatui-widgets`. Existing widgets render into
/// rectangles produced by these values. [`crate::participant::LayoutParticipant`] is an experiment
/// for measured, app-owned children, while [`crate::frame::FrameSnapshot`] is an experiment for
/// collecting coordination data after rendering.
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::{Rect, Size};
/// use ratatui_layout::measure::{MeasureConstraint, SizeHint};
/// use ratatui_layout::participant::{
///     LayoutParticipant, MeasureContext, ParticipantFn, RenderContext,
/// };
///
/// let mut label = ParticipantFn::new(
///     |_: usize, _, _| SizeHint::exact(Size::new(4, 1)),
///     |_, _: Rect, _: &mut Buffer, _| {},
/// );
/// let hint = label.measure(0, MeasureConstraint::Unbounded, MeasureContext);
/// let mut buffer = Buffer::empty(Rect::new(0, 0, hint.preferred.width, 1));
/// label.render(0, buffer.area, &mut buffer, RenderContext::default());
/// ```
///
/// # Future direction
///
/// A later component layer may return render outcomes, semantic actions, and richer participant
/// contracts. Those ideas should stay outside core traits until examples show that the additional
/// concepts reduce complexity for real applications.
///
/// # Tradeoff
///
/// Keep using normal widgets when a render call is enough. Reach for participants and frame
/// snapshots only when parent and child need to collaborate across measurement, rendering, and
/// input routing.
pub mod widget_contracts {}

/// Documentation review checklist for this crate.
///
/// # Problem
///
/// Experimental UI crates can easily become a pile of type names: every doc links to every other
/// doc, but no page explains why the reader should care. That is especially risky here because the
/// crate is exploring a coordination model, not just a handful of independent helpers.
///
/// This checklist is the review bar for new or changed docs in `ratatui-layout`.
///
/// A useful review usually looks for a relationship between the overview, the API map, and the
/// examples:
///
/// ```text
/// - The module names the problem and the simpler Ratatui alternative.
/// - The type explains ownership and lists its constructors plus common method groups.
/// - The examples show a realistic method combination, such as layout -> hit_test -> state update.
/// - Links have enough surrounding explanation that the reader knows why to follow them.
/// ```
///
/// # Checklist
///
/// 1. Start with the user problem.
///
///    A module or type should say what pressure created it: previous-frame input routing,
///    externally owned rows, clipped child values, app-owned selection, or another concrete need.
///    Avoid opening with an implementation inventory.
///
/// 2. Show multiple common uses for core primitives.
///
///    The more primitive a type is, the more likely it supports several workflows. A core type like
///    [`crate::regions::Regions`] should explain at least a few uses: input routing, app-owned
///    rendering, child composition, testing, diagnostics, or clipping. Narrow helper methods can
///    stay shorter, but should still say when a caller would reach for them.
///
/// 3. Couple uses to examples where possible.
///
///    A use-case list is easier to trust when at least some items are shown in code. Prefer small
///    doctests that prove the shape: a previous-frame hit test, child regions translated into a
///    parent, visible ids driving selection, or disabled pointer targets being skipped. Use prose
///    only when the example would be noisy or require a full terminal app.
///
///    For public modules, at least one example should usually live near the module-level "common
///    uses" explanation. For primitive structs, examples should cover the most important distinct
///    workflows rather than repeating the constructor shape.
///
/// 4. Keep example density proportional.
///
///    Most type pages should have a few examples that cover the workflows a reader is likely to
///    compare while browsing. More examples are useful for broad primitives such as
///    [`crate::regions::Regions`], [`crate::pointer::PointerTargets`], or
///    [`crate::list::VirtualList`]. Fewer examples are enough for simple enums, field containers,
///    or accessors when the surrounding type already explains the workflow.
///
/// 5. Explain generic id choices at the right level.
///
///    Generic `Id` parameters are a shared concept, so the crate root and guide pages should
///    explain the common choices: integers for generated order, string names for examples and
///    diagnostics, enums for normal app controls, and stable record keys for filtered or reordered
///    data. Individual types should repeat just enough of that guidance for local context.
///
/// 6. Explain ownership boundaries.
///
///    Every coordination type should say what it owns and what it deliberately does not own. For
///    example, [`crate::pointer::PointerTargets`] owns visible pointer targets;
///    [`crate::pointer::PointerState`] owns hover and press state; neither owns callbacks or
///    backend events.
///
/// 7. Name the simpler Ratatui alternative.
///
///    If ordinary `Layout`, `Block`, `Widget`, `StatefulWidget`, `List`, or direct rendering is
///    better for a small case, say so. This keeps the experimental APIs from sounding mandatory.
///
/// 8. Link after giving enough context.
///
///    Intra-doc links should be useful exits, not substitutes for explanation. Before linking to a
///    guide page or related type, give the reader enough local context to understand why that link
///    matters.
///
/// 9. Keep related concepts navigable.
///
///    Related snapshot, state, target, and layout values should link to each other:
///    [`crate::frame::FrameSnapshot`] to its child values, [`crate::focus::FocusTargets`] to
///    [`crate::focus::FocusState`], [`crate::pointer::PointerTargets`] to
///    [`crate::pointer::PointerState`], [`crate::list::VirtualList`] to
///    [`crate::list::VirtualListState`] and [`crate::list::ListLayout`], and so on.
///
/// 10. Provide scan-friendly API maps.
///
///    Struct docs should list constructors, setters, inspection methods, composition methods, and
///    routing/state methods when those categories exist. Module docs should list public types and
///    free functions. These maps should help readers orient themselves before reading individual
///    method docs.
///
/// 11. Document behavior at the field and method level when the name is not enough.
///
///    Fields like `z`, `disabled`, `clip`, `visible_row`, and `selected` need semantics, not just
///    labels. Methods that merge, map, translate, or clip should explain why the operation exists
///    in component composition.
///
/// 12. Separate current behavior from future direction.
///
///    If a page mentions semantic actions, components, or stronger widget contracts, mark that as
///    future direction. Do not imply those contracts already exist.
///
/// 13. Prefer small compiling examples for core ideas.
///
///    Examples should prove the concept without hiding it behind app scaffolding. Use runnable
///    examples when a complete terminal app is clearer than a doctest.
///
/// 14. Avoid link noise.
///
///    Link the first meaningful mention of a type in a local section and important method/field
///    references. Do not link every repeated word if it makes the prose harder to read.
pub mod documentation_review {}
