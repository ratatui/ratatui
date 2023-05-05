use crate::{
    buffer::Buffer,
    layout::{Corner, Rect},
    style::Style,
    text::Text,
    widgets::{Block, StatefulWidget, Widget},
};
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Default)]
pub struct ListState {
    offset: usize,
    selected: Option<usize>,
}

impl ListState {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem<'a> {
    content: Text<'a>,
    style: Style,
}

impl<'a> ListItem<'a> {
    pub fn new<T>(content: T) -> ListItem<'a>
    where
        T: Into<Text<'a>>,
    {
        ListItem {
            content: content.into(),
            style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> ListItem<'a> {
        self.style = style;
        self
    }

    pub fn height(&self) -> usize {
        self.content.height()
    }

    pub fn width(&self) -> usize {
        self.content.width()
    }
}

/// A widget to display several items among which one can be selected (optional)
///
/// # Examples
///
/// ```
/// # use ratatui::widgets::{Block, Borders, List, ListItem};
/// # use ratatui::style::{Style, Color, Modifier};
/// let items = [ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
/// List::new(items)
///     .block(Block::default().title("List").borders(Borders::ALL))
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
///     .highlight_symbol(">>");
/// ```
#[derive(Debug, Clone)]
pub struct List<'a> {
    block: Option<Block<'a>>,
    items: Vec<ListItem<'a>>,
    /// Style used as a base style for the widget
    style: Style,
    start_corner: Corner,
    /// Style used to render selected item
    highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'a str>,
    /// Whether to repeat the highlight symbol for each line of the selected item
    repeat_highlight_symbol: bool,
    /// Padding between each item
    padding: usize,
    truncate_last_item: bool,
}

impl<'a> List<'a> {
    pub fn new<T>(items: T) -> List<'a>
    where
        T: Into<Vec<ListItem<'a>>>,
    {
        List {
            block: None,
            style: Style::default(),
            items: items.into(),
            start_corner: Corner::TopLeft,
            highlight_style: Style::default(),
            highlight_symbol: None,
            repeat_highlight_symbol: false,
            padding: 0,
            truncate_last_item: true,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> List<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> List<'a> {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> List<'a> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> List<'a> {
        self.highlight_style = style;
        self
    }

    pub fn repeat_highlight_symbol(mut self, repeat: bool) -> List<'a> {
        self.repeat_highlight_symbol = repeat;
        self
    }

    pub fn start_corner(mut self, corner: Corner) -> List<'a> {
        self.start_corner = corner;
        self
    }

    /// indicates the amount of padding between each item in the list
    pub fn padding(mut self, padding: usize) -> List<'a> {
        self.padding = padding;
        self
    }

    /// indicates that the last item should be truncated if it is too long
    /// This defaults to true as this is the historical behaviour of the
    /// list widget.
    pub fn truncate_last_item(mut self, truncate: bool) -> List<'a> {
        self.truncate_last_item = truncate;
        self
    }

    /// Returns the bounds of the items that will be rendered
    ///
    /// Ensures that the selected item is always visible, and that the items
    /// are not rendered outside the available space.
    fn get_items_bounds(
        &self,
        selected: Option<usize>,
        offset: usize,
        max_height: usize,
    ) -> (usize, usize) {
        let offset = offset.min(self.items.len().saturating_sub(1));
        let mut start = offset;
        let mut end = offset;
        let mut height = 0;
        // calculate which items will be showing starting from offset and
        // filling only the items that fit in the available space
        for item in self.items.iter().skip(offset) {
            // don't include the padding in the max height calculation
            // as we don't need to render the last item's padding
            if height + item.height() > max_height {
                break;
            }
            height += item.height() + self.padding;
            end += 1;
        }

        // if the selected item is after the range of items that will be
        // showing, then we need to adjust the start and end bounds to
        // reflect this. We add one item at the end, and remove 0 or more
        // items from the start to ensure that the height is less than or
        // equal to the max height.
        let selected = selected.unwrap_or(0).min(self.items.len() - 1);
        while selected >= end {
            height = height.saturating_add(self.items[end].height() + self.padding);
            end += 1;
            while height > max_height {
                height = height.saturating_sub(self.items[start].height() + self.padding);
                start += 1;
            }
        }

        // here we do the same thing as above, but in the opposite direction
        // to ensure that the start and end bounds are correct. We remove one
        // item from the start, and add 0 or more items to the end to ensure
        // that the height is less than or equal to the max height.
        while selected < start {
            start -= 1;
            height = height.saturating_add(self.items[start].height() + self.padding);
            while height > max_height {
                end -= 1;
                height = height.saturating_sub(self.items[end].height() + self.padding);
            }
        }

        if !self.truncate_last_item && height < max_height {
            // if the height is less than the max height, then we need to
            // adjust the end bound to include the last item.
            end += 1;
        }

        (start, end)
    }
}

impl<'a> StatefulWidget for List<'a> {
    type State = ListState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        let list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        if self.items.is_empty() {
            return;
        }
        let list_height = list_area.height as usize;

        let (start, end) = self.get_items_bounds(state.selected, state.offset, list_height);
        state.offset = start;

        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = " ".repeat(highlight_symbol.width());

        let mut current_height = 0;
        let has_selection = state.selected.is_some();
        for (i, item) in self
            .items
            .iter_mut()
            .enumerate()
            .skip(state.offset)
            .take(end - start)
        {
            let (x, y) = match self.start_corner {
                Corner::BottomLeft => {
                    current_height += item.height() as u16 + self.padding as u16;
                    (
                        list_area.left(),
                        list_area.bottom().saturating_sub(current_height),
                    )
                }
                _ => {
                    let pos = (list_area.left(), list_area.top() + current_height);
                    current_height += item.height() as u16 + self.padding as u16;
                    pos
                }
            };
            let height = if current_height < list_height as u16 {
                // the item fits in the available height so we can render it in full
                item.height() as u16
            } else {
                // the last item in the list overflows the available height
                // so we need to truncate it to fit in the space available
                list_height as u16 - (current_height - item.height() as u16 - self.padding as u16)
            };
            let area = Rect {
                x,
                y,
                width: list_area.width,
                height,
            };
            let item_style = self.style.patch(item.style);
            buf.set_style(area, item_style);

            let is_selected = state.selected.map(|s| s == i).unwrap_or(false);
            for (line_index, line) in item.content.lines.iter().enumerate() {
                // the number of lines to truncate from the item
                let truncated_height = item.height() - area.height as usize;
                if self.start_corner == Corner::BottomLeft {
                    if line_index < truncated_height {
                        // don't render the first part of a widget when it is truncated
                        continue;
                    }
                } else if line_index >= area.height as usize {
                    // don't render the last part of a widget when it is truncated
                    break;
                }

                // if the item is selected, we need to display the highlight symbol:
                // - either for the first line of the item only,
                // - or for each line of the item if the appropriate option is set
                let symbol = if is_selected && (line_index == 0 || self.repeat_highlight_symbol) {
                    highlight_symbol
                } else {
                    &blank_symbol
                };

                let y = if self.start_corner == Corner::BottomLeft && truncated_height > 0 {
                    // truncate the first item, by shifting everything up by the difference
                    y + line_index as u16 - truncated_height as u16
                } else {
                    y + line_index as u16
                };

                let (elem_x, max_element_width) = if has_selection {
                    let (elem_x, _) =
                        buf.set_stringn(x, y, symbol, list_area.width as usize, item_style);
                    (elem_x, (list_area.width - (elem_x - x)))
                } else {
                    (x, list_area.width)
                };
                buf.set_spans(elem_x, y, line, max_element_width);
            }
            if is_selected {
                buf.set_style(area, self.highlight_style);
            }
        }
    }
}

impl<'a> Widget for List<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ListState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}
