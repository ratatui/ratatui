use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::ops::Deref;

use ratatui_core::style::{Style, Styled};

use super::Cell;

/// A single row of data to be displayed in a [`Table`] widget.
///
/// A `Row` is a collection of [`Cell`]s.
///
/// By default, a row has a height of 1 but you can change this using [`Row::height`].
///
/// You can set the style of the entire row using [`Row::style`]. This [`Style`] will be combined
/// with the [`Style`] of each individual [`Cell`] by adding the [`Style`] of the [`Cell`] to the
/// [`Style`] of the [`Row`].
///
/// # Examples
///
/// You can create `Row`s from simple strings.
///
/// ```rust
/// use ratatui::widgets::Row;
///
/// Row::new(vec!["Cell1", "Cell2", "Cell3"]);
/// ```
///
/// If you need a bit more control over individual cells, you can explicitly create [`Cell`]s:
///
/// ```rust
/// use ratatui::style::Stylize;
/// use ratatui::widgets::{Cell, Row};
///
/// Row::new(vec![
///     Cell::from("Cell1"),
///     Cell::from("Cell2").red().italic(),
/// ]);
/// ```
///
/// You can also construct a row from any type that can be converted into [`Text`]:
///
/// ```rust
/// use std::borrow::Cow;
///
/// use ratatui::widgets::{Cell, Row};
///
/// Row::new(vec![
///     Cow::Borrowed("hello"),
///     Cow::Owned("world".to_uppercase()),
/// ]);
/// ```
///
/// An iterator whose item type is convertible into [`Text`] can be collected into a row.
///
/// ```rust
/// use ratatui::widgets::Row;
///
/// (0..10).map(|i| format!("{i}")).collect::<Row>();
/// ```
///
/// `Row` implements [`Styled`] which means you can use style shorthands from the [`Stylize`] trait
/// to set the style of the row concisely.
///
/// ```rust
/// use ratatui::style::Stylize;
/// use ratatui::widgets::Row;
///
/// let cells = vec!["Cell1", "Cell2", "Cell3"];
/// Row::new(cells).red().italic();
/// ```
///
/// [`Table`]: super::Table
/// [`Text`]: ratatui_core::text::Text
/// [`Stylize`]: ratatui_core::style::Stylize
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Row<'a> {
    pub(crate) cells: Cow<'a, [Cell<'a>]>,
    pub(crate) height: u16,
    pub(crate) top_margin: u16,
    pub(crate) bottom_margin: u16,
    pub(crate) style: Style,
}

impl<'a> Row<'a> {
    /// Creates a new [`Row`]
    ///
    /// The `cells` parameter accepts any value that can be converted into an iterator of anything
    /// that can be converted into a [`Cell`] (e.g. `Vec<&str>`, `&[Cell<'a>]`, `Vec<String>`, etc.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::{Cell, Row};
    ///
    /// let row = Row::new(vec!["Cell 1", "Cell 2", "Cell 3"]);
    /// let row = Row::new(vec![
    ///     Cell::new("Cell 1"),
    ///     Cell::new("Cell 2"),
    ///     Cell::new("Cell 3"),
    /// ]);
    /// ```
    #[must_use = "constructor"]
    pub fn new<T>(cells: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Cell<'a>>,
    {
        Self {
            cells: cells.into_iter().map(Into::into).collect(),
            height: 1,
            ..Default::default()
        }
    }

    /// Set the cells of the [`Row`]
    ///
    /// The `cells` parameter accepts any value that can be converted into an iterator of anything
    /// that can be converted into a [`Cell`] (e.g. `Vec<&str>`, `&[Cell<'a>]`, `Vec<String>`, etc.)
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::{Cell, Row};
    ///
    /// let row = Row::default().cells(vec!["Cell 1", "Cell 2", "Cell 3"]);
    /// let row = Row::default().cells(vec![
    ///     Cell::new("Cell 1"),
    ///     Cell::new("Cell 2"),
    ///     Cell::new("Cell 3"),
    /// ]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn cells<T>(mut self, cells: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Cell<'a>>,
    {
        self.cells = cells.into_iter().map(Into::into).collect();
        self
    }

    /// Set the fixed height of the [`Row`]
    ///
    /// Any [`Cell`] whose content has more lines than this height will see its content truncated.
    ///
    /// By default, the height is `1`.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::Row;
    ///
    /// let cells = vec!["Cell 1\nline 2", "Cell 2", "Cell 3"];
    /// let row = Row::new(cells).height(2);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    /// Set the top margin. By default, the top margin is `0`.
    ///
    /// The top margin is the number of blank lines to be displayed before the row.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::Row;
    /// let cells = vec!["Cell 1", "Cell 2", "Cell 3"];
    ///
    /// let row = Row::default().top_margin(1);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn top_margin(mut self, margin: u16) -> Self {
        self.top_margin = margin;
        self
    }

    /// Set the bottom margin. By default, the bottom margin is `0`.
    ///
    /// The bottom margin is the number of blank lines to be displayed after the row.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::Row;
    ///
    /// let cells = vec!["Cell 1", "Cell 2", "Cell 3"];
    /// let row = Row::default().bottom_margin(1);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn bottom_margin(mut self, margin: u16) -> Self {
        self.bottom_margin = margin;
        self
    }

    /// Set the [`Style`] of the entire row
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This [`Style`] can be overridden by the [`Style`] of a any individual [`Cell`] or by their
    /// [`Text`] content.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::style::{Style, Stylize};
    /// use ratatui::widgets::Row;
    /// let cells = vec!["Cell 1", "Cell 2", "Cell 3"];
    /// let row = Row::new(cells).style(Style::new().red().italic());
    /// ```
    ///
    /// `Row` also implements the [`Styled`] trait, which means you can use style shorthands from
    /// the [`Stylize`] trait to set the style of the widget more concisely.
    ///
    /// ```rust
    /// use ratatui::style::Stylize;
    /// use ratatui::widgets::Row;
    ///
    /// let cells = vec!["Cell 1", "Cell 2", "Cell 3"];
    /// let row = Row::new(cells).red().italic();
    /// ```
    ///
    /// [`Color`]: ratatui_core::style::Color
    /// [`Stylize`]: ratatui_core::style::Stylize
    /// [`Text`]: ratatui_core::text::Text
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Return whether the row owns its cells.
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui::widgets::{Cell, Row};
    ///
    /// let cells = vec!["Cell 1", "Cell 2", "Cell 3"];
    /// let row = Row::new(cells);
    /// assert!(row.is_owned());
    /// ```
    pub const fn is_owned(&self) -> bool {
        match self.cells {
            Cow::Borrowed(_) => false,
            Cow::Owned(_) => true,
        }
    }

    /// Return whether the row borrows its cells.
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui::widgets::{Cell, Row};
    ///
    /// let cells = [
    ///     Cell::from("Cell 1"),
    ///     Cell::from("Cell 2"),
    ///     Cell::from("Cell 3"),
    /// ];
    /// let row = Row::from(&cells);
    /// assert!(row.is_borrowed());
    /// ```
    pub const fn is_borrowed(&self) -> bool {
        !self.is_owned()
    }
}

// private methods for rendering
impl Row<'_> {
    /// Returns the total height of the row.
    pub(crate) const fn height_with_margin(&self) -> u16 {
        self.height
            .saturating_add(self.top_margin)
            .saturating_add(self.bottom_margin)
    }
}

impl Styled for Row<'_> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl<'a> From<Vec<Cell<'a>>> for Row<'a> {
    fn from(value: Vec<Cell<'a>>) -> Self {
        Self {
            cells: Cow::Owned(value),
            ..Self::default()
        }
    }
}

impl<'a> From<&'a Vec<Cell<'a>>> for Row<'a> {
    fn from(value: &'a Vec<Cell<'a>>) -> Self {
        Self {
            cells: Cow::Borrowed(value),
            ..Self::default()
        }
    }
}

impl<'a> From<&'a [Cell<'a>]> for Row<'a> {
    fn from(value: &'a [Cell<'a>]) -> Self {
        Self {
            cells: Cow::Borrowed(value),
            ..Self::default()
        }
    }
}

impl<'a, const N: usize> From<&'a [Cell<'a>; N]> for Row<'a> {
    fn from(value: &'a [Cell<'a>; N]) -> Self {
        Self {
            cells: Cow::Borrowed(value),
            ..Self::default()
        }
    }
}

impl<'a, Item> FromIterator<Item> for Row<'a>
where
    Item: Into<Cell<'a>>,
{
    fn from_iter<IterCells: IntoIterator<Item = Item>>(cells: IterCells) -> Self {
        Self::new(cells)
    }
}

impl<'a> From<Cow<'a, [Cell<'a>]>> for Row<'a> {
    fn from(value: Cow<'a, [Cell<'a>]>) -> Self {
        Self {
            cells: value,
            ..Self::default()
        }
    }
}

/// A copy-on-write container for a row of cells.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RowCow<'a> {
    /// A borrowed row of cells.
    Borrowed(&'a Row<'a>),
    /// An owned row of cells.
    Owned(Row<'a>),
}

impl<'a> Deref for RowCow<'a> {
    type Target = Row<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            RowCow::Borrowed(row) => row,
            RowCow::Owned(row) => row,
        }
    }
}

impl<'a> From<&'a Row<'a>> for RowCow<'a> {
    fn from(value: &'a Row<'a>) -> Self {
        RowCow::Borrowed(value)
    }
}

impl<'a> From<Row<'a>> for RowCow<'a> {
    fn from(value: Row<'a>) -> Self {
        RowCow::Owned(value)
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use ratatui_core::style::{Color, Modifier, Stylize};

    use super::*;

    #[test]
    fn new() {
        let cells = vec![Cell::from("")];
        let row = Row::new(cells.clone());
        assert_eq!(row.cells, cells);
    }

    #[test]
    fn collect() {
        let cells = vec![Cell::from("")];
        let row: Row = cells.iter().cloned().collect();
        assert_eq!(row.cells, cells);
    }

    #[test]
    fn cells() {
        let cells = vec![Cell::from("")];
        let row = Row::default().cells(cells.clone());
        assert_eq!(row.cells, cells);
    }

    #[test]
    fn from_vec() {
        let cells = vec![Cell::from("")];
        let row = Row::from(cells);
        assert_eq!(row.cells, Cow::<[Cell]>::Owned(vec![Cell::new("")]));
    }

    #[test]
    fn from_vec_ref() {
        let cells = vec![Cell::from("")];
        let row = Row::from(&cells);
        assert_eq!(row.cells, Cow::Borrowed(&[Cell::new("")]));
    }

    #[test]
    fn from_slice() {
        let cells = vec![Cell::from("")];
        let row = Row::from(cells.as_slice());
        assert_eq!(row.cells, Cow::Borrowed(&[Cell::new("")]));
    }

    #[test]
    fn from_array() {
        let cells = [Cell::from("")];
        let row = Row::from(&cells);
        assert_eq!(row.cells, Cow::Borrowed(&[Cell::new("")]));
    }

    #[test]
    fn height() {
        let row = Row::default().height(2);
        assert_eq!(row.height, 2);
    }

    #[test]
    fn top_margin() {
        let row = Row::default().top_margin(1);
        assert_eq!(row.top_margin, 1);
    }

    #[test]
    fn bottom_margin() {
        let row = Row::default().bottom_margin(1);
        assert_eq!(row.bottom_margin, 1);
    }

    #[test]
    fn style() {
        let style = Style::default().red().italic();
        let row = Row::default().style(style);
        assert_eq!(row.style, style);
    }

    #[test]
    fn stylize() {
        assert_eq!(
            Row::new(vec![Cell::from("")])
                .black()
                .on_white()
                .bold()
                .not_italic()
                .style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::ITALIC)
        );
    }

    #[test]
    fn from_cow_borrowed() {
        let cells: [_; 1] = [Cell::from("Item0")];
        let cow: Cow<[Cell<'_>]> = Cow::Borrowed(&cells);
        let row = Row::from(cow);
        assert_eq!(row.cells, Cow::Borrowed(&[Cell::from("Item0")]));
    }

    #[test]
    fn from_cow_owned() {
        let cells: Vec<_> = [Cell::from("Item0")].to_vec();
        let cow: Cow<[Cell<'_>]> = Cow::Owned(cells);
        let row = Row::from(cow);
        assert_eq!(row.cells, Cow::<[Cell]>::Owned(vec![Cell::from("Item0")]));
    }
}
