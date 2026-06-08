//! Traits and contexts for externally owned layout participants.
//!
//! A participant is application-owned content that can measure and render when a parent layout asks
//! it to. The parent does not store the participant. Instead, it passes an id, a constraint or
//! area, and a small context value. This keeps data ownership, widget state, and mutation policy in
//! the application while still letting containers coordinate layout.
//!
//! Use [`LayoutParticipant`] for reusable components and [`ParticipantFn`] for one-off closures in
//! examples, tests, or small apps.
//!
//! Keep using Ratatui's `Widget` and `StatefulWidget` traits when a rectangle is already known and
//! a render call is enough. Participants are for experimental parent/child contracts where a parent
//! must ask app-owned content to measure before assigning final rectangles.
//!
//! # Types and traits
//!
//! - [`MeasureContext`] is the shared measurement context passed with [`MeasureConstraint`].
//! - [`RenderState`] carries common interaction flags such as focus, selection, hover, and disabled
//!   state.
//! - [`RenderContext`] wraps [`RenderState`] for render callbacks.
//! - [`LayoutParticipant`] is the experimental measure-then-render trait for app-owned content.
//! - [`ParticipantFn`] adapts measure and render closures into [`LayoutParticipant`].
//!
//! See [`crate::docs::widget_contracts`] for why participants are kept experimental and separate
//! from Ratatui's existing `Widget` and `StatefulWidget` traits.
//!
//! # Examples
//!
//! Measure app-owned content first, then render it once the parent assigns a final rectangle:
//!
//! ```rust
//! use ratatui_core::buffer::Buffer;
//! use ratatui_core::layout::{Rect, Size};
//! use ratatui_layout::{
//!     LayoutParticipant, MeasureConstraint, MeasureContext, ParticipantFn, RenderContext,
//!     SizeHint,
//! };
//!
//! let labels = ["open", "save"];
//! let mut participant = ParticipantFn::new(
//!     |id: usize, _, _| SizeHint::exact(Size::new(labels[id].len() as u16, 1)),
//!     |_, _: Rect, _: &mut Buffer, _| {},
//! );
//!
//! let hint = participant.measure(0, MeasureConstraint::Unbounded, MeasureContext);
//! let mut buffer = Buffer::empty(Rect::new(0, 0, hint.preferred.width, 1));
//! participant.render(0, buffer.area, &mut buffer, RenderContext::default());
//! ```

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;

use crate::measure::{MeasureConstraint, SizeHint};

/// Context supplied while measuring external content.
///
/// Use [`MeasureContext`] as the common context argument for
/// [`LayoutParticipant::measure`]. It is currently empty because the first APIs only need the
/// constraint and id, but it keeps measurement calls consistent with [`RenderContext`] and leaves
/// room for shared measurement data later.
///
/// # Examples
///
/// ```rust
/// use ratatui_layout::MeasureContext;
///
/// let context = MeasureContext;
/// assert_eq!(context, MeasureContext);
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MeasureContext;

/// Common interaction flags supplied while rendering external content.
///
/// Use [`RenderState`] when a parent wants to pass interaction state without owning the rendering
/// policy. A list can mark an item as selected; the row renderer decides whether to reverse the
/// style, draw a marker, render a nested control differently, or ignore the flag.
///
/// # Fields
///
/// - [`RenderState::focused`] says keyboard input currently targets the item.
/// - [`RenderState::selected`] says the item is part of app-owned selection.
/// - [`RenderState::hovered`] says pointer state currently rests on the item.
/// - [`RenderState::disabled`] says the item should render as unavailable.
///
/// # Examples
///
/// Combine interaction data from different values before calling an app-owned renderer:
///
/// ```rust
/// use ratatui_layout::RenderState;
///
/// let state = RenderState {
///     focused: true,
///     selected: false,
///     hovered: true,
///     disabled: false,
/// };
///
/// assert!(state.focused && state.hovered);
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[allow(clippy::struct_excessive_bools)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderState {
    /// Whether the rendered item has focus.
    pub focused: bool,
    /// Whether the rendered item is selected.
    pub selected: bool,
    /// Whether the rendered item is hovered.
    pub hovered: bool,
    /// Whether the rendered item is disabled.
    pub disabled: bool,
}

/// Context supplied while rendering external content.
///
/// Use [`RenderContext`] as the common rendering argument when a layout primitive calls back into
/// app-owned content. It carries [`RenderState`] shared by multiple container shapes. Specialized
/// containers wrap it with additional fields; for example, [`crate::list::ListItemContext`] adds
/// the item index, visible index, and clipping metadata.
///
/// # Constructors
///
/// - [`RenderContext::selected`] creates a context with only selected state set.
///
/// # Examples
///
/// Pass shared render state through a list, table, or participant-specific context:
///
/// ```rust
/// use ratatui_layout::{RenderContext, RenderState};
///
/// let context = RenderContext {
///     state: RenderState {
///         focused: true,
///         selected: true,
///         ..RenderState::default()
///     },
/// };
///
/// assert!(context.state.focused);
/// assert!(context.state.selected);
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderContext {
    /// Common interaction flags.
    pub state: RenderState,
}

impl RenderContext {
    /// Creates a context with only the selected flag set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::RenderContext;
    ///
    /// let context = RenderContext::selected(true);
    /// assert!(context.state.selected);
    /// assert!(!context.state.focused);
    /// ```
    pub const fn selected(selected: bool) -> Self {
        Self {
            state: RenderState {
                focused: false,
                selected,
                hovered: false,
                disabled: false,
            },
        }
    }
}

/// Externally owned content that can collaborate with layout primitives.
///
/// The trait separates measurement from rendering. A parent first asks for a [`SizeHint`] under a
/// [`MeasureConstraint`], solves a layout, and later calls [`LayoutParticipant::render`] with the
/// final [`Rect`]. The participant may use `id` to look up application data or state.
///
/// The trait takes `&mut self` for rendering because rendering may update local caches, nested
/// widget state, or diagnostics. Measurement takes `&self` so layout can be computed without
/// implying a render-side effect.
///
/// # Required methods
///
/// - [`LayoutParticipant::measure`] returns a [`SizeHint`] for one app-owned id under a
///   [`MeasureConstraint`].
/// - [`LayoutParticipant::render`] draws one app-owned id into its final [`Rect`] with a
///   [`RenderContext`].
///
/// # Examples
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::{Rect, Size};
/// use ratatui_core::text::Line;
/// use ratatui_core::widgets::Widget;
/// use ratatui_layout::{
///     LayoutParticipant, MeasureConstraint, MeasureContext, RenderContext, SizeHint,
/// };
///
/// struct Labels(&'static [&'static str]);
///
/// impl LayoutParticipant for Labels {
///     fn measure(&self, id: usize, _: MeasureConstraint, _: MeasureContext) -> SizeHint {
///         SizeHint::exact(Size::new(self.0[id].len() as u16, 1))
///     }
///
///     fn render(&mut self, id: usize, area: Rect, buf: &mut Buffer, _: RenderContext) {
///         Line::from(self.0[id]).render(area, buf);
///     }
/// }
/// ```
pub trait LayoutParticipant<Id = usize> {
    /// Measures the participant identified by `id`.
    ///
    /// Implementations should avoid mutating application state. If measurement depends on cached
    /// expensive layout information, keep cache mutation explicit in the owning type rather than
    /// hiding it behind this method unless that cache is an internal implementation detail.
    fn measure(&self, id: Id, constraint: MeasureConstraint, ctx: MeasureContext) -> SizeHint;

    /// Renders the participant identified by `id` into the assigned area.
    ///
    /// The `area` is final. Participants should not attempt to negotiate a different size during
    /// rendering; they should clip, truncate, wrap, or leave empty space according to their own
    /// rendering contract.
    ///
    /// # Examples
    ///
    /// Render app-owned content after measurement and parent layout have assigned a final area:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::{Rect, Size};
    /// use ratatui_core::text::Line;
    /// use ratatui_core::widgets::Widget;
    /// use ratatui_layout::{
    ///     LayoutParticipant, MeasureConstraint, MeasureContext, RenderContext, SizeHint,
    /// };
    ///
    /// struct Labels(&'static [&'static str]);
    ///
    /// impl LayoutParticipant for Labels {
    ///     fn measure(&self, id: usize, _: MeasureConstraint, _: MeasureContext) -> SizeHint {
    ///         SizeHint::exact(Size::new(self.0[id].len() as u16, 1))
    ///     }
    ///
    ///     fn render(&mut self, id: usize, area: Rect, buf: &mut Buffer, _: RenderContext) {
    ///         Line::from(self.0[id]).render(area, buf);
    ///     }
    /// }
    ///
    /// let mut labels = Labels(&["open"]);
    /// let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 1));
    /// labels.render(
    ///     0,
    ///     Rect::new(0, 0, 4, 1),
    ///     &mut buffer,
    ///     RenderContext::default(),
    /// );
    /// ```
    fn render(&mut self, id: Id, area: Rect, buf: &mut Buffer, ctx: RenderContext);
}

/// Closure adapter for [`LayoutParticipant`].
///
/// Use [`ParticipantFn`] for small adapters where the participant is just two closures over local
/// state. For example, a test can measure ids from a static slice and render labels without
/// introducing a named type.
///
/// Reusable components should usually implement [`LayoutParticipant`] directly so their measurement
/// and render contracts can be documented near the type.
///
/// # Constructor
///
/// - [`ParticipantFn::new`] stores measure and render closures behind the trait contract.
///
/// # Examples
///
/// Use closures for a small component while still exercising the participant contract:
///
/// ```rust
/// use ratatui_core::buffer::Buffer;
/// use ratatui_core::layout::{Rect, Size};
/// use ratatui_layout::{
///     LayoutParticipant, MeasureConstraint, MeasureContext, ParticipantFn, RenderContext,
///     SizeHint,
/// };
///
/// let labels = ["yes", "no"];
/// let mut participant = ParticipantFn::new(
///     |id: usize, _, _| SizeHint::exact(Size::new(labels[id].len() as u16, 1)),
///     |_, _: Rect, _: &mut Buffer, _| {},
/// );
///
/// assert_eq!(
///     participant
///         .measure(1, MeasureConstraint::Unbounded, MeasureContext)
///         .preferred,
///     Size::new(2, 1),
/// );
/// participant.render(
///     1,
///     Rect::new(0, 0, 2, 1),
///     &mut Buffer::empty(Rect::new(0, 0, 2, 1)),
///     RenderContext::default(),
/// );
/// ```
#[derive(Debug, Clone)]
pub struct ParticipantFn<M, R> {
    measure: M,
    render: R,
}

impl<M, R> ParticipantFn<M, R> {
    /// Creates a participant from measure and render closures.
    ///
    /// The measure closure is called through `&self`; the render closure is called through
    /// `&mut self`, matching the trait contract.
    ///
    /// # Examples
    ///
    /// Build a small participant adapter for a test or local component:
    ///
    /// ```rust
    /// use ratatui_core::buffer::Buffer;
    /// use ratatui_core::layout::{Rect, Size};
    /// use ratatui_core::text::Line;
    /// use ratatui_core::widgets::Widget;
    /// use ratatui_layout::{
    ///     LayoutParticipant, MeasureContext, ParticipantFn, RenderContext, SizeHint,
    /// };
    ///
    /// let labels = ["open", "save"];
    /// let mut participant = ParticipantFn::new(
    ///     |id: usize, _, _: MeasureContext| SizeHint::exact(Size::new(labels[id].len() as u16, 1)),
    ///     |id: usize, area: Rect, buf: &mut Buffer, _: RenderContext| {
    ///         Line::from(labels[id]).render(area, buf);
    ///     },
    /// );
    /// let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
    ///
    /// assert_eq!(
    ///     participant
    ///         .measure(1, Default::default(), MeasureContext)
    ///         .preferred,
    ///     Size::new(4, 1)
    /// );
    /// participant.render(0, Rect::new(0, 0, 4, 1), &mut buf, RenderContext::default());
    /// ```
    pub const fn new(measure: M, render: R) -> Self {
        Self { measure, render }
    }
}

impl<Id, M, R> LayoutParticipant<Id> for ParticipantFn<M, R>
where
    M: Fn(Id, MeasureConstraint, MeasureContext) -> SizeHint,
    R: FnMut(Id, Rect, &mut Buffer, RenderContext),
{
    fn measure(&self, id: Id, constraint: MeasureConstraint, ctx: MeasureContext) -> SizeHint {
        (self.measure)(id, constraint, ctx)
    }

    fn render(&mut self, id: Id, area: Rect, buf: &mut Buffer, ctx: RenderContext) {
        (self.render)(id, area, buf, ctx);
    }
}
