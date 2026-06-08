# 0004 Multi-Pane Scroll Routing

## User Story

An app has several scrollable panes: a project tree, task table, log pane, details pane, or help
overlay. Mouse-wheel input should scroll the pane under the pointer, including blank space. It
should not change the selected item unless the user explicitly moves selection.

This is common in dashboards and inspectors where selection drives details while the user scans
nearby content.

## Concrete App Shapes

### Release Inspector

A release board has a project tree on the left, a task table in the center, and a details log on the
right. Wheel input over each pane should move only that pane's viewport. The selected task remains
the selected task.

### Log Viewer

A split log viewer shows build output and service logs side by side. The pointer may be over blank
space below the last visible line, but wheel input should still route to the pane the user is
looking at.

### Help Overlay

A modal help overlay may contain long text. Wheel input inside the overlay should scroll help text,
not the page behind it. Wheel input outside the overlay might dismiss, be ignored, or route to a
backdrop policy, depending on the app.

### Virtual Table

A large table may scroll vertically and horizontally. Wheel input should adjust viewport offsets.
Selection should move only on keyboard navigation, row clicks, or explicit activation.

## Core Necessity

Multi-pane scroll routing needs four separate decisions:

1. Which pane rectangle was visible in the last frame?
1. Which pane should receive wheel input at the pointer position?
1. How does that pane update its viewport offset?
1. Which selection state, if any, remains unchanged?

The important split is that mouse-wheel routing is pointer routing plus viewport state. Selection is
not part of the normal wheel path.

## Current Crate Path

Use [`FrameSnapshot::scroll_region`](../src/frame.rs) or
[`FrameTargets::mouse_region`](../src/frame.rs) to register whole-pane wheel targets. Whole-region
targets make blank pane space interactive, so wheel input does not depend on hitting a visible row.

```rust
use ratatui_core::layout::Rect;
use ratatui_layout::FrameSnapshot;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Pane {
    BuildLog,
    Details,
}

let screen = Rect::new(0, 0, 80, 24);
let details = Rect::new(40, 1, 39, 22);
let frame = FrameSnapshot::new(screen).scroll_region(Pane::Details, details);

assert_eq!(frame.route_scroll((45, 20)).unwrap().id, Pane::Details);
```

Use [`FrameSnapshot::route_scroll`](../src/frame.rs) on the next event. The input handler should route
the pointer position through the previous frame, then update the pane's viewport offset.

```rust
use ratatui_core::layout::Rect;
use ratatui_layout::FrameSnapshot;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Pane {
    BuildLog,
}

let frame = FrameSnapshot::new(Rect::new(0, 0, 40, 10))
    .scroll_region(Pane::BuildLog, Rect::new(0, 0, 40, 10));
let hit = frame.route_scroll((5, 9)).unwrap();

assert_eq!(hit.id, Pane::BuildLog);
```

Use [`ScrollMetrics`](../src/scroll.rs) for simple fixed-height panes. It clamps the requested
offset, reports scrollbar/status math, and now exposes
[`ScrollMetrics::visible_range`](../src/scroll.rs) for slicing app-owned rows.

```rust
use ratatui_layout::ScrollMetrics;

let rows = ["queued", "running", "blocked", "done"];
let metrics = ScrollMetrics::new(rows.len() as u32, 2, 10);

assert_eq!(metrics.offset, 2);
assert_eq!(&rows[metrics.visible_range()], &["blocked", "done"]);
```

When the pane is virtualized, use the state type that belongs to that virtualized view.
[`VirtualListState::scroll_viewport_by`](../src/list.rs) and
[`VirtualTableState::scroll_viewport_by`](../src/table.rs) move the viewport without changing
selection.

```rust
use ratatui_layout::list::VirtualListState;

let mut state = VirtualListState::default();
state.select(Some(10));
state.scroll_viewport_by(3);

assert_eq!(state.selected(), Some(10));
```

Runnable examples:

- `cargo run -p ratatui-layout --example pointer_only_scroll_region`
- `cargo run -p ratatui-layout --example scroll_regions`
- `cargo run -p ratatui-layout --example visible_selection_virtual_list`
- `cargo run -p ratatui-layout --example table_inspector`
- `cargo run -p ratatui-layout --example viewport`
- `cargo run --manifest-path examples/apps/layout-workspace-inspector/Cargo.toml`

## Coordination Data Analysis

Scroll routing does not require a full frame aggregate for every pane, but it often benefits from
one at page boundaries because layout, mouse regions, and scroll metrics are derived together.

| Shape           | Layout | Mouse | Focus | Selection | Cursor | Aggregate |
| --------------- | ------ | ----- | ----- | --------- | ------ | --------- |
| Split log panes | maybe  | yes   | no    | no        | no     | maybe     |
| Details pane    | yes    | yes   | maybe | no        | no     | maybe     |
| Help overlay    | yes    | yes   | no    | no        | no     | maybe     |
| Virtual list    | yes    | maybe | no    | maybe     | no     | maybe     |
| Virtual table   | yes    | maybe | no    | maybe     | no     | maybe     |

The central frame value is the mouse region for the pane. Layout and metrics help render scroll
status. Focus is useful only when keyboard scrolling should target the focused pane. Selection is
persistent app state and should stay out of wheel routing unless the app intentionally couples the
two.

## What This Validates

The crate now covers the common scroll-routing path:

- whole-pane mouse targets route wheel events over blank space;
- previous-frame routing keeps input aligned with what the user just saw;
- `ScrollMetrics::visible_range` removes repeated slice math from simple line panes;
- virtual list and table states already separate viewport scrolling from selection movement;
- pane ids remain app-owned enums, strings, or other stable ids.

The important design pressure is keeping scrolling explicit without forcing every pane into
`VirtualList` or `VirtualTable`. Simple text panes should remain simple.

## Related Use Cases

- [`0003 Filtered Record Selection`](0003-filtered-record-selection.md) explains why wheel input
  should not silently move durable selection.
- [`0005 Virtual List Row Rendering`](0005-virtual-list-row-rendering.md) covers row virtualization
  when simple fixed-height slicing is not enough.
- [`0006 Virtual Table Inspection`](0006-virtual-table-inspection.md) covers two-axis table
  scrolling and cell selection.
- [`0010 Frame Snapshots`](0010-frame-snapshots.md) explains when scroll routing needs a
  full `FrameSnapshot` and when a smaller pointer target collection is enough.

## Remaining Design Questions

The current model is enough for common panes, but two possible helpers may become useful:

- a small one-axis `ScrollState` for simple fixed-height panes if examples keep storing bare offsets;
- a pane bundle that pairs a scroll region, metrics, and a clamped offset without becoming a
  scrollbar widget.

Both should wait for more examples. The current narrow improvement is `ScrollMetrics::visible_range`
plus examples that keep selection and wheel scrolling separate.
