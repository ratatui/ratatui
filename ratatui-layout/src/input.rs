//! Rendering-agnostic state helpers for simple controls.
//!
//! `ratatui-layout` mostly deals in frame-local geometry and routing data, but real applications
//! also need a small amount of persistent state behind common controls. This module contains
//! helpers for the mechanical parts that should not be reimplemented in every example: text-field
//! cursor editing and button-row focus movement.
//!
//! These types deliberately do not render. They do not know about styles, labels, borders, or key
//! bindings. A Ratatui app can render a field or button however it wants, then use these helpers to
//! keep the state transitions predictable.
//!
//! # Types
//!
//! - [`TextFieldState`] stores a character cursor and edits an externally owned
//!   [`String`](alloc::string::String).
//! - [`ButtonRowState`] stores the focused button position for a row of app-owned ids.
//! - [`ButtonRow`] stores a fixed row of button ids with its [`ButtonRowState`].
//!
//! Use full widgets or application-specific state when the control owns validation, completion,
//! history, undo, or other domain behavior. Use these helpers when the problem is just editing a
//! temporary string or moving focus across a small row of buttons.

use alloc::string::String;
use alloc::vec::Vec;

use ratatui_core::layout::{Position, Rect};

use crate::cursor::CursorRequest;

/// Cursor and edit state for an externally owned text value.
///
/// [`TextFieldState`] owns only the cursor position. The text itself stays in application state, so
/// cancel/save flows can copy values into a temporary dialog struct and apply them later. Cursor
/// positions are measured in characters rather than bytes, which keeps insertion and deletion from
/// splitting a UTF-8 code point.
///
/// # Common uses
///
/// - Store one `TextFieldState` beside each editable field in a modal dialog.
/// - Move the cursor with left/right/home/end keys while keeping the text in a domain edit buffer.
/// - Insert, backspace, or delete characters without duplicating byte-index conversion at every
///   call site.
///
/// # Methods
///
/// - [`TextFieldState::new`] starts with the cursor at the beginning.
/// - [`TextFieldState::at_end`] starts with the cursor after the current value.
/// - [`TextFieldState::cursor`] reads the character cursor.
/// - [`TextFieldState::set_cursor`] stores a cursor clamped to a value.
/// - [`TextFieldState::clamp_to`] clamps the cursor after a value changed externally.
/// - [`TextFieldState::move_left`], [`TextFieldState::move_right`], [`TextFieldState::move_home`],
///   and [`TextFieldState::move_end`] implement ordinary cursor navigation.
/// - [`TextFieldState::cursor_request_after_prefix`] converts the text cursor into a terminal
///   cursor request for a rendered field.
/// - [`TextFieldState::insert_char`], [`TextFieldState::backspace`], and [`TextFieldState::delete`]
///   implement canonical single-character editing.
///
/// # Examples
///
/// Edit a temporary dialog value without storing the text inside the field state:
///
/// ```rust
/// use std::string::String;
///
/// use ratatui_layout::TextFieldState;
///
/// let mut value = String::from("abcd");
/// let mut field = TextFieldState::new();
///
/// field.set_cursor(2, &value);
/// field.insert_char(&mut value, 'X');
/// field.backspace(&mut value);
/// field.move_right(&value);
/// field.delete(&mut value);
///
/// assert_eq!(value, "abc");
/// assert_eq!(field.cursor(), 3);
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextFieldState {
    cursor: usize,
}

impl TextFieldState {
    /// Creates field state with the cursor at the start.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::TextFieldState;
    ///
    /// let field = TextFieldState::new();
    ///
    /// assert_eq!(field.cursor(), 0);
    /// ```
    pub const fn new() -> Self {
        Self { cursor: 0 }
    }

    /// Creates field state with the cursor at the end of a value.
    ///
    /// Use this when opening an edit dialog from an existing record. The user can immediately
    /// append text, while left/right navigation still works in character coordinates.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::TextFieldState;
    ///
    /// let field = TextFieldState::at_end("owner");
    ///
    /// assert_eq!(field.cursor(), 5);
    /// ```
    pub fn at_end(value: &str) -> Self {
        Self {
            cursor: value.chars().count(),
        }
    }

    /// Returns the character cursor.
    ///
    /// The cursor is a character index, not a byte index. Use it for terminal cursor placement
    /// after accounting for the field label width.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::TextFieldState;
    ///
    /// let field = TextFieldState::at_end("ab");
    ///
    /// assert_eq!(field.cursor(), 2);
    /// ```
    pub const fn cursor(self) -> usize {
        self.cursor
    }

    /// Sets the cursor, clamped to the current value length.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::TextFieldState;
    ///
    /// let mut field = TextFieldState::new();
    /// field.set_cursor(10, "abc");
    ///
    /// assert_eq!(field.cursor(), 3);
    /// ```
    pub fn set_cursor(&mut self, cursor: usize, value: &str) {
        self.cursor = cursor.min(value.chars().count());
    }

    /// Clamps the cursor after the value was changed outside this state.
    ///
    /// This is useful after replacing a field value from autocomplete, paste handling, or a domain
    /// refresh.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::TextFieldState;
    ///
    /// let mut field = TextFieldState::at_end("abcdef");
    /// field.clamp_to("ab");
    ///
    /// assert_eq!(field.cursor(), 2);
    /// ```
    pub fn clamp_to(&mut self, value: &str) {
        self.set_cursor(self.cursor, value);
    }

    /// Moves the cursor one character left.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::TextFieldState;
    ///
    /// let mut field = TextFieldState::at_end("ab");
    /// field.move_left();
    ///
    /// assert_eq!(field.cursor(), 1);
    /// ```
    pub const fn move_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    /// Moves the cursor one character right, clamped to the value length.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::TextFieldState;
    ///
    /// let mut field = TextFieldState::new();
    /// field.move_right("ab");
    ///
    /// assert_eq!(field.cursor(), 1);
    /// ```
    pub fn move_right(&mut self, value: &str) {
        self.cursor = self.cursor.saturating_add(1).min(value.chars().count());
    }

    /// Moves the cursor to the beginning.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::TextFieldState;
    ///
    /// let mut field = TextFieldState::at_end("ab");
    /// field.move_home();
    ///
    /// assert_eq!(field.cursor(), 0);
    /// ```
    pub const fn move_home(&mut self) {
        self.cursor = 0;
    }

    /// Moves the cursor to the end of the value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::TextFieldState;
    ///
    /// let mut field = TextFieldState::new();
    /// field.move_end("abc");
    ///
    /// assert_eq!(field.cursor(), 3);
    /// ```
    pub fn move_end(&mut self, value: &str) {
        self.cursor = value.chars().count();
    }

    /// Returns a cursor request for a field with a fixed prefix.
    ///
    /// `prefix_width` is the rendered width before editable text, such as `"title: "`. The cursor
    /// is clamped to the field area so an overlong value does not request a position outside the
    /// visible row.
    ///
    /// # Examples
    ///
    /// Place the terminal cursor after a rendered label prefix:
    ///
    /// ```rust
    /// use ratatui_core::layout::{Position, Rect};
    /// use ratatui_layout::TextFieldState;
    ///
    /// let field = TextFieldState::at_end("abc");
    /// let request = field.cursor_request_after_prefix(Rect::new(10, 2, 20, 1), 7);
    ///
    /// assert_eq!(request.position, Position::new(20, 2));
    /// ```
    pub fn cursor_request_after_prefix(self, area: Rect, prefix_width: u16) -> CursorRequest {
        let x = area
            .x
            .saturating_add(prefix_width)
            .saturating_add(self.cursor as u16)
            .min(area.right().saturating_sub(1));
        CursorRequest::visible(Position::new(x, area.y))
    }

    /// Inserts a character at the cursor and moves the cursor after it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::string::String;
    ///
    /// use ratatui_layout::TextFieldState;
    ///
    /// let mut value = String::from("ac");
    /// let mut field = TextFieldState::new();
    /// field.set_cursor(1, &value);
    /// field.insert_char(&mut value, 'b');
    ///
    /// assert_eq!(value, "abc");
    /// assert_eq!(field.cursor(), 2);
    /// ```
    pub fn insert_char(&mut self, value: &mut String, ch: char) {
        let index = byte_index(value, self.cursor);
        value.insert(index, ch);
        self.cursor += 1;
    }

    /// Removes the character before the cursor and moves left.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::string::String;
    ///
    /// use ratatui_layout::TextFieldState;
    ///
    /// let mut value = String::from("abc");
    /// let mut field = TextFieldState::at_end(&value);
    /// field.backspace(&mut value);
    ///
    /// assert_eq!(value, "ab");
    /// assert_eq!(field.cursor(), 2);
    /// ```
    pub fn backspace(&mut self, value: &mut String) {
        if self.cursor == 0 {
            return;
        }
        let start = byte_index(value, self.cursor - 1);
        let end = byte_index(value, self.cursor);
        value.replace_range(start..end, "");
        self.cursor -= 1;
    }

    /// Removes the character at the cursor.
    ///
    /// Delete leaves the cursor in place. It is a no-op at the end of the value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::string::String;
    ///
    /// use ratatui_layout::TextFieldState;
    ///
    /// let mut value = String::from("abc");
    /// let mut field = TextFieldState::new();
    /// field.set_cursor(1, &value);
    /// field.delete(&mut value);
    ///
    /// assert_eq!(value, "ac");
    /// assert_eq!(field.cursor(), 1);
    /// ```
    pub fn delete(self, value: &mut String) {
        if self.cursor >= value.chars().count() {
            return;
        }
        let start = byte_index(value, self.cursor);
        let end = byte_index(value, self.cursor + 1);
        value.replace_range(start..end, "");
    }
}

/// Focus state for a horizontal row of buttons.
///
/// [`ButtonRowState`] owns the focused position inside a row, while the application owns the button
/// ids and actions. This is the small state helper behind a common modal pattern: left and right
/// keys move between `Cancel` and `Save`, while Enter or Space activates the focused id.
///
/// The helper is intentionally index-based internally. Callers pass the current visible button ids
/// when reading or changing focus, which keeps it useful for enum ids, integer ids, and filtered or
/// disabled button sets.
///
/// # Methods
///
/// - [`ButtonRowState::new`] starts at the first button.
/// - [`ButtonRowState::focused_index`] reads the current position.
/// - [`ButtonRowState::focused_id`] maps the current position to the caller's id slice.
/// - [`ButtonRowState::focus_index`] stores a clamped position.
/// - [`ButtonRowState::focus_id`] stores the position of a specific id.
/// - [`ButtonRowState::move_next`] and [`ButtonRowState::move_previous`] move horizontally with
///   wrapping.
///
/// # Examples
///
/// Keep a dialog button row aligned with enum ids:
///
/// ```rust
/// use ratatui_layout::ButtonRowState;
///
/// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// enum Action {
///     Cancel,
///     Save,
/// }
///
/// let buttons = [Action::Cancel, Action::Save];
/// let mut row = ButtonRowState::new();
///
/// row.move_next(buttons.len());
/// assert_eq!(row.focused_id(&buttons), Some(Action::Save));
/// row.focus_id(&buttons, &Action::Cancel);
/// assert_eq!(row.focused_index(), 0);
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ButtonRowState {
    focused: usize,
}

impl ButtonRowState {
    /// Creates button-row state focused on the first position.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::ButtonRowState;
    ///
    /// let row = ButtonRowState::new();
    ///
    /// assert_eq!(row.focused_index(), 0);
    /// ```
    pub const fn new() -> Self {
        Self { focused: 0 }
    }

    /// Returns the focused button position.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::ButtonRowState;
    ///
    /// let row = ButtonRowState::new();
    ///
    /// assert_eq!(row.focused_index(), 0);
    /// ```
    pub const fn focused_index(self) -> usize {
        self.focused
    }

    /// Returns the id at the focused position.
    ///
    /// Passing the current id slice keeps the state independent from the application's id type and
    /// from any buttons that may be hidden or disabled this frame.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::ButtonRowState;
    ///
    /// let row = ButtonRowState::new();
    ///
    /// assert_eq!(row.focused_id(&["cancel", "save"]), Some("cancel"));
    /// ```
    pub fn focused_id<Id: Copy>(self, ids: &[Id]) -> Option<Id> {
        ids.get(self.focused).copied()
    }

    /// Stores a focused index clamped to the visible button count.
    ///
    /// Empty rows always store zero so later non-empty rows start from the first button.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::ButtonRowState;
    ///
    /// let mut row = ButtonRowState::new();
    /// row.focus_index(10, 2);
    ///
    /// assert_eq!(row.focused_index(), 1);
    /// ```
    pub const fn focus_index(&mut self, index: usize, len: usize) {
        self.focused = if len == 0 {
            0
        } else if index >= len {
            len - 1
        } else {
            index
        };
    }

    /// Focuses the first matching id in the current visible button slice.
    ///
    /// Use this when focus is stored globally as an application enum and the button row needs to
    /// synchronize its local index before handling left/right movement.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::ButtonRowState;
    ///
    /// let mut row = ButtonRowState::new();
    /// row.focus_id(&["cancel", "save"], &"save");
    ///
    /// assert_eq!(row.focused_index(), 1);
    /// ```
    pub fn focus_id<Id: Eq>(&mut self, ids: &[Id], id: &Id) {
        if let Some(index) = ids.iter().position(|button| button == id) {
            self.focused = index;
        }
    }

    /// Moves focus to the next button, wrapping at the end.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::ButtonRowState;
    ///
    /// let mut row = ButtonRowState::new();
    /// row.move_next(2);
    /// row.move_next(2);
    ///
    /// assert_eq!(row.focused_index(), 0);
    /// ```
    pub const fn move_next(&mut self, len: usize) {
        if len == 0 {
            self.focused = 0;
        } else {
            self.focused = (self.focused + 1) % len;
        }
    }

    /// Moves focus to the previous button, wrapping at the start.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::ButtonRowState;
    ///
    /// let mut row = ButtonRowState::new();
    /// row.move_previous(2);
    ///
    /// assert_eq!(row.focused_index(), 1);
    /// ```
    pub const fn move_previous(&mut self, len: usize) {
        if len == 0 {
            self.focused = 0;
        } else {
            self.focused = (self.focused + len - 1) % len;
        }
    }
}

/// Button-row focus state that owns the row's ids.
///
/// [`ButtonRowState`] is intentionally minimal: callers pass the current visible id slice every
/// time they move or read focus. [`ButtonRow`] is the convenience wrapper for fixed or mostly fixed
/// rows where the same ids are used repeatedly. Dialogs with `Cancel` and `Save` buttons are the
/// canonical case: rendering owns labels and styles, while this helper owns only the row ids and
/// horizontal focus position.
///
/// Use [`ButtonRowState`] directly when the visible id slice is produced fresh each frame, filtered
/// by permissions, or borrowed from another component. Use [`ButtonRow`] when a component would
/// otherwise have to rebuild the same id vector for every left/right movement.
///
/// # Methods
///
/// - [`ButtonRow::new`] stores ids and starts focused on the first id.
/// - [`ButtonRow::ids`] exposes the owned ids for rendering or frame-snapshot construction.
/// - [`ButtonRow::focused_id`] returns the currently focused id.
/// - [`ButtonRow::focus_id`] synchronizes local button focus with global app focus.
/// - [`ButtonRow::move_next`] and [`ButtonRow::move_previous`] move horizontally with wrapping.
/// - [`ButtonRow::state`] and [`ButtonRow::state_mut`] expose the inner [`ButtonRowState`] when a
///   caller needs index-level control.
///
/// # Examples
///
/// Store fixed dialog button ids once and move through them with left/right keys:
///
/// ```rust
/// use ratatui_layout::ButtonRow;
///
/// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// enum DialogButton {
///     Cancel,
///     Save,
/// }
///
/// let mut buttons = ButtonRow::new([DialogButton::Cancel, DialogButton::Save]);
///
/// buttons.move_next();
/// assert_eq!(buttons.focused_id(), Some(DialogButton::Save));
/// buttons.focus_id(&DialogButton::Cancel);
/// assert_eq!(buttons.focused_id(), Some(DialogButton::Cancel));
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ButtonRow<Id = usize> {
    ids: Vec<Id>,
    state: ButtonRowState,
}

impl<Id> ButtonRow<Id> {
    /// Creates a button row from ids in left-to-right order.
    ///
    /// The row starts focused on the first id. Empty rows are allowed so a component can keep a
    /// single field even when no actions are currently visible.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::ButtonRow;
    ///
    /// let row = ButtonRow::new(["cancel", "save"]);
    ///
    /// assert_eq!(row.ids(), &["cancel", "save"]);
    /// ```
    pub fn new(ids: impl IntoIterator<Item = Id>) -> Self {
        Self {
            ids: ids.into_iter().collect(),
            state: ButtonRowState::new(),
        }
    }

    /// Returns the ids in left-to-right visual order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::ButtonRow;
    ///
    /// let row = ButtonRow::new([1, 2]);
    ///
    /// assert_eq!(row.ids(), &[1, 2]);
    /// ```
    pub fn ids(&self) -> &[Id] {
        &self.ids
    }

    /// Returns the inner position-based state.
    ///
    /// This is useful for diagnostics or for code that still needs the focused index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::ButtonRow;
    ///
    /// let row = ButtonRow::new(["cancel", "save"]);
    ///
    /// assert_eq!(row.state().focused_index(), 0);
    /// ```
    pub const fn state(&self) -> ButtonRowState {
        self.state
    }

    /// Returns mutable access to the inner position-based state.
    ///
    /// Use this when a caller has a reason to set the focused index directly while still keeping
    /// the ids owned by [`ButtonRow`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::ButtonRow;
    ///
    /// let mut row = ButtonRow::new(["cancel", "save"]);
    /// row.state_mut().focus_index(1, 2);
    ///
    /// assert_eq!(row.focused_id(), Some("save"));
    /// ```
    pub const fn state_mut(&mut self) -> &mut ButtonRowState {
        &mut self.state
    }

    /// Moves focus to the next id, wrapping at the end.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::ButtonRow;
    ///
    /// let mut row = ButtonRow::new(["cancel", "save"]);
    /// row.move_next();
    ///
    /// assert_eq!(row.focused_id(), Some("save"));
    /// ```
    pub const fn move_next(&mut self) {
        self.state.move_next(self.ids.len());
    }

    /// Moves focus to the previous id, wrapping at the start.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::ButtonRow;
    ///
    /// let mut row = ButtonRow::new(["cancel", "save"]);
    /// row.move_previous();
    ///
    /// assert_eq!(row.focused_id(), Some("save"));
    /// ```
    pub const fn move_previous(&mut self) {
        self.state.move_previous(self.ids.len());
    }
}

impl<Id: Copy> ButtonRow<Id> {
    /// Returns the currently focused id.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::ButtonRow;
    ///
    /// let row = ButtonRow::new(["cancel", "save"]);
    ///
    /// assert_eq!(row.focused_id(), Some("cancel"));
    /// ```
    pub fn focused_id(&self) -> Option<Id> {
        self.state.focused_id(&self.ids)
    }
}

impl<Id: Eq> ButtonRow<Id> {
    /// Focuses the first matching id.
    ///
    /// This synchronizes row-local focus with app-level focus before handling left/right movement.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_layout::ButtonRow;
    ///
    /// let mut row = ButtonRow::new(["cancel", "save"]);
    /// row.focus_id(&"save");
    ///
    /// assert_eq!(row.state().focused_index(), 1);
    /// ```
    pub fn focus_id(&mut self, id: &Id) {
        self.state.focus_id(&self.ids, id);
    }
}

fn byte_index(value: &str, cursor: usize) -> usize {
    value
        .char_indices()
        .nth(cursor)
        .map_or(value.len(), |(index, _)| index)
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    use super::*;

    #[test]
    fn text_field_edits_at_character_cursor() {
        let mut value = String::from("aé");
        let mut field = TextFieldState::at_end(&value);

        field.move_left();
        field.insert_char(&mut value, 'X');
        field.delete(&mut value);
        field.backspace(&mut value);

        assert_eq!(value, "a");
        assert_eq!(field.cursor(), 1);
    }

    #[test]
    fn text_field_clamps_cursor_to_value() {
        let mut field = TextFieldState::new();

        field.set_cursor(99, "abc");
        field.move_right("abc");
        field.clamp_to("a");

        assert_eq!(field.cursor(), 1);
    }

    #[test]
    fn text_field_builds_cursor_request_after_prefix() {
        let field = TextFieldState::at_end("abcdef");

        let request = field.cursor_request_after_prefix(Rect::new(10, 2, 12, 1), 7);

        assert_eq!(request.position, Position::new(21, 2));
    }

    #[test]
    fn button_row_moves_and_resolves_ids() {
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        enum Button {
            Cancel,
            Save,
        }

        let buttons = [Button::Cancel, Button::Save];
        let mut row = ButtonRowState::new();

        row.move_previous(buttons.len());
        assert_eq!(row.focused_id(&buttons), Some(Button::Save));
        row.focus_id(&buttons, &Button::Cancel);
        assert_eq!(row.focused_id(&buttons), Some(Button::Cancel));
    }

    #[test]
    fn owned_button_row_moves_and_resolves_ids() {
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        enum Button {
            Cancel,
            Save,
        }

        let mut row = ButtonRow::new([Button::Cancel, Button::Save]);

        row.move_previous();
        assert_eq!(row.focused_id(), Some(Button::Save));
        row.focus_id(&Button::Cancel);
        assert_eq!(row.focused_id(), Some(Button::Cancel));
        assert_eq!(row.ids(), &[Button::Cancel, Button::Save]);
    }
}
