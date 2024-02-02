//! A prelude for conveniently writing applications using this library.
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
#[cfg(feature = "termion")]
pub use crate::backend::TermionBackend;
#[cfg(feature = "termwiz")]
pub use crate::backend::TermwizBackend;
pub(crate) use crate::widgets::{StatefulWidgetRef, WidgetRef};
pub use crate::{
    backend::{self, Backend},
    buffer::{self, Buffer},
    layout::{self, Alignment, Constraint, Corner, Direction, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style, Styled, Stylize},
    symbols::{self, Marker},
    terminal::{CompletedFrame, Frame, Terminal, TerminalOptions, Viewport},
    text::{self, Line, Masked, Span, Text},
    widgets::{block::BlockExt, StatefulWidget, Widget},
};
