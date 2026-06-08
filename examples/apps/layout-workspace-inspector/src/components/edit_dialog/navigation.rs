//! Focus movement rules for the edit dialog.
//!
//! The dialog has two traversal models: Tab moves through every control in wrapping order, while
//! vertical arrows move through stacked fields and stop at the button row. Isolating that policy keeps
//! text editing and rendering from carrying navigation tables.

use super::DialogOutcome;
use super::fields::{
    DialogControl, DialogControlKind, TextFieldId, field_above, field_below, focus_fields,
};
use crate::ids::{DialogField, TargetId};

/// Pure focus movement rules for the dialog.
///
/// This keeps traversal policy separate from text editing and rendering. The dialog component still
/// owns the current app focus target, but these helpers describe where focus should move.
pub(super) struct DialogNavigation;

impl DialogNavigation {
    /// Moves through all dialog controls in wrapping Tab order.
    pub(super) fn tab(focused: Option<TargetId>, delta: isize) -> DialogOutcome {
        let fields = focus_fields();
        let current = focused_field(focused)
            .and_then(|field| fields.iter().position(|candidate| *candidate == field))
            .unwrap_or_default();
        let next = wrap_index(current, delta, fields.len());
        DialogOutcome::Focus(TargetId::Dialog(fields[next]))
    }

    /// Moves through the vertical stack of fields and the button row.
    pub(super) fn vertical(focused: Option<TargetId>, delta: isize) -> DialogOutcome {
        let Some(field) = focused_field(focused) else {
            return DialogOutcome::Focus(TargetId::Dialog(DialogField::Title));
        };
        let next = if delta > 0 {
            field_below(field)
        } else {
            field_above(field)
        };
        match next {
            Some(field) => DialogOutcome::Focus(TargetId::Dialog(field)),
            None => DialogOutcome::Continue,
        }
    }
}

/// Narrows app focus to a dialog field.
const fn focused_field(focused: Option<TargetId>) -> Option<DialogField> {
    match focused {
        Some(TargetId::Dialog(field)) => Some(field),
        _ => None,
    }
}

/// Narrows app focus to a free-text dialog field.
pub(super) fn focused_text_field(focused: Option<TargetId>) -> Option<TextFieldId> {
    match focused_field(focused) {
        Some(field) => match DialogControl::for_field(field) {
            DialogControl {
                kind: DialogControlKind::Text(field),
                ..
            } => Some(field),
            _ => None,
        },
        None => None,
    }
}

/// Applies wrapping movement to a non-empty index range.
const fn wrap_index(index: usize, delta: isize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    let len = len as isize;
    (index as isize + delta).rem_euclid(len) as usize
}
