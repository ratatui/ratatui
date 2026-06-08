# 0010 Frame Snapshots

## User Story

An app author is building one visible surface at a time: a command strip, modal dialog, list, table,
scroll pane, picker, or form. Each surface needs some frame-local data from rendering, but rarely
needs every value the crate can produce.

The author needs a way to ask:

- Which data does this surface produce during render?
- Which persistent state does the app keep between frames?
- Which data need to be stored for the next input event?
- When should a component return a full `FrameSnapshot` instead of a smaller value such as
  `Regions` or `PointerTargets`?

This is the broader coordination question behind the previous use cases. The crate should make it
normal to compose only the data a surface needs.

## Vocabulary

### Frame-Local Data

Frame-local data is produced by rendering or layout and usually describes what was visible in one
frame. It is safe to rebuild every draw and store for the next event.

Current frame-value types include:

- `Regions`: visible regions, clipping, z-order, and layout hit testing.
- `FocusTargets`: keyboard traversal targets visible in the current frame.
- `PointerTargets`: pointer targets visible in the current frame.
- `CursorRequests`: terminal cursor requests produced during rendering.
- `FrameSnapshot`: an aggregate when several frame-local values should travel together.

Several `*Layout` values are also frame-local data even though they are not named `Plan`:

- `ContainerLayout`: outer area, inner area, clipping boundary, and optional child region.
- `GridLayout`: row areas, column areas, and typed cell regions.
- `ListLayout`: visible rows, content height, selection position, and list hit testing.
- `TableLayout`: visible cells, pinned headers, table hit testing, and scroll metrics.
- `ViewportLayout`: clamped viewport offset and visible content area.

### Persistent State

State survives across frames and belongs to the app or to small app-owned helpers. It is the input
to the next render pass, not the visible data produced by that render pass.

Current persistent state types include:

- `FocusState`: the focused app id.
- `PointerState`: hover and pressed ids.
- `SelectionState` and `VisibleSelection`: selected ids and visible-position bridges.
- `TextFieldState` and `ButtonRowState`: small control-local state.
- `ViewportState`, `VirtualListState`, and `VirtualTableState`: viewport and selection state for
  scrolling views.

### Context

Context is already a narrower term in the crate. `MeasureContext`, `RenderContext`,
`ListItemContext`, and `TableCellContext` are inputs passed down while measuring or rendering
external content.

That makes `Context` a poor name for the cross-cutting output of a frame. The useful phrase here is
`frame-local data`: concrete output produced during render and optionally stored for input routing.

## Data Selection Matrix

The current use cases can be described by which data they naturally need. `maybe` means the need is
policy-dependent or only appears in one common variant.

| Use case                  | Layout | Mouse | Focus | Selection | Cursor | Frame |
| ------------------------- | ------ | ----- | ----- | --------- | ------ | ----- |
| Ordered action surface    | yes    | maybe | maybe | maybe     | no     | maybe |
| Modal dialog              | yes    | yes   | maybe | no        | maybe  | maybe |
| Filtered record selection | maybe  | maybe | no    | yes       | no     | no    |
| Multi-pane scroll routing | yes    | yes   | maybe | no        | no     | maybe |
| Virtual list              | yes    | maybe | no    | maybe     | no     | maybe |
| Virtual table             | yes    | maybe | no    | maybe     | no     | maybe |
| Component composition     | yes    | maybe | maybe | app-owned | maybe  | yes   |
| Mouse overlays            | yes    | yes   | no    | no        | no     | maybe |
| Form editing              | yes    | maybe | yes   | maybe     | yes    | maybe |

The matrix suggests two design rules:

1. The smaller value types should remain first-class. `PointerTargets` alone should feel as normal
   as a `FrameSnapshot` when a surface only routes pointer input.
1. Aggregation should be convenient without becoming mandatory. `FrameSnapshot` is a carrying case
   for several values, not a declaration that every surface has layout, mouse, focus, selection, and
   cursor behavior.

## Composition Shapes

### One Value

A surface can return only one value when that is enough:

```rust
let mouse = PointerTargets::new().region(Pane::Details, details_area);
app.previous_details_mouse = mouse;
```

This is appropriate for simple hover/click routing, one scrollable pane, or a component that does
not participate in page focus.

### Several Independent Facts

A surface can store data separately when that keeps ownership clearer:

```rust
app.previous_focus = FocusTargets::from_regions(fields.regions().iter().copied());
app.previous_mouse = PointerTargets::from_targets(rendered_buttons);
```

This works well for small examples and for apps where focus and pointer routing are handled by
separate modules.

### Aggregate Boundary

A surface should return `FrameSnapshot` when a component boundary would otherwise return several values
that share ids and coordinates:

```rust
let frame = FrameSnapshot::from_layout(layout)
    .focus(focus)
    .mouse(mouse)
    .cursor(cursor);
```

The aggregate is useful at boundaries: parent components can map ids, translate coordinates, clip
hidden child data, and merge children without knowing how each child internally built its data.

### Derived Layout Plus State

Virtualized views often produce a rich layout value and update persistent state separately:

```rust
let layout = list.layout(area, items, &mut state);
let metrics = ScrollMetrics::from_list(&layout);
```

This shape keeps viewport math and visible-row data together while leaving selection and scroll
state app-owned.

## Cross-Use-Case Analysis

### 0001 Ordered Action Surfaces

Action surfaces often need layout, mouse, focus, disabled policy, and optional selection. They do
not need cursor requests. `FrameTargets::from_regions` fits because it derives aligned layout,
mouse, and focus target data from one ordered region list.

The gap is not the value model. The gap is an optional action-surface helper that can pair id, label,
width, enabled state, shortcut, and orientation while still returning ordinary values.

### 0002 Modal Dialog Coordination

Dialogs need layout and z-order. Most dialogs need pointer data because outside-click behavior and
frontmost routing matter. Focus is policy-dependent: a prompt with two buttons needs focus, a help
overlay may only capture Escape and scroll. Cursor data are child-dependent and should remain
optional.

The current pieces cover the model, but the call sites are a little too manual:

- `Container` computes outer and inner areas.
- `Row`, `Column`, or `Overlay` values child regions.
- `FrameTargets` can build child routing data.
- `FrameSnapshot::scroll_region` or `FrameTargets::mouse_region` can add a backdrop target.
- `FrameSnapshot::route_click` can produce a negative hit when no backdrop is used.

The `modal_prompt` example proves those pieces are enough for a small confirmation dialog: it
renders a page behind the prompt, traps focus in the prompt buttons, routes clicks through an
explicit backdrop target, and leaves form state out of the dialog model.

The missing abstraction, if one appears, is likely a small modal-shell helper, not a form helper:
outer area, inner area, z value, optional backdrop id, and a child frame merge. That helper should
not own fields, buttons, validation, or submit behavior.

### 0003 Filtered Record Selection

This use case is mostly persistent selection state plus a visible-id bridge. It may use region data
for rendering and pointer data for click selection, but the central problem is not a frame aggregate.

The common visible-id path now has direct helpers. Any remaining gap is around richer fallback
policies, range selection, or adapters from virtualized hit tests into durable record ids.

### 0004 Multi-Pane Scroll Routing

Scroll routing needs pointer data for whole-pane regions and layout or metrics for rendering status.
Focus is optional: some panes are keyboard-scrollable, others only scroll under the pointer.

The current pieces work for common panes. `ScrollMetrics::visible_range` covers simple fixed-height
line panes, while `VirtualListState` and `VirtualTableState` cover virtualized views. A small
non-virtual `ScrollState` may still be useful if examples keep storing bare offsets.

### 0005 Virtual List Row Rendering

Virtual lists produce a derived `ListLayout`, not just a generic `Regions`. Mouse data are
needed only when the app wants row clicks or wheel regions. Selection state is app-owned and should
remain distinct from viewport scrolling.

`VirtualListState::select_relative` now covers common keyboard movement over source indexes, while
`VirtualListState::scroll_viewport_by` keeps wheel scrolling separate from selection. This use case
validates having rich layout outputs in addition to generic values.

### 0006 Virtual Table Inspection

Tables need a richer layout output than a generic region list: visible cells, header/body positions,
two-axis metrics, and hit testing. Selection is persistent state; pointer data are optional depending
on whether clicks are enabled.

`VirtualTableState::select_relative` now covers common keyboard movement over source cell
positions. `TableLayout::cell_regions` and `TableLayout::cells_regions` let apps project visible cells
into generic layout or mouse coordination when needed, while `TableLayout::select_hit` keeps
body-cell click selection separate from header actions.

This use case argues against forcing everything through `FrameSnapshot`. A table can expose
`TableLayout` and let the app decide which frame-local data to derive.

### 0007 Component Frame Composition

This is the strongest `FrameSnapshot` use case. The parent component needs to merge child data,
translate local coordinates, clip hidden regions, and preserve cursor request order.

`component_frames` now shows a middle-sized version of that boundary: children return local
`FrameSnapshot` values, the parent maps ids into one app enum, places the child frames with solved
screen areas, and stores the merged snapshot for input. Cursor requests translate and clip with the
rest of the child data, which matters for focused text inputs inside panes.

The aggregate exists for this boundary. It should stay optional for smaller components.

### 0008 Mouse Overlays And Disabled Targets

This use case can be solved entirely with `PointerTargets` and `PointerState`. Layout data may be useful
for rendering, but pointer routing should not require focus or selection.

`PointerPhase` now covers the backend-agnostic hover, press, and release transition that examples
were repeating. Backends still convert their own event kinds at the app edge, and scroll, drag,
buttons, and modifiers remain app policy until examples prove a narrower helper.

This use case is the clearest reminder that smaller values must remain ergonomic. A hoverable row,
disabled button, or popup target should not require a full `FrameSnapshot`.

### 0009 Form Editing

Forms combine layout, focus, cursor, and app-owned edit buffers. Mouse data are optional, and
selection depends on whether the form has radio groups, segmented choices, or list-like fields.

`form_dialog` demonstrates the small-piece path directly: named field regions, focus repair,
`TextFieldState` for edit mechanics, and cursor requests derived from the focused field's label
prefix. `profile_editor` adds a saved value, pending draft, validation status, disabled Save
command, and mixed vertical-field/horizontal-command focus. A broad form abstraction would likely
hide too many app decisions too early. Better candidates are narrow helpers around
validation-to-disabled-command wiring if more examples repeat that shape.

## Review Outcome

The current crate has the right conceptual pieces. The main risk is presentation: a reader should
not infer that `FrameSnapshot` is the normal return type for every surface.

The use-case review led to three concrete documentation and example decisions:

- Keep "Choosing what to store" in the crate root as the concept-level explanation of frame-local data,
  persistent state, derived layouts, metrics, and render contexts.
- Teach the same aggregate-vs-smaller-value rule from `docs::frame_snapshots`, where readers already
  arrive when they see `FrameSnapshot`.
- Add `pointer_only_scroll_region` because the existing `scroll_regions` example used
  `FrameSnapshot` for a shape that can be shown with `PointerTargets` alone.

The other small-value examples already exist under clearer names:

| Shape                 | Example                      | Demonstrates                         |
| --------------------- | ---------------------------- | ------------------------------------ |
| Mouse-only scroll     | `pointer_only_scroll_region` | wheel routing with `PointerTargets`  |
| Focus-only fields     | `focus_traversal`            | `FocusTargets` and `FocusState`      |
| Selection without agg | `selection_modes`            | selection outside values             |
| Backdrop without form | `modal_prompt`               | shell data separate from form        |
| Child aggregate       | `component_frames`           | where `FrameSnapshot` earns its keep |

Do not add bundle helpers yet. The repeated patterns are real, but they still carry app policy:

- modal shells decide backdrop behavior, escape behavior, focus trap policy, and child ownership;
- action surfaces decide labels, shortcuts, orientation, selection, and enabled state;
- field rows decide validation, labels, cursor prefixes, and edit-buffer ownership;
- scroll panes decide whether wheel, keyboard focus, selection, and virtualization are coupled.

Those may become helpers later, but the next proof should come from examples where the same
coordination machinery appears again with the same policy boundaries.

## Review Questions

When reviewing a new primitive or example, ask:

1. Which frame-local data does this surface actually produce?
1. Which persistent state does the app own?
1. Does the example use `FrameSnapshot` because a boundary benefits from aggregation, or only
   because it is available?
1. Could a smaller value make the example clearer?
1. Does a helper reduce repeated coordination, or does it hide a policy the app should make?
1. Are derived layouts such as `ListLayout` or `TableLayout` more appropriate than generic values?
1. Is the word `Context` being used only for callback inputs rather than frame outputs?
