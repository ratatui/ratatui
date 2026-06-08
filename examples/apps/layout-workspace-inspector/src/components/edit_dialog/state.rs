//! Temporary edit state for the modal dialog.
//!
//! The dialog edits a copy of the selected release item. That gives cancel real semantics and keeps
//! typing separate from durable domain mutation.

use super::fields::TextFieldId;
use crate::domain::{ReleaseItem, Status};

/// Temporary edit buffer for the modal dialog.
///
/// The dialog stores values here until save. This separates editing state from `ReleaseItem`, which
/// keeps cancel cheap and avoids mutating queue data on every keystroke.
#[derive(Debug, Clone)]
pub(crate) struct DialogState {
    /// Edited item title.
    pub(crate) title: String,

    /// Edited item owner.
    pub(crate) owner: String,

    /// Edited status value.
    pub(crate) status: Status,
}

impl DialogState {
    /// Copies editable fields out of the selected release item.
    pub(super) fn from_item(item: &ReleaseItem) -> Self {
        Self {
            title: item.title.clone(),
            owner: item.owner.clone(),
            status: item.status,
        }
    }

    /// Returns the display value for a dialog field.
    ///
    /// Buttons have no editable value, so rendering can call this for every field without needing a
    /// separate branch before formatting text fields.
    pub(super) fn value(&self, field: crate::ids::DialogField) -> &str {
        match field {
            crate::ids::DialogField::Title => &self.title,
            crate::ids::DialogField::Owner => &self.owner,
            crate::ids::DialogField::Status => self.status.label(),
            crate::ids::DialogField::Save | crate::ids::DialogField::Cancel => "",
        }
    }

    /// Returns the editable string for a text field.
    pub(super) const fn text_value_mut(&mut self, field: TextFieldId) -> &mut String {
        match field {
            TextFieldId::Title => &mut self.title,
            TextFieldId::Owner => &mut self.owner,
        }
    }

    /// Cycles the constrained status field in the requested direction.
    pub(super) fn cycle_status(&mut self, delta: isize) {
        self.status = if delta >= 0 {
            self.status.next()
        } else {
            self.status.previous()
        };
    }

    /// Creates small deterministic state for layout and editing tests.
    #[cfg(test)]
    pub(super) const fn for_test() -> Self {
        Self {
            title: String::new(),
            owner: String::new(),
            status: Status::Queued,
        }
    }
}
