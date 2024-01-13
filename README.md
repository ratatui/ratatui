# Ratatui Macros

[![Crates.io](https://img.shields.io/crates/v/ratatui-macros)](https://crates.io/crates/ratatui-macros)
[![Docs.rs](https://docs.rs/ratatui-macros/badge.svg)](https://docs.rs/ratatui-macros)
[![Build Status](https://github.com/kdheepak/ratatui-macros/actions/workflows/ci.yml/badge.svg)](https://github.com/kdheepak/ratatui-macros/actions)
[![License](https://img.shields.io/crates/l/ratatui-macros)](https://crates.io/crates/ratatui-macros#license)
[![Downloads](https://img.shields.io/crates/d/ratatui-macros)](https://crates.io/crates/ratatui-macros)

`ratatui-macros` is a Rust crate that provides easy-to-use macros for simplifying boilerplate
associated with creating UI using [Ratatui](https://github.com/ratatui-org/ratatui).

This is an experimental playground for us to explore macros that would be useful to have in Ratatui proper.

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
use ratatui_macros::{
    constraint,
    constraints,
    horizontal,
    vertical,
};
```

### Layout

If you are new to Ratatui, check out <https://ratatui.rs/concepts/layout/> before proceeding.

Use the `constraints!` macro to define layout constraints:

```rust
use ratatui::prelude::*;
use ratatui_macros::constraints;

assert_eq!(
    constraints![==50, ==30%, >=3, <=1, ==1/2],
    [
        Constraint::Length(50),
        Constraint::Percentage(30),
        Constraint::Min(3),
        Constraint::Max(1),
        Constraint::Ratio(1, 2),
    ]
)
```

```rust
use ratatui::prelude::*;
use ratatui_macros::constraints;

assert_eq!(
    constraints![==1/4; 4],
    [
        Constraint::Ratio(1, 4),
        Constraint::Ratio(1, 4),
        Constraint::Ratio(1, 4),
        Constraint::Ratio(1, 4),
    ]
)
```

Use the `constraint!` macro to define individual constraints:

```rust
use ratatui::prelude::*;
use ratatui_macros::constraint;

assert_eq!(
    constraint!(==50),
    Constraint::Length(50),
)
```

Create vertical and horizontal layouts using the `vertical!` and `horizontal!` macros:

```rust
use ratatui::prelude::*;
use ratatui_macros::{vertical, horizontal};

let area = Rect { x: 0, y: 0, width: 10, height: 10 };

let [main, bottom] = vertical![==100%, >=3]
    .split(area)
    .to_vec()
    .try_into()
    .unwrap();

assert_eq!(bottom.y, 7);
assert_eq!(bottom.height, 3);

let [left, main, right] = horizontal![>=3, ==100%, >=3]
    .split(area)
    .to_vec()
    .try_into()
    .unwrap();

assert_eq!(left.width, 3);
assert_eq!(right.width, 3);
```

## Contributing

Contributions to `ratatui-macros` are welcome! Whether it's submitting a bug report, a feature
request, or a pull request, all forms of contributions are valued and appreciated.
