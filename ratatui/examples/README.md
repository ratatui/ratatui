# Examples

This folder might use unreleased code. View the examples for the latest release instead.

> [!WARNING]
>
> There may be backwards incompatible changes in these examples, as they are designed to compile
> against the `main` branch.
>
> There are a few workaround for this problem:
>
> - View the examples as they were when the latest version was release by selecting the tag that
>   matches that version. E.g. <https://github.com/ratatui/ratatui/tree/v0.26.1/examples>.
> - If you're viewing this file on GitHub, there is a combo box at the top of this page  which
>   allows you to select any previous tagged version.
> - To view the code locally, checkout the tag. E.g. `git switch --detach v0.26.1`.
> - Use the latest [alpha version of Ratatui] in your app. These are released weekly on Saturdays.
> - Compile your code against the main branch either locally by adding e.g. `path = "../ratatui"` to
>   the dependency, or remotely by adding `git = "https://github.com/ratatui/ratatui"`
>
> For a list of unreleased breaking changes, see [BREAKING-CHANGES.md].
>
> We don't keep the CHANGELOG updated with unreleased changes, check the git commit history or run
> `git-cliff -u` against a cloned version of this repository.

## Design choices

The examples contain some opinionated choices in order to make it easier for newer rustaceans to
easily be productive in creating applications:

- Each example has an App struct, with methods that implement a main loop, handle events and drawing
  the UI.
- We use color_eyre for handling errors and panics. See [How to use color-eyre with Ratatui] on the
  website for more information about this.
- Common code is not extracted into a separate file. This makes each example self-contained and easy
  to read as a whole.

Not every example has been updated with all these points in mind yet, however over time they will
be. None of the above choices are strictly necessary for Ratatui apps, but these choices make
examples easier to run, maintain and explain. These choices are designed to help newer users fall
into the pit of success when incorporating example code into their own apps. We may also eventually
move some of these design choices into the core of Ratatui to simplify apps.

[How to use color-eyre with Ratatui]: https://ratatui.rs/how-to/develop-apps/color_eyre/

## Demo2

This is the demo example from the main README and crate page. Source: [demo2](./demo2/).

```shell
cargo run --example=demo2 --features="crossterm widget-calendar"
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

## Barchart (Grouped)

Demonstrates the [`BarChart`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.BarChart.html)
widget with groups. Source: [barchart-grouped.rs](./barchart-grouped.rs).

```shell
cargo run --example=barchart-grouped --features=crossterm
```

![Barchart Grouped][barchart-grouped.gif]

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

Note: VHs renders full screen animations poorly, so this is a screen capture rather than the output
of the VHS tape.

<https://github.com/ratatui/ratatui/assets/381361/485e775a-e0b5-4133-899b-1e8aeb56e774>

## Constraint Explorer

Demonstrates the behaviour of each
[`Constraint`](https://docs.rs/ratatui/latest/ratatui/layout/enum.Constraint.html) option with
respect to each other across different `Flex` modes.

```shell
cargo run --example=constraint-explorer --features=crossterm
```

![Constraint Explorer][constraint-explorer.gif]

## Constraints

Demonstrates how to use
[`Constraint`](https://docs.rs/ratatui/latest/ratatui/layout/enum.Constraint.html) options for
defining layout element sizes.

![Constraints][constraints.gif]

```shell
cargo run --example=constraints --features=crossterm
```

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

## Flex

Demonstrates the different [`Flex`](https://docs.rs/ratatui/latest/ratatui/layout/enum.Flex.html)
modes for controlling layout space distribution.

```shell
cargo run --example=flex --features=crossterm
```

![Flex][flex.gif]

## Line Gauge

Demonstrates the [`Line
Gauge`](https://docs.rs/ratatui/latest/ratatui/widgets/struct.LineGauge.html) widget. Source:
[line_gauge.rs](./line_gauge.rs).

```shell
cargo run --example=line_gauge --features=crossterm
```

![LineGauge][line_gauge.gif]

## Hyperlink

Demonstrates how to use OSC 8 to create hyperlinks in the terminal.

```shell
cargo run --example=hyperlink --features="crossterm unstable-widget-ref"
```

![Hyperlink][hyperlink.gif]

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

## Minimal

Demonstrates how to create a minimal `Hello World!` program.

```shell
cargo run --example=minimal --features=crossterm
```

![Minimal][minimal.gif]

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

## Ratatui-logo

A fun example of using half blocks to render graphics Source:
[ratatui-logo.rs](./ratatui-logo.rs).

>
```shell
cargo run --example=ratatui-logo --features=crossterm
```

![Ratatui Logo][ratatui-logo.gif]

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

## Tracing

Demonstrates how to use the [tracing crate](https://crates.io/crates/tracing) for logging. Creates
a file named `tracing.log` in the current directory.

```shell
cargo run --example=tracing --features=crossterm
```

![Tracing][tracing.gif]

## User Input

Demonstrates one approach to accepting user input. Source [user_input.rs](./user_input.rs).

> [!NOTE]
> Consider using [`tui-textarea`](https://crates.io/crates/tui-textarea) or
> [`tui-input`](https://crates.io/crates/tui-input) crates for more functional text entry UIs.

```shell
cargo run --example=user_input --features=crossterm
```

![User Input][user_input.gif]

## How to update these examples

These gifs were created using [VHS](https://github.com/charmbracelet/vhs). Each example has a
corresponding `.tape` file that holds instructions for how to generate the images. Note that the
images themselves are stored in a separate `images` git branch to avoid bloating the main
repository.

<!--

Links to images to make them easier to update in bulk. Use the following script to update and upload
the examples to the images branch. (Requires push access to the branch).

```shell
examples/vhs/generate.bash
```
-->

[barchart.gif]: https://github.com/ratatui/ratatui/blob/images/examples/barchart.gif?raw=true
[barchart-grouped.gif]: https://github.com/ratatui/ratatui/blob/images/examples/barchart-grouped.gif?raw=true
[block.gif]: https://github.com/ratatui/ratatui/blob/images/examples/block.gif?raw=true
[calendar.gif]: https://github.com/ratatui/ratatui/blob/images/examples/calendar.gif?raw=true
[canvas.gif]: https://github.com/ratatui/ratatui/blob/images/examples/canvas.gif?raw=true
[chart.gif]: https://github.com/ratatui/ratatui/blob/images/examples/chart.gif?raw=true
[colors.gif]: https://github.com/ratatui/ratatui/blob/images/examples/colors.gif?raw=true
[constraint-explorer.gif]: https://github.com/ratatui/ratatui/blob/images/examples/constraint-explorer.gif?raw=true
[constraints.gif]: https://github.com/ratatui/ratatui/blob/images/examples/constraints.gif?raw=true
[custom_widget.gif]: https://github.com/ratatui/ratatui/blob/images/examples/custom_widget.gif?raw=true
[demo.gif]: https://github.com/ratatui/ratatui/blob/images/examples/demo.gif?raw=true
[demo2.gif]: https://github.com/ratatui/ratatui/blob/images/examples/demo2.gif?raw=true
[flex.gif]: https://github.com/ratatui/ratatui/blob/images/examples/flex.gif?raw=true
[gauge.gif]: https://github.com/ratatui/ratatui/blob/images/examples/gauge.gif?raw=true
[hello_world.gif]: https://github.com/ratatui/ratatui/blob/images/examples/hello_world.gif?raw=true
[hyperlink.gif]: https://github.com/ratatui/ratatui/blob/images/examples/hyperlink.gif?raw=true
[inline.gif]: https://github.com/ratatui/ratatui/blob/images/examples/inline.gif?raw=true
[layout.gif]: https://github.com/ratatui/ratatui/blob/images/examples/layout.gif?raw=true
[list.gif]: https://github.com/ratatui/ratatui/blob/images/examples/list.gif?raw=true
[line_gauge.gif]: https://github.com/ratatui/ratatui/blob/images/examples/line_gauge.gif?raw=true
[minimal.gif]: https://github.com/ratatui/ratatui/blob/images/examples/minimal.gif?raw=true
[modifiers.gif]: https://github.com/ratatui/ratatui/blob/images/examples/modifiers.gif?raw=true
[panic.gif]: https://github.com/ratatui/ratatui/blob/images/examples/panic.gif?raw=true
[paragraph.gif]: https://github.com/ratatui/ratatui/blob/images/examples/paragraph.gif?raw=true
[popup.gif]: https://github.com/ratatui/ratatui/blob/images/examples/popup.gif?raw=true
[ratatui-logo.gif]: https://github.com/ratatui/ratatui/blob/images/examples/ratatui-logo.gif?raw=true
[scrollbar.gif]: https://github.com/ratatui/ratatui/blob/images/examples/scrollbar.gif?raw=true
[sparkline.gif]: https://github.com/ratatui/ratatui/blob/images/examples/sparkline.gif?raw=true
[table.gif]:  https://vhs.charm.sh/vhs-6njXBytDf0rwPufUtmSSpI.gif
[tabs.gif]: https://github.com/ratatui/ratatui/blob/images/examples/tabs.gif?raw=true
[tracing.gif]: https://github.com/ratatui/ratatui/blob/images/examples/tracing.gif?raw=true
[user_input.gif]: https://github.com/ratatui/ratatui/blob/images/examples/user_input.gif?raw=true

[alpha version of Ratatui]: https://crates.io/crates/ratatui/versions
[BREAKING-CHANGES.md]: https://github.com/ratatui/ratatui/blob/main/BREAKING-CHANGES.md
