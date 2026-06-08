use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Flex, Margin, Rect};
use ratatui::style::{Color, Style};

use crate::ids::{PaneId, TargetId};

/// Chooses a pane border style from the currently focused routed target.
///
/// Regions share this helper so focus styling stays consistent after rendering is split across
/// multiple region structs.
pub(crate) fn pane_style_for(focused: Option<TargetId>, pane: PaneId) -> Style {
    if focused.is_some_and(|id| id.pane() == pane) {
        Style::new().fg(Color::Cyan)
    } else {
        Style::new().fg(Color::DarkGray)
    }
}

/// Centers a fixed-size rectangle inside an available area.
///
/// This uses Ratatui's built-in `Layout` solver because centering is ordinary geometry. The
/// frame-local values start after the final rectangle is known.
pub(crate) fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let height_constraint = [Constraint::Length(height)];
    let width_constraint = [Constraint::Length(width)];
    let vertical_layout = ratatui::layout::Layout::vertical(height_constraint).flex(Flex::Center);
    let horizontal_layout =
        ratatui::layout::Layout::horizontal(width_constraint).flex(Flex::Center);
    let [vertical] = vertical_layout.areas(area);
    let [horizontal] = horizontal_layout.areas(vertical);
    horizontal
}

/// Applies a signed movement delta to a bounded zero-based index.
///
/// Tree and table movement both use this helper so their keyboard behavior clamps consistently at
/// the first and last visible item.
pub(crate) fn offset_index(index: usize, delta: isize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    if delta.is_negative() {
        index.saturating_sub(delta.unsigned_abs())
    } else {
        index.saturating_add(delta as usize).min(len - 1)
    }
}

/// Applies a signed movement delta to a terminal coordinate or scroll offset.
///
/// The details pane uses this for scroll input. The viewport later clamps the desired offset to the
/// valid range for the current content height.
pub(crate) const fn offset_u16(index: u16, delta: isize) -> u16 {
    if delta.is_negative() {
        index.saturating_sub(delta.unsigned_abs() as u16)
    } else {
        index.saturating_add(delta as u16)
    }
}

/// Builds a Ratatui margin value without repeating field names at call sites.
///
/// The helper keeps render code focused on layout intent: `margin(1, 1)` reads as horizontal and
/// vertical inset, while the returned type remains Ratatui's standard `Margin`.
pub(crate) const fn margin(horizontal: u16, vertical: u16) -> Margin {
    Margin {
        horizontal,
        vertical,
    }
}

/// Detects Shift-Tab across common terminal encodings.
///
/// Some terminals send Shift-Tab as `BackTab`; others send `Tab` with the shift modifier set. Keeping
/// this at the terminal-input boundary lets page and dialog handlers share the compatibility rule.
pub(crate) fn is_shift_tab(key: KeyEvent) -> bool {
    key.code == KeyCode::BackTab
        || (key.code == KeyCode::Tab && key.modifiers.contains(KeyModifiers::SHIFT))
}
