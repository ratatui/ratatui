use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Styled},
    text::Text,
    widgets::{Block, StatefulWidget, Widget},
};

/// A [`Cell`] contains the [`Text`] to be displayed in a [`Row`] of a [`Table`].
///
/// It can be created from anything that can be converted to a [`Text`].
/// ```rust
/// # use ratatui::widgets::Cell;
/// # use ratatui::style::{Style, Modifier};
/// # use ratatui::text::{Span, Line, Text};
/// # use std::borrow::Cow;
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
/// # use ratatui::widgets::Row;
/// Row::new(vec!["Cell1", "Cell2", "Cell3"]);
/// ```
///
/// But if you need a bit more control over individual cells, you can explicitly create [`Cell`]s:
/// ```rust
/// # use ratatui::widgets::{Row, Cell};
/// # use ratatui::style::{Style, Color};
/// Row::new(vec![
///     Cell::from("Cell1"),
///     Cell::from("Cell2").style(Style::default().fg(Color::Yellow)),
/// ]);
/// ```
///
/// You can also construct a row from any type that can be converted into [`Text`]:
/// ```rust
/// # use std::borrow::Cow;
/// # use ratatui::widgets::Row;
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

/// This option allows to configure the behavior of the "highlight symbol" column width allocation
#[derive(Debug, PartialEq, Eq, Clone, Default, Hash)]
pub enum HighlightSpacing {
    /// Always add spacing for the row selection symbol
    ///
    /// With this variant, the column for the selection symbol will always be allocated, and so the
    /// table will never change size, regardless of if a row is selected or not
    Always,
    /// Only add spacing for the row selection symbol, if a row is selected
    ///
    /// With this variant, the column for the selection symbol will only be allocated if there is a
    /// selection, causing the table to shift if selected / unselected
    #[default]
    WhenSelected,
    /// Never add spacing selection symbol spacing, regardless of if something is selected or not
    ///
    /// This effectively changes that the highlight symbol will never be drawn
    Never,
}

/// A widget to display data in formatted columns.
///
/// It is a collection of [`Row`]s, themselves composed of [`Cell`]s:
/// ```rust
/// # use ratatui::widgets::{Block, Borders, Table, Row, Cell};
/// # use ratatui::layout::Constraint;
/// # use ratatui::style::{Style, Color, Modifier};
/// # use ratatui::text::{Text, Line, Span};
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
/// .highlight_style(Style::default().add_modifier(Modifier::BOLD))
/// // ...and potentially show a symbol in front of the selection.
/// .highlight_symbol(">>");
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
    /// Style used to render the selected row
    highlight_style: Style,
    /// Symbol in front of the selected rom
    highlight_symbol: Option<&'a str>,
    /// Optional header
    header: Option<Row<'a>>,
    /// Data to display in each row
    rows: Vec<Row<'a>>,
    /// Decides when to allocate spacing for the row selection
    highlight_spacing: HighlightSpacing,
}

impl<'a> Table<'a> {
    pub fn new<T>(rows: T) -> Self
    where
        T: IntoIterator<Item = Row<'a>>,
    {
        Self {
            block: None,
            style: Style::default(),
            widths: &[],
            column_spacing: 1,
            highlight_style: Style::default(),
            highlight_symbol: None,
            header: None,
            rows: rows.into_iter().collect(),
            highlight_spacing: HighlightSpacing::default(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn header(mut self, header: Row<'a>) -> Self {
        self.header = Some(header);
        self
    }

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

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> Self {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, highlight_style: Style) -> Self {
        self.highlight_style = highlight_style;
        self
    }

    /// Set which style of selection space allocation to use
    pub fn highlight_set_selection_space(mut self, value: HighlightSpacing) -> Self {
        self.highlight_spacing = value;
        self
    }

    pub fn column_spacing(mut self, spacing: u16) -> Self {
        self.column_spacing = spacing;
        self
    }

    fn get_columns_widths(&self, max_width: u16, add_selection_space: bool) -> Vec<u16> {
        let mut constraints = Vec::with_capacity(self.widths.len() * 2 + 1);
        if add_selection_space {
            let highlight_symbol_width = self.highlight_symbol.map_or(0, |s| s.width() as u16);
            constraints.push(Constraint::Length(highlight_symbol_width));
        }
        for constraint in self.widths {
            constraints.push(*constraint);
            constraints.push(Constraint::Length(self.column_spacing));
        }
        if !self.widths.is_empty() {
            constraints.pop();
        }
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .expand_to_fill(false)
            .split(Rect {
                x: 0,
                y: 0,
                width: max_width,
                height: 1,
            });
        let mut chunks = &chunks[..];
        if add_selection_space {
            chunks = &chunks[1..];
        }
        chunks.iter().step_by(2).map(|c| c.width).collect()
    }

    fn get_row_bounds(
        &self,
        selected: Option<usize>,
        offset: usize,
        max_height: u16,
    ) -> (usize, usize) {
        let offset = offset.min(self.rows.len().saturating_sub(1));
        let mut start = offset;
        let mut end = offset;
        let mut height = 0;
        for item in self.rows.iter().skip(offset) {
            if height + item.height > max_height {
                break;
            }
            height += item.total_height();
            end += 1;
        }

        let selected = selected.unwrap_or(0).min(self.rows.len() - 1);
        while selected >= end {
            height = height.saturating_add(self.rows[end].total_height());
            end += 1;
            while height > max_height {
                height = height.saturating_sub(self.rows[start].total_height());
                start += 1;
            }
        }
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

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct TableState {
    offset: usize,
    selected: Option<usize>,
}

impl TableState {
    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn offset_mut(&mut self) -> &mut usize {
        &mut self.offset
    }

    pub fn with_selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
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

        let add_selection_space = match self.highlight_spacing {
            HighlightSpacing::Always => true,
            HighlightSpacing::WhenSelected => state.selected.is_some(),
            HighlightSpacing::Never => false,
        };
        let columns_widths = self.get_columns_widths(table_area.width, add_selection_space);
        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = " ".repeat(highlight_symbol.width());
        let mut current_height = 0;
        let mut rows_height = table_area.height;

        // Draw header
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
            let mut col = table_area.left();
            if add_selection_space {
                col += (highlight_symbol.width() as u16).min(table_area.width);
            }
            for (width, cell) in columns_widths.iter().zip(header.cells.iter()) {
                render_cell(
                    buf,
                    cell,
                    Rect {
                        x: col,
                        y: table_area.top(),
                        width: *width,
                        height: max_header_height,
                    },
                );
                col += *width + self.column_spacing;
            }
            current_height += max_header_height;
            rows_height = rows_height.saturating_sub(max_header_height);
        }

        // Draw rows
        if self.rows.is_empty() {
            return;
        }
        let (start, end) = self.get_row_bounds(state.selected, state.offset, rows_height);
        state.offset = start;
        for (i, table_row) in self
            .rows
            .iter_mut()
            .enumerate()
            .skip(state.offset)
            .take(end - start)
        {
            let (row, col) = (table_area.top() + current_height, table_area.left());
            current_height += table_row.total_height();
            let table_row_area = Rect {
                x: col,
                y: row,
                width: table_area.width,
                height: table_row.height,
            };
            buf.set_style(table_row_area, table_row.style);
            let is_selected = state.selected.map_or(false, |s| s == i);
            let table_row_start_col = if add_selection_space {
                let symbol = if is_selected {
                    highlight_symbol
                } else {
                    &blank_symbol
                };
                let (col, _) =
                    buf.set_stringn(col, row, symbol, table_area.width as usize, table_row.style);
                col
            } else {
                col
            };
            let mut col = table_row_start_col;
            for (width, cell) in columns_widths.iter().zip(table_row.cells.iter()) {
                render_cell(
                    buf,
                    cell,
                    Rect {
                        x: col,
                        y: row,
                        width: *width,
                        height: table_row.height,
                    },
                );
                col += *width + self.column_spacing;
            }
            if is_selected {
                buf.set_style(table_row_area, self.highlight_style);
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
        buf.set_line(area.x, area.y + i as u16, line, area.width);
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
    use crate::style::{Color, Modifier, Style, Stylize};
    #[test]
    #[should_panic]
    fn table_invalid_percentages() {
        Table::new(vec![]).widths(&[Constraint::Percentage(110)]);
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
}
