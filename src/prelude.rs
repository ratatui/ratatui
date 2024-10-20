//! A prelude for conveniently writing applications using this library.
//!
//! The prelude module is no longer used universally in Ratatui, as it can make it harder to
//! distinguish between library and non-library types, especially when viewing source code
//! outside of an IDE (such as on GitHub or in a git diff). For more details and user feedback,
//! see [Issue #1150]. However, the prelude is still available for backward compatibility and for
//! those who prefer to use it.
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
