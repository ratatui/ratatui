#![warn(missing_docs)]
//! This module provides the backend implementations for different terminal libraries.
//!
//! It defines the [`Backend`] trait which is used to abstract over the specific terminal library
//! being used.
//!
//! Supported terminal backends:
//! - [Crossterm]: enable the `crossterm` feature (enabled by default) and use [`CrosstermBackend`]
//! - [Termion]: enable the `termion` feature and use [`TermionBackend`]
//! - [Termwiz]: enable the `termwiz` feature and use [`TermwizBackend`]
//!
//! Additionally, a [`TestBackend`] is provided for testing purposes.
//!
//! See the [Backend Comparison] section of the [Ratatui Website] for more details on the different
//! backends.
//!
//! Each backend supports a number of features, such as [raw mode](#raw-mode), [alternate
//! screen](#alternate-screen), and [mouse capture](#mouse-capture). These features are generally
//! not enabled by default, and must be enabled by the application before they can be used. See the
//! documentation for each backend for more details.
//!
//! Note: most applications should use the [`Terminal`] struct instead of directly calling methods
//! on the backend.
//!
//! # Example
//!
//! ```rust,no_run
//! use std::io::stdout;
//!
//! use ratatui::prelude::*;
//!
//! let backend = CrosstermBackend::new(stdout());
//! let mut terminal = Terminal::new(backend)?;
//! terminal.clear()?;
//! terminal.draw(|frame| {
//!     // -- snip --
//! })?;
//! # std::io::Result::Ok(())
//! ```
//!
//! See the the [Examples] directory for more examples.
//!
//! # Raw Mode
//!
//! Raw mode is a mode where the terminal does not perform any processing or handling of the input
//! and output. This means that features such as echoing input characters, line buffering, and
//! special character processing (e.g., CTRL-C for SIGINT) are disabled. This is useful for
//! applications that want to have complete control over the terminal input and output, processing
//! each keystroke themselves.
//!
//! For example, in raw mode, the terminal will not perform line buffering on the input, so the
//! application will receive each key press as it is typed, instead of waiting for the user to
//! press enter. This makes it suitable for real-time applications like text editors,
//! terminal-based games, and more.
//!
//! Each backend handles raw mode differently, so the behavior may vary depending on the backend
//! being used. Be sure to consult the backend's specific documentation for exact details on how it
//! implements raw mode.

//! # Alternate Screen
//!
//! The alternate screen is a separate buffer that some terminals provide, distinct from the main
//! screen. When activated, the terminal will display the alternate screen, hiding the current
//! content of the main screen. Applications can write to this screen as if it were the regular
//! terminal display, but when the application exits, the terminal will switch back to the main
//! screen, and the contents of the alternate screen will be cleared. This is useful for
//! applications like text editors or terminal games that want to use the full terminal window
//! without disrupting the command line or other terminal content.
//!
//! This creates a seamless transition between the application and the regular terminal session, as
//! the content displayed before launching the application will reappear after the application
//! exits.
//!
//! Note that not all terminal emulators support the alternate screen, and even those that do may
//! handle it differently. As a result, the behavior may vary depending on the backend being used.
//! Always consult the specific backend's documentation to understand how it implements the
//! alternate screen.
//!
//! # Mouse Capture
//!
//! Mouse capture is a mode where the terminal captures mouse events such as clicks, scrolls, and
//! movement, and sends them to the application as special sequences or events. This enables the
//! application to handle and respond to mouse actions, providing a more interactive and graphical
//! user experience within the terminal. It's particularly useful for applications like
//! terminal-based games, text editors, or other programs that require more direct interaction from
//! the user.
//!
//! Each backend handles mouse capture differently, with variations in the types of events that can
//! be captured and how they are represented. As such, the behavior may vary depending on the
//! backend being used, and developers should consult the specific backend's documentation to
//! understand how it implements mouse capture.
//!
//! [`TermionBackend`]: termion/struct.TermionBackend.html
//! [`Terminal`]: crate::terminal::Terminal
//! [`TermionBackend`]: termion/struct.TermionBackend.html
//! [Crossterm]: https://crates.io/crates/crossterm
//! [Termion]: https://crates.io/crates/termion
//! [Termwiz]: https://crates.io/crates/termwiz
//! [Examples]: https://github.com/ratatui/ratatui/tree/main/examples/README.md
//! [Backend Comparison]:
//!     https://ratatui.rs/concepts/backends/comparison/
//! [Ratatui Website]: https://ratatui.rs
use std::io;

pub use ratatui_core::backend::{Backend, ClearType, TestBackend, WindowSize};
use ratatui_core::{
    buffer::Cell,
    layout::{Position, Size},
};
#[cfg(feature = "crossterm")]
pub use ratatui_crossterm::CrosstermBackend;
#[cfg(all(not(windows), feature = "termion"))]
pub use ratatui_termion::TermionBackend;
#[cfg(feature = "termwiz")]
pub use ratatui_termwiz::TermwizBackend;
use strum::{Display, EnumString};
