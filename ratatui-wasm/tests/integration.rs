use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_wasm::PluginWidget;

fn wasm_path() -> &'static str {
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../examples/wasm-widgets/hello-widget/target/wasm32-wasip2/release/hello_widget.wasm"
    )
}

#[test]
fn loads_and_renders_hello_widget() {
    let path = wasm_path();
    let mut widget = PluginWidget::from_file(&path, &[]).expect("widget loads");
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
