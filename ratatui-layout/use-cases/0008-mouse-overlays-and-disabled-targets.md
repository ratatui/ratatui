# 0008 Pointer Overlays And Disabled Targets

## User Story

An app has pointer-sensitive regions that can overlap: a popup over a table, a command palette over
an editor, a tooltip over a row, or a modal dialog over a page. Some controls are visible but
disabled. The app needs hover, press/release matching, local coordinates, and z-order-aware hit
testing without depending on one terminal backend.

This is where simple layout hit testing is not enough. Layout can say which rectangle contains a
position, but pointer routing also needs disabled policy, overlay priority, and state that survives
from press to release.

## Concrete App Shapes

### Popup Over Content

A popup is drawn above an editor or table. Clicks inside the popup should route to popup controls
even when the base content also contains that position. Clicks outside may route to the base content
or close the popup, depending on the app policy.

### Disabled Command

A toolbar or dialog button is visible but disabled because validation failed or a background task is
running. The button still needs disabled styling, but hover and click routing should skip it.

### Hoverable Row Or Cell

A table row, tree item, or grid cell changes style on hover and uses target-local coordinates to
interpret which subregion was clicked.

### Press And Release Activation

A user presses on one target and releases on another. The app should not treat that as activation.
This is common for buttons, menus, palette items, and clickable rows.

## Core Necessity

Pointer overlay routing needs a narrow pointer-specific contract:

1. Rendering produces pointer targets for visible regions.
1. Each target records id, area, disabled state, and z-order.
1. Hit testing chooses the topmost enabled target under a terminal position.
1. The hit carries local coordinates so controls do not recompute their origin.
1. Persistent state stores hover and pressed ids between backend events.
1. Press/release matching only activates when both phases hit the same target.
1. Backend event types stay at the app edge.

Focus and selection may react to a routed pointer hit, but they are not owned by pointer routing.

## Current Crate Path

Use [`PointerTarget`](../src/pointer.rs) when a visible region has pointer-specific behavior such as
disabled state, z-order, or an interactive area that does not exactly match a layout region.

```rust
use ratatui_core::layout::Rect;
use ratatui_layout::{PointerTargets, PointerTarget};

let targets = PointerTargets::from_targets([
    PointerTarget::new("editor", Rect::new(0, 0, 30, 8)),
    PointerTarget::new("palette", Rect::new(4, 2, 12, 3)).z(10),
]);

assert_eq!(targets.hit_test((5, 3)).unwrap().id, "palette");
```

Disabled targets remain visible data but do not win real routing:

```rust
use ratatui_core::layout::Rect;
use ratatui_layout::{PointerTargets, PointerTarget};

let targets = PointerTargets::from_targets([
    PointerTarget::new("fallback", Rect::new(0, 0, 10, 1)),
    PointerTarget::new("disabled-save", Rect::new(0, 0, 10, 1))
        .z(5)
        .disabled(true),
]);

assert_eq!(targets.hit_test((1, 0)).unwrap().id, "fallback");
```

Use [`PointerState`](../src/pointer.rs) with [`PointerPhase`](../src/pointer.rs) after converting a
backend mouse event into a position and phase. The backend-specific part is still only the
conversion at the app edge.

```rust
use ratatui_core::layout::Rect;
use ratatui_layout::{PointerTargets, PointerState, PointerTarget, PointerPhase};

let targets = PointerTargets::from_targets([
    PointerTarget::new("open", Rect::new(0, 0, 6, 1)),
    PointerTarget::new("save", Rect::new(6, 0, 6, 1)),
]);
let mut mouse = PointerState::default();

mouse.route(&targets, (1, 0), PointerPhase::Press);
assert!(mouse.route(&targets, (7, 0), PointerPhase::Release).is_none());

mouse.route(&targets, (7, 0), PointerPhase::Press);
assert_eq!(
    mouse.route(&targets, (7, 0), PointerPhase::Release)
        .unwrap()
        .id,
    "save"
);
```

Runnable examples:

- `cargo run -p ratatui-layout --example pointer_targets`
- `cargo run -p ratatui-layout --example grid_palette`
- `cargo run -p layout-workspace-inspector`

## Coordination Data Analysis

Mouse overlays are mostly a pointer-target problem. Layout data may help derive target rectangles,
but focus, selection, and frame aggregation should remain optional reactions.

| Shape                        | Layout | Mouse | Focus | Selection | Cursor | Frame |
| ---------------------------- | ------ | ----- | ----- | --------- | ------ | ----- |
| Popup over content           | maybe  | yes   | maybe | no        | no     | maybe |
| Disabled command             | maybe  | yes   | maybe | no        | no     | maybe |
| Hoverable row or cell        | maybe  | yes   | no    | maybe     | no     | no    |
| Press/release activation     | no     | yes   | no    | no        | no     | no    |
| Whole-pane wheel routing     | maybe  | yes   | maybe | no        | no     | maybe |

This use case argues for keeping [`PointerTargets`](../src/pointer.rs) ergonomic on its own. An app
should not need a full [`FrameSnapshot`](../src/frame.rs) just to route a hover or button click.

## What This Validates

The crate now covers the common mouse-overlay path:

- targets carry pointer-specific z-order and disabled state;
- hit testing prefers the topmost enabled target and returns local coordinates;
- pointer target collections can be mapped, translated, clipped, and merged for child components;
- persistent state tracks hover and press across events;
- `PointerPhase` lets examples centralize hover, press, and release transitions without importing
  crossterm or another backend into the crate.

The important boundary is that `PointerPhase` is intentionally small. It does not model scroll
direction, buttons, modifiers, drag payloads, double-click policy, or backend event lifetimes.

## Related Use Cases

- [`0001 Command Toolbar Routing`](0001-command-toolbar-routing.md) covers ordered action surfaces
  that often derive mouse targets from regions.
- [`0002 Modal Dialog Coordination`](0002-modal-dialog-coordination.md) covers overlay/backdrop
  behavior and outside-click policy.
- [`0004 Multi-Pane Scroll Routing`](0004-multi-pane-scroll-routing.md) covers whole-pane mouse
  regions for wheel routing.
- [`0007 Component Frame Composition`](0007-component-frame-composition.md) covers mapping and
  placing child-local pointer data inside larger frame snapshots.
- [`0010 Frame Snapshots`](0010-frame-snapshots.md) explains why a mouse-only surface should
  not be forced through a full frame aggregate.

## Remaining Design Questions

The current model is enough for ordinary hover, click, disabled controls, and overlays. Possible
future pressure:

- drag phases if examples need pointer capture and continuous movement after press;
- button/modifier policy if examples need different behavior for left, right, or shifted clicks;
- outside-click helpers if modal and popup examples repeat the same negative-hit behavior;
- scroll phase helpers if wheel routing repeats more than the current pane-region pattern.

Those should be driven by examples rather than by mirroring a specific terminal backend.
