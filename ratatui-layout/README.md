# ratatui-layout

`ratatui-layout` is a `0.0.1-alpha.0` experiment for frame-local UI coordination in Ratatui.

This crate is extremely experimental. It exists to test ideas, examples, and API pressure around
immediate-mode UI coordination. Expect names, module boundaries, responsibilities, and maybe the
crate shape itself to change substantially; the properly baked version is likely to look different
from this alpha.

The crate keeps Ratatui immediate-mode: apps still render inside `Terminal::draw`, and ordinary
widgets still draw into `Rect` and `Buffer`. The added layer is optional data produced by a render
pass: visible regions, focus targets, pointer targets, selection state, cursor requests, viewport
metadata, and scroll metrics.

Rustdoc is the main guide. Start with `ratatui_layout::docs` for the coordination model, the normal
Ratatui alternatives, and the current API boundaries.

For product and API validation, see [`use-cases/`](use-cases/). Those notes describe the app shapes
used to check whether a proposed helper removes real coordination work.

## Examples

End-to-end showcase:

```bash
cargo run -p layout-workspace-inspector
cargo run -p layout-routing-lab
```

Layout and composition:

```bash
cargo run -p ratatui-layout --example left_right_row
cargo run -p ratatui-layout --example external_participants
cargo run -p ratatui-layout --example modal_prompt
cargo run -p ratatui-layout --example modal_shell
cargo run -p ratatui-layout --example container_dialog
cargo run -p ratatui-layout --example component_frames
```

Interaction coordination:

```bash
cargo run -p ratatui-layout --example pointer_targets
cargo run -p ratatui-layout --example focus_traversal
cargo run -p ratatui-layout --example selection_modes
cargo run -p ratatui-layout --example pointer_only_scroll_region
cargo run -p ratatui-layout --example scroll_regions
cargo run -p ratatui-layout --example cursor_request
cargo run -p ratatui-layout --example toolbar_targets
cargo run -p ratatui-layout --example action_surface
cargo run -p ratatui-layout --example form_dialog
cargo run -p ratatui-layout --example profile_editor
cargo run -p ratatui-layout --example text_input
cargo run -p ratatui-layout --example grid_palette
cargo run -p ratatui-layout --example frame_snapshot
cargo run -p ratatui-layout --example visible_selection_filter
```

Virtualization and scroll metadata:

```bash
cargo run -p ratatui-layout --example viewport
cargo run -p ratatui-layout --example scrollable_pane
cargo run -p ratatui-layout --example virtual_list
cargo run -p ratatui-layout --example visible_selection_virtual_list
cargo run -p ratatui-layout --example virtual_record_list
cargo run -p ratatui-layout --example table_inspector
cargo run -p ratatui-layout --example tree_browser
```

Use normal Ratatui `Layout` directly when solved rectangles are consumed immediately. Use this crate
when visible regions or interaction data need to be stored, inspected, merged, clipped, or used to
route the next input event.
