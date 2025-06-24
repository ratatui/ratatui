#![no_std]
//! `ratatui-macros` is a Rust crate that provides easy-to-use macros for simplifying boilerplate
//! associated with creating UI using [Ratatui].
//!
//! This is an experimental playground for us to explore macros that would be useful to have in
//! Ratatui proper.
//!
//! # Features
//!
//! - [Text macros](#text-macros) for easily defining styled [`Text`]s, [`Line`]s, and [`Span`]s.
//! - [Layout macros](#layout-macros) for defining [`Layout`]s with [`Constraint`]s and directions.
//! - [Table macros](#table-macros) for creating [`Row`]s and [`Cell`]s.
//!
//! # Getting Started
//!
//! Add `ratatui-macros` as a dependency in your `Cargo.toml`:
//!
//! ```shell
//! cargo add ratatui-macros
//! ```
//!
//! Then, import the macros in your Rust file:
//!
//! ```rust
//! use ratatui_macros::{constraint, constraints, horizontal, line, row, span, text, vertical};
//! ```
//!
//! # Text Macros
//!
//! The [`span!`] macro creates raw or styled [`Span`]s.
//!
//! ```rust
//! # use ratatui_core::style::{Color, Modifier, Style, Stylize};
//! # use ratatui_macros::span;
//! let name = "world!";
//! let raw_greeting = span!("hello {name}");
//! let styled_greeting = span!(Style::new().green(); "hello {name}");
//! let colored_greeting = span!(Color::Green; "hello {name}");
//! let modified_greeting = span!(Modifier::BOLD; "hello {name}");
//! ```
//!
//! The [`line!`] macro creates a [`Line`] that contains a sequence of [`Span`]s. It is similar to
//! the [`vec!`] macro. Each element is converted into a [`Span`] using [`Into::into`].
//!
//! ```rust
//! # use ratatui_core::style::{Color, Stylize};
//! # use ratatui_macros::{line, span};
//! let name = "world!";
//! let line = line!["hello", format!("{name}")];
//! let line = line!["hello ", span!(Color::Green; "{name}")];
//! let line = line!["Name: ".bold(), "Remy".italic()];
//! let line = line!["bye"; 2];
//! ```
//!
//! The [`text!`] macro creates a [`Text`] that contains a sequence of [`Line`]. It is similar to
//! the [`vec!`] macro. Each element is converted to a [`Line`] using [`Into::into`].
//!
//! ```rust
//! # use ratatui_core::style::{Modifier, Stylize};
//! # use ratatui_macros::{span, line, text};
//! let name = "world!";
//! let text = text!["hello", format!("{name}")];
//! let text = text!["bye"; 2];
//! let name = "Bye!!!";
//! let text = text![line!["hello", "world".bold()], span!(Modifier::BOLD; "{name}")];
//! ```
//!
//! # Layout Macros
//!
//! If you are new to Ratatui, check out the [Layout concepts] article on the Ratatui website before
//! proceeding.
//!
//! The [`constraints!`] macro defines an array of [`Constraint`]s:
//!
//! ```rust
//! # use ratatui_core::layout::Layout;
//! # use ratatui_macros::constraints;
//! let layout = Layout::horizontal(constraints![==50, ==30%, >=3, <=1, ==1/2, *=1]);
//! ```
//!
//! The [`constraint!`] macro defines individual [`Constraint`]s:
//!
//! ```rust
//! # use ratatui_core::layout::Layout;
//! # use ratatui_macros::constraint;
//! let layout = Layout::horizontal([constraint!(==50)]);
//! ```
//!
//! The [`vertical!`] and [`horizontal!`] macros are a shortcut to defining a [`Layout`]:
//!
//! ```rust
//! # use ratatui_core::layout::Rect;
//! # use ratatui_macros::{vertical, horizontal};
//! # let area = Rect { x: 0, y: 0, width: 10, height: 10 };
//! let [top, main, bottom] = vertical![==1, *=1, >=3].areas(area);
//! let [left, main, right] = horizontal![>=20, *=1, >=20].areas(main);
//! ```
//!
//! # Table Macros
//!
//! The [`row!`] macro creates a [`Row`] for a [`Table`] that contains a sequence of [`Cell`]s. It
//! is similar to the [`vec!`] macro.
//!
//! ```rust
//! # use ratatui_core::style::{Modifier, Stylize};
//! # use ratatui_macros::{constraints, line, row, span, text};
//! # use ratatui_widgets::table::Table;
//! let rows = [
//!     row!["hello", "world"],
//!     row!["goodbye", "world"],
//!     row![
//!         text!["line 1", line!["Line", "2".bold()]],
//!         span!(Modifier::BOLD; "Cell 2"),
//!     ],
//! ];
//! let table = Table::new(rows, constraints![==20, *=1]);
//! ```
//!
//! # Contributing
//!
//! Contributions to `ratatui-macros` are welcome! Whether it's submitting a bug report, a feature
//! request, or a pull request, all forms of contributions are valued and appreciated.
//!
//! [Crates.io badge]: https://img.shields.io/crates/v/ratatui-macros?logo=rust&style=flat-square
//! [License badge]: https://img.shields.io/crates/l/ratatui-macros
//! [CI Badge]:
//!   https://img.shields.io/github/actions/workflow/status/ratatui/ratatui/ci.yml?logo=github&style=flat-square
//! [Docs.rs badge]: https://img.shields.io/docsrs/ratatui-macros?logo=rust&style=flat-square
//! [Crate Downloads badge]:
//!     https://img.shields.io/crates/d/ratatui-macros?logo=rust&style=flat-square
//! [ratatui_macros crate]: https://crates.io/crates/ratatui-macros
//! [API Docs]: https://docs.rs/ratatui-macros
//! [CI Status]: https://github.com/ratatui/ratatui/actions
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [Layout concepts]: https://ratatui.rs/concepts/layout
//! [`Constraint`]: ratatui_core::layout::Constraint
//! [`Layout`]: ratatui_core::layout::Layout
//! [`Span`]: ratatui_core::text::Span
//! [`Line`]: ratatui_core::text::Line
//! [`Text`]: ratatui_core::text::Text
//! [`Row`]: ratatui_widgets::table::Row
//! [`Cell`]: ratatui_widgets::table::Cell
//! [`Table`]: ratatui_widgets::table::Table

extern crate alloc;

#[doc(hidden)]
pub use alloc::{format, vec};

mod layout;
mod line;
mod row;
mod span;
mod text;

// Re-export the core crate to use the types in macros
pub use ratatui_core;
