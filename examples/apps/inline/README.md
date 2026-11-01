# Inline demo

This example shows how to use an inline viewport to render in a specific area of the screen.

It simulates a small pool of workers downloading a queue of files. The viewport shows the total
progress and a row for each worker, and each finished download is printed above the viewport with
`Terminal::insert_before`, so it stays in the terminal's scrollback.

To run this demo:

```shell
cargo run -p inline
```
