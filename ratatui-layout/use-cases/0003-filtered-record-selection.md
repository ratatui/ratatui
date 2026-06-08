# 0003 Filtered Record Selection

## User Story

An app displays domain records: tasks, log streams, database rows, releases, issues, or files. The
user can filter, sort, or scroll the visible list. Commands should operate on the selected record,
not on whichever visible row happens to occupy the same index after filtering.

This is the point where a widget-local selected index stops being enough. The app needs a stable
record id for commands and details panes, while rendering still needs the selected visible position.

## Concrete App Shapes

### Filtered Task List

A task board toggles between all tasks and blocked tasks. If `DOC-004` is selected and the blocked
filter hides it, the app should choose an explicit fallback, usually the first visible blocked task.
If no tasks are visible, selection should clear.

### Sorted Database Rows

A database browser sorts rows by different columns. The selected row should stay attached to the
same primary key when its visible position changes.

### Search Results

A search pane updates visible results on every keystroke. The selected result should stay selected
while it remains visible, fall back predictably when it disappears, and never accidentally point at
a different record just because that record moved into the same index.

### Virtualized Record View

A large list or table may render only a visible window. The app still needs commands to operate on a
durable id, while row highlighting, keyboard movement, and hit testing use visible positions.

## Core Necessity

Filtered record selection needs two identities for one selected thing:

1. The app owns a durable selected id, such as a task id, file path, database primary key, or record
   handle.
1. The current frame owns a visible position, such as a row index or cell coordinate.
1. Filtering, sorting, and virtualization can change the visible position without changing the
   durable id.
1. If the durable id is no longer visible, the app needs an explicit fallback.
1. If no visible ids exist, selection should clear.
1. Mouse wheel scrolling should move the viewport, not the selected record.

This use case is mostly persistent selection state plus a visible-id bridge. It should not require
a full `FrameSnapshot`.

## Current Crate Path

Use [`VisibleSelection`](../src/selection.rs) when selection must keep both a durable id and a
visible index. The common case is a visible slice of durable ids:

```rust
use ratatui_layout::VisibleSelection;

#[derive(Clone, Copy, Eq, PartialEq)]
struct TaskId(u16);

let mut selection = VisibleSelection::new();
let visible_ids = [TaskId(1), TaskId(2), TaskId(3)];

selection.sync_ids(&visible_ids);
assert_eq!(selection.position(), Some(0));
assert_eq!(selection.selected_id(), Some(TaskId(1)));
```

Use [`VisibleSelection::sync_ids`](../src/selection.rs) after filtering or sorting changes the
visible ids. It preserves the current selected id when possible and falls back to the first visible
id when the old selection is gone.

```rust
use ratatui_layout::VisibleSelection;

let mut selection = VisibleSelection::new();
selection.select_visible(1, "docs");

let filtered_ids = ["api"];
selection.sync_ids(&filtered_ids);

assert_eq!(selection.position(), Some(0));
assert_eq!(selection.selected_id(), Some("api"));
```

Use [`VisibleSelection::select_index`](../src/selection.rs) after a click or hit test has resolved
to a visible row index.

```rust
use ratatui_layout::VisibleSelection;

let visible_ids = ["api", "docs"];
let mut selection = VisibleSelection::new();

selection.select_index(1, &visible_ids);

assert_eq!(selection.selected_id(), Some("docs"));
```

Use [`VisibleSelection::move_by`](../src/selection.rs) for keyboard movement through visible ids.
When nothing visible is selected yet, the first movement selects the first visible id. Later
movement applies the requested offset, clamps at the first and last visible id, and clears selection
when no ids are visible.

```rust
use ratatui_layout::VisibleSelection;

let visible_ids = ["api", "docs", "ops"];
let mut selection = VisibleSelection::new();

selection.move_by(1, &visible_ids);
assert_eq!(selection.selected_id(), Some("api"));

selection.move_by(1, &visible_ids);
assert_eq!(selection.selected_id(), Some("docs"));

selection.move_by(99, &visible_ids);
assert_eq!(selection.selected_id(), Some("ops"));
```

Keep [`VisibleSelection::sync`](../src/selection.rs) and
[`VisibleSelection::select_position`](../src/selection.rs) for custom projections. They are still
the right tool when the visible position is not a simple `usize` index into an id slice.

Runnable examples:

- `cargo run -p ratatui-layout --example visible_selection_filter`
- `cargo run -p ratatui-layout --example visible_selection_virtual_list`
- `cargo run -p ratatui-layout --example selection_modes`
- `cargo run -p ratatui-layout --example virtual_list`

## Coordination Data Analysis

Filtered record selection does not usually need an aggregate frame snapshot.

| Shape                   | Layout | Mouse | Focus | Selection | Cursor | Aggregate |
| ----------------------- | ------ | ----- | ----- | --------- | ------ | --------- |
| Filtered task list      | maybe  | maybe | no    | yes       | no     | no        |
| Sorted database rows    | maybe  | maybe | no    | yes       | no     | no        |
| Search results          | maybe  | maybe | maybe | yes       | no     | maybe     |
| Virtualized record view | yes    | maybe | no    | yes       | no     | maybe     |

The central state is selection, not layout. Layout, mouse, focus, or virtual list data may help the
view render or route input, but the selected record id should not be hidden inside that data.

## What This Validates

The crate now covers the common visible-id path directly:

- durable selected ids remain separate from visible indexes;
- visible-id slices can repair stale selection after filtering;
- keyboard movement can operate over visible ids without manually clamping indexes;
- click selection can select by visible index while storing the durable id;
- the closure-based APIs remain available for custom projections.

The important design pressure is keeping selection app-owned. The crate can help bridge visible
positions to durable ids, but it should not decide filtering, sorting, fallback policy beyond the
simple first-visible default, or command behavior.

## Related Use Cases

- [`0010 Frame Snapshots`](0010-frame-snapshots.md) explains why this use case mostly needs
  persistent state rather than a full `FrameSnapshot`.
- [`0005 Virtual List Row Rendering`](0005-virtual-list-row-rendering.md) covers virtualized row
  layout when visible rows are expensive or variable height.
- [`0006 Virtual Table Inspection`](0006-virtual-table-inspection.md) covers two-axis visible
  positions and selected cells.
- [`0004 Multi-Pane Scroll Routing`](0004-multi-pane-scroll-routing.md) covers wheel routing that
  should not move selection.

## Remaining Design Questions

The current helpers are intentionally narrow. Possible future pressure:

- a configurable fallback other than first visible id, such as nearest previous visible id;
- range selection for shift-click or shift-arrow workflows;
- stable record-id helpers for `VirtualList` and `VirtualTable` examples;
- a small adapter from visible row/cell hit tests into durable record ids.

Those should be validated with more examples before adding a broader record-selection abstraction.
