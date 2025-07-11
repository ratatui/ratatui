//! The [`Table`] widget is used to display data in rows and columns.

mod cell;
mod highlight_spacing;
mod row;
mod state;

pub use cell::Cell;
pub use highlight_spacing::HighlightSpacing;
pub use row::Row;
pub use state::TableState;

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Constraint, Flex, Layout, Rect};
use ratatui_core::style::{Style, Styled};
use ratatui_core::symbols;
use ratatui_core::text::Text;
use ratatui_core::widgets::{StatefulWidget, Widget};

use crate::block::Block;

#[derive(Clone, Debug, PartialEq)]
pub enum TableBorder {
    None,
    Horizontal,
    Vertical,
    All,
}

#[derive(Clone, Debug)]
pub struct Table<'a> {
    pub rows: Vec<Row<'a>>,
    pub header: Option<Row<'a>>,
    pub footer: Option<Row<'a>>,
    pub widths: Vec<Constraint>,
    pub column_spacing: u16,
    pub block: Option<Block<'a>>,
    pub style: Style,
    pub row_highlight_style: Style,
    pub column_highlight_style: Style,
    pub cell_highlight_style: Style,
    pub highlight_symbol: Text<'a>,
    pub highlight_spacing: HighlightSpacing,
    pub flex: Flex,
    pub border_type: TableBorder,
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
            border_type: TableBorder::None,
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

    pub const fn border_type(mut self, border: TableBorder) -> Self {
        self.border_type = border;
        self
    }

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

        self.render_internal_borders(area, buf, selection_width, columns_widths, start_index, end_index);

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
        use symbols::line;
        match self.border_type {
            TableBorder::None => return,
            TableBorder::Horizontal => {
                self.render_horizontal_borders(area, buf, selection_width, start_index, end_index);
            }
            TableBorder::Vertical => {
                self.render_vertical_borders(area, buf, selection_width, columns_widths);
            }
            TableBorder::All => {
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
                        let cell = buf.get_mut(x, border_y);
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
        selection_width: u16,
        columns_widths: &[(u16, u16)],
    ) {
        use symbols::line;
        for (i, (x, width)) in columns_widths.iter().enumerate() {
            if i < columns_widths.len() - 1 {
                let border_x = if self.column_spacing > 0 {
                    area.x + x + width + self.column_spacing / 2
                } else {
                    area.x + x + width
                };
                if border_x < area.right() {
                    for y in area.y..area.bottom() {
                        let cell = buf.get_mut(border_x, y);
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
        let selection_width = self.selection_width(state);
        let column_widths = self.get_column_widths(table_area.width, selection_width, column_count);
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
        self.render_rows(rows_area, buf, state, selection_width, &column_widths);
        self.render_footer(footer_area, buf, &column_widths);
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
        Self::new(rows, widths)
    }
}