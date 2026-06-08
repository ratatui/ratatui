//! Pane components used by the release-board example.
//!
//! Each component owns the persistent UI state for one region and returns frame-local data after it
//! renders. `App` merges that data into the frame snapshot used by the next input event.

mod command_strip;
mod details;
mod edit_dialog;
mod help_overlay;
mod project_tree;
mod queue_rows;
mod status_bar;
mod tree_rows;
mod work_queue;

pub(crate) use command_strip::CommandStrip;
pub(crate) use details::DetailsPane;
pub(crate) use edit_dialog::{DialogOutcome, EditDialog};
pub(crate) use help_overlay::HelpOverlay;
pub(crate) use project_tree::ProjectTree;
pub(crate) use status_bar::StatusBar;
pub(crate) use work_queue::WorkQueue;
