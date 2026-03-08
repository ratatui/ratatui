#![warn(missing_docs)]
//! A module for the [`Buffer`] and [`Cell`] types.

mod assert;
mod buffer;
mod cell;
mod diff_iter;

pub use buffer::Buffer;
pub use cell::{Cell, CellDiffOption};
pub use diff_iter::BufferDiff;
