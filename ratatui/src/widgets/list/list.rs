use strum::{Display, EnumString};

use crate::{
    style::{Style, Styled},
    widgets::{Block, HighlightSpacing, ListItem},
};

/// A widget to display several items among which one can be selected (optional)
///
/// A list is a collection of [`ListItem`]s.
///
/// This is different from a [`Table`] because it does not handle columns, headers or footers and
/// the item's height is automatically determined. A `List` can also be put in reverse order (i.e.
/// *bottom to top*) whereas a [`Table`] cannot.
///
/// [`Table`]: crate::widgets::Table
///
/// List items can be aligned using [`Text::alignment`], for more details see [`ListItem`].
///
/// [`List`] implements [`Widget`] and so it can be drawn using
/// [`Frame::render_widget`](crate::terminal::Frame::render_widget).
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
/// use ratatui::{
///     layout::Rect,
///     style::{Style, Stylize},
///     widgets::{Block, List, ListDirection, ListItem},
///     Frame,
/// };
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
/// use ratatui::{
///     layout::Rect,
///     style::{Style, Stylize},
///     widgets::{Block, List, ListState},
///     Frame,
/// };
///
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
/// // This should be stored outside of the function in your application state.
/// let mut state = ListState::default();
/// let items = ["Item 1", "Item 2", "Item 3"];
/// let list = List::new(items)
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
/// [`ListState`]: crate::widgets::list::ListState
/// [scroll]: crate::widgets::list::ListState::offset
/// [select]: crate::widgets::list::ListState::select
/// [`Text::alignment`]: crate::text::Text::alignment
/// [`StatefulWidget`]: crate::widgets::StatefulWidget
/// [`Widget`]: crate::widgets::Widget
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct List<'a> {
    /// An optional block to wrap the widget in
    pub(crate) block: Option<Block<'a>>,
    /// The items in the list
    pub(crate) items: Vec<ListItem<'a>>,
    /// Style used as a base style for the widget
    pub(crate) style: Style,
    /// List display direction
    pub(crate) direction: ListDirection,
    /// Style used to render selected item
    pub(crate) highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    pub(crate) highlight_symbol: Option<&'a str>,
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

impl<'a> List<'a> {
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
    /// use ratatui::widgets::List;
    ///
    /// let list = List::new(["Item 1", "Item 2"]);
    /// ```
    ///
    /// From [`Text`]
    ///
    /// ```
    /// use ratatui::{
    ///     style::{Style, Stylize},
    ///     text::Text,
    ///     widgets::List,
    /// };
    ///
    /// let list = List::new([
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
    /// [`Text`]: crate::text::Text
    pub fn new<T>(items: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<ListItem<'a>>,
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
    /// [`Text`]: crate::text::Text
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn items<T>(mut self, items: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<ListItem<'a>>,
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
    pub fn block(mut self, block: Block<'a>) -> Self {
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
    /// use ratatui::{
    ///     style::{Style, Stylize},
    ///     widgets::List,
    /// };
    ///
    /// let items = ["Item 1"];
    /// let list = List::new(items).style(Style::new().red().italic());
    /// ```
    ///
    /// `List` also implements the [`Styled`] trait, which means you can use style shorthands from
    /// the [`Stylize`] trait to set the style of the widget more concisely.
    ///
    /// [`Stylize`]: crate::style::Stylize
    ///
    /// ```rust
    /// use ratatui::{style::Stylize, widgets::List};
    ///
    /// let items = ["Item 1"];
    /// let list = List::new(items).red().italic();
    /// ```
    ///
    /// [`Color`]: crate::style::Color
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
    pub const fn highlight_symbol(mut self, highlight_symbol: &'a str) -> Self {
        self.highlight_symbol = Some(highlight_symbol);
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
    /// use ratatui::{
    ///     style::{Style, Stylize},
    ///     widgets::List,
    /// };
    ///
    /// let items = ["Item 1", "Item 2"];
    /// let list = List::new(items).highlight_style(Style::new().red().italic());
    /// ```
    ///
    /// [`Color`]: crate::style::Color
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
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the list contains no elements.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<'a> Styled for List<'a> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl<'a> Styled for ListItem<'a> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl<'a, Item> FromIterator<Item> for List<'a>
where
    Item: Into<ListItem<'a>>,
{
    fn from_iter<Iter: IntoIterator<Item = Item>>(iter: Iter) -> Self {
        Self::new(iter)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::style::{Color, Modifier, Stylize};

    #[test]
    fn collect_list_from_iterator() {
        let collected: List = (0..3).map(|i| format!("Item{i}")).collect();
        let expected = List::new(["Item0", "Item1", "Item2"]);
        assert_eq!(collected, expected);
    }

    #[test]
    fn can_be_stylized() {
        assert_eq!(
            List::new::<Vec<&str>>(vec![])
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
}
