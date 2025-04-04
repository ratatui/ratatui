#![warn(clippy::missing_const_for_fn)]
//! Provides types and traits for working with layout and positioning in the terminal.

mod alignment;
mod constraint;
mod direction;
mod flex;
mod layout;
mod margin;
mod position;
mod rect;
mod size;

pub use alignment::{Alignment, HorizontalAlignment, VerticalAlignment};
pub use constraint::Constraint;
pub use direction::Direction;
pub use flex::Flex;
pub use layout::{Layout, Spacing};
pub use margin::Margin;
pub use position::Position;
pub use rect::{Columns, Offset, Positions, Rect, Rows};
pub use size::Size;
