//! Text-input coordination without choosing a text widget.
//!
//! Text entry needs several small pieces to agree: edit commands mutate the string, the cursor
//! state moves through character positions, a rendered field provides a focus and pointer target,
//! and the terminal cursor should land inside the editable text. This module packages those
//! mechanics while leaving labels, borders, validation, masking, and styling to the app or widget.
//!
//! # Types
//!
//! - [`TextEdit`] is a rendering-agnostic edit command.
//! - [`TextInputState`] owns the persistent cursor state for one editable string.
//! - [`TextInput`] describes the current frame's target id, prefix width, focus order, and z-order.
//! - [`TextInputLayout`] exposes the field area, editable area, cursor request, and frame snapshot.
//!
//! # Examples
//!
//! ```rust
//! use ratatui_core::layout::Rect;
//! use ratatui_layout::{TextEdit, TextInput, TextInputState};
//!
//! let mut value = String::from("ops");
//! let mut state = TextInputState::at_end(&value);
//! state.apply(TextEdit::Insert('!'), &mut value);
//! let input = TextInput::new("owner").prefix_width(7);
//! let layout = input.layout(Rect::new(0, 0, 20, 1), state, true);
//!
//! assert_eq!(value, "ops!");
//! assert_eq!(layout.edit_area().x, 7);
//! assert_eq!(layout.cursor_request().position.x, 11);
//! ```

use alloc::string::String;

use ratatui_core::layout::{Position, Rect};

use crate::cursor::CursorRequests;
use crate::frame::{FrameSnapshot, FrameTargets};
use crate::input::TextFieldState;

/// Edit command understood by [`TextInputState`].
///
/// This enum is deliberately backend-agnostic. An app maps crossterm, termwiz, or custom key
/// events into these commands, then calls [`TextInputState::apply`] to mutate the string and cursor
/// together.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TextEdit {
    /// Insert one character at the cursor.
    Insert(char),
    /// Remove the character before the cursor.
    Backspace,
    /// Remove the character at the cursor.
    Delete,
    /// Move the cursor one character left.
    Left,
    /// Move the cursor one character right.
    Right,
    /// Move the cursor to the start of the string.
    Home,
    /// Move the cursor to the end of the string.
    End,
}

/// Persistent cursor state for one editable string.
///
/// [`TextInputState`] wraps [`TextFieldState`] with a command-level API. The app still owns the
/// text value so validation, persistence, undo, and domain conversion stay in application code.
/// Store one state per editable value.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextInputState {
    field: TextFieldState,
}

impl TextInputState {
    /// Creates a cursor state at the start of the text.
    pub const fn new() -> Self {
        Self {
            field: TextFieldState::new(),
        }
    }

    /// Creates a cursor state at the end of an existing value.
    ///
    /// This is useful when opening an edit dialog and placing the cursor after the current text.
    pub fn at_end(value: &str) -> Self {
        Self {
            field: TextFieldState::at_end(value),
        }
    }

    /// Returns the underlying cursor state.
    ///
    /// Use this when interoperating with lower-level helpers that already accept
    /// [`TextFieldState`].
    pub const fn field(&self) -> &TextFieldState {
        &self.field
    }

    /// Returns mutable access to the underlying cursor state.
    ///
    /// This keeps escape hatches local for apps that need custom text movement or validation.
    pub const fn field_mut(&mut self) -> &mut TextFieldState {
        &mut self.field
    }

    /// Returns the current cursor position in characters.
    pub const fn cursor(self) -> usize {
        self.field.cursor()
    }

    /// Clamps the cursor to the current value length.
    ///
    /// Call this after replacing the entire text value from validation, paste handling, or external
    /// application state.
    pub fn clamp_to(&mut self, value: &str) {
        self.field.clamp_to(value);
    }

    /// Applies an edit command to the value and cursor.
    ///
    /// The command operates on Rust `char` boundaries. It does not implement grapheme clusters,
    /// masking, or horizontal scrolling; those concerns belong in a richer text widget built on top
    /// of this state helper.
    pub fn apply(&mut self, edit: TextEdit, value: &mut String) {
        match edit {
            TextEdit::Insert(character) => self.field.insert_char(value, character),
            TextEdit::Backspace => self.field.backspace(value),
            TextEdit::Delete => self.field.delete(value),
            TextEdit::Left => self.field.move_left(),
            TextEdit::Right => self.field.move_right(value),
            TextEdit::Home => self.field.move_home(),
            TextEdit::End => self.field.move_end(value),
        }
    }
}

/// Frame-local layout policy for a single-line text input.
///
/// [`TextInput`] identifies the field, reserves optional prefix columns for a label, and produces
/// focus, pointer, and cursor data for the current frame. It does not render the label or text and
/// does not own the edited string.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TextInput<Id = usize> {
    id: Id,
    prefix_width: u16,
    focus_order: u16,
    z: u16,
}

impl<Id> TextInput<Id> {
    /// Creates text-input layout policy for one app-owned id.
    pub const fn new(id: Id) -> Self {
        Self {
            id,
            prefix_width: 0,
            focus_order: 0,
            z: 0,
        }
    }

    /// Reserves columns before the editable text.
    ///
    /// Use this when a field renders a label such as `owner:` before the editable value. Cursor
    /// placement and pointer-to-cursor conversion happen inside the editable area after the prefix.
    #[must_use = "method returns the modified text input"]
    pub const fn prefix_width(mut self, prefix_width: u16) -> Self {
        self.prefix_width = prefix_width;
        self
    }

    /// Sets the focus order assigned to the field.
    #[must_use = "method returns the modified text input"]
    pub const fn focus_order(mut self, focus_order: u16) -> Self {
        self.focus_order = focus_order;
        self
    }

    /// Sets the z-order assigned to the field target.
    #[must_use = "method returns the modified text input"]
    pub const fn z(mut self, z: u16) -> Self {
        self.z = z;
        self
    }

    /// Solves the editable area and creates frame-local input targets.
    ///
    /// When `focused` is true, the returned frame includes a cursor request based on the current
    /// cursor state and prefix width.
    pub fn layout(self, area: Rect, state: TextInputState, focused: bool) -> TextInputLayout<Id>
    where
        Id: Copy,
    {
        let edit_area = Rect {
            x: area.x.saturating_add(self.prefix_width).min(area.right()),
            width: area.width.saturating_sub(self.prefix_width),
            ..area
        };
        let mut frame = FrameTargets::new(area, self.focus_order)
            .z(self.z)
            .region(self.id, area);
        let cursor_request = state
            .field
            .cursor_request_after_prefix(area, self.prefix_width);
        if focused {
            frame = frame.cursor(CursorRequests::new().request(cursor_request));
        }
        TextInputLayout {
            id: self.id,
            area,
            edit_area,
            cursor_request,
            frame,
        }
    }
}

/// Solved frame-local data for one [`TextInput`].
///
/// Use this value to render text into [`TextInputLayout::edit_area`], store
/// [`TextInputLayout::frame`] for routing, place the terminal cursor, and map pointer positions
/// back to cursor indexes.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TextInputLayout<Id = usize> {
    id: Id,
    area: Rect,
    edit_area: Rect,
    cursor_request: crate::CursorRequest,
    frame: FrameSnapshot<Id>,
}

impl<Id> TextInputLayout<Id> {
    /// Returns the field id.
    pub const fn id(&self) -> &Id {
        &self.id
    }

    /// Returns the whole field area, including any label prefix.
    pub const fn area(&self) -> Rect {
        self.area
    }

    /// Returns the editable text area after the label prefix.
    pub const fn edit_area(&self) -> Rect {
        self.edit_area
    }

    /// Returns the cursor request computed for the editable text.
    ///
    /// The request is available even when the field is not focused. Only focused layouts add it to
    /// the returned frame snapshot.
    pub const fn cursor_request(&self) -> &crate::CursorRequest {
        &self.cursor_request
    }

    /// Returns the frame snapshot for focus, pointer, and cursor routing.
    pub const fn frame(&self) -> &FrameSnapshot<Id> {
        &self.frame
    }

    /// Places the cursor from a pointer position inside the editable area.
    ///
    /// This maps terminal columns to character positions using one column per `char`. A richer text
    /// widget can replace this with width-aware or grapheme-aware logic while still using the same
    /// field target and cursor state.
    pub fn place_cursor_from_position(
        &self,
        position: impl Into<Position>,
        value: &str,
        state: &mut TextInputState,
    ) -> bool {
        let position = position.into();
        if !self.edit_area.contains(position) {
            return false;
        }
        let local_x = position.x.saturating_sub(self.edit_area.x) as usize;
        let cursor = local_x.min(value.chars().count());
        state.field.set_cursor(cursor, value);
        true
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    use ratatui_core::layout::Rect;

    use super::{TextEdit, TextInput, TextInputState};

    #[test]
    fn applies_text_edits_to_value_and_cursor() {
        let mut value = String::from("ab");
        let mut state = TextInputState::at_end(&value);

        state.apply(TextEdit::Insert('c'), &mut value);
        state.apply(TextEdit::Left, &mut value);
        state.apply(TextEdit::Backspace, &mut value);

        assert_eq!(value, "ac");
        assert_eq!(state.cursor(), 1);
    }

    #[test]
    fn input_layout_accounts_for_prefix_width() {
        let value = String::from("owner");
        let state = TextInputState::at_end(&value);
        let layout =
            TextInput::new("field")
                .prefix_width(7)
                .layout(Rect::new(0, 0, 20, 1), state, true);

        assert_eq!(layout.edit_area(), Rect::new(7, 0, 13, 1));
        assert_eq!(layout.cursor_request().position.x, 12);
    }

    #[test]
    fn pointer_position_places_cursor_in_edit_area() {
        let value = String::from("abcd");
        let mut state = TextInputState::new();
        let layout =
            TextInput::new("field")
                .prefix_width(2)
                .layout(Rect::new(0, 0, 10, 1), state, false);

        assert!(layout.place_cursor_from_position((5, 0), &value, &mut state));
        assert_eq!(state.cursor(), 3);
    }
}
