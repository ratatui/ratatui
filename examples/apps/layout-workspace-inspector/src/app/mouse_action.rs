//! Page-level mouse input for the release-board screen.
//!
//! This module keeps backend event decoding separate from app side effects. Crossterm events become
//! small pointer inputs first; `App` then applies those inputs through the previous frame's mouse
//! data.

use crossterm::event::{MouseEvent, MouseEventKind};

/// Terminal cell position from a backend mouse event.
pub(super) type MousePosition = (u16, u16);

/// Backend-agnostic pointer input used by the app.
///
/// The app still owns hover, press, focus, activation, and scroll behavior. This enum only records
/// what happened at a terminal position so policy can be applied in one place.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum MouseInput {
    /// Pointer moved over the terminal.
    Hover(MousePosition),

    /// Pointer button pressed at a position.
    Press(MousePosition),

    /// Pointer button released at a position.
    Release(MousePosition),

    /// Vertical wheel movement over a position.
    Scroll {
        /// Position under the pointer when the wheel moved.
        position: MousePosition,

        /// Signed row delta. Positive scrolls down, negative scrolls up.
        delta: isize,
    },

    /// Mouse event shape that this example intentionally ignores.
    None,
}

/// Decodes a crossterm mouse event into page-level pointer input.
pub(super) const fn mouse_input_for_event(mouse: MouseEvent) -> MouseInput {
    let position = (mouse.column, mouse.row);
    match mouse.kind {
        MouseEventKind::Moved => MouseInput::Hover(position),
        MouseEventKind::Down(_) => MouseInput::Press(position),
        MouseEventKind::Up(_) => MouseInput::Release(position),
        MouseEventKind::ScrollDown => MouseInput::Scroll { position, delta: 1 },
        MouseEventKind::ScrollUp => MouseInput::Scroll {
            position,
            delta: -1,
        },
        MouseEventKind::ScrollLeft | MouseEventKind::ScrollRight | MouseEventKind::Drag(_) => {
            MouseInput::None
        }
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

    use super::{MouseInput, mouse_input_for_event};

    #[test]
    fn decodes_mouse_scroll_without_horizontal_scroll() {
        let event = MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 3,
            row: 4,
            modifiers: KeyModifiers::empty(),
        };

        assert_eq!(
            mouse_input_for_event(event),
            MouseInput::Scroll {
                position: (3, 4),
                delta: -1
            }
        );
    }

    #[test]
    fn decodes_mouse_press_and_ignores_drag() {
        let press = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 2,
            row: 7,
            modifiers: KeyModifiers::empty(),
        };
        let drag = MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column: 2,
            row: 7,
            modifiers: KeyModifiers::empty(),
        };

        assert_eq!(mouse_input_for_event(press), MouseInput::Press((2, 7)));
        assert_eq!(mouse_input_for_event(drag), MouseInput::None);
    }
}
