#![deny(missing_docs)]
//! Provides the [`Terminal`], [`Frame`] and related types.
//!
//! The [`Terminal`] is the main interface of this library. It is responsible for drawing and
//! maintaining the state of the different widgets that compose your application.
//!
//! The [`Frame`] is a consistent view into the terminal state for rendering. It is obtained via
//! the closure argument of [`Terminal::draw`]. It is used to render widgets to the terminal and
//! control the cursor position.
//!
//! # Example
//!
//! ```rust,no_run
//! use std::io::stdout;
//!
//! use ratatui::{prelude::*, widgets::Paragraph};
//!
//! let backend = CrosstermBackend::new(stdout());
//! let mut terminal = Terminal::new(backend)?;
//! terminal.draw(|frame| {
//!     let area = frame.area();
//!     frame.render_widget(Paragraph::new("Hello world!"), area);
//! })?;
//! # std::io::Result::Ok(())
//! ```
//!
//! [Crossterm]: https://crates.io/crates/crossterm
//! [Termion]: https://crates.io/crates/termion
//! [Termwiz]: https://crates.io/crates/termwiz
//! [`backend`]: crate::backend
//! [`Backend`]: crate::backend::Backend
//! [`Buffer`]: crate::buffer::Buffer

mod frame;
#[cfg(feature = "crossterm")]
mod init;
mod terminal;
mod viewport;

pub use frame::{CompletedFrame, Frame};
#[cfg(feature = "crossterm")]
pub use init::{
    init, init_with_options, restore, try_init, try_init_with_options, try_restore, DefaultTerminal,
};
pub use terminal::{Options as TerminalOptions, Terminal};
pub use viewport::Viewport;
