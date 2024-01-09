# Ratatui Macros

[![Crates.io](https://img.shields.io/crates/v/ratatui-macros)](https://crates.io/crates/ratatui-macros)
[![Docs.rs](https://docs.rs/ratatui-macros/badge.svg)](https://docs.rs/ratatui-macros)
[![Build Status](https://github.com/kdheepak/ratatui-macros/actions/workflows/ci.yml/badge.svg)](https://github.com/kdheepak/ratatui-macros/actions)
[![License](https://img.shields.io/crates/l/ratatui-macros)](https://crates.io/crates/ratatui-macros#license)
[![Downloads](https://img.shields.io/crates/d/ratatui-macros)](https://crates.io/crates/ratatui-macros)

`ratatui-macros` is a Rust crate that provides easy-to-use macros for simplifying boilerplate
associated with creating UI using ratatui.

## Features

- Constraint-based Layouts: Easily define layout constraints such as fixed, percentage, minimum, and
  maximum sizes, as well as ratios.
- Directional Layouts: Specify layouts as either horizontal or vertical with simple macro commands.

## Getting Started

To use `ratatui-macros` in your Rust project, add it as a dependency in your `Cargo.toml`:

```shell
cargo add ratatui-macros
```

Then, import the macros in your Rust file:

```rust
use ratatui_macros::{constraints, vertical, horizontal};
```

Use the `constraints!` macro to define layout constraints:

```rust
use ratatui_macros::constraints;
assert_eq!(
    constraints![==50, ==30%, >=3, <=1, ==1/2],
    [
        ratatui::prelude::Constraint::Length(50),
        ratatui::prelude::Constraint::Percentage(30),
        ratatui::prelude::Constraint::Min(3),
        ratatui::prelude::Constraint::Max(1),
        ratatui::prelude::Constraint::Ratio(1, 2),
    ]
)
```

```rust
use ratatui_macros::constraints;
assert_eq!(
    constraints![==1/4; 4],
    [
        ratatui::prelude::Constraint::Ratio(1, 4),
        ratatui::prelude::Constraint::Ratio(1, 4),
        ratatui::prelude::Constraint::Ratio(1, 4),
        ratatui::prelude::Constraint::Ratio(1, 4),
    ]
)
```

Create vertical and horizontal layouts using the `vertical!` and `horizontal!` macros:

```rust
use ratatui_macros::{vertical, horizontal};
use ratatui::prelude::Rect;
let area = Rect { x: 0, y: 0, width: 10, height: 10 };

let vertical_layout = vertical![==100%, >=3];
let [main, bottom] = vertical_layout.split(area).to_vec().try_into().unwrap();

let horizontal_layout = horizontal![==10, ==100%, ==10];
let [left, main, right] = horizontal_layout.split(area).to_vec().try_into().unwrap();
```

## Contributing

Contributions to `ratatui-macros` are welcome! Whether it's submitting a bug report, a feature
request, or a pull request, all forms of contributions are valued and appreciated.
