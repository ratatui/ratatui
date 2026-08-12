use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::widgets::Widget;
use ratatui_wasm::{PluginWidget, StatefulWasmWidget, WasmWidget, event};

const fn manifest_path() -> &'static str {
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../examples/wasm-widgets/hello-rust/ratatui.plugin.toml"
    )
}

const fn c_wasm_path() -> &'static str {
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../examples/wasm-widgets/hello-c/hello_c.wasm"
    )
}

const fn wasm_path() -> &'static str {
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../examples/wasm-widgets/hello-rust/target/wasm32-wasip2/release/hello_rust.wasm"
    )
}

#[test]
fn loads_and_renders_hello_rust() {
    let path = wasm_path();
    let mut widget = PluginWidget::from_file(path, &[]).expect("widget loads");
    let mut buf = Buffer::empty(Rect::new(0, 0, 40, 3));
    widget
        .render(Rect::new(0, 0, 40, 3), &mut buf)
        .expect("widget renders");

    let line: String = buf
        .content()
        .iter()
        .take(40)
        .map(ratatui_core::buffer::Cell::symbol)
        .collect();
    assert!(
        line.contains("Hello from WASM"),
        "expected rendered text, got: {line:?}"
    );
}

#[test]
fn wasm_widget_wrapper_renders_via_widget_trait() {
    let path = wasm_path();
    let widget = WasmWidget::from_file(path, &[]);
    let mut buf = Buffer::empty(Rect::new(0, 0, 40, 3));
    widget.render(Rect::new(0, 0, 40, 3), &mut buf);

    let line: String = buf
        .content()
        .iter()
        .take(40)
        .map(ratatui_core::buffer::Cell::symbol)
        .collect();
    assert!(
        line.contains("Hello from WASM"),
        "expected rendered text via WasmWidget wrapper, got: {line:?}"
    );
}

#[test]
fn widget_handles_key_events() {
    let path = wasm_path();
    let mut widget = PluginWidget::from_file(path, &[]).expect("widget loads");

    let event = event::key(event::char_key('q'), 0);
    let handled = widget
        .handle_event(&event)
        .expect("widget can handle events");
    assert!(!handled, "hello-rust does not claim key events");
}

#[test]
fn stateful_widget_persists_state() {
    let path = wasm_path();
    let widget = StatefulWasmWidget::from_file(path, &[]);
    let mut state: Vec<u8> = Vec::new();
    let mut buf = Buffer::empty(Rect::new(0, 0, 40, 3));
    ratatui_core::widgets::StatefulWidget::render(
        widget,
        Rect::new(0, 0, 40, 3),
        &mut buf,
        &mut state,
    );

    let line: String = buf
        .content()
        .iter()
        .take(40)
        .map(ratatui_core::buffer::Cell::symbol)
        .collect();
    assert!(
        line.contains("Hello from WASM"),
        "expected rendered text via StatefulWasmWidget, got: {line:?}"
    );
    assert!(state.is_empty(), "hello-rust does not emit state");
}

#[test]
fn loads_widget_from_manifest() {
    let mut widget = PluginWidget::from_manifest(manifest_path()).expect("manifest loads widget");
    let mut buf = Buffer::empty(Rect::new(0, 0, 40, 3));
    widget
        .render(Rect::new(0, 0, 40, 3), &mut buf)
        .expect("render");

    let line: String = buf
        .content()
        .iter()
        .take(40)
        .map(ratatui_core::buffer::Cell::symbol)
        .collect();
    assert!(
        line.contains("Hello from WASM"),
        "expected rendered text from manifest, got: {line:?}"
    );
}

#[test]
fn wasm_widget_from_manifest_renders() {
    let widget = WasmWidget::from_manifest(manifest_path()).expect("manifest creates widget");
    let mut buf = Buffer::empty(Rect::new(0, 0, 40, 3));
    ratatui_core::widgets::Widget::render(widget, Rect::new(0, 0, 40, 3), &mut buf);

    let line: String = buf
        .content()
        .iter()
        .take(40)
        .map(ratatui_core::buffer::Cell::symbol)
        .collect();
    assert!(
        line.contains("Hello from WASM"),
        "expected rendered text from WasmWidget manifest, got: {line:?}"
    );
}

#[test]
fn c_guest_renders() {
    let path = c_wasm_path();
    if !std::path::Path::new(path).exists() {
        return;
    }
    let mut widget = PluginWidget::from_file(path, &[]).expect("C widget loads");
    let mut buf = Buffer::empty(Rect::new(0, 0, 40, 3));
    widget
        .render(Rect::new(0, 0, 40, 3), &mut buf)
        .expect("C widget renders");

    let line: String = buf
        .content()
        .iter()
        .take(40)
        .map(ratatui_core::buffer::Cell::symbol)
        .collect();
    assert!(
        line.contains("Hello from C"),
        "expected rendered text from C guest, got: {line:?}"
    );
}

#[test]
fn from_file_fails_when_wasm_missing() {
    let result = PluginWidget::from_file("/tmp/ratatui-wasm-missing.wasm", &[]);
    assert!(result.is_err());
}

#[test]
fn from_manifest_fails_when_manifest_missing() {
    let result = PluginWidget::from_manifest("/tmp/ratatui-wasm-missing.toml");
    assert!(result.is_err());
}

#[test]
fn wasm_widget_from_manifest_fails_when_missing() {
    let result = WasmWidget::from_manifest("/tmp/ratatui-wasm-missing.toml");
    assert!(result.is_err());
}

#[test]
fn widget_host_can_be_constructed() {
    let _host = ratatui_wasm::WasmWidgetHost::new();
}

#[test]
fn widget_fails_with_invalid_wasm_file() {
    let temp_dir = std::env::temp_dir().join(format!("ratatui-wasm-bad-{}", std::process::id()));
    let _ = std::fs::remove_file(&temp_dir);
    std::fs::write(&temp_dir, b"not a wasm component").unwrap();
    let result = PluginWidget::from_file(&temp_dir, &[]);
    assert!(result.is_err());
    let _ = std::fs::remove_file(&temp_dir);
}
