/// Creates a fixed sized array of [`Cell`]s.
///
/// Can be used together with [`Row::from`] to minimize memory allocations during rendering.
///
/// # Example
///
/// ```rust
/// use ratatui::style::Stylize;
/// use ratatui::widgets::{Row, row_cells};
///
/// let cells = row_cells!["Cell1", "Cell2", "Cell3"];
/// Row::from(&cells);
/// ```
///
/// [`Cell`]: crate::table::Cell
/// [`Row::from`]: crate::table::Row::from
#[macro_export]
macro_rules! row_cells {
    () => (
        ([] as [$crate::table::Cell<'_>; 0])
    );
    ($($x:expr),+ $(,)?) => (
        [$(Into::<$crate::table::Cell<'_>>::into($x)),+]
    );
}

#[cfg(test)]
mod tests {
    use alloc::borrow::Cow;
    use alloc::vec;

    use crate::table::{Cell, Row};

    #[test]
    fn empty_borrowed() {
        let cells = row_cells![];
        let row = Row::from(&cells);
        assert_eq!(row.cells, Cow::Borrowed(&[]));
    }

    #[test]
    fn empty_owned() {
        let cells = row_cells!();
        let row = Row::new(cells);
        assert_eq!(row.cells, Cow::<[Cell]>::Owned(vec![]));
    }

    #[test]
    fn single_borrowed() {
        let cells = row_cells!["Item0"];
        let row = Row::from(&cells);
        assert_eq!(row.cells, Cow::Borrowed(&[Cell::from("Item0")]));
    }

    #[test]
    fn single_owned() {
        let cells = row_cells!["Item0"];
        let row = Row::new(cells);
        assert_eq!(row.cells, Cow::<[Cell]>::Owned(vec![Cell::from("Item0")]));
    }

    #[test]
    fn double_borrowed() {
        let cells = row_cells!["Item0", "Item1"];
        let row = Row::from(&cells);
        assert_eq!(
            row.cells,
            Cow::Borrowed(&[Cell::from("Item0"), Cell::from("Item1")])
        );
    }

    #[test]
    fn double_owned() {
        let cells = row_cells!["Item0", "Item1"];
        let row = Row::new(cells);
        assert_eq!(
            row.cells,
            Cow::<[Cell]>::Owned(vec![Cell::from("Item0"), Cell::from("Item1")])
        );
    }
}
