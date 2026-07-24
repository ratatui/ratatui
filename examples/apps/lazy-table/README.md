# Lazy table demo

This example shows how to display a table with 10 million rows using lazy implementation.

The status bar reports how many rows the table actually built for the last frame and how long the
frame took, so you can watch scrolling stay flat no matter how far into the dataset you jump. Press
`v` to switch between uniform and variable row heights.

To run this demo:

```shell
cargo run -p lazy-table
```
