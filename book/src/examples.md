# Examples

The demo shown in the gif above is available on all available backends.

```shell
# crossterm
cargo run --example demo
# termion
cargo run --example demo --no-default-features --features=termion
# termwiz
cargo run --example demo --no-default-features --features=termwiz
```

The UI code for this is in [examples/demo/ui.rs](./examples/demo/ui.rs) while the application state
is in [examples/demo/app.rs](./examples/demo/app.rs).

If the user interface contains glyphs that are not displayed correctly by your terminal, you may
want to run the demo without those symbols:

```shell
cargo run --example demo --release -- --tick-rate 200 --enhanced-graphics false
```

More examples are available in the [examples](./examples/) folder.
