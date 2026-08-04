//! Convenience builders for host-side input events delivered to WASM widgets.

use crate::wit::{Event, KeyCode, KeyEvent, ResizeEvent};

/// Build a keyboard event.
pub fn key(code: KeyCode, modifiers: u8) -> Event {
    Event::Key(KeyEvent { code, modifiers })
}

/// Build a terminal resize event.
pub fn resize(cols: u16, rows: u16) -> Event {
    Event::Resize(ResizeEvent { cols, rows })
}

/// Build a `char` key code.
pub fn char_key(c: char) -> KeyCode {
    KeyCode::Codepoint(c.to_string())
}

/// Build a function key code (`F1`..`F24`).
pub fn f_key(n: u8) -> KeyCode {
    KeyCode::Function(n)
}
