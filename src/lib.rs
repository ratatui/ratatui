#![forbid(unsafe_code)]

//! [ratatui](https://github.com/ratatui-org/ratatui) is a library used to build rich terminal user
//! interfaces (TUIs) and dashboards.
//!
//! ![Demo](https://vhs.charm.sh/vhs-tF0QbuPbtHgUeG0sTVgFr.gif)
//!
//! # Get started
//!
//! ## Add `ratatui` as a dependency using `crossterm`
//!
//! By default, `ratatui` uses [Crossterm] via the [`CrosstermBackend`] as this is supported on most
//! platforms. This means that you can start using it right away by adding the `ratatui` and
//! `crossterm` crates as dependencies to your project:
//!
//! ```bash
//! cargo add ratatui crossterm
//! ```
//!
//! <details>
//! <summary>Cargo.toml</summary>
//!
//! ```toml
//! [dependencies]
//! crossterm = "0.27.0"
//! ratatui = "0.22.0"
//! ```
//! </details>
//!
//! <details>
//! <summary>Other backends (Termion, Termwiz)</summary>
//!
//! ## Termion
//! To use [Termion] via the [`TermionBackend`]:
//!
//! ```bash
//! cargo add ratatui --no-default-features --features termion
//! cargo add termion
//! ````
//! <details>
//! <summary>Cargo.toml</summary>
//!
//! ```toml
//! [dependencies]
//! termion = "2.0.1"
//! ratatui = { version = "0.22.0", default-features = false, features = ['termion'] }
//! ```
//! </details>
//!
//! ## Termwiz
//!
//! To use [Termwiz] via the [`TermwizBackend`], change your dependencies to the following:
//!
//! ```bash
//! cargo add ratatui --no-default-features --features termwiz
//! cargo add termwiz
//! ```
//! <details>
//! <summary>Cargo.toml</summary>
//!
//! ```toml
//! [dependencies]
//! termwiz = "0.20.0"
//! ratatui = { version = "0.22.0", default-features = false, features = ['termwiz'] }
//! ```
//! </details>
//! </details>
//!
//! ## Creating a `Terminal`
//!
//! Every application using `ratatui` starts by instantiating a [`Backend`] and a [`Terminal`]. The
//! `Terminal` is a light abstraction over available backends that provides basic functionalities
//! such as clearing the screen, hiding the cursor, etc.
//!
//! ```rust,no_run
//! use ratatui::TerminalBuilder;
//!
//! fn main() -> std::io::Result<()> {
//!     let mut terminal = TerminalBuilder::crossterm_on_stdout().build()?;
//!     Ok(())
//! }
//! ```
//!
//! For more information on how to configure the backend for the Terminal, please refer to the
//! [`backend`] and [`terminal`] module documentation. Most of the examples are written using the
//! [`CrosstermBackend`], but the [Demo Example] has code to show how to use the other backends.
//!
//! ## Building a User Interface (UI)
//!
//! Every component of your interface will be implementing the `Widget` trait. The library comes
//! with a predefined set of widgets that should meet most of your use cases. You are also free to
//! implement your own.
//!
//! Each widget follows a builder pattern API providing a default configuration along with methods
//! to customize them. The widget is then rendered using [`Frame::render_widget`] which takes your
//! widget instance and an area to draw to.
//!
//! The following example renders a block of the size of the terminal:
//!
//! ```rust,no_run
//! use std::{io, thread, time::Duration};
//! use crossterm::event;
//! use ratatui::{
//!     prelude::*, // This is a prelude that re-exports most of the types you need
//!     widgets::{Block, Borders},
//! };
//!
//! fn main() -> io::Result<()> {
//!     let mut terminal = TerminalBuilder::crossterm_on_stdout().build()?;
//!
//!     terminal.draw(|frame| {
//!         let size = frame.size();
//!         let block = Block::default()
//!             .title("Block")
//!             .borders(Borders::ALL);
//!         frame.render_widget(block, size);
//!     })?;
//!
//!     // Start a thread to discard any input events. Without handling events, the
//!     // stdin buffer will fill up, and be read into the shell when the program exits.
//!     thread::spawn(|| loop {
//!         event::read();
//!     });
//!
//!     thread::sleep(Duration::from_millis(5000));
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Layout
//!
//! The library comes with a basic yet useful layout management object called [`Layout`]. As you may
//! see below and in the examples, the library makes heavy use of the builder pattern to provide
//! full customization and `Layout` is no exception:
//!
//! ```rust,no_run
//! use ratatui::prelude::*;
//! use ratatui::widgets::{Block, Borders};
//!
//! fn ui<B: Backend>(frame: &mut Frame<B>) {
//!    let areas = Layout::default()
//!         .direction(Direction::Vertical)
//!         .margin(1)
//!         .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
//!         .split(frame.size());
//!     frame.render_widget(
//!         Block::new().title("Block 1").borders(Borders::all()),
//!         areas[0],
//!     );
//!     frame.render_widget(
//!         Block::new().title("Block 2").borders(Borders::all()),
//!         areas[1],
//!    );
//! }
//! ```
//!
//! This let you describe responsive terminal UI by nesting layouts. You should note that by default
//! the computed layout tries to fill the available space completely. So if for any reason you might
//! need a blank space somewhere, try to pass an additional constraint and don't use the
//! corresponding area.
//!
//! # Features
//!
//! The crate provides a set of optional features that can be enabled to get additional
//! functionalities. Generally an application will only use one backend, so you should only enable
//! one of the following features:
//!
//! * `crossterm` - enables the [`CrosstermBackend`] backend and adds a dependency on the
//! [crossterm](https://crates.io/crates/crossterm) crate. Enabled by default.
//! * `termion` - enables the [`TermionBackend`] backend and adds a dependency on the
//! [termion](https://crates.io/crates/termion) crate.
//! * `termwiz` - enables the [`TermwizBackend`] backend and adds a dependency on the
//! [termwiz](https://crates.io/crates/termwiz) crate.
//!
//! The following features are available for all backends:
//!
//! * `serde` - enables serialization and deserialization of style and color types. This is useful
//! if you want to save themes to a file.
//! * `macros` - enables the [`border!`] macro.
//! * `all-widgets` - enables all widgets.
//!
//! Widgets which add dependencies are gated behind feature flags to prevent unused transitive
//! dependencies. The available features are:
//!
//! * `widget-calendar` - enables [`widgets::calendar`] and adds a dependency on the [time
//! crate](https://crates.io/crates/time).
//!
//! [`Layout`]: layout::Layout
//! [`backend`]: backend
//! [`CrosstermBackend`]: backend::CrosstermBackend
//! [`TermionBackend`]: backend::TermionBackend
//! [`TermwizBackend`]: backend::TermwizBackend
//! [Crossterm]: https://crates.io/crates/crossterm
//! [Termion]: https://crates.io/crates/termion
//! [Termwiz]: https://crates.io/crates/termwiz
//! [Demo Example]: https://github.com/ratatui-org/ratatui/tree/main/examples/demo

// show the feature flags in the generated documentation
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]

pub mod backend;
pub mod buffer;
pub mod layout;
pub mod style;
pub mod symbols;
pub mod terminal;
mod terminal_builder;
pub mod text;
pub mod widgets;

pub mod prelude;

#[doc(inline)]
pub use self::terminal::{CompletedFrame, Frame, Terminal, TerminalOptions, Viewport};
#[doc(inline)]
pub use self::terminal_builder::{AlternateScreenMode, MouseCapture, RawMode, TerminalBuilder};
