use std::iter::{self, Iterator};

use unicode_width::UnicodeWidthStr;

use crate::buffer::Buffer;
use crate::layout::{Corner, Rect};
use crate::style::Style;
use crate::widgets::{Block, StatefulWidget, Text, Widget};

pub struct ListState {
    offset: usize,
    selected: Option<usize>,
}

impl Default for ListState {
    fn default() -> ListState {
        ListState {
            offset: 0,
            selected: None,
        }
    }
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

/// A widget to display several items among which one can be selected (optional)
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders, List, Text};
/// # use tui::style::{Style, Color, Modifier};
/// let items = ["Item 1", "Item 2", "Item 3"].iter().map(|i| Text::raw(*i));
/// List::new(items)
///     .block(Block::default().title("List").borders(Borders::ALL))
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().modifier(Modifier::ITALIC))
///     .highlight_symbol(">>");
/// ```
pub struct List<'b, L>
where
    L: Iterator<Item = Text<'b>>,
{
    block: Option<Block<'b>>,
    items: L,
    start_corner: Corner,
    /// Base style of the widget
    style: Style,
    /// Style used to render selected item
    highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'b str>,
}

impl<'b, L> Default for List<'b, L>
where
    L: Iterator<Item = Text<'b>> + Default,
{
    fn default() -> List<'b, L> {
        List {
            block: None,
            items: L::default(),
            style: Default::default(),
            start_corner: Corner::TopLeft,
            highlight_style: Style::default(),
            highlight_symbol: None,
        }
    }
}

impl<'b, L> List<'b, L>
where
    L: Iterator<Item = Text<'b>>,
{
    pub fn new(items: L) -> List<'b, L> {
        List {
            block: None,
            items,
            style: Default::default(),
            start_corner: Corner::TopLeft,
            highlight_style: Style::default(),
            highlight_symbol: None,
        }
    }

    pub fn block(mut self, block: Block<'b>) -> List<'b, L> {
        self.block = Some(block);
        self
    }

    pub fn items<I>(mut self, items: I) -> List<'b, L>
    where
        I: IntoIterator<Item = Text<'b>, IntoIter = L>,
    {
        self.items = items.into_iter();
        self
    }

    pub fn style(mut self, style: Style) -> List<'b, L> {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'b str) -> List<'b, L> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, highlight_style: Style) -> List<'b, L> {
        self.highlight_style = highlight_style;
        self
    }

    pub fn start_corner(mut self, corner: Corner) -> List<'b, L> {
        self.start_corner = corner;
        self
    }
}

impl<'b, L> StatefulWidget for List<'b, L>
where
    L: Iterator<Item = Text<'b>>,
{
    type State = ListState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let list_area = match self.block {
            Some(ref mut b) => {
                b.render(area, buf);
                b.inner(area)
            }
            None => area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        let list_height = list_area.height as usize;

        buf.set_background(list_area, self.style.bg);

        // Use highlight_style only if something is selected
        let (selected, highlight_style) = match state.selected {
            Some(i) => (Some(i), self.highlight_style),
            None => (None, self.style),
        };
        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = iter::repeat(" ")
            .take(highlight_symbol.width())
            .collect::<String>();

        // Make sure the list show the selected item
        state.offset = if let Some(selected) = selected {
            if selected >= list_height + state.offset - 1 {
                selected + 1 - list_height
            } else if selected < state.offset {
                selected
            } else {
                state.offset
            }
        } else {
            0
        };

        for (i, item) in self
            .items
            .skip(state.offset)
            .enumerate()
            .take(list_area.height as usize)
        {
            let (x, y) = match self.start_corner {
                Corner::TopLeft => (list_area.left(), list_area.top() + i as u16),
                Corner::BottomLeft => (list_area.left(), list_area.bottom() - (i + 1) as u16),
                // Not supported
                _ => (list_area.left(), list_area.top() + i as u16),
            };
            let (x, style) = if let Some(s) = selected {
                if s == i + state.offset {
                    let (x, _) = buf.set_stringn(
                        x,
                        y,
                        highlight_symbol,
                        list_area.width as usize,
                        highlight_style,
                    );
                    (x + 1, Some(highlight_style))
                } else {
                    let (x, _) = buf.set_stringn(
                        x,
                        y,
                        &blank_symbol,
                        list_area.width as usize,
                        highlight_style,
                    );
                    (x + 1, None)
                }
            } else {
                (x, None)
            };
            match item {
                Text::Raw(ref v) => {
                    buf.set_stringn(
                        x,
                        y,
                        v,
                        list_area.width as usize,
                        style.unwrap_or(self.style),
                    );
                }
                Text::Styled(ref v, s) => {
                    buf.set_stringn(x, y, v, list_area.width as usize, style.unwrap_or(s));
                }
            };
        }
    }
}

impl<'b, L> Widget for List<'b, L>
where
    L: Iterator<Item = Text<'b>>,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ListState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}
