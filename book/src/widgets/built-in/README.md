# Built in

The library comes with the following
[widgets](https://docs.rs/ratatui/latest/ratatui/widgets/index.html):

* [Canvas](canvas.md) ([rustdoc](https://docs.rs/ratatui/latest/ratatui/widgets/canvas/struct.Canvas.html)) which allows
  rendering [points, lines, shapes and a world
  map](https://docs.rs/ratatui/latest/ratatui/widgets/canvas/index.html)
* [BarChart](barchart.md) ([rustdoc](https://docs.rs/ratatui/latest/ratatui/widgets/struct.BarChart.html))
* [Block](block.md) ([rustdoc](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Block.html))
* [Calendar](calendar.md) ([rustdoc](https://docs.rs/ratatui/latest/ratatui/widgets/calendar/index.html))
* [Chart](chart.md) ([rustdoc](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Chart.html))
* [Gauge](gauge.md) ([rustdoc](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Gauge.html))
* [List](list.md) ([rustdoc](https://docs.rs/ratatui/latest/ratatui/widgets/struct.List.html))
* [Paragraph](paragraph.md) ([rustdoc](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Paragraph.html))
* [Sparkline](sparkline.md) ([rustdoc](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Sparkline.html))
* [Table](table.md) ([rustdoc](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Table.html))
* [Tabs](tabs.md) ([rustdoc](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Tabs.html))

Each widget has an associated example which can be found in the [examples](./examples/) folder. Run
each examples with cargo (e.g. to run the gauge example `cargo run --example gauge`), and quit by
pressing `q`.

You can also run all examples by running `cargo make run-examples` (requires `cargo-make` that can
be installed with `cargo install cargo-make`).