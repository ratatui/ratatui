# ratatui-core

[![Crates.io](https://img.shields.io/crates/v/ratatui-core)](https://crates.io/crates/ratatui-core)
[![Documentation](https://docs.rs/ratatui-core/badge.svg)](https://docs.rs/ratatui-core)
[![License](https://img.shields.io/crates/l/ratatui-core)](../LICENSE)

## Overview

**ratatui-core** is the core library of the [ratatui](https://github.com/ratatui/ratatui) project,
providing the essential building blocks for creating rich terminal user interfaces in Rust.

### Why ratatui-core?

The `ratatui-core` crate is split from the main [`ratatui`](https://crates.io/crates/ratatui) crate
to offer better stability for widget library authors. Widget libraries should generally depend on
`ratatui-core`, benefiting from a stable API and reducing the need for frequent updates.
Applications, on the other hand, should depend on the main `ratatui` crate, which includes built-in
widgets and additional features.

## Installation

Add `ratatui-core` to your `Cargo.toml`:

```shell
cargo add ratatui-core
```

## Contributing

We welcome contributions from the community! Please see our [CONTRIBUTING](../CONTRIBUTING.md) guide for more details on how to get involved.

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.
