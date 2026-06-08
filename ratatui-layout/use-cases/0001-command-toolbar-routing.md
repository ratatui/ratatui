# 0001 Ordered Action Surfaces

## User Story

An app exposes a small ordered set of actions or choices as visible UI. The surface might look like
a footer command strip, a toolbar, a dialog button row, a menu, a tab bar, a radio group, or a
segmented control. The app still renders ordinary Ratatui widgets, but keyboard focus, mouse input,
disabled state, selection, and activation must all agree on the same app-level ids.

The app can render labels, icons, brackets, tabs, or buttons however it wants. The stable concern is
coordination: a visible ordered action id must carry enough frame-local data that the next input
event can route back to the same app action.

## Concrete App Shapes

### Footer Command Strip

A task board has a footer:

```text
commands  e edit    r run    m mark done    ? help
```

The app needs `MarkDone` to stay visible but disabled when no task is selected. Keyboard shortcuts
can activate commands directly, Tab can move through visible command targets, and mouse clicks can
activate the command under the pointer.

The app-specific command id is probably an enum:

```rust
enum Command {
    Edit,
    Run,
    MarkDone,
    Help,
}
```

The command strip needs an ordered layout, disabled-state policy, focus targets, mouse targets, and
activation routing. Labels, styles, shortcuts, and command effects stay app-owned.

### Toolbar

A database console has a top toolbar:

```text
[ connect ] [ refresh ] [ explain ] [ export ]
```

Some commands depend on connection state or query selection. The toolbar may be rendered with
bordered buttons, but the same coordination problem applies:

- disabled commands remain visible;
- focus traversal skips disabled commands;
- hover styling uses the command id;
- click routing uses previous-frame regions;
- command execution stays in the app.

A toolbar may reserve larger fixed widths, use icons, or live in a page header. Those rendering
choices stay outside the primitive model.

### Dialog Button Row

A modal edit dialog has a button row:

```text
[ cancel ] [ save ]
```

The row is still an ordered action surface, but the local navigation policy may differ:

- Left and Right move between buttons.
- Enter activates the focused button.
- Escape activates `Cancel`.
- `Save` may be disabled until the form is valid.

This case shows why "global focus movement" and "local action-row movement" are separate concerns.
The app might use a page-level `FocusState` to focus the button row, then a `ButtonRow` or similar
local state to focus one button inside the row.

### Menu

A file picker or editor has a menu:

```text
file
  new
  open
  save
  save as
```

The same ordered action concept can render top to bottom. A menu may also have nested submenus,
separators, accelerators, and disabled items. Those are rendering and menu policy details. The core
frame-local data are familiar:

- visible item ids in traversal order;
- rectangles for hit testing;
- disabled items that remain visible;
- focus or hover movement through enabled items;
- activation of the focused or clicked item.

Menus make orientation a layout policy. The same action-surface model needs to cover horizontal
toolbars, vertical menus, and eventually grid-like command palettes.

### Tab Bar

A log viewer has tabs:

```text
all | errors | warnings | deploys
```

Tabs are ordered action ids, but activation changes a persistent selected tab. Disabled state is
less common, while selection is central. Arrow keys may move focus without activating, or they may
activate immediately depending on the app's policy.

The important distinction:

- focus is "where keyboard input is currently aimed";
- selection is "which tab controls the visible content";
- activation is "the user committed a tab change."

The crate needs to coordinate these states without collapsing them into one overloaded value.

### Segmented Control

A release board has a compact mode selector:

```text
queued | running | blocked | done
```

This is tab-like, but it is usually embedded in another pane and may sit outside the main page Tab
order. It still wants:

- ordered ids;
- selected id;
- mouse hit testing;
- optional focus traversal;
- app-owned rendering.

This case pressures the design to avoid assuming every ordered action surface is globally focusable.

### Radio Group

A settings dialog has a radio-style choice:

```text
( ) compact
(*) comfortable
( ) spacious
```

Radio groups share the selected-value shape with tab bars and segmented controls. The selected
value has a different meaning: it is form state. Activating another option changes the pending form
value. Focus may move between options without changing the selected value, or the app may choose
immediate selection on arrow movement.

This case reinforces the state split:

- focus is the option that receives keyboard interaction;
- selection is the app-owned value;
- activation is the user committing a choice;
- disabled options can remain visible and unselectable.

The crate needs to expose enough data for either radio policy without deciding the policy itself.

## Core Necessity

All of these shapes need the same irreducible frame-local model:

1. The app owns an ordered list of action ids.
1. The render pass assigns each visible id a rectangle.
1. The render pass records whether each visible id is enabled.
1. The render pass may record whether each id is focusable or mouseable.
1. Rendering uses those same ids to choose visual state.
1. The next keyboard or mouse event routes through data from the previous frame.
1. The app owns activation effects.

The core crate concern is therefore:

> Given a visible ordered set of app action ids, produce consistent geometry, focus targets, mouse
> targets, disabled behavior, selection hooks, and activation routing for the next event.

## Primitive Boundary

These concerns belong to the app or to a higher-level widget crate:

- labels, icons, shortcut text, and styling;
- whether the surface renders as tabs, footer text, bordered buttons, or compact segments;
- whether the surface is horizontal, vertical, or grid-like;
- command execution side effects;
- crossterm or backend-specific event types;
- validation rules that decide whether a command is enabled;
- retained children or a retained toolbar widget.

## Current Crate Path

The current crate has no `ActionSurface` widget. It solves this with smaller frame-local
primitives:

- `Row::named`, `Column::named`, and `Grid::layout` assign rectangles to app-owned ids.
- `Regions` and `Region` preserve visible geometry, render order, clipping, and z-order.
- `FrameTargets::from_regions` derives layout, focus, and pointer data from the same solved regions.
- `RegionTargets::disabled`, `RegionTargets::focusable`, and
  `RegionTargets::mouseable` keep visible, enabled, focusable, and mouseable policy separate.
- `FocusState::ensure_visible` repairs stale focus against the current frame with an explicit
  `FocusFallback`.
- `SelectionState` keeps selected values separate from focus, hover, and activation.
- `ButtonRow` handles local left/right focus inside dialogs.

### Command Strip With Current Primitives

Start by naming layout regions with app command ids:

```rust
let commands = [
    (Command::Edit, Constraint::Length(8)),
    (Command::Run, Constraint::Length(6)),
    (Command::MarkDone, Constraint::Length(12)),
    (Command::Help, Constraint::Length(6)),
];
let command_regions = Row::named(commands)
    .spacing(1)
    .regions(area);
```

The same regions produce frame-local routing data:

```rust
let frame = FrameTargets::from_regions(command_regions, COMMAND_FOCUS)
    .disabled(|command| !app.is_enabled(command))
    .build();

app.focus.ensure_visible(&frame.focus, FocusFallback::First);
app.previous_frame = frame;
```

Disabled commands remain renderable in `frame.layout`, while click, hover, and focus traversal skip
disabled targets.

### Rendering State From The Same Ids

Rendering still belongs to the app. The crate gives stable ids and areas; the app chooses labels,
shortcut text, and styles:

```rust
for region in frame.layout.regions() {
    let focused = app.focus.focused() == Some(region.id);
    let selected = app.selection.is_selected(region.id);
    let enabled = app.is_enabled(region.id);
    render_command(frame, region.id, region.area, focused, selected, enabled);
}
```

The same ids can render footer text, toolbar buttons, tabs, or radio rows.

### Pointer And Focus Policy

Some surfaces are mouseable but not page-focusable:

```rust
let frame = FrameTargets::from_regions(command_regions, STATUS_FILTER_FOCUS)
    .focusable(|_| false)
    .build();
```

Some surfaces are focusable but only partly mouseable:

```rust
let frame = FrameTargets::from_regions(command_regions, FORM_FOCUS)
    .mouseable(|id| matches!(id, FieldTarget::EditButton(_)))
    .build();
```

`route_position` remains available for broad geometry queries. Click, hover, and wheel routing use
explicit mouse targets when a pointer target collection exists.

### Orientation As Layout Policy

Orientation comes from layout:

```rust
let toolbar = Row::named(toolbar_commands)
    .spacing(1)
    .regions(toolbar_area);
let menu = Column::named(menu_items).regions(menu_area);
```

Both values can feed `FrameTargets::from_regions`; geometry and key policy provide the difference.

### Selection And Activation

Tabs, segmented controls, and radio groups need selected values. `SelectionState` keeps selection
separate from focus:

```rust
if let Some(hit) = app.previous_frame.route_click(position) {
    app.selection.select(hit.id);
    app.activate(hit.id);
}
```

A tab bar may select on arrow movement; a radio group may wait until Enter. That remains app
policy.

### Dialog Button Rows

`ButtonRow` remains useful for dialog buttons. Page-level `FocusState` can focus the row as a
whole while `ButtonRowState` chooses the button inside it.

## Remaining Design Questions

The remaining gaps are above region-to-frame construction:

1. A reusable action description could pair id, width, enabled state, shortcut text, and label
   without turning into a retained widget.
1. A future `ActionSurface` could wrap `Row`, `Column`, or `Grid` for command strips, menus, tabs,
   segmented controls, and radio groups while still returning ordinary `FrameSnapshot` data.
1. Local roving focus helpers could expose ordered enabled ids for menus, tab bars, and radio
   groups without forcing every surface into page-level `FocusTargets`.
1. Selection adapters could make "focus moves only" and "focus movement selects immediately"
   policies easier to express at the call site.
1. A higher-level widget crate could provide default rendering for buttons, tabs, menu items, and
   radio rows while this crate stays focused on geometry and frame-local coordination.

## Validation Checklist

Any proposed primitive for this use case needs to support:

- enum ids for app commands;
- stable render order;
- fixed widths and fill widths;
- hidden actions removed from layout;
- disabled actions visible but skipped by focus and mouse activation;
- focus movement that can be global or local depending on the app;
- selection distinct from focus;
- mouse hover and click routing through previous-frame-local data;
- a non-focusable segmented control;
- a vertical menu using the same routing model as a horizontal toolbar;
- a radio group where selected value is form state;
- a dialog button row with Cancel and Save;
- app-owned labels, styling, shortcuts, and command effects.

If an API cannot explain at least the footer strip, toolbar, button row, menu, tab bar, radio
group, and segmented control without special cases, it is probably too narrow.
