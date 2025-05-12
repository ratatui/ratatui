# Ratatui Mutable state examples

This set of examples shows a few different ways to use mutable state while rendering with the
Ratatui library. Each example demonstrates a solution to the same problem. When an event occurs, a
counter is incremented and the message on the screen is updated to reflect the new value of the
counter.

To run the examples, clone the repository and run:

```bash
cargo run --bin examples/<example_name>.rs
```

Press any key to increment the counter. Press `<Esc>` to exit.

## Examples

- [`component_trait.rs`]: Implements a custom `Component` trait (with a `&mut self` render method)
  for state mutation.
- [`mutable_widget.rs`]: Implements `Widget` for `&mut CounterWidget`, allowing direct state changes
  during rendering.
- [`nested_mutable_widget.rs`]: Nests a `&mut CounterWidget` within a parent widget for hierarchical
  state mutation.
- [`nested_stateful_widget.rs`]: Nests one `StatefulWidget` within a parent for hierarchical state
  management.
- [`refcell.rs`]: Uses `RefCell` (`Rc<Cell<usize>>`) within a widget for interior mutability,
  enabling state changes on immutable borrows.
- [`render_function.rs`]: Uses a simple render function taking a mutable state reference for direct
  state mutation.
- [`stateful_widget.rs`]: Employs `StatefulWidget` to separate rendering logic from externally
  managed mutable state.
- [`widget_with_mutable_ref.rs`]: Stores a mutable state reference (`&'a mut usize`) directly as a
  field in the widget struct

[`component_trait.rs`]: ./src/bin/component_trait.rs
[`mutable_widget.rs`]: ./src/bin/mutable_widget.rs
[`nested_mutable_widget.rs`]: ./src/bin/nested_mutable_widget.rs
[`nested_stateful_widget.rs`]: ./src/bin/nested_stateful_widget.rs
[`refcell.rs`]: ./src/bin/refcell.rs
[`render_function.rs`]: ./src/bin/render_function.rs
[`stateful_widget.rs`]: ./src/bin/stateful_widget.rs
[`widget_with_mutable_ref.rs`]: ./src/bin/widget_with_mutable_ref.rs
