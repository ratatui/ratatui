# ratatui-layout Use Cases

These notes ground `ratatui-layout` in application needs rather than implementation inventory.
Each use case describes a real TUI shape, the coordination problem it creates, what the crate
currently offers, and which API gaps still need sharper names or stronger primitives.

Rustdoc remains the main API guide. These use cases are a design validation catalog for deciding
whether a helper reduces real application complexity or only hides code.

## Catalog

- [0001-command-toolbar-routing](0001-command-toolbar-routing.md): command strips where disabled,
  focused, clicked, and selected controls share the same ids.
- [0002-modal-dialog-coordination](0002-modal-dialog-coordination.md): raised surfaces, z-order,
  local focus, and routing while the page behind remains visible.
- [0003-filtered-record-selection](0003-filtered-record-selection.md): durable selection for
  filtered or sorted application records.
- [0004-multi-pane-scroll-routing](0004-multi-pane-scroll-routing.md): mouse wheel routing to the
  pane under the pointer without changing selection.
- [0005-virtual-list-row-rendering](0005-virtual-list-row-rendering.md): large or variable-height
  lists where the app owns row data and row rendering.
- [0006-virtual-table-inspection](0006-virtual-table-inspection.md): two-axis tables with pinned
  headers, selected cells, hit testing, and scroll metrics.
- [0007-component-frame-composition](0007-component-frame-composition.md): componentized apps that
  return frame-local data without adopting a retained widget tree.
- [0008-mouse-overlays-and-disabled-targets](0008-mouse-overlays-and-disabled-targets.md):
  overlapping pointer targets, disabled controls, hover, press/release, and local coordinates.
- [0009-form-editing](0009-form-editing.md): editable fields, temporary edit buffers, focus
  traversal, cursor placement, validation pressure, and form command rows.
- [0010-frame-snapshots](0010-frame-snapshots.md): choosing which frame-local data, persistent
  state, derived layouts, metrics, and contexts each surface actually needs.
- [0011-nested-event-routing-and-focus-scopes](0011-nested-event-routing-and-focus-scopes.md):
  parent/child route paths, local focus scopes, event order, and pointer capture pressure.

## Review Questions

Use these questions when adding or changing APIs:

1. Does the helper make one of these use cases read more like the user's intent?
1. Does it preserve Ratatui's immediate-mode rendering model?
1. Does the application still own domain data, widget state, and event policy?
1. Does it reduce repeated coordination machinery rather than only shortening example code?
1. Can a focused example show the helper without becoming a framework tutorial?
