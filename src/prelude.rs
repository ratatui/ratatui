//! A prelude for conveniently writing applications using this library.

#[cfg(feature = "crossterm")]
pub use crate::backend::CrosstermBackend;
#[cfg(feature = "termion")]
pub use crate::backend::TermionBackend;
#[cfg(feature = "termwiz")]
pub use crate::backend::TermwizBackend;
pub use crate::{
    backend::{self, Backend},
    buffer::{self, Buffer},
    layout::{self, Alignment, Constraint, Corner, Direction, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style, Styled, Stylize},
    symbols::{self, Marker},
    terminal::{CompletedFrame, Frame, Terminal, TerminalOptions, Viewport},
    text::{self, Line, Masked, Span, Text},
};
