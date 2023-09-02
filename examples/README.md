# Examples

These gifs were created using [Charm VHS](https://github.com/charmbracelet/vhs).

VHS has a problem rendering some background color transitions, which shows up in several examples
below. See <https://github.com/charmbracelet/vhs/issues/344> for more info. These problems don't
occur in a terminal.

## Demo

This is the demo example from the main README. It is available for each of the backends. Source:
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
cargo run --example=calendar --features=crossterm widget-calendar
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

## Custom Widget

Demonstrates how to implement the
[`Widget`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html) trait. Source:
[custom_widget.rs](./custom_widget.rs).

```shell
cargo run --example=custom_widget --features=crossterm
```

![Custom Widget][custom_widget.gif]

## Gauge

Demonstrates the [`Gauge`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Gauge.html) widget.
Source: [gauge.rs](./gauge.rs).

> [!NOTE] The backgrounds render poorly when we generate this example using VHS. This problem
> doesn't generally happen during normal rendering in a terminal. See
> [vhs#344](https://github.com/charmbracelet/vhs/issues/344) for more details.

```shell
cargo run --example=gauge --features=crossterm
```

![Gauge][gauge.gif]

## Inline

Demonstrates the
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

> [!NOTE] The background renders poorly after the popup when we generate this example using VHS.
> This problem doesn't generally happen during normal rendering in a terminal. See
> [vhs#344](https://github.com/charmbracelet/vhs/issues/344) for more details.

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

> [!NOTE] The background renders poorly in the second sparkline when we generate this example using
> VHS. This problem doesn't generally happen during normal rendering in a terminal. See
> [vhs#344](https://github.com/charmbracelet/vhs/issues/344) for more details.

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
# build to ensure that running the examples doesn't have to wait so long
cargo build --examples --features=crossterm,all-widgets
for i in examples/*.tape
do
    echo -n "[${i:s:examples/:::s:.tape:.gif:}]: "
    vhs $i --publish --quiet
    # may need to adjust this depending on if you see rate limiting from VHS
    sleep 1
done
```
-->
[barchart.gif]: https://vhs.charm.sh/vhs-6przhDzUmjMVb0wH4RdPa9.gif
[block.gif]: https://vhs.charm.sh/vhs-1NBeg0ChTWTVrCV7D1tLPe.gif
[calendar.gif]: https://vhs.charm.sh/vhs-c5xBWMM5tnf3m8IV3gE2d.gif
[canvas.gif]: https://vhs.charm.sh/vhs-44kPYDX7PM0jxUFt6Q6EYL.gif
[chart.gif]: https://vhs.charm.sh/vhs-7aCL8RiYpokkxsPKsNIaPb.gif
[colors.gif]: https://vhs.charm.sh/vhs-7r0yKjxlxUfpdLIhBwgUxA.gif
[custom_widget.gif]: https://vhs.charm.sh/vhs-216pwM49VNpd66jGKXW66h.gif
[demo.gif]: https://vhs.charm.sh/vhs-6xQ9Z8WBH3YPXyEdE0BKEq.gif
[gauge.gif]: https://vhs.charm.sh/vhs-3CcCQ6yFlw0Xz5een5up3C.gif
[hello_world.gif]: https://vhs.charm.sh/vhs-5rnQv0HMJzSV2aIADDbA0b.gif
[inline.gif]: https://vhs.charm.sh/vhs-2nNMIZ3gp84Akf3wd7lKQK.gif
[layout.gif]: https://vhs.charm.sh/vhs-27Ama8v8HtB1dmMBabT86v.gif
[list.gif]: https://vhs.charm.sh/vhs-3u1sL2KG7mTPtCN6Rrbfzq.gif
[modifiers.gif]: https://vhs.charm.sh/vhs-4W9MyKaRzC4Q4YSBzhnkti.gif
[panic.gif]: https://vhs.charm.sh/vhs-1iwBb1mttYAeN8BS8AlE7A.gif
[paragraph.gif]: https://vhs.charm.sh/vhs-2dCG3AJ3thIgtn446NIts8.gif
[popup.gif]: https://vhs.charm.sh/vhs-7LBrgNore6e71V0tPzq8WX.gif
[scrollbar.gif]: https://vhs.charm.sh/vhs-5ow9scHcnDKwVB0IzFH9JD.gif
[sparkline.gif]: https://vhs.charm.sh/vhs-1DABKSnfu1Qg7i1t68UZ4C.gif
[table.gif]: https://vhs.charm.sh/vhs-287MZTovoqTc7VZyLpNieQ.gif
[tabs.gif]: https://vhs.charm.sh/vhs-2KqXTLF1hxi1xokPOJ9hlC.gif
[user_input.gif]: https://vhs.charm.sh/vhs-1WJfxWDKUsOzGp2prUhIvT.gif
