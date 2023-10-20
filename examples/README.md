# Examples

These gifs were created using [VHS](https://github.com/charmbracelet/vhs). Each example has a
corresponding `.tape` file that holds instructions for how to generate the images. Note that the
images themselves are stored in a separate git branch to avoid bloating the main repository.

## Demo2

This is the demo example from the main README and crate page. Source: [demo2](./demo2/).

```shell
cargo run --example=demo2 --features=crossterm
```

![Demo2][demo2.gif]

## Demo

This is the previous demo example from the main README. It is available for each of the backends. Source:
[demo.rs](./demo/).

```shell
cargo run --example=demo --features=crossterm
cargo run --example=demo --no-default-features --features=termion
cargo run --example=demo --no-default-features --features=termwiz
```

![Demo][demo.gif]

## Hello World

This is a pretty boring example, but it contains some good documentation
on writing tui apps. Source: [hello_world.rs](./hello_world.rs).

```shell
cargo run --example=hello_world --features=crossterm
```

![Hello World][hello_world.gif]

## Barchart

Demonstrates the [`BarChart`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.BarChart.html)
widget. Source: [barchart.rs](./barchart.rs).

```shell
cargo run --example=barchart --features=crossterm
```

![Barchart][barchart.gif]

## Block

Demonstrates the [`Block`](https://docs.rs/ratatui/latest/ratatui/widgets/block/struct.Block.html)
widget. Source: [block.rs](./block.rs).

```shell
cargo run --example=block --features=crossterm
```

![Block][block.gif]

## Calendar

Demonstrates the [`Calendar`](https://docs.rs/ratatui/latest/ratatui/widgets/calendar/index.html)
widget. Source: [calendar.rs](./calendar.rs).

```shell
cargo run --example=calendar --features="crossterm widget-calendar"
```

![Calendar][calendar.gif]

## Canvas

Demonstrates the [`Canvas`](https://docs.rs/ratatui/latest/ratatui/widgets/canvas/index.html) widget
and related shapes in the
[`canvas`](https://docs.rs/ratatui/latest/ratatui/widgets/canvas/index.html) module. Source:
[canvas.rs](./canvas.rs).

```shell
cargo run --example=canvas --features=crossterm
```

![Canvas][canvas.gif]

## Chart

Demonstrates the [`Chart`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Chart.html) widget.
Source: [chart.rs](./chart.rs).

```shell
cargo run --example=chart --features=crossterm
```

![Chart][chart.gif]

## Colors

Demonstrates the available [`Color`](https://docs.rs/ratatui/latest/ratatui/style/enum.Color.html)
options. These can be used in any style field. Source: [colors.rs](./colors.rs).

```shell
cargo run --example=colors --features=crossterm
```

![Colors][colors.gif]

## Colors (RGB)

Demonstrates the available RGB
[`Color`](https://docs.rs/ratatui/latest/ratatui/style/enum.Color.html) options. These can be used
in any style field. Source: [colors_rgb.rs](./colors_rgb.rs). Uses a half block technique to render
two square-ish pixels in the space of a single rectangular terminal cell.

```shell
cargo run --example=colors_rgb --features=crossterm
```

![Colors RGB][colors_rgb.png]

## Custom Widget

Demonstrates how to implement the
[`Widget`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html) trait. Also shows mouse
interaction. Source: [custom_widget.rs](./custom_widget.rs).

```shell
cargo run --example=custom_widget --features=crossterm
```

![Custom Widget][custom_widget.gif]

## Gauge

Demonstrates the [`Gauge`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Gauge.html) widget.
Source: [gauge.rs](./gauge.rs).

```shell
cargo run --example=gauge --features=crossterm
```

![Gauge][gauge.gif]

## Inline

Demonstrates how to use the
[`Inline`](https://docs.rs/ratatui/latest/ratatui/terminal/enum.Viewport.html#variant.Inline)
Viewport mode for ratatui apps. Source: [inline.rs](./inline.rs).

```shell
cargo run --example=inline --features=crossterm
```

![Inline][inline.gif]

## Layout

Demonstrates the [`Layout`](https://docs.rs/ratatui/latest/ratatui/layout/struct.Layout.html) and
interaction between each constraint. Source:  [layout.rs](./layout.rs).

```shell
cargo run --example=layout --features=crossterm
```

![Layout][layout.gif]

## List

Demonstrates the [`List`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.List.html) widget.
Source: [list.rs](./list.rs).

```shell
cargo run --example=list --features=crossterm
```

![List][list.gif]

## Modifiers

Demonstrates the style
[`Modifiers`](https://docs.rs/ratatui/latest/ratatui/style/struct.Modifier.html). Source:
[modifiers.rs](./modifiers.rs).

```shell
cargo run --example=modifiers --features=crossterm
```

![Modifiers][modifiers.gif]

## Panic

Demonstrates how to handle panics by ensuring that panic messages are written correctly to the
screen. Source: [panic.rs](./panic.rs).

```shell
cargo run --example=panic --features=crossterm
```

![Panic][panic.gif]

## Paragraph

Demonstrates the [`Paragraph`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Paragraph.html)
widget. Source: [paragraph.rs](./paragraph.rs)

```shell
cargo run --example=paragraph --features=crossterm
```

![Paragraph][paragraph.gif]

## Popup

Demonstrates how to render a widget over the top of previously rendered widgets using the
[`Clear`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Clear.html) widget. Source:
[popup.rs](./popup.rs).

>
```shell
cargo run --example=popup --features=crossterm
```

![Popup][popup.gif]

## Scrollbar

Demonstrates the [`Scrollbar`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Scrollbar.html)
widget. Source: [scrollbar.rs](./scrollbar.rs).

```shell
cargo run --example=scrollbar --features=crossterm
```

![Scrollbar][scrollbar.gif]

## Sparkline

Demonstrates the [`Sparkline`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Sparkline.html)
widget. Source: [sparkline.rs](./sparkline.rs).

```shell
cargo run --example=sparkline --features=crossterm
```

![Sparkline][sparkline.gif]

## Table

Demonstrates the [`Table`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Table.html) widget.
Source: [table.rs](./table.rs).

```shell
cargo run --example=table --features=crossterm
```

![Table][table.gif]

## Tabs

Demonstrates the [`Tabs`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Tabs.html) widget.
Source: [tabs.rs](./tabs.rs).

```shell
cargo run --example=tabs --features=crossterm
```

![Tabs][tabs.gif]

## User Input

Demonstrates one approach to accepting user input. Source [user_input.rs](./user_input.rs).

> [!NOTE] Consider using [`tui-textarea`](https://crates.io/crates/tui-textarea) or
> [`tui-input`](https://crates.io/crates/tui-input) crates for more functional text entry UIs.

```shell
cargo run --example=user_input --features=crossterm
```

![User Input][user_input.gif]

<!--
links to images to make it easier to update in bulk
These are generated with `vhs publish examples/xxx.gif`

To update these examples in bulk:
```shell
examples/generate.bash
```
-->

[barchart.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/barchart.gif?raw=true
[block.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/block.gif?raw=true
[calendar.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/calendar.gif?raw=true
[canvas.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/canvas.gif?raw=true
[chart.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/chart.gif?raw=true
[colors.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/colors.gif?raw=true
[colors_rgb.png]: https://github.com/ratatui-org/ratatui/blob/images/examples/colors_rgb.png?raw=true
[custom_widget.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/custom_widget.gif?raw=true
[demo.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/demo.gif?raw=true
[demo2.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/demo2.gif?raw=true
[gauge.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/gauge.gif?raw=true
[hello_world.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/hello_world.gif?raw=true
[inline.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/inline.gif?raw=true
[layout.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/layout.gif?raw=true
[list.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/list.gif?raw=true
[modifiers.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/modifiers.gif?raw=true
[panic.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/panic.gif?raw=true
[paragraph.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/paragraph.gif?raw=true
[popup.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/popup.gif?raw=true
[scrollbar.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/scrollbar.gif?raw=true
[sparkline.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/sparkline.gif?raw=true
[table.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/table.gif?raw=true
[tabs.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/tabs.gif?raw=true
[user_input.gif]: https://github.com/ratatui-org/ratatui/blob/images/examples/user_input.gif?raw=true
