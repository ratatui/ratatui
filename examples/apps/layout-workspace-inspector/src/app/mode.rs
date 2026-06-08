//! Top-level app interaction modes.
//!
//! Modes own modal state when that state is only valid in the mode. That keeps the app from
//! representing "editing without a dialog" or "page mode with stale dialog edits".

use crate::components::EditDialog;

/// Current top-level interaction mode.
///
/// The page, help overlay, and edit dialog each have different input rules. Keeping that as one enum
/// prevents independent booleans from creating ambiguous states such as "help is open under a modal
/// editor".
#[derive(Debug)]
pub(super) enum AppMode {
    /// Normal dashboard interaction.
    Page,

    /// Help overlay is visible and owns keyboard input.
    Help,

    /// Edit dialog is visible and owns keyboard input.
    Editing(EditDialog),
}
