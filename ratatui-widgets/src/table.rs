//! The [`Table`] widget is used to display multiple rows and columns in a grid and allows selecting
//! one or multiple cells.
pub use self::{
    cell::Cell, highlight_spacing::HighlightSpacing, row::Row, table::Table,
    table_state::TableState,
};

mod cell;
mod highlight_spacing;
mod row;
mod table;
mod table_state;
