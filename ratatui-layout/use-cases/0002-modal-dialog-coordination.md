# 0002 Modal Dialog Coordination

## User Story

An app opens a raised surface over the current screen. The page behind it remains visible, but input
belongs to the dialog until the user dismisses it or completes the dialog action.

The dialog might contain a confirmation prompt, a help overlay, a picker, a details preview, or a
form. Those contents matter to the app, but the dialog coordination problem is narrower: place the
frontmost region, keep routing local to it, and make the next event use the same regions the user
just saw.

## Concrete App Shapes

### Confirmation Prompt

A release dashboard asks before cancelling a deployment:

```text
cancel deployment?

[ no ]  [ yes, cancel ]
```

The app needs the prompt to sit above the board, route clicks to the visible buttons, and make
Escape or `No` dismiss the prompt. It may also dismiss when the user clicks outside the prompt, or
it may intentionally ignore outside clicks. There may be no editable fields at all.

### Help Overlay

A complex inspector opens a help overlay with command descriptions. It needs to clear and draw above
the page, capture `Esc`, and route scroll events inside the help text if the content is long.

This is a dialog even though it has no form state and no Save button.

### Picker Dialog

A file picker, command picker, or connection picker shows a filtered list over the current page. It
needs a raised region, focused rows, mouse hit testing, and either Enter or click activation. The
selected value belongs to the app; the dialog only contributes visible routing data.

### Form Dialog

An edit dialog places fields and Save/Cancel buttons inside the same raised region. This combines
dialog coordination with the separate form-editing use case. The dialog owns the overlay behavior;
the form owns edit buffers, text cursors, validation, and submit/cancel policy.

## Core Necessity

All modal dialogs need the same frame-local mechanics:

1. Compute an outer region that visually sits above the page.
1. Compute an inner region for content without forcing a retained widget tree.
1. Record z-order so pointer routing agrees with the visual stack.
1. Keep keyboard focus inside the dialog while it is open.
1. Route mouse clicks, hover, and scroll to frontmost dialog targets.
1. Decide what a click outside the dialog means by checking for a negative hit or routing through an
   explicit backdrop target.
1. Optionally request a terminal cursor from the active child control.
1. Return one frame-local artifact that the parent app stores for the next input event.

The contents of the dialog can still be ordinary Ratatui rendering. The crate should coordinate
regions and input data, not decide whether the dialog is a form, picker, help view, or prompt.

## Current Crate Path

Use [`Container`](../src/container.rs) and [`Padding`](../src/container.rs) to derive the outer
dialog region and the inner content region. The app can render `Clear` and a Ratatui `Block` into
the outer region, then render any content into the inner region.

```rust
use ratatui_core::layout::Rect;
use ratatui_layout::{Container, Padding};

let outer = Rect::new(10, 4, 52, 10);
let dialog = Container::<()>::new()
    .padding(Padding::new(2, 2, 2, 1))
    .layout(outer);

assert_eq!(dialog.outer, outer);
assert!(dialog.inner.width < dialog.outer.width);
```

Use [`Column`](../src/linear.rs), [`Row`](../src/linear.rs), or
[`Overlay`](../src/overlay.rs) to produce named content regions. Enum ids are usually the clearest
choice because the app will match on them when handling focus, activation, or dismissal.

```rust
use ratatui_core::layout::{Constraint, Rect};
use ratatui_layout::Row;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum PromptAction {
    No,
    YesCancel,
}

let prompt_buttons = [
    (PromptAction::No, Constraint::Length(8)),
    (PromptAction::YesCancel, Constraint::Length(14)),
];
let buttons = Row::named(prompt_buttons)
    .spacing(2)
    .regions(Rect::new(12, 10, 36, 1));
```

Use [`FrameTargets`](../src/frame.rs) or [`FrameSnapshot`](../src/frame.rs) to return the data the
dialog produced. The z value is what makes the modal route before the page behind it.

```rust
use ratatui_core::layout::{Constraint, Rect};
use ratatui_layout::{FrameTargets, Row};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum PromptAction {
    No,
    YesCancel,
}

let prompt_buttons = [
    (PromptAction::No, Constraint::Length(8)),
    (PromptAction::YesCancel, Constraint::Length(14)),
];
let buttons = Row::named(prompt_buttons).regions(Rect::new(12, 10, 30, 1));

let frame = FrameTargets::from_regions(buttons, 100).z(20).build();

assert_eq!(frame.mouse.hit_test((13, 10)).unwrap().id, PromptAction::No);
```

Clicking outside the dialog is its own policy decision. A modal confirmation might dismiss, a
destructive form might ignore the click, and a picker might clear hover without closing. If the app
routes against the dialog frame before the page behind it, a negative hit means the click missed the
dialog.

```rust
use ratatui_core::layout::{Constraint, Rect};
use ratatui_layout::{FrameTargets, Row};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum PromptAction {
    No,
    YesCancel,
}

let prompt_buttons = [
    (PromptAction::No, Constraint::Length(8)),
    (PromptAction::YesCancel, Constraint::Length(14)),
];
let buttons = Row::named(prompt_buttons).regions(Rect::new(12, 10, 30, 1));
let frame = FrameTargets::from_regions(buttons, 100).z(20).build();

assert!(frame.route_click((2, 2)).is_none());

// The app decides whether that negative hit dismisses the dialog or is ignored.
```

When the app wants outside clicks to be routed explicitly, add a backdrop target behind the dialog's
child targets. Region targets still win inside the dialog because they are added after the whole-region
mouse target.

```rust
use ratatui_core::layout::{Constraint, Rect};
use ratatui_layout::{FrameTargets, Row};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum PromptAction {
    No,
    YesCancel,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum DialogTarget {
    Backdrop,
    Button(PromptAction),
}

let screen = Rect::new(0, 0, 80, 24);
let prompt_buttons = [
    (PromptAction::No, Constraint::Length(8)),
    (PromptAction::YesCancel, Constraint::Length(14)),
];
let buttons = Row::named(prompt_buttons).regions(Rect::new(12, 10, 30, 1));

let frame = FrameTargets::new(screen, 100)
    .z(20)
    .mouse_region(DialogTarget::Backdrop, screen)
    .build_focusable(buttons.regions().iter().copied(), DialogTarget::Button);

assert_eq!(frame.route_click((2, 2)).unwrap().id, DialogTarget::Backdrop);
assert_eq!(
    frame.route_click((13, 10)).unwrap().id,
    DialogTarget::Button(PromptAction::No),
);
```

Use [`FocusTargets::from_regions`](../src/focus.rs) when every planned dialog region should
participate in keyboard traversal.

```rust
use ratatui_core::layout::{Constraint, Rect};
use ratatui_layout::{FocusFallback, FocusTargets, FocusState, Row};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum DialogButton {
    Cancel,
    Confirm,
}

let dialog_buttons = [
    (DialogButton::Cancel, Constraint::Length(10)),
    (DialogButton::Confirm, Constraint::Length(10)),
];
let buttons = Row::named(dialog_buttons).regions(Rect::new(0, 0, 22, 1));

let focus_plan = FocusTargets::from_regions(buttons.regions().iter().copied());
let mut focus = FocusState::default();

focus.ensure_visible(&focus_plan, FocusFallback::First);
```

Use [`CursorRequests`](../src/cursor.rs) only when a child control needs the terminal cursor. A
confirmation prompt or help overlay may never request one. A form field inside the dialog can
contribute one cursor request, and the parent app can apply the final cursor after all frame snapshots
are merged.

## Coordination Data Analysis

A dialog does not always need every coordination type. The useful bundle depends on what the dialog
contains and how outside interaction should behave.

| Shape               | Layout | Mouse | Focus | Selection | Cursor | Aggregate |
| ------------------- | ------ | ----- | ----- | --------- | ------ | --------- |
| Confirmation prompt | yes    | yes   | maybe | no        | no     | maybe     |
| Help overlay        | yes    | maybe | no    | no        | no     | maybe     |
| Picker dialog       | yes    | yes   | maybe | yes       | no     | maybe     |
| Form dialog         | yes    | maybe | yes   | maybe     | yes    | maybe     |

The common dialog data are geometry, z-order, and routing boundary. Focus, selection, and cursor
data come from the dialog contents. This is why the dialog primitive should stay smaller than a
form primitive.

For a prompt, the simple shape can be a button layout plus a backdrop target:

```rust
let frame = FrameTargets::new(screen, 100)
    .z(20)
    .mouse_region(DialogTarget::Backdrop, screen)
    .build_focusable(buttons.regions().iter().copied(), DialogTarget::Button);
let clicked = frame.route_click(position);
```

For a help overlay, the useful data may be only the outer region and a scroll target:

```rust
let frame = FrameSnapshot::new(screen)
    .scroll_region(DialogTarget::HelpText, help_area);
```

For a form dialog, the dialog shell and the form contents can remain separate:

```rust
let dialog = FrameSnapshot::new(screen).mouse_target(backdrop);
let form = render_form_fields().map_id(DialogTarget::Form);
let frame = dialog.merge(form);
```

The examples above are intentionally different. A single dialog helper should not force all three
shapes to carry focus, selection, and cursor requests when they do not need them.

Runnable examples:

- `cargo run -p ratatui-layout --example modal_prompt`
- `cargo run -p ratatui-layout --example container_dialog`
- `cargo run -p ratatui-layout --example frame_snapshot`
- `cargo run --manifest-path examples/apps/layout-workspace-inspector/Cargo.toml`

## What This Validates

The crate already covers the dialog mechanics without defining a dialog widget:

- container layout separates the modal shell from ordinary Ratatui rendering;
- named regions make rendered regions routeable by app-owned ids;
- z-order lets the dialog win pointer routing over the page behind it;
- negative hits and backdrop targets make outside-click behavior explicit instead of accidental;
- focus target collections and focus state keep keyboard input local while the dialog is open;
- frame snapshots give the parent app one artifact to store after rendering and use for the next event;
- cursor requests remain optional child data rather than a dialog requirement.

The useful design pressure is to make overlay coordination explicit and repeatable without deciding
what the dialog contains.

## Simplicity Check

The current pieces cover the core dialog mechanics:

- `Container` gives outer and inner regions without owning rendering.
- `Row`, `Column`, and `Overlay` produce child regions with app-owned ids.
- `FrameTargets` builds aligned layout, mouse, and focus target data when a child set needs them.
- `FrameSnapshot::new`, `FrameSnapshot::mouse_target`, and `FrameSnapshot::scroll_region` support dialog-level
  data that are not tied to child regions.
- `FrameSnapshot::merge` lets the shell and contents stay separate.
- `route_click` returning `None` is enough for negative-hit outside-click behavior when the app does
  not want an explicit backdrop id.

The rough spots are call-site shape and naming, not missing capability. A small helper may be worth
exploring if examples keep repeating the same shell code. The `modal_prompt` example currently has
to compute a container, build named button regions, add a backdrop target, apply a high z value, and
merge that data by hand:

```rust
let shell = ModalShell::new(screen, dialog_area)
    .z(20)
    .backdrop(DialogTarget::Backdrop)
    .layout();
let frame = shell.frame().merge(content_frame);
```

That helper would be useful only if it stays narrow:

- compute outer, inner, and clipping areas;
- apply a z value consistently;
- optionally add a backdrop mouse target;
- merge child data into the dialog coordinate space.

It should not own buttons, fields, validation, submit/cancel behavior, or focus policy for every
dialog. Those belong to ordered action surfaces, form editing, picker/list selection, or the app.

## Related Use Cases

- [`0001 Ordered Action Surfaces`](0001-command-toolbar-routing.md) covers the button row shape used
  by prompts and dialogs.
- [`0009 Form Editing`](0009-form-editing.md) covers text fields, edit buffers, validation, and
  save/cancel policy that may appear inside a dialog.
- [`0008 Mouse Overlays And Disabled Targets`](0008-mouse-overlays-and-disabled-targets.md) covers
  overlapping pointer targets and disabled controls in more detail.

## Remaining Design Questions

Keep dialog coordination separate from form semantics. The remaining questions are dialog-level:

- whether there should be a small helper for "raised modal frame with inner content area";
- how focus trapping should be represented when the page still has a previous frame snapshot;
- whether outside-click behavior needs a helper or should remain an ordinary negative-hit/backdrop
  routing pattern;
- how scroll regions inside dialogs should compose with page scroll regions behind them;
- whether dismissal policy should remain app-specific or get a tiny route helper;
- how much frame-snapshot construction can be wrapped before it hides useful app decisions.
