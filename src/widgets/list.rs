use strum::{Display, EnumString};
use unicode_width::UnicodeWidthStr;

use crate::{
    prelude::*,
    style::Styled,
    widgets::{Block, HighlightSpacing},
};

/// State of the [`List`] widget
///
/// This state can be used to scroll through items and select one. When the list is rendered as a
/// stateful widget, the selected item will be highlighted and the list will be shifted to ensure
/// that the selected item is visible. This will modify the [`ListState`] object passed to the
/// [`Frame::render_stateful_widget`](crate::terminal::Frame::render_stateful_widget) method.
///
/// The state consists of two fields:
/// - [`offset`]: the index of the first item to be displayed
/// - [`selected`]: the index of the selected item, which can be `None` if no item is selected
///
/// [`offset`]: ListState::offset()
/// [`selected`]: ListState::selected()
///
/// See the list in the [Examples] directory for a more in depth example of the various
/// configuration options and for how to handle state.
///
/// [Examples]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md
///
/// # Example
///
/// ```rust
/// # use ratatui::{prelude::*, widgets::*};
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
/// # let items = ["Item 1"];
/// let list = List::new(items);
///
/// // This should be stored outside of the function in your application state.
/// let mut state = ListState::default();
///
/// *state.offset_mut() = 1; // display the second item and onwards
/// state.select(Some(3)); // select the forth item (0-indexed)
///
/// frame.render_stateful_widget(list, area, &mut state);
/// # }
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListState {
    offset: usize,
    selected: Option<usize>,
}

impl ListState {
    /// Sets the index of the first item to be displayed
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let state = ListState::default().with_offset(1);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Sets the index of the selected item
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let state = ListState::default().with_selected(Some(1));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn with_selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    /// Index of the first item to be displayed
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let state = ListState::default();
    /// assert_eq!(state.offset(), 0);
    /// ```
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Mutable reference to the index of the first item to be displayed
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let mut state = ListState::default();
    /// *state.offset_mut() = 1;
    /// ```
    pub fn offset_mut(&mut self) -> &mut usize {
        &mut self.offset
    }

    /// Index of the selected item
    ///
    /// Returns `None` if no item is selected
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let state = TableState::default();
    /// assert_eq!(state.selected(), None);
    /// ```
    pub const fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Mutable reference to the index of the selected item
    ///
    /// Returns `None` if no item is selected
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let mut state = ListState::default();
    /// *state.selected_mut() = Some(1);
    /// ```
    pub fn selected_mut(&mut self) -> &mut Option<usize> {
        &mut self.selected
    }

    /// Sets the index of the selected item
    ///
    /// Set to `None` if no item is selected. This will also reset the offset to `0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let mut state = ListState::default();
    /// state.select(Some(1));
    /// ```
    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }

    /// Selects the next item or the first one if no item is selected
    ///
    /// Note: until the list is rendered, the number of items is not known, so the index is set to
    /// `0` and will be corrected when the list is rendered
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let mut state = ListState::default();
    /// state.select_next();
    /// ```
    pub fn select_next(&mut self) {
        let next = self.selected.map_or(0, |i| i.saturating_add(1));
        self.select(Some(next));
    }

    /// Selects the previous item or the last one if no item is selected
    ///
    /// Note: until the list is rendered, the number of items is not known, so the index is set to
    /// `usize::MAX` and will be corrected when the list is rendered
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let mut state = ListState::default();
    /// state.select_previous();
    /// ```
    pub fn select_previous(&mut self) {
        let previous = self.selected.map_or(usize::MAX, |i| i.saturating_sub(1));
        self.select(Some(previous));
    }

    /// Selects the first item
    ///
    /// Note: until the list is rendered, the number of items is not known, so the index is set to
    /// `0` and will be corrected when the list is rendered
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let mut state = ListState::default();
    /// state.select_first();
    /// ```
    pub fn select_first(&mut self) {
        self.select(Some(0));
    }

    /// Selects the last item
    ///
    /// Note: until the list is rendered, the number of items is not known, so the index is set to
    /// `usize::MAX` and will be corrected when the list is rendered
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let mut state = ListState::default();
    /// state.select_last();
    /// ```
    pub fn select_last(&mut self) {
        self.select(Some(usize::MAX));
    }
}

/// A single item in a [`List`]
///
/// The item's height is defined by the number of lines it contains. This can be queried using
/// [`ListItem::height`]. Similarly, [`ListItem::width`] will return the maximum width of all
/// lines.
///
/// You can set the style of an item with [`ListItem::style`] or using the [`Stylize`] trait.
/// This [`Style`] will be combined with the [`Style`] of the inner [`Text`]. The [`Style`]
/// of the [`Text`] will be added to the [`Style`] of the [`ListItem`].
///
/// You can also align a `ListItem` by aligning its underlying [`Text`] and [`Line`]s. For that,
/// see [`Text::alignment`] and [`Line::alignment`]. On a multiline `Text`, one `Line` can override
/// the alignment by setting it explicitly.
///
/// # Examples
///
/// You can create [`ListItem`]s from simple `&str`
///
/// ```rust
/// # use ratatui::{prelude::*, widgets::*};
/// let item = ListItem::new("Item 1");
/// ```
///
/// Anything that can be converted to [`Text`] can be a [`ListItem`].
///
/// ```rust
/// # use ratatui::{prelude::*, widgets::*};
/// let item1: ListItem = "Item 1".into();
/// let item2: ListItem = Line::raw("Item 2").into();
/// ```
///
/// A [`ListItem`] styled with [`Stylize`]
///
/// ```rust
/// # use ratatui::{prelude::*, widgets::*};
/// let item = ListItem::new("Item 1").red().on_white();
/// ```
///
/// If you need more control over the item's style, you can explicitly style the underlying
/// [`Text`]
///
/// ```rust
/// # use ratatui::{prelude::*, widgets::*};
/// let mut text = Text::default();
/// text.extend(["Item".blue(), Span::raw(" "), "1".bold().red()]);
/// let item = ListItem::new(text);
/// ```
///
/// A right-aligned `ListItem`
///
/// ```rust
/// # use ratatui::{prelude::*, widgets::*};
/// ListItem::new(Text::from("foo").alignment(Alignment::Right));
/// ```
///
/// [`Stylize`]: crate::style::Stylize
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ListItem<'a> {
    content: Text<'a>,
    style: Style,
}

impl<'a> ListItem<'a> {
    /// Creates a new [`ListItem`]
    ///
    /// The `content` parameter accepts any value that can be converted into [`Text`].
    ///
    /// # Examples
    ///
    /// You can create [`ListItem`]s from simple `&str`
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let item = ListItem::new("Item 1");
    /// ```
    ///
    /// Anything that can be converted to [`Text`] can be a [`ListItem`].
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let item1: ListItem = "Item 1".into();
    /// let item2: ListItem = Line::raw("Item 2").into();
    /// ```
    ///
    /// You can also create multilines item
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let item = ListItem::new("Multi-line\nitem");
    /// ```
    ///
    /// # See also
    ///
    /// - [`List::new`] to create a list of items that can be converted to [`ListItem`]
    pub fn new<T>(content: T) -> Self
    where
        T: Into<Text<'a>>,
    {
        Self {
            content: content.into(),
            style: Style::default(),
        }
    }

    /// Sets the item style
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This [`Style`] can be overridden by the [`Style`] of the [`Text`] content.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let item = ListItem::new("Item 1").style(Style::new().red().italic());
    /// ```
    ///
    /// `ListItem` also implements the [`Styled`] trait, which means you can use style shorthands
    /// from the [`Stylize`](crate::style::Stylize) trait to set the style of the widget more
    /// concisely.
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let item = ListItem::new("Item 1").red().italic();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Returns the item height
    ///
    /// # Examples
    ///
    /// One line item
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let item = ListItem::new("Item 1");
    /// assert_eq!(item.height(), 1);
    /// ```
    ///
    /// Two lines item (note the `\n`)
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let item = ListItem::new("Multi-line\nitem");
    /// assert_eq!(item.height(), 2);
    /// ```
    pub fn height(&self) -> usize {
        self.content.height()
    }

    /// Returns the max width of all the lines
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let item = ListItem::new("12345");
    /// assert_eq!(item.width(), 5);
    /// ```
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let item = ListItem::new("12345\n1234567");
    /// assert_eq!(item.width(), 7);
    /// ```
    pub fn width(&self) -> usize {
        self.content.width()
    }
}

impl<'a, T> From<T> for ListItem<'a>
where
    T: Into<Text<'a>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

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
/// the user to [scroll](ListState::offset) through items and [select](ListState::select) one of
/// them.
///
/// See the list in the [Examples] directory for a more in depth example of the various
/// configuration options and for how to handle state.
///
/// [Examples]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md
///
/// # Fluent setters
///
/// - [`List::highlight_style`] sets the style of the selected item.
/// - [`List::highlight_symbol`] sets the symbol to be displayed in front of the selected item.
/// - [`List::repeat_highlight_symbol`] sets whether to repeat the symbol and style over selected
/// multi-line items
/// - [`List::direction`] sets the list direction
///
/// # Examples
///
/// ```
/// use ratatui::{prelude::*, widgets::*};
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
/// let items = ["Item 1", "Item 2", "Item 3"];
/// let list = List::new(items)
///     .block(Block::bordered().title("List"))
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
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
/// # use ratatui::{prelude::*, widgets::*};
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
/// // This should be stored outside of the function in your application state.
/// let mut state = ListState::default();
/// let items = ["Item 1", "Item 2", "Item 3"];
/// let list = List::new(items)
///     .block(Block::bordered().title("List"))
///     .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
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
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct List<'a> {
    block: Option<Block<'a>>,
    items: Vec<ListItem<'a>>,
    /// Style used as a base style for the widget
    style: Style,
    /// List display direction
    direction: ListDirection,
    /// Style used to render selected item
    highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'a str>,
    /// Whether to repeat the highlight symbol for each line of the selected item
    repeat_highlight_symbol: bool,
    /// Decides when to allocate spacing for the selection symbol
    highlight_spacing: HighlightSpacing,
    /// How many items to try to keep visible before and after the selected item
    scroll_padding: usize,
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
    /// # use ratatui::{prelude::*, widgets::*};
    /// let list = List::new(["Item 1", "Item 2"]);
    /// ```
    ///
    /// From [`Text`]
    ///
    /// ```
    /// # use ratatui::{prelude::*, widgets::*};
    /// let list = List::new([
    ///     Text::styled("Item 1", Style::default().red()),
    ///     Text::styled("Item 2", Style::default().red()),
    /// ]);
    /// ```
    ///
    /// You can also create an empty list using the [`Default`] implementation and use the
    /// [`List::items`] fluent setter.
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let empty_list = List::default();
    /// let filled_list = empty_list.items(["Item 1"]);
    /// ```
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
    /// # use ratatui::{prelude::*, widgets::*};
    /// let list = List::default().items(["Item 1", "Item 2"]);
    /// ```
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
    /// # use ratatui::{prelude::*, widgets::*};
    /// # let items = ["Item 1"];
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
    /// # use ratatui::{prelude::*, widgets::*};
    /// # let items = ["Item 1"];
    /// let list = List::new(items).style(Style::new().red().italic());
    /// ```
    ///
    /// `List` also implements the [`Styled`] trait, which means you can use style shorthands from
    /// the [`Stylize`] trait to set the style of the widget more concisely.
    ///
    /// [`Stylize`]: crate::style::Stylize
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// # let items = ["Item 1"];
    /// let list = List::new(items).red().italic();
    /// ```
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
    /// # use ratatui::{prelude::*, widgets::*};
    /// # let items = ["Item 1", "Item 2"];
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
    /// # use ratatui::{prelude::*, widgets::*};
    /// # let items = ["Item 1", "Item 2"];
    /// let list = List::new(items).highlight_style(Style::new().red().italic());
    /// ```
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
    /// # use ratatui::{prelude::*, widgets::*};
    /// # let items = ["Item 1"];
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
    /// # use ratatui::{prelude::*, widgets::*};
    /// # let items = ["Item 1"];
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
    /// # use ratatui::{prelude::*, widgets::*};
    /// # let items = ["Item 1"];
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

    /// Applies scroll padding to the selected index, reducing the padding value to keep the
    /// selected item on screen even with items of inconsistent sizes
    ///
    /// This function is sensitive to how the bounds checking function handles item height
    fn apply_scroll_padding_to_selected_index(
        &self,
        selected: Option<usize>,
        max_height: usize,
        first_visible_index: usize,
        last_visible_index: usize,
    ) -> Option<usize> {
        let last_valid_index = self.items.len().saturating_sub(1);
        let selected = selected?.min(last_valid_index);

        // The bellow loop handles situations where the list item sizes may not be consistent,
        // where the offset would have excluded some items that we want to include, or could
        // cause the offset value to be set to an inconsistent value each time we render.
        // The padding value will be reduced in case any of these issues would occur
        let mut scroll_padding = self.scroll_padding;
        while scroll_padding > 0 {
            let mut height_around_selected = 0;
            for index in selected.saturating_sub(scroll_padding)
                ..=selected
                    .saturating_add(scroll_padding)
                    .min(last_valid_index)
            {
                height_around_selected += self.items[index].height();
            }
            if height_around_selected <= max_height {
                break;
            }
            scroll_padding -= 1;
        }

        Some(
            if (selected + scroll_padding).min(last_valid_index) >= last_visible_index {
                selected + scroll_padding
            } else if selected.saturating_sub(scroll_padding) < first_visible_index {
                selected.saturating_sub(scroll_padding)
            } else {
                selected
            }
            .min(last_valid_index),
        )
    }

    /// Given an offset, calculate which items can fit in a given area
    fn get_items_bounds(
        &self,
        selected: Option<usize>,
        offset: usize,
        max_height: usize,
    ) -> (usize, usize) {
        let offset = offset.min(self.items.len().saturating_sub(1));

        // Note: visible here implies visible in the given area
        let mut first_visible_index = offset;
        let mut last_visible_index = offset;

        // Current height of all items in the list to render, beginning at the offset
        let mut height_from_offset = 0;

        // Calculate the last visible index and total height of the items
        // that will fit in the available space
        for item in self.items.iter().skip(offset) {
            if height_from_offset + item.height() > max_height {
                break;
            }

            height_from_offset += item.height();

            last_visible_index += 1;
        }

        // Get the selected index and apply scroll_padding to it, but still honor the offset if
        // nothing is selected. This allows for the list to stay at a position after select()ing
        // None.
        let index_to_display = self
            .apply_scroll_padding_to_selected_index(
                selected,
                max_height,
                first_visible_index,
                last_visible_index,
            )
            .unwrap_or(offset);

        // Recall that last_visible_index is the index of what we
        // can render up to in the given space after the offset
        // If we have an item selected that is out of the viewable area (or
        // the offset is still set), we still need to show this item
        while index_to_display >= last_visible_index {
            height_from_offset =
                height_from_offset.saturating_add(self.items[last_visible_index].height());

            last_visible_index += 1;

            // Now we need to hide previous items since we didn't have space
            // for the selected/offset item
            while height_from_offset > max_height {
                height_from_offset =
                    height_from_offset.saturating_sub(self.items[first_visible_index].height());

                // Remove this item to view by starting at the next item index
                first_visible_index += 1;
            }
        }

        // Here we're doing something similar to what we just did above
        // If the selected item index is not in the viewable area, let's try to show the item
        while index_to_display < first_visible_index {
            first_visible_index -= 1;

            height_from_offset =
                height_from_offset.saturating_add(self.items[first_visible_index].height());

            // Don't show an item if it is beyond our viewable height
            while height_from_offset > max_height {
                last_visible_index -= 1;

                height_from_offset =
                    height_from_offset.saturating_sub(self.items[last_visible_index].height());
            }
        }

        (first_visible_index, last_visible_index)
    }
}

impl Widget for List<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        WidgetRef::render_ref(&self, area, buf);
    }
}

impl WidgetRef for List<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut state = ListState::default();
        StatefulWidgetRef::render_ref(self, area, buf, &mut state);
    }
}

impl StatefulWidget for List<'_> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidgetRef::render_ref(&self, area, buf, state);
    }
}

// Note: remove this when StatefulWidgetRef is stabilized and replace with the blanket impl
impl StatefulWidget for &List<'_> {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidgetRef::render_ref(self, area, buf, state);
    }
}

impl StatefulWidgetRef for List<'_> {
    type State = ListState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        self.block.render_ref(area, buf);
        let list_area = self.block.inner_if_some(area);

        if list_area.is_empty() {
            return;
        }

        if self.items.is_empty() {
            state.select(None);
            return;
        }

        // If the selected index is out of bounds, set it to the last item
        if state.selected.is_some_and(|s| s >= self.items.len()) {
            state.select(Some(self.items.len().saturating_sub(1)));
        }

        let list_height = list_area.height as usize;

        let (first_visible_index, last_visible_index) =
            self.get_items_bounds(state.selected, state.offset, list_height);

        // Important: this changes the state's offset to be the beginning of the now viewable items
        state.offset = first_visible_index;

        // Get our set highlighted symbol (if one was set)
        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = " ".repeat(highlight_symbol.width());

        let mut current_height = 0;
        let selection_spacing = self.highlight_spacing.should_add(state.selected.is_some());
        for (i, item) in self
            .items
            .iter()
            .enumerate()
            .skip(state.offset)
            .take(last_visible_index - first_visible_index)
        {
            let (x, y) = if self.direction == ListDirection::BottomToTop {
                current_height += item.height() as u16;
                (list_area.left(), list_area.bottom() - current_height)
            } else {
                let pos = (list_area.left(), list_area.top() + current_height);
                current_height += item.height() as u16;
                pos
            };

            let row_area = Rect {
                x,
                y,
                width: list_area.width,
                height: item.height() as u16,
            };

            let item_style = self.style.patch(item.style);
            buf.set_style(row_area, item_style);

            let is_selected = state.selected.map_or(false, |s| s == i);

            let item_area = if selection_spacing {
                let highlight_symbol_width = self.highlight_symbol.unwrap_or("").width() as u16;
                Rect {
                    x: row_area.x + highlight_symbol_width,
                    width: row_area.width.saturating_sub(highlight_symbol_width),
                    ..row_area
                }
            } else {
                row_area
            };
            item.content.clone().render(item_area, buf);

            for j in 0..item.content.height() {
                // if the item is selected, we need to display the highlight symbol:
                // - either for the first line of the item only,
                // - or for each line of the item if the appropriate option is set
                let symbol = if is_selected && (j == 0 || self.repeat_highlight_symbol) {
                    highlight_symbol
                } else {
                    &blank_symbol
                };
                if selection_spacing {
                    buf.set_stringn(
                        x,
                        y + j as u16,
                        symbol,
                        list_area.width as usize,
                        item_style,
                    );
                }
            }

            if is_selected {
                buf.set_style(row_area, self.highlight_style);
            }
        }
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
    use std::borrow::Cow;

    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn single_line_buf() -> Buffer {
        Buffer::empty(Rect::new(0, 0, 10, 1))
    }

    #[test]
    fn test_list_state_selected() {
        let mut state = ListState::default();
        assert_eq!(state.selected(), None);

        state.select(Some(1));
        assert_eq!(state.selected(), Some(1));

        state.select(None);
        assert_eq!(state.selected(), None);
    }

    #[test]
    fn test_list_state_select() {
        let mut state = ListState::default();
        assert_eq!(state.selected, None);
        assert_eq!(state.offset, 0);

        state.select(Some(2));
        assert_eq!(state.selected, Some(2));
        assert_eq!(state.offset, 0);

        state.select(None);
        assert_eq!(state.selected, None);
        assert_eq!(state.offset, 0);
    }

    #[test]
    fn test_list_state_navigation() {
        let mut state = ListState::default();
        state.select_first();
        assert_eq!(state.selected, Some(0));

        state.select_previous(); // should not go below 0
        assert_eq!(state.selected, Some(0));

        state.select_next();
        assert_eq!(state.selected, Some(1));

        state.select_previous();
        assert_eq!(state.selected, Some(0));

        state.select_last();
        assert_eq!(state.selected, Some(usize::MAX));

        state.select_next(); // should not go above usize::MAX
        assert_eq!(state.selected, Some(usize::MAX));

        state.select_previous();
        assert_eq!(state.selected, Some(usize::MAX - 1));

        state.select_next();
        assert_eq!(state.selected, Some(usize::MAX));

        let mut state = ListState::default();
        state.select_next();
        assert_eq!(state.selected, Some(0));

        let mut state = ListState::default();
        state.select_previous();
        assert_eq!(state.selected, Some(usize::MAX));
    }

    #[rstest]
    fn test_list_state_empty_list(mut single_line_buf: Buffer) {
        let mut state = ListState::default();

        let items: Vec<ListItem> = Vec::new();
        let list = List::new(items);
        state.select_first();
        StatefulWidget::render(list, single_line_buf.area, &mut single_line_buf, &mut state);
        assert_eq!(state.selected, None);
    }

    #[rstest]
    fn test_list_state_single_item(mut single_line_buf: Buffer) {
        let mut state = ListState::default();

        let items = vec![ListItem::new("Item 1")];
        let list = List::new(items);
        state.select_first();
        StatefulWidget::render(
            &list,
            single_line_buf.area,
            &mut single_line_buf,
            &mut state,
        );
        assert_eq!(state.selected, Some(0));

        state.select_last();
        StatefulWidget::render(
            &list,
            single_line_buf.area,
            &mut single_line_buf,
            &mut state,
        );
        assert_eq!(state.selected, Some(0));

        state.select_previous();
        StatefulWidget::render(
            &list,
            single_line_buf.area,
            &mut single_line_buf,
            &mut state,
        );
        assert_eq!(state.selected, Some(0));

        state.select_next();
        StatefulWidget::render(
            &list,
            single_line_buf.area,
            &mut single_line_buf,
            &mut state,
        );
        assert_eq!(state.selected, Some(0));
    }

    #[test]
    fn test_list_item_new_from_str() {
        let item = ListItem::new("Test item");
        assert_eq!(item.content, Text::from("Test item"));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_list_item_new_from_string() {
        let item = ListItem::new("Test item".to_string());
        assert_eq!(item.content, Text::from("Test item"));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_list_item_new_from_cow_str() {
        let item = ListItem::new(Cow::Borrowed("Test item"));
        assert_eq!(item.content, Text::from("Test item"));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_list_item_new_from_span() {
        let span = Span::styled("Test item", Style::default().fg(Color::Blue));
        let item = ListItem::new(span.clone());
        assert_eq!(item.content, Text::from(span));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_list_item_new_from_spans() {
        let spans = Line::from(vec![
            Span::styled("Test ", Style::default().fg(Color::Blue)),
            Span::styled("item", Style::default().fg(Color::Red)),
        ]);
        let item = ListItem::new(spans.clone());
        assert_eq!(item.content, Text::from(spans));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_list_item_new_from_vec_spans() {
        let lines = vec![
            Line::from(vec![
                Span::styled("Test ", Style::default().fg(Color::Blue)),
                Span::styled("item", Style::default().fg(Color::Red)),
            ]),
            Line::from(vec![
                Span::styled("Second ", Style::default().fg(Color::Green)),
                Span::styled("line", Style::default().fg(Color::Yellow)),
            ]),
        ];
        let item = ListItem::new(lines.clone());
        assert_eq!(item.content, Text::from(lines));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_str_into_list_item() {
        let s = "Test item";
        let item: ListItem = s.into();
        assert_eq!(item.content, Text::from(s));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_string_into_list_item() {
        let s = String::from("Test item");
        let item: ListItem = s.clone().into();
        assert_eq!(item.content, Text::from(s));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_span_into_list_item() {
        let s = Span::from("Test item");
        let item: ListItem = s.clone().into();
        assert_eq!(item.content, Text::from(s));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_vec_lines_into_list_item() {
        let lines = vec![Line::raw("l1"), Line::raw("l2")];
        let item: ListItem = lines.clone().into();
        assert_eq!(item.content, Text::from(lines));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_list_item_style() {
        let item = ListItem::new("Test item").style(Style::default().bg(Color::Red));
        assert_eq!(item.content, Text::from("Test item"));
        assert_eq!(item.style, Style::default().bg(Color::Red));
    }

    #[test]
    fn test_list_item_height() {
        let item = ListItem::new("Test item");
        assert_eq!(item.height(), 1);

        let item = ListItem::new("Test item\nSecond line");
        assert_eq!(item.height(), 2);
    }

    #[test]
    fn test_list_item_width() {
        let item = ListItem::new("Test item");
        assert_eq!(item.width(), 9);
    }

    /// helper method to render a widget to an empty buffer with the default state
    fn render_widget(widget: List<'_>, width: u16, height: u16) -> Buffer {
        let mut buffer = Buffer::empty(Rect::new(0, 0, width, height));
        Widget::render(widget, buffer.area, &mut buffer);
        buffer
    }

    /// helper method to render a widget to an empty buffer with a given state
    fn render_stateful_widget(
        widget: List<'_>,
        state: &mut ListState,
        width: u16,
        height: u16,
    ) -> Buffer {
        let mut buffer = Buffer::empty(Rect::new(0, 0, width, height));
        StatefulWidget::render(widget, buffer.area, &mut buffer, state);
        buffer
    }

    #[test]
    fn test_list_does_not_render_in_small_space() {
        let items = vec!["Item 0", "Item 1", "Item 2"];
        let list = List::new(items.clone()).highlight_symbol(">>");
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));

        // attempt to render into an area of the buffer with 0 width
        Widget::render(list.clone(), Rect::new(0, 0, 0, 3), &mut buffer);
        assert_eq!(&buffer, &Buffer::empty(buffer.area));

        // attempt to render into an area of the buffer with 0 height
        Widget::render(list.clone(), Rect::new(0, 0, 15, 0), &mut buffer);
        assert_eq!(&buffer, &Buffer::empty(buffer.area));

        let list = List::new(items)
            .highlight_symbol(">>")
            .block(Block::bordered());
        // attempt to render into an area of the buffer with zero height after
        // setting the block borders
        Widget::render(list, Rect::new(0, 0, 15, 2), &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┌─────────────┐",
            "└─────────────┘",
            "               ",
        ]);
        assert_eq!(buffer, expected,);
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_list_combinations() {
        #[track_caller]
        fn test_case_render<'line, Lines>(items: &[ListItem], expected: Lines)
        where
            Lines: IntoIterator,
            Lines::Item: Into<Line<'line>>,
        {
            let list = List::new(items.to_owned()).highlight_symbol(">>");
            let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));
            Widget::render(list, buffer.area, &mut buffer);
            assert_eq!(buffer, Buffer::with_lines(expected));
        }

        #[track_caller]
        fn test_case_render_stateful<'line, Lines>(
            items: &[ListItem],
            selected: Option<usize>,
            expected: Lines,
        ) where
            Lines: IntoIterator,
            Lines::Item: Into<Line<'line>>,
        {
            let list = List::new(items.to_owned()).highlight_symbol(">>");
            let mut state = ListState::default().with_selected(selected);
            let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));
            StatefulWidget::render(list, buffer.area, &mut buffer, &mut state);
            assert_eq!(buffer, Buffer::with_lines(expected));
        }

        let empty_items = Vec::new();
        let single_item = vec!["Item 0".into()];
        let multiple_items = vec!["Item 0".into(), "Item 1".into(), "Item 2".into()];
        let multi_line_items = vec!["Item 0\nLine 2".into(), "Item 1".into(), "Item 2".into()];

        // empty list
        test_case_render(
            &empty_items,
            [
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &empty_items,
            None,
            [
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &empty_items,
            Some(0),
            [
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );

        // single item
        test_case_render(
            &single_item,
            [
                "Item 0    ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &single_item,
            None,
            [
                "Item 0    ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &single_item,
            Some(0),
            [
                ">>Item 0  ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &single_item,
            Some(1),
            [
                ">>Item 0  ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );

        // multiple items
        test_case_render(
            &multiple_items,
            [
                "Item 0    ",
                "Item 1    ",
                "Item 2    ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &multiple_items,
            None,
            [
                "Item 0    ",
                "Item 1    ",
                "Item 2    ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &multiple_items,
            Some(0),
            [
                ">>Item 0  ",
                "  Item 1  ",
                "  Item 2  ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &multiple_items,
            Some(1),
            [
                "  Item 0  ",
                ">>Item 1  ",
                "  Item 2  ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &multiple_items,
            Some(3),
            [
                "  Item 0  ",
                "  Item 1  ",
                ">>Item 2  ",
                "          ",
                "          ",
            ],
        );

        // multi line items
        test_case_render(
            &multi_line_items,
            [
                "Item 0    ",
                "Line 2    ",
                "Item 1    ",
                "Item 2    ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &multi_line_items,
            None,
            [
                "Item 0    ",
                "Line 2    ",
                "Item 1    ",
                "Item 2    ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &multi_line_items,
            Some(0),
            [
                ">>Item 0  ",
                "  Line 2  ",
                "  Item 1  ",
                "  Item 2  ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &multi_line_items,
            Some(1),
            [
                "  Item 0  ",
                "  Line 2  ",
                ">>Item 1  ",
                "  Item 2  ",
                "          ",
            ],
        );
    }

    #[test]
    fn test_list_items_setter() {
        let list = List::default().items(["Item 0", "Item 1", "Item 2"]);
        let buffer = render_widget(list, 10, 5);
        let expected = Buffer::with_lines([
            "Item 0    ",
            "Item 1    ",
            "Item 2    ",
            "          ",
            "          ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_list_with_empty_strings() {
        let list = List::new(["Item 0", "", "", "Item 1", "Item 2"])
            .block(Block::bordered().title("List"));
        let buffer = render_widget(list, 10, 7);
        let expected = Buffer::with_lines([
            "┌List────┐",
            "│Item 0  │",
            "│        │",
            "│        │",
            "│Item 1  │",
            "│Item 2  │",
            "└────────┘",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_collect_list_from_iterator() {
        let collected: List = (0..3).map(|i| format!("Item{i}")).collect();
        let expected = List::new(["Item0", "Item1", "Item2"]);
        assert_eq!(collected, expected);
    }

    #[test]
    fn test_list_block() {
        let list = List::new(["Item 0", "Item 1", "Item 2"]).block(Block::bordered().title("List"));
        let buffer = render_widget(list, 10, 7);
        let expected = Buffer::with_lines([
            "┌List────┐",
            "│Item 0  │",
            "│Item 1  │",
            "│Item 2  │",
            "│        │",
            "│        │",
            "└────────┘",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_list_style() {
        let list = List::new(["Item 0", "Item 1", "Item 2"]).style(Style::default().fg(Color::Red));
        let buffer = render_widget(list, 10, 5);
        let expected = Buffer::with_lines([
            "Item 0    ".red(),
            "Item 1    ".red(),
            "Item 2    ".red(),
            "          ".red(),
            "          ".red(),
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_list_highlight_symbol_and_style() {
        let list = List::new(["Item 0", "Item 1", "Item 2"])
            .highlight_symbol(">>")
            .highlight_style(Style::default().fg(Color::Yellow));
        let mut state = ListState::default();
        state.select(Some(1));
        let buffer = render_stateful_widget(list, &mut state, 10, 5);
        let expected = Buffer::with_lines([
            "  Item 0  ".into(),
            ">>Item 1  ".yellow(),
            "  Item 2  ".into(),
            "          ".into(),
            "          ".into(),
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_list_highlight_spacing_default_whenselected() {
        // when not selected
        {
            let list = List::new(["Item 0", "Item 1", "Item 2"]).highlight_symbol(">>");
            let mut state = ListState::default();
            let buffer = render_stateful_widget(list, &mut state, 10, 5);
            let expected = Buffer::with_lines([
                "Item 0    ",
                "Item 1    ",
                "Item 2    ",
                "          ",
                "          ",
            ]);
            assert_eq!(buffer, expected);
        }

        // when selected
        {
            let list = List::new(["Item 0", "Item 1", "Item 2"]).highlight_symbol(">>");
            let mut state = ListState::default();
            state.select(Some(1));
            let buffer = render_stateful_widget(list, &mut state, 10, 5);
            let expected = Buffer::with_lines([
                "  Item 0  ",
                ">>Item 1  ",
                "  Item 2  ",
                "          ",
                "          ",
            ]);
            assert_eq!(buffer, expected);
        }
    }

    #[test]
    fn test_list_highlight_spacing_default_always() {
        // when not selected
        {
            let list = List::new(["Item 0", "Item 1", "Item 2"])
                .highlight_symbol(">>")
                .highlight_spacing(HighlightSpacing::Always);
            let mut state = ListState::default();
            let buffer = render_stateful_widget(list, &mut state, 10, 5);
            let expected = Buffer::with_lines([
                "  Item 0  ",
                "  Item 1  ",
                "  Item 2  ",
                "          ",
                "          ",
            ]);
            assert_eq!(buffer, expected);
        }

        // when selected
        {
            let list = List::new(["Item 0", "Item 1", "Item 2"])
                .highlight_symbol(">>")
                .highlight_spacing(HighlightSpacing::Always);
            let mut state = ListState::default();
            state.select(Some(1));
            let buffer = render_stateful_widget(list, &mut state, 10, 5);
            let expected = Buffer::with_lines([
                "  Item 0  ",
                ">>Item 1  ",
                "  Item 2  ",
                "          ",
                "          ",
            ]);
            assert_eq!(buffer, expected);
        }
    }

    #[test]
    fn test_list_highlight_spacing_default_never() {
        // when not selected
        {
            let list = List::new(["Item 0", "Item 1", "Item 2"])
                .highlight_symbol(">>")
                .highlight_spacing(HighlightSpacing::Never);
            let mut state = ListState::default();
            let buffer = render_stateful_widget(list, &mut state, 10, 5);
            let expected = Buffer::with_lines([
                "Item 0    ",
                "Item 1    ",
                "Item 2    ",
                "          ",
                "          ",
            ]);
            assert_eq!(buffer, expected);
        }

        // when selected
        {
            let list = List::new(["Item 0", "Item 1", "Item 2"])
                .highlight_symbol(">>")
                .highlight_spacing(HighlightSpacing::Never);
            let mut state = ListState::default();
            state.select(Some(1));
            let buffer = render_stateful_widget(list, &mut state, 10, 5);
            let expected = Buffer::with_lines([
                "Item 0    ",
                "Item 1    ",
                "Item 2    ",
                "          ",
                "          ",
            ]);
            assert_eq!(buffer, expected);
        }
    }

    #[test]
    fn test_list_repeat_highlight_symbol() {
        let list = List::new(["Item 0\nLine 2", "Item 1", "Item 2"])
            .highlight_symbol(">>")
            .highlight_style(Style::default().fg(Color::Yellow))
            .repeat_highlight_symbol(true);
        let mut state = ListState::default();
        state.select(Some(0));
        let buffer = render_stateful_widget(list, &mut state, 10, 5);
        let expected = Buffer::with_lines([
            ">>Item 0  ".yellow(),
            ">>Line 2  ".yellow(),
            "  Item 1  ".into(),
            "  Item 2  ".into(),
            "          ".into(),
        ]);
        assert_eq!(buffer, expected);
    }

    #[rstest]
    #[case::top_to_bottom(ListDirection::TopToBottom, [
        "Item 0    ",
        "Item 1    ",
        "Item 2    ",
        "          ",
    ])]
    #[case::top_to_bottom(ListDirection::BottomToTop, [
        "          ",
        "Item 2    ",
        "Item 1    ",
        "Item 0    ",
    ])]
    fn list_direction<'line, Lines>(#[case] direction: ListDirection, #[case] expected: Lines)
    where
        Lines: IntoIterator,
        Lines::Item: Into<Line<'line>>,
    {
        let list = List::new(["Item 0", "Item 1", "Item 2"]).direction(direction);
        let buffer = render_widget(list, 10, 4);
        assert_eq!(buffer, Buffer::with_lines(expected));
    }

    #[test]
    fn test_list_truncate_items() {
        let list = List::new(["Item 0", "Item 1", "Item 2", "Item 3", "Item 4"]);
        let buffer = render_widget(list, 10, 3);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "Item 0    ",
            "Item 1    ",
            "Item 2    ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_offset_renders_shifted() {
        let list = List::new([
            "Item 0", "Item 1", "Item 2", "Item 3", "Item 4", "Item 5", "Item 6",
        ]);
        let mut state = ListState::default().with_offset(3);
        let buffer = render_stateful_widget(list, &mut state, 6, 3);

        let expected = Buffer::with_lines(["Item 3", "Item 4", "Item 5"]);
        assert_eq!(buffer, expected);
    }

    #[rstest]
    #[case(None, [
        "Item 0 with a v",
        "Item 1         ",
        "Item 2         ",
    ])]
    #[case(Some(0), [
        ">>Item 0 with a",
        "  Item 1       ",
        "  Item 2       ",
    ])]
    fn test_list_long_lines<'line, Lines>(#[case] selected: Option<usize>, #[case] expected: Lines)
    where
        Lines: IntoIterator,
        Lines::Item: Into<Line<'line>>,
    {
        let items = [
            "Item 0 with a very long line that will be truncated",
            "Item 1",
            "Item 2",
        ];
        let list = List::new(items).highlight_symbol(">>");
        let mut state = ListState::default().with_selected(selected);
        let buffer = render_stateful_widget(list, &mut state, 15, 3);
        assert_eq!(buffer, Buffer::with_lines(expected));
    }

    #[test]
    fn test_list_selected_item_ensures_selected_item_is_visible_when_offset_is_before_visible_range(
    ) {
        let items = [
            "Item 0", "Item 1", "Item 2", "Item 3", "Item 4", "Item 5", "Item 6",
        ];
        let list = List::new(items).highlight_symbol(">>");
        // Set the initial visible range to items 3, 4, and 5
        let mut state = ListState::default().with_selected(Some(1)).with_offset(3);
        let buffer = render_stateful_widget(list, &mut state, 10, 3);

        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            ">>Item 1  ",
            "  Item 2  ",
            "  Item 3  ",
        ]);

        assert_eq!(buffer, expected);
        assert_eq!(state.selected, Some(1));
        assert_eq!(
            state.offset, 1,
            "did not scroll the selected item into view"
        );
    }

    #[test]
    fn test_list_selected_item_ensures_selected_item_is_visible_when_offset_is_after_visible_range()
    {
        let items = [
            "Item 0", "Item 1", "Item 2", "Item 3", "Item 4", "Item 5", "Item 6",
        ];
        let list = List::new(items).highlight_symbol(">>");
        // Set the initial visible range to items 3, 4, and 5
        let mut state = ListState::default().with_selected(Some(6)).with_offset(3);
        let buffer = render_stateful_widget(list, &mut state, 10, 3);

        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "  Item 4  ",
            "  Item 5  ",
            ">>Item 6  ",
        ]);

        assert_eq!(buffer, expected);
        assert_eq!(state.selected, Some(6));
        assert_eq!(
            state.offset, 4,
            "did not scroll the selected item into view"
        );
    }

    #[test]
    fn list_can_be_stylized() {
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

    #[test]
    fn list_item_can_be_stylized() {
        assert_eq!(
            ListItem::new("").black().on_white().bold().not_dim().style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::DIM)
        );
    }

    #[test]
    fn test_render_list_with_alignment() {
        let list = List::new([
            Line::from("Left").alignment(Alignment::Left),
            Line::from("Center").alignment(Alignment::Center),
            Line::from("Right").alignment(Alignment::Right),
        ]);
        let buffer = render_widget(list, 10, 4);
        let expected = Buffer::with_lines(["Left      ", "  Center  ", "     Right", ""]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_list_alignment_odd_line_odd_area() {
        let list = List::new([
            Line::from("Odd").alignment(Alignment::Left),
            Line::from("Even").alignment(Alignment::Center),
            Line::from("Width").alignment(Alignment::Right),
        ]);
        let buffer = render_widget(list, 7, 4);
        let expected = Buffer::with_lines(["Odd    ", " Even  ", "  Width", ""]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_list_alignment_even_line_even_area() {
        let list = List::new([
            Line::from("Odd").alignment(Alignment::Left),
            Line::from("Even").alignment(Alignment::Center),
            Line::from("Width").alignment(Alignment::Right),
        ]);
        let buffer = render_widget(list, 6, 4);
        let expected = Buffer::with_lines(["Odd   ", " Even ", " Width", ""]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_list_alignment_odd_line_even_area() {
        let list = List::new([
            Line::from("Odd").alignment(Alignment::Left),
            Line::from("Even").alignment(Alignment::Center),
            Line::from("Width").alignment(Alignment::Right),
        ]);
        let buffer = render_widget(list, 8, 4);
        let expected = Buffer::with_lines(["Odd     ", "  Even  ", "   Width", ""]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_list_alignment_even_line_odd_area() {
        let list = List::new([
            Line::from("Odd").alignment(Alignment::Left),
            Line::from("Even").alignment(Alignment::Center),
            Line::from("Width").alignment(Alignment::Right),
        ]);
        let buffer = render_widget(list, 6, 4);
        let expected = Buffer::with_lines(["Odd   ", " Even ", " Width", ""]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_list_alignment_zero_line_width() {
        let list = List::new([Line::from("This line has zero width").alignment(Alignment::Center)]);
        let buffer = render_widget(list, 0, 2);
        assert_eq!(buffer, Buffer::with_lines([""; 2]));
    }

    #[test]
    fn test_render_list_alignment_zero_area_width() {
        let list = List::new([Line::from("Text").alignment(Alignment::Left)]);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 1));
        Widget::render(list, Rect::new(0, 0, 4, 0), &mut buffer);
        assert_eq!(buffer, Buffer::with_lines(["    "]));
    }

    #[test]
    fn test_render_list_alignment_line_less_than_width() {
        let list = List::new([Line::from("Small").alignment(Alignment::Center)]);
        let buffer = render_widget(list, 10, 2);
        let expected = Buffer::with_lines(["  Small   ", ""]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_list_alignment_line_equal_to_width() {
        let list = List::new([Line::from("Exact").alignment(Alignment::Left)]);
        let buffer = render_widget(list, 5, 2);
        assert_eq!(buffer, Buffer::with_lines(["Exact", ""]));
    }

    #[test]
    fn test_render_list_alignment_line_greater_than_width() {
        let list = List::new([Line::from("Large line").alignment(Alignment::Left)]);
        let buffer = render_widget(list, 5, 2);
        assert_eq!(buffer, Buffer::with_lines(["Large", ""]));
    }

    #[rstest]
    #[case::no_padding(
        4,
        2, // Offset
        0, // Padding
        Some(2), // Selected
        [
            ">> Item 2 ",
            "   Item 3 ",
            "   Item 4 ",
            "   Item 5 ",
        ]
    )]
    #[case::one_before(
        4,
        2, // Offset
        1, // Padding
        Some(2), // Selected
        [
            "   Item 1 ",
            ">> Item 2 ",
            "   Item 3 ",
            "   Item 4 ",
        ]
    )]
    #[case::one_after(
        4,
        1, // Offset
        1, // Padding
        Some(4), // Selected
        [
            "   Item 2 ",
            "   Item 3 ",
            ">> Item 4 ",
            "   Item 5 ",
        ]
    )]
    #[case::check_padding_overflow(
        4,
        1, // Offset
        2, // Padding
        Some(4), // Selected
        [
            "   Item 2 ",
            "   Item 3 ",
            ">> Item 4 ",
            "   Item 5 ",
        ]
    )]
    #[case::no_padding_offset_behavior(
        5, // Render Area Height
        2, // Offset
        0, // Padding
        Some(3), // Selected
        [
            "   Item 2 ",
            ">> Item 3 ",
            "   Item 4 ",
            "   Item 5 ",
            "          ",
        ]
    )]
    #[case::two_before(
        5, // Render Area Height
        2, // Offset
        2, // Padding
        Some(3), // Selected
        [
            "   Item 1 ",
            "   Item 2 ",
            ">> Item 3 ",
            "   Item 4 ",
            "   Item 5 ",
        ]
    )]
    #[case::keep_selected_visible(
        4,
        0, // Offset
        4, // Padding
        Some(1), // Selected
        [
            "   Item 0 ",
            ">> Item 1 ",
            "   Item 2 ",
            "   Item 3 ",
        ]
    )]
    fn test_padding<'line, Lines>(
        #[case] render_height: u16,
        #[case] offset: usize,
        #[case] padding: usize,
        #[case] selected: Option<usize>,
        #[case] expected: Lines,
    ) where
        Lines: IntoIterator,
        Lines::Item: Into<Line<'line>>,
    {
        let backend = backend::TestBackend::new(10, render_height);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut state = ListState::default();

        *state.offset_mut() = offset;
        state.select(selected);

        let list = List::new(["Item 0", "Item 1", "Item 2", "Item 3", "Item 4", "Item 5"])
            .scroll_padding(padding)
            .highlight_symbol(">> ");
        terminal
            .draw(|f| {
                let size = f.size();
                f.render_stateful_widget(list, size, &mut state);
            })
            .unwrap();
        terminal.backend().assert_buffer_lines(expected);
    }

    /// If there isn't enough room for the selected item and the requested padding the list can jump
    /// up and down every frame if something isn't done about it. This code tests to make sure that
    /// isn't currently happening
    #[test]
    fn test_padding_flicker() {
        let backend = backend::TestBackend::new(10, 5);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut state = ListState::default();

        *state.offset_mut() = 2;
        state.select(Some(4));

        let items = [
            "Item 0", "Item 1", "Item 2", "Item 3", "Item 4", "Item 5", "Item 6", "Item 7",
        ];
        let list = List::new(items).scroll_padding(3).highlight_symbol(">> ");

        terminal
            .draw(|f| {
                let size = f.size();
                f.render_stateful_widget(&list, size, &mut state);
            })
            .unwrap();

        let offset_after_render = state.offset();

        terminal
            .draw(|f| {
                let size = f.size();
                f.render_stateful_widget(&list, size, &mut state);
            })
            .unwrap();

        // Offset after rendering twice should remain the same as after once
        assert_eq!(offset_after_render, state.offset());
    }

    #[test]
    fn test_padding_inconsistent_item_sizes() {
        let backend = backend::TestBackend::new(10, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut state = ListState::default().with_offset(0).with_selected(Some(3));

        let items = [
            ListItem::new("Item 0"),
            ListItem::new("Item 1"),
            ListItem::new("Item 2"),
            ListItem::new("Item 3"),
            ListItem::new("Item 4\nTest\nTest"),
            ListItem::new("Item 5"),
        ];
        let list = List::new(items).scroll_padding(1).highlight_symbol(">> ");

        terminal
            .draw(|f| {
                let size = f.size();
                f.render_stateful_widget(list, size, &mut state);
            })
            .unwrap();

        #[rustfmt::skip]
        let expected = [
            "   Item 1 ",
            "   Item 2 ",
            ">> Item 3 ",
        ];
        terminal.backend().assert_buffer_lines(expected);
    }

    // Tests to make sure when it's pushing back the first visible index value that it doesnt
    // include an item that's too large
    #[test]
    fn test_padding_offset_pushback_break() {
        let backend = backend::TestBackend::new(10, 4);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut state = ListState::default();

        *state.offset_mut() = 1;
        state.select(Some(2));

        let items = [
            ListItem::new("Item 0\nTest\nTest"),
            ListItem::new("Item 1"),
            ListItem::new("Item 2"),
            ListItem::new("Item 3"),
        ];
        let list = List::new(items).scroll_padding(2).highlight_symbol(">> ");

        terminal
            .draw(|f| {
                let size = f.size();
                f.render_stateful_widget(list, size, &mut state);
            })
            .unwrap();

        terminal.backend().assert_buffer_lines([
            "   Item 1 ",
            ">> Item 2 ",
            "   Item 3 ",
            "          ",
        ]);
    }

    /// Regression test for a bug where highlight symbol being greater than width caused a panic due
    /// to subtraction with underflow.
    ///
    /// See [#949](https://github.com/ratatui-org/ratatui/pull/949) for details
    #[rstest]
    #[case::under(">>>>", "Item1", ">>>>Item1 ")] // enough space to render the highlight symbol
    #[case::exact(">>>>>", "Item1", ">>>>>Item1")] // exact space to render the highlight symbol
    #[case::overflow(">>>>>>", "Item1", ">>>>>>Item")] // not enough space
    fn highlight_symbol_overflow(
        #[case] highlight_symbol: &str,
        #[case] item: &str,
        #[case] expected: &str,
        mut single_line_buf: Buffer,
    ) {
        let list = List::new([item]).highlight_symbol(highlight_symbol);
        let mut state = ListState::default();
        state.select(Some(0));
        StatefulWidget::render(list, single_line_buf.area, &mut single_line_buf, &mut state);
        assert_eq!(single_line_buf, Buffer::with_lines([expected]));
    }
}
