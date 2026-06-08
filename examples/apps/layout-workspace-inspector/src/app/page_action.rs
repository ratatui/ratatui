//! Page-level key bindings for the release-board screen.
//!
//! This module decodes keys into intent without mutating app state. Keeping key policy separate from
//! side effects makes behavior easier to test and keeps `App` focused on applying already-decoded
//! actions.

use crossterm::event::{KeyCode, KeyEvent};

use crate::ids::{CommandId, TargetId};
use crate::ui::is_shift_tab;

/// Intent decoded from a page-level key press.
///
/// The key decoder answers "what did the user ask for?" without mutating app state. Applying the
/// action is a separate step, which keeps key bindings, focus movement, and domain commands from being
/// interleaved in one large match.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum PageAction {
    /// No page-level action.
    None,

    /// Quit the event loop.
    Quit,

    /// Move focus to the next visible target.
    FocusNext,

    /// Move focus to the previous visible target.
    FocusPrevious,

    /// Move inside the currently focused page region.
    Move {
        /// Row delta for lists, tables, or scrollable panes.
        row: isize,

        /// Column delta for table cells or horizontal command movement.
        column: isize,
    },

    /// Activate the currently focused target.
    ActivateFocused,

    /// Activate a routed target directly.
    Activate(TargetId),

    /// Focus a specific routed target.
    Focus(TargetId),
}

/// Decodes a key press into page-level intent.
///
/// This keeps key binding policy separate from side effects. The returned action may still depend on
/// terminal quirks, such as Shift-Tab arriving as a shifted Tab event.
pub(super) fn page_action_for_key(key: KeyEvent) -> PageAction {
    if is_shift_tab(key) {
        return PageAction::FocusPrevious;
    }
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => PageAction::Quit,
        KeyCode::Tab => PageAction::FocusNext,
        KeyCode::Char('j') | KeyCode::Down => PageAction::Move { row: 1, column: 0 },
        KeyCode::Char('k') | KeyCode::Up => PageAction::Move { row: -1, column: 0 },
        KeyCode::Char('l') | KeyCode::Right => PageAction::Move { row: 0, column: 1 },
        KeyCode::Char('h') | KeyCode::Left => PageAction::Move { row: 0, column: -1 },
        KeyCode::Char(' ') | KeyCode::Enter => PageAction::ActivateFocused,
        KeyCode::Char('e') => PageAction::Activate(TargetId::Command(CommandId::Edit)),
        KeyCode::Char('r') => PageAction::Activate(TargetId::Command(CommandId::Run)),
        KeyCode::Char('m') => PageAction::Activate(TargetId::Command(CommandId::Mark)),
        KeyCode::Char('?') => PageAction::Activate(TargetId::Command(CommandId::Help)),
        KeyCode::Char('/') => PageAction::Focus(TargetId::Command(CommandId::Edit)),
        _ => PageAction::None,
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use super::{PageAction, page_action_for_key};
    use crate::ids::{CommandId, TargetId};

    #[test]
    fn decodes_navigation_keys() {
        assert_eq!(
            page_action_for_key(KeyEvent::from(KeyCode::Down)),
            PageAction::Move { row: 1, column: 0 }
        );
        assert_eq!(
            page_action_for_key(KeyEvent::from(KeyCode::Char('h'))),
            PageAction::Move { row: 0, column: -1 }
        );
    }

    #[test]
    fn decodes_commands_and_focus_shortcuts() {
        assert_eq!(
            page_action_for_key(KeyEvent::from(KeyCode::Char('r'))),
            PageAction::Activate(TargetId::Command(CommandId::Run))
        );
        assert_eq!(
            page_action_for_key(KeyEvent::from(KeyCode::Char('/'))),
            PageAction::Focus(TargetId::Command(CommandId::Edit))
        );
    }

    #[test]
    fn decodes_shift_tab_as_previous_focus() {
        let key = KeyEvent::new(KeyCode::Tab, KeyModifiers::SHIFT);

        assert_eq!(page_action_for_key(key), PageAction::FocusPrevious);
    }
}
