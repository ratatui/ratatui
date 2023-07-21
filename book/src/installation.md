# Installation

```shell
cargo add ratatui --features all-widgets
```

Or modify your `Cargo.toml`

```toml
[dependencies]
ratatui = { version = "0.22.0", features = ["all-widgets"]}
```

Ratatui is mostly backwards compatible with `tui-rs`. To migrate an existing project, it may be
easier to rename the ratatui dependency to `tui` rather than updating every usage of the crate.
E.g.:

```toml
[dependencies]
tui = { package = "ratatuiw, version = "0.22.0", features = ["all-widgets"]}
