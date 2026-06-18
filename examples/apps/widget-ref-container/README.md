# WidgetRef Container demo

This example shows how to use [`WidgetRef`] to store widgets in a container.

`WidgetRef` is experimental and requires the `unstable-widget-ref` feature flag. It is useful when
you need dynamic dispatch, such as storing different widget types behind `Box<dyn WidgetRef>`.

For normal application code, start with a simpler enum and `match` when the possible widgets for an
area are known. In an app, that might look like:

```rust
match self.message {
    Message::Greeting => frame.render_widget(&Greeting, area),
    Message::Farewell => frame.render_widget(&Farewell, area),
}
```

That keeps ownership and state flow explicit. Use `WidgetRef` when you specifically need a boxed
heterogeneous widget container:

```rust
use ratatui::widgets::WidgetRef;

let widget: Box<dyn WidgetRef> = match self.message {
    Message::Greeting => Box::new(&Greeting),
    Message::Farewell => Box::new(&Farewell),
};

widget.render_ref(area, frame.buffer_mut());
```

The reason this example uses `WidgetRef` rather than `Widget` is that
[`Widget::render`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html#tymethod.render)
consumes `self`, so `Box<dyn Widget>` is not the right trait object for this pattern. See the
[`WidgetRef` tracking issue](https://github.com/ratatui/ratatui/issues/1287) for the ongoing API
discussion.

To run this demo:

```shell
cargo run -p widget-ref-container
```

[`WidgetRef`]: https://docs.rs/ratatui/latest/ratatui/widgets/trait.WidgetRef.html
