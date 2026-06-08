# 0007 Component Frame Composition

## User Story

An app is large enough to split into components: navigation pane, work queue, details pane, command
strip, and modal overlays. Each component renders ordinary Ratatui widgets and owns local helper
state, but the parent needs one combined record of layout regions, focus targets, mouse targets, and
cursor requests from the whole screen.

The app does not want a retained widget tree. It wants immediate-mode rendering with explicit data
returned from each render pass and stored for the next input event.

## Concrete App Shapes

### Multi-Pane Inspector

A project inspector has a tree pane, a table or list pane, a details pane, and a command strip. Each
pane has different rendering and input policy, but tab traversal and mouse routing must work across
the whole page.

### Modal Overlay

A dialog renders above the base page. Its targets need higher z-order, its focus targets should
temporarily dominate traversal, and its cursor request should win when an editable field is active.
The base page still exists underneath, but the frame-local data used for input should match what the
user can interact with.

### Nested Viewport

A child component renders a scrollable region in local coordinates. The parent places that child
inside a clipped pane. Only visible child targets should be merged into the parent frame, and local
coordinates need to become terminal coordinates before the next mouse event is routed.

### Focused Text Input

A command input or form field computes a cursor position while rendering. If the input is a child
component, the cursor request may be local to that child and must be translated with the rest of the
child frame-local data.

## Core Necessity

Component frame composition needs a narrow contract between children and parent:

1. A child renders ordinary widgets into the area the parent assigns.
1. The child returns frame-local data for only what it rendered.
1. The child may use local ids that are meaningful only inside that component.
1. The child may compute local geometry before the parent places it on screen.
1. The parent maps child ids into app-level ids.
1. The parent translates and clips child data to the actual screen region.
1. The parent merges child data in render order and stores the result for the next event.

The component boundary should not require a retained widget tree, callback registry, app-wide trait,
or semantic action type before examples prove those shapes.

## Current Crate Path

Return [`FrameSnapshot`](../src/frame.rs) from component render methods when a child produces several
coordination data together. Use smaller values directly when a component only needs one concern.

```rust
use ratatui_core::layout::Rect;
use ratatui_layout::{FrameSnapshot, FrameTargets, Region};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum RowTarget {
    Row(usize),
}

fn render_child(area: Rect) -> FrameSnapshot<RowTarget> {
    let local = Rect::new(0, 0, area.width, area.height);
    let rows = [
        Region::new(RowTarget::Row(0), Rect::new(1, 1, local.width - 2, 1)),
        Region::new(RowTarget::Row(1), Rect::new(1, 2, local.width - 2, 1)),
    ];

    FrameTargets::new(local, 0).build_focusable(rows, |id| id)
}
```

Map child-local ids into an app-level routing enum before merging. This keeps each component's
local id model small while giving the parent one type for focus, mouse, and layout routing.

```rust
use ratatui_core::layout::Rect;
use ratatui_layout::{FrameSnapshot, Regions, Region};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum AppTarget {
    Project(usize),
}

let project_regions = Regions::from_regions(
    Rect::new(0, 0, 10, 1),
    [Region::new(0, Rect::new(0, 0, 10, 1))],
);
let child = FrameSnapshot::from_layout(project_regions).map_id(AppTarget::Project);

assert_eq!(child.route_position((1, 0)).unwrap().id, AppTarget::Project(0));
```

Place local child data after the parent solves the child area. [`FrameSnapshot::place_child`](../src/frame.rs)
combines the common sequence: translate to the area's origin, clip to the area, then merge. It moves
layout, focus, mouse, and cursor requests together.

```rust
use ratatui_core::layout::{Position, Rect};
use ratatui_layout::{CursorRequests, CursorRequest, FrameSnapshot, Regions, Region};

let input_regions = Regions::from_regions(
    Rect::new(0, 0, 8, 1),
    [Region::new("input", Rect::new(0, 0, 8, 1))],
);
let input_cursor = CursorRequests::new().request(CursorRequest::visible(Position::new(2, 0)));
let child = FrameSnapshot::from_layout(input_regions).cursor(input_cursor);

let input_area = Rect::new(4, 2, 8, 1);
let parent = FrameSnapshot::new(Rect::new(0, 0, 20, 4)).place_child(child, input_area);

assert_eq!(parent.route_position((5, 2)).unwrap().id, "input");
assert_eq!(
    parent.cursor.final_cursor(),
    Some(CursorRequest::visible(Position::new(6, 2)))
);
```

Store the merged frame after rendering and route the next event through it.

```rust
use ratatui_core::layout::Rect;
use ratatui_layout::FrameSnapshot;

let previous_frame = FrameSnapshot::<&str>::new(Rect::new(0, 0, 20, 4))
    .scroll_region("details", Rect::new(0, 0, 20, 4));

assert_eq!(previous_frame.route_scroll((10, 3)).unwrap().id, "details");
```

Runnable examples:

- `cargo run -p ratatui-layout --example component_frames`
- `cargo run -p ratatui-layout --example frame_snapshot`
- `cargo run -p layout-workspace-inspector`

## Coordination Data Analysis

Component composition is the strongest case for an aggregate frame value. The parent often needs to
move several value types across the same boundary at once.

| Shape                | Layout | Mouse | Focus | Selection | Cursor | Frame |
| -------------------- | ------ | ----- | ----- | --------- | ------ | ----- |
| Multi-pane inspector | yes    | yes   | yes   | app-owned | maybe  | yes   |
| Modal overlay        | yes    | yes   | yes   | maybe     | maybe  | yes   |
| Nested viewport      | yes    | maybe | maybe | maybe     | maybe  | yes   |
| Focused text input   | yes    | maybe | yes   | no        | yes    | yes   |

Selection remains app state. `FrameSnapshot` tells the app which target was visible and routed; the
app decides whether that target means selecting a row, opening a dialog, scrolling a pane, editing
text, or running a command.

## What This Validates

The crate now covers the common component-frame path:

- children can render ordinary widgets and return `FrameSnapshot` values;
- child-local ids can be mapped into a parent routing enum;
- child-local layout, focus, mouse, and cursor requests can be translated together;
- child data can be clipped before the parent stores them for input;
- child frames can be placed with a solved screen area instead of manually passing offsets;
- parent frames can merge child data in render order;
- a middle-sized example shows the pattern without a component trait.

The important design pressure is keeping the boundary explicit. Returning `FrameSnapshot` is useful
because it collects data. It should not force a component trait, semantic action enum, or retained
tree until several examples prove the same shape.

## Related Use Cases

- [`0001 Command Toolbar Routing`](0001-command-toolbar-routing.md) covers a compact component that
  returns aligned layout, focus, and pointer data.
- [`0002 Modal Dialog Coordination`](0002-modal-dialog-coordination.md) covers overlays and
  negative hit testing.
- [`0004 Multi-Pane Scroll Routing`](0004-multi-pane-scroll-routing.md) covers pane-level mouse
  regions and wheel routing.
- [`0009 Form Editing`](0009-form-editing.md) covers cursor placement and field editing.
- [`0010 Frame Snapshots`](0010-frame-snapshots.md) compares when an aggregate frame is useful
  and when smaller values are enough.

## Remaining Design Questions

The current model is enough for explicit child-to-parent composition. Possible future pressure:

- a `UiComponent` trait once examples converge on the same render signature;
- a semantic outcome or action type once examples repeatedly return commands from rendering;
- modal stack helpers if multiple overlay examples repeat the same z-order and focus policy.

Those should be validated through examples before adding a broader component layer.
