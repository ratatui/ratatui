//! Local routing concepts being pressure-tested by the example.
//!
//! These are deliberately not crate APIs. They show the kind of relationship data a nested TUI can
//! need after ordinary layout, mouse, and focus target collections have already identified visible regions.

use std::fmt::{self, Display, Formatter};

use ratatui::layout::{Position, Rect};
use ratatui_layout::{FocusFallback, FocusState, FocusTargets, PointerTarget, PointerTargets};

use crate::details::{FormCommand, FormField};
use crate::model::TaskId;

/// App-level target id for route, focus, and pointer data.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Target {
    /// Whole page.
    Page,
    /// Queue pane.
    QueuePane,
    /// Queue row background.
    QueueRow(TaskId),
    /// Inline row action.
    QueueRun(TaskId),
    /// Draggable pane splitter.
    Splitter,
    /// Details pane.
    DetailsPane,
    /// Form group inside the details pane.
    DetailsForm,
    /// Editable form field.
    Field(FormField),
    /// Form command button.
    FormCommand(FormCommand),
    /// Modal backdrop.
    HelpBackdrop,
    /// Help dialog shell.
    HelpDialog,
    /// Help close button.
    HelpClose,
    /// Diagnostic footer.
    Diagnostics,
}

impl Target {
    /// Returns the default parent for a target.
    pub const fn parent(self) -> Option<Self> {
        match self {
            Self::Page => None,
            Self::QueuePane | Self::DetailsPane | Self::Splitter | Self::Diagnostics => {
                Some(Self::Page)
            }
            Self::QueueRow(_) => Some(Self::QueuePane),
            Self::QueueRun(id) => Some(Self::QueueRow(id)),
            Self::DetailsForm => Some(Self::DetailsPane),
            Self::Field(_) | Self::FormCommand(_) => Some(Self::DetailsForm),
            Self::HelpBackdrop | Self::HelpDialog => Some(Self::Page),
            Self::HelpClose => Some(Self::HelpDialog),
        }
    }

    /// Returns a short diagnostic label.
    pub fn label(self) -> String {
        match self {
            Self::Page => "page".into(),
            Self::QueuePane => "queue".into(),
            Self::QueueRow(id) => format!("row {}", id.0),
            Self::QueueRun(id) => format!("run {}", id.0),
            Self::Splitter => "splitter".into(),
            Self::DetailsPane => "details".into(),
            Self::DetailsForm => "form".into(),
            Self::Field(field) => format!("field {}", field.label()),
            Self::FormCommand(command) => format!("command {}", command.label()),
            Self::HelpBackdrop => "help backdrop".into(),
            Self::HelpDialog => "help dialog".into(),
            Self::HelpClose => "help close".into(),
            Self::Diagnostics => "diagnostics".into(),
        }
    }
}

/// One routed event path from leaf target back to the page.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RoutePath {
    targets: Vec<Target>,
}

impl RoutePath {
    /// Builds a route path from leaf to root.
    pub fn from_leaf(leaf: Target) -> Self {
        let mut targets = Vec::new();
        let mut current = Some(leaf);
        while let Some(target) = current {
            targets.push(target);
            current = target.parent();
        }
        Self { targets }
    }

    /// Returns the leaf target.
    pub fn leaf(&self) -> Option<Target> {
        self.targets.first().copied()
    }

    /// Returns whether the path contains a target.
    pub fn contains(&self, target: Target) -> bool {
        self.targets.contains(&target)
    }
}

impl Display for RoutePath {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let mut labels = self.targets.iter().map(|target| target.label());
        if let Some(first) = labels.next() {
            write!(formatter, "{first}")?;
        }
        for label in labels {
            write!(formatter, " -> {label}")?;
        }
        Ok(())
    }
}

/// Frame-local route data for mouse hit testing plus parent-chain diagnostics.
#[derive(Debug, Clone)]
pub struct RouteMap {
    targets: Vec<RouteTarget>,
}

impl Default for RouteMap {
    fn default() -> Self {
        Self::new()
    }
}

impl RouteMap {
    /// Creates an empty route map.
    pub const fn new() -> Self {
        Self {
            targets: Vec::new(),
        }
    }

    /// Adds one target using the target's default parent.
    pub fn target(mut self, id: Target, area: Rect, z: u16) -> Self {
        self.targets.push(RouteTarget {
            id,
            area,
            z,
            disabled: false,
        });
        self
    }

    /// Adds one target that stays visible but does not receive pointer routing.
    pub fn disabled_target(mut self, id: Target, area: Rect, z: u16) -> Self {
        self.targets.push(RouteTarget {
            id,
            area,
            z,
            disabled: true,
        });
        self
    }

    /// Appends another route map.
    pub fn extend(&mut self, other: Self) {
        self.targets.extend(other.targets);
    }

    /// Routes a pointer position to a leaf target and its parent path.
    pub fn hit_path<P: Into<Position>>(&self, position: P) -> Option<RoutePath> {
        let pointer_targets = self
            .targets
            .iter()
            .map(|target| {
                PointerTarget::new(target.id, target.area)
                    .z(target.z)
                    .disabled(target.disabled)
            })
            .collect::<Vec<_>>();
        let mouse = PointerTargets::from_targets(pointer_targets);
        mouse
            .hit_test(position)
            .map(|hit| RoutePath::from_leaf(hit.id))
    }
}

/// One route target visible in the previous frame.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct RouteTarget {
    id: Target,
    area: Rect,
    z: u16,
    disabled: bool,
}

/// A local keyboard focus scope.
#[derive(Debug, Clone)]
pub struct FocusScope {
    /// Group target owning this scope.
    pub owner: Target,
    /// Focusable children in this group.
    pub targets: FocusTargets<Target>,
    /// Whether focus should remain inside this scope while it is active.
    pub trapped: bool,
}

impl FocusScope {
    /// Creates a scope from a group target and child focus target collection.
    pub const fn new(owner: Target, targets: FocusTargets<Target>, trapped: bool) -> Self {
        Self {
            owner,
            targets,
            trapped,
        }
    }

    /// Returns whether the scope owns a focused target.
    pub fn owns(&self, focused: Option<Target>) -> bool {
        focused.is_some_and(|target| self.contains(target))
    }

    /// Returns whether the scope contains a target.
    pub fn contains(&self, target: Target) -> bool {
        self.owner == target || self.targets.targets().iter().any(|item| item.id == target)
    }

    /// Repairs focus against this scope.
    pub fn ensure_visible(&self, focus: &mut FocusState<Target>) {
        let fallback = if self.trapped {
            FocusFallback::First
        } else {
            FocusFallback::Clear
        };
        focus.ensure_visible(&self.targets, fallback);
    }

    /// Moves to the next child target.
    pub fn next(&self, focus: &mut FocusState<Target>) {
        focus.next(&self.targets);
    }

    /// Moves to the previous child target.
    pub fn previous(&self, focus: &mut FocusState<Target>) {
        focus.previous(&self.targets);
    }
}

/// Persistent pointer capture state.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct PointerCapture {
    target: Option<Target>,
}

impl PointerCapture {
    /// Starts capture for a target.
    pub const fn capture(&mut self, target: Target) {
        self.target = Some(target);
    }

    /// Clears any active capture.
    pub const fn release(&mut self) {
        self.target = None;
    }

    /// Returns the captured target.
    pub const fn target(self) -> Option<Target> {
        self.target
    }

    /// Returns whether capture is active.
    pub const fn is_active(self) -> bool {
        self.target.is_some()
    }
}
