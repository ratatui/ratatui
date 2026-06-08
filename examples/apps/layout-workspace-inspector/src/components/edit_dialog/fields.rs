//! Field identities and fixed region data for the edit dialog.
//!
//! Keeping these ids separate from rendering makes the dialog easier to scan. The same
//! [`DialogField`] values drive focus order, row planning, mouse targets, cursor placement, and button
//! navigation.

use crate::ids::{DialogField, TargetId};

/// Width of the modal edit dialog.
pub(super) const DIALOG_WIDTH: u16 = 56;

/// Height required for three fields and one button row with one-line spacing.
///
/// The dialog content uses top/bottom padding of three rows total, so this height leaves seven
/// content rows for title, owner, status, and a side-by-side save/cancel row.
pub(super) const DIALOG_HEIGHT: u16 = 10;

/// Dialog controls in user traversal order.
///
/// This is the local table that ties together a control's routed field id, visual row, and behavior
/// kind. Rendering, key handling, and tests can ask this table what a field means instead of
/// rediscovering that from scattered matches.
pub(super) const CONTROLS: [DialogControl; 5] = [
    DialogControl::new(
        DialogField::Title,
        DialogRow::Title,
        DialogControlKind::Text(TextFieldId::Title),
    ),
    DialogControl::new(
        DialogField::Owner,
        DialogRow::Owner,
        DialogControlKind::Text(TextFieldId::Owner),
    ),
    DialogControl::new(
        DialogField::Status,
        DialogRow::Status,
        DialogControlKind::Status,
    ),
    DialogControl::new(
        DialogField::Cancel,
        DialogRow::Buttons,
        DialogControlKind::Button,
    ),
    DialogControl::new(
        DialogField::Save,
        DialogRow::Buttons,
        DialogControlKind::Button,
    ),
];

/// One control in the modal dialog.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) struct DialogControl {
    /// Routed field id used for focus, mouse, and activation.
    pub(super) field: DialogField,

    /// Visual row that contains this control.
    pub(super) row: DialogRow,

    /// Behavior family for the control.
    pub(super) kind: DialogControlKind,
}

impl DialogControl {
    /// Creates a dialog-control descriptor.
    const fn new(field: DialogField, row: DialogRow, kind: DialogControlKind) -> Self {
        Self { field, row, kind }
    }

    /// Finds the descriptor for a routed dialog field.
    pub(super) fn for_field(field: DialogField) -> Self {
        CONTROLS
            .iter()
            .copied()
            .find(|control| control.field == field)
            .expect("every dialog field has a control descriptor")
    }

    /// Reports whether this control is an action button.
    pub(super) const fn is_button(self) -> bool {
        matches!(self.kind, DialogControlKind::Button)
    }
}

/// Returns focusable fields in traversal order.
pub(super) fn focus_fields() -> [DialogField; CONTROLS.len()] {
    CONTROLS.map(|control| control.field)
}

/// Returns button fields in left-to-right visual order.
pub(super) fn button_fields() -> Vec<DialogField> {
    CONTROLS
        .iter()
        .copied()
        .filter(|control| control.is_button())
        .map(|control| control.field)
        .collect()
}

/// Returns the first focusable field on the row below the current field.
pub(super) fn field_below(field: DialogField) -> Option<DialogField> {
    let row = DialogControl::for_field(field).row;
    let mut passed_current_row = false;
    for control in CONTROLS {
        if control.row == row {
            passed_current_row = true;
        }
        if control.row != row && passed_current_row {
            return Some(control.field);
        }
    }
    None
}

/// Returns the first focusable field on the row above the current field.
pub(super) fn field_above(field: DialogField) -> Option<DialogField> {
    let row = DialogControl::for_field(field).row;
    let mut previous_row = None;
    let mut current_row = None;
    for control in CONTROLS {
        if control.row == row {
            return previous_row;
        }
        if current_row != Some(control.row) {
            previous_row = Some(control.field);
            current_row = Some(control.row);
        }
    }
    None
}

/// Behavior family for a dialog control.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum DialogControlKind {
    /// Free-text field with cursor state.
    Text(TextFieldId),

    /// Constrained status picker.
    Status,

    /// Action button.
    Button,
}

/// Free-text fields in the dialog.
///
/// [`DialogField`] includes text fields, a status picker, and action buttons because all of those are
/// focus targets. `TextFieldId` is narrower: only these controls accept typed characters and own a
/// cursor.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum TextFieldId {
    /// Item title input.
    Title,

    /// Item owner input.
    Owner,
}

impl TextFieldId {
    /// Returns the dialog field used for layout and cursor placement.
    pub(super) const fn dialog_field(self) -> DialogField {
        match self {
            Self::Title => DialogField::Title,
            Self::Owner => DialogField::Owner,
        }
    }
}

/// Named vertical rows inside the edit dialog.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(super) enum DialogRow {
    /// Editable title row.
    Title,

    /// Editable owner row.
    Owner,

    /// Editable status row.
    Status,

    /// Horizontal row containing save and cancel buttons.
    Buttons,
}

/// Small classification helpers for routed dialog fields.
pub(super) struct DialogFieldKind;

impl DialogFieldKind {
    /// Returns whether focus is on a dialog action button.
    pub(super) fn is_button(focused: Option<TargetId>) -> bool {
        let Some(TargetId::Dialog(field)) = focused else {
            return false;
        };
        DialogControl::for_field(field).is_button()
    }
}
