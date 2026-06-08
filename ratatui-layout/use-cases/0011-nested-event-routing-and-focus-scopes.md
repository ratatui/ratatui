# 0011 Nested Event Routing And Focus Scopes

## User Story

An app author is building a screen with meaningful local relationships: a form inside a details
pane, inline buttons inside table rows, a command strip inside a dialog, or a popup over an editor.
The app can identify the rectangle under the pointer and the currently focused id, but it also
needs to know how nearby and nested controls relate to each other.

The author needs a way to answer:

- Which target is the leaf that should see this input first?
- Which parent groups should see the input after the leaf declines or handles it?
- Which focus scope owns Tab, Shift-Tab, arrow movement, and Escape?
- Which pointer target captures a drag or release after the press starts?
- Which page-level shortcuts should still run when a child field, menu, or dialog has focus?

This is the relationship and routing-order question behind richer apps. It is adjacent to
`FrameSnapshot`, but it is not the same problem as storing rectangles or hit targets.

## Scenarios

### Form Group Inside A Pane

A details pane contains editable fields, a Save/Cancel row, and pane-level shortcuts. A text field
should handle character input, Backspace, Delete, Home, End, and local cursor movement. The form
group should handle Tab and Shift-Tab across fields and buttons. The pane may handle Escape or help,
and the page may handle global commands.

The app needs a clear route from focused field to form group to pane to page. A single flat
`FocusState<Target>` can name the focused field, but it does not describe the parent chain.

### Inline Controls Inside Rows

A list row may contain a checkbox, row title, status pill, and action button. A click on the button
should activate the button first. A click on row background should select the row. Keyboard focus
may move between rows, while local arrow keys or Space may act on the inline control.

The app needs both hit testing and containment. The target under the pointer is a leaf, but the row
is also meaningful context.

### Modal Or Popup Above A Page

A modal dialog or popup sits above page content. Pointer routing should prefer the topmost child.
Clicking outside may dismiss the popup, while clicking through to the page may be blocked. Keyboard
focus may be trapped inside the modal until it closes.

The app needs topmost hit testing, negative hit behavior, and focus-scope policy. Z-order alone
does not say whether the page behind can receive the input.

### Nested Roving Focus

A command strip, menu, segmented control, tab bar, and radio group can all use local roving focus.
The page may use Tab to enter or leave the group, while Left/Right or Up/Down moves within the
group. The selected item may be different from the locally focused item.

The app needs local focus scopes that can be entered, exited, and repaired when children are
disabled or removed.

### Captured Pointer Interaction

A slider, drag handle, scrollbar thumb, or text selection gesture should keep receiving pointer
updates after the pointer leaves its original rectangle. A release should route to the captured
target, not just the target currently under the pointer.

The current `PointerState` tracks pressed id enough for simple press/release activation. Drag and
capture need stronger route ownership.

## Background Reading

This use case was informed by broader Rust GUI architecture writing, but those projects are
brainstorm material rather than direct requirements. They solve different problems with different
constraints, often using retained widget trees, pass systems, accessibility trees, and platform
rendering concerns that are outside this crate's current scope.

The useful ideas to keep in mind are narrower:

1. Events often need a target path, not just a target id. A route from root to leaf gives each local
   layer a chance to adapt state or policy.
1. Event systems often separate pointer, keyboard/text, accessibility, layout, paint, and update
   work. Even when Ratatui stays immediate-mode, it is useful to name which previous-frame data an
   input event is allowed to use.
1. Full GUI toolkits need strong answers for text input, keyboard layouts, IME, accessibility,
   pointer capture, and focus updates. This crate should leave room for those concerns without
   pretending to solve them.

`ratatui-layout` does not have a retained widget tree, and it should not copy those architectures
directly. The useful lesson is narrower: once an app has nested surfaces, flat ids are not enough to
explain input ownership. The app needs an explicit route or scope model if the crate wants to help
with nested focus and event propagation.

## Current Crate Coverage

The crate already covers the frame-local data needed to build routes:

- `Regions` records visible regions and z-order.
- `PointerTargets` records pointer targets, disabled state, local coordinates, and z-order.
- `FocusTargets` records enabled focus targets in traversal order.
- `FrameSnapshot` can map, translate, clip, and merge child data across component boundaries.
- `FrameTargets` can derive aligned layout, mouse, and focus target data from one region list.

The crate also has examples that approach this from different sides:

- `layout-routing-lab` shows app-owned route paths, local focus scopes, pointer capture, and route
  diagnostics in one nested app.
- `component_frames` shows child frame snapshots being mapped into app-level ids and merged.
- `modal_prompt` shows a raised surface with backdrop routing and local button focus.
- `profile_editor` shows field focus, a command row, and validation-disabled Save.
- `pointer_targets` shows z-order, disabled pointer targets, hover, press, and release.
- `toolbar_targets` shows action-surface ids shared across layout, focus, and pointer data.

Those examples prove the low-level data are useful, but they still encode route relationships by
hand in application enums and event handlers.

## Coordination Data Analysis

Nested routing usually combines several value types, but the central missing value is relationship.

| Shape                | Layout | Mouse | Focus | Selection | Cursor | Route |
| -------------------- | ------ | ----- | ----- | --------- | ------ | ----- |
| Form inside pane     | yes    | maybe | yes   | maybe     | yes    | yes   |
| Inline row controls  | yes    | yes   | maybe | yes       | no     | yes   |
| Modal over page      | yes    | yes   | yes   | no        | maybe  | yes   |
| Roving focus group   | yes    | maybe | yes   | maybe     | no     | yes   |
| Drag/capture target  | yes    | yes   | no    | maybe     | no     | yes   |

`Route` here means parent/child relationship and propagation order. It is different from
`PointerTargets::hit_test`, which returns the topmost leaf target, and different from
`FocusTargets`, which returns the next enabled focus target in a flat traversal order.

## Possible API Direction

Do not add this yet. The shape needs at least one strong example first.

The likely primitives are small and explicit:

- `TargetPath<Id>` or `RoutePath<Id>`: a root-to-leaf path such as page, pane, form, field.
- `RouteMap<Id>`: frame-local parent/child relationships plus optional areas for routing.
- `FocusScope<Id>`: a named group with ordered children, entry target, fallback target, and trap
  policy.
- `EventRoute<Id>`: the result of routing one event, including leaf target and ancestor chain.
- `PointerCapture<Id>`: persistent state saying pointer updates belong to the pressed/captured id.

The most Ratatui-compatible version would keep these as optional data. Widgets would still render
into `Rect` and `Buffer`. Apps would still own domain state and decide what each routed event means.

## Proof Example

The first proof is now `layout-routing-lab`:

```shell
cargo run -p layout-routing-lab
```

It includes:

- a page with two panes;
- a details pane containing a form group;
- a row list where each row has an inline action;
- a modal or popup above both panes;
- keyboard routing that lets fields handle text first, then form-level traversal, then page-level
  shortcuts;
- mouse routing that can report both leaf target and parent row or modal shell;
- one captured pointer interaction, such as dragging a splitter or scrollbar thumb.

The example starts with app-owned route code. Only extract API after the repeated mechanics are
visible and boring.

## What This Validates

This use case explains a limitation of the current flat values:

- topmost hit testing is not the same as route propagation;
- focus traversal is not the same as focus scoping;
- z-order is not the same as outside-click or capture policy;
- enum ids can encode hierarchy, but the app still has to write the route mechanics;
- `FrameSnapshot` can carry data across component boundaries, but it does not define who handles an
  event first.

The current crate should document this as future pressure. Adding route paths or focus scopes too
early would likely create a framework-shaped API before the example pressure is clear.

## Related Use Cases

- [`0001 Ordered Action Surfaces`](0001-command-toolbar-routing.md) covers roving action surfaces.
- [`0002 Modal Dialog Coordination`](0002-modal-dialog-coordination.md) covers raised surfaces and
  outside-click policy.
- [`0007 Component Frame Composition`](0007-component-frame-composition.md) covers child frame
  mapping and merging.
- [`0008 Mouse Overlays And Disabled Targets`](0008-mouse-overlays-and-disabled-targets.md) covers
  pointer z-order, disabled targets, hover, and press/release.
- [`0010 Frame Snapshots`](0010-frame-snapshots.md) covers choosing which data a surface
  should produce.
