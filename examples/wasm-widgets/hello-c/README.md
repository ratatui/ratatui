# hello-c

A C guest widget for `ratatui-wasm`.

This example shows that the widget interface is not tied to Rust. Any language that compiles to a WebAssembly Component Model module can implement the `ratatui:widget/widget` WIT world.

## What it does

Renders the text `Hello from C` with a green foreground and black background, using the same low-level cell commands as the Rust `hello-rust`.

## Prerequisites

- The WASI SDK, installed at `C:\wasi-sdk` or pointed to by the `WASI_SDK` environment variable.
- `wit-bindgen` CLI version `0.60.0`:

```sh
cargo install wit-bindgen-cli --version 0.60.0
```

## Build

On Windows:

```powershell
.\build.ps1
```

The script does three steps:

1. Generate C bindings from the WIT file.
2. Compile `main.c` and the generated binding source to WebAssembly object files.
3. Link the objects into a component module `hello_c.wasm`.

## Validate

From the workspace root:

```sh
cargo run -p ratatui-wasm --features cli -- check examples/wasm-widgets/hello-c/ratatui.plugin.toml
```

## How it maps to Rust

| Rust guest | C guest |
|------------|---------|
| `Guest::render` | `exports_ratatui_widget_widget_render` |
| `Guest::handle_event` | `exports_ratatui_widget_widget_handle_event` |
| `Guest::capabilities` | `exports_ratatui_widget_widget_capabilities` |
| `RenderCommand::Cell` | `EXPORTS_RATATUI_WIDGET_WIDGET_RENDER_COMMAND_CELL` |
| `Color::Green` | `EXPORTS_RATATUI_WIDGET_WIDGET_COLOR_GREEN` |

Strings are owned by the guest and freed through the generated helpers after the host lifts the return value.
