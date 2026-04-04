/// Creates a fixed sized array of [`Cell`]s.
///
/// Can be used together with [`Row::from`] to minimize memory allocations during rendering.
///
/// # Example
///
/// ```
/// use ratatui_macros::row_cells;
/// use ratatui_widgets::table::Row;
///
/// let cells = row_cells!["Cell1", "Cell2", "Cell3"];
/// Row::from(&cells);
/// ```
///
/// [`Cell`]: ratatui_widgets::table::Cell
/// [`Row::from`]: ratatui_widgets::table::Row::from
#[macro_export]
macro_rules! row_cells {
    () => (
        ([] as [::ratatui_widgets::table::Cell<'_>; 0])
    );
    ($($x:expr),+ $(,)?) => (
        [$(Into::<::ratatui_widgets::table::Cell<'_>>::into($x)),+]
    );
}

#[cfg(test)]
mod tests {
    use ratatui_widgets::table::Row;

    #[test]
    fn empty_borrowed() {
        let cells = row_cells![];
        let row = Row::from(&cells);
        assert!(row.is_borrowed());
    }

    #[test]
    fn empty_owned() {
        let cells = row_cells!();
        let row = Row::new(cells);
        assert!(row.is_owned());
    }

    #[test]
    fn single_borrowed() {
        let cells = row_cells!["Item0"];
        let row = Row::from(&cells);
        assert!(row.is_borrowed());
    }

    #[test]
    fn single_owned() {
        let cells = row_cells!["Item0"];
        let row = Row::new(cells);
        assert!(row.is_owned());
    }

    #[test]
    fn double_borrowed() {
        let cells = row_cells!["Item0", "Item1"];
        let row = Row::from(&cells);
        assert!(row.is_borrowed());
    }

    #[test]
    fn double_owned() {
        let cells = row_cells!["Item0", "Item1"];
        let row = Row::new(cells);
        assert!(row.is_owned());
    }
}
