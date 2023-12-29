#![warn(missing_docs)]
//! A module for the [`Buffer`] and [`Cell`] types.

mod assert;
#[allow(clippy::module_inception)]
mod buffer;
mod cell;

pub use buffer::Buffer;
pub use cell::Cell;
