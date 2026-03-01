/// Creates a fixed sized array of [`Row`]s.
///
/// Can be used together with [`Table::from`] to minimize memory allocations during rendering.
///
/// # Example
///
/// ```
/// use ratatui_core::layout::Constraint;
/// use ratatui_macros::table_rows;
/// use ratatui_widgets::table::Table;
///
/// let rows = table_rows![
///     ["Cell1", "Value1"],
///     ["Cell2", "Value2"],
///     ["Cell3", "Value3"]
/// ];
/// let widths = [
///     Constraint::Length(5),
///     Constraint::Length(5),
///     Constraint::Length(10),
/// ];
/// let table = Table::from(&rows).widths(widths);
/// ```
///
/// [`Row`]: ratatui_widgets::table::Row
/// [`Table::from`]: ratatui_widgets::table::Table::from
#[macro_export]
macro_rules! table_rows {
    () => (
        ([] as [::ratatui_widgets::table::Row<'_>; 0])
    );
    ($([$($x:expr),+ $(,)?]),+ $(,)?) => (
        // TODO: avoid to_vec() to minimize memory allocations
        [$(Into::<::ratatui_widgets::table::Row<'_>>::into(vec![$(Into::<::ratatui_widgets::table::Cell<'_>>::into($x)),+])),+]
    );
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use ratatui_core::layout::Constraint;
    use ratatui_widgets::table::Table;

    #[test]
    fn empty_borrowed() {
        let rows = table_rows![];
        let table = Table::from(&rows).widths([] as [Constraint; 0]);
        assert!(table.is_borrowed());
    }

    #[test]
    fn empty_owned() {
        let rows = table_rows!();
        let table = Table::new(rows, [] as [Constraint; 0]);
        assert!(table.is_owned());
    }

    #[test]
    fn single_borrowed() {
        let rows = table_rows![["Item0"]];
        let array = [Constraint::Percentage(100)];
        let table = Table::from(&rows).widths(array);
        assert!(table.is_borrowed());
    }

    #[test]
    fn single_owned() {
        let rows = table_rows![["Item0"]];
        let array = [Constraint::Percentage(100)];
        let table = Table::new(rows, array);
        assert!(table.is_owned());
    }

    #[test]
    fn double_borrowed() {
        let rows = table_rows![["Item0", "Value0"], ["Item1", "Value1"]];
        let array = [Constraint::Percentage(100)];
        let table = Table::from(&rows).widths(array);
        assert!(table.is_borrowed());
    }

    #[test]
    fn double_owned() {
        let rows = table_rows![["Item0", "Value0"], ["Item1", "Value1"]];
        let array = [Constraint::Percentage(100)];
        let table = Table::new(rows, array);
        assert!(table.is_owned());
    }
}
