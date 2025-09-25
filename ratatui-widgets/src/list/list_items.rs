/// Creates a fixed sized array of [`ListItem`]s.
///
/// Can be used together with [`List::from`] to minimize memory allocations during rendering.
///
/// # Example
///
/// ```rust
/// use ratatui::Frame;
/// use ratatui::layout::Rect;
/// use ratatui::style::{Style, Stylize};
/// use ratatui::widgets::{Block, List, list_items};
///
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
/// let items = list_items!["Item 1", "Item 2", "Item 3"];
/// let list = List::from(&items);
///
/// frame.render_widget(list, area);
/// # }
/// ```
///
/// [`List::from`]: crate::list::List::from
/// [`ListItem`]: crate::list::ListItem
#[macro_export]
macro_rules! list_items {
    () => (
        [] as [$crate::list::ListItem<'_>; 0]
    );
    ($($x:expr),+ $(,)?) => (
        [$(Into::<$crate::list::ListItem<'_>>::into($x)),+]
    );
}

#[cfg(test)]
mod tests {
    use alloc::borrow::Cow;
    use alloc::vec;

    use ratatui_core::style::Style;
    use ratatui_core::text::Text;

    use crate::list::{Line, List, ListItem};

    #[test]
    fn empty_borrowed() {
        let items = list_items![];
        let list = List::from(&items);
        assert_eq!(list.items, Cow::Borrowed(&[]));
    }

    #[test]
    fn empty_owned() {
        let items = list_items!();
        let list = List::new(items);
        assert_eq!(list.items, Cow::<[ListItem]>::Owned(vec!()));
    }

    #[test]
    fn single_borrowed() {
        let items = list_items!["Item0"];
        let list = List::from(&items);
        assert_eq!(
            list.items,
            Cow::Borrowed(&[ListItem {
                content: Text::from(Line::from("Item0")),
                style: Style::new(),
            }])
        );
    }

    #[test]
    fn single_owned() {
        let items = list_items!["Item0"];
        let list = List::new(items);
        assert_eq!(
            list.items,
            Cow::<[ListItem]>::Owned(vec![ListItem {
                content: Text::from(Line::from("Item0")),
                style: Style::new(),
            }])
        );
    }

    #[test]
    fn double_borrowed() {
        let items = list_items!["Item0", "Item1"];
        let list = List::from(&items);
        assert_eq!(
            list.items,
            Cow::Borrowed(&[
                ListItem {
                    content: Text::from(Line::from("Item0")),
                    style: Style::new(),
                },
                ListItem {
                    content: Text::from(Line::from("Item1")),
                    style: Style::new(),
                }
            ])
        );
    }

    #[test]
    fn double_owned() {
        let items = list_items!["Item0", "Item1"];
        let list = List::new(items);
        assert_eq!(
            list.items,
            Cow::<[ListItem]>::Owned(vec![
                ListItem {
                    content: Text::from(Line::from("Item0")),
                    style: Style::new(),
                },
                ListItem {
                    content: Text::from(Line::from("Item1")),
                    style: Style::new(),
                }
            ])
        );
    }
}
