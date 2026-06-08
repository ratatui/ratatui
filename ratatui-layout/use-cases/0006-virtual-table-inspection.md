# 0006 Virtual Table Inspection

## User Story

An app displays a table that can be larger than the terminal in both directions. The app owns the
records and cell rendering, but it needs frame-local data for pinned headers, selected cells, mouse
hit testing, visible-cell diagnostics, and row or column scroll status.

Examples include database browsers, metric grids, trace inspectors, test matrices, release boards,
and spreadsheet-like admin tools.

## Concrete App Shapes

### Database Browser

A database browser shows many rows and enough columns that only part of the schema fits on screen.
The app wants pinned column headers, keyboard movement by cell, a details pane for the selected
record, and status text that reports row and column offsets.

### Release Board

A release board is row-oriented: each row is a task or change request, and each column is a field.
Horizontal scrolling may be disabled by app policy even though the table primitive can support it.
Commands usually operate on the selected row, while clicks still need to identify the exact cell.

### Test Matrix

A test matrix is genuinely two-dimensional. Rows are platforms, columns are checks, and the selected
cell is the meaningful unit. Pinned headers help the user keep context while row and column
scrolling move independently.

### Trace Inspector

A trace inspector may show spans, timing, status, owner, and metadata columns. It can render only
visible cells from app-owned records, route header clicks to sort commands, and route body clicks to
selection or details.

## Core Necessity

Virtual table rendering needs a narrow contract between the app and the table:

1. The app owns records, field definitions, and any durable ids.
1. The table asks the app how many body rows and columns exist.
1. The table computes pinned header cells, visible body cells, row scroll, and column scroll.
1. The table calls the app renderer only for visible cells.
1. The returned layout remains available for hit testing, scroll metrics, and diagnostics.
1. Keyboard selection and viewport scrolling stay separate.
1. Header hits and body-cell selection stay distinguishable.

The table should not own row records, schema metadata, sort behavior, durable row ids, or command
semantics.

## Current Crate Path

Implement [`TableItems`](../src/table.rs) for an app-owned adapter. The adapter supplies row count,
column count, and cell rendering. Rendering receives [`TableCellContext`](../src/table.rs), which
tells the renderer whether the cell is selected and where it sits among visible rows and columns.

```rust
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::text::Line;
use ratatui_core::widgets::Widget;
use ratatui_layout::table::{CellPosition, TableCellContext, TableItems};

struct Rows(&'static [[&'static str; 2]]);

impl TableItems for Rows {
    fn row_count(&self) -> usize {
        self.0.len()
    }

    fn column_count(&self) -> usize {
        2
    }

    fn render_cell(
        &mut self,
        position: CellPosition,
        area: Rect,
        buf: &mut Buffer,
        _: TableCellContext,
    ) {
        let text = position
            .row
            .map(|row| self.0[row][position.column])
            .unwrap_or("header");
        Line::from(text).render(area, buf);
    }
}
```

Use [`VirtualTableState`](../src/table.rs) as persistent table state. It stores body-cell selection
and row/column scroll offsets. [`VirtualTableState::select_relative`](../src/table.rs) handles
common keyboard movement without repeated row and column clamping in the app.

```rust
use ratatui_layout::table::{CellPosition, VirtualTableState};

let mut state = VirtualTableState::default();

assert_eq!(state.select_relative(1, 0, 3, 2), Some(CellPosition::body(0, 0)));
assert_eq!(state.select_relative(1, 1, 3, 2), Some(CellPosition::body(1, 1)));
assert_eq!(state.select_relative(99, 99, 3, 2), Some(CellPosition::body(2, 1)));
```

Use [`VirtualTableState::scroll_rows_by`](../src/table.rs),
[`VirtualTableState::scroll_columns_by`](../src/table.rs), or
[`VirtualTableState::scroll_viewport_by`](../src/table.rs) for wheel and page scrolling that should
not move selection. Apps that do not want horizontal scroll can simply avoid routing input to the
column helpers.

Use [`VirtualTable::render`](../src/table.rs) for the normal path. It computes layout, renders only
visible cells, mutates state to clamp scroll/selection, and returns [`TableLayout`](../src/table.rs)
so the next input event can route through the cells the user just saw.

```rust
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Constraint, Rect};
use ratatui_layout::table::{
    CellPosition, TableCellContext, TableItems, VirtualTable, VirtualTableState,
};

struct Rows;

impl TableItems for Rows {
    fn row_count(&self) -> usize { 2 }
    fn column_count(&self) -> usize { 2 }
    fn render_cell(&mut self, _: CellPosition, _: Rect, _: &mut Buffer, _: TableCellContext) {}
}

let mut rows = Rows;
let mut state = VirtualTableState::default();
let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 2));
let layout = VirtualTable::new([Constraint::Length(4), Constraint::Length(4)])
    .render(buffer.area, &mut buffer, &mut state, &mut rows);

assert_eq!(layout.hit_position((5, 1)), Some(CellPosition::body(0, 1)));
```

Use [`TableLayout::select_hit`](../src/table.rs) when a click should select a body cell. Use
[`TableLayout::hit_position`](../src/table.rs) when the app also needs header hits for sorting or
column actions. Use [`TableLayout::hit_test`](../src/table.rs) when local pointer coordinates
matter.

Use [`TableLayout::cell_regions`](../src/table.rs) or [`TableLayout::cells_regions`](../src/table.rs)
when visible cells need to join generic layout, pointer, or frame coordination code. `TableLayout`
keeps the full table data; the cell `Regions` value is the geometry projection.

Runnable examples:

- `cargo run -p ratatui-layout --example table_inspector`
- `cargo run -p layout-workspace-inspector`

## Coordination Data Analysis

Virtual tables produce richer data than a generic region set. A visible cell has a source
`CellPosition`, area, visible row, visible column, selected state, and header/body meaning. That is
why `TableLayout` exists beside `Regions`.

| Shape            | Layout | Mouse | Focus | Selection | Cursor | Frame |
| ---------------- | ------ | ----- | ----- | --------- | ------ | ----- |
| Database browser | yes    | maybe | no    | yes       | no     | maybe |
| Release board    | yes    | maybe | maybe | yes       | no     | maybe |
| Test matrix      | yes    | maybe | no    | yes       | no     | maybe |
| Trace inspector  | yes    | maybe | maybe | yes       | no     | maybe |

The central frame value is `TableLayout`. Mouse data are needed only when the app wants clicks or
wheel routing over the table region. Focus is usually pane-level unless the app treats individual
cells as focus targets. Selection can be a source [`CellPosition`](../src/table.rs), a durable row id
owned by the app, or a pair such as `(RecordId, FieldId)`.

## What This Validates

The crate now covers the common virtual-table path:

- app-owned cell adapters render cells without retained cell widgets;
- pinned headers and body cells share one typed coordinate model;
- only visible cells render;
- keyboard selection can move through row and column indexes with `select_relative`;
- viewport scrolling can move rows or columns without changing selection;
- body-cell clicks can update table selection with `select_hit`;
- header hits remain available for sort or column commands;
- visible cells can be projected into `Regions` when a frame needs geometry interop;
- scroll metrics can drive status text or app-owned scrollbars.

The important design pressure is preserving app policy. `VirtualTable` should make row, column, and
viewport math reliable, but it should not decide whether a release board supports horizontal scroll
or whether commands operate on a row, a cell, or a stable record id.

## Related Use Cases

- [`0003 Filtered Record Selection`](0003-filtered-record-selection.md) covers durable record ids
  when source positions are not enough.
- [`0004 Multi-Pane Scroll Routing`](0004-multi-pane-scroll-routing.md) covers wheel routing to pane
  regions, including blank space.
- [`0005 Virtual List Row Rendering`](0005-virtual-list-row-rendering.md) covers the one-axis
  version of this problem.
- [`0010 Frame Snapshots`](0010-frame-snapshots.md) explains why virtual tables often return a
  derived layout value instead of forcing everything through `FrameSnapshot`.

## Remaining Design Questions

The current model is enough for fixed-height virtual tables with pinned headers. Possible future
pressure:

- variable-height table rows;
- pinned leading columns;
- durable-id helpers that pair `CellPosition` with `(RecordId, FieldId)`;
- table range selection for spreadsheet-like workflows;
- a pane-level helper that pairs a table layout with whole-table scroll regions.

Those should be validated through examples before adding broader table abstractions.
