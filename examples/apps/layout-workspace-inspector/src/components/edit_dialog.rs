//! Modal editor for the selected release-board item.
//!
//! The dialog is split by concern because modal components otherwise become hard to read quickly:
//! field ids describe the controls, state owns the temporary edit buffer, navigation owns focus rules,
//! and rendering owns the overlay frame snapshot. The parent module keeps the user-facing behavior in one
//! place so the app can treat the dialog as a single modal controller.

mod fields;
mod navigation;
mod render;
mod state;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui_layout::input::{ButtonRow, TextFieldState};

use self::fields::{DialogFieldKind, TextFieldId, button_fields};
use self::navigation::{DialogNavigation, focused_text_field};
pub(crate) use self::state::DialogState;
use crate::domain::ReleaseItem;
use crate::ids::{DialogField, TargetId};
use crate::ui::is_shift_tab;

/// High-level result of a modal dialog key press.
///
/// The dialog owns field editing and local focus policy. `App` owns durable side effects, so it
/// receives this outcome and decides whether to save, cancel, or update global focus.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum DialogOutcome {
    /// The dialog consumed the key without requiring app-level work.
    Continue,

    /// Focus should move to the given dialog target.
    Focus(TargetId),

    /// The user accepted the dialog.
    Save,

    /// The user cancelled the dialog.
    Cancel,
}

/// Intent decoded from one modal dialog key press.
///
/// This is local to the dialog because the app should not know whether a left arrow means cursor
/// movement, button movement, or status cycling. Separating decode from apply keeps those rules
/// visible without making `handle_key` perform every side effect inline.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum DialogKeyAction {
    /// No dialog behavior applies to the key.
    None,

    /// Cancel the dialog.
    Cancel,

    /// Move through all dialog controls in Tab order.
    Tab(isize),

    /// Move through the vertical stack of dialog controls.
    Vertical(isize),

    /// Move horizontally within the focused control.
    Horizontal(isize),

    /// Move the focused text cursor to the start.
    CursorHome,

    /// Move the focused text cursor to the end.
    CursorEnd,

    /// Activate the focused dialog control.
    Enter,

    /// Delete the character before the focused text cursor.
    Backspace,

    /// Delete the character after the focused text cursor.
    Delete,

    /// Insert a typed character into the focused text field.
    Insert(char),
}

/// Stateful controller for the modal edit dialog.
///
/// An `EditDialog` exists only while the app is in editing mode, so it always owns a
/// [`DialogState`]. That keeps modal state valid by construction: there is no separate "open" flag
/// and no empty dialog to accidentally render. `App` decides which item is being edited, sends modal
/// key presses here, and consumes the state returned by [`EditDialog::into_state`] when saving.
///
/// The dialog demonstrates overlay coordination. It renders above the page, clears its rectangle,
/// creates higher-z layout and mouse targets, contributes its own focus targets, and emits a
/// `CursorRequests` for editable fields. Keyboard input is modal while the dialog is open: `App` sends
/// keys here, and the dialog returns a [`DialogOutcome`] when the app needs to focus a target, save,
/// or cancel.
#[derive(Debug)]
pub(crate) struct EditDialog {
    /// Temporary values edited by the user before save.
    state: DialogState,

    /// Cursor state for the title field.
    title_field: TextFieldState,

    /// Cursor state for the owner field.
    owner_field: TextFieldState,

    /// Focus movement state and ids for the side-by-side button row.
    buttons: ButtonRow<DialogField>,
}

impl EditDialog {
    /// Opens a dialog with a copy of the selected item.
    ///
    /// Copying into [`DialogState`] gives cancel real semantics. The release item is not mutated until
    /// `App` consumes the dialog state and applies it to the domain model.
    pub(crate) fn open(item: &ReleaseItem) -> Self {
        let state = DialogState::from_item(item);
        let mut buttons = ButtonRow::new(button_fields());
        buttons.focus_id(&DialogField::Cancel);
        Self {
            title_field: TextFieldState::at_end(&state.title),
            owner_field: TextFieldState::at_end(&state.owner),
            buttons,
            state,
        }
    }

    /// Consumes the dialog and returns the edited values.
    ///
    /// This is the save boundary. Cancel simply drops the dialog, while save moves the temporary state
    /// back to `App` so it can update the selected domain item by stable id.
    pub(crate) fn into_state(self) -> DialogState {
        self.state
    }

    /// Handles one key press while the dialog is modal.
    ///
    /// Text editing, status cycling, field traversal, and button-row movement are all local dialog
    /// behavior. The returned [`DialogOutcome`] is the only part `App` needs to interpret.
    pub(crate) fn handle_key(&mut self, focused: Option<TargetId>, key: KeyEvent) -> DialogOutcome {
        let action = Self::key_action(focused, key);
        self.apply_key_action(focused, action)
    }

    /// Decodes a raw key press into dialog-local intent.
    fn key_action(focused: Option<TargetId>, key: KeyEvent) -> DialogKeyAction {
        if is_shift_tab(key) {
            return DialogKeyAction::Tab(-1);
        }
        match key.code {
            KeyCode::Esc => DialogKeyAction::Cancel,
            KeyCode::Tab => DialogKeyAction::Tab(1),
            KeyCode::Down => DialogKeyAction::Vertical(1),
            KeyCode::Up => DialogKeyAction::Vertical(-1),
            KeyCode::Right => DialogKeyAction::Horizontal(1),
            KeyCode::Left => DialogKeyAction::Horizontal(-1),
            KeyCode::Home => DialogKeyAction::CursorHome,
            KeyCode::End => DialogKeyAction::CursorEnd,
            KeyCode::Enter => DialogKeyAction::Enter,
            KeyCode::Char(' ') if DialogFieldKind::is_button(focused) => DialogKeyAction::Enter,
            KeyCode::Backspace => DialogKeyAction::Backspace,
            KeyCode::Delete => DialogKeyAction::Delete,
            KeyCode::Char(ch) => DialogKeyAction::Insert(ch),
            _ => DialogKeyAction::None,
        }
    }

    /// Applies dialog-local intent to temporary state or returns an app-level outcome.
    fn apply_key_action(
        &mut self,
        focused: Option<TargetId>,
        action: DialogKeyAction,
    ) -> DialogOutcome {
        match action {
            DialogKeyAction::None => DialogOutcome::Continue,
            DialogKeyAction::Cancel => DialogOutcome::Cancel,
            DialogKeyAction::Tab(delta) => DialogNavigation::tab(focused, delta),
            DialogKeyAction::Vertical(delta) => DialogNavigation::vertical(focused, delta),
            DialogKeyAction::Horizontal(delta) => self.move_horizontal(focused, delta),
            DialogKeyAction::CursorHome => {
                self.move_cursor_home(focused);
                DialogOutcome::Continue
            }
            DialogKeyAction::CursorEnd => {
                self.move_cursor_end(focused);
                DialogOutcome::Continue
            }
            DialogKeyAction::Enter => Self::enter(focused),
            DialogKeyAction::Backspace => {
                self.pop_field(focused);
                DialogOutcome::Continue
            }
            DialogKeyAction::Delete => {
                self.delete_field(focused);
                DialogOutcome::Continue
            }
            DialogKeyAction::Insert(ch) => {
                self.push_field(focused, ch);
                DialogOutcome::Continue
            }
        }
    }

    /// Removes one character from the focused editable field.
    ///
    /// The focused routed target is narrowed to [`TextFieldId`] so status, save, and cancel never
    /// receive text editing behavior.
    fn pop_field(&mut self, focused: Option<TargetId>) {
        self.edit_focused_text(focused, |value, state| state.backspace(value));
    }

    /// Removes one character after the cursor in the focused editable field.
    ///
    /// Delete differs from Backspace by leaving the cursor in place. Buttons ignore it because they
    /// are focus targets, not editable values.
    fn delete_field(&mut self, focused: Option<TargetId>) {
        self.edit_focused_text(focused, |value, state| state.delete(value));
    }

    /// Adds one character to the focused editable field.
    ///
    /// Text input is handled by the dialog only while it is modal. `App` decides that mode, and this
    /// method applies the character to the temporary field buffer.
    fn push_field(&mut self, focused: Option<TargetId>, ch: char) {
        self.edit_focused_text(focused, |value, state| state.insert_char(value, ch));
    }

    /// Cycles the constrained status field when it is focused.
    fn cycle_status(&mut self, focused: Option<TargetId>, delta: isize) {
        if focused == Some(TargetId::Dialog(DialogField::Status)) {
            self.state.cycle_status(delta);
        }
    }

    /// Moves the cursor left inside the focused editable field.
    fn move_cursor_left(&mut self, focused: Option<TargetId>) {
        if let Some(field) = focused_text_field(focused) {
            self.field_state_mut(field).move_left();
        }
    }

    /// Moves the cursor right inside the focused editable field.
    fn move_cursor_right(&mut self, focused: Option<TargetId>) {
        self.edit_focused_text(focused, |value, state| state.move_right(value));
    }

    /// Moves the cursor to the start of the focused editable field.
    fn move_cursor_home(&mut self, focused: Option<TargetId>) {
        if let Some(field) = focused_text_field(focused) {
            self.field_state_mut(field).move_home();
        }
    }

    /// Moves the cursor to the end of the focused editable field.
    fn move_cursor_end(&mut self, focused: Option<TargetId>) {
        self.edit_focused_text(focused, |value, state| state.move_end(value));
    }

    /// Moves focus horizontally within the dialog button row.
    ///
    /// The global app focus still owns the actual focused `TargetId`. This helper keeps the row-local
    /// button index in `ButtonRow`, then returns the next dialog button field for `App` to focus.
    fn move_button_focus(&mut self, focused: DialogField, delta: isize) -> Option<DialogField> {
        self.buttons.focus_id(&focused);
        if delta >= 0 {
            self.buttons.move_next();
        } else {
            self.buttons.move_previous();
        }
        self.buttons.focused_id()
    }

    /// Moves inside the currently focused dialog control.
    fn move_horizontal(&mut self, focused: Option<TargetId>, delta: isize) -> DialogOutcome {
        match (focused, delta) {
            (Some(TargetId::Dialog(field @ DialogField::Cancel)), 1..)
            | (Some(TargetId::Dialog(field @ DialogField::Save)), ..=-1) => {
                self.move_button(field, delta)
            }
            (Some(TargetId::Dialog(DialogField::Status)), delta) => {
                self.cycle_status(focused, delta);
                DialogOutcome::Continue
            }
            (_, 1..) => {
                self.move_cursor_right(focused);
                DialogOutcome::Continue
            }
            (_, ..=-1) => {
                self.move_cursor_left(focused);
                DialogOutcome::Continue
            }
            _ => DialogOutcome::Continue,
        }
    }

    /// Moves between side-by-side buttons and returns the target that should receive focus.
    fn move_button(&mut self, field: DialogField, delta: isize) -> DialogOutcome {
        self.move_button_focus(field, delta)
            .map_or(DialogOutcome::Continue, |field| {
                DialogOutcome::Focus(TargetId::Dialog(field))
            })
    }

    /// Activates the focused dialog control.
    const fn enter(focused: Option<TargetId>) -> DialogOutcome {
        match focused {
            Some(TargetId::Dialog(DialogField::Cancel)) => DialogOutcome::Cancel,
            _ => DialogOutcome::Save,
        }
    }

    /// Returns immutable text field state for a free-text dialog field.
    const fn field_state(&self, field: TextFieldId) -> TextFieldState {
        match field {
            TextFieldId::Title => self.title_field,
            TextFieldId::Owner => self.owner_field,
        }
    }

    /// Returns mutable text field state for a free-text dialog field.
    const fn field_state_mut(&mut self, field: TextFieldId) -> &mut TextFieldState {
        match field {
            TextFieldId::Title => &mut self.title_field,
            TextFieldId::Owner => &mut self.owner_field,
        }
    }

    /// Returns a mutable text value and matching field state.
    const fn text_field_mut(&mut self, field: TextFieldId) -> (&mut String, &mut TextFieldState) {
        let value = self.state.text_value_mut(field);
        match field {
            TextFieldId::Title => (value, &mut self.title_field),
            TextFieldId::Owner => (value, &mut self.owner_field),
        }
    }

    /// Applies an operation to the focused free-text field.
    ///
    /// The status picker and buttons are focus targets too, but they should ignore typed characters
    /// and text-editing keys. Centralizing that narrowing keeps each editing method focused on the
    /// operation it performs.
    fn edit_focused_text(
        &mut self,
        focused: Option<TargetId>,
        edit: impl FnOnce(&mut String, &mut TextFieldState),
    ) {
        if let Some(field) = focused_text_field(focused) {
            let (value, state) = self.text_field_mut(field);
            edit(value, state);
        }
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent};
    use ratatui::layout::Rect;
    use ratatui_layout::input::TextFieldState;

    use super::fields::{DIALOG_HEIGHT, DIALOG_WIDTH};
    use super::{DialogKeyAction, DialogOutcome, DialogState, EditDialog};
    use crate::domain::Status;
    use crate::ids::{DialogField, TargetId};

    #[test]
    fn dialog_plans_non_empty_area_for_every_field() {
        let dialog = EditDialog::from_state(DialogState::for_test());
        let area = Rect::new(0, 0, DIALOG_WIDTH, DIALOG_HEIGHT);
        let container = dialog.layout(area);
        let rows = dialog.rows(container.inner);

        for field in [
            DialogField::Title,
            DialogField::Owner,
            DialogField::Status,
            DialogField::Save,
            DialogField::Cancel,
        ] {
            let region = rows.region_for(field).expect("dialog field is planned");
            assert_eq!(region.area.height, 1, "{field:?} row should be visible");
        }
    }

    #[test]
    fn dialog_plans_save_and_cancel_side_by_side() {
        let dialog = EditDialog::from_state(DialogState::for_test());
        let area = Rect::new(0, 0, DIALOG_WIDTH, DIALOG_HEIGHT);
        let container = dialog.layout(area);
        let rows = dialog.rows(container.inner);

        let save = rows.region_for(DialogField::Save).expect("save is planned");
        let cancel = rows
            .region_for(DialogField::Cancel)
            .expect("cancel is planned");

        assert_eq!(save.area.y, cancel.area.y);
        assert!(cancel.area.right() < save.area.x);
    }

    #[test]
    fn dialog_edits_text_at_the_field_cursor() {
        let mut dialog = EditDialog::from_state(DialogState {
            title: "abcd".to_string(),
            owner: String::new(),
            status: Status::Queued,
        });
        dialog.title_field = TextFieldState::new();
        dialog.title_field.set_cursor(2, "abcd");
        let focused = Some(TargetId::Dialog(DialogField::Title));

        dialog.push_field(focused, 'X');
        dialog.pop_field(focused);
        dialog.move_cursor_right(focused);
        dialog.delete_field(focused);

        assert_eq!(dialog.state.title, "abc");
        assert_eq!(dialog.title_field.cursor(), 3);
    }

    #[test]
    fn dialog_cursor_clamps_to_field_end() {
        let mut cursor = TextFieldState::at_end("ab");

        cursor.move_right("ab");

        assert_eq!(cursor.cursor(), 2);
    }

    #[test]
    fn dialog_cycles_status_instead_of_editing_text() {
        let mut dialog = EditDialog::from_state(DialogState::for_test());
        let focused = Some(TargetId::Dialog(DialogField::Status));
        let key = KeyEvent::from(KeyCode::Right);

        dialog.push_field(focused, 'x');
        dialog.handle_key(focused, key);
        dialog.cycle_status(focused, -1);

        assert_eq!(dialog.state.status, Status::Queued);
    }

    #[test]
    fn dialog_decodes_keys_before_applying_behavior() {
        let focused = Some(TargetId::Dialog(DialogField::Cancel));

        assert_eq!(
            EditDialog::key_action(focused, KeyEvent::from(KeyCode::Char(' '))),
            DialogKeyAction::Enter
        );
        assert_eq!(
            EditDialog::key_action(focused, KeyEvent::from(KeyCode::Backspace)),
            DialogKeyAction::Backspace
        );
    }

    #[test]
    fn dialog_actions_return_app_level_outcomes() {
        let mut dialog = EditDialog::from_state(DialogState::for_test());
        let focused = Some(TargetId::Dialog(DialogField::Title));

        assert_eq!(
            dialog.apply_key_action(
                Some(TargetId::Dialog(DialogField::Save)),
                DialogKeyAction::Enter
            ),
            DialogOutcome::Save
        );
        assert_eq!(
            dialog.apply_key_action(focused, DialogKeyAction::Tab(1)),
            DialogOutcome::Focus(TargetId::Dialog(DialogField::Owner))
        );
    }
}
