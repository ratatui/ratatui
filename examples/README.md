# Examples

These gifs were created using [Charm VHS](https://github.com/charmbracelet/vhs).

VHS has a problem rendering some background color transitions, which shows up in several examples
below. See <https://github.com/charmbracelet/vhs/issues/344> for more info. These problems don't
occur in a terminal.

## Demo ([demo.rs](./demo/))

This is the demo example from the main README. It is available for each of the backends.

```shell
cargo run --example=demo --features=crossterm
cargo run --example=demo --no-default-features --features=termion
cargo run --example=demo --no-default-features --features=termwiz
```

![Demo][demo.gif]

## Barchart ([barchart.rs](./barchart.rs)

```shell
cargo run --example=barchart --features=crossterm
```

![Barchart][barchart.gif]

## Block ([block.rs](./block.rs))

```shell
cargo run --example=block --features=crossterm
```

![Block][block.gif]

## Calendar ([calendar.rs](./calendar.rs))

```shell
cargo run --example=calendar --features=crossterm widget-calendar
```

![Calendar][calendar.gif]

## Canvas ([canvas.rs](./canvas.rs))

```shell
cargo run --example=canvas --features=crossterm
```

![Canvas][canvas.gif]

## Chart ([chart.rs](./chart.rs))

```shell
cargo run --example=chart --features=crossterm
```

![Chart][chart.gif]

## Colors ([colors.rs](./colors.rs))

```shell
cargo run --example=colors --features=crossterm
```

![Colors][colors.gif]

## Custom Widget ([custom_widget.rs](./custom_widget.rs))

```shell
cargo run --example=custom_widget --features=crossterm
```

This is not a particularly exciting example visually, but it demonstrates how to implement your own widget.

![Custom Widget][custom_widget.gif]

## Gauge ([gauge.rs](./gauge.rs))

Please note: the background renders poorly when we generate this example using VHS.
This problem doesn't generally happen during normal rendering in a terminal.
See <https://github.com/charmbracelet/vhs/issues/344> for more details

```shell
cargo run --example=gauge --features=crossterm
```

![Gauge][gauge.gif]

## Hello World ([hello_world.rs](./hello_world.rs))

```shell
cargo run --example=hello_world --features=crossterm
```

This is a pretty boring example, but it contains some good comments of documentation on some of the
standard approaches to writing tui apps.

![Hello World][hello_world.gif]

## Inline ([inline.rs](./inline.rs))

```shell
cargo run --example=inline --features=crossterm
```

![Inline][inline.gif]

## Layout ([layout.rs](./layout.rs))

```shell
cargo run --example=layout --features=crossterm
```

![Layout][layout.gif]

## List ([list.rs](./list.rs))

```shell
cargo run --example=list --features=crossterm
```

![List][list.gif]

## Modifiers ([modifiers.rs](./modifiers.rs))

```shell
cargo run --example=modifiers --features=crossterm
```

![Modifiers][modifiers.gif]

## Panic ([panic.rs](./panic.rs))

```shell
cargo run --example=panic --features=crossterm
```

![Panic][panic.gif]

## Paragraph ([paragraph.rs](./paragraph.rs))

```shell
cargo run --example=paragraph --features=crossterm
```

![Paragraph][paragraph.gif]

## Popup ([popup.rs](./popup.rs))

```shell
cargo run --example=popup --features=crossterm
```

Please note: the background renders poorly when we generate this example using VHS.
This problem doesn't generally happen during normal rendering in a terminal.
See <https://github.com/charmbracelet/vhs/issues/344> for more details

![Popup][popup.gif]

## Scrollbar ([scrollbar.rs](./scrollbar.rs))

```shell
cargo run --example=scrollbar --features=crossterm
```

![Scrollbar][scrollbar.gif]

## Sparkline ([sparkline.rs](./sparkline.rs))

```shell
cargo run --example=sparkline --features=crossterm
```

![Sparkline][sparkline.gif]

## Table ([table.rs](./table.rs))

```shell
cargo run --example=table --features=crossterm
```

![Table][table.gif]

## Tabs ([tabs.rs](./tabs.rs))

```shell
cargo run --example=tabs --features=crossterm
```

![Tabs][tabs.gif]

## User Input ([user_input.rs](./user_input.rs))

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
[barchart.gif]: https://vhs.charm.sh/vhs-6ioxdeRBVkVpyXcjIEVaJU.gif
[block.gif]: https://vhs.charm.sh/vhs-1TyeDa5GN7kewhNjKxJ4Br.gif
[calendar.gif]: https://vhs.charm.sh/vhs-1dBcpMSSP80WkBgm4lBhNo.gif
[canvas.gif]: https://vhs.charm.sh/vhs-4zeWEPF6bLEFSHuJrvaHlN.gif
[chart.gif]: https://vhs.charm.sh/vhs-zRzsE2AwRixQhcWMTAeF1.gif
[colors.gif]: https://vhs.charm.sh/vhs-2ZCqYbTbXAaASncUeWkt1z.gif
[custom_widget.gif]: https://vhs.charm.sh/vhs-32mW1TpkrovTcm79QXmBSu.gif
[demo.gif]: https://vhs.charm.sh/vhs-tF0QbuPbtHgUeG0sTVgFr.gif
[gauge.gif]: https://vhs.charm.sh/vhs-2rvSeP5r4lRkGTzNCKpm9a.gif
[hello_world.gif]: https://vhs.charm.sh/vhs-3CKUwxFuQi8oKQMS5zkPfQ.gif
[inline.gif]: https://vhs.charm.sh/vhs-miRl1mosKFoJV7LjjvF4T.gif
[layout.gif]: https://vhs.charm.sh/vhs-1ZNoNLNlLtkJXpgg9nCV5e.gif
[list.gif]: https://vhs.charm.sh/vhs-4goo9reeUM9r0nYb54R7SP.gif
[modifiers.gif]: https://vhs.charm.sh/vhs-2ovGBz5l3tfRGdZ7FCw0am.gif
[panic.gif]: https://vhs.charm.sh/vhs-HrvKCHV4yeN69fb1EadTH.gif
[paragraph.gif]: https://vhs.charm.sh/vhs-2qIPDi79DUmtmeNDEeHVEF.gif
[popup.gif]: https://vhs.charm.sh/vhs-2QnC682AUeNYNXcjNlKTyp.gif
[scrollbar.gif]: https://vhs.charm.sh/vhs-2p13MMFreW7Gwt1xIonIWu.gif
[sparkline.gif]: https://vhs.charm.sh/vhs-4t59Vxw5Za33Rtvt9QrftA.gif
[table.gif]: https://vhs.charm.sh/vhs-6IrGHgT385DqA6xnwGF9oD.gif
[tabs.gif]: https://vhs.charm.sh/vhs-61WkbfhyDk0kbkjncErdHT.gif
[user_input.gif]: https://vhs.charm.sh/vhs-4fxUgkpEWcVyBRXuyYKODY.gif
