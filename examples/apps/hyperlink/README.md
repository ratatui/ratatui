# Hyperlink demo

This example shows how to render hyperlinks in a terminal using [OSC 8](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda).

It renders text as usual and then attaches links to the buffer cells with `Buffer::set_hyperlink`
and `Buffer::set_line_with_hyperlink`, which need the `cell-extensions` feature of `ratatui`. The
backend writes the escape sequences around the linked cells, so links don't affect the layout.

To run this demo:

```shell
cargo run -p hyperlink
```
