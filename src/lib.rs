#![allow(clippy::must_use_candidate)]

use std::collections::HashSet;
use tui::buffer::Buffer;
use tui::layout::{Corner, Rect};
use tui::style::Style;
use tui::text::Text;
use tui::widgets::{Block, StatefulWidget, Widget};
use unicode_width::UnicodeWidthStr;

mod flatten;
mod identifier;

pub use flatten::{flatten, Flattened};
pub use identifier::{
    get_without_leaf as get_identifier_without_leaf, TreeIdentifier, TreeIdentifierVec,
};

#[derive(Debug, Default, Clone)]
pub struct TreeState {
    offset: usize,
    selected: TreeIdentifierVec,
    opened: HashSet<TreeIdentifierVec>,
}

impl TreeState {
    pub fn selected(&self) -> Vec<usize> {
        self.selected.clone()
    }

    pub fn select<I>(&mut self, identifier: I)
    where
        I: Into<Vec<usize>>,
    {
        self.selected = identifier.into();

        // TODO: ListState does this. Is this relevant?
        if self.selected.is_empty() {
            self.offset = 0;
        }
    }

    /// Open a tree node.
    /// Returns `true` if the node was closed and has been opened.
    /// Returns `false` if the node was already open.
    pub fn open(&mut self, identifier: TreeIdentifierVec) -> bool {
        if identifier.is_empty() {
            false
        } else {
            self.opened.insert(identifier)
        }
    }

    /// Close a tree node.
    /// Returns `true` if the node was open and has been closed.
    /// Returns `false` if the node was already closed.
    pub fn close(&mut self, identifier: TreeIdentifier) -> bool {
        self.opened.remove(identifier)
    }

    /// Toggles a tree node.
    /// If the node is in opened then it calls `close()`. Otherwise it calls `open()`.
    pub fn toggle(&mut self) {
        if self.opened.contains(&self.selected()) {
            self.close(&self.selected());
        } else {
            self.open(self.selected());
        }
    }

    pub fn close_all(&mut self) {
        self.opened.clear();
    }

    pub fn get_all_opened(&self) -> Vec<TreeIdentifierVec> {
        self.opened.iter().cloned().collect()
    }

    pub const fn get_offset(&self) -> usize {
        self.offset
    }
}

#[derive(Debug, Clone)]
pub struct TreeItem<'a> {
    text: Text<'a>,
    style: Style,
    children: Vec<TreeItem<'a>>,
}

impl<'a> TreeItem<'a> {
    pub fn new_leaf<T>(text: T) -> Self
    where
        T: Into<Text<'a>>,
    {
        Self {
            text: text.into(),
            style: Style::default(),
            children: Vec::new(),
        }
    }

    pub fn new<T, Children>(text: T, children: Children) -> Self
    where
        T: Into<Text<'a>>,
        Children: Into<Vec<TreeItem<'a>>>,
    {
        Self {
            text: text.into(),
            style: Style::default(),
            children: children.into(),
        }
    }

    pub fn child(&self, index: usize) -> Option<&Self> {
        self.children.get(index)
    }

    pub fn child_mut(&mut self, index: usize) -> Option<&mut Self> {
        self.children.get_mut(index)
    }

    pub fn height(&self) -> usize {
        self.text.height()
    }

    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn add_child(&mut self, child: TreeItem<'a>) {
        self.children.push(child);
    }
}

#[derive(Debug, Clone)]
pub struct Tree<'a> {
    block: Option<Block<'a>>,
    items: Vec<TreeItem<'a>>,
    /// Style used as a base style for the widget
    style: Style,
    start_corner: Corner,
    /// Style used to render selected item
    highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'a str>,
}

impl<'a> Tree<'a> {
    pub fn new<T>(items: T) -> Self
    where
        T: Into<Vec<TreeItem<'a>>>,
    {
        Self {
            block: None,
            style: Style::default(),
            items: items.into(),
            start_corner: Corner::TopLeft,
            highlight_style: Style::default(),
            highlight_symbol: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub const fn highlight_symbol(mut self, highlight_symbol: &'a str) -> Self {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub const fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    pub const fn start_corner(mut self, corner: Corner) -> Self {
        self.start_corner = corner;
        self
    }
}

impl<'a> StatefulWidget for Tree<'a> {
    type State = TreeState;

    #[allow(clippy::too_many_lines)]
    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        let area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if area.width < 1 || area.height < 1 {
            return;
        }

        let visible = flatten(&state.get_all_opened(), &self.items);
        if visible.is_empty() {
            return;
        }
        let available_height = area.height as usize;

        let selected_index = if state.selected.is_empty() {
            0
        } else {
            visible
                .iter()
                .position(|o| o.identifier == state.selected)
                .unwrap_or(0)
        };

        let mut start = state.offset.min(selected_index);
        let mut end = start;
        let mut height = 0;
        for item in visible.iter().skip(start) {
            if height + item.item.height() > available_height {
                break;
            }

            height += item.item.height();
            end += 1;
        }

        while selected_index >= end {
            height = height.saturating_add(visible[end].item.height());
            end += 1;
            while height > available_height {
                height = height.saturating_sub(visible[start].item.height());
                start += 1;
            }
        }

        state.offset = start;

        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = " ".repeat(highlight_symbol.width());

        let mut current_height = 0;
        let has_selection = !state.selected.is_empty();
        #[allow(clippy::cast_possible_truncation)]
        for item in visible.iter().skip(state.offset).take(end - start) {
            #[allow(clippy::single_match_else)] // Keep same as List impl
            let (x, y) = match self.start_corner {
                Corner::BottomLeft => {
                    current_height += item.item.height() as u16;
                    (area.left(), area.bottom() - current_height)
                }
                _ => {
                    let pos = (area.left(), area.top() + current_height);
                    current_height += item.item.height() as u16;
                    pos
                }
            };
            let area = Rect {
                x,
                y,
                width: area.width,
                height: item.item.height() as u16,
            };

            let item_style = self.style.patch(item.item.style);
            buf.set_style(area, item_style);

            let is_selected = state.selected == item.identifier;
            let after_highlight_symbol_x = if has_selection {
                let symbol = if is_selected {
                    highlight_symbol
                } else {
                    &blank_symbol
                };
                let (x, _) = buf.set_stringn(x, y, symbol, area.width as usize, item_style);
                x
            } else {
                x
            };

            let after_depth_x = {
                let symbol = if item.item.children.is_empty() {
                    " "
                } else if state.opened.contains(&item.identifier) {
                    "\u{25bc}" // Arrow down
                } else {
                    "\u{25b6}" // Arrow to right
                };
                let string = format!("{:>width$}{} ", "", symbol, width = item.depth() * 2);
                let max_width = area.width.saturating_sub(after_highlight_symbol_x - x);
                let (x, _) = buf.set_stringn(
                    after_highlight_symbol_x,
                    y,
                    string,
                    max_width as usize,
                    item_style,
                );
                x
            };

            let max_element_width = area.width.saturating_sub(after_depth_x - x);
            for (j, line) in item.item.text.lines.iter().enumerate() {
                buf.set_spans(after_depth_x, y + j as u16, line, max_element_width);
            }
            if is_selected {
                buf.set_style(area, self.highlight_style);
            }
        }
    }
}

impl<'a> Widget for Tree<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = TreeState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}
