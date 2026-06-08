# 0005 Virtual List Row Rendering

## User Story

An app displays many rows, or rows whose height depends on terminal width. The app owns the row
data and wants custom row rendering. Rendering every row every frame is wasteful, and ordinary
selected-index widgets do not expose enough visible-row data for hit testing, clipping, and scroll
status.

Examples include file trees, issue lists, search results, log entries, release tasks, and command
pickers.

## Concrete App Shapes

### Variable-Height Issue List

An issue tracker renders rows with titles, labels, and wrapped summaries. Row height depends on the
final pane width. The list needs to measure at that width, render only visible row slices, and keep
the selected issue visible during keyboard movement.

### Tree Browser

A file tree or project tree flattens expanded nodes into visible rows. Expansion state and stable
node ids belong to the app. The virtual list only needs source row indexes, row heights, selection
styling, and hit testing for the rows visible this frame.

### Search Results

A search pane can rebuild visible rows after each query change. The app may keep durable record
selection separately, while `VirtualListState` keeps the visible source index used for row styling
and viewport reveal.

### Long Log Pane

A log pane may use fixed-height rows and no selection at all. It still benefits from viewport
scrolling, visible-row data, and scroll metrics, but should not pay for a retained row widget tree.

## Core Necessity

Virtual list rendering needs a narrow contract between the app and the list:

1. The app owns the row data and any stable row ids.
1. The list asks the app how many source rows exist.
1. The list asks how tall each row is at the final width.
1. The list computes which row slices are visible for the current viewport.
1. The list calls the app renderer only for visible slices.
1. The returned layout remains available for hit testing, scroll status, and diagnostics.
1. Keyboard selection and mouse-wheel scrolling stay separate.

The list should not own row widgets, tree semantics, filtering, sorting, or durable record ids.

## Current Crate Path

Implement [`ListItems`](../src/list.rs) for an app-owned row adapter. The adapter supplies row count,
row measurement, and row rendering. Rendering receives [`ListItemContext`](../src/list.rs), which
tells the row whether it is selected and whether the visible slice is clipped.

```rust
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::text::Line;
use ratatui_core::widgets::Widget;
use ratatui_layout::list::{ListItemContext, ListItems};
use ratatui_layout::MeasureContext;

struct Rows(&'static [&'static str]);

impl ListItems for Rows {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn height_for_width(&self, index: usize, width: u16, _: MeasureContext) -> u16 {
        self.0[index].len().div_ceil(width.max(1) as usize).max(1) as u16
    }

    fn render_item(&mut self, index: usize, area: Rect, buf: &mut Buffer, _: ListItemContext) {
        Line::from(self.0[index]).render(area, buf);
    }
}
```

Use [`VirtualListState`](../src/list.rs) as persistent list state. It stores source-index selection
and line-aware scroll position. [`VirtualListState::select_relative`](../src/list.rs) handles common
keyboard movement without repeated index clamping in the app.

```rust
use ratatui_layout::list::VirtualListState;

let mut state = VirtualListState::default();

assert_eq!(state.select_relative(1, 4), Some(0));
assert_eq!(state.select_relative(1, 4), Some(1));
assert_eq!(state.select_relative(99, 4), Some(3));
```

Use [`VirtualListState::scroll_viewport_by`](../src/list.rs) for mouse-wheel or page scrolling that
should not move selection.

```rust
use ratatui_layout::list::{ScrollPosition, VirtualListState};

let mut state = VirtualListState::default();
state.select(Some(5));
state.set_scroll(ScrollPosition::new(1, 0));
state.scroll_viewport_by(2);

assert_eq!(state.selected(), Some(5));
assert_eq!(state.scroll(), ScrollPosition::new(3, 0));
```

Use [`VirtualList::render`](../src/list.rs) for the normal path. It computes layout, renders only
visible rows, mutates state to clamp scroll/selection, and returns [`ListLayout`](../src/list.rs) so
the next input event can hit test against the rows the user just saw.

```rust
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_layout::list::{ListItemContext, ListItems, VirtualList, VirtualListState};
use ratatui_layout::MeasureContext;

struct Rows;

impl ListItems for Rows {
    fn len(&self) -> usize { 3 }
    fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 { 1 }
    fn render_item(&mut self, _: usize, _: Rect, _: &mut Buffer, _: ListItemContext) {}
}

let mut rows = Rows;
let mut state = VirtualListState::default();
let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 2));
let layout = VirtualList::new().render(buffer.area, &mut buffer, &mut state, &mut rows);

assert_eq!(layout.visible_indices().collect::<Vec<_>>(), vec![0, 1]);
```

Use [`ListLayout::hit_index`](../src/list.rs) when a click only needs the row index. Use
[`ListLayout::hit_test`](../src/list.rs) when the row also needs local coordinates, such as a tree
row that treats the disclosure marker differently from the label. Filtered or sorted lists can use
[`ListLayout::select_hit`](../src/list.rs) to move from a click to a durable id stored in
[`VisibleSelection`](../src/selection.rs).

```rust
use ratatui_core::layout::Rect;
use ratatui_layout::VisibleSelection;
use ratatui_layout::list::{ListLayout, ScrollPosition, VisibleItem};

let ids = ["api", "docs"];
let layout = ListLayout {
    viewport: Rect::new(0, 0, 10, 2),
    item_count: ids.len(),
    content_height: 2,
    scroll_offset: 0,
    scroll: ScrollPosition::default(),
    visible_items: vec![
        VisibleItem {
            index: 0,
            area: Rect::new(0, 0, 10, 1),
            full_height: 1,
            y_offset: 0,
            clipped_top: false,
            clipped_bottom: false,
        },
        VisibleItem {
            index: 1,
            area: Rect::new(0, 1, 10, 1),
            full_height: 1,
            y_offset: 0,
            clipped_top: false,
            clipped_bottom: false,
        },
    ],
    selected: None,
};
let mut selection = VisibleSelection::new();

layout.select_hit((2, 1), &mut selection, &ids);
assert_eq!(selection.selected_id(), Some("docs"));
```

Use [`ListLayout::row_regions`](../src/list.rs) or
[`ListLayout::rows_regions`](../src/list.rs) when the visible rows need to join generic layout,
pointer, or frame coordination code. `ListLayout` keeps the full virtual-list data; the row
`Regions` value is the geometry projection.

Use [`ListHeightCache`](../src/list.rs) when measurement is expensive and row heights can be reused
across frames. The cache remains explicit app state so invalidation stays visible when rows or width
change.

Runnable examples:

- `cargo run -p ratatui-layout --example virtual_list`
- `cargo run -p ratatui-layout --example visible_selection_virtual_list`
- `cargo run -p ratatui-layout --example tree_browser`

## Coordination Data Analysis

Virtual lists produce richer data than a generic region set. A row has a source index, visible
area, measured full height, clipped-line offset, and selected state. That is why `ListLayout` exists
beside `Regions`.

| Shape                | Layout | Mouse | Focus | Selection | Cursor | Aggregate |
| -------------------- | ------ | ----- | ----- | --------- | ------ | --------- |
| Variable-height list | yes    | maybe | no    | maybe     | no     | maybe     |
| Tree browser         | yes    | maybe | no    | yes       | no     | maybe     |
| Search results       | yes    | maybe | maybe | yes       | no     | maybe     |
| Log pane             | yes    | maybe | no    | no        | no     | maybe     |

The central frame value is `ListLayout`. Mouse data are needed only when the app wants clicks or
wheel routing over the pane. Focus is usually outside the list unless the app treats a pane as a
keyboard-focus target. Selection can be a source index in `VirtualListState`, a durable id in
`VisibleSelection`, or both.

## What This Validates

The crate now covers the common virtual-list path:

- app-owned row adapters measure and render rows without retained row widgets;
- row measurement happens after final width is known;
- only visible row slices render;
- layout exposes visible row metadata and hit testing;
- keyboard selection can move through source indexes with `select_relative`;
- viewport scrolling can move without changing selection;
- visible rows can be projected into `Regions` when a frame needs geometry interop;
- click handling can use row indexes directly or bridge to durable ids;
- durable record ids remain outside `VirtualListState` when the app needs them.

The important design pressure is preserving ownership boundaries. `VirtualList` should make the
viewport math reliable, but it should not become a list widget framework.

## Related Use Cases

- [`0003 Filtered Record Selection`](0003-filtered-record-selection.md) covers durable record ids
  when source indexes are not enough.
- [`0004 Multi-Pane Scroll Routing`](0004-multi-pane-scroll-routing.md) covers wheel routing to
  pane regions, including blank space.
- [`0006 Virtual Table Inspection`](0006-virtual-table-inspection.md) covers the two-axis version
  of this problem.
- [`0010 Frame Snapshots`](0010-frame-snapshots.md) explains why virtual lists often return a
  derived layout value instead of forcing everything through `FrameSnapshot`.

## Remaining Design Questions

The current model is enough for variable-height row rendering. Possible future pressure:

- range selection for shift-click or shift-arrow list workflows;
- a durable-id adapter that pairs `ListLayout` hit tests with `VisibleSelection`;
- stronger cache invalidation helpers if examples keep clearing `ListHeightCache` manually;
- a pane-level helper that pairs a virtual list layout with a whole-pane scroll region.

Those should be validated through examples before adding broader list abstractions.
