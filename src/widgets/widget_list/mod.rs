use crate::{
    buffer::Buffer,
    layout::{Constraint, Corner, Rect},
    style::Style,
    widgets::{Block, StatefulWidget, Widget},
};

use self::layout::Layout;

use super::SizeHint;

mod layout;
pub mod widget_list_item;

#[derive(Debug, Clone, Default)]
pub struct WidgetListState {
    offset: usize,
    selected: Option<usize>,
}

impl WidgetListState {
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

/// A widget to display several items among which one can be selected (optional)
///
/// # Examples
///
/// ```
/// # use ratatui::widgets::{Block, Borders, WidgetList, Paragraph};
/// # use ratatui::style::{Style, Color};
/// let items = [
///     Paragraph::new("Item 1\ndescription"),
///     Paragraph::new("Item 2\ndescription"),
///     Paragraph::new("Item 3\ndescription"),
/// ];
/// WidgetList::new(items)
///     .block(Block::default().title("List").borders(Borders::ALL))
///     .style(Style::default().fg(Color::White));
/// ```
#[derive(Debug, Clone)]
pub struct WidgetList<'a, E: Widget + SizeHint> {
    block: Option<Block<'a>>,
    items: Vec<E>,
    style: Style,
    start_corner: Corner,
    /// Style used to render selected item
    highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'a str>,
    /// Whether to repeat the highlight symbol for each line of the selected item
    repeat_highlight_symbol: bool,
    /// Spacing between each item
    spacing: u16,
    item_heights: Vec<Option<Constraint>>,
}

impl<'a, E> WidgetList<'a, E>
where
    E: Widget + SizeHint,
{
    pub fn new<T>(items: T) -> WidgetList<'a, E>
    where
        T: Into<Vec<E>>,
        E: Widget + SizeHint,
    {
        WidgetList {
            block: None,
            style: Style::default(),
            items: items.into(),
            start_corner: Corner::TopLeft,
            spacing: 0,
            item_heights: vec![],
            highlight_symbol: None,
            highlight_style: Style::default(),
            repeat_highlight_symbol: false,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> WidgetList<'a, E> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> WidgetList<'a, E> {
        self.style = style;
        self
    }

    /// indicate an individual constraint to list items.
    /// if the given vector is smaller than the item count, then the missing constraint will be set to None
    pub fn item_heights(mut self, item_heights: Vec<Option<Constraint>>) -> WidgetList<'a, E> {
        self.item_heights = item_heights;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> WidgetList<'a, E> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> WidgetList<'a, E> {
        self.highlight_style = style;
        self
    }

    pub fn repeat_highlight_symbol(mut self, repeat: bool) -> WidgetList<'a, E> {
        self.repeat_highlight_symbol = repeat;
        self
    }

    pub fn start_corner(mut self, corner: Corner) -> WidgetList<'a, E> {
        self.start_corner = corner;
        self
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// indicates the amount of spacing between each item in the list
    pub fn spacing(mut self, spacing: u16) -> WidgetList<'a, E> {
        self.spacing = spacing;
        self
    }
}

impl<'a, E> StatefulWidget for WidgetList<'a, E>
where
    E: Widget + SizeHint,
{
    type State = WidgetListState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        let mut list_area = match self.block.take() {
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

        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let symbol_x = list_area.x;

        if state.selected.is_some() && !highlight_symbol.is_empty() {
            list_area.width = list_area
                .width
                .saturating_sub(highlight_symbol.len() as u16);
            list_area.x += highlight_symbol.len() as u16;
        }

        let layout = Layout::new(
            &list_area,
            self.spacing,
            &self.items,
            &state.selected,
            state.offset,
            &self.item_heights,
        );

        state.offset = layout.offset;

        for (i, (item, mut area)) in self
            .items
            .into_iter()
            .skip(state.offset)
            .zip(layout.item_areas)
            .enumerate()
        {
            if let Corner::BottomLeft = self.start_corner {
                area.y = list_area.bottom() - area.bottom() + list_area.y;
            }

            let is_selected = state
                .selected
                .map(|s| s == i + state.offset)
                .unwrap_or(false);

            if is_selected && !highlight_symbol.is_empty() {
                let selected_y = area.y + ((area.height - 1) >> 1);
                let blank_symbol = " ".repeat(highlight_symbol.len());
                for y in area.y..(area.y + area.height) {
                    buf.set_stringn(
                        symbol_x,
                        y,
                        if self.repeat_highlight_symbol || selected_y == y {
                            highlight_symbol
                        } else {
                            &blank_symbol
                        },
                        highlight_symbol.len(),
                        self.highlight_style,
                    );
                }
            }

            item.render(area, buf);

            if is_selected {
                buf.set_style(area, self.highlight_style);
            }
        }
    }
}

impl<'a, E> Widget for WidgetList<'a, E>
where
    E: Widget + SizeHint,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = WidgetListState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}
