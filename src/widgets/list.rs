use std::rc::Rc;

use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Buffer,
    layout::{Corner, Rect},
    style::{Style, Styled},
    text::Text,
    widgets::{Block, StatefulWidget, Widget},
};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ListState {
    offset: usize,
    selected: Option<usize>,
}

impl ListState {
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

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct CommonList<'a> {
    block: Option<Block<'a>>,
    /// Style used as a base style for the widget
    style: Style,
    start_corner: Corner,
    /// Style used to render selected item
    highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'a str>,
    /// Whether to repeat the highlight symbol for each line of the selected item
    repeat_highlight_symbol: bool,
}

fn get_items_bounds<F>(
    mut get_height: F,
    items_len: usize,
    selected: Option<usize>,
    offset: usize,
    max_height: usize,
) -> (usize, usize)
where
    F: FnMut(usize) -> usize,
{
    let offset = offset.min(items_len.saturating_sub(1));
    let mut start = offset;
    let mut end = offset;
    let mut height = 0;

    for item_id in (0..items_len).skip(offset) {
        let item_height = get_height(item_id);
        if height + item_height > max_height {
            break;
        }
        height += item_height;
        end += 1;
    }

    let selected = selected.unwrap_or(0).min(items_len - 1);
    while selected >= end {
        let item_height = get_height(end);
        height = height.saturating_add(item_height);
        end += 1;
        while height > max_height {
            let item_height = get_height(start);
            height = height.saturating_sub(item_height);
            start += 1;
        }
    }
    while selected < start {
        start -= 1;
        let item_height = get_height(start);
        height = height.saturating_add(item_height);
        while height > max_height {
            end -= 1;
            let item_height = get_height(end);
            height = height.saturating_sub(item_height);
        }
    }
    (start, end)
}

fn render_item<'a>(
    common: &mut CommonList<'a>,
    state: &mut ListState,
    list_area: &Rect,
    buf: &mut Buffer,
    (id, item): (usize, &ListItem<'a>),
    current_height: &mut u16,
    blank_symbol: &str,
) {
    let highlight_symbol = common.highlight_symbol.unwrap_or("");
    let has_selection = state.selected.is_some();

    let (x, y) = if common.start_corner == Corner::BottomLeft {
        *current_height += item.height() as u16;
        (list_area.left(), list_area.bottom() - *current_height)
    } else {
        let pos = (list_area.left(), list_area.top() + *current_height);
        *current_height += item.height() as u16;
        pos
    };
    let area = Rect {
        x,
        y,
        width: list_area.width,
        height: item.height() as u16,
    };
    let item_style = common.style.patch(item.style);
    buf.set_style(area, item_style);

    let is_selected = state.selected.map_or(false, |s| s == id);
    for (j, line) in item.content.lines.iter().enumerate() {
        // if the item is selected, we need to display the highlight symbol:
        // - either for the first line of the item only,
        // - or for each line of the item if the appropriate option is set
        let symbol = if is_selected && (j == 0 || common.repeat_highlight_symbol) {
            highlight_symbol
        } else {
            blank_symbol
        };
        let (elem_x, max_element_width) = if has_selection {
            let (elem_x, _) = buf.set_stringn(
                x,
                y + j as u16,
                symbol,
                list_area.width as usize,
                item_style,
            );
            (elem_x, (list_area.width - (elem_x - x)))
        } else {
            (x, list_area.width)
        };
        buf.set_line(elem_x, y + j as u16, line, max_element_width);
    }

    if is_selected {
        buf.set_style(area, common.highlight_style);
    }
}

#[derive(Clone)]
struct LazyVec<'a> {
    builder: Rc<dyn Fn(usize) -> ListItem<'a> + 'a>,
    items: Vec<Option<ListItem<'a>>>,
}

impl<'a> LazyVec<'a> {
    fn new<F>(len: usize, builder: F) -> Self
    where
        F: Fn(usize) -> ListItem<'a> + 'a,
    {
        Self {
            builder: Rc::new(builder),
            items: vec![None; len],
        }
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn iter_ids(&self) -> std::ops::Range<usize> {
        0..self.len()
    }

    fn get_or_init<'b>(&'b mut self, id: usize) -> &'b ListItem<'a> {
        match self.items.get_mut(id) {
            Some(item) => match item {
                Some(item) => item,
                None => {
                    *item = Some((self.builder)(id));
                    item.as_ref().unwrap()
                }
            },
            _ => {
                unreachable!()
            }
        }
    }
}

/// A widget to display several items among which one can be selected (optional)
///
/// # Examples
///
/// ```
/// # use ratatui::widgets::{Block, Borders, LazyList, ListItem};
/// # use ratatui::style::{Style, Color, Modifier};
/// LazyList::new(3, |id| ListItem::new(format!("Item {id}")))
///     .block(Block::default().title("List").borders(Borders::ALL))
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
///     .highlight_symbol(">>");
/// ```
#[derive(Clone)]
pub struct LazyList<'a> {
    common: CommonList<'a>,
    items: LazyVec<'a>,
}

impl<'a> LazyList<'a> {
    pub fn new<F>(len: usize, builder: F) -> LazyList<'a>
    where
        F: Fn(usize) -> ListItem<'a> + 'a,
    {
        Self {
            items: LazyVec::new(len, builder),
            common: CommonList {
                block: None,
                style: Style::default(),
                start_corner: Corner::TopLeft,
                highlight_style: Style::default(),
                highlight_symbol: None,
                repeat_highlight_symbol: false,
            },
        }
    }

    pub fn block(mut self, block: Block<'a>) -> LazyList<'a> {
        self.common.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> LazyList<'a> {
        self.common.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> LazyList<'a> {
        self.common.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> LazyList<'a> {
        self.common.highlight_style = style;
        self
    }

    pub fn repeat_highlight_symbol(mut self, repeat: bool) -> LazyList<'a> {
        self.common.repeat_highlight_symbol = repeat;
        self
    }

    pub fn start_corner(mut self, corner: Corner) -> LazyList<'a> {
        self.common.start_corner = corner;
        self
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn get_items_bounds(
        &mut self,
        selected: Option<usize>,
        offset: usize,
        max_height: usize,
    ) -> (usize, usize) {
        let len = self.items.len();

        get_items_bounds(
            |id| self.items.get_or_init(id).height(),
            len,
            selected,
            offset,
            max_height,
        )
    }
}

impl<'a> StatefulWidget for LazyList<'a> {
    type State = ListState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.common.style);
        let list_area = match self.common.block.take() {
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

        let highlight_symbol = self.common.highlight_symbol.unwrap_or("");
        let blank_symbol = " ".repeat(highlight_symbol.width());

        let mut current_height = 0;

        for id in self.items.iter_ids().skip(state.offset).take(end - start) {
            render_item(
                &mut self.common,
                state,
                &list_area,
                buf,
                (id, self.items.get_or_init(id)),
                &mut current_height,
                &blank_symbol,
            );
        }
    }
}

impl<'a> Widget for LazyList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ListState::default();
        StatefulWidget::render(self, area, buf, &mut state);
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct List<'a> {
    common: CommonList<'a>,
    items: Vec<ListItem<'a>>,
}

impl<'a> List<'a> {
    pub fn new<T>(items: T) -> List<'a>
    where
        T: Into<Vec<ListItem<'a>>>,
    {
        List {
            items: items.into(),
            common: CommonList {
                block: None,
                style: Style::default(),
                start_corner: Corner::TopLeft,
                highlight_style: Style::default(),
                highlight_symbol: None,
                repeat_highlight_symbol: false,
            },
        }
    }

    pub fn block(mut self, block: Block<'a>) -> List<'a> {
        self.common.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> List<'a> {
        self.common.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> List<'a> {
        self.common.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> List<'a> {
        self.common.highlight_style = style;
        self
    }

    pub fn repeat_highlight_symbol(mut self, repeat: bool) -> List<'a> {
        self.common.repeat_highlight_symbol = repeat;
        self
    }

    pub fn start_corner(mut self, corner: Corner) -> List<'a> {
        self.common.start_corner = corner;
        self
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn get_items_bounds(
        &self,
        selected: Option<usize>,
        offset: usize,
        max_height: usize,
    ) -> (usize, usize) {
        get_items_bounds(
            |id| self.items[id].height(),
            self.items.len(),
            selected,
            offset,
            max_height,
        )
    }
}

impl<'a> StatefulWidget for List<'a> {
    type State = ListState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.common.style);
        let list_area = match self.common.block.take() {
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

        let highlight_symbol = self.common.highlight_symbol.unwrap_or("");
        let blank_symbol = " ".repeat(highlight_symbol.width());

        let mut current_height = 0;

        for entry in self
            .items
            .iter()
            .enumerate()
            .skip(state.offset)
            .take(end - start)
        {
            render_item(
                &mut self.common,
                state,
                &list_area,
                buf,
                entry,
                &mut current_height,
                &blank_symbol,
            );
        }
    }
}

impl<'a> Widget for List<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ListState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl<'a> Styled for List<'a> {
    type Item = List<'a>;

    fn style(&self) -> Style {
        self.common.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}

impl<'a> Styled for ListItem<'a> {
    type Item = ListItem<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;
    use crate::{
        assert_buffer_eq,
        style::{Color, Modifier, Stylize},
        text::{Line, Span},
        widgets::{Borders, StatefulWidget, Widget},
    };

    #[test]
    fn test_list_state_selected() {
        let mut state = ListState::default();
        assert_eq!(state.selected(), None);

        state.select(Some(1));
        assert_eq!(state.selected(), Some(1));

        state.select(None);
        assert_eq!(state.selected(), None);
    }

    #[test]
    fn test_list_state_select() {
        let mut state = ListState::default();
        assert_eq!(state.selected, None);
        assert_eq!(state.offset, 0);

        state.select(Some(2));
        assert_eq!(state.selected, Some(2));
        assert_eq!(state.offset, 0);

        state.select(None);
        assert_eq!(state.selected, None);
        assert_eq!(state.offset, 0);
    }

    #[test]
    fn test_list_item_new_from_str() {
        let item = ListItem::new("Test item");
        assert_eq!(item.content, Text::from("Test item"));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_list_item_new_from_string() {
        let item = ListItem::new("Test item".to_string());
        assert_eq!(item.content, Text::from("Test item"));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_list_item_new_from_cow_str() {
        let item = ListItem::new(Cow::Borrowed("Test item"));
        assert_eq!(item.content, Text::from("Test item"));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_list_item_new_from_span() {
        let span = Span::styled("Test item", Style::default().fg(Color::Blue));
        let item = ListItem::new(span.clone());
        assert_eq!(item.content, Text::from(span));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_list_item_new_from_spans() {
        let spans = Line::from(vec![
            Span::styled("Test ", Style::default().fg(Color::Blue)),
            Span::styled("item", Style::default().fg(Color::Red)),
        ]);
        let item = ListItem::new(spans.clone());
        assert_eq!(item.content, Text::from(spans));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_list_item_new_from_vec_spans() {
        let lines = vec![
            Line::from(vec![
                Span::styled("Test ", Style::default().fg(Color::Blue)),
                Span::styled("item", Style::default().fg(Color::Red)),
            ]),
            Line::from(vec![
                Span::styled("Second ", Style::default().fg(Color::Green)),
                Span::styled("line", Style::default().fg(Color::Yellow)),
            ]),
        ];
        let item = ListItem::new(lines.clone());
        assert_eq!(item.content, Text::from(lines));
        assert_eq!(item.style, Style::default());
    }

    #[test]
    fn test_list_item_style() {
        let item = ListItem::new("Test item").style(Style::default().bg(Color::Red));
        assert_eq!(item.content, Text::from("Test item"));
        assert_eq!(item.style, Style::default().bg(Color::Red));
    }

    #[test]
    fn test_list_item_height() {
        let item = ListItem::new("Test item");
        assert_eq!(item.height(), 1);

        let item = ListItem::new("Test item\nSecond line");
        assert_eq!(item.height(), 2);
    }

    #[test]
    fn test_list_item_width() {
        let item = ListItem::new("Test item");
        assert_eq!(item.width(), 9);
    }

    /// helper method to take a vector of strings and return a vector of list items
    fn list_items(items: Vec<&str>) -> Vec<ListItem> {
        items.iter().map(|i| ListItem::new(i.to_string())).collect()
    }

    /// helper method to render a widget to an empty buffer with the default state
    fn render_widget(widget: List<'_>, width: u16, height: u16) -> Buffer {
        let mut buffer = Buffer::empty(Rect::new(0, 0, width, height));
        Widget::render(widget, buffer.area, &mut buffer);
        buffer
    }

    /// helper method to render a widget to an empty buffer with a given state
    fn render_stateful_widget(
        widget: List<'_>,
        state: &mut ListState,
        width: u16,
        height: u16,
    ) -> Buffer {
        let mut buffer = Buffer::empty(Rect::new(0, 0, width, height));
        StatefulWidget::render(widget, buffer.area, &mut buffer, state);
        buffer
    }

    #[test]
    fn test_list_does_not_render_in_small_space() {
        let items = list_items(vec!["Item 0", "Item 1", "Item 2"]);
        let list = List::new(items.clone()).highlight_symbol(">>");
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));

        // attempt to render into an area of the buffer with 0 width
        Widget::render(list.clone(), Rect::new(0, 0, 0, 3), &mut buffer);
        assert_buffer_eq!(buffer, Buffer::empty(buffer.area));

        // attempt to render into an area of the buffer with 0 height
        Widget::render(list.clone(), Rect::new(0, 0, 15, 0), &mut buffer);
        assert_buffer_eq!(buffer, Buffer::empty(buffer.area));

        let list = List::new(items)
            .highlight_symbol(">>")
            .block(Block::default().borders(Borders::all()));
        // attempt to render into an area of the buffer with zero height after
        // setting the block borders
        Widget::render(list, Rect::new(0, 0, 15, 2), &mut buffer);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![
                "┌─────────────┐",
                "└─────────────┘",
                "               "
            ])
        );
    }

    #[test]
    fn test_list_combinations() {
        fn test_case_render(items: &[ListItem], expected_lines: Vec<&str>) {
            let list = List::new(items.to_owned()).highlight_symbol(">>");
            let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));

            Widget::render(list, buffer.area, &mut buffer);

            let expected = Buffer::with_lines(expected_lines);
            assert_buffer_eq!(buffer, expected);
        }
        fn test_case_render_stateful(
            items: &[ListItem],
            selected: Option<usize>,
            expected_lines: Vec<&str>,
        ) {
            let list = List::new(items.to_owned()).highlight_symbol(">>");
            let mut state = ListState::default().with_selected(selected);
            let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));

            StatefulWidget::render(list, buffer.area, &mut buffer, &mut state);

            let expected = Buffer::with_lines(expected_lines);
            assert_buffer_eq!(buffer, expected);
        }

        let empty_items: Vec<ListItem> = Vec::new();
        let single_item = list_items(vec!["Item 0"]);
        let multiple_items = list_items(vec!["Item 0", "Item 1", "Item 2"]);
        let multi_line_items = list_items(vec!["Item 0\nLine 2", "Item 1", "Item 2"]);

        // empty list
        test_case_render(
            &empty_items,
            vec![
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &empty_items,
            None,
            vec![
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &empty_items,
            Some(0),
            vec![
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );

        // single item
        test_case_render(
            &single_item,
            vec![
                "Item 0    ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &single_item,
            None,
            vec![
                "Item 0    ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &single_item,
            Some(0),
            vec![
                ">>Item 0  ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &single_item,
            Some(1),
            vec![
                "  Item 0  ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );

        // multiple items
        test_case_render(
            &multiple_items,
            vec![
                "Item 0    ",
                "Item 1    ",
                "Item 2    ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &multiple_items,
            None,
            vec![
                "Item 0    ",
                "Item 1    ",
                "Item 2    ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &multiple_items,
            Some(0),
            vec![
                ">>Item 0  ",
                "  Item 1  ",
                "  Item 2  ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &multiple_items,
            Some(1),
            vec![
                "  Item 0  ",
                ">>Item 1  ",
                "  Item 2  ",
                "          ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &multiple_items,
            Some(3),
            vec![
                "  Item 0  ",
                "  Item 1  ",
                "  Item 2  ",
                "          ",
                "          ",
            ],
        );

        // multi line items
        test_case_render(
            &multi_line_items,
            vec![
                "Item 0    ",
                "Line 2    ",
                "Item 1    ",
                "Item 2    ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &multi_line_items,
            None,
            vec![
                "Item 0    ",
                "Line 2    ",
                "Item 1    ",
                "Item 2    ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &multi_line_items,
            Some(0),
            vec![
                ">>Item 0  ",
                "  Line 2  ",
                "  Item 1  ",
                "  Item 2  ",
                "          ",
            ],
        );
        test_case_render_stateful(
            &multi_line_items,
            Some(1),
            vec![
                "  Item 0  ",
                "  Line 2  ",
                ">>Item 1  ",
                "  Item 2  ",
                "          ",
            ],
        );
    }

    #[test]
    fn test_list_with_empty_strings() {
        let items = list_items(vec!["Item 0", "", "", "Item 1", "Item 2"]);
        let list = List::new(items).block(Block::default().title("List").borders(Borders::ALL));
        let buffer = render_widget(list, 10, 7);

        let expected = Buffer::with_lines(vec![
            "┌List────┐",
            "│Item 0  │",
            "│        │",
            "│        │",
            "│Item 1  │",
            "│Item 2  │",
            "└────────┘",
        ]);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_list_block() {
        let items = list_items(vec!["Item 0", "Item 1", "Item 2"]);
        let list = List::new(items).block(Block::default().title("List").borders(Borders::ALL));
        let buffer = render_widget(list, 10, 7);

        let expected = Buffer::with_lines(vec![
            "┌List────┐",
            "│Item 0  │",
            "│Item 1  │",
            "│Item 2  │",
            "│        │",
            "│        │",
            "└────────┘",
        ]);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_list_style() {
        let items = list_items(vec!["Item 0", "Item 1", "Item 2"]);
        let list = List::new(items).style(Style::default().fg(Color::Red));
        let buffer = render_widget(list, 10, 5);

        let mut expected = Buffer::with_lines(vec![
            "Item 0    ",
            "Item 1    ",
            "Item 2    ",
            "          ",
            "          ",
        ]);
        expected.set_style(buffer.area, Style::default().fg(Color::Red));
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_list_highlight_symbol_and_style() {
        let items = list_items(vec!["Item 0", "Item 1", "Item 2"]);
        let list = List::new(items)
            .highlight_symbol(">>")
            .highlight_style(Style::default().fg(Color::Yellow));
        let mut state = ListState::default();
        state.select(Some(1));

        let buffer = render_stateful_widget(list, &mut state, 10, 5);

        let mut expected = Buffer::with_lines(vec![
            "  Item 0  ",
            ">>Item 1  ",
            "  Item 2  ",
            "          ",
            "          ",
        ]);
        expected.set_style(Rect::new(0, 1, 10, 1), Style::default().fg(Color::Yellow));
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_list_repeat_highlight_symbol() {
        let items = list_items(vec!["Item 0\nLine 2", "Item 1", "Item 2"]);
        let list = List::new(items)
            .highlight_symbol(">>")
            .highlight_style(Style::default().fg(Color::Yellow))
            .repeat_highlight_symbol(true);
        let mut state = ListState::default();
        state.select(Some(0));

        let buffer = render_stateful_widget(list, &mut state, 10, 5);

        let mut expected = Buffer::with_lines(vec![
            ">>Item 0  ",
            ">>Line 2  ",
            "  Item 1  ",
            "  Item 2  ",
            "          ",
        ]);
        expected.set_style(Rect::new(0, 0, 10, 2), Style::default().fg(Color::Yellow));
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_list_start_corner_top_left() {
        let items = list_items(vec!["Item 0", "Item 1", "Item 2"]);
        let list = List::new(items).start_corner(Corner::TopLeft);
        let buffer = render_widget(list, 10, 5);
        let expected = Buffer::with_lines(vec![
            "Item 0    ",
            "Item 1    ",
            "Item 2    ",
            "          ",
            "          ",
        ]);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_list_start_corner_bottom_left() {
        let items = list_items(vec!["Item 0", "Item 1", "Item 2"]);
        let list = List::new(items).start_corner(Corner::BottomLeft);
        let buffer = render_widget(list, 10, 5);
        let expected = Buffer::with_lines(vec![
            "          ",
            "          ",
            "Item 2    ",
            "Item 1    ",
            "Item 0    ",
        ]);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_list_truncate_items() {
        let items = list_items(vec!["Item 0", "Item 1", "Item 2", "Item 3", "Item 4"]);
        let list = List::new(items);
        let buffer = render_widget(list, 10, 3);
        let expected = Buffer::with_lines(vec!["Item 0    ", "Item 1    ", "Item 2    "]);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_list_long_lines() {
        let items = list_items(vec![
            "Item 0 with a very long line that will be truncated",
            "Item 1",
            "Item 2",
        ]);
        let list = List::new(items).highlight_symbol(">>");

        fn test_case(list: List, selected: Option<usize>, expected_lines: Vec<&str>) {
            let mut state = ListState::default();
            state.select(selected);
            let buffer = render_stateful_widget(list.clone(), &mut state, 15, 3);
            let expected = Buffer::with_lines(expected_lines);
            assert_buffer_eq!(buffer, expected);
        }

        test_case(
            list.clone(),
            None,
            vec!["Item 0 with a v", "Item 1         ", "Item 2         "],
        );
        test_case(
            list,
            Some(0),
            vec![">>Item 0 with a", "  Item 1       ", "  Item 2       "],
        );
    }

    #[test]
    fn test_list_selected_item_ensures_selected_item_is_visible_when_offset_is_before_visible_range(
    ) {
        let items = list_items(vec![
            "Item 0", "Item 1", "Item 2", "Item 3", "Item 4", "Item 5", "Item 6",
        ]);
        let list = List::new(items).highlight_symbol(">>");
        // Set the initial visible range to items 3, 4, and 5
        let mut state = ListState::default().with_selected(Some(1)).with_offset(3);
        let buffer = render_stateful_widget(list, &mut state, 10, 3);

        let expected = Buffer::with_lines(vec![">>Item 1  ", "  Item 2  ", "  Item 3  "]);
        assert_buffer_eq!(buffer, expected);
        assert_eq!(state.selected, Some(1));
        assert_eq!(
            state.offset, 1,
            "did not scroll the selected item into view"
        );
    }

    #[test]
    fn test_list_selected_item_ensures_selected_item_is_visible_when_offset_is_after_visible_range()
    {
        let items = list_items(vec![
            "Item 0", "Item 1", "Item 2", "Item 3", "Item 4", "Item 5", "Item 6",
        ]);
        let list = List::new(items).highlight_symbol(">>");
        // Set the initial visible range to items 3, 4, and 5
        let mut state = ListState::default().with_selected(Some(6)).with_offset(3);
        let buffer = render_stateful_widget(list, &mut state, 10, 3);

        let expected = Buffer::with_lines(vec!["  Item 4  ", "  Item 5  ", ">>Item 6  "]);
        assert_buffer_eq!(buffer, expected);
        assert_eq!(state.selected, Some(6));
        assert_eq!(
            state.offset, 4,
            "did not scroll the selected item into view"
        );
    }

    #[test]
    fn list_can_be_stylized() {
        assert_eq!(
            List::new(vec![])
                .black()
                .on_white()
                .bold()
                .not_dim()
                .common
                .style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::DIM)
        )
    }

    #[test]
    fn list_item_can_be_stylized() {
        assert_eq!(
            ListItem::new("").black().on_white().bold().not_dim().style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::DIM)
        )
    }
}
