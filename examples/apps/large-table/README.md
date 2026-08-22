# Large Table demo

This example shows how to create a table that handles large datasets with
pagination and virtual scrolling.

To run this demo:

```shell
cargo run -p large-table
```

Use `-n` to specify the number of rows (default 20). Pass 0 for an empty table:

```shell
cargo run -p large-table -- -n 100
```
