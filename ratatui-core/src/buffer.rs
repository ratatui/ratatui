#![warn(missing_docs)]
//! A module for the [`Buffer`] and [`Cell`] types.

mod assert;
mod buffer;
mod cell;
mod cell_width;
mod diff;
#[cfg(feature = "cell-extensions")]
mod ext;

pub use buffer::Buffer;
pub use cell::{Cell, CellDiffOption};
pub use cell_width::CellWidth;
pub use diff::BufferDiff;
#[cfg(feature = "cell-extensions")]
pub use ext::{CellExt, Hyperlink, OpenSequence};
