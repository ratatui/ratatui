#![warn(missing_docs)]
//! A module for the [`Buffer`] and [`Cell`] types.

mod assert;
mod buffer;
mod cell;
mod cell_width;

pub use buffer::Buffer;
pub use cell::{Cell, CellDiffOption};
pub use cell_width::StrCellWidth;
