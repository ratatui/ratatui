//! Convenience builders for host-side input events delivered to WASM widgets.

use crate::wit::{Event, KeyCode, KeyEvent, ResizeEvent};

/// Build a keyboard event.
pub const fn key(code: KeyCode, modifiers: u8) -> Event {
    Event::Key(KeyEvent { code, modifiers })
}

/// Build a terminal resize event.
pub const fn resize(cols: u16, rows: u16) -> Event {
    Event::Resize(ResizeEvent { cols, rows })
}

/// Build a `char` key code.
pub fn char_key(c: char) -> KeyCode {
    KeyCode::Codepoint(c.to_string())
}

/// Build a function key code (`F1`..`F24`).
pub const fn f_key(n: u8) -> KeyCode {
    KeyCode::Function(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize_builds_event() {
        let event = resize(80, 24);
        assert!(
            matches!(event, Event::Resize(ResizeEvent { cols: 80, rows: 24 })),
            "got {event:?}"
        );
    }

    #[test]
    fn f_key_builds_function_code() {
        let code = f_key(5);
        assert!(matches!(code, KeyCode::Function(5)), "got {code:?}");
    }

    #[test]
    fn key_builds_key_event() {
        let code = char_key('x');
        let event = key(code, 1);
        assert!(
            matches!(event, Event::Key(KeyEvent { code: KeyCode::Codepoint(ref s), modifiers: 1 }) if s == "x"),
            "got {event:?}"
        );
    }
}
