//! The [`Table`] widget is used to display data in rows and columns.

mod cell;
mod highlight_spacing;
mod row;
mod state;

use std::vec;
use std::vec::Vec;

pub use cell::Cell;
pub use highlight_spacing::HighlightSpacing;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Constraint, Flex, Layout, Rect};
use ratatui_core::style::{Style, Styled};
use ratatui_core::symbols;
use ratatui_core::text::Text;
use ratatui_core::widgets::{StatefulWidget, Widget};
pub use row::Row;
pub use state::TableState;

use crate::block::Block;

/// The type of borders for a table.
///
/// This enum defines the different border styles that can be applied to a table.
/// It allows for controlling which borders are displayed in the table.
///
/// # Examples
///
/// ```rust
/// use ratatui_core::style::{Color, Style};
/// use ratatui_widgets::table::{Table, TableBorderType};
///
/// let table = Table::new(vec![], vec![])
///     .border_type(TableBorderType::All)
///     .border_style(Style::default().fg(Color::Blue));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TableBorderType {
    /// No borders displayed.
    None,
    /// Only horizontal borders displayed.
    Horizontal,
    /// Only vertical borders displayed.
    Vertical,
    /// All borders displayed.
    All,
}

/// A table widget that displays data in rows and columns.
#[derive(Clone, Debug)]
pub struct Table<'a> {
    /// The rows of the table.
    pub rows: Vec<Row<'a>>,
    /// The optional header row.
    pub header: Option<Row<'a>>,
    /// The optional footer row.
    pub footer: Option<Row<'a>>,
    /// The constraints for column widths.
    pub widths: Vec<Constraint>,
    /// The spacing between columns.
    pub column_spacing: u16,
    /// The optional block to wrap the table.
    pub block: Option<Block<'a>>,
    /// The style for the table.
    pub style: Style,
    /// The style for highlighted rows.
    pub row_highlight_style: Style,
    /// The style for highlighted columns.
    pub column_highlight_style: Style,
    /// The style for highlighted cells.
    pub cell_highlight_style: Style,
    /// The symbol to use for highlighting.
    pub highlight_symbol: Text<'a>,
    /// The spacing behavior for highlighting.
    pub highlight_spacing: HighlightSpacing,
    /// The flex behavior for column alignment.
    pub flex: Flex,
    /// The type of borders to display.
    pub border_type: TableBorderType,
    /// The style for borders.
    pub border_style: Style,
}

impl<'a> Default for Table<'a> {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            header: None,
            footer: None,
            widths: Vec::new(),
            column_spacing: 1,
            block: None,
            style: Style::default(),
            row_highlight_style: Style::default(),
            column_highlight_style: Style::default(),
            cell_highlight_style: Style::default(),
            highlight_symbol: Text::default(),
            highlight_spacing: HighlightSpacing::WhenSelected,
            flex: Flex::Start,
            border_type: TableBorderType::None,
            border_style: Style::default(),
        }
    }
}

impl<'a> Table<'a> {
    pub fn new<R, I>(rows: R, widths: I) -> Self
    where
        R: IntoIterator<Item = Row<'a>>,
        I: IntoIterator<Item = Constraint>,
    {
        let widths: Vec<Constraint> = widths.into_iter().collect();
        ensure_percentages_less_than_100(&widths);
        Self {
            rows: rows.into_iter().collect(),
            widths,
            ..Default::default()
        }
    }

    pub fn rows<T>(mut self, rows: T) -> Self
    where
        T: IntoIterator<Item = Row<'a>>,
    {
        self.rows = rows.into_iter().collect();
        self
    }

    pub fn header(mut self, header: Row<'a>) -> Self {
        self.header = Some(header);
        self
    }

    pub fn footer(mut self, footer: Row<'a>) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn widths<I>(mut self, widths: I) -> Self
    where
        I: IntoIterator<Item = Constraint>,
    {
        let widths: Vec<Constraint> = widths.into_iter().collect();
        ensure_percentages_less_than_100(&widths);
        self.widths = widths;
        self
    }

    pub const fn column_spacing(mut self, spacing: u16) -> Self {
        self.column_spacing = spacing;
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    pub fn row_highlight_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.row_highlight_style = style.into();
        self
    }

    pub fn column_highlight_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.column_highlight_style = style.into();
        self
    }

    pub fn cell_highlight_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.cell_highlight_style = style.into();
        self
    }

    pub fn highlight_symbol<T: Into<Text<'a>>>(mut self, symbol: T) -> Self {
        self.highlight_symbol = symbol.into();
        self
    }

    pub const fn highlight_spacing(mut self, spacing: HighlightSpacing) -> Self {
        self.highlight_spacing = spacing;
        self
    }

    pub const fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    /// Sets the border type for the table.
    ///
    /// This determines which borders are displayed in the table.
    /// The border style can be customized with [`border_style`].
    ///
    /// [`border_style`]: Self::border_style
    pub const fn border_type(mut self, border: TableBorderType) -> Self {
        self.border_type = border;
        self
    }

    /// Sets the border style for the table.
    ///
    /// This determines the styling (color, modifiers) of the table borders.
    /// The border type can be set with [`border_type`].
    ///
    /// [`border_type`]: Self::border_type
    pub fn border_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.border_style = style.into();
        self
    }

    /// Return the indexes of the visible rows.
    fn visible_rows(&self, state: &TableState, area: Rect) -> (usize, usize) {
        let last_row = self.rows.len().saturating_sub(1);
        let mut start = state.offset().min(last_row);
        if let Some(selected) = state.selected() {
            start = start.min(selected);
        }

        let mut end = start;
        let mut height = 0;

        for item in self.rows.iter().skip(start) {
            if height + item.height > area.height {
                break;
            }
            height += item.height_with_margin();
            end += 1;
        }

        if let Some(selected) = state.selected() {
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

    fn get_column_widths(
        &self,
        max_width: u16,
        selection_width: u16,
        col_count: usize,
    ) -> Vec<(u16, u16)> {
        let widths = if self.widths.is_empty() {
            vec![Constraint::Length(max_width / col_count.max(1) as u16); col_count]
        } else {
            self.widths.clone()
        };
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

    fn selection_width(&self, state: &TableState) -> u16 {
        let has_selection = state.selected().is_some();
        if self.highlight_spacing.should_add(has_selection) {
            self.highlight_symbol.width() as u16
        } else {
            0
        }
    }

    fn max_selection_width(&self) -> u16 {
        match self.highlight_spacing {
            HighlightSpacing::Always | HighlightSpacing::WhenSelected => {
                self.highlight_symbol.width() as u16
            }
            HighlightSpacing::Never => 0,
        }
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
        *state.offset_mut() = start_index;

        let mut y_offset = 0;

        let mut selected_row_area = None;
        for (i, row) in self
            .rows
            .iter()
            .enumerate()
            .skip(start_index)
            .take(end_index - start_index)
        {
            let y = area.y + y_offset + row.top_margin;
            let height = (y + row.height).min(area.bottom()).saturating_sub(y);
            let row_area = Rect { y, height, ..area };
            buf.set_style(row_area, row.style);

            let is_selected = state.selected().is_some_and(|index| index == i);
            if selection_width > 0 && is_selected {
                let selection_area = Rect {
                    width: selection_width,
                    ..row_area
                };
                buf.set_style(selection_area, row.style);
                Widget::render(&self.highlight_symbol, selection_area, buf);
            }
            for ((x, width), cell) in columns_widths.iter().zip(row.cells.iter()) {
                cell.render(
                    Rect::new(row_area.x + x, row_area.y, *width, row_area.height),
                    buf,
                );
            }
            if is_selected {
                selected_row_area = Some(row_area);
            }
            y_offset += row.height_with_margin();
        }

        self.render_internal_borders(
            area,
            buf,
            selection_width,
            columns_widths,
            start_index,
            end_index,
        );

        let selected_column_area = state.selected_column().and_then(|s| {
            columns_widths.get(s).map(|(x, width)| Rect {
                x: x + area.x,
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
        match self.border_type {
            TableBorderType::None => return,
            TableBorderType::Horizontal => {
                self.render_horizontal_borders(area, buf, selection_width, start_index, end_index);
            }
            TableBorderType::Vertical => {
                self.render_vertical_borders(area, buf, selection_width, columns_widths);
            }
            TableBorderType::All => {
                self.render_horizontal_borders(area, buf, selection_width, start_index, end_index);
                self.render_vertical_borders(area, buf, selection_width, columns_widths);
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
        use symbols::line;
        let mut y_offset = 0;
        for (i, row) in self
            .rows
            .iter()
            .enumerate()
            .skip(start_index)
            .take(end_index - start_index)
        {
            y_offset += row.top_margin + row.height;
            if i < end_index - 1 && y_offset < area.height {
                let border_y = area.y + y_offset;
                if border_y < area.bottom() {
                    for x in (area.x + selection_width)..area.right() {
                        let cell = &mut buf[(x, border_y)];
                        let symbol = if cell.symbol() == line::NORMAL.vertical {
                            line::NORMAL.cross
                        } else {
                            line::NORMAL.horizontal
                        };
                        cell.set_symbol(symbol).set_style(self.border_style);
                    }
                }
            }
            y_offset += row.bottom_margin;
        }
    }

    fn render_vertical_borders(
        &self,
        area: Rect,
        buf: &mut Buffer,
        _selection_width: u16,
        columns_widths: &[(u16, u16)],
    ) {
        for (i, (x, width)) in columns_widths.iter().enumerate() {
            if i < columns_widths.len() - 1 {
                let border_x = if self.column_spacing > 0 {
                    area.x + x + width + self.column_spacing / 2
                } else {
                    area.x + x + width
                };
                if border_x < area.right() {
                    for y in area.y..area.bottom() {
                        let cell = &mut buf[(border_x, y)];
                        let symbol = if cell.symbol() == symbols::line::NORMAL.horizontal {
                            symbols::line::NORMAL.cross
                        } else {
                            symbols::line::NORMAL.vertical
                        };
                        cell.set_symbol(symbol).set_style(self.border_style);
                    }
                }
            }
        }
    }
}

impl<'a> Widget for Table<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = TableState::default();
        StatefulWidget::render(&self, area, buf, &mut state);
    }
}

impl<'a> Widget for &Table<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = TableState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl<'a> StatefulWidget for Table<'a> {
    type State = TableState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidget::render(&self, area, buf, state);
    }
}

impl<'a> StatefulWidget for &Table<'a> {
    type State = TableState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        if let Some(ref block) = self.block {
            Widget::render(block, area, buf);
        }
        let table_area = if let Some(ref block) = self.block {
            block.inner(area)
        } else {
            area
        };
        if table_area.is_empty() {
            return;
        }
        if state.selected().is_some_and(|s| s >= self.rows.len()) {
            state.select(Some(self.rows.len().saturating_sub(1)));
        }
        if self.rows.is_empty() {
            state.select(None);
        }
        let column_count = self.column_count();
        if state.selected_column().is_some_and(|s| s >= column_count) {
            state.select_column(Some(column_count.saturating_sub(1)));
        }
        if column_count == 0 {
            state.select_column(None);
        }
        let max_selection_width = self.max_selection_width();
        let column_widths =
            self.get_column_widths(table_area.width, max_selection_width, column_count);
        let selection_width = self.selection_width(state);
        let (header_area, rows_area, footer_area) = {
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
            .split(table_area);
            (layout[1], layout[3], layout[5])
        };
        self.render_header(header_area, buf, &column_widths);
        let (start_index, end_index) = self.visible_rows(state, rows_area);
        self.render_rows(rows_area, buf, state, selection_width, &column_widths);
        self.render_footer(footer_area, buf, &column_widths);
        self.render_internal_borders(
            rows_area,
            buf,
            selection_width,
            &column_widths,
            start_index,
            end_index,
        );
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

impl<'a, Item> ::core::iter::FromIterator<Item> for Table<'a>
where
    Item: Into<Row<'a>>,
{
    fn from_iter<Iter: IntoIterator<Item = Item>>(rows: Iter) -> Self {
        let widths: [Constraint; 0] = [];
        let converted_rows: Vec<Row<'a>> = rows.into_iter().map(|item| item.into()).collect();
        Self::new(converted_rows, widths)
    }
}

#[cfg(test)]
mod tests {
    use ratatui_core::buffer::Buffer;
    use ratatui_core::layout::Constraint::*;
    use ratatui_core::layout::{Alignment, Flex, Rect};
    use ratatui_core::style::{Color, Modifier, Style};
    use ratatui_core::text::{Line, Text};
    use ratatui_core::widgets::{StatefulWidget, Widget};
    use rstest::rstest;

    use super::*;
    use crate::block::Block;

    mod table {
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
                .rows(vec![])
                .header(Row::new(vec!["f", "g"]))
                .column_spacing(0);
            assert_eq!(table.get_column_widths(10, 0, 2), [(0, 5), (5, 5)]);
        }

        #[test]
        fn no_constraint_with_footer() {
            let table = Table::default()
                .rows(vec![])
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
}
