//! A prelude for conveniently writing applications using this library.
//!
//! We moved away from using the prelude module universally in Ratatui because it makes it harder to
//! see which types are coming from the library and which are not and occasionally especially when
//! reading source code in a non-IDE context (such as on GitHub or in a git diff). See [Issue #1150]
//! for more information and links to user feedback. We have kept the prelude module in the library
//! for backwards compatibility and for users who prefer to use it.
//!
//! [Issue #1150]: https://github.com/ratatui/ratatui/issues/1150
//!
//! # Examples
//!
//! ```rust,no_run
//! use ratatui::prelude::*;
//! ```
//!
//! Aside from the main types that are used in the library, this prelude also re-exports several
//! modules to make it easy to qualify types that would otherwise collide. E.g.:
//!
//! ```rust
//! use ratatui::{prelude::*, widgets::*};
//!
//! #[derive(Debug, Default, PartialEq, Eq)]
//! struct Line;
//!
//! assert_eq!(Line::default(), Line);
//! assert_eq!(text::Line::default(), ratatui::text::Line::from(vec![]));
//! ```

#[cfg(feature = "crossterm")]
pub use crate::backend::CrosstermBackend;
#[cfg(all(not(windows), feature = "termion"))]
pub use crate::backend::TermionBackend;
#[cfg(feature = "termwiz")]
pub use crate::backend::TermwizBackend;
pub use crate::{
    backend::{self, Backend},
    buffer::{self, Buffer},
    layout::{self, Alignment, Constraint, Direction, Layout, Margin, Position, Rect, Size},
    style::{self, Color, Modifier, Style, Stylize},
    symbols::{self},
    text::{self, Line, Masked, Span, Text},
    widgets::{block::BlockExt, StatefulWidget, Widget},
    Frame, Terminal,
};
