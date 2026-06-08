//! Action surfaces for command rows, toolbars, tabs, and menus.
//!
//! Many terminal controls are the same coordination problem with different rendering: a row of
//! commands, a vertical menu, a tab bar, or a segmented control lays out named actions, skips
//! disabled actions for input, and routes pointer hits back to app-owned ids. This module packages
//! that pattern without choosing labels, colors, shortcuts, or command effects.
//!
//! Use [`ActionSurface`] when the app already owns persistent focus or selection state and only
//! needs the current frame's regions and targets. Use [`CommandRow`] when a horizontal command
//! surface also needs a small focused-button cursor for left/right keyboard movement.
//!
//! # Types
//!
//! - [`ActionOrientation`] chooses horizontal or vertical layout.
//! - [`ActionSurface`] stores action ids, constraints, spacing, flex policy, and target policy.
//! - [`ActionSurfaceLayout`] is the solved frame-local output for one render pass.
//! - [`CommandRow`] combines a horizontal [`ActionSurface`] with [`crate::ButtonRowState`].
//!
//! # Examples
//!
//! Build a horizontal command strip whose disabled `Save` command remains visible but does not
//! receive focus or pointer activation:
//!
//! ```rust
//! use ratatui_core::layout::{Constraint, Rect};
//! use ratatui_layout::{ActionSurface, FocusFallback, FocusState};
//!
//! #[derive(Debug, Clone, Copy, Eq, PartialEq)]
//! enum Command {
//!     Open,
//!     Save,
//! }
//!
//! let command_slots = [
//!     (Command::Open, Constraint::Length(8)),
//!     (Command::Save, Constraint::Length(8)),
//! ];
//! let layout = ActionSurface::horizontal(command_slots)
//!     .spacing(1)
//!     .layout_with(Rect::new(0, 0, 17, 1), |id| id == Command::Save);
//! let mut focus = FocusState::default();
//!
//! focus.ensure_visible(&layout.frame().focus, FocusFallback::First);
//!
//! assert_eq!(focus.focused(), Some(Command::Open));
//! assert!(layout.frame().route_click((10, 0)).is_none());
//! ```

use alloc::vec::Vec;

use ratatui_core::layout::{Constraint, Flex, Rect};

use crate::frame::{FrameSnapshot, FrameTargets};
use crate::input::ButtonRowState;
use crate::linear::{Column, Row};
use crate::regions::Regions;

/// Direction used to solve an [`ActionSurface`].
///
/// The same action model appears in horizontal controls such as command rows and tabs and in
/// vertical controls such as menus. The orientation changes layout only; ids, disabled policy,
/// focus targets, and pointer routing work the same way.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActionOrientation {
    /// Lay actions out left to right.
    #[default]
    Horizontal,
    /// Lay actions out top to bottom.
    Vertical,
}

/// Layout and target policy for a visible set of actions.
///
/// [`ActionSurface`] owns the frame-invariant description of an action strip: app-owned ids, size
/// constraints, spacing, flex policy, focus order, and z-order. It does not own labels, styles,
/// command callbacks, selected values, or persistent focus. Rendering code uses the returned
/// [`ActionSurfaceLayout`] to draw labels and stores its [`FrameSnapshot`] for the next input
/// event.
///
/// Common uses include:
///
/// - command strips where disabled commands are still visible;
/// - tab bars that route clicks to tab ids while selection lives in app state;
/// - vertical menus where each menu item is a focus and pointer target;
/// - segmented controls that share layout and input routing with button rows.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ActionSurface<Id = usize> {
    orientation: ActionOrientation,
    items: Vec<(Id, Constraint)>,
    spacing: u16,
    flex: Flex,
    focus_start: u16,
    z: u16,
}

impl<Id> ActionSurface<Id> {
    /// Creates a horizontal action surface.
    ///
    /// Pair each id with the constraint used to size that action. Keeping the id beside the sizing
    /// rule makes later routing easier to audit than indexing anonymous slots.
    pub fn horizontal<I>(items: I) -> Self
    where
        I: IntoIterator<Item = (Id, Constraint)>,
    {
        Self::new(ActionOrientation::Horizontal, items)
    }

    /// Creates a vertical action surface.
    ///
    /// Vertical surfaces are useful for menus, command palettes, and radio-like option lists where
    /// the same focus and pointer rules as a toolbar apply top to bottom.
    pub fn vertical<I>(items: I) -> Self
    where
        I: IntoIterator<Item = (Id, Constraint)>,
    {
        Self::new(ActionOrientation::Vertical, items)
    }

    fn new<I>(orientation: ActionOrientation, items: I) -> Self
    where
        I: IntoIterator<Item = (Id, Constraint)>,
    {
        Self {
            orientation,
            items: items.into_iter().collect(),
            spacing: 0,
            flex: Flex::Start,
            focus_start: 0,
            z: 0,
        }
    }

    /// Sets fixed spacing between actions.
    ///
    /// Spacing is delegated to Ratatui's layout solver, so it follows the same behavior as
    /// [`ratatui_core::layout::Layout::spacing`].
    #[must_use = "method returns the modified action surface"]
    pub const fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets how extra space is distributed around the action group.
    ///
    /// Use this when a toolbar should be centered, right-aligned, or spread using Ratatui's
    /// existing flex behavior while still returning inspectable action regions.
    #[must_use = "method returns the modified action surface"]
    pub const fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    /// Sets the first focus order assigned to the actions.
    ///
    /// Parent components can reserve focus ranges so a merged page traverses in reader order.
    #[must_use = "method returns the modified action surface"]
    pub const fn focus_start(mut self, focus_start: u16) -> Self {
        self.focus_start = focus_start;
        self
    }

    /// Sets the z-order for generated layout and pointer targets.
    ///
    /// Higher z values route before lower values when action areas overlap other UI, which is
    /// common for menus and floating command palettes.
    #[must_use = "method returns the modified action surface"]
    pub const fn z(mut self, z: u16) -> Self {
        self.z = z;
        self
    }

    /// Returns the action ids in layout order.
    ///
    /// This is useful when rendering labels from a parallel application data structure or when
    /// repairing a focused id after the command set changes.
    pub fn ids(&self) -> impl Iterator<Item = Id> + '_
    where
        Id: Copy,
    {
        self.items.iter().map(|(id, _)| *id)
    }

    /// Solves action regions and creates enabled focus and pointer targets for every action.
    ///
    /// This is the common path when all visible actions are available. Use
    /// [`ActionSurface::layout_with`] when some actions should remain visible while disabled.
    pub fn layout(&self, area: Rect) -> ActionSurfaceLayout<Id>
    where
        Id: Copy,
    {
        self.layout_with(area, |_| false)
    }

    /// Solves action regions and creates targets while marking disabled actions.
    ///
    /// Disabled actions remain in [`ActionSurfaceLayout::regions`] so rendering can draw them, but
    /// generated focus and pointer targets are disabled so keyboard traversal and pointer routing
    /// skip them.
    pub fn layout_with(&self, area: Rect, disabled: impl Fn(Id) -> bool) -> ActionSurfaceLayout<Id>
    where
        Id: Copy,
    {
        let regions = match self.orientation {
            ActionOrientation::Horizontal => Row::named(self.items.iter().copied())
                .spacing(self.spacing)
                .flex(self.flex)
                .regions(area),
            ActionOrientation::Vertical => Column::named(self.items.iter().copied())
                .spacing(self.spacing)
                .flex(self.flex)
                .regions(area),
        };
        let frame = FrameTargets::from_regions(regions.clone(), self.focus_start)
            .z(self.z)
            .disabled(disabled)
            .build();
        ActionSurfaceLayout { regions, frame }
    }
}

/// Solved output for one [`ActionSurface`] render pass.
///
/// The layout exposes both the regions used for drawing labels and the [`FrameSnapshot`] that the
/// app stores for the next event. It owns no persistent state; rebuild it every frame from the
/// visible action surface.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ActionSurfaceLayout<Id = usize> {
    regions: Regions<Id>,
    frame: FrameSnapshot<Id>,
}

impl<Id> ActionSurfaceLayout<Id> {
    /// Returns the visible action regions for rendering.
    pub const fn regions(&self) -> &Regions<Id> {
        &self.regions
    }

    /// Returns the frame snapshot for focus, pointer, and hit-test routing.
    pub const fn frame(&self) -> &FrameSnapshot<Id> {
        &self.frame
    }

    /// Converts the layout into its owned frame snapshot.
    ///
    /// Use this when a component returns only routing data to its parent after rendering labels.
    pub fn into_frame(self) -> FrameSnapshot<Id> {
        self.frame
    }
}

/// Horizontal action surface with persistent button-row focus state.
///
/// [`CommandRow`] covers the common toolbar and command-strip case. It uses [`ActionSurface`] for
/// the current frame's geometry and targets, and [`ButtonRowState`] for keyboard movement among
/// buttons. Rendering remains application-owned so command labels, shortcuts, and styles can match
/// the app.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CommandRow<Id = usize> {
    surface: ActionSurface<Id>,
    state: ButtonRowState,
}

impl<Id> CommandRow<Id> {
    /// Creates a horizontal command row from `(id, constraint)` pairs.
    ///
    /// Use enum ids for normal app commands, string ids in examples and diagnostics, and integers
    /// for generated command sets where position is the stable identity.
    pub fn new<I>(items: I) -> Self
    where
        I: IntoIterator<Item = (Id, Constraint)>,
    {
        Self {
            surface: ActionSurface::horizontal(items),
            state: ButtonRowState::new(),
        }
    }

    /// Sets fixed spacing between command buttons.
    #[must_use = "method returns the modified command row"]
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.surface = self.surface.spacing(spacing);
        self
    }

    /// Sets the row flex policy.
    #[must_use = "method returns the modified command row"]
    pub fn flex(mut self, flex: Flex) -> Self {
        self.surface = self.surface.flex(flex);
        self
    }

    /// Sets the first focus order assigned to the row.
    #[must_use = "method returns the modified command row"]
    pub fn focus_start(mut self, focus_start: u16) -> Self {
        self.surface = self.surface.focus_start(focus_start);
        self
    }

    /// Sets the z-order assigned to generated targets.
    #[must_use = "method returns the modified command row"]
    pub fn z(mut self, z: u16) -> Self {
        self.surface = self.surface.z(z);
        self
    }

    /// Returns the persistent focused-button state.
    pub const fn state(&self) -> &ButtonRowState {
        &self.state
    }

    /// Returns mutable access to the persistent focused-button state.
    pub const fn state_mut(&mut self) -> &mut ButtonRowState {
        &mut self.state
    }

    /// Returns the focused command id, if the row has any commands.
    pub fn focused_id(&self) -> Option<Id>
    where
        Id: Copy,
    {
        self.surface.ids().nth(self.state.focused_index())
    }

    /// Focuses a command by id.
    ///
    /// This repairs row-local focus from app-level focus or pointer input without exposing index
    /// math at the call site.
    pub fn focus_id(&mut self, id: Id) -> bool
    where
        Id: Copy + Eq,
    {
        if let Some(index) = self.surface.ids().position(|item| item == id) {
            self.state.focus_index(index, self.surface.items.len());
            true
        } else {
            false
        }
    }

    /// Moves focus to the next command, wrapping at the end.
    pub const fn move_next(&mut self) {
        self.state.move_next(self.surface.items.len());
    }

    /// Moves focus to the previous command, wrapping at the start.
    pub const fn move_previous(&mut self) {
        self.state.move_previous(self.surface.items.len());
    }

    /// Moves focus to the next enabled command.
    ///
    /// Use this for left/right command-row movement when disabled commands should stay visible but
    /// keyboard focus should not stop on them.
    pub fn move_next_enabled(&mut self, disabled: impl Fn(Id) -> bool)
    where
        Id: Copy,
    {
        self.move_enabled(1, disabled);
    }

    /// Moves focus to the previous enabled command.
    ///
    /// This is the mirror of [`CommandRow::move_next_enabled`] for left-arrow or shift-tab style
    /// movement inside a command row.
    pub fn move_previous_enabled(&mut self, disabled: impl Fn(Id) -> bool)
    where
        Id: Copy,
    {
        self.move_enabled(-1, disabled);
    }

    fn move_enabled(&mut self, delta: isize, disabled: impl Fn(Id) -> bool)
    where
        Id: Copy,
    {
        if self.surface.items.is_empty() {
            return;
        }
        for _ in 0..self.surface.items.len() {
            if delta.is_negative() {
                self.move_previous();
            } else {
                self.move_next();
            }
            if self.focused_id().is_some_and(|id| !disabled(id)) {
                break;
            }
        }
    }

    /// Solves the command row and creates frame-local targets.
    ///
    /// The returned layout is used for rendering labels and for storing routing data. It does not
    /// mutate row-local focus state.
    pub fn layout(&self, area: Rect) -> ActionSurfaceLayout<Id>
    where
        Id: Copy,
    {
        self.surface.layout(area)
    }

    /// Solves the command row while marking disabled commands.
    ///
    /// Disabled commands remain in the returned regions for rendering but are skipped by generated
    /// focus and pointer targets.
    pub fn layout_with(&self, area: Rect, disabled: impl Fn(Id) -> bool) -> ActionSurfaceLayout<Id>
    where
        Id: Copy,
    {
        self.surface.layout_with(area, disabled)
    }
}

#[cfg(test)]
mod tests {
    use ratatui_core::layout::{Constraint, Rect};

    use super::{ActionSurface, CommandRow};

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    enum Command {
        Edit,
        Save,
        Cancel,
    }

    #[test]
    fn disabled_actions_remain_visible_but_do_not_route() {
        let slots = [
            (Command::Edit, Constraint::Length(4)),
            (Command::Save, Constraint::Length(4)),
        ];
        let layout = ActionSurface::horizontal(slots)
            .layout_with(Rect::new(0, 0, 8, 1), |id| id == Command::Save);

        assert_eq!(layout.regions().regions().len(), 2);
        assert_eq!(
            layout.frame().route_click((1, 0)).unwrap().id,
            Command::Edit
        );
        assert!(layout.frame().route_click((5, 0)).is_none());
    }

    #[test]
    fn command_row_moves_over_disabled_commands() {
        let slots = [
            (Command::Edit, Constraint::Length(4)),
            (Command::Save, Constraint::Length(4)),
            (Command::Cancel, Constraint::Length(6)),
        ];
        let mut row = CommandRow::new(slots);

        row.move_next_enabled(|id| id == Command::Save);

        assert_eq!(row.focused_id(), Some(Command::Cancel));
    }
}
