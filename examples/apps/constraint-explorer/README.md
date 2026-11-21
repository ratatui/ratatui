# Constraint explorer demo

This interactive example helps build intuition for Ratatui's layout engine. Experiment with
different constraint types and values, add or remove layout blocks, change the spacing between them
(including overlap), and compare how the `Flex` modes distribute the available space.

Use the arrow keys to select and edit a block, `1`–`6` to change its constraint type, `a` and `x`
to add or remove blocks, and `+` and `-` to change spacing. Press `q` or `Esc` to quit.

![Constraint Explorer demo][constraint-explorer.gif]

To run this demo:

```shell
cargo run -p constraint-explorer
```

[constraint-explorer.gif]: https://github.com/ratatui/ratatui/blob/images/examples/constraint-explorer.gif?raw=true
