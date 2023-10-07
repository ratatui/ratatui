use strum::{Display, EnumString};
use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect, SegmentSize},
    style::{Style, Styled},
    text::Text,
    widgets::{Block, StatefulWidget, Widget},
};

/// A [`Cell`] contains the [`Text`] to be displayed in a [`Row`] of a [`Table`].
///
/// It can be created from anything that can be converted to a [`Text`].
/// ```rust
/// use std::borrow::Cow;
/// use ratatui::{prelude::*, widgets::*};
///
/// Cell::from("simple string");
///
/// Cell::from(Span::from("span"));
///
/// Cell::from(Line::from(vec![
///     Span::raw("a vec of "),
///     Span::styled("spans", Style::default().add_modifier(Modifier::BOLD))
/// ]));
///
/// Cell::from(Text::from("a text"));
///
/// Cell::from(Text::from(Cow::Borrowed("hello")));
/// ```
///
/// You can apply a [`Style`] on the entire [`Cell`] using [`Cell::style`] or rely on the styling
/// capabilities of [`Text`].
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Cell<'a> {
    content: Text<'a>,
    style: Style,
}

impl<'a> Cell<'a> {
    /// Set the `Style` of this cell.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a, T> From<T> for Cell<'a>
where
    T: Into<Text<'a>>,
{
    fn from(content: T) -> Cell<'a> {
        Cell {
            content: content.into(),
            style: Style::default(),
        }
    }
}

impl<'a> Styled for Cell<'a> {
    type Item = Cell<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}

/// Holds data to be displayed in a [`Table`] widget.
///
/// A [`Row`] is a collection of cells. It can be created from simple strings:
/// ```rust
/// use ratatui::{prelude::*, widgets::*};
///
/// Row::new(vec!["Cell1", "Cell2", "Cell3"]);
/// ```
///
/// But if you need a bit more control over individual cells, you can explicitly create [`Cell`]s:
/// ```rust
/// use ratatui::{prelude::*, widgets::*};
///
/// Row::new(vec![
///     Cell::from("Cell1"),
///     Cell::from("Cell2").style(Style::default().fg(Color::Yellow)),
/// ]);
/// ```
///
/// You can also construct a row from any type that can be converted into [`Text`]:
/// ```rust
/// use std::borrow::Cow;
/// use ratatui::{prelude::*, widgets::*};
///
/// Row::new(vec![
///     Cow::Borrowed("hello"),
///     Cow::Owned("world".to_uppercase()),
/// ]);
/// ```
///
/// By default, a row has a height of 1 but you can change this using [`Row::height`].
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Row<'a> {
    cells: Vec<Cell<'a>>,
    height: u16,
    style: Style,
    bottom_margin: u16,
}

impl<'a> Row<'a> {
    /// Creates a new [`Row`] from an iterator where items can be converted to a [`Cell`].
    pub fn new<T>(cells: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Cell<'a>>,
    {
        Self {
            height: 1,
            cells: cells.into_iter().map(Into::into).collect(),
            style: Style::default(),
            bottom_margin: 0,
        }
    }

    /// Set the fixed height of the [`Row`]. Any [`Cell`] whose content has more lines than this
    /// height will see its content truncated.
    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    /// Set the [`Style`] of the entire row. This [`Style`] can be overridden by the [`Style`] of a
    /// any individual [`Cell`] or event by their [`Text`] content.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the bottom margin. By default, the bottom margin is `0`.
    pub fn bottom_margin(mut self, margin: u16) -> Self {
        self.bottom_margin = margin;
        self
    }

    /// Returns the total height of the row.
    fn total_height(&self) -> u16 {
        self.height.saturating_add(self.bottom_margin)
    }
}

impl<'a> Styled for Row<'a> {
    type Item = Row<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}

/// This option allows the user to configure WHEN should the "highlight symbol" be drawn
#[derive(Debug, Display, EnumString, PartialEq, Eq, Clone, Default, Hash)]
pub enum HighlightSpacing {
    /// Always add spacing for the selection symbol column
    ///
    /// With this variant, the column for the selection symbol will always be allocated, and so the
    /// table will never change size, regardless of if a row is selected or not
    Always,
    /// Only add spacing for the selection symbol column if a row is selected
    ///
    /// With this variant, the column for the selection symbol will only be allocated if there is a
    /// selection, causing the table to shift if selected / unselected
    #[default]
    WhenSelected,
    /// Never add spacing to the selection symbol column, regardless of whether something is
    /// selected or not
    ///
    /// This means that the highlight symbol will never be drawn
    Never,
}

impl HighlightSpacing {
    /// Determine if a selection should be done, based on variant
    /// Input "selection_state" should be similar to `state.selected.is_some()`
    pub fn should_add(&self, selection_state: bool) -> bool {
        match self {
            HighlightSpacing::Always => true,
            HighlightSpacing::WhenSelected => selection_state,
            HighlightSpacing::Never => false,
        }
    }
}

/// This option allows the user to configure WHICH columns should draw the "highlight symbol"
///
/// This setting is ignored when [`HighlightSpacing`] is set to [`HighlightSpacing::Never`]
#[derive(Debug, Display, EnumString, PartialEq, Eq, Clone, Default, Hash)]
pub enum ColumnHighlightSpacing {
    /// Only add spacing for the highlight symbol to the left of the first column
    #[default]
    FirstColumnOnly,
    /// Add spacing for the highlight symbol only to the selected column
    SelectedColumn,
    /// Add spacing for the highlight symbol to the columns specified in the vector
    SpecificColumns(Vec<usize>),
    /// Add spacing for the highlight symbol to all columns
    AllColumns,
}

/// A widget to display data in formatted columns.
///
/// It is a collection of [`Row`]s, themselves composed of [`Cell`]s:
/// ```rust
/// use ratatui::{prelude::*, widgets::*};
///
/// Table::new(vec![
///     // Row can be created from simple strings.
///     Row::new(vec!["Row11", "Row12", "Row13"]),
///     // You can style the entire row.
///     Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::default().fg(Color::Blue)),
///     // If you need more control over the styling you may need to create Cells directly
///     Row::new(vec![
///         Cell::from("Row31"),
///         Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
///         Cell::from(Line::from(vec![
///             Span::raw("Row"),
///             Span::styled("33", Style::default().fg(Color::Green))
///         ])),
///     ]),
///     // If a Row need to display some content over multiple lines, you just have to change
///     // its height.
///     Row::new(vec![
///         Cell::from("Row\n41"),
///         Cell::from("Row\n42"),
///         Cell::from("Row\n43"),
///     ]).height(2),
/// ])
/// // You can set the style of the entire Table.
/// .style(Style::default().fg(Color::White))
/// // It has an optional header, which is simply a Row always visible at the top.
/// .header(
///     Row::new(vec!["Col1", "Col2", "Col3"])
///         .style(Style::default().fg(Color::Yellow))
///         // If you want some space between the header and the rest of the rows, you can always
///         // specify some margin at the bottom.
///         .bottom_margin(1)
/// )
/// // As any other widget, a Table can be wrapped in a Block.
/// .block(Block::default().title("Table"))
/// // Columns widths are constrained in the same way as Layout...
/// .widths(&[Constraint::Length(5), Constraint::Length(5), Constraint::Length(10)])
/// // ...and they can be separated by a fixed spacing.
/// .column_spacing(1)
/// // If you wish to highlight a row in any specific way when it is selected...
/// .row_highlight_style(Style::default().add_modifier(Modifier::BOLD))
/// // ... or a selected column...
/// .col_highlight_style(Style::default().bg(Color::Green))
/// // ... or a selected cell ..
/// .cell_highlight_style(Style::default().bg(Color::Red))
/// // ...and potentially show a symbol in front of the selection.
/// .highlight_symbol(">>")
/// // You can also specify when to show the highlight symbol
/// .highlight_spacing(HighlightSpacing::WhenSelected)
/// // You can also specify which columns should show the highlight symbol
/// .columns_with_highlight_spacing(ColumnHighlightSpacing::SelectedColumn);
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Table<'a> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Base style for the widget
    style: Style,
    /// Width constraints for each column
    widths: &'a [Constraint],
    /// Space between each column
    column_spacing: u16,
    /// Style used to render the selected cell
    cell_highlight_style: Style,
    /// Style used to render the selected row
    row_highlight_style: Style,
    /// Style used to render the selected column
    col_highlight_style: Style,
    /// Symbol in front of the selected rom
    highlight_symbol: Option<&'a str>,
    /// Optional header
    header: Option<Row<'a>>,
    /// Data to display in each row
    rows: Vec<Row<'a>>,
    /// Decides when to allocate spacing for the selection
    highlight_spacing: HighlightSpacing,
    /// Decides which columns to allocate spacing for the selection
    columns_with_highlight_spacing: ColumnHighlightSpacing,
}

impl<'a> Table<'a> {
    /// Creates a new [`Table`] widget with the given rows.
    ///
    /// The `rows` parameter is a Vector of [`Row`], this holds the data to be displayed by the
    /// table
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let table = Table::new(vec![
    ///     Row::new(vec![
    ///         Cell::from("Cell1"),
    ///         Cell::from("Cell2")
    ///     ]),
    ///     Row::new(vec![
    ///         Cell::from("Cell3"),
    ///         Cell::from("Cell4")
    ///     ]),
    /// ]);
    /// ```
    pub fn new<T>(rows: T) -> Self
    where
        T: IntoIterator<Item = Row<'a>>,
    {
        Self {
            block: None,
            style: Style::default(),
            widths: &[],
            column_spacing: 1,
            cell_highlight_style: Style::default(),
            row_highlight_style: Style::default(),
            col_highlight_style: Style::default(),
            highlight_symbol: None,
            header: None,
            rows: rows.into_iter().collect(),
            highlight_spacing: HighlightSpacing::default(),
            columns_with_highlight_spacing: ColumnHighlightSpacing::default(),
        }
    }

    /// Creates a custom block around a [`Table`] widget.
    ///
    /// The `block` parameter is of type [`Block`]. This holds the specified block to be
    /// created around the [`Table`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let table = Table::new(vec![
    ///     Row::new(vec![
    ///         Cell::from("Cell1"),
    ///         Cell::from("Cell2")
    ///     ]),
    ///     Row::new(vec![
    ///         Cell::from("Cell3"),
    ///         Cell::from("Cell4")
    ///     ]),
    /// ]).block(Block::default().title("Table"));
    /// ```
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Creates a header for a [`Table`] widget.
    ///
    /// The `header` parameter is of type [`Row`] and this holds the cells to be displayed at the
    /// top of the [`Table`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let table = Table::new(vec![
    ///     Row::new(vec![
    ///         Cell::from("Cell1"),
    ///         Cell::from("Cell2")
    ///     ])
    /// ]).header(
    ///     Row::new(vec![
    ///         Cell::from("Header Cell 1"),
    ///         Cell::from("Header Cell 2")
    ///     ])
    /// );
    /// ```
    pub fn header(mut self, header: Row<'a>) -> Self {
        self.header = Some(header);
        self
    }

    /// Sets the widths for the columns of the table
    ///
    /// The `widths` parameter is a slice of [`Constraint`]s. This holds the widths for each column
    /// You should also consider the [`Table::column_spacing`] method to set the spacing between
    pub fn widths(mut self, widths: &'a [Constraint]) -> Self {
        let between_0_and_100 = |&w| match w {
            Constraint::Percentage(p) => p <= 100,
            _ => true,
        };
        assert!(
            widths.iter().all(between_0_and_100),
            "Percentages should be between 0 and 100 inclusively."
        );
        self.widths = widths;
        self
    }

    /// Sets the style for the whole table
    ///
    /// This style will be used as a base for all the other styles of the table
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the highlight symbol for the table
    ///
    /// This symbol will be displayed in front of the selected row or cell
    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> Self {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    /// Applies the given [`Style`] to the selected row.
    // #[deprecated] --> Had to comment this because it has a conflict in `cargo make` with the same
    // method in the [`Tabs`] Widget
    pub fn highlight_style(mut self, highlight_style: Style) -> Self {
        // We emulate the behaviour before deprecating the method to avoid
        // breaking changes. Applies `highlight_style` to `row_highlight_style`
        self.row_highlight_style = highlight_style;
        self
    }

    /// Applies the given [`Style`] to the selected row.
    ///
    /// This highlight [`Style`] will be applied first, before the highlight [`Style`] in
    /// [`table.col_highlight_style()`] and [`table.cell_highlight_style()`]
    pub fn row_highlight_style(mut self, row_highlight_style: Style) -> Self {
        self.row_highlight_style = row_highlight_style;
        self
    }

    /// Applies the given [`Style`] to the selected column.
    ///
    /// This highlighting [`Style`] will be applied after the highlight [`Style`] in
    /// [`table.row_highlight_style()`], but before the highlight [`Style`] in
    /// [`table.cell_highlight_style()`]
    pub fn col_highlight_style(mut self, col_highlight_style: Style) -> Self {
        self.col_highlight_style = col_highlight_style;
        self
    }

    /// Applies the given [`Style`] to the selected cell.
    ///
    /// This highlighting [`Style`] will be applied last, after the highlight [`Style`] in
    /// [`table.row_highlight_style()`] and [`table.col_highlight_style()`]
    pub fn cell_highlight_style(mut self, cell_highlight_style: Style) -> Self {
        self.cell_highlight_style = cell_highlight_style;
        self
    }

    /// Set when to show the highlight spacing
    ///
    /// See [`HighlightSpacing`] about which variant affects spacing in which way
    pub fn highlight_spacing(mut self, value: HighlightSpacing) -> Self {
        self.highlight_spacing = value;
        self
    }

    /// Set which columns should show the highlight spacing
    ///
    /// See [`ColumnHighlightSpacing`] about which variant affects spacing in which way
    pub fn columns_with_highlight_spacing(mut self, value: ColumnHighlightSpacing) -> Self {
        self.columns_with_highlight_spacing = value;
        self
    }

    /// Sets the spacing between each column
    pub fn column_spacing(mut self, spacing: u16) -> Self {
        self.column_spacing = spacing;
        self
    }

    /// Gets the index of the columns that should have spacing for highlihgting symbol
    fn get_columns_with_spacing(&self, state: &TableState) -> Vec<usize> {
        match &self.columns_with_highlight_spacing {
            ColumnHighlightSpacing::FirstColumnOnly => {
                vec![0]
            }
            ColumnHighlightSpacing::SelectedColumn => vec![state.selected_col().unwrap_or(0)],
            ColumnHighlightSpacing::SpecificColumns(vec) => vec.clone(),
            ColumnHighlightSpacing::AllColumns => self
                .widths
                .iter()
                .enumerate()
                .map(|(i, _)| i)
                .collect::<Vec<usize>>(),
        }
    }

    /// Get all offsets and widths of all user specified columns
    /// Returns (x, width)
    fn get_columns_widths(
        &self,
        max_width: u16,
        selection_width: u16,
        state: &TableState,
    ) -> Vec<(u16, u16)> {
        let columns_with_spacing = self.get_columns_with_spacing(state); // Get cols with highlight symbol spacing
        let mut highlight_symbol_indexes: Vec<usize> = Vec::new(); //track cols with highlight symbol

        // We need to generate 1 constraint for each column + 1 constraint for the space between
        // each column + 1 constraint for the selection symbol spacing.. (This generates 1
        // space extra that we will `pop()` later)
        let mut constraints =
            Vec::with_capacity(self.widths.len() * 2 + columns_with_spacing.len());

        let mut curr_index: usize = 0; // This represents where we are in the constraints vector
        for (col_num, col_width) in self.widths.iter().enumerate() {
            // If we have it for this column, add a constraint for the selection symbol
            if columns_with_spacing.contains(&col_num) {
                constraints.push(Constraint::Length(selection_width));
                highlight_symbol_indexes.push(curr_index);
                curr_index += 1;
            }

            // Add the constraint for the column width
            constraints.push(*col_width);

            // Add the constraint for the column spacing
            constraints.push(Constraint::Length(self.column_spacing));
            curr_index += 2;
        }
        if !self.widths.is_empty() {
            // remove last column spacing
            constraints.pop();
        }
        // Create a layout with the constraints
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .segment_size(SegmentSize::None)
            .split(Rect {
                x: 0,
                y: 0,
                width: max_width,
                height: 1,
            });

        // Skip and filter spacing constraints
        // Return the start coordinate (x) and the width of each
        chunks
            .iter()
            .enumerate()
            .filter(|(i, _)| !highlight_symbol_indexes.contains(i)) //skip symbol spacing constraint
            .map(|(_, c)| (c.x, c.width))
            .step_by(2) // skip column spacing constraint
            .collect()
    }

    /// Returns the index of the visible top and bottom rows of the table
    fn get_row_bounds(
        &self,
        selected: Option<TableSelection>,
        offset: usize,
        max_height: u16,
    ) -> (usize, usize) {
        let offset = offset.min(self.rows.len().saturating_sub(1));
        let mut start = offset;
        let mut end = offset;
        let mut height = 0;

        // Find the current visible top and bottom rows
        for item in self.rows.iter().skip(offset) {
            if height + item.height > max_height {
                break;
            }
            height += item.total_height();
            end += 1;
        }

        // Get the current selected row index. If none selected, return current bounds
        let selected = match selected {
            Some(selection) => match selection {
                TableSelection::Row(row) => row,
                TableSelection::Col(_) => 0,
                TableSelection::Cell { row, .. } => row,
            },
            None => return (start, end),
        }
        .min(self.rows.len() - 1); // clamp to last row

        // Shift visible rows by one until selection is visible
        while selected >= end {
            height = height.saturating_add(self.rows[end].total_height());
            end += 1;
            while height > max_height {
                height = height.saturating_sub(self.rows[start].total_height());
                start += 1;
            }
        }

        // Shift visible rows by minus one until selection is visible
        while selected < start {
            start -= 1;
            height = height.saturating_add(self.rows[start].total_height());
            while height > max_height {
                end -= 1;
                height = height.saturating_sub(self.rows[end].total_height());
            }
        }
        (start, end)
    }
}

impl<'a> Styled for Table<'a> {
    type Item = Table<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}

/// An Enum to represent the current selection of a [`Table`]
///
/// Use [`TableSelection::Row`] to select a row.
/// Use [`TableSelection::Col`] to select a column.
/// Use [`TableSelection::Cell`] to select a cell (row and column).
/// Use [`TableSelection::From<usize>(row)`] to select a row with a usize.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TableSelection {
    /// Select a Row, with index `usize`
    Row(usize),
    /// Select a Column, with index `usize`
    Col(usize),
    /// Select a Cell, with index `usize` for row and `usize` for column
    Cell { row: usize, col: usize },
}

impl Default for TableSelection {
    fn default() -> Self {
        Self::Cell { row: 0, col: 0 }
    }
}

impl From<usize> for TableSelection {
    /// Convert from a usize to a TableSelection::Row, to preserve backwards compatibility
    fn from(row: usize) -> Self {
        Self::Row(row)
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct TableState {
    offset: usize,
    selected: Option<TableSelection>,
}

impl TableState {
    /// Returns the current offset. The offset is the index of the first row to render.
    ///
    /// This is used to implement scrolling. The offset is the index of the first row to render
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Mutable reference to the offset. The offset is the index of the first row to render.
    ///
    /// This is used to implement scrolling. The offset is the index of the first row to render
    pub fn offset_mut(&mut self) -> &mut usize {
        &mut self.offset
    }

    /// Sets the current selected index and returns Self for chaining methods
    ///
    /// If the index is [`None`], the selection will be cleared
    /// You can Select a row, column or cell with the [`TableSelection`] enum
    /// Or you can pass a usize to select a row
    pub fn with_selected(mut self, selected: Option<impl Into<TableSelection>>) -> Self {
        match selected {
            Some(index) => {
                self.selected = Some(index.into());
            }
            None => self.selected = None,
        }
        self
    }

    /// Sets the current offset and returns Self for chaining methods
    ///
    /// The offset is the index of the first row to render in a scrolled table
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Returns the current selected index, if any
    #[deprecated]
    pub fn selected(&self) -> Option<usize> {
        self.selected_row()
    }

    /// Returns the current selected index, if any
    pub fn selection(&self) -> Option<TableSelection> {
        self.selected
    }

    /// Returns the current selected row index, if any
    pub fn selected_row(&self) -> Option<usize> {
        match self.selected {
            Some(TableSelection::Row(row)) => Some(row),
            Some(TableSelection::Cell { row, .. }) => Some(row),
            _ => None,
        }
    }

    /// Returns the current selected col index, if any
    pub fn selected_col(&self) -> Option<usize> {
        match self.selected {
            Some(TableSelection::Col(col)) => Some(col),
            Some(TableSelection::Cell { col, .. }) => Some(col),
            _ => None,
        }
    }

    /// Sets the current selection
    ///
    /// If the index is [`None`], the selection will be cleared
    /// You can Select a row, column or cell with the [`TableSelection`] enum
    /// Or you can pass a usize to select a row
    pub fn select(&mut self, index: Option<impl Into<TableSelection>>) {
        self.selected = match index {
            Some(index) => Some(index.into()),
            None => {
                self.offset = 0;
                None
            }
        };
    }

    /// Adds, update or removes the row component of the [`TableSelection`]
    ///
    /// If current TableSelection is [`None`], it will be set as [`TableSelection::Row`]
    /// If current is [`TableSelection::Col`], it will be set as [`TableSelection::Cell`],
    /// If current is [`TableSelection::Cell`] and you set row to [`None`], the selection will be
    ///     turned into a [`TableSelection::Col`]
    pub fn select_row(&mut self, row: Option<usize>) {
        match row {
            Some(new_row) => match self.selection() {
                None | Some(TableSelection::Row(_)) => {
                    self.select(Some(TableSelection::Row(new_row)));
                }
                Some(TableSelection::Col(col)) => {
                    self.select(Some(TableSelection::Cell { row: new_row, col }));
                }
                Some(TableSelection::Cell { col, .. }) => {
                    self.select(Some(TableSelection::Cell { row: new_row, col }));
                }
            },
            None => match self.selection() {
                Some(TableSelection::Cell { col, .. }) => {
                    self.select(Some(TableSelection::Col(col)));
                }
                Some(TableSelection::Row(_)) => {
                    self.select(None::<TableSelection>);
                }
                _ => {}
            },
        }
    }

    /// Adds, updates or removes the column component of the [`TableSelection`]
    ///
    /// If the current [`TableSelection`] is [`None`], it will be set as a [`TableSelection::Col`]
    /// If  current is [`TableSelection::Row`], it will be set as a [`TableSelection::Cell`]
    /// If current is [`TableSelection::Cell`] and you set col to [`None`], the selection will be
    ///     turned into a [`TableSelection::Row`]
    pub fn select_col(&mut self, col: Option<usize>) {
        match col {
            Some(new_col) => match self.selection() {
                None | Some(TableSelection::Col(_)) => {
                    self.select(Some(TableSelection::Col(new_col)));
                }
                Some(TableSelection::Row(row)) => {
                    self.select(Some(TableSelection::Cell { row, col: new_col }))
                }
                Some(TableSelection::Cell { row, .. }) => {
                    self.select(Some(TableSelection::Cell { row, col: new_col }))
                }
            },
            None => match self.selection() {
                Some(TableSelection::Cell { row, .. }) => {
                    self.select(Some(TableSelection::Row(row)))
                }
                Some(TableSelection::Col(_)) => {
                    self.select(None::<TableSelection>);
                }
                _ => {}
            },
        }
    }
}

impl Table<'_> {
    fn render_header(
        &self,
        table_area: Rect,
        buf: &mut Buffer,
        current_height: &mut u16,
        columns_widths: &Vec<(u16, u16)>,
        rows_height: &mut u16,
    ) {
        if let Some(ref header) = self.header {
            let max_header_height = table_area.height.min(header.total_height());
            buf.set_style(
                Rect {
                    x: table_area.left(),
                    y: table_area.top(),
                    width: table_area.width,
                    height: table_area.height.min(header.height),
                },
                header.style,
            );
            let inner_offset = table_area.left();
            for ((x, width), cell) in columns_widths.iter().zip(header.cells.iter()) {
                render_cell(
                    buf,
                    cell,
                    Rect {
                        x: inner_offset + x,
                        y: table_area.top(),
                        width: *width,
                        height: max_header_height,
                    },
                );
            }
            *current_height += max_header_height;
            *rows_height = rows_height.saturating_sub(max_header_height);
        }
    }
}

impl<'a> StatefulWidget for Table<'a> {
    type State = TableState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.area() == 0 {
            return;
        }
        buf.set_style(area, self.style);
        let table_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let selection_width = if self.highlight_spacing.should_add(state.selected.is_some()) {
            self.highlight_symbol.map_or(0, |s| s.width() as u16)
        } else {
            0
        };
        let columns_widths = self.get_columns_widths(table_area.width, selection_width, state);
        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let mut current_height = 0;
        let mut rows_height = table_area.height;

        // Draw header
        self.render_header(
            table_area,
            buf,
            &mut current_height,
            &columns_widths,
            &mut rows_height,
        );

        // Draw rows
        if self.rows.is_empty() {
            return;
        }
        let (start, end) = self.get_row_bounds(state.selected, state.offset, rows_height);
        state.offset = start;

        let cols_with_symbol_spacing = self.get_columns_with_spacing(state);
        // Loop through each row
        for (row_num, table_row) in self
            .rows
            .iter_mut()
            .enumerate()
            .skip(state.offset)
            .take(end - start)
        {
            let (row, inner_offset) = (table_area.top() + current_height, table_area.left());
            current_height += table_row.total_height();
            let table_row_area = Rect {
                x: inner_offset,
                y: row,
                width: table_area.width,
                height: table_row.height,
            };
            buf.set_style(table_row_area, table_row.style);

            let selected = state.selection();
            let is_selected_row = match selected {
                Some(TableSelection::Row(row)) => row == row_num,
                Some(TableSelection::Cell { row, .. }) => row == row_num,
                _ => false,
            };

            // Loop trough each column in row (i.e loop through each cell)
            for ((col_num, (x, width)), cell) in columns_widths
                .iter()
                .enumerate()
                .zip(table_row.cells.iter())
            {
                let should_show_symbol_in_col = cols_with_symbol_spacing.contains(&col_num);
                let is_selected_col = match selected {
                    Some(TableSelection::Col(col)) => col == col_num,
                    Some(TableSelection::Cell { col, .. }) => col == col_num,
                    _ => false,
                };
                let is_selected_cell = match selected {
                    None => false,
                    Some(TableSelection::Row(_)) => is_selected_row && col_num == 0,
                    Some(TableSelection::Col(_)) => is_selected_col && row_num == 0,
                    Some(TableSelection::Cell { .. }) => is_selected_row && is_selected_col,
                };

                if selection_width > 0 && is_selected_cell && should_show_symbol_in_col {
                    // if selection_width > 0 && is_selected_cell {
                    // this should in normal cases be safe, because "get_columns_widths" allocates
                    // "highlight_symbol.width()" space but "get_columns_widths"
                    // currently does not bind it to max table.width()
                    buf.set_stringn(
                        inner_offset + x - selection_width,
                        row,
                        highlight_symbol,
                        table_area.width as usize,
                        table_row.style,
                    );
                };
                let table_cell_area = Rect {
                    x: inner_offset + x,
                    y: row,
                    width: *width,
                    height: table_row.height,
                };

                render_cell(buf, cell, table_cell_area);
                if is_selected_cell {
                    // Highlight ROW first
                    buf.set_style(table_row_area, self.row_highlight_style);

                    // Then, highlight column (on top of row)
                    let vertical_rect = Rect {
                        x: inner_offset + x,
                        y: table_area.top(),
                        width: *width,
                        height: table_area.height,
                    };
                    buf.set_style(vertical_rect, self.col_highlight_style);

                    // Finally, highlight cell (on top of row and column)
                    buf.set_style(table_cell_area, self.cell_highlight_style);
                }
            }
        }
    }
}

fn render_cell(buf: &mut Buffer, cell: &Cell, area: Rect) {
    buf.set_style(area, cell.style);
    for (i, line) in cell.content.lines.iter().enumerate() {
        if i as u16 >= area.height {
            break;
        }

        let x_offset = match line.alignment {
            Some(Alignment::Center) => (area.width / 2).saturating_sub(line.width() as u16 / 2),
            Some(Alignment::Right) => area.width.saturating_sub(line.width() as u16),
            _ => 0,
        };

        buf.set_line(area.x + x_offset, area.y + i as u16, line, area.width);
    }
}

impl<'a> Widget for Table<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = TableState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::{
        layout::Constraint::*,
        style::{Color, Modifier, Style, Stylize},
        text::Line,
    };
    #[test]
    #[should_panic]
    fn table_invalid_percentages() {
        Table::new(vec![]).widths(&[Constraint::Percentage(110)]);
    }

    // test how constraints interact with table column width allocation
    mod table_column_widths {
        use super::*;

        /// Construct a a new table with the given constraints, available and selection widths and
        /// tests that the widths match the expected list of (x, width) tuples.
        #[track_caller]
        fn test(
            constraints: &[Constraint],
            available_width: u16,
            selection_width: u16,
            expected: &[(u16, u16)],
        ) {
            let table = Table::new(vec![]).widths(constraints);

            let widths =
                table.get_columns_widths(available_width, selection_width, &TableState::default());
            assert_eq!(widths, expected);
        }

        #[test]
        fn length_constraint() {
            // without selection, more than needed width
            test(&[Length(4), Length(4)], 20, 0, &[(0, 4), (5, 4)]);

            // with selection, more than needed width
            test(&[Length(4), Length(4)], 20, 3, &[(3, 4), (8, 4)]);

            // without selection, less than needed width
            test(&[Length(4), Length(4)], 7, 0, &[(0, 4), (5, 2)]);

            // with selection, less than needed width
            test(&[Length(4), Length(4)], 7, 3, &[(3, 4), (7, 0)]);
        }

        #[test]
        fn max_constraint() {
            // without selection, more than needed width
            test(&[Max(4), Max(4)], 20, 0, &[(0, 4), (5, 4)]);

            // with selection, more than needed width
            test(&[Max(4), Max(4)], 20, 3, &[(3, 4), (8, 4)]);

            // without selection, less than needed width
            test(&[Max(4), Max(4)], 7, 0, &[(0, 4), (5, 2)]);

            // with selection, less than needed width
            test(&[Max(4), Max(4)], 7, 3, &[(3, 3), (7, 0)]);
        }

        #[test]
        fn min_constraint() {
            // in its currently stage, the "Min" constraint does not grow to use the possible
            // available length and enabling "expand_to_fill" will just stretch the last
            // constraint and not split it with all available constraints

            // without selection, more than needed width
            test(&[Min(4), Min(4)], 20, 0, &[(0, 4), (5, 4)]);

            // with selection, more than needed width
            test(&[Min(4), Min(4)], 20, 3, &[(3, 4), (8, 4)]);

            // without selection, less than needed width
            // allocates no spacer
            test(&[Min(4), Min(4)], 7, 0, &[(0, 4), (4, 3)]);

            // with selection, less than needed width
            // allocates no selection and no spacer
            test(&[Min(4), Min(4)], 7, 3, &[(0, 4), (4, 3)]);
        }

        #[test]
        fn percentage_constraint() {
            // without selection, more than needed width
            test(&[Percentage(30), Percentage(30)], 20, 0, &[(0, 6), (7, 6)]);

            // with selection, more than needed width
            test(&[Percentage(30), Percentage(30)], 20, 3, &[(3, 6), (10, 6)]);

            // without selection, less than needed width
            // rounds from positions: [0.0, 0.0, 2.1, 3.1, 5.2, 7.0]
            test(&[Percentage(30), Percentage(30)], 7, 0, &[(0, 2), (3, 2)]);

            // with selection, less than needed width
            // rounds from positions: [0.0, 3.0, 5.1, 6.1, 7.0, 7.0]
            test(&[Percentage(30), Percentage(30)], 7, 3, &[(3, 2), (6, 1)]);
        }

        #[test]
        fn ratio_constraint() {
            // without selection, more than needed width
            // rounds from positions: [0.00, 0.00, 6.67, 7.67, 14.33]
            test(&[Ratio(1, 3), Ratio(1, 3)], 20, 0, &[(0, 7), (8, 6)]);

            // with selection, more than needed width
            // rounds from positions: [0.00, 3.00, 10.67, 17.33, 20.00]
            test(&[Ratio(1, 3), Ratio(1, 3)], 20, 3, &[(3, 7), (11, 6)]);

            // without selection, less than needed width
            // rounds from positions: [0.00, 2.33, 3.33, 5.66, 7.00]
            test(&[Ratio(1, 3), Ratio(1, 3)], 7, 0, &[(0, 2), (3, 3)]);

            // with selection, less than needed width
            // rounds from positions: [0.00, 3.00, 5.33, 6.33, 7.00, 7.00]
            test(&[Ratio(1, 3), Ratio(1, 3)], 7, 3, &[(3, 2), (6, 1)]);
        }
    }

    #[test]
    fn test_render_table_with_alignment() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 3));
        let table = Table::new(vec![
            Row::new(vec![Line::from("Left").alignment(Alignment::Left)]),
            Row::new(vec![Line::from("Center").alignment(Alignment::Center)]),
            Row::new(vec![Line::from("Right").alignment(Alignment::Right)]),
        ])
        .widths(&[Percentage(100)]);

        Widget::render(table, Rect::new(0, 0, 20, 3), &mut buf);

        let expected = Buffer::with_lines(vec![
            "Left                ",
            "       Center       ",
            "               Right",
        ]);

        assert_eq!(buf, expected);
    }

    #[test]
    fn cell_can_be_stylized() {
        assert_eq!(
            Cell::from("").black().on_white().bold().not_dim().style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::DIM)
        )
    }

    #[test]
    fn row_can_be_stylized() {
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
        )
    }

    #[test]
    fn table_can_be_stylized() {
        assert_eq!(
            Table::new(vec![Row::new(vec![Cell::from("")])])
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
        )
    }

    #[test]
    fn highlight_spacing_to_string() {
        assert_eq!(HighlightSpacing::Always.to_string(), "Always".to_string());
        assert_eq!(
            HighlightSpacing::WhenSelected.to_string(),
            "WhenSelected".to_string()
        );
        assert_eq!(HighlightSpacing::Never.to_string(), "Never".to_string());
    }

    #[test]
    fn highlight_spacing_from_str() {
        assert_eq!(
            "Always".parse::<HighlightSpacing>(),
            Ok(HighlightSpacing::Always)
        );
        assert_eq!(
            "WhenSelected".parse::<HighlightSpacing>(),
            Ok(HighlightSpacing::WhenSelected)
        );
        assert_eq!(
            "Never".parse::<HighlightSpacing>(),
            Ok(HighlightSpacing::Never)
        );
        assert_eq!(
            "".parse::<HighlightSpacing>(),
            Err(strum::ParseError::VariantNotFound)
        );
    }
}
