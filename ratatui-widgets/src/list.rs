//! The [`List`] widget is used to display a list of items and allows selecting one or multiple
//! items.

use alloc::borrow::Cow;
use alloc::vec::Vec;

use ratatui_core::style::{Style, Styled};
use ratatui_core::text::Line;
use strum::{Display, EnumString};

pub use self::item::ListItem;
pub use self::state::ListState;
use crate::block::Block;
use crate::table::HighlightSpacing;

mod item;
mod rendering;
mod state;

/// A widget to display several items among which one can be selected (optional)
///
/// A list is a collection of [`ListItem`]s.
///
/// This is different from a [`Table`] because it does not handle columns, headers or footers and
/// the item's height is automatically determined. A `List` can also be put in reverse order (i.e.
/// *bottom to top*) whereas a [`Table`] cannot.
///
/// [`Table`]: crate::table::Table
///
/// List items can be aligned using [`Text::alignment`], for more details see [`ListItem`].
///
/// [`List`] is also a [`StatefulWidget`], which means you can use it with [`ListState`] to allow
/// the user to [scroll] through items and [select] one of them.
///
/// See the list in the [Examples] directory for a more in depth example of the various
/// configuration options and for how to handle state.
///
/// [Examples]: https://github.com/ratatui/ratatui/blob/main/examples/README.md
///
/// # Fluent setters
///
/// - [`List::highlight_style`] sets the style of the selected item.
/// - [`List::highlight_symbol`] sets the symbol to be displayed in front of the selected item.
/// - [`List::repeat_highlight_symbol`] sets whether to repeat the symbol and style over selected
///   multi-line items
/// - [`List::direction`] sets the list direction
///
/// # Examples
///
/// ```
/// use ratatui::layout::Rect;
/// use ratatui::style::{Style, Stylize};
/// use ratatui::widgets::{Block, List, ListDirection};
/// use ratatui::Frame;
///
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
/// let items = ["Item 1", "Item 2", "Item 3"];
/// let list = List::new(items)
///     .block(Block::bordered().title("List"))
///     .style(Style::new().white())
///     .highlight_style(Style::new().italic())
///     .highlight_symbol(">>")
///     .repeat_highlight_symbol(true)
///     .direction(ListDirection::BottomToTop);
///
/// frame.render_widget(list, area);
/// # }
/// ```
///
/// # Stateful example
///
/// ```rust
/// use ratatui::layout::Rect;
/// use ratatui::style::{Style, Stylize};
/// use ratatui::widgets::{items, Block, List, ListState};
/// use ratatui::Frame;
///
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
///
/// // This should be stored outside of the function in your application state.
/// let mut state = ListState::default();
/// let items = items!["Item 1", "Item 2", "Item 3"];
///
/// let list = List::from(&items)
///     .block(Block::bordered().title("List"))
///     .highlight_style(Style::new().reversed())
///     .highlight_symbol(">>")
///     .repeat_highlight_symbol(true);
///
/// frame.render_stateful_widget(list, area, &mut state);
/// # }
/// ```
///
/// In addition to `List::new`, any iterator whose element is convertible to `ListItem` can be
/// collected into `List`.
///
/// ```
/// use ratatui::widgets::List;
///
/// (0..5).map(|i| format!("Item{i}")).collect::<List>();
/// ```
///
/// [`ListState`]: crate::list::ListState
/// [scroll]: crate::list::ListState::offset
/// [select]: crate::list::ListState::select
/// [`Text::alignment`]: ratatui_core::text::Text::alignment
/// [`StatefulWidget`]: ratatui_core::widgets::StatefulWidget
/// [`Widget`]: ratatui_core::widgets::Widget
#[derive(Debug, Clone, Default, Eq, Hash, PartialEq)]
pub struct List<'lend, 'data> {
    /// An optional block to wrap the widget in
    pub(crate) block: Option<Block<'data>>,
    /// The items in the list
    pub(crate) items: Cow<'lend, [ListItem<'data>]>,
    /// Style used as a base style for the widget
    pub(crate) style: Style,
    /// List display direction
    pub(crate) direction: ListDirection,
    /// Style used to render selected item
    pub(crate) highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    pub(crate) highlight_symbol: Option<Line<'data>>,
    /// Whether to repeat the highlight symbol for each line of the selected item
    pub(crate) repeat_highlight_symbol: bool,
    /// Decides when to allocate spacing for the selection symbol
    pub(crate) highlight_spacing: HighlightSpacing,
    /// How many items to try to keep visible before and after the selected item
    pub(crate) scroll_padding: usize,
}

/// Defines the direction in which the list will be rendered.
///
/// If there are too few items to fill the screen, the list will stick to the starting edge.
///
/// See [`List::direction`].
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ListDirection {
    /// The first value is on the top, going to the bottom
    #[default]
    TopToBottom,
    /// The first value is on the bottom, going to the top.
    BottomToTop,
}

/// Creates a fixed sized array of [`ListItem`]s.
///
/// Can be used together with `List::from` to only generate the [`ListItem`]s if
/// any changed instead of each time the [`List`] is rendered.
///
/// # Example
///
/// ```rust
/// use ratatui::layout::Rect;
/// use ratatui::style::{Style, Stylize};
/// use ratatui::widgets::{items, Block, List};
/// use ratatui::Frame;
///
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
/// let items = items!["Item 1", "Item 2", "Item 3"];
/// let list = List::from(&items)
///     .block(Block::bordered().title("List"))
///     .highlight_style(Style::new().reversed())
///     .highlight_symbol(">>")
///     .repeat_highlight_symbol(true);
///
/// frame.render_widget(list, area);
/// # }
/// ```
#[macro_export]
macro_rules! items {
    () => (
        ([] as [$crate::list::ListItem<'_>; 0])
    );
    ($($x:expr),+ $(,)?) => (
        [$(Into::<$crate::list::ListItem<'_>>::into($x)),+]
    );
}

impl<'data> List<'_, 'data> {
    /// Creates a new list from [`ListItem`]s
    ///
    /// The `items` parameter accepts any value that can be converted into an iterator of
    /// [`Into<ListItem>`]. This includes arrays of [`&str`] or [`Vec`]s of [`Text`].
    ///
    /// # Example
    ///
    /// From a slice of [`&str`]
    ///
    /// ```
    /// use ratatui::widgets::{items, List};
    ///
    /// let list = List::new(items!["Item 1", "Item 2"]);
    /// ```
    ///
    /// From [`Text`]
    ///
    /// ```
    /// use ratatui::style::{Style, Stylize};
    /// use ratatui::text::Text;
    /// use ratatui::widgets::{items, List};
    ///
    /// let list = List::new(items![
    ///     Text::styled("Item 1", Style::new().red()),
    ///     Text::styled("Item 2", Style::new().red()),
    /// ]);
    /// ```
    ///
    /// You can also create an empty list using the [`Default`] implementation and use the
    /// [`List::items`] fluent setter.
    ///
    /// ```rust
    /// use ratatui::widgets::List;
    ///
    /// let empty_list = List::default();
    /// let filled_list = empty_list.items(["Item 1"]);
    /// ```
    ///
    /// [`Text`]: ratatui_core::text::Text
    #[must_use]
    pub fn new<T>(items: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<ListItem<'data>>,
    {
        Self {
            block: None,
            style: Style::default(),
            items: items.into_iter().map(Into::into).collect(),
            direction: ListDirection::default(),
            ..Self::default()
        }
    }

    /// Set the items
    ///
    /// The `items` parameter accepts any value that can be converted into an iterator of
    /// [`Into<ListItem>`]. This includes arrays of [`&str`] or [`Vec`]s of [`Text`].
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::widgets::List;
    ///
    /// let list = List::default().items(["Item 1", "Item 2"]);
    /// ```
    ///
    /// [`Text`]: ratatui_core::text::Text
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn items<T>(mut self, items: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<ListItem<'data>>,
    {
        self.items = items.into_iter().map(Into::into).collect();
        self
    }

    /// Wraps the list with a custom [`Block`] widget.
    ///
    /// The `block` parameter holds the specified [`Block`] to be created around the [`List`]
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::{Block, List};
    ///
    /// let items = ["Item 1"];
    /// let block = Block::bordered().title("List");
    /// let list = List::new(items).block(block);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'data>) -> Self {
        self.block = Some(block);
        self
    }

    /// Sets the base style of the widget
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// All text rendered by the widget will use this style, unless overridden by [`Block::style`],
    /// [`ListItem::style`], or the styles of the [`ListItem`]'s content.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::style::{Style, Stylize};
    /// use ratatui::widgets::List;
    ///
    /// let items = ["Item 1"];
    /// let list = List::new(items).style(Style::new().red().italic());
    /// ```
    ///
    /// `List` also implements the [`Styled`] trait, which means you can use style shorthands from
    /// the [`Stylize`] trait to set the style of the widget more concisely.
    ///
    /// [`Stylize`]: ratatui_core::style::Stylize
    ///
    /// ```rust
    /// use ratatui::style::Stylize;
    /// use ratatui::widgets::List;
    ///
    /// let items = ["Item 1"];
    /// let list = List::new(items).red().italic();
    /// ```
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Set the symbol to be displayed in front of the selected item
    ///
    /// By default there are no highlight symbol.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::List;
    ///
    /// let items = ["Item 1", "Item 2"];
    /// let list = List::new(items).highlight_symbol(">>");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn highlight_symbol<L: Into<Line<'data>>>(mut self, highlight_symbol: L) -> Self {
        self.highlight_symbol = Some(highlight_symbol.into());
        self
    }

    /// Set the style of the selected item
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be applied to the entire item, including the
    /// [highlight symbol](List::highlight_symbol) if it is displayed, and will override any style
    /// set on the item or on the individual cells.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::style::{Style, Stylize};
    /// use ratatui::widgets::List;
    ///
    /// let items = ["Item 1", "Item 2"];
    /// let list = List::new(items).highlight_style(Style::new().red().italic());
    /// ```
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn highlight_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.highlight_style = style.into();
        self
    }

    /// Set whether to repeat the highlight symbol and style over selected multi-line items
    ///
    /// This is `false` by default.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn repeat_highlight_symbol(mut self, repeat: bool) -> Self {
        self.repeat_highlight_symbol = repeat;
        self
    }

    /// Set when to show the highlight spacing
    ///
    /// The highlight spacing is the spacing that is allocated for the selection symbol (if enabled)
    /// and is used to shift the list when an item is selected. This method allows you to configure
    /// when this spacing is allocated.
    ///
    /// - [`HighlightSpacing::Always`] will always allocate the spacing, regardless of whether an
    ///   item is selected or not. This means that the table will never change size, regardless of
    ///   if an item is selected or not.
    /// - [`HighlightSpacing::WhenSelected`] will only allocate the spacing if an item is selected.
    ///   This means that the table will shift when an item is selected. This is the default setting
    ///   for backwards compatibility, but it is recommended to use `HighlightSpacing::Always` for a
    ///   better user experience.
    /// - [`HighlightSpacing::Never`] will never allocate the spacing, regardless of whether an item
    ///   is selected or not. This means that the highlight symbol will never be drawn.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::{HighlightSpacing, List};
    ///
    /// let items = ["Item 1"];
    /// let list = List::new(items).highlight_spacing(HighlightSpacing::Always);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn highlight_spacing(mut self, value: HighlightSpacing) -> Self {
        self.highlight_spacing = value;
        self
    }

    /// Defines the list direction (up or down)
    ///
    /// Defines if the `List` is displayed *top to bottom* (default) or *bottom to top*.
    /// If there is too few items to fill the screen, the list will stick to the starting edge.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Example
    ///
    /// Bottom to top
    ///
    /// ```rust
    /// use ratatui::widgets::{List, ListDirection};
    ///
    /// let items = ["Item 1"];
    /// let list = List::new(items).direction(ListDirection::BottomToTop);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn direction(mut self, direction: ListDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the number of items around the currently selected item that should be kept visible
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Example
    ///
    /// A padding value of 1 will keep 1 item above and 1 item bellow visible if possible
    ///
    /// ```rust
    /// use ratatui::widgets::List;
    ///
    /// let items = ["Item 1"];
    /// let list = List::new(items).scroll_padding(1);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn scroll_padding(mut self, padding: usize) -> Self {
        self.scroll_padding = padding;
        self
    }

    /// Returns the number of [`ListItem`]s in the list
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the list contains no elements.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Styled for List<'_, '_> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl Styled for ListItem<'_> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl<'data, Item> FromIterator<Item> for List<'_, 'data>
where
    Item: Into<ListItem<'data>>,
{
    fn from_iter<Iter: IntoIterator<Item = Item>>(iter: Iter) -> Self {
        let items = iter.into_iter().map(Into::into).collect();
        Self::from(Cow::Owned(items))
    }
}

impl<'lend, 'data> From<Cow<'lend, [ListItem<'data>]>> for List<'lend, 'data> {
    fn from(value: Cow<'lend, [ListItem<'data>]>) -> Self {
        Self {
            items: value,
            ..Self::default()
        }
    }
}

impl<'data> From<Vec<ListItem<'data>>> for List<'_, 'data> {
    fn from(value: Vec<ListItem<'data>>) -> Self {
        Self {
            items: Cow::Owned(value),
            ..Self::default()
        }
    }
}

impl<'lend, 'data> From<&'lend Vec<ListItem<'data>>> for List<'lend, 'data> {
    fn from(value: &'lend Vec<ListItem<'data>>) -> Self {
        Self {
            items: Cow::Borrowed(value),
            ..Self::default()
        }
    }
}

impl<'lend, 'data, const N: usize> From<&'lend [ListItem<'data>; N]> for List<'lend, 'data> {
    fn from(value: &'lend [ListItem<'data>; N]) -> Self {
        Self {
            items: Cow::Borrowed(value),
            ..Self::default()
        }
    }
}

impl<'lend, 'data> From<&'lend [ListItem<'data>]> for List<'lend, 'data> {
    fn from(value: &'lend [ListItem<'data>]) -> Self {
        Self {
            items: Cow::Borrowed(value),
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::{format, vec};

    use pretty_assertions::assert_eq;
    use ratatui_core::style::{Color, Modifier, Stylize};
    use ratatui_core::text::Text;

    use super::*;

    #[test]
    fn collect_list_from_iterator() {
        let collected: List = (0..3).map(|i| format!("Item{i}")).collect();
        let items = items!["Item0", "Item1", "Item2"];
        let expected = List::new(items);
        assert!(matches!(collected.items, Cow::Owned(_)));
        assert!(matches!(expected.items, Cow::Owned(_)));
        assert_eq!(collected, expected);

        let collected: List = (0..3).map(|i| format!("Item{i}")).collect();
        let items = [
            ListItem::from("Item0"),
            ListItem::from("Item1"),
            ListItem::from("Item2"),
        ];
        let expected = List::new(items);
        assert!(matches!(collected.items, Cow::Owned(_)));
        assert!(matches!(expected.items, Cow::Owned(_)));
        assert_eq!(collected, expected);
    }

    #[test]
    fn fluent_items() {
        let collected: List = (0..3).map(|i| format!("Item{i}")).collect();
        let expected = List::default().items(vec![
            ListItem::from("Item0"),
            "Item1".into(),
            "Item2".into(),
        ]);
        assert!(matches!(collected.items, Cow::Owned(_)));
        assert!(matches!(expected.items, Cow::Owned(_)));
        assert_eq!(collected, expected);

        let collected: List = (0..3).map(|i| format!("Item{i}")).collect();
        let items = [
            ListItem::from("Item0"),
            ListItem::from("Item1"),
            ListItem::from("Item2"),
        ];
        let expected = List::default().items(items);
        assert!(matches!(collected.items, Cow::Owned(_)));
        assert!(matches!(expected.items, Cow::Owned(_)));
        assert_eq!(collected, expected);
    }

    #[test]
    fn can_be_stylized() {
        assert_eq!(
            List::new::<Vec<ListItem<'_>>>(vec![])
                .black()
                .on_white()
                .bold()
                .not_dim()
                .style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::DIM)
        );
    }

    #[test]
    fn macro_items() {
        let items = items![];
        let list = List::new(items);
        assert!(matches!(list.items, Cow::Owned(_)));
        assert_eq!(*list.items, []);

        let items = items!();
        let list = List::new(items);
        assert!(matches!(list.items, Cow::Owned(_)));
        assert_eq!(*list.items, []);

        let items = items!["Item0"];
        let list = List::new(items);
        assert!(matches!(list.items, Cow::Owned(_)));
        assert_eq!(
            *list.items,
            [ListItem {
                content: Text::from(Line::from("Item0")),
                style: Style::new(),
            }]
        );

        let items = items!["Item0", "Item1"];
        let list = List::new(items);
        assert!(matches!(list.items, Cow::Owned(_)));
        assert_eq!(
            *list.items,
            [
                ListItem {
                    content: Text::from(Line::from("Item0")),
                    style: Style::new(),
                },
                ListItem {
                    content: Text::from(Line::from("Item1")),
                    style: Style::new(),
                }
            ]
        );
    }

    #[test]
    fn from_cow() {
        let items = items!["Item0"];
        let cow: Cow<[ListItem<'_>]> = Cow::Borrowed(&items);
        let list = List::from(cow);
        assert!(matches!(list.items, Cow::Borrowed(_)));
        assert_eq!(
            *list.items,
            [ListItem {
                content: Text::from(Line::from("Item0")),
                style: Style::new(),
            }]
        );

        let items: Vec<_> = items!["Item0"].to_vec();
        let cow: Cow<[ListItem<'_>]> = Cow::Owned(items);
        let list = List::from(cow);
        assert!(matches!(list.items, Cow::Owned(_)));
        assert_eq!(
            *list.items,
            [ListItem {
                content: Text::from(Line::from("Item0")),
                style: Style::new(),
            }]
        );
    }

    #[test]
    fn from_vec() {
        let items: Vec<_> = items!["Item0"].to_vec();
        let list = List::from(items);
        assert!(matches!(list.items, Cow::Owned(_)));
        assert_eq!(
            *list.items,
            [ListItem {
                content: Text::from(Line::from("Item0")),
                style: Style::new(),
            }]
        );

        let items: Vec<_> = items!["Item0"].to_vec();
        let list = List::from(&items);
        assert!(matches!(list.items, Cow::Borrowed(_)));
        assert_eq!(
            *list.items,
            [ListItem {
                content: Text::from(Line::from("Item0")),
                style: Style::new(),
            }]
        );
    }

    #[test]
    fn from_slice() {
        let items = items!["Item0"];
        let list = List::from(&items);
        assert!(matches!(list.items, Cow::Borrowed(_)));
        assert_eq!(
            *list.items,
            [ListItem {
                content: Text::from(Line::from("Item0")),
                style: Style::new(),
            }]
        );

        let items: Vec<_> = items!["Item0"].to_vec();
        let list = List::from(items.as_slice());
        assert!(matches!(list.items, Cow::Borrowed(_)));
        assert_eq!(
            *list.items,
            [ListItem {
                content: Text::from(Line::from("Item0")),
                style: Style::new(),
            }]
        );
    }
}
