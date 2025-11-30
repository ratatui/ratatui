/// Creates a fixed sized array of [`Row`]s.
///
/// Can be used together with [`Table::from`] to minimize memory allocations during rendering.
///
/// # Example
///
/// ```rust
/// use ratatui::layout::Constraint;
/// use ratatui::widgets::{Table, table_rows};
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
/// [`Row`]: crate::table::Row
/// [`Table::from`]: crate::table::Table::from
#[macro_export]
macro_rules! table_rows {
    () => (
        ([] as [$crate::table::Row<'_>; 0])
    );
    ($([$($x:expr),+ $(,)?]),+ $(,)?) => (
        // TODO: avoid to_vec() to minimize memory allocations
        [$(Into::<$crate::table::Row<'_>>::into(vec![$(Into::<$crate::table::Cell<'_>>::into($x)),+])),+]
    );
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use ratatui_core::layout::Constraint;

    use crate::row_cells;
    use crate::table::{Cell, Cow, Row, Table};

    #[test]
    fn empty_borrowed() {
        let rows = table_rows![];
        let table = Table::from(&rows).widths([] as [Constraint; 0]);
        assert_eq!(table.rows, Cow::Borrowed(&[]));
    }

    #[test]
    fn empty_owned() {
        let rows = table_rows!();
        let table = Table::new(rows, [] as [Constraint; 0]);
        assert_eq!(table.rows, Cow::<[Row]>::Owned(vec![]));
    }

    #[test]
    fn single_borrowed() {
        let rows = table_rows![["Item0"]];
        let array = [Constraint::Percentage(100)];
        let table = Table::from(&rows).widths(array);
        assert_eq!(
            table.rows,
            Cow::Borrowed(&[Row {
                cells: Cow::Owned(vec![Cell::from("Item0")]),
                ..Default::default()
            }])
        );
    }

    #[test]
    fn single_owned() {
        let rows = table_rows![["Item0"]];
        let array = [Constraint::Percentage(100)];
        let table = Table::new(rows, array);
        assert_eq!(
            table.rows,
            Cow::<[Row]>::Owned(vec![Row {
                cells: Cow::Owned(vec![Cell::from("Item0")]),
                ..Default::default()
            }])
        );
    }

    #[test]
    fn double_borrowed() {
        let rows = table_rows![["Item0", "Value0"], ["Item1", "Value1"]];
        let array = [Constraint::Percentage(100)];
        let table = Table::from(&rows).widths(array);
        assert_eq!(
            table.rows,
            Cow::Borrowed(&[
                Row {
                    cells: Cow::Owned(vec![Cell::from("Item0"), Cell::from("Value0")]),
                    ..Default::default()
                },
                Row {
                    cells: Cow::Owned(vec![Cell::from("Item1"), Cell::from("Value1")]),
                    ..Default::default()
                }
            ])
        );
    }

    #[test]
    fn double_owned() {
        let rows = table_rows![["Item0", "Value0"], ["Item1", "Value1"]];
        let array = [Constraint::Percentage(100)];
        let table = Table::new(rows, array);
        assert_eq!(
            table.rows,
            Cow::<[Row]>::Owned(vec![
                Row {
                    cells: Cow::Owned(vec![Cell::from("Item0"), Cell::from("Value0")]),
                    ..Default::default()
                },
                Row {
                    cells: Cow::Owned(vec![Cell::from("Item1"), Cell::from("Value1")]),
                    ..Default::default()
                }
            ])
        );
    }

    #[test]
    fn macro_rows() {
        // TODO: implements this as rows![]
        let row_zero = row_cells!["Item0", "Value0"];
        let row_one = row_cells!["Item1", "Value1"];
        let rows = [Row::from(&row_zero), Row::from(&row_one)];

        let array = [Constraint::Percentage(100)];
        let table = Table::from(&rows).widths(array);
        assert_eq!(
            table.rows,
            Cow::Borrowed(&[
                Row {
                    cells: Cow::Owned(vec![Cell::from("Item0"), Cell::from("Value0")]),
                    ..Default::default()
                },
                Row {
                    cells: Cow::Owned(vec![Cell::from("Item1"), Cell::from("Value1")]),
                    ..Default::default()
                }
            ])
        );
    }
}
