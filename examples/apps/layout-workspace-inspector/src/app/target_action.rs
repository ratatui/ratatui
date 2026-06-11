//! App behavior associated with routed frame targets.
//!
//! Frame values route input to ids, but ids are not behavior by themselves. This module turns those
//! ids into app-level intent so `App` can keep hit testing, focus movement, and domain mutation in
//! separate steps.

use ratatui_layout::table::CellPosition;

use crate::ids::{CommandId, DialogField, NodeId, PaneId, TargetId};

/// Region-level behavior for the currently focused target.
///
/// Keyboard movement is coarser than activation. A queue cell moves in two dimensions, a tree row
/// moves vertically, details scrolls, and command buttons use focus traversal. This classifier keeps
/// that policy out of the raw `TargetId` enum.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum FocusedRegion {
    /// Project-tree row selection.
    Tree,

    /// Work-queue cell selection.
    Queue,

    /// Details viewport scrolling.
    Details,

    /// Footer command focus traversal.
    Commands,

    /// Any target that should fall back to focus traversal.
    Other,
}

impl FocusedRegion {
    /// Classifies a focused target for keyboard movement.
    pub(super) const fn from_target(target: Option<TargetId>) -> Self {
        match target {
            Some(TargetId::TreeNode(_)) => Self::Tree,
            Some(TargetId::QueueCell(_)) => Self::Queue,
            Some(TargetId::Pane(PaneId::Details)) => Self::Details,
            Some(TargetId::Command(_)) => Self::Commands,
            _ => Self::Other,
        }
    }
}

/// App-level intent produced by activating a routed target.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum TargetAction {
    /// No behavior applies to this target.
    None,

    /// Select a project-tree node.
    SelectTree(NodeId),

    /// Select a queue body cell.
    SelectQueue(CellPosition),

    /// Run a footer command.
    RunCommand(CommandId),

    /// Focus a dialog field.
    FocusDialog(DialogField),

    /// Save dialog edits.
    SaveDialog,

    /// Cancel dialog edits.
    CancelDialog,
}

/// Converts a routed target into the behavior it represents.
///
/// Availability checks are applied when the action runs. Keeping this conversion pure means key
/// shortcuts, mouse releases, and focused activation all decode routed ids the same way.
pub(super) const fn target_action_for_id(id: TargetId) -> TargetAction {
    match id {
        TargetId::TreeNode(node) => TargetAction::SelectTree(node),
        TargetId::QueueCell(position) if position.row.is_some() => {
            TargetAction::SelectQueue(position)
        }
        TargetId::Command(command) => TargetAction::RunCommand(command),
        TargetId::Dialog(DialogField::Save) => TargetAction::SaveDialog,
        TargetId::Dialog(DialogField::Cancel) => TargetAction::CancelDialog,
        TargetId::Dialog(field) => TargetAction::FocusDialog(field),
        TargetId::Pane(_) | TargetId::QueueCell(_) => TargetAction::None,
    }
}

#[cfg(test)]
mod tests {
    use ratatui_layout::table::CellPosition;

    use super::{FocusedRegion, TargetAction, target_action_for_id};
    use crate::ids::{CommandId, DialogField, NodeId, PaneId, TargetId};

    #[test]
    fn decodes_selectable_targets() {
        assert_eq!(
            target_action_for_id(TargetId::TreeNode(NodeId::Docs)),
            TargetAction::SelectTree(NodeId::Docs)
        );
        assert_eq!(
            target_action_for_id(TargetId::QueueCell(CellPosition::body(2, 1))),
            TargetAction::SelectQueue(CellPosition::body(2, 1))
        );
    }

    #[test]
    fn decodes_commands_and_skips_header_cells() {
        assert_eq!(
            target_action_for_id(TargetId::Command(CommandId::Edit)),
            TargetAction::RunCommand(CommandId::Edit)
        );
        assert_eq!(
            target_action_for_id(TargetId::QueueCell(CellPosition::header(0))),
            TargetAction::None
        );
    }

    #[test]
    fn decodes_dialog_buttons() {
        assert_eq!(
            target_action_for_id(TargetId::Dialog(DialogField::Cancel)),
            TargetAction::CancelDialog
        );
        assert_eq!(
            target_action_for_id(TargetId::Dialog(DialogField::Title)),
            TargetAction::FocusDialog(DialogField::Title)
        );
    }

    #[test]
    fn classifies_focused_targets_for_movement() {
        assert_eq!(
            FocusedRegion::from_target(Some(TargetId::Pane(PaneId::Details))),
            FocusedRegion::Details
        );
        assert_eq!(
            FocusedRegion::from_target(Some(TargetId::Command(CommandId::Help))),
            FocusedRegion::Commands
        );
    }
}
