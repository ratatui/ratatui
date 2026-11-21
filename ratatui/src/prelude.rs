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
//! use ratatui::prelude::*;
//! use ratatui::widgets::*;
//!
//! #[derive(Debug, Default, PartialEq, Eq)]
//! struct Line;
//!
//! assert_eq!(Line::default(), Line);
//! assert_eq!(text::Line::default(), ratatui::text::Line::from(vec![]));
//! ```

#[doc(no_inline)]
pub use ratatui_core::backend::{self, Backend};
#[cfg(feature = "crossterm")]
#[doc(no_inline)]
pub use ratatui_crossterm::{CrosstermBackend, FromCrossterm, IntoCrossterm};

#[cfg(feature = "termina")]
#[doc(no_inline)]
pub use crate::backend::{FromTermina, IntoTermina, TerminaBackend};
#[cfg(all(not(windows), feature = "termion"))]
#[doc(no_inline)]
pub use crate::backend::{FromTermion, IntoTermion, TermionBackend};
#[cfg(feature = "termwiz")]
#[doc(no_inline)]
pub use crate::backend::{FromTermwiz, IntoTermwiz, TermwizBackend};
#[doc(no_inline)]
pub use crate::buffer::{self, Buffer};
#[doc(no_inline)]
pub use crate::layout::{
    self, Alignment, Constraint, Direction, HorizontalAlignment, Layout, Margin, Position, Rect,
    Size, VerticalAlignment,
};
#[doc(no_inline)]
pub use crate::style::{self, Color, Modifier, Style, Stylize};
#[doc(no_inline)]
pub use crate::text::{self, Line, Masked, Span, Text};
#[doc(no_inline)]
pub use crate::widgets::{BlockExt, StatefulWidget, Widget};
#[doc(no_inline)]
pub use crate::{Frame, Terminal, symbols};
