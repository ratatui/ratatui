#![allow(dead_code)]
#![allow(unreachable_code)]

use crate::{buffer::Buffer, layout::Rect, style::Style};

use super::StatefulWidget;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ScrollPosition {
    x: u16,
    y: u16,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum ScrollDirection {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Default)]
struct ScrollState {
    visible_height: u16,
    visible_width: u16,
    scroll_position: ScrollPosition,
    scroll_item_index: usize,
    item_count: usize,
    last_scroll_direction: ScrollDirection,
}

#[rustfmt::skip]
impl ScrollState {
    fn prev_item(&mut self) { self.scroll_item_index = self.scroll_item_index.saturating_sub(1); }
    fn next_item(&mut self) {
        self.scroll_item_index =
            self.scroll_item_index .saturating_add(1).clamp(0, self.item_count - 1)
    }
    fn first_item(&mut self) { self.scroll_item_index = 0; }
    fn last_item(&mut self) { self.scroll_item_index = self.item_count.saturating_sub(1) }

    fn scroll_up(&mut self) { self.scroll_position.y = self.scroll_position.y.saturating_sub(1) }
    fn scroll_down(&mut self) { self.scroll_position.y = self.scroll_position.y.saturating_add(1) }
    fn scroll_left(&mut self) { self.scroll_position.x = self.scroll_position.x.saturating_sub(1) }
    fn scroll_right(&mut self) { self.scroll_position.x = self.scroll_position.x.saturating_add(1) }

    fn scroll_direction(&mut self, direction: ScrollDirection) {
        // TODO perhaps the scroll_xxx() methods should be convenience on top of this
        // method, which would allow this method to update the position based on the
        // visible item
        match direction {
            ScrollDirection::Up => { self.scroll_up(); }
            ScrollDirection::Down => { self.scroll_down(); }
            ScrollDirection::Left => { self.scroll_left(); }
            ScrollDirection::Right => { self.scroll_right(); }
        }
    }

    fn scroll_to_top(&mut self) { self.scroll_position.y = 0; }
    fn scroll_to_bottom(&mut self) {
        // needs to be calculated base on total item height and visible height
        self.scroll_position.y = todo!();
    }
    fn scroll_to_position(&mut self, position: ScrollPosition) {
        self.scroll_position = position;
    }

    // this adjusts the scroll position to ensure that it's within the bounds of the
    // area allowed to be scrolled to. e.g. this might be the total width of the
    // content minus the width of the area, or the total height of the content
    // minus the height of the area. This allows the various scroll functions to
    // be called without worrying about the bounds of the scrollable area.
    fn ensure_scroll_position_in_bounds(&mut self, x_max: u16, y_max: u16) {
        self.scroll_position.x = self.scroll_position.x.clamp(0, x_max);
        self.scroll_position.y = self.scroll_position.y.clamp(0, y_max);
    }

    fn calculate_vertical_scroll_position(&self, total_height: usize, height: usize) -> (usize, usize) {
        let start_percent = self.scroll_position.y as f32 / total_height as f32;
        let end_percent = (self.scroll_position.y + height as u16) as f32 / total_height as f32;
        let start = (start_percent * height as f32) as usize;
        let end = (end_percent * height as f32) as usize;
        (start, end)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum ShowScrollBar {
    #[default]
    Always, // Always show the scrollbar
    Auto,  // Only show when the content is larger than the area
    Never, // Never show the scrollbar
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ScrollBar {
    // TODO support horizontal / vertical bars?
    show: ShowScrollBar,
}

impl ScrollBar {
    fn new() -> Self {
        Self::default()
    }
}

impl StatefulWidget for ScrollBar {
    type State = ScrollState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if self.show == ShowScrollBar::Never {
            return;
        }
        let total_height = state.item_count.saturating_sub(1);
        let (start, end) =
            state.calculate_vertical_scroll_position(total_height, area.height as usize);
        for y in 0..start {
            buf.set_string(area.x, area.y + y as u16, "│", Style::default());
        }
        for y in start..end {
            buf.set_string(area.x, area.y + y as u16, "🮈", Style::default());
        }
        for y in end..area.height as usize - 1 {
            buf.set_string(area.x, area.y + y as u16, "│", Style::default());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Style;
    use crate::{
        assert_buffer_eq, buffer::Buffer, layout::Rect, text::Text, widgets::StatefulWidget,
    };

    #[derive(Clone, Debug, Default)]
    struct ScrollableParagraph<'a> {
        text: Text<'a>,
        show_scrollbar: bool,
    }

    impl StatefulWidget for ScrollableParagraph<'_> {
        type State = ScrollState;

        fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
            let total_height = self.text.lines.len();
            let total_width = self.text.width() - self.show_scrollbar as usize;

            let inner_area = if self.show_scrollbar {
                let scrollbar_area = Rect {
                    x: area.x + area.width - 1,
                    ..area
                };
                ScrollBar::default().render(scrollbar_area, buf, state);
                Rect {
                    width: area.width - 1,
                    ..area
                }
            } else {
                area
            };

            state.ensure_scroll_position_in_bounds(
                total_width.saturating_sub(area.width as usize) as u16,
                total_height.saturating_sub(area.height as usize) as u16,
            );

            // this is a simplified version of the actual rendering code
            for (y, line) in self
                .text
                .lines
                .iter()
                .skip(state.scroll_position.y as usize)
                .take(inner_area.height as usize)
                .enumerate()
            {
                let (_hidden, content) =
                    line.0[0].content.split_at(state.scroll_position.x as usize);
                buf.set_stringn(
                    0,
                    y as u16,
                    content,
                    inner_area.width as usize,
                    Style::default(),
                );
            }
        }
    }

    #[test]
    fn test_scroll_state() {
        fn test_case(
            state: &mut ScrollState,
            expected_str: &str,
            expected_scroll_position: ScrollPosition,
        ) {
            let mut buf = Buffer::empty(Rect::new(0, 0, 3, 1));
            let paragraph = ScrollableParagraph {
                text: Text::from("Hello\nWorld"),
                show_scrollbar: false,
            };
            paragraph.render(buf.area, &mut buf, state);
            assert_buffer_eq!(buf, Buffer::with_lines(vec![expected_str]));
            assert_eq!(state.scroll_position, expected_scroll_position);
        }

        let mut state = ScrollState::default();
        test_case(&mut state, "Hel", ScrollPosition { x: 0, y: 0 });

        state.scroll_right();
        test_case(&mut state, "ell", ScrollPosition { x: 1, y: 0 });

        state.scroll_right();
        test_case(&mut state, "llo", ScrollPosition { x: 2, y: 0 });

        // attempt to scroll beyond the width of the content
        state.scroll_right();
        test_case(&mut state, "llo", ScrollPosition { x: 2, y: 0 });

        state.scroll_down();
        test_case(&mut state, "rld", ScrollPosition { x: 2, y: 1 });

        // attempt to scroll beyond the height of the content
        state.scroll_down();
        test_case(&mut state, "rld", ScrollPosition { x: 2, y: 1 });

        state.scroll_left();
        test_case(&mut state, "orl", ScrollPosition { x: 1, y: 1 });

        state.scroll_left();
        test_case(&mut state, "Wor", ScrollPosition { x: 0, y: 1 });

        // attempt to scroll beyond the start of the content
        state.scroll_left();
        test_case(&mut state, "Wor", ScrollPosition { x: 0, y: 1 });

        state.scroll_up();
        test_case(&mut state, "Hel", ScrollPosition { x: 0, y: 0 });

        // attempt to scroll beyond the top of the content
        state.scroll_up();
        test_case(&mut state, "Hel", ScrollPosition { x: 0, y: 0 });
    }

    #[test]
    fn test_handles_3_lines_of_content() {
        let paragraph = ScrollableParagraph {
            text: Text::from("Hello\nWorld\nHow are you?"),
            show_scrollbar: false,
        };
        let mut buf = Buffer::empty(Rect::new(0, 0, 3, 2));
        let mut state = ScrollState::default();
        paragraph.clone().render(buf.area, &mut buf, &mut state);
        assert_buffer_eq!(buf, Buffer::with_lines(vec!["Hel", "Wor"]));

        state.scroll_down();
        paragraph.clone().render(buf.area, &mut buf, &mut state);
        assert_buffer_eq!(buf, Buffer::with_lines(vec!["Wor", "How"]));

        // attempt to scroll beyond the height of the content
        state.scroll_down();
        paragraph.clone().render(buf.area, &mut buf, &mut state);
        assert_buffer_eq!(buf, Buffer::with_lines(vec!["Wor", "How"]));
    }

    #[test]
    fn test_scrollbar_shown_when_content_is_longer_than_area() {
        fn test_case(state: &mut ScrollState, expected: Vec<&str>) {
            let paragraph = ScrollableParagraph {
                text: Text::from("1\n2\n3\n4\n5\n6\n7\n8\n9"),
                show_scrollbar: true,
            };
            let mut buf = Buffer::empty(Rect::new(0, 0, 2, 3));
            paragraph.render(buf.area, &mut buf, state);
            assert_buffer_eq!(buf, Buffer::with_lines(expected));
        }
        let mut state = ScrollState::default();
        // TODO the scroll bar is only showing 3 elements, but crosses two lines due to
        test_case(&mut state, vec!["1🮈", "2▕", "3▕"]);
        state.scroll_down();
        test_case(&mut state, vec!["2🮈", "3▕", "4▕"]);
        state.scroll_down();
        test_case(&mut state, vec!["3🮈", "4▕", "5▕"]);
        state.scroll_down();
        test_case(&mut state, vec!["4▕", "5🮈", "6▕"]);
        state.scroll_down();
        test_case(&mut state, vec!["5▕", "6🮈", "7▕"]);
        state.scroll_down();
        test_case(&mut state, vec!["6▕", "7🮈", "8▕"]);
        state.scroll_down();
        test_case(&mut state, vec!["7▕", "8▕", "9🮈"]);
        state.scroll_down();
        test_case(&mut state, vec!["7▕", "8▕", "9🮈"]);
    }
}
