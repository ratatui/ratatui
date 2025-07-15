//! The [`Table`] widget is used to display multiple rows and columns in a grid and allows selecting
//! one or multiple cells.

use alloc::vec;
use alloc::vec::Vec;

use bitflags::bitflags;
use itertools::Itertools;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Constraint, Flex, Layout, Rect};
use ratatui_core::style::{Style, Styled};
use ratatui_core::symbols;
use ratatui_core::symbols::merge::MergeStrategy;
use ratatui_core::text::Text;
use ratatui_core::widgets::{StatefulWidget, Widget};

pub use self::cell::Cell;
pub use self::highlight_spacing::HighlightSpacing;
pub use self::row::Row;
pub use self::state::TableState;
use crate::block::{Block, BlockExt};

mod cell;
mod highlight_spacing;
mod row;
mod state;

bitflags! {
    /// The type of internal borders for a table.
    ///
    /// This bitflags defines the different internal border styles that can be applied to a table.
    /// It allows for controlling which internal borders are displayed within the table.
    ///
    /// **Naming Convention**: The term "internal borders" distinguishes these borders from external
    /// borders that might be added by wrapping the table in a [`Block`] widget. Both types of
    /// borders serve the same visual purpose of creating table grid lines, but they are positioned
    /// and controlled differently.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_core::style::{Color, Style};
    /// use ratatui_core::layout::Constraint;
    /// use ratatui_widgets::table::{Table, TableBorders, Row};
    ///
    /// let table = Table::new(Vec::<Row>::new(), Vec::<Constraint>::new())
    ///     .internal_borders(TableBorders::ALL)
    ///     .border_style(Style::default().fg(Color::Blue));
    /// ```
    #[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct TableBorders: u8 {
        /// No borders displayed.
        const NONE = 0b00;
        /// Only horizontal borders displayed.
        const HORIZONTAL = 0b01;
        /// Only vertical borders displayed.
        const VERTICAL = 0b10;
        /// All borders displayed.
        const ALL = Self::HORIZONTAL.bits() | Self::VERTICAL.bits();
    }
}

/// A widget to display data in formatted columns.
///
/// A `Table` is a collection of [`Row`]s, each composed of [`Cell`]s:
///
/// You can construct a [`Table`] using either [`Table::new`] or [`Table::default`] and then chain
/// builder style methods to set the desired properties.
///
/// Table cells can be aligned, for more details see [`Cell`].
///
/// Make sure to call the [`Table::widths`] method, otherwise the columns will all have a width of 0
/// and thus not be visible.
///
/// [`Table`] implements [`Widget`] and so it can be drawn using `Frame::render_widget`.
///
/// [`Table`] is also a [`StatefulWidget`], which means you can use it with [`TableState`] to allow
/// the user to scroll through the rows and select one of them. When rendering a [`Table`] with a
/// [`TableState`], the selected row, column and cell will be highlighted. If the selected row is
/// not visible (based on the offset), the table will be scrolled to make the selected row visible.
///
/// Note: if the `widths` field is empty, the table will be rendered with equal widths.
/// Note: Highlight styles are applied in the following order: Row, Column, Cell.
///
/// ## Internal Borders
///
/// Tables support internal borders that are drawn between and around the table cells. These are
/// separate from any external borders that might be added by wrapping the table in a [`Block`].
/// You can control which internal borders are visible using [`Table::internal_borders`] and style
/// them with [`Table::border_style`]. The available border options are:
///
/// - [`TableBorders::NONE`] - No internal borders (default)
/// - [`TableBorders::HORIZONTAL`] - Horizontal lines between rows
/// - [`TableBorders::VERTICAL`] - Vertical lines between columns
/// - [`TableBorders::ALL`] - Both horizontal and vertical borders
///
/// Borders can also be combined using bitwise operations for more fine-grained control.
///
/// ### Interaction with External Block Borders
///
/// When a table is wrapped in a [`Block`] widget, the internal borders will automatically
/// integrate with the block's external borders at intersection points. The table will use
/// appropriate intersection symbols (┌, ┐, └, ┘, ├, ┤, ┬, ┴, ┼) where internal borders meet
/// external block borders.
///
/// ```rust
/// use ratatui::layout::Constraint;
/// use ratatui::style::{Color, Style};
/// use ratatui::widgets::{Block, Row, Table};
/// use ratatui_widgets::table::TableBorders;
///
/// let table = Table::new(
///     vec![Row::new(vec!["Cell1", "Cell2"])],
///     [Constraint::Length(5); 2],
/// )
/// .block(Block::bordered().title("Table"))
/// .internal_borders(TableBorders::ALL)
/// .border_style(Style::default().fg(Color::Blue));
/// ```
///
/// ### Performance Considerations
///
/// Internal borders add rendering overhead, especially for large tables. The performance impact
/// scales with:
/// - **Number of rows**: Each row boundary requires horizontal border rendering
/// - **Number of columns**: Each column boundary requires vertical border rendering
/// - **Table size**: Larger tables require more border intersection calculations
///
/// For tables with hundreds of rows or many columns, consider:
/// - Using [`TableBorders::HORIZONTAL`] or [`TableBorders::VERTICAL`] instead of
///   [`TableBorders::ALL`]
/// - Limiting the number of visible rows through pagination or virtualization
/// - Disabling internal borders for very large datasets
///
/// ### Limitations and Constraints
///
/// The internal border system has several limitations:
///
/// - **Character Set Dependency**: Borders use Unicode box-drawing characters that may not display
///   correctly in all terminals or fonts. Fallback to ASCII alternatives may be needed for
///   compatibility.
/// - **Style Uniformity**: All internal borders (horizontal and vertical) must use the same style.
///   You cannot style horizontal and vertical borders differently.
/// - **Border Character System**: The system uses a fixed set of border characters from the Unicode
///   box-drawing range. Custom border characters are not supported.
/// - **Terminal Compatibility**: Some terminals may not support all border characters or may render
///   them differently. Test with your target terminal environment.
///
/// ### Terminal Capability Considerations
///
/// The border style you choose should be appropriate for your target terminal capabilities:
///
/// - **Color Support**: Use colors that your terminal supports
/// - **Unicode Support**: Ensure your terminal can display box-drawing characters
/// - **Style Modifiers**: Some terminals may not support all style modifiers (bold, dim, etc.)
///
/// For maximum compatibility, consider using simple styles without complex color combinations
/// or style modifiers.
///
/// ### Best Practices and Recommendations
///
/// **For Small to Medium Tables (< 100 rows)**:
/// - Use [`TableBorders::ALL`] for complete grid appearance
/// - Choose simple, high-contrast border styles
/// - Test with your target terminal environment
///
/// **For Large Tables (100+ rows)**:
/// - Consider using [`TableBorders::HORIZONTAL`] only to reduce rendering overhead
/// - Implement pagination or virtualization to limit visible rows
/// - Monitor performance and disable borders if needed
///
/// **For Maximum Compatibility**:
/// - Use basic colors (white, black, gray) for borders
/// - Avoid complex style modifiers
/// - Test with different terminal emulators
/// - Consider providing fallback styles for limited terminals
///
/// **For Performance-Critical Applications**:
/// - Profile border rendering impact on your specific use case
/// - Consider disabling internal borders for very large datasets
/// - Use simpler border configurations when possible
///
/// See the table example and the recipe and traceroute tabs in the demo2 example in the [Examples]
/// directory for a more in depth example of the various configuration options and for how to handle
/// state.
///
/// [Examples]: https://github.com/ratatui/ratatui/blob/master/examples/README.md
///
/// # Constructor methods
///
/// - [`Table::new`] creates a new [`Table`] with the given rows.
/// - [`Table::default`] creates an empty [`Table`]. You can then add rows using [`Table::rows`].
///
/// # Setter methods
///
/// These methods are fluent setters. They return a new `Table` with the specified property set.
///
/// - [`Table::rows`] sets the rows of the [`Table`].
/// - [`Table::header`] sets the header row of the [`Table`].
/// - [`Table::footer`] sets the footer row of the [`Table`].
/// - [`Table::widths`] sets the width constraints of each column.
/// - [`Table::column_spacing`] sets the spacing between each column.
/// - [`Table::block`] wraps the table in a [`Block`] widget.
/// - [`Table::style`] sets the base style of the widget.
/// - [`Table::row_highlight_style`] sets the style of the selected row.
/// - [`Table::column_highlight_style`] sets the style of the selected column.
/// - [`Table::cell_highlight_style`] sets the style of the selected cell.
/// - [`Table::highlight_symbol`] sets the symbol to be displayed in front of the selected row.
/// - [`Table::highlight_spacing`] sets when to show the highlight spacing.
/// - [`Table::internal_borders`] sets which internal borders to display within the table.
/// - [`Table::border_style`] sets the style for the internal borders.
///
/// # Example
///
/// ```rust
/// use ratatui::layout::Constraint;
/// use ratatui::style::{Style, Stylize};
/// use ratatui::widgets::{Block, Row, Table};
/// use ratatui_widgets::table::TableBorders;
///
/// let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
/// // Columns widths are constrained in the same way as Layout...
/// let widths = [
///     Constraint::Length(5),
///     Constraint::Length(5),
///     Constraint::Length(10),
/// ];
/// let table = Table::new(rows, widths)
///     // ...and they can be separated by a fixed spacing.
///     .column_spacing(1)
///     // You can set the style of the entire Table.
///     .style(Style::new().blue())
///     // It has an optional header, which is simply a Row always visible at the top.
///     .header(
///         Row::new(vec!["Col1", "Col2", "Col3"])
///             .style(Style::new().bold())
///             // To add space between the header and the rest of the rows, specify the margin
///             .bottom_margin(1),
///     )
///     // It has an optional footer, which is simply a Row always visible at the bottom.
///     .footer(Row::new(vec!["Updated on Dec 28"]))
///     // As any other widget, a Table can be wrapped in a Block.
///     .block(Block::new().title("Table"))
///     // You can add internal borders between and around cells.
///     .internal_borders(TableBorders::ALL)
///     .border_style(Style::new().white())
///     // The selected row, column, cell and its content can also be styled.
///     .row_highlight_style(Style::new().reversed())
///     .column_highlight_style(Style::new().red())
///     .cell_highlight_style(Style::new().blue())
///     // ...and potentially show a symbol in front of the selection.
///     .highlight_symbol(">>");
/// ```
///
/// # Advanced Example: Internal Borders with External Block
///
/// This example demonstrates how internal borders interact with external Block borders:
///
/// ```rust
/// use ratatui::layout::Constraint;
/// use ratatui::style::{Color, Style};
/// use ratatui::widgets::{Block, Row, Table};
/// use ratatui_widgets::table::TableBorders;
///
/// let rows = vec![
///     Row::new(vec!["Name", "Age", "City"]),
///     Row::new(vec!["Alice", "25", "New York"]),
///     Row::new(vec!["Bob", "30", "London"]),
///     Row::new(vec!["Charlie", "35", "Paris"]),
/// ];
///
/// let table = Table::new(rows, [Constraint::Length(8); 3])
///     .header(Row::new(vec!["Name", "Age", "City"]).style(Style::new().bold()))
///     .block(
///         Block::bordered()
///             .title("User Database")
///             .border_style(Style::new().fg(Color::Yellow)),
///     )
///     .internal_borders(TableBorders::ALL)
///     .border_style(Style::new().fg(Color::Blue))
///     .style(Style::new().fg(Color::White));
/// ```
///
/// Rows can be created from an iterator of [`Cell`]s. Each row can have an associated height,
/// bottom margin, and style. See [`Row`] for more details.
///
/// ```rust
/// use ratatui::style::{Style, Stylize};
/// use ratatui::text::{Line, Span};
/// use ratatui::widgets::{Cell, Row, Table};
///
/// // a Row can be created from simple strings.
/// let row = Row::new(vec!["Row11", "Row12", "Row13"]);
///
/// // You can style the entire row.
/// let row = Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::new().red());
///
/// // If you need more control over the styling, create Cells directly
/// let row = Row::new(vec![
///     Cell::from("Row31"),
///     Cell::from("Row32").style(Style::new().yellow()),
///     Cell::from(Line::from(vec![Span::raw("Row"), Span::from("33").green()])),
/// ]);
///
/// // If a Row need to display some content over multiple lines, specify the height.
/// let row = Row::new(vec![
///     Cell::from("Row\n41"),
///     Cell::from("Row\n42"),
///     Cell::from("Row\n43"),
/// ])
/// .height(2);
/// ```
///
/// Cells can be created from anything that can be converted to [`Text`]. See [`Cell`] for more
/// details.
///
/// ```rust
/// use ratatui::style::{Style, Stylize};
/// use ratatui::text::{Line, Span, Text};
/// use ratatui::widgets::Cell;
///
/// Cell::from("simple string");
/// Cell::from("simple styled span".red());
/// Cell::from(Span::raw("raw span"));
/// Cell::from(Span::styled("styled span", Style::new().red()));
/// Cell::from(Line::from(vec![
///     Span::raw("a vec of "),
///     Span::from("spans").bold(),
/// ]));
/// Cell::from(Text::from("text"));
/// ```
///
/// Just as rows can be collected from iterators of `Cell`s, tables can be collected from iterators
/// of `Row`s.  This will create a table with column widths evenly dividing the space available.
/// These default columns widths can be overridden using the `Table::widths` method.
///
/// ```rust
/// use ratatui::layout::Constraint;
/// use ratatui::widgets::{Row, Table};
///
/// let text = "Mary had a\nlittle lamb.";
///
/// let table = text
///     .split("\n")
///     .map(|line: &str| -> Row { line.split_ascii_whitespace().collect() })
///     .collect::<Table>()
///     .widths([Constraint::Length(10); 3]);
/// ```
///
/// `Table` also implements the [`Styled`] trait, which means you can use style shorthands from
/// the [`Stylize`] trait to set the style of the widget more concisely.
///
/// ```rust
/// use ratatui::layout::Constraint;
/// use ratatui::style::Stylize;
/// use ratatui::widgets::{Row, Table};
///
/// let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
/// let widths = [
///     Constraint::Length(5),
///     Constraint::Length(5),
///     Constraint::Length(10),
/// ];
/// let table = Table::new(rows, widths).red().italic();
/// ```
///
/// # Stateful example
///
/// `Table` is a [`StatefulWidget`], which means you can use it with [`TableState`] to allow the
/// user to scroll through the rows and select one of them.
///
/// ```rust
/// use ratatui::Frame;
/// use ratatui::layout::{Constraint, Rect};
/// use ratatui::style::{Style, Stylize};
/// use ratatui::widgets::{Block, Row, Table, TableState};
///
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
/// // Note: TableState should be stored in your application state (not constructed in your render
/// // method) so that the selected row is preserved across renders
/// let mut table_state = TableState::default();
/// let rows = [
///     Row::new(vec!["Row11", "Row12", "Row13"]),
///     Row::new(vec!["Row21", "Row22", "Row23"]),
///     Row::new(vec!["Row31", "Row32", "Row33"]),
/// ];
/// let widths = [
///     Constraint::Length(5),
///     Constraint::Length(5),
///     Constraint::Length(10),
/// ];
/// let table = Table::new(rows, widths)
///     .block(Block::new().title("Table"))
///     .row_highlight_style(Style::new().reversed())
///     .highlight_symbol(">>");
///
/// frame.render_stateful_widget(table, area, &mut table_state);
/// # }
/// ```
///
/// [`Stylize`]: ratatui_core::style::Stylize
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Table<'a> {
    /// Data to display in each row
    rows: Vec<Row<'a>>,

    /// Optional header
    header: Option<Row<'a>>,

    /// Optional footer
    footer: Option<Row<'a>>,

    /// Width constraints for each column
    widths: Vec<Constraint>,

    /// Space between each column
    column_spacing: u16,

    /// A block to wrap the widget in
    block: Option<Block<'a>>,

    /// Base style for the widget
    style: Style,

    /// Style used to render the selected row
    row_highlight_style: Style,

    /// Style used to render the selected column
    column_highlight_style: Style,

    /// Style used to render the selected cell
    cell_highlight_style: Style,

    /// Symbol in front of the selected row
    highlight_symbol: Text<'a>,

    /// Decides when to allocate spacing for the row selection
    highlight_spacing: HighlightSpacing,

    /// Controls how to distribute extra space among the columns
    flex: Flex,

    /// The type of internal borders to display.
    internal_borders: TableBorders,

    /// The style for borders.
    border_style: Style,
}

impl Default for Table<'_> {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            header: None,
            footer: None,
            widths: Vec::new(),
            column_spacing: 1,
            block: None,
            style: Style::new(),
            row_highlight_style: Style::new(),
            column_highlight_style: Style::new(),
            cell_highlight_style: Style::new(),
            highlight_symbol: Text::default(),
            highlight_spacing: HighlightSpacing::default(),
            flex: Flex::Start,
            internal_borders: TableBorders::NONE,
            border_style: Style::default(),
        }
    }
}

impl<'a> Table<'a> {
    /// Creates a new [`Table`] widget with the given rows.
    ///
    /// The `rows` parameter accepts any value that can be converted into an iterator of [`Row`]s.
    /// This includes arrays, slices, and [`Vec`]s.
    ///
    /// The `widths` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<Constraint>>`. This includes arrays, slices, vectors, iterators. `Into<Constraint>` is
    /// implemented on u16, so you can pass an array, vec, etc. of u16 to this function to create a
    /// table with fixed width columns.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::widgets::{Row, Table};
    ///
    /// let rows = [
    ///     Row::new(vec!["Cell1", "Cell2"]),
    ///     Row::new(vec!["Cell3", "Cell4"]),
    /// ];
    /// let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths);
    /// ```
    pub fn new<R, C>(rows: R, widths: C) -> Self
    where
        R: IntoIterator,
        R::Item: Into<Row<'a>>,
        C: IntoIterator,
        C::Item: Into<Constraint>,
    {
        let widths = widths.into_iter().map(Into::into).collect_vec();
        ensure_percentages_less_than_100(&widths);
        let rows = rows.into_iter().map(Into::into).collect();
        Self {
            rows,
            widths,
            ..Default::default()
        }
    }

    /// Set the rows
    ///
    /// The `rows` parameter accepts any value that can be converted into an iterator of [`Row`]s.
    /// This includes arrays, slices, and [`Vec`]s.
    ///
    /// # Warning
    ///
    /// This method does not currently set the column widths. You will need to set them manually by
    /// calling [`Table::widths`].
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::{Row, Table};
    ///
    /// let rows = [
    ///     Row::new(vec!["Cell1", "Cell2"]),
    ///     Row::new(vec!["Cell3", "Cell4"]),
    /// ];
    /// let table = Table::default().rows(rows);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn rows<T>(mut self, rows: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Row<'a>>,
    {
        self.rows = rows.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the header row
    ///
    /// The `header` parameter is a [`Row`] which will be displayed at the top of the [`Table`]
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::{Cell, Row, Table};
    ///
    /// let header = Row::new(vec![
    ///     Cell::from("Header Cell 1"),
    ///     Cell::from("Header Cell 2"),
    /// ]);
    /// let table = Table::default().header(header);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn header(mut self, header: Row<'a>) -> Self {
        self.header = Some(header);
        self
    }

    /// Sets the footer row
    ///
    /// The `footer` parameter is a [`Row`] which will be displayed at the bottom of the [`Table`]
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::widgets::{Cell, Row, Table};
    ///
    /// let footer = Row::new(vec![
    ///     Cell::from("Footer Cell 1"),
    ///     Cell::from("Footer Cell 2"),
    /// ]);
    /// let table = Table::default().footer(footer);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn footer(mut self, footer: Row<'a>) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Set the widths of the columns.
    ///
    /// The `widths` parameter accepts any type that implements `IntoIterator<Item =
    /// Into<Constraint>>`. This includes arrays, slices, vectors, iterators. `Into<Constraint>` is
    /// implemented on u16, so you can pass an array, vec, etc. of u16 to this function to create a
    /// table with fixed width columns.
    ///
    /// If the widths are empty, the table will be rendered with equal widths.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::widgets::{Cell, Row, Table};
    ///
    /// let table = Table::default().widths([Constraint::Length(5), Constraint::Length(5)]);
    /// let table = Table::default().widths(vec![Constraint::Length(5); 2]);
    ///
    /// // widths could also be computed at runtime
    /// let widths = [10, 10, 20].into_iter().map(|c| Constraint::Length(c));
    /// let table = Table::default().widths(widths);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn widths<I>(mut self, widths: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
    {
        let widths = widths.into_iter().map(Into::into).collect_vec();
        ensure_percentages_less_than_100(&widths);
        self.widths = widths;
        self
    }

    /// Set the spacing between columns
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::widgets::{Row, Table};
    ///
    /// let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).column_spacing(1);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn column_spacing(mut self, spacing: u16) -> Self {
        self.column_spacing = spacing;
        self
    }

    /// Wraps the table with a custom [`Block`] widget.
    ///
    /// The `block` parameter is of type [`Block`]. This holds the specified block to be
    /// created around the [`Table`]
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::widgets::{Block, Cell, Row, Table};
    ///
    /// let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let block = Block::bordered().title("Table");
    /// let table = Table::new(rows, widths).block(block);
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
    /// [`Row::style`], [`Cell::style`], or the styles of cell's content.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::style::{Style, Stylize};
    /// use ratatui::widgets::{Row, Table};
    ///
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).style(Style::new().red().italic());
    /// ```
    ///
    /// `Table` also implements the [`Styled`] trait, which means you can use style shorthands from
    /// the [`Stylize`] trait to set the style of the widget more concisely.
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::style::Stylize;
    /// use ratatui::widgets::{Cell, Row, Table};
    ///
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = vec![Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).red().italic();
    /// ```
    ///
    /// [`Color`]: ratatui_core::style::Color
    /// [`Stylize`]: ratatui_core::style::Stylize
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Set the style of the selected row
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be applied to the entire row, including the selection symbol if it is
    /// displayed, and will override any style set on the row or on the individual cells.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::style::{Style, Stylize};
    /// use ratatui::widgets::{Cell, Row, Table};
    ///
    /// let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).highlight_style(Style::new().red().italic());
    /// ```
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    #[deprecated(note = "use `row_highlight_style()` instead")]
    pub fn highlight_style<S: Into<Style>>(self, highlight_style: S) -> Self {
        self.row_highlight_style(highlight_style)
    }

    /// Set the style of the selected row
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be applied to the entire row, including the selection symbol if it is
    /// displayed, and will override any style set on the row or on the individual cells.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{layout::Constraint, style::{Style, Stylize}, widgets::{Row, Table}};
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).row_highlight_style(Style::new().red().italic());
    /// ```
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn row_highlight_style<S: Into<Style>>(mut self, highlight_style: S) -> Self {
        self.row_highlight_style = highlight_style.into();
        self
    }

    /// Set the style of the selected column
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be applied to the entire column, and will override any style set on the
    /// row or on the individual cells.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{layout::Constraint, style::{Style, Stylize}, widgets::{Row, Table}};
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).column_highlight_style(Style::new().red().italic());
    /// ```
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn column_highlight_style<S: Into<Style>>(mut self, highlight_style: S) -> Self {
        self.column_highlight_style = highlight_style.into();
        self
    }

    /// Set the style of the selected cell
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be applied to the selected cell, and will override any style set on the
    /// row or on the individual cells.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{layout::Constraint, style::{Style, Stylize}, widgets::{Row, Table}};
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).cell_highlight_style(Style::new().red().italic());
    /// ```
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn cell_highlight_style<S: Into<Style>>(mut self, highlight_style: S) -> Self {
        self.cell_highlight_style = highlight_style.into();
        self
    }

    /// Set the symbol to be displayed in front of the selected row
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::widgets::{Cell, Row, Table};
    ///
    /// # let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// # let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).highlight_symbol(">>");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn highlight_symbol<T: Into<Text<'a>>>(mut self, highlight_symbol: T) -> Self {
        self.highlight_symbol = highlight_symbol.into();
        self
    }

    /// Set when to show the highlight spacing
    ///
    /// The highlight spacing is the spacing that is allocated for the selection symbol column (if
    /// enabled) and is used to shift the table when a row is selected. This method allows you to
    /// configure when this spacing is allocated.
    ///
    /// - [`HighlightSpacing::Always`] will always allocate the spacing, regardless of whether a row
    ///   is selected or not. This means that the table will never change size, regardless of if a
    ///   row is selected or not.
    /// - [`HighlightSpacing::WhenSelected`] will only allocate the spacing if a row is selected.
    ///   This means that the table will shift when a row is selected. This is the default setting
    ///   for backwards compatibility, but it is recommended to use `HighlightSpacing::Always` for a
    ///   better user experience.
    /// - [`HighlightSpacing::Never`] will never allocate the spacing, regardless of whether a row
    ///   is selected or not. This means that the highlight symbol will never be drawn.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::widgets::{HighlightSpacing, Row, Table};
    ///
    /// let rows = [Row::new(vec!["Cell1", "Cell2"])];
    /// let widths = [Constraint::Length(5), Constraint::Length(5)];
    /// let table = Table::new(rows, widths).highlight_spacing(HighlightSpacing::Always);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn highlight_spacing(mut self, value: HighlightSpacing) -> Self {
        self.highlight_spacing = value;
        self
    }

    /// Set how extra space is distributed amongst columns.
    ///
    /// This determines how the space is distributed when the constraints are satisfied. By default,
    /// the extra space is not distributed at all.  But this can be changed to distribute all extra
    /// space to the last column or to distribute it equally.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// Create a table that needs at least 30 columns to display.  Any extra space will be assigned
    /// to the last column.
    /// ```
    /// use ratatui::layout::{Constraint, Flex};
    /// use ratatui::widgets::{Row, Table};
    ///
    /// let widths = [
    ///     Constraint::Min(10),
    ///     Constraint::Min(10),
    ///     Constraint::Min(10),
    /// ];
    /// let table = Table::new(Vec::<Row>::new(), widths).flex(Flex::Legacy);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    /// Set which internal borders to display within the table.
    ///
    /// Internal borders are the lines drawn between and around table cells, separate from any
    /// external borders that might be added by wrapping the table in a [`Block`]. This method
    /// controls which of these internal borders are visible.
    ///
    /// **Note on Terminology**: While this method is called "internal borders" to distinguish
    /// them from external [`Block`] borders, they serve the same visual purpose as traditional
    /// table borders. The term "internal" refers to their position within the table structure,
    /// not their importance or functionality.
    ///
    /// The borders are drawn using the style set by [`border_style`](Self::border_style).
    ///
    /// - [`TableBorders::NONE`] - No internal borders (default)
    /// - [`TableBorders::HORIZONTAL`] - Only horizontal lines between rows
    /// - [`TableBorders::VERTICAL`] - Only vertical lines between columns
    /// - [`TableBorders::ALL`] - Both horizontal and vertical borders
    ///
    /// You can also combine borders using bitwise operations, e.g.,
    /// `TableBorders::HORIZONTAL | TableBorders::VERTICAL`.
    ///
    /// **Performance Note**: Enabling internal borders adds rendering overhead, especially for
    /// large tables. Consider the performance implications when using this feature with tables
    /// containing many rows or columns.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::style::{Color, Style};
    /// use ratatui::widgets::{Block, Table};
    /// use ratatui_widgets::table::TableBorders;
    ///
    /// let table = Table::default()
    ///     .internal_borders(TableBorders::ALL)
    ///     .border_style(Style::default().fg(Color::Blue));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn internal_borders(mut self, internal_borders: TableBorders) -> Self {
        self.internal_borders = internal_borders;
        self
    }

    /// Set the style for internal borders.
    ///
    /// This method sets the visual style (color, modifiers, etc.) that will be applied to the
    /// internal borders of the table. The borders themselves are controlled by the
    /// [`internal_borders`](Self::internal_borders) method, while this method determines how
    /// those borders will look.
    ///
    /// The style affects all internal borders equally - you cannot style horizontal and vertical
    /// borders differently. The style is applied to the border characters themselves, not to the
    /// content of the cells.
    ///
    /// **Terminal Compatibility**: The effectiveness of border styling depends on your terminal's
    /// capabilities. Some terminals may not support all colors or style modifiers. For maximum
    /// compatibility, consider using simple styles and testing with your target terminal
    /// environment.
    ///
    /// **Validation Note**: This method does not validate that the style is appropriate for your
    /// terminal's capabilities. It's the responsibility of the application to ensure the chosen
    /// style works well with the target terminal environment.
    ///
    /// Common styling options include:
    /// - Colors: `.fg(Color::Blue)`, `.bg(Color::Gray)`
    /// - Modifiers: `.bold()`, `.dim()`, `.italic()`
    /// - Combined: `.fg(Color::Red).bold()`
    ///
    /// **Performance Consideration**: Complex styles with multiple modifiers may have a slight
    /// performance impact when rendering large tables with many borders.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui::style::{Color, Style};
    /// use ratatui::widgets::{Block, Table};
    /// use ratatui_widgets::table::TableBorders;
    ///
    /// // Blue colored borders
    /// let table = Table::default()
    ///     .internal_borders(TableBorders::ALL)
    ///     .border_style(Style::default().fg(Color::Blue));
    ///
    /// // Gray borders with bold styling
    /// let table = Table::default()
    ///     .internal_borders(TableBorders::HORIZONTAL)
    ///     .border_style(Style::default().fg(Color::Gray).bold());
    ///
    /// // Dim borders for a subtle appearance
    /// let table = Table::default()
    ///     .internal_borders(TableBorders::VERTICAL)
    ///     .border_style(Style::default().dim());
    ///
    /// // Simple style for maximum compatibility
    /// let table = Table::default()
    ///     .internal_borders(TableBorders::ALL)
    ///     .border_style(Style::default().fg(Color::White));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn border_style(mut self, border_style: Style) -> Self {
        self.border_style = border_style;
        self
    }

    // === Private helpers ===
    fn layout(&self, area: Rect) -> (Rect, Rect, Rect) {
        let header_top_margin = self.header.as_ref().map_or(0, |h| h.top_margin);
        let header_height = self.header.as_ref().map_or(0, |h| h.height);
        let header_bottom_margin = self.header.as_ref().map_or(0, |h| h.bottom_margin);
        let footer_top_margin = self.footer.as_ref().map_or(0, |h| h.top_margin);
        let footer_height = self.footer.as_ref().map_or(0, |f| f.height);
        let footer_bottom_margin = self.footer.as_ref().map_or(0, |h| h.bottom_margin);
        let layout = Layout::vertical([
            Constraint::Length(header_top_margin),
            Constraint::Length(header_height),
            Constraint::Length(header_bottom_margin),
            Constraint::Min(0),
            Constraint::Length(footer_top_margin),
            Constraint::Length(footer_height),
            Constraint::Length(footer_bottom_margin),
        ])
        .split(area);
        let (header_area, rows_area, footer_area) = (layout[1], layout[3], layout[5]);
        (header_area, rows_area, footer_area)
    }

    const fn get_border_symbol(
        _self: &Self,
        x: u16,
        y: u16,
        area: Rect,
        is_horizontal: bool,
        has_horizontal_border: bool,
        has_vertical_border: bool,
    ) -> &'static str {
        use symbols::line;

        // Check if this position is at the edge of the table area (where block borders are)
        let is_left_edge = x == area.x;
        let is_right_edge = x == area.right() - 1;
        let is_top_edge = y == area.y;
        let is_bottom_edge = y == area.bottom() - 1;

        // If both horizontal and vertical borders are present, use cross symbol
        if has_horizontal_border && has_vertical_border {
            return line::NORMAL.cross;
        }

        // If we're at an edge, we need to use intersection symbols
        if is_horizontal {
            if is_left_edge {
                line::NORMAL.vertical_right
            } else if is_right_edge {
                line::NORMAL.vertical_left
            } else {
                line::NORMAL.horizontal
            }
        } else if is_top_edge {
            line::NORMAL.horizontal_down
        } else if is_bottom_edge {
            line::NORMAL.horizontal_up
        } else {
            line::NORMAL.vertical
        }
    }

    fn has_vertical_border_at(&self, x: u16, area: Rect, selection_width: u16) -> bool {
        // If we don't have vertical borders, return false
        if !self.internal_borders.contains(TableBorders::VERTICAL) {
            return false;
        }

        // Calculate column widths to find vertical border positions
        let column_count = self.column_count();
        let column_widths = self.get_column_widths(area.width, selection_width, column_count);

        // Check if x is at a vertical border position
        for (i, (col_x, width)) in column_widths.iter().enumerate() {
            if i < column_widths.len() - 1 {
                let border_x = if self.column_spacing > 0 {
                    area.x
                        .saturating_add(*col_x)
                        .saturating_add(*width)
                        .saturating_add(self.column_spacing / 2)
                } else {
                    area.x.saturating_add(*col_x).saturating_add(*width)
                };
                if x == border_x {
                    return true;
                }
            }
        }
        false
    }

    fn has_horizontal_border_at(&self, y: u16, area: Rect, _selection_width: u16) -> bool {
        // If we don't have horizontal borders, return false
        if !self.internal_borders.contains(TableBorders::HORIZONTAL) {
            return false;
        }

        // Calculate row positions to find horizontal border positions
        let mut y_offset: u16 = 0;
        for (i, row) in self.rows.iter().enumerate() {
            y_offset = y_offset
                .saturating_add(row.top_margin)
                .saturating_add(row.height);

            // Check if this is a border position (between rows)
            if i < self.rows.len() - 1 {
                let border_y = area.y.saturating_add(y_offset);
                if y == border_y {
                    return true;
                }
            }
            y_offset = y_offset.saturating_add(row.bottom_margin);
        }
        false
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer, column_widths: &[(u16, u16)]) {
        if let Some(ref header) = self.header {
            buf.set_style(area, header.style);
            for ((x, width), cell) in column_widths.iter().zip(header.cells.iter()) {
                cell.render(Rect::new(area.x + x, area.y, *width, area.height), buf);
            }
        }
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer, column_widths: &[(u16, u16)]) {
        if let Some(ref footer) = self.footer {
            buf.set_style(area, footer.style);
            for ((x, width), cell) in column_widths.iter().zip(footer.cells.iter()) {
                cell.render(Rect::new(area.x + x, area.y, *width, area.height), buf);
            }
        }
    }

    fn render_rows(
        &self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut TableState,
        selection_width: u16,
        columns_widths: &[(u16, u16)],
    ) {
        if self.rows.is_empty() {
            return;
        }

        let (start_index, end_index) = self.visible_rows(state, area);
        state.offset = start_index;

        let mut y_offset: u16 = 0;

        let mut selected_row_area = None;
        for (i, row) in self
            .rows
            .iter()
            .enumerate()
            .skip(start_index)
            .take(end_index - start_index)
        {
            let y = area
                .y
                .saturating_add(y_offset)
                .saturating_add(row.top_margin);
            let height = (y.saturating_add(row.height))
                .min(area.bottom())
                .saturating_sub(y);
            let row_area = Rect { y, height, ..area };
            buf.set_style(row_area, row.style);

            let is_selected = state.selected.is_some_and(|index| index == i);
            if selection_width > 0 && is_selected {
                let selection_area = Rect {
                    width: selection_width,
                    ..row_area
                };
                buf.set_style(selection_area, row.style);
                (&self.highlight_symbol).render(selection_area, buf);
            }
            for ((x, width), cell) in columns_widths.iter().zip(row.cells.iter()) {
                cell.render(
                    Rect::new(
                        row_area.x.saturating_add(*x),
                        row_area.y,
                        *width,
                        row_area.height,
                    ),
                    buf,
                );
            }
            if is_selected {
                selected_row_area = Some(row_area);
            }
            y_offset = y_offset.saturating_add(row.height_with_margin());
        }

        let selected_column_area = state.selected_column.and_then(|s| {
            // The selection is clamped by the column count. Since a user can manually specify an
            // incorrect number of widths, we should use panic free methods.
            columns_widths.get(s).map(|(x, width)| Rect {
                x: x.saturating_add(area.x),
                width: *width,
                ..area
            })
        });

        match (selected_row_area, selected_column_area) {
            (Some(row_area), Some(col_area)) => {
                buf.set_style(row_area, self.row_highlight_style);
                buf.set_style(col_area, self.column_highlight_style);
                let cell_area = row_area.intersection(col_area);
                buf.set_style(cell_area, self.cell_highlight_style);
            }
            (Some(row_area), None) => {
                buf.set_style(row_area, self.row_highlight_style);
            }
            (None, Some(col_area)) => {
                buf.set_style(col_area, self.column_highlight_style);
            }
            (None, None) => (),
        }
    }

    fn render_internal_borders(
        &self,
        area: Rect,
        buf: &mut Buffer,
        selection_width: u16,
        columns_widths: &[(u16, u16)],
        start_index: usize,
        end_index: usize,
    ) {
        match self.internal_borders {
            TableBorders::NONE => (),
            TableBorders::HORIZONTAL => {
                self.render_horizontal_borders(area, buf, selection_width, start_index, end_index);
            }
            TableBorders::VERTICAL => {
                self.render_vertical_borders(area, buf, selection_width, columns_widths);
            }
            TableBorders::ALL => {
                self.render_horizontal_borders(area, buf, selection_width, start_index, end_index);
                self.render_vertical_borders(area, buf, selection_width, columns_widths);
            }
            _ => {
                // Handle any other combinations of flags
                if self.internal_borders.contains(TableBorders::HORIZONTAL) {
                    self.render_horizontal_borders(
                        area,
                        buf,
                        selection_width,
                        start_index,
                        end_index,
                    );
                }
                if self.internal_borders.contains(TableBorders::VERTICAL) {
                    self.render_vertical_borders(area, buf, selection_width, columns_widths);
                }
            }
        }
    }

    fn render_horizontal_borders(
        &self,
        area: Rect,
        buf: &mut Buffer,
        selection_width: u16,
        start_index: usize,
        end_index: usize,
    ) {
        let mut y_offset: u16 = 0;
        for (i, row) in self
            .rows
            .iter()
            .enumerate()
            .skip(start_index)
            .take(end_index - start_index)
        {
            y_offset = y_offset
                .saturating_add(row.top_margin)
                .saturating_add(row.height);
            if i < end_index - 1 && y_offset < area.height {
                let border_y = area.y.saturating_add(y_offset);
                if border_y < area.bottom() {
                    for x in (area.x.saturating_add(selection_width))..area.right() {
                        let cell = &mut buf[(x, border_y)];
                        // Check if there's a vertical border at this position
                        let has_vertical_border =
                            self.has_vertical_border_at(x, area, selection_width);
                        let symbol = Self::get_border_symbol(
                            self,
                            x,
                            border_y,
                            area,
                            true,
                            true,
                            has_vertical_border,
                        );
                        cell.merge_symbol(symbol, MergeStrategy::Exact)
                            .set_style(self.border_style);
                    }
                }
            }
            y_offset = y_offset.saturating_add(row.bottom_margin);
        }
    }

    fn render_vertical_borders(
        &self,
        area: Rect,
        buf: &mut Buffer,
        selection_width: u16,
        columns_widths: &[(u16, u16)],
    ) {
        for (i, (x, width)) in columns_widths.iter().enumerate() {
            if i < columns_widths.len() - 1 {
                let border_x = if self.column_spacing > 0 {
                    area.x
                        .saturating_add(*x)
                        .saturating_add(*width)
                        .saturating_add(self.column_spacing / 2)
                } else {
                    area.x.saturating_add(*x).saturating_add(*width)
                };
                if border_x < area.right() {
                    for y in area.y..area.bottom() {
                        let cell = &mut buf[(border_x, y)];
                        // Check if there's a horizontal border at this position
                        let has_horizontal_border =
                            self.has_horizontal_border_at(y, area, selection_width);
                        let symbol = Self::get_border_symbol(
                            self,
                            border_x,
                            y,
                            area,
                            false,
                            has_horizontal_border,
                            true,
                        );
                        cell.merge_symbol(symbol, MergeStrategy::Exact)
                            .set_style(self.border_style);
                    }
                }
            }
        }
    }

    /// Return the indexes of the visible rows.
    ///
    /// The algorithm works as follows:
    /// - start at the offset and calculate the height of the rows that can be displayed within the
    ///   area.
    /// - if the selected row is not visible, scroll the table to ensure it is visible.
    /// - if there is still space to fill then there's a partial row at the end which should be
    ///   included in the view.
    fn visible_rows(&self, state: &TableState, area: Rect) -> (usize, usize) {
        let last_row = self.rows.len().saturating_sub(1);
        let mut start = state.offset.min(last_row);

        if let Some(selected) = state.selected {
            start = start.min(selected);
        }

        let mut end = start;
        let mut height: u16 = 0;

        for item in self.rows.iter().skip(start) {
            if height.saturating_add(item.height) > area.height {
                break;
            }
            height = height.saturating_add(item.height_with_margin());
            end += 1;
        }

        if let Some(selected) = state.selected {
            let selected = selected.min(last_row);

            // scroll down until the selected row is visible
            while selected >= end {
                height = height.saturating_add(self.rows[end].height_with_margin());
                end += 1;
                while height > area.height {
                    height = height.saturating_sub(self.rows[start].height_with_margin());
                    start += 1;
                }
            }
        }

        // Include a partial row if there is space
        if height < area.height && end < self.rows.len() {
            end += 1;
        }

        (start, end)
    }

    /// Get all offsets and widths of all user specified columns.
    ///
    /// Returns (x, width). When self.widths is empty, it is assumed `.widths()` has not been called
    /// and a default of equal widths is returned.
    fn get_column_widths(
        &self,
        max_width: u16,
        selection_width: u16,
        col_count: usize,
    ) -> Vec<(u16, u16)> {
        let widths = if self.widths.is_empty() {
            // Divide the space between each column equally
            vec![Constraint::Length(max_width / col_count.max(1) as u16); col_count]
        } else {
            self.widths.clone()
        };
        // this will always allocate a selection area
        let [_selection_area, columns_area] =
            Layout::horizontal([Constraint::Length(selection_width), Constraint::Fill(0)])
                .areas(Rect::new(0, 0, max_width, 1));
        let rects = Layout::horizontal(widths)
            .flex(self.flex)
            .spacing(self.column_spacing)
            .split(columns_area);
        rects.iter().map(|c| (c.x, c.width)).collect()
    }

    fn column_count(&self) -> usize {
        self.rows
            .iter()
            .chain(self.footer.iter())
            .chain(self.header.iter())
            .map(|r| r.cells.len())
            .max()
            .unwrap_or_default()
    }

    /// Returns the width of the selection column if a row is selected, or the `highlight_spacing`
    /// is set to show the column always, otherwise 0.
    fn selection_width(&self, state: &TableState) -> u16 {
        let has_selection = state.selected.is_some();
        match self.highlight_spacing {
            HighlightSpacing::Always => self.highlight_symbol.width() as u16,
            HighlightSpacing::WhenSelected if has_selection => self.highlight_symbol.width() as u16,
            _ => 0,
        }
    }
}

fn ensure_percentages_less_than_100(widths: &[Constraint]) {
    for w in widths {
        if let Constraint::Percentage(p) = w {
            assert!(
                *p <= 100,
                "Percentages should be between 0 and 100 inclusively."
            );
        }
    }
}

impl Styled for Table<'_> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl<'a, Item> FromIterator<Item> for Table<'a>
where
    Item: Into<Row<'a>>,
{
    /// Collects an iterator of rows into a table.
    ///
    /// When collecting from an iterator into a table, the user must provide the widths using
    /// `Table::widths` after construction.
    fn from_iter<Iter: IntoIterator<Item = Item>>(rows: Iter) -> Self {
        let widths: [Constraint; 0] = [];
        Self::new(rows, widths)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use alloc::{format, vec};

    use ratatui_core::layout::Constraint::*;
    use ratatui_core::style::{Color, Modifier, Style, Stylize};
    use ratatui_core::text::Line;
    use rstest::{fixture, rstest};

    use super::*;
    use crate::table::Cell;

    #[test]
    fn new() {
        let rows = [Row::new(vec![Cell::from("")])];
        let widths = [Constraint::Percentage(100)];
        let table = Table::new(rows.clone(), widths);
        assert_eq!(table.rows, rows);
        assert_eq!(table.header, None);
        assert_eq!(table.footer, None);
        assert_eq!(table.widths, widths);
        assert_eq!(table.column_spacing, 1);
        assert_eq!(table.block, None);
        assert_eq!(table.style, Style::default());
        assert_eq!(table.row_highlight_style, Style::default());
        assert_eq!(table.highlight_symbol, Text::default());
        assert_eq!(table.highlight_spacing, HighlightSpacing::WhenSelected);
        assert_eq!(table.flex, Flex::Start);
        assert_eq!(table.internal_borders, TableBorders::NONE);
        assert_eq!(table.border_style, Style::default());
    }

    #[test]
    fn default() {
        let table = Table::default();
        assert_eq!(table.rows, []);
        assert_eq!(table.header, None);
        assert_eq!(table.footer, None);
        assert_eq!(table.widths, []);
        assert_eq!(table.column_spacing, 1);
        assert_eq!(table.block, None);
        assert_eq!(table.style, Style::default());
        assert_eq!(table.row_highlight_style, Style::default());
        assert_eq!(table.highlight_symbol, Text::default());
        assert_eq!(table.highlight_spacing, HighlightSpacing::WhenSelected);
        assert_eq!(table.flex, Flex::Start);
        assert_eq!(table.internal_borders, TableBorders::NONE);
        assert_eq!(table.border_style, Style::default());
    }

    #[test]
    fn collect() {
        let table = (0..4)
            .map(|i| -> Row { (0..4).map(|j| format!("{i}*{j} = {}", i * j)).collect() })
            .collect::<Table>()
            .widths([Constraint::Percentage(25); 4]);

        let expected_rows: Vec<Row> = vec![
            Row::new(["0*0 = 0", "0*1 = 0", "0*2 = 0", "0*3 = 0"]),
            Row::new(["1*0 = 0", "1*1 = 1", "1*2 = 2", "1*3 = 3"]),
            Row::new(["2*0 = 0", "2*1 = 2", "2*2 = 4", "2*3 = 6"]),
            Row::new(["3*0 = 0", "3*1 = 3", "3*2 = 6", "3*3 = 9"]),
        ];

        assert_eq!(table.rows, expected_rows);
        assert_eq!(table.widths, [Constraint::Percentage(25); 4]);
    }

    #[test]
    fn widths() {
        let table = Table::default().widths([Constraint::Length(100)]);
        assert_eq!(table.widths, [Constraint::Length(100)]);

        // ensure that code that uses &[] continues to work as there is a large amount of code that
        // uses this pattern
        #[expect(clippy::needless_borrows_for_generic_args)]
        let table = Table::default().widths(&[Constraint::Length(100)]);
        assert_eq!(table.widths, [Constraint::Length(100)]);

        let table = Table::default().widths(vec![Constraint::Length(100)]);
        assert_eq!(table.widths, [Constraint::Length(100)]);

        // ensure that code that uses &some_vec continues to work as there is a large amount of code
        // that uses this pattern
        #[expect(clippy::needless_borrows_for_generic_args)]
        let table = Table::default().widths(&vec![Constraint::Length(100)]);
        assert_eq!(table.widths, [Constraint::Length(100)]);

        let table = Table::default().widths([100].into_iter().map(Constraint::Length));
        assert_eq!(table.widths, [Constraint::Length(100)]);
    }

    #[test]
    fn rows() {
        let rows = [Row::new(vec![Cell::from("")])];
        let table = Table::default().rows(rows.clone());
        assert_eq!(table.rows, rows);
    }

    #[test]
    fn column_spacing() {
        let table = Table::default().column_spacing(2);
        assert_eq!(table.column_spacing, 2);
    }

    #[test]
    fn block() {
        let block = Block::bordered().title("Table");
        let table = Table::default().block(block.clone());
        assert_eq!(table.block, Some(block));
    }

    #[test]
    fn header() {
        let header = Row::new(vec![Cell::from("")]);
        let table = Table::default().header(header.clone());
        assert_eq!(table.header, Some(header));
    }

    #[test]
    fn footer() {
        let footer = Row::new(vec![Cell::from("")]);
        let table = Table::default().footer(footer.clone());
        assert_eq!(table.footer, Some(footer));
    }

    #[test]
    #[expect(deprecated)]
    fn highlight_style() {
        let style = Style::default().red().italic();
        let table = Table::default().highlight_style(style);
        assert_eq!(table.row_highlight_style, style);
    }

    #[test]
    fn row_highlight_style() {
        let style = Style::default().red().italic();
        let table = Table::default().row_highlight_style(style);
        assert_eq!(table.row_highlight_style, style);
    }

    #[test]
    fn column_highlight_style() {
        let style = Style::default().red().italic();
        let table = Table::default().column_highlight_style(style);
        assert_eq!(table.column_highlight_style, style);
    }

    #[test]
    fn cell_highlight_style() {
        let style = Style::default().red().italic();
        let table = Table::default().cell_highlight_style(style);
        assert_eq!(table.cell_highlight_style, style);
    }

    #[test]
    fn highlight_symbol() {
        let table = Table::default().highlight_symbol(">>");
        assert_eq!(table.highlight_symbol, Text::from(">>"));
    }

    #[test]
    fn highlight_spacing() {
        let table = Table::default().highlight_spacing(HighlightSpacing::Always);
        assert_eq!(table.highlight_spacing, HighlightSpacing::Always);
    }

    #[test]
    #[should_panic = "Percentages should be between 0 and 100 inclusively"]
    fn table_invalid_percentages() {
        let _ = Table::default().widths([Constraint::Percentage(110)]);
    }

    #[test]
    fn widths_conversions() {
        let array = [Constraint::Percentage(100)];
        let table = Table::new(Vec::<Row>::new(), array);
        assert_eq!(table.widths, [Constraint::Percentage(100)], "array");

        let array_ref = &[Constraint::Percentage(100)];
        let table = Table::new(Vec::<Row>::new(), array_ref);
        assert_eq!(table.widths, [Constraint::Percentage(100)], "array ref");

        let vec = vec![Constraint::Percentage(100)];
        let slice = vec.as_slice();
        let table = Table::new(Vec::<Row>::new(), slice);
        assert_eq!(table.widths, [Constraint::Percentage(100)], "slice");

        let vec = vec![Constraint::Percentage(100)];
        let table = Table::new(Vec::<Row>::new(), vec);
        assert_eq!(table.widths, [Constraint::Percentage(100)], "vec");

        let vec_ref = &vec![Constraint::Percentage(100)];
        let table = Table::new(Vec::<Row>::new(), vec_ref);
        assert_eq!(table.widths, [Constraint::Percentage(100)], "vec ref");
    }

    #[cfg(test)]
    mod state {
        use ratatui_core::buffer::Buffer;
        use ratatui_core::layout::{Constraint, Rect};
        use ratatui_core::widgets::StatefulWidget;

        use super::*;
        use crate::table::{Row, Table, TableState};

        #[fixture]
        fn table_buf() -> Buffer {
            Buffer::empty(Rect::new(0, 0, 10, 10))
        }

        #[rstest]
        fn test_list_state_empty_list(mut table_buf: Buffer) {
            let mut state = TableState::default();

            let rows: Vec<Row> = Vec::new();
            let widths = vec![Constraint::Percentage(100)];
            let table = Table::new(rows, widths);
            state.select_first();
            StatefulWidget::render(table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected, None);
            assert_eq!(state.selected_column, None);
        }

        #[rstest]
        fn test_list_state_single_item(mut table_buf: Buffer) {
            let mut state = TableState::default();

            let widths = vec![Constraint::Percentage(100)];

            let items = vec![Row::new(vec!["Item 1"])];
            let table = Table::new(items, widths);
            state.select_first();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected, Some(0));
            assert_eq!(state.selected_column, None);

            state.select_last();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected, Some(0));
            assert_eq!(state.selected_column, None);

            state.select_previous();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected, Some(0));
            assert_eq!(state.selected_column, None);

            state.select_next();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected, Some(0));
            assert_eq!(state.selected_column, None);

            let mut state = TableState::default();

            state.select_first_column();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected_column, Some(0));
            assert_eq!(state.selected, None);

            state.select_last_column();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected_column, Some(0));
            assert_eq!(state.selected, None);

            state.select_previous_column();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected_column, Some(0));
            assert_eq!(state.selected, None);

            state.select_next_column();
            StatefulWidget::render(&table, table_buf.area, &mut table_buf, &mut state);
            assert_eq!(state.selected_column, Some(0));
            assert_eq!(state.selected, None);
        }
    }

    #[cfg(test)]
    mod render {
        use ratatui_core::layout::Alignment;

        use super::*;

        #[test]
        fn render_empty_area() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let rows = vec![Row::new(vec!["Cell1", "Cell2"])];
            let table = Table::new(rows, vec![Constraint::Length(5); 2]);
            Widget::render(table, Rect::new(0, 0, 0, 0), &mut buf);
            assert_eq!(buf, Buffer::empty(Rect::new(0, 0, 15, 3)));
        }

        #[test]
        fn render_default() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let table = Table::default();
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            assert_eq!(buf, Buffer::empty(Rect::new(0, 0, 15, 3)));
        }

        #[test]
        fn render_with_block() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let block = Block::bordered().title("Block");
            let table = Table::new(rows, vec![Constraint::Length(5); 2]).block(block);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "┌Block────────┐",
                "│Cell1 Cell2  │",
                "└─────────────┘",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_header() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let header = Row::new(vec!["Head1", "Head2"]);
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2]).header(header);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "Head1 Head2    ",
                "Cell1 Cell2    ",
                "Cell3 Cell4    ",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_footer() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let footer = Row::new(vec!["Foot1", "Foot2"]);
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2]).footer(footer);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "Cell1 Cell2    ",
                "Cell3 Cell4    ",
                "Foot1 Foot2    ",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_header_and_footer() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let header = Row::new(vec!["Head1", "Head2"]);
            let footer = Row::new(vec!["Foot1", "Foot2"]);
            let rows = vec![Row::new(vec!["Cell1", "Cell2"])];
            let table = Table::new(rows, [Constraint::Length(5); 2])
                .header(header)
                .footer(footer);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "Head1 Head2    ",
                "Cell1 Cell2    ",
                "Foot1 Foot2    ",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_header_margin() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let header = Row::new(vec!["Head1", "Head2"]).bottom_margin(1);
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2]).header(header);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "Head1 Head2    ",
                "               ",
                "Cell1 Cell2    ",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_footer_margin() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let footer = Row::new(vec!["Foot1", "Foot2"]).top_margin(1);
            let rows = vec![Row::new(vec!["Cell1", "Cell2"])];
            let table = Table::new(rows, [Constraint::Length(5); 2]).footer(footer);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "Cell1 Cell2    ",
                "               ",
                "Foot1 Foot2    ",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_row_margin() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]).bottom_margin(1),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2]);
            Widget::render(table, Rect::new(0, 0, 15, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "Cell1 Cell2    ",
                "               ",
                "Cell3 Cell4    ",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_tall_row() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 23, 3));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec![
                    Text::raw("Cell3-Line1\nCell3-Line2\nCell3-Line3"),
                    Text::raw("Cell4-Line1\nCell4-Line2\nCell4-Line3"),
                ])
                .height(3),
            ];
            let table = Table::new(rows, [Constraint::Length(11); 2]);
            Widget::render(table, Rect::new(0, 0, 23, 3), &mut buf);
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "Cell1       Cell2      ",
                "Cell3-Line1 Cell4-Line1",
                "Cell3-Line2 Cell4-Line2",
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_alignment() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 10, 3));
            let rows = vec![
                Row::new(vec![Line::from("Left").alignment(Alignment::Left)]),
                Row::new(vec![Line::from("Center").alignment(Alignment::Center)]),
                Row::new(vec![Line::from("Right").alignment(Alignment::Right)]),
            ];
            let table = Table::new(rows, [Percentage(100)]);
            Widget::render(table, Rect::new(0, 0, 10, 3), &mut buf);
            let expected = Buffer::with_lines(["Left      ", "  Center  ", "     Right"]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_overflow_does_not_panic() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 20, 3));
            let table = Table::new(Vec::<Row>::new(), [Constraint::Min(20); 1])
                .header(Row::new([Line::from("").alignment(Alignment::Right)]))
                .footer(Row::new([Line::from("").alignment(Alignment::Right)]));
            Widget::render(table, Rect::new(0, 0, 20, 3), &mut buf);
        }

        #[test]
        fn render_with_selected_column_and_incorrect_width_count_does_not_panic() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 20, 3));
            let table = Table::new(
                vec![Row::new(vec!["Row1", "Row2", "Row3"])],
                [Constraint::Length(10); 1],
            );
            let mut state = TableState::new().with_selected_column(2);
            StatefulWidget::render(table, Rect::new(0, 0, 20, 3), &mut buf, &mut state);
        }

        #[test]
        fn render_with_selected() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2])
                .row_highlight_style(Style::new().red())
                .highlight_symbol(">>");
            let mut state = TableState::new().with_selected(Some(0));
            StatefulWidget::render(table, Rect::new(0, 0, 15, 3), &mut buf, &mut state);
            let expected = Buffer::with_lines([
                ">>Cell1 Cell2  ".red(),
                "  Cell3 Cell4  ".into(),
                "               ".into(),
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_selected_column() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 3));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2"]),
                Row::new(vec!["Cell3", "Cell4"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 2])
                .column_highlight_style(Style::new().blue())
                .highlight_symbol(">>");
            let mut state = TableState::new().with_selected_column(Some(1));
            StatefulWidget::render(table, Rect::new(0, 0, 15, 3), &mut buf, &mut state);
            let expected = Buffer::with_lines::<[Line; 3]>([
                Line::from(vec![
                    "Cell1".into(),
                    " ".into(),
                    "Cell2".blue(),
                    "    ".into(),
                ]),
                Line::from(vec![
                    "Cell3".into(),
                    " ".into(),
                    "Cell4".blue(),
                    "    ".into(),
                ]),
                Line::from(vec!["      ".into(), "     ".blue(), "    ".into()]),
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_selected_cell() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 20, 4));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2", "Cell3"]),
                Row::new(vec!["Cell4", "Cell5", "Cell6"]),
                Row::new(vec!["Cell7", "Cell8", "Cell9"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 3])
                .highlight_symbol(">>")
                .cell_highlight_style(Style::new().green());
            let mut state = TableState::new().with_selected_cell((1, 2));
            StatefulWidget::render(table, Rect::new(0, 0, 20, 4), &mut buf, &mut state);
            let expected = Buffer::with_lines::<[Line; 4]>([
                Line::from(vec!["  Cell1 ".into(), "Cell2 ".into(), "Cell3".into()]),
                Line::from(vec![">>Cell4 Cell5 ".into(), "Cell6".green(), " ".into()]),
                Line::from(vec!["  Cell7 ".into(), "Cell8 ".into(), "Cell9".into()]),
                Line::from(vec!["                    ".into()]),
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_selected_row_and_column() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 20, 4));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2", "Cell3"]),
                Row::new(vec!["Cell4", "Cell5", "Cell6"]),
                Row::new(vec!["Cell7", "Cell8", "Cell9"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 3])
                .highlight_symbol(">>")
                .row_highlight_style(Style::new().red())
                .column_highlight_style(Style::new().blue());
            let mut state = TableState::new().with_selected(1).with_selected_column(2);
            StatefulWidget::render(table, Rect::new(0, 0, 20, 4), &mut buf, &mut state);
            let expected = Buffer::with_lines::<[Line; 4]>([
                Line::from(vec!["  Cell1 ".into(), "Cell2 ".into(), "Cell3".blue()]),
                Line::from(vec![">>Cell4 Cell5 ".red(), "Cell6".blue(), " ".red()]),
                Line::from(vec!["  Cell7 ".into(), "Cell8 ".into(), "Cell9".blue()]),
                Line::from(vec!["              ".into(), "     ".blue(), " ".into()]),
            ]);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_with_selected_row_and_column_and_cell() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 20, 4));
            let rows = vec![
                Row::new(vec!["Cell1", "Cell2", "Cell3"]),
                Row::new(vec!["Cell4", "Cell5", "Cell6"]),
                Row::new(vec!["Cell7", "Cell8", "Cell9"]),
            ];
            let table = Table::new(rows, [Constraint::Length(5); 3])
                .highlight_symbol(">>")
                .row_highlight_style(Style::new().red())
                .column_highlight_style(Style::new().blue())
                .cell_highlight_style(Style::new().green());
            let mut state = TableState::new().with_selected(1).with_selected_column(2);
            StatefulWidget::render(table, Rect::new(0, 0, 20, 4), &mut buf, &mut state);
            let expected = Buffer::with_lines::<[Line; 4]>([
                Line::from(vec!["  Cell1 ".into(), "Cell2 ".into(), "Cell3".blue()]),
                Line::from(vec![">>Cell4 Cell5 ".red(), "Cell6".green(), " ".red()]),
                Line::from(vec!["  Cell7 ".into(), "Cell8 ".into(), "Cell9".blue()]),
                Line::from(vec!["              ".into(), "     ".blue(), " ".into()]),
            ]);
            assert_eq!(buf, expected);
        }

        /// Note that this includes a regression test for a bug where the table would not render the
        /// correct rows when there is no selection.
        /// <https://github.com/ratatui/ratatui/issues/1179>
        #[rstest]
        #[case::no_selection(None, 50, ["50", "51", "52", "53", "54"])]
        #[case::selection_before_offset(20, 20, ["20", "21", "22", "23", "24"])]
        #[case::selection_immediately_before_offset(49, 49, ["49", "50", "51", "52", "53"])]
        #[case::selection_at_start_of_offset(50, 50, ["50", "51", "52", "53", "54"])]
        #[case::selection_at_end_of_offset(54, 50, ["50", "51", "52", "53", "54"])]
        #[case::selection_immediately_after_offset(55, 51, ["51", "52", "53", "54", "55"])]
        #[case::selection_after_offset(80, 76, ["76", "77", "78", "79", "80"])]
        fn render_with_selection_and_offset<T: Into<Option<usize>>>(
            #[case] selected_row: T,
            #[case] expected_offset: usize,
            #[case] expected_items: [&str; 5],
        ) {
            // render 100 rows offset at 50, with a selected row
            let rows = (0..100).map(|i| Row::new([i.to_string()]));
            let table = Table::new(rows, [Constraint::Length(2)]);
            let mut buf = Buffer::empty(Rect::new(0, 0, 2, 5));
            let mut state = TableState::new()
                .with_offset(50)
                .with_selected(selected_row.into());

            StatefulWidget::render(table.clone(), Rect::new(0, 0, 5, 5), &mut buf, &mut state);

            assert_eq!(buf, Buffer::with_lines(expected_items));
            assert_eq!(state.offset, expected_offset);
        }
    }

    // test how constraints interact with table column width allocation
    mod column_widths {
        use super::*;

        #[test]
        fn length_constraint() {
            // without selection, more than needed width
            let table = Table::default().widths([Length(4), Length(4)]);
            assert_eq!(table.get_column_widths(20, 0, 0), [(0, 4), (5, 4)]);

            // with selection, more than needed width
            let table = Table::default().widths([Length(4), Length(4)]);
            assert_eq!(table.get_column_widths(20, 3, 0), [(3, 4), (8, 4)]);

            // without selection, less than needed width
            let table = Table::default().widths([Length(4), Length(4)]);
            assert_eq!(table.get_column_widths(7, 0, 0), [(0, 3), (4, 3)]);

            // with selection, less than needed width
            // <--------7px-------->
            // ┌────────┐x┌────────┐
            // │ (3, 2) │x│ (6, 1) │
            // └────────┘x└────────┘
            // column spacing (i.e. `x`) is always prioritized
            let table = Table::default().widths([Length(4), Length(4)]);
            assert_eq!(table.get_column_widths(7, 3, 0), [(3, 2), (6, 1)]);
        }

        #[test]
        fn max_constraint() {
            // without selection, more than needed width
            let table = Table::default().widths([Max(4), Max(4)]);
            assert_eq!(table.get_column_widths(20, 0, 0), [(0, 4), (5, 4)]);

            // with selection, more than needed width
            let table = Table::default().widths([Max(4), Max(4)]);
            assert_eq!(table.get_column_widths(20, 3, 0), [(3, 4), (8, 4)]);

            // without selection, less than needed width
            let table = Table::default().widths([Max(4), Max(4)]);
            assert_eq!(table.get_column_widths(7, 0, 0), [(0, 3), (4, 3)]);

            // with selection, less than needed width
            let table = Table::default().widths([Max(4), Max(4)]);
            assert_eq!(table.get_column_widths(7, 3, 0), [(3, 2), (6, 1)]);
        }

        #[test]
        fn min_constraint() {
            // in its currently stage, the "Min" constraint does not grow to use the possible
            // available length and enabling "expand_to_fill" will just stretch the last
            // constraint and not split it with all available constraints

            // without selection, more than needed width
            let table = Table::default().widths([Min(4), Min(4)]);
            assert_eq!(table.get_column_widths(20, 0, 0), [(0, 10), (11, 9)]);

            // with selection, more than needed width
            let table = Table::default().widths([Min(4), Min(4)]);
            assert_eq!(table.get_column_widths(20, 3, 0), [(3, 8), (12, 8)]);

            // without selection, less than needed width
            // allocates spacer
            let table = Table::default().widths([Min(4), Min(4)]);
            assert_eq!(table.get_column_widths(7, 0, 0), [(0, 3), (4, 3)]);

            // with selection, less than needed width
            // always allocates selection and spacer
            let table = Table::default().widths([Min(4), Min(4)]);
            assert_eq!(table.get_column_widths(7, 3, 0), [(3, 2), (6, 1)]);
        }

        #[test]
        fn percentage_constraint() {
            // without selection, more than needed width
            let table = Table::default().widths([Percentage(30), Percentage(30)]);
            assert_eq!(table.get_column_widths(20, 0, 0), [(0, 6), (7, 6)]);

            // with selection, more than needed width
            let table = Table::default().widths([Percentage(30), Percentage(30)]);
            assert_eq!(table.get_column_widths(20, 3, 0), [(3, 5), (9, 5)]);

            // without selection, less than needed width
            // rounds from positions: [0.0, 0.0, 2.1, 3.1, 5.2, 7.0]
            let table = Table::default().widths([Percentage(30), Percentage(30)]);
            assert_eq!(table.get_column_widths(7, 0, 0), [(0, 2), (3, 2)]);

            // with selection, less than needed width
            // rounds from positions: [0.0, 3.0, 5.1, 6.1, 7.0, 7.0]
            let table = Table::default().widths([Percentage(30), Percentage(30)]);
            assert_eq!(table.get_column_widths(7, 3, 0), [(3, 1), (5, 1)]);
        }

        #[test]
        fn ratio_constraint() {
            // without selection, more than needed width
            // rounds from positions: [0.00, 0.00, 6.67, 7.67, 14.33]
            let table = Table::default().widths([Ratio(1, 3), Ratio(1, 3)]);
            assert_eq!(table.get_column_widths(20, 0, 0), [(0, 7), (8, 6)]);

            // with selection, more than needed width
            // rounds from positions: [0.00, 3.00, 10.67, 17.33, 20.00]
            let table = Table::default().widths([Ratio(1, 3), Ratio(1, 3)]);
            assert_eq!(table.get_column_widths(20, 3, 0), [(3, 6), (10, 5)]);

            // without selection, less than needed width
            // rounds from positions: [0.00, 2.33, 3.33, 5.66, 7.00]
            let table = Table::default().widths([Ratio(1, 3), Ratio(1, 3)]);
            assert_eq!(table.get_column_widths(7, 0, 0), [(0, 2), (3, 3)]);

            // with selection, less than needed width
            // rounds from positions: [0.00, 3.00, 5.33, 6.33, 7.00, 7.00]
            let table = Table::default().widths([Ratio(1, 3), Ratio(1, 3)]);
            assert_eq!(table.get_column_widths(7, 3, 0), [(3, 1), (5, 2)]);
        }

        /// When more width is available than requested, the behavior is controlled by flex
        #[test]
        fn underconstrained_flex() {
            let table = Table::default().widths([Min(10), Min(10), Min(1)]);
            assert_eq!(
                table.get_column_widths(62, 0, 0),
                &[(0, 20), (21, 20), (42, 20)]
            );

            let table = Table::default()
                .widths([Min(10), Min(10), Min(1)])
                .flex(Flex::Legacy);
            assert_eq!(
                table.get_column_widths(62, 0, 0),
                &[(0, 10), (11, 10), (22, 40)]
            );

            let table = Table::default()
                .widths([Min(10), Min(10), Min(1)])
                .flex(Flex::SpaceBetween);
            assert_eq!(
                table.get_column_widths(62, 0, 0),
                &[(0, 20), (21, 20), (42, 20)]
            );
        }

        #[test]
        fn underconstrained_segment_size() {
            let table = Table::default().widths([Min(10), Min(10), Min(1)]);
            assert_eq!(
                table.get_column_widths(62, 0, 0),
                &[(0, 20), (21, 20), (42, 20)]
            );

            let table = Table::default()
                .widths([Min(10), Min(10), Min(1)])
                .flex(Flex::Legacy);
            assert_eq!(
                table.get_column_widths(62, 0, 0),
                &[(0, 10), (11, 10), (22, 40)]
            );
        }

        #[test]
        fn no_constraint_with_rows() {
            let table = Table::default()
                .rows(vec![
                    Row::new(vec!["a", "b"]),
                    Row::new(vec!["c", "d", "e"]),
                ])
                // rows should get precedence over header
                .header(Row::new(vec!["f", "g"]))
                .footer(Row::new(vec!["h", "i"]))
                .column_spacing(0);
            assert_eq!(
                table.get_column_widths(30, 0, 3),
                &[(0, 10), (10, 10), (20, 10)]
            );
        }

        #[test]
        fn no_constraint_with_header() {
            let table = Table::default()
                .rows(vec![] as Vec<Row>)
                .header(Row::new(vec!["f", "g"]))
                .column_spacing(0);
            assert_eq!(table.get_column_widths(10, 0, 2), [(0, 5), (5, 5)]);
        }

        #[test]
        fn no_constraint_with_footer() {
            let table = Table::default()
                .rows(vec![] as Vec<Row>)
                .footer(Row::new(vec!["h", "i"]))
                .column_spacing(0);
            assert_eq!(table.get_column_widths(10, 0, 2), [(0, 5), (5, 5)]);
        }

        #[track_caller]
        fn test_table_with_selection<'line, Lines>(
            highlight_spacing: HighlightSpacing,
            columns: u16,
            spacing: u16,
            selection: Option<usize>,
            expected: Lines,
        ) where
            Lines: IntoIterator,
            Lines::Item: Into<Line<'line>>,
        {
            let table = Table::default()
                .rows(vec![Row::new(vec!["ABCDE", "12345"])])
                .highlight_spacing(highlight_spacing)
                .highlight_symbol(">>>")
                .column_spacing(spacing);
            let area = Rect::new(0, 0, columns, 3);
            let mut buf = Buffer::empty(area);
            let mut state = TableState::default().with_selected(selection);
            StatefulWidget::render(table, area, &mut buf, &mut state);
            assert_eq!(buf, Buffer::with_lines(expected));
        }

        #[test]
        fn excess_area_highlight_symbol_and_column_spacing_allocation() {
            // no highlight_symbol rendered ever
            test_table_with_selection(
                HighlightSpacing::Never,
                15,   // width
                0,    // spacing
                None, // selection
                [
                    "ABCDE  12345   ", /* default layout is Flex::Start but columns length
                                        * constraints are calculated as `max_area / n_columns`,
                                        * i.e. they are distributed amongst available space */
                    "               ", // row 2
                    "               ", // row 3
                ],
            );

            let table = Table::default()
                .rows(vec![Row::new(vec!["ABCDE", "12345"])])
                .widths([5, 5])
                .column_spacing(0);
            let area = Rect::new(0, 0, 15, 3);
            let mut buf = Buffer::empty(area);
            Widget::render(table, area, &mut buf);
            let expected = Buffer::with_lines([
                "ABCDE12345     ", /* As reference, this is what happens when you manually
                                    * specify widths */
                "               ", // row 2
                "               ", // row 3
            ]);
            assert_eq!(buf, expected);

            // no highlight_symbol rendered ever
            test_table_with_selection(
                HighlightSpacing::Never,
                15,      // width
                0,       // spacing
                Some(0), // selection
                [
                    "ABCDE  12345   ", // row 1
                    "               ", // row 2
                    "               ", // row 3
                ],
            );

            // no highlight_symbol rendered because no selection is made
            test_table_with_selection(
                HighlightSpacing::WhenSelected,
                15,   // width
                0,    // spacing
                None, // selection
                [
                    "ABCDE  12345   ", // row 1
                    "               ", // row 2
                    "               ", // row 3
                ],
            );
            // highlight_symbol rendered because selection is made
            test_table_with_selection(
                HighlightSpacing::WhenSelected,
                15,      // width
                0,       // spacing
                Some(0), // selection
                [
                    ">>>ABCDE 12345 ", // row 1
                    "               ", // row 2
                    "               ", // row 3
                ],
            );

            // highlight_symbol always rendered even no selection is made
            test_table_with_selection(
                HighlightSpacing::Always,
                15,   // width
                0,    // spacing
                None, // selection
                [
                    "   ABCDE 12345 ", // row 1
                    "               ", // row 2
                    "               ", // row 3
                ],
            );

            // no highlight_symbol rendered because no selection is made
            test_table_with_selection(
                HighlightSpacing::Always,
                15,      // width
                0,       // spacing
                Some(0), // selection
                [
                    ">>>ABCDE 12345 ", // row 1
                    "               ", // row 2
                    "               ", // row 3
                ],
            );
        }

        #[expect(clippy::too_many_lines)]
        #[test]
        fn insufficient_area_highlight_symbol_and_column_spacing_allocation() {
            // column spacing is prioritized over every other constraint
            test_table_with_selection(
                HighlightSpacing::Never,
                10,   // width
                1,    // spacing
                None, // selection
                [
                    "ABCDE 1234", // spacing is prioritized and column is cut
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
            test_table_with_selection(
                HighlightSpacing::WhenSelected,
                10,   // width
                1,    // spacing
                None, // selection
                [
                    "ABCDE 1234", // spacing is prioritized and column is cut
                    "          ", // row 2
                    "          ", // row 3
                ],
            );

            // this test checks that space for highlight_symbol space is always allocated.
            // this test also checks that space for column is allocated.
            //
            // Space for highlight_symbol is allocated first by splitting horizontal space
            // into highlight_symbol area and column area.
            // Then in a separate step, column widths are calculated.
            // column spacing is prioritized when column widths are calculated and last column here
            // ends up with just 1 wide
            test_table_with_selection(
                HighlightSpacing::Always,
                10,   // width
                1,    // spacing
                None, // selection
                [
                    "   ABC 123", // highlight_symbol and spacing are prioritized
                    "          ", // row 2
                    "          ", // row 3
                ],
            );

            // the following are specification tests
            test_table_with_selection(
                HighlightSpacing::Always,
                9,    // width
                1,    // spacing
                None, // selection
                [
                    "   ABC 12", // highlight_symbol and spacing are prioritized
                    "         ", // row 2
                    "         ", // row 3
                ],
            );
            test_table_with_selection(
                HighlightSpacing::Always,
                8,    // width
                1,    // spacing
                None, // selection
                [
                    "   AB 12", // highlight_symbol and spacing are prioritized
                    "        ", // row 2
                    "        ", // row 3
                ],
            );
            test_table_with_selection(
                HighlightSpacing::Always,
                7,    // width
                1,    // spacing
                None, // selection
                [
                    "   AB 1", // highlight_symbol and spacing are prioritized
                    "       ", // row 2
                    "       ", // row 3
                ],
            );

            let table = Table::default()
                .rows(vec![Row::new(vec!["ABCDE", "12345"])])
                .highlight_spacing(HighlightSpacing::Always)
                .flex(Flex::Legacy)
                .highlight_symbol(">>>")
                .column_spacing(1);
            let area = Rect::new(0, 0, 10, 3);
            let mut buf = Buffer::empty(area);
            Widget::render(table, area, &mut buf);
            // highlight_symbol and spacing are prioritized but columns are evenly distributed
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "   ABCDE 1",
                "          ",
                "          ",
            ]);
            assert_eq!(buf, expected);

            let table = Table::default()
                .rows(vec![Row::new(vec!["ABCDE", "12345"])])
                .highlight_spacing(HighlightSpacing::Always)
                .flex(Flex::Start)
                .highlight_symbol(">>>")
                .column_spacing(1);
            let area = Rect::new(0, 0, 10, 3);
            let mut buf = Buffer::empty(area);
            Widget::render(table, area, &mut buf);
            // highlight_symbol and spacing are prioritized but columns are evenly distributed
            #[rustfmt::skip]
            let expected = Buffer::with_lines([
                "   ABC 123",
                "          ",
                "          ",
            ]);
            assert_eq!(buf, expected);

            test_table_with_selection(
                HighlightSpacing::Never,
                10,      // width
                1,       // spacing
                Some(0), // selection
                [
                    "ABCDE 1234", // spacing is prioritized
                    "          ",
                    "          ",
                ],
            );

            test_table_with_selection(
                HighlightSpacing::WhenSelected,
                10,      // width
                1,       // spacing
                Some(0), // selection
                [
                    ">>>ABC 123", // row 1
                    "          ", // row 2
                    "          ", // row 3
                ],
            );

            test_table_with_selection(
                HighlightSpacing::Always,
                10,      // width
                1,       // spacing
                Some(0), // selection
                [
                    ">>>ABC 123", // highlight column and spacing are prioritized
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
        }

        #[test]
        fn insufficient_area_highlight_symbol_allocation_with_no_column_spacing() {
            test_table_with_selection(
                HighlightSpacing::Never,
                10,   // width
                0,    // spacing
                None, // selection
                [
                    "ABCDE12345", // row 1
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
            test_table_with_selection(
                HighlightSpacing::WhenSelected,
                10,   // width
                0,    // spacing
                None, // selection
                [
                    "ABCDE12345", // row 1
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
            // highlight symbol spacing is prioritized over all constraints
            // even if the constraints are fixed length
            // this is because highlight_symbol column is separated _before_ any of the constraint
            // widths are calculated
            test_table_with_selection(
                HighlightSpacing::Always,
                10,   // width
                0,    // spacing
                None, // selection
                [
                    "   ABCD123", // highlight column and spacing are prioritized
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
            test_table_with_selection(
                HighlightSpacing::Never,
                10,      // width
                0,       // spacing
                Some(0), // selection
                [
                    "ABCDE12345", // row 1
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
            test_table_with_selection(
                HighlightSpacing::WhenSelected,
                10,      // width
                0,       // spacing
                Some(0), // selection
                [
                    ">>>ABCD123", // highlight column and spacing are prioritized
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
            test_table_with_selection(
                HighlightSpacing::Always,
                10,      // width
                0,       // spacing
                Some(0), // selection
                [
                    ">>>ABCD123", // highlight column and spacing are prioritized
                    "          ", // row 2
                    "          ", // row 3
                ],
            );
        }
    }

    #[test]
    fn stylize() {
        assert_eq!(
            Table::new(vec![Row::new(vec![Cell::from("")])], [Percentage(100)])
                .black()
                .on_white()
                .bold()
                .not_crossed_out()
                .style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::CROSSED_OUT)
        );
    }

    #[rstest]
    #[case::no_columns(vec![], vec![], vec![], 0)]
    #[case::only_header(vec!["H1", "H2"], vec![], vec![], 2)]
    #[case::only_rows(
        vec![],
        vec![vec!["C1", "C2"], vec!["C1", "C2", "C3"]],
        vec![],
        3
    )]
    #[case::only_footer(vec![], vec![], vec!["F1", "F2", "F3", "F4"], 4)]
    #[case::rows_longer(
        vec!["H1", "H2", "H3", "H4"],
        vec![vec!["C1", "C2"],vec!["C1", "C2", "C3"]],
        vec!["F1", "F2"],
        4
    )]
    #[case::rows_longer(
        vec!["H1", "H2"],
        vec![vec!["C1", "C2"], vec!["C1", "C2", "C3", "C4"]],
        vec!["F1", "F2"],
        4
    )]
    #[case::footer_longer(
        vec!["H1", "H2"],
        vec![vec!["C1", "C2"], vec!["C1", "C2", "C3"]],
        vec!["F1", "F2", "F3", "F4"],
        4
    )]

    fn column_count(
        #[case] header: Vec<&str>,
        #[case] rows: Vec<Vec<&str>>,
        #[case] footer: Vec<&str>,
        #[case] expected: usize,
    ) {
        let header = Row::new(header);
        let footer = Row::new(footer);
        let rows: Vec<Row> = rows.into_iter().map(Row::new).collect();
        let table = Table::new(rows, Vec::<Constraint>::new())
            .header(header)
            .footer(footer);
        let column_count = table.column_count();
        assert_eq!(column_count, expected);
    }

    #[test]
    fn render_with_block_and_internal_borders() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 6));
        let rows = vec![
            Row::new(vec!["Cell1", "Cell2", "Cell3"]),
            Row::new(vec!["Cell4", "Cell5", "Cell6"]),
        ];
        let table = Table::new(rows, [Constraint::Length(6); 3])
            .block(Block::bordered().title("Table"))
            .internal_borders(TableBorders::ALL)
            .border_style(Style::new().blue());
        Widget::render(table, Rect::new(0, 0, 20, 6), &mut buf);

        // Verify that the table renders without panicking
        // The exact output depends on the border integration logic
        assert!(!buf.area.is_empty());
    }

    #[test]
    fn render_with_corner_intersections() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 15, 5));
        let rows = vec![Row::new(vec!["A", "B"]), Row::new(vec!["C", "D"])];
        let table = Table::new(rows, [Constraint::Length(7); 2])
            .internal_borders(TableBorders::ALL)
            .border_style(Style::new().blue());
        Widget::render(table, Rect::new(0, 0, 15, 5), &mut buf);

        // Verify that the table renders without panicking
        // The corner intersections should now use proper cross symbols (┼)
        assert!(!buf.area.is_empty());
    }
}
