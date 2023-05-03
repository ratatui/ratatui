use crate::{
    buffer::Buffer,
    layout::{Rect, Size},
    style::Style,
    text::Text,
    widgets::Widget,
};

use super::{SizeHint, WidgetList, WidgetListState};

pub type ListState = WidgetListState;

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

impl<'a> SizeHint for ListItem<'a> {
    fn size_hint(&self, area: &Rect) -> Size {
        Size::new(
            area.width.min(self.content.width() as u16),
            self.content.height() as u16,
        )
    }
}

impl<'a> Widget for ListItem<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);

        for (j, line) in self.content.lines.iter().enumerate() {
            buf.set_line(area.x, area.y + j as u16, line, area.width);
        }
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
pub type List<'a> = WidgetList<'a, ListItem<'a>>;

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{
        assert_buffer_eq,
        layout::Corner,
        style::Color,
        text::{Line, Span},
        widgets::{Block, Borders, StatefulWidget, Widget},
    };

    use super::*;

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
        assert_eq!(state.selected(), None);
        assert_eq!(state.offset(), 0);

        state.select(Some(2));
        assert_eq!(state.selected(), Some(2));
        assert_eq!(state.offset(), 0);

        state.select(None);
        assert_eq!(state.selected(), None);
        assert_eq!(state.offset(), 0);
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
        assert_eq!(state.selected(), Some(1));
        assert_eq!(
            state.offset(),
            1,
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
        assert_eq!(state.selected(), Some(6));
        assert_eq!(
            state.offset(),
            4,
            "did not scroll the selected item into view"
        );
    }
}
