# ratatui-wasm

Capability-based WebAssembly plugin host for Ratatui widgets.

Load `.wasm` components at runtime and render them inside a Ratatui application without rebuilding the host. Each plugin declares the capabilities it needs, and the host only grants those capabilities through WASI.

## Why

Ratatui applications normally bundle every widget as Rust code. This crate turns widgets into reloadable plugins so you can:

- Update UI pieces without restarting the application.
- Let third parties write widgets in any language that compiles to WASM.
- Run untrusted widgets with a small, explicit permission set.

## How it works

A guest module exports the `ratatui:widget/widget` interface defined in `wit/widget.wit`. The host:

1. Loads the `.wasm` component and checks its declared capabilities against the manifest.
2. Builds a `WasiCtx` with only the granted capabilities.
3. Calls `render` and receives draw commands such as cells or styled lines.
4. Blits those commands onto a `ratatui_core::buffer::Buffer`.

```text
guest .wasm  --WIT-->  ratatui-wasm host  -->  Ratatui buffer  -->  terminal
```

## Quick start

Build the example Rust guest:

```sh
cargo build --target wasm32-wasip2 --release \
  -p hello-rust --manifest-path examples/wasm-widgets/hello-rust/Cargo.toml
```

Validate the plugin:

```sh
cargo run -p ratatui-wasm --features cli -- check \
  examples/wasm-widgets/hello-rust/ratatui.plugin.toml
```

Run the demo app:

```sh
cargo run -p wasm-demo
```

Inside the demo, press `+` or `-` to change the counter, and `q` to quit.

Try the C guest:

```powershell
cd examples/wasm-widgets/hello-c
.\build.ps1
cargo run -p ratatui-wasm --features cli -- check ratatui.plugin.toml
```

## API overview

### Render a plugin widget

```rust
use ratatui_wasm::WasmWidget;

let widget = WasmWidget::from_file("hello_rust.wasm", &[]);
frame.render_widget(widget, area);
```

`WasmWidget` implements `ratatui_core::widgets::Widget`. The module is re-instantiated on every frame, so editing and recompiling the guest is visible immediately.

### Stateful widgets

Use `StatefulWasmWidget` when the guest needs to persist state across frames.

```rust
use ratatui_wasm::StatefulWasmWidget;

let widget = StatefulWasmWidget::from_file("stateful.wasm", &[]);
frame.render_stateful_widget(widget, area, &mut state);
```

### Send events to a plugin

For interactive widgets, keep a `PluginWidget` instance and call `handle_event`.

```rust
use ratatui_wasm::{event, PluginWidget};

let mut plugin = PluginWidget::from_file("input.wasm", &[])?;
let consumed = plugin.handle_event(&event::key(event::char_key('q'), 0))?;
```

### Load from a manifest

A `ratatui.plugin.toml` file keeps metadata, the entry wasm path, and required capabilities.

```rust
use ratatui_wasm::WasmWidget;

let widget = WasmWidget::from_manifest("my-widget/ratatui.plugin.toml")?;
```

## Capabilities

Plugins declare what they need. The host refuses to load a plugin that asks for a capability it was not granted. The manifest is the source of truth; the WASI context is configured from it.

| Capability | Meaning |
|------------|---------|
| `stdio:stdout` | Write to host stdout |
| `stdio:stderr` | Write to host stderr |
| `stdio:stdin` | Read from host stdin |
| `env:read` | Read host environment variables |

Clock and file capabilities will follow the same pattern as WASI support expands.

## Hot reload and caching

The host caches parsed WASM components keyed by file modification time. On each render it checks the file again, so a recompiled plugin is reloaded automatically while unmodified plugins stay fast.

## Writing guests in other languages

The interface is language-agnostic. Any toolchain that implements the `ratatui:widget/widget` WIT world can produce a plugin. See `examples/wasm-widgets/hello-c` for a C guest built with `wit-bindgen` and the WASI SDK.

## Project layout

- `wit/widget.wit`: Interface definition.
- `src/host.rs`: Component loader, capability enforcement, render helpers.
- `src/event.rs`: Host-side event builders.
- `src/manifest.rs`: `ratatui.plugin.toml` parser.
- `src/cache.rs`: File modification time cache for hot reload.
- `src/lib.rs`: draw command blitting and WIT type re-exports.
- `src/main.rs`: Optional CLI under the `cli` feature.
- `tests/`: Integration and command tests.
- `examples/wasm-widgets/hello-rust`: Rust guest example.
- `examples/apps/wasm-demo`: Minimal Ratatui app using a plugin.

## License

Same as the `ratatui` workspace.
