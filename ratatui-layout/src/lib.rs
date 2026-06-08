#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/favicon.ico"
)]
#![warn(missing_docs)]
//! Experimental frame-local UI coordination primitives for Ratatui.
//!
//! This is a `0.0.1-alpha.0` experiment. The crate exists to validate ideas, examples, and API
//! pressure around immediate-mode UI coordination. Names, module boundaries, responsibilities, and
//! even the crate shape may change substantially; the properly baked version is likely to look
//! different from this alpha.
//!
//! `ratatui-layout` explores immediate-mode UI coordination for applications that need more than a
//! one-pass render call. A render pass can compute visible data such as regions, focus targets,
//! pointer targets, cursor requests, viewport metrics, and selected ids. The application can store
//! that output after drawing and use it to route the next input event.
//!
//! The crate does not introduce a retained widget tree and does not make containers store child
//! widgets. Application code keeps ownership of domain data, widget state, focus state, selection
//! state, and event handling. These primitives only make the output produced by a frame explicit.
//!
//! [`Rect`]: ratatui_core::layout::Rect
//!
//! # Relationship to Ratatui layout
//!
//! Use Ratatui's built-in [`Layout`] directly when the rectangles are only an implementation detail
//! of the current render function. The normal `Layout::areas` and `Layout::split` APIs are still
//! the right tool for ordinary page structure:
//!
//! ```rust
//! use ratatui_core::layout::{Constraint, Layout, Rect};
//!
//! let area = Rect::new(0, 0, 80, 24);
//! let columns = [Constraint::Length(20), Constraint::Fill(1)];
//! let [sidebar, content] = Layout::horizontal(columns).areas(area);
//! ```
//!
//! Use this crate when the solved layout needs to be observed or reused after the split:
//!
//! - pointer hit testing needs to map a position back to an item;
//! - a scrollable viewport needs clamped content-space coordinates;
//! - rows or cells are rendered by external application state instead of child widgets;
//! - selection, clipping, z-order, or stable ids need to travel with a rectangle;
//! - tests or diagnostics need to inspect the visible regions directly.
//!
//! [`Layout`]: ratatui_core::layout::Layout
//!
//! # Areas and regions
//!
//! Ratatui already uses the word "area" for a [`Rect`]: a rectangle in terminal coordinates. An
//! area is geometry only. It is the right word when code is about drawing into a rectangle or
//! splitting a page into rectangles.
//!
//! This crate uses "region" for a rectangle that has become part of frame-local coordination. A
//! [`Region`] contains an area plus an app-owned id, clipping metadata, and z-order. A
//! [`Regions`] value stores those records together so a later input event can route back to the
//! thing that was visible. Put another way: render into areas, store regions when those areas need
//! identity or behavior after rendering.
//!
//! # Choosing what to store
//!
//! The crate has several kinds of values because an immediate-mode app should store only the data
//! it needs for the next event, not a retained widget tree.
//!
//! Frame-local data is produced by rendering or layout and describes what was visible in one frame.
//! It is safe to rebuild every draw and store for the next input event. [`Regions`],
//! [`FocusTargets`], [`PointerTargets`], [`CursorRequests`], and [`FrameSnapshot`] are
//! frame-local outputs. Rich layout outputs such as [`ContainerLayout`], [`GridLayout`],
//! [`list::ListLayout`], [`table::TableLayout`], and [`ViewportLayout`] are also frame-local data.
//!
//! Persistent state belongs to the app or to small app-owned helpers. [`FocusState`],
//! [`PointerState`], [`SelectionState`], [`VisibleSelection`], [`TextFieldState`],
//! [`ButtonRowState`], [`ViewportState`], [`list::VirtualListState`], and
//! [`table::VirtualTableState`] survive across frames and feed the next render pass.
//!
//! Render and measure contexts are callback inputs, not frame outputs. [`MeasureContext`],
//! [`RenderContext`], [`list::ListItemContext`], and [`table::TableCellContext`] describe the
//! current item or cell while external application content is being measured or drawn.
//!
//! Use [`FrameSnapshot`] when a component boundary needs regions, focus targets, pointer targets,
//! and cursor requests to travel together. Use the smaller values directly when a surface only
//! needs one concern, such as a pointer-only scroll region or a focus-only field set.
//!
//! # Frame-local model
//!
//! Ratatui widgets are usually immediate-mode render commands. That works well for simple widgets,
//! but container-like widgets often need more than a final render call. A list, table, menu, or
//! toolbar may need to know where each child ended up so the app can perform hit testing, scroll a
//! viewport, draw a selection, or render custom row content.
//!
//! This crate makes those intermediate results explicit:
//!
//! - [`Regions`] stores solved [`Region`] values for geometry.
//! - [`FocusTargets`], [`FocusState`], and [`FocusFallback`] expose focus traversal over visible
//!   regions and repair stale focus after a frame changes.
//! - [`PointerTargets`], [`PointerState`], and [`PointerPhase`] route pointer positions through
//!   visible targets.
//! - [`FrameTargets`] turns visible regions into aligned regions, pointer targets, and focus target
//!   data.
//! - [`RegionTargets`] starts from an existing [`Regions`] and adds disabled, focusable, mouseable,
//!   and z-order policy.
//! - [`SelectionState`] stores app-owned selection independently from geometry.
//! - [`VisibleSelection`] bridges durable selected ids to visible positions in virtualized views.
//! - [`CursorRequests`] records cursor placement requests produced during rendering.
//! - [`FrameSnapshot`] optionally aggregates one frame's coordination data.
//! - [`Row`], [`Column`], [`Grid`], and [`Overlay`] produce region sets for common arrangements.
//! - [`Container`] computes outer, inner, clipping, and child-region geometry.
//! - [`Viewport`] computes clamped scroll metadata for rectangular content.
//! - [`list::VirtualList`] measures and renders externally owned list items by index.
//! - [`table::VirtualTable`] lays out app-owned cells with pinned headers and two-axis scrolling.
//! - [`TextFieldState`], [`ButtonRowState`], and [`ButtonRow`] handle small rendering-agnostic
//!   control state.
//! - [`ActionSurface`], [`CommandRow`], [`TextInput`], [`ModalShell`], [`ScrollablePane`], and
//!   [`VirtualRecordList`] package common app-level patterns on top of the lower-level primitives.
//!
//! The application still owns its data, row state, and widget state. The layout primitive owns only
//! the geometry calculation and the state needed to keep that geometry stable.
//! [`Row`] and [`Column`] are intentionally thin adapters over the existing Ratatui [`Layout`]
//! solver; their value is the returned [`Regions`], not different split behavior.
//!
//! # Choosing id types
//!
//! Most routing and geometry types are generic over `Id` because the frame-local output should
//! route back to application data without owning that data. Pick the id type that matches how
//! stable the identity must be:
//!
//! - Use integers for generated positions where order is the contract, such as [`Row`] regions,
//!   [`Column`] regions, simple indexed lists, or row-major [`Grid::regions`] cells.
//! - Use string names in examples, tests, diagnostics, and small prototypes where readable output
//!   matters more than exhaustively modeling app state.
//! - Use enums for most application controls. An enum makes routing explicit, gives the compiler a
//!   closed set of cases, and avoids mixing unrelated ids that happen to have the same string or
//!   number.
//! - Use stable record keys when rows can be filtered, sorted, or moved but input should still
//!   route to the same underlying application item.
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::{Region, Regions};
//!
//! #[derive(Debug, Clone, Copy, Eq, PartialEq)]
//! enum Command {
//!     Open,
//!     Save,
//! }
//!
//! let command_regions = Regions::from_regions(
//!     Rect::new(0, 0, 20, 1),
//!     [
//!         Region::new(Command::Open, Rect::new(0, 0, 10, 1)),
//!         Region::new(Command::Save, Rect::new(10, 0, 10, 1)),
//!     ],
//! );
//!
//! assert_eq!(command_regions.hit_test((12, 0)).unwrap().id, Command::Save);
//! ```
//!
//! # First example
//!
//! A small use is to solve a row and render externally owned content into the assigned regions:
//!
//! ```rust
//! use ratatui_core::buffer::Buffer;
//! use ratatui_core::layout::{Constraint, Rect};
//! use ratatui_core::text::Line;
//! use ratatui_core::widgets::Widget;
//! use ratatui_layout::Row;
//!
//! let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 1));
//! let row_regions = Row::new([Constraint::Min(0), Constraint::Length(5)]).regions(buffer.area);
//!
//! Line::from("download").render(row_regions.regions()[0].area, &mut buffer);
//! Line::from("done")
//!     .right_aligned()
//!     .render(row_regions.regions()[1].area, &mut buffer);
//! ```
//!
//! For a complete runnable app that combines region sets, frame snapshots, focus, pointer routing,
//! selection, cursor placement, containers, overlays, viewports, virtual lists, virtual tables, and
//! ordinary Ratatui widgets, run:
//!
//! ```text
//! cargo run -p layout-workspace-inspector
//! ```
//!
//! # Reader paths
//!
//! - App authors: start with [`docs::frame_snapshots`], [`docs::interaction`], and
//!   [`docs::virtualization`] when input routing or scrolling needs previous-frame data.
//! - API reviewers: read the repository `ratatui-layout/use-cases/` catalog to check proposed
//!   helpers against real app shapes before adding broader abstractions.
//! - Widget authors: start with [`docs::regions`], [`docs::containers`], and
//!   [`docs::widget_contracts`] when experimenting with app-owned children.
//! - Maintainers: read [`docs::frame_snapshots`] and [`docs::widget_contracts`] for the split
//!   between current API and future component/action direction, then use
//!   [`docs::documentation_review`] when reviewing new docs.
//!
//! # Crate status
//!
//! This crate is intentionally separate from `ratatui-core` and `ratatui-widgets`. It is a proving
//! ground for reusable coordination APIs. Existing Ratatui widgets continue to work normally; this
//! crate gives widget authors and applications a place to experiment with visible layout state
//! before any of these ideas become core Ratatui contracts.
//!
//! The APIs are interop-first. They also document design pressure around future widget contracts:
//! measured participants, render outcomes, focus targets, cursor requests, and app-owned children
//! can be represented explicitly without requiring a parent container to retain child widgets.

#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::alloc_instead_of_core)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

/// Action surfaces for command rows, toolbars, tabs, and menus.
pub mod action;
/// Container geometry without retained child widgets.
pub mod container;
/// Cursor placement metadata.
pub mod cursor;
/// Documentation guide pages for the coordination model.
pub mod docs;
/// Focus targets and persistent focus state.
pub mod focus;
/// Aggregate frame-local UI data.
pub mod frame;
/// Grid regions and typed cell coordinates.
pub mod grid;
/// Rendering-agnostic input state helpers.
pub mod input;
/// Linear row and column regions.
pub mod linear;
/// Virtual list layout and external row rendering.
pub mod list;
/// Measurement types for externally owned content.
pub mod measure;
/// Modal-shell geometry and outside-click routing.
pub mod modal;
/// Overlay regions for overlapping UI.
pub mod overlay;
/// Scrollable pane coordination.
pub mod pane;
/// External layout participant traits and adapters.
pub mod participant;
/// Pointer target routing and pointer state.
pub mod pointer;
/// Durable-id adapter for virtual lists.
pub mod record_list;
/// Region, clipping, and hit-test types.
pub mod regions;
/// Scroll metrics.
pub mod scroll;
/// App-owned selection state.
pub mod selection;
/// Virtual table layout and rendering.
pub mod table;
/// Text-input coordination without choosing a text widget.
pub mod text_input;
/// Generic viewport layout.
pub mod viewport;

pub use action::{ActionOrientation, ActionSurface, ActionSurfaceLayout, CommandRow};
pub use container::{Container, ContainerLayout, Padding};
pub use cursor::{CursorRequest, CursorRequests};
pub use focus::{FocusFallback, FocusState, FocusTarget, FocusTargets};
pub use frame::{FrameSnapshot, FrameTargets, RegionTargets};
pub use grid::{Grid, GridLayout, GridPosition};
pub use input::{ButtonRow, ButtonRowState, TextFieldState};
pub use linear::{Column, RegionsExt, Row};
pub use measure::{MeasureConstraint, SizeHint};
pub use modal::{ModalLayout, ModalShell};
pub use overlay::Overlay;
pub use pane::{ScrollablePane, ScrollablePaneLayout};
pub use participant::{
    LayoutParticipant, MeasureContext, ParticipantFn, RenderContext, RenderState,
};
pub use pointer::{PointerPhase, PointerState, PointerTarget, PointerTargets};
pub use record_list::{VirtualRecordList, VirtualRecordListLayout, VirtualRecordListState};
pub use regions::{Clip, Hit, Region, Regions};
pub use scroll::ScrollMetrics;
pub use selection::{SelectionMode, SelectionState, VisibleSelection};
pub use table::{
    CellPosition, TableCellContext, TableItems, TableLayout, VirtualTable, VirtualTableState,
    VisibleCell,
};
pub use text_input::{TextEdit, TextInput, TextInputLayout, TextInputState};
pub use viewport::{Viewport, ViewportLayout, ViewportState};
