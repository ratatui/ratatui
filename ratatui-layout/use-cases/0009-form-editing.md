# 0009 Form Editing

## User Story

An app lets the user edit a small set of values. The form might live in a modal dialog, a side pane,
a settings page, or an inline editor. The app owns the domain object and usually owns a temporary
edit buffer so changes can be saved, cancelled, or validated before they affect durable state.

The form coordination problem is about fields and edit state, not about whether the form is modal.
It needs field ids, text cursors, focus traversal, validation policy, buttons or commands, and
cursor placement for the active field.

## Concrete App Shapes

### Inline Row Editor

A table row expands into editable fields for title and owner. The page remains interactive outside
the row, but the focused field still needs text editing and terminal cursor placement.

### Settings Page

A settings page edits a group of values and enables Apply only when pending values differ from the
saved configuration. The form is not a dialog, but it still has field focus, disabled buttons, and
validation state.

### Connection Profile Editor

A database client edits host, port, username, and database name. The edit buffer may be loaded from
a selected profile, discarded with Cancel, or saved back to the profile list.

### Modal Edit Form

A modal edit dialog combines this use case with dialog coordination. The dialog supplies the raised
region and routing boundary; the form supplies field ids, edit buffers, text cursor state, and
submit/cancel behavior.

## Core Necessity

Form editing needs a different set of data than modal dialogs:

1. The app owns current values and pending values.
1. Each editable field owns text cursor state.
1. The render pass assigns each visible field a rectangle.
1. Focus traversal moves through visible fields and commands.
1. Text input edits the focused field's pending value.
1. The focused text field requests terminal cursor placement.
1. Validation can mark fields or commands as disabled without hiding them.
1. Save, Apply, Reset, and Cancel remain app-owned actions.

The crate should reduce repeated field coordination and cursor math. It should not own domain data,
validation rules, or submit side effects.

## Current Crate Path

Use [`Column`](../src/linear.rs), [`Row`](../src/linear.rs), and named regions to give each field and
command an app-owned id. Enum ids are usually the clearest because the app will match on them when
editing values or applying commands.

```rust
use ratatui_core::layout::{Constraint, Rect};
use ratatui_layout::Column;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Field {
    Title,
    Owner,
    Status,
}

let field_rows = [
    (Field::Title, Constraint::Length(1)),
    (Field::Owner, Constraint::Length(1)),
    (Field::Status, Constraint::Length(1)),
];
let fields = Column::named(field_rows)
    .spacing(1)
    .regions(Rect::new(2, 2, 40, 5));
```

Use [`FocusTargets::from_regions`](../src/focus.rs) when every visible field should participate in
traversal. Use a custom [`FocusTargets`](../src/focus.rs) when commands, disabled fields, or non-visual
ordering need more policy.

```rust
use ratatui_core::layout::{Constraint, Rect};
use ratatui_layout::Column;
use ratatui_layout::{FocusFallback, FocusTargets, FocusState};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Field {
    Title,
    Owner,
}

let field_rows = [
    (Field::Title, Constraint::Length(1)),
    (Field::Owner, Constraint::Length(1)),
];
let fields = Column::named(field_rows).regions(Rect::new(0, 0, 20, 2));

let focus_plan = FocusTargets::from_regions(fields.regions().iter().copied());
let mut focus = FocusState::default();

focus.ensure_visible(&focus_plan, FocusFallback::First);
focus.next(&focus_plan);
```

Use [`TextFieldState`](../src/input.rs) beside each editable string. The helper owns the character
cursor and canonical single-line editing operations. The string stays in the app's pending edit
buffer.

```rust
use ratatui_core::layout::{Position, Rect};
use ratatui_layout::TextFieldState;

let mut title = String::from("deplo");
let mut title_field = TextFieldState::at_end(&title);

title_field.insert_char(&mut title, 'y');
assert_eq!(title, "deploy");

let area = Rect::new(4, 8, 30, 1);
let prefix_width = "title: ".len() as u16;
let request = title_field.cursor_request_after_prefix(area, prefix_width);
assert_eq!(request.position, Position::new(17, 8));
```

Use [`ButtonRow`](../src/input.rs) for local left/right movement across a form command row. The row
does not decide what Save or Cancel means; it only tracks which visible command is focused.

```rust
use ratatui_layout::ButtonRow;

#[derive(Clone, Copy, Eq, PartialEq)]
enum FormCommand {
    Cancel,
    Save,
}

let mut buttons = ButtonRow::new([FormCommand::Cancel, FormCommand::Save]);
buttons.move_next();

assert_eq!(buttons.focused_id(), Some(FormCommand::Save));
```

Use [`CursorRequests`](../src/cursor.rs) to collect the cursor request produced by the focused field.
The parent app can merge that request with other frame-local data before applying the final
terminal cursor.

Runnable examples:

- `cargo run -p ratatui-layout --example form_dialog`
- `cargo run -p ratatui-layout --example profile_editor`
- `cargo run -p ratatui-layout --example cursor_request`
- `cargo run --manifest-path examples/apps/layout-workspace-inspector/Cargo.toml`

## Coordination Data Analysis

Form editing is a mixed concern, but the reusable parts are small. A form needs layout and focus for
field traversal, cursor requests for focused text fields, and app-owned state for pending values.
Mouse and frame aggregation depend on where the form lives.

| Shape                     | Layout | Mouse | Focus | Selection | Cursor | Frame |
| ------------------------- | ------ | ----- | ----- | --------- | ------ | ----- |
| Inline row editor         | yes    | maybe | yes   | maybe     | yes    | maybe |
| Settings page             | yes    | maybe | yes   | maybe     | yes    | maybe |
| Connection profile editor | yes    | maybe | yes   | no        | yes    | maybe |
| Modal edit form           | yes    | yes   | yes   | maybe     | yes    | yes   |
| Command button row        | yes    | maybe | yes   | maybe     | no     | maybe |

This argues for small helpers rather than a single `Form` type. [`TextFieldState`](../src/input.rs)
should make editing a string and placing the cursor boring. [`ButtonRow`](../src/input.rs) should
make horizontal command focus boring. The app should still own validation, save/cancel effects, and
the pending edit buffer.

## What This Validates

The crate already covers the small reusable pieces of form editing:

- named layout regions keep field rendering and input routing aligned;
- focus state remains app-owned and is repaired against visible fields;
- text field state owns cursor/edit mechanics without owning field values;
- cursor request helpers remove repeated terminal coordinate arithmetic;
- button rows handle local command movement without defining submit semantics.

The `form_dialog` example follows the smallest path directly: fields are solved as named regions,
focus is repaired against the visible field regions, text editing goes through `TextFieldState`, and
the focused field emits a cursor request from its label prefix.

The `profile_editor` example exercises the same pieces in a fuller settings-form shape. It keeps a
saved profile separate from a pending draft, validates the draft, disables Save when the draft is
invalid or unchanged, and combines vertical field traversal with horizontal button movement.

The useful design pressure is to make form code read as a workflow over pending values, not as a
mix of index bookkeeping and cursor math.

## Related Use Cases

- [`0002 Modal Dialog Coordination`](0002-modal-dialog-coordination.md) covers the raised overlay
  surface that can contain a form.
- [`0001 Ordered Action Surfaces`](0001-command-toolbar-routing.md) covers Save/Cancel button rows,
  segmented controls, and radio-like choices.
- [`0003 Filtered Record Selection`](0003-filtered-record-selection.md) covers stable selected
  records that may be loaded into an edit buffer.
- [`0010 Frame Snapshots`](0010-frame-snapshots.md) explains why field state, frame-local data, and
  app-owned pending values should stay separate.

## Remaining Design Questions

The current model is enough for small single-line forms. Do not add a full form abstraction yet.
Several choices still need more examples:

- how validation errors should reserve space, affect focus, and disable Save;
- whether submit/cancel semantics belong in reusable helpers or stay app-specific;
- how multiline fields, completions, and horizontally clipped text should request the cursor;
- whether field labels, value spans, and cursor prefixes need a small helper or should stay local.
