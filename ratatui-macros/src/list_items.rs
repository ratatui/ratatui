/// Creates a fixed sized array of [`ListItem`]s.
///
/// Can be used together with [`List::from`] to minimize memory allocations during rendering.
///
/// # Example
///
/// ```
/// use ratatui_macros::list_items;
/// use ratatui_widgets::list::List;
///
/// let items = list_items!["Item 1", "Item 2", "Item 3"];
/// let list = List::from(&items);
/// assert!(list.is_borrowed());
/// ```
///
/// [`List::from`]: ratatui_widgets::list::List::from
/// [`ListItem`]: ratatui_widgets::list::ListItem
#[macro_export]
macro_rules! list_items {
    () => (
        [] as [::ratatui_widgets::list::ListItem<'_>; 0]
    );
    ($($x:expr),+ $(,)?) => (
        [$(Into::<::ratatui_widgets::list::ListItem<'_>>::into($x)),+]
    );
}

#[cfg(test)]
mod tests {
    use ratatui_widgets::list::List;

    #[test]
    fn empty_borrowed() {
        let items = list_items![];
        let list = List::from(&items);
        assert!(list.is_borrowed());
    }

    #[test]
    fn empty_owned() {
        let items = list_items!();
        let list = List::new(items);
        assert!(list.is_owned());
    }

    #[test]
    fn single_borrowed() {
        let items = list_items!["Item0"];
        let list = List::from(&items);
        assert!(list.is_borrowed());
    }

    #[test]
    fn single_owned() {
        let items = list_items!["Item0"];
        let list = List::new(items);
        assert!(list.is_owned());
    }

    #[test]
    fn double_borrowed() {
        let items = list_items!["Item0", "Item1"];
        let list = List::from(&items);
        assert!(list.is_borrowed());
    }

    #[test]
    fn double_owned() {
        let items = list_items!["Item0", "Item1"];
        let list = List::new(items);
        assert!(list.is_owned());
    }
}
