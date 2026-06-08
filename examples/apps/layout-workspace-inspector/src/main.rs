//! Inspect a release board with frame-local layout, interaction, virtualization, and overlays.
//!
//! This is the broad showcase example for `ratatui-layout`. It keeps ordinary Ratatui widgets for
//! rendering, while the app stores the previous `FrameSnapshot` to route keyboard focus, mouse clicks,
//! selection, scroll state, and cursor placement through the UI data produced by the last frame.
//!
//! Read this example from top to bottom. The `App::render` method first composes the screen and
//! stores a new frame snapshot. The input handlers then interpret key and mouse events against that
//! previous plan. That one-frame handoff is the central pattern: drawing remains immediate-mode,
//! while coordination uses durable data from the last completed render pass.
//!
//! The example is split into small components because each pane has a different coordination
//! problem. `ProjectTree` renders a virtual list and owns navigation selection. `WorkQueue` renders
//! a virtual table and owns cell selection. `DetailsPane` renders a scrollable viewport for the
//! selected item. `EditDialog` renders a modal overlay, contributes higher-z mouse targets, and
//! requests the terminal cursor for the active text field. `App` ties those pieces together by
//! merging each component's `FrameSnapshot` and routing the next input event through that merged plan.

use std::io::stdout;

use color_eyre::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;

mod app;
mod components;
mod domain;
mod ids;
mod ui;

use app::App;

/// Focus order band for project tree rows.
///
/// The example gives each region a numeric range so `Tab` moves through panes in a predictable
/// order while arrow keys still handle local movement inside the focused pane.
const TREE_FOCUS: u16 = 100;

/// Focus order band for work queue body cells.
const QUEUE_FOCUS: u16 = 1_000;

/// Focus order band for the details viewport.
const DETAILS_FOCUS: u16 = 2_000;

/// Focus order band for footer commands.
const COMMAND_FOCUS: u16 = 3_000;

/// Focus order band for modal dialog controls.
///
/// Dialog controls sort before the page regions because the dialog is modal. When it is rendered,
/// these targets become the only useful targets for dialog keyboard handling.
const DIALOG_FOCUS: u16 = 0;

/// Z index used by the help overlay.
///
/// Overlay regions need to sit above the normal page regions so mouse routing prefers the overlay when
/// it covers another target.
const HELP_Z: u16 = 30;

/// Runs the immediate-mode event loop.
///
/// The loop has two phases: render a full frame, then route one input event through the app state
/// produced by that frame. Real apps usually add ticking, async messages, or error screens, but the
/// frame-local coordination pattern stays the same.
fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = App::new();
    let mut terminal = ratatui::init();
    if let Err(error) = execute!(stdout(), EnableMouseCapture) {
        ratatui::restore();
        return Err(error.into());
    }
    let app_result = app.run(&mut terminal);
    let mouse_result = execute!(stdout(), DisableMouseCapture);
    ratatui::restore();
    mouse_result?;
    app_result
}
