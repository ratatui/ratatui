use super::StatefulWidget;
use crate::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols::scrollbar::{Set, DOUBLE_HORIZONTAL, DOUBLE_VERTICAL},
};

/// An enum representing the direction of scrolling in a Scrollbar widget.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum ScrollDirection {
    /// Forward scroll direction, usually corresponds to scrolling downwards or rightwards.
    #[default]
    Forward,
    /// Backward scroll direction, usually corresponds to scrolling upwards or leftwards.
    Backward,
}

/// A struct representing the state of a Scrollbar widget.
///
/// For example, in the following list, assume there are 4 bullet points:
///
/// - the `position` is 0
/// - the `content_length` is 4
/// - the `viewport_content_length` is 2
///
/// ```text
/// ┌───────────────┐
/// │1. this is a   █
/// │   single item █
/// │2. this is a   ║
/// │   second item ║
/// └───────────────┘
/// ```
///
/// If you don't have multi-line content, you can leave the `viewport_content_length` set to the
/// default of 0 and it'll use the track size as a `viewport_content_length`.
#[derive(Debug, Default, Clone, Copy)]
pub struct ScrollbarState {
    // The current position within the scrollable content.
    position: u16,
    // The total length of the scrollable content.
    content_length: u16,
    // The length of content in current viewport.
    viewport_content_length: u16,
}

impl ScrollbarState {
    /// Sets the scroll position of the scrollbar and returns the modified ScrollbarState.
    pub fn position(mut self, position: u16) -> Self {
        self.position = position;
        self
    }

    /// Sets the length of the scrollable content and returns the modified ScrollbarState.
    pub fn content_length(mut self, content_length: u16) -> Self {
        self.content_length = content_length;
        self
    }

    /// Sets the length of the viewport content and returns the modified ScrollbarState.
    pub fn viewport_content_length(mut self, viewport_content_length: u16) -> Self {
        self.viewport_content_length = viewport_content_length;
        self
    }

    /// Decrements the scroll position by one, ensuring it doesn't go below zero.
    pub fn prev(&mut self) {
        self.position = self.position.saturating_sub(1);
    }

    /// Increments the scroll position by one, ensuring it doesn't exceed the length of the content.
    pub fn next(&mut self) {
        self.position = self
            .position
            .saturating_add(1)
            .clamp(0, self.content_length.saturating_sub(1))
    }

    /// Sets the scroll position to the start of the scrollable content.
    pub fn first(&mut self) {
        self.position = 0;
    }

    /// Sets the scroll position to the end of the scrollable content.
    pub fn last(&mut self) {
        self.position = self.content_length.saturating_sub(1)
    }

    /// Changes the scroll position based on the provided ScrollDirection.
    pub fn scroll(&mut self, direction: ScrollDirection) {
        match direction {
            ScrollDirection::Forward => {
                self.next();
            }
            ScrollDirection::Backward => {
                self.prev();
            }
        }
    }
}

/// Scrollbar Orientation
#[derive(Debug, Default, Clone)]
pub enum ScrollbarOrientation {
    #[default]
    VerticalRight,
    VerticalLeft,
    HorizontalBottom,
    HorizontalTop,
}

/// A widget to display a scrollbar
///
/// The following components of the scrollbar are customizable in symbol and style.
///
/// ```text
/// <--▮------->
/// ^  ^   ^   ^
/// │  │   │   └ end
/// │  │   └──── track
/// │  └──────── thumb
/// └─────────── begin
/// ```
#[derive(Debug, Clone)]
pub struct Scrollbar<'a> {
    orientation: ScrollbarOrientation,
    thumb_style: Style,
    thumb_symbol: &'a str,
    track_style: Style,
    track_symbol: &'a str,
    begin_symbol: Option<&'a str>,
    begin_style: Style,
    end_symbol: Option<&'a str>,
    end_style: Style,
}

impl<'a> Default for Scrollbar<'a> {
    fn default() -> Self {
        Self {
            orientation: ScrollbarOrientation::default(),
            thumb_symbol: DOUBLE_VERTICAL.thumb,
            thumb_style: Style::default(),
            track_symbol: DOUBLE_VERTICAL.track,
            track_style: Style::default(),
            begin_symbol: Some(DOUBLE_VERTICAL.begin),
            begin_style: Style::default(),
            end_symbol: Some(DOUBLE_VERTICAL.end),
            end_style: Style::default(),
        }
    }
}

impl<'a> Scrollbar<'a> {
    pub fn new(orientation: ScrollbarOrientation) -> Self {
        Self::default().orientation(orientation)
    }

    /// Sets the orientation of the scrollbar.
    /// Resets the symbols to [`DOUBLE_VERTICAL`] or [`DOUBLE_HORIZONTAL`] based on orientation
    pub fn orientation(mut self, orientation: ScrollbarOrientation) -> Self {
        self.orientation = orientation;
        let set = if self.is_vertical() {
            DOUBLE_VERTICAL
        } else {
            DOUBLE_HORIZONTAL
        };
        self.symbols(set)
    }

    /// Sets the orientation and symbols for the scrollbar from a [`Set`].
    pub fn orientation_and_symbol(mut self, orientation: ScrollbarOrientation, set: Set) -> Self {
        self.orientation = orientation;
        self.symbols(set)
    }

    /// Sets the symbol that represents the thumb of the scrollbar.
    pub fn thumb_symbol(mut self, thumb_symbol: &'a str) -> Self {
        self.thumb_symbol = thumb_symbol;
        self
    }

    /// Sets the style that represents the thumb of the scrollbar.
    pub fn thumb_style(mut self, thumb_style: Style) -> Self {
        self.thumb_style = thumb_style;
        self
    }

    /// Sets the symbol that represents the track of the scrollbar.
    pub fn track_symbol(mut self, track_symbol: &'a str) -> Self {
        self.track_symbol = track_symbol;
        self
    }

    /// Sets the style that is used for the track of the scrollbar.
    pub fn track_style(mut self, track_style: Style) -> Self {
        self.track_style = track_style;
        self
    }

    /// Sets the symbol that represents the beginning of the scrollbar.
    pub fn begin_symbol(mut self, begin_symbol: Option<&'a str>) -> Self {
        self.begin_symbol = begin_symbol;
        self
    }

    /// Sets the style that is used for the beginning of the scrollbar.
    pub fn begin_style(mut self, begin_style: Style) -> Self {
        self.begin_style = begin_style;
        self
    }

    /// Sets the symbol that represents the end of the scrollbar.
    pub fn end_symbol(mut self, end_symbol: Option<&'a str>) -> Self {
        self.end_symbol = end_symbol;
        self
    }

    /// Sets the style that is used for the end of the scrollbar.
    pub fn end_style(mut self, end_style: Style) -> Self {
        self.end_style = end_style;
        self
    }

    /// Sets the symbols used for the various parts of the scrollbar from a [`Set`].
    ///
    /// ```text
    /// <--▮------->
    /// ^  ^   ^   ^
    /// │  │   │   └ end
    /// │  │   └──── track
    /// │  └──────── thumb
    /// └─────────── begin
    /// ```
    ///
    /// Only sets begin_symbol and end_symbol if they already contain a value.
    /// If begin_symbol and/or end_symbol were set to `None` explicitly, this function will respect
    /// that choice.
    pub fn symbols(mut self, symbol: Set) -> Self {
        self.track_symbol = symbol.track;
        self.thumb_symbol = symbol.thumb;
        if self.begin_symbol.is_some() {
            self.begin_symbol = Some(symbol.begin);
        }
        if self.end_symbol.is_some() {
            self.end_symbol = Some(symbol.end);
        }
        self
    }

    /// Sets the style used for the various parts of the scrollbar from a [`Style`].
    /// ```text
    /// <--▮------->
    /// ^  ^   ^   ^
    /// │  │   │   └ end
    /// │  │   └──── track
    /// │  └──────── thumb
    /// └─────────── begin
    /// ```
    pub fn style(mut self, style: Style) -> Self {
        self.track_style = style;
        self.thumb_style = style;
        self.begin_style = style;
        self.end_style = style;
        self
    }

    fn is_vertical(&self) -> bool {
        match self.orientation {
            ScrollbarOrientation::VerticalRight | ScrollbarOrientation::VerticalLeft => true,
            ScrollbarOrientation::HorizontalBottom | ScrollbarOrientation::HorizontalTop => false,
        }
    }

    fn get_track_area(&self, area: Rect) -> Rect {
        // Decrease track area if a begin arrow is present
        let area = if self.begin_symbol.is_some() {
            if self.is_vertical() {
                // For vertical scrollbar, reduce the height by one
                Rect::new(
                    area.x,
                    area.y + 1,
                    area.width,
                    area.height.saturating_sub(1),
                )
            } else {
                // For horizontal scrollbar, reduce the width by one
                Rect::new(
                    area.x + 1,
                    area.y,
                    area.width.saturating_sub(1),
                    area.height,
                )
            }
        } else {
            area
        };
        // Further decrease scrollbar area if an end arrow is present
        if self.end_symbol.is_some() {
            if self.is_vertical() {
                // For vertical scrollbar, reduce the height by one
                Rect::new(area.x, area.y, area.width, area.height.saturating_sub(1))
            } else {
                // For horizontal scrollbar, reduce the width by one
                Rect::new(area.x, area.y, area.width.saturating_sub(1), area.height)
            }
        } else {
            area
        }
    }

    fn should_not_render(&self, track_start: u16, track_end: u16, content_length: u16) -> bool {
        if track_end - track_start == 0 || content_length == 0 {
            return true;
        }
        false
    }

    fn get_track_start_end(&self, area: Rect) -> (u16, u16, u16) {
        match self.orientation {
            ScrollbarOrientation::VerticalRight => {
                (area.top(), area.bottom(), area.right().saturating_sub(1))
            }
            ScrollbarOrientation::VerticalLeft => (area.top(), area.bottom(), area.left()),
            ScrollbarOrientation::HorizontalBottom => {
                (area.left(), area.right(), area.bottom().saturating_sub(1))
            }
            ScrollbarOrientation::HorizontalTop => (area.left(), area.right(), area.top()),
        }
    }

    /// Calculate the starting and ending position of a scrollbar thumb.
    ///
    /// The scrollbar thumb's position and size are determined based on the current state of the
    /// scrollbar, and the dimensions of the scrollbar track.
    ///
    /// This function returns a tuple `(thumb_start, thumb_end)` where `thumb_start` is the position
    /// at which the scrollbar thumb begins, and `thumb_end` is the position at which the
    /// scrollbar thumb ends.
    ///
    /// The size of the thumb (i.e., `thumb_end - thumb_start`) is proportional to the ratio of the
    /// viewport content length to the total content length.
    ///
    /// The position of the thumb (i.e., `thumb_start`) is proportional to the ratio of the current
    /// scroll position to the total content length.
    fn get_thumb_start_end(
        &self,
        state: &ScrollbarState,
        track_start_end: (u16, u16),
    ) -> (u16, u16) {
        // let (track_start, track_end) = track_start_end;
        // let track_size = track_end - track_start;
        // let thumb_size =
        //     ((state.viewport_content_length / state.content_length) * track_size).max(1);
        // let thumb_start = (state.position / state.content_length) *
        //                    state.viewport_content_length;
        // let thumb_end = thumb_size + thumb_start;
        // (thumb_start, thumb_end)

        let (track_start, track_end) = track_start_end;

        let viewport_content_length = if state.viewport_content_length == 0 {
            track_end - track_start
        } else {
            state.viewport_content_length
        };

        let scroll_position_ratio = (state.position as f64 / state.content_length as f64).min(1.0);

        let thumb_size = (((viewport_content_length as f64 / state.content_length as f64)
            * (track_end - track_start) as f64)
            .round() as u16)
            .max(1);

        let track_size = (track_end - track_start).saturating_sub(thumb_size);

        let thumb_start = track_start + (scroll_position_ratio * track_size as f64).round() as u16;

        let thumb_end = thumb_start + thumb_size;

        (thumb_start, thumb_end)
    }
}

impl<'a> StatefulWidget for Scrollbar<'a> {
    type State = ScrollbarState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        //
        // For ScrollbarOrientation::VerticalRight
        //
        //                   ┌───────── track_axis  (x)
        //                   v
        //   ┌───────────────┐
        //   │               ║<──────── track_start (y1)
        //   │               █
        //   │               █
        //   │               ║
        //   │               ║<──────── track_end   (y2)
        //   └───────────────┘
        //
        // For ScrollbarOrientation::HorizontalBottom
        //
        //   ┌───────────────┐
        //   │               │
        //   │               │
        //   │               │
        //   └═══███═════════┘<──────── track_axis  (y)
        //    ^             ^
        //    │             └────────── track_end   (x2)
        //    │
        //    └──────────────────────── track_start (x1)
        //

        // Find track_start, track_end, and track_axis
        let area = self.get_track_area(area);
        let (track_start, track_end, track_axis) = self.get_track_start_end(area);

        if self.should_not_render(track_start, track_end, state.content_length) {
            return;
        }

        let (thumb_start, thumb_end) = self.get_thumb_start_end(state, (track_start, track_end));

        for i in track_start..track_end {
            let (style, symbol) = if i >= thumb_start && i < thumb_end {
                (self.thumb_style, self.thumb_symbol)
            } else {
                (self.track_style, self.track_symbol)
            };

            if self.is_vertical() {
                buf.set_string(track_axis, i, symbol, style);
            } else {
                buf.set_string(i, track_axis, symbol, style);
            }
        }

        if let Some(s) = self.begin_symbol {
            if self.is_vertical() {
                buf.set_string(track_axis, track_start - 1, s, self.begin_style);
            } else {
                buf.set_string(track_start - 1, track_axis, s, self.begin_style);
            }
        };
        if let Some(s) = self.end_symbol {
            if self.is_vertical() {
                buf.set_string(track_axis, track_end, s, self.end_style);
            } else {
                buf.set_string(track_end, track_axis, s, self.end_style);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assert_buffer_eq,
        symbols::scrollbar::{HORIZONTAL, VERTICAL},
    };

    #[test]
    fn test_no_render_when_area_zero() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 0, 0));
        let mut state = ScrollbarState::default().position(0).content_length(1);
        Scrollbar::default().render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::empty(buffer.area));
    }

    #[test]
    fn test_no_render_when_height_zero_with_without_arrows() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 0));
        let mut state = ScrollbarState::default().position(0).content_length(1);
        Scrollbar::default().render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::empty(buffer.area));

        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 0));
        let mut state = ScrollbarState::default().position(0).content_length(1);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::empty(buffer.area));
    }

    #[test]
    fn test_no_render_when_height_too_small_for_arrows() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
        let mut state = ScrollbarState::default().position(0).content_length(1);
        Scrollbar::default().render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["    ", "    "]));
    }

    #[test]
    fn test_renders_all_thumbs_at_minimum_height_without_arrows() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
        let mut state = ScrollbarState::default().position(0).content_length(1);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["   █", "   █"]));
    }

    #[test]
    fn test_renders_all_thumbs_at_minimum_height_and_minimum_width_without_arrows() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 2));
        let mut state = ScrollbarState::default().position(0).content_length(1);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["█", "█"]));
    }

    #[test]
    fn test_renders_two_arrows_one_thumb_at_minimum_height_with_arrows() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 3));
        let mut state = ScrollbarState::default().position(0).content_length(1);
        Scrollbar::default().render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["   ▲", "   █", "   ▼"]));
    }

    #[test]
    fn test_no_render_when_content_length_zero() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 2));
        let mut state = ScrollbarState::default().position(0).content_length(0);
        Scrollbar::default().render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["  ", "  "]));
    }

    #[test]
    fn test_renders_all_thumbs_when_height_equals_content_length() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 2));
        let mut state = ScrollbarState::default().position(0).content_length(2);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![" █", " █"]));

        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 8));
        let mut state = ScrollbarState::default().position(0).content_length(8);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![" █", " █", " █", " █", " █", " █", " █", " █"])
        );
    }

    #[test]
    fn test_renders_single_vertical_thumb_when_content_length_square_of_height() {
        for i in 0..=17 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 4));
            let mut state = ScrollbarState::default().position(i).content_length(16);
            Scrollbar::default()
                .begin_symbol(None)
                .end_symbol(None)
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 2 {
                vec![" █", " ║", " ║", " ║"]
            } else if i <= 7 {
                vec![" ║", " █", " ║", " ║"]
            } else if i <= 13 {
                vec![" ║", " ║", " █", " ║"]
            } else {
                vec![" ║", " ║", " ║", " █"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_renders_single_horizontal_thumb_when_content_length_square_of_width() {
        for i in 0..=17 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
            let mut state = ScrollbarState::default().position(i).content_length(16);
            Scrollbar::default()
                .begin_symbol(None)
                .end_symbol(None)
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 2 {
                vec!["    ", "█═══"]
            } else if i <= 7 {
                vec!["    ", "═█══"]
            } else if i <= 13 {
                vec!["    ", "══█═"]
            } else {
                vec!["    ", "═══█"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_renders_one_thumb_for_large_content_relative_to_height() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
        let mut state = ScrollbarState::default().position(0).content_length(1600);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .render(buffer.area, &mut buffer, &mut state);
        let expected = vec!["    ", "█═══"];
        assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));

        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
        let mut state = ScrollbarState::default().position(800).content_length(1600);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .render(buffer.area, &mut buffer, &mut state);
        let expected = vec!["    ", "══█═"];
        assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
    }

    #[test]
    fn test_renders_two_thumb_default_symbols_for_content_double_height() {
        for i in 0..=7 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 4));
            let mut state = ScrollbarState::default().position(i).content_length(8);
            Scrollbar::default()
                .begin_symbol(None)
                .end_symbol(None)
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec![" █", " █", " ║", " ║"]
            } else if i <= 5 {
                vec![" ║", " █", " █", " ║"]
            } else {
                vec![" ║", " ║", " █", " █"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_renders_two_thumb_custom_symbols_for_content_double_height() {
        for i in 0..=7 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 4));
            let mut state = ScrollbarState::default().position(i).content_length(8);
            Scrollbar::default()
                .symbols(VERTICAL)
                .begin_symbol(None)
                .end_symbol(None)
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec![" █", " █", " │", " │"]
            } else if i <= 5 {
                vec![" │", " █", " █", " │"]
            } else {
                vec![" │", " │", " █", " █"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_renders_two_thumb_default_symbols_for_content_double_width() {
        for i in 0..=7 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
            let mut state = ScrollbarState::default().position(i).content_length(8);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(None)
                .end_symbol(None)
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec!["    ", "██══"]
            } else if i <= 5 {
                vec!["    ", "═██═"]
            } else {
                vec!["    ", "══██"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_renders_two_thumb_custom_symbols_for_content_double_width() {
        for i in 0..=7 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
            let mut state = ScrollbarState::default().position(i).content_length(8);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .symbols(HORIZONTAL)
                .begin_symbol(None)
                .end_symbol(None)
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec!["    ", "██──"]
            } else if i <= 5 {
                vec!["    ", "─██─"]
            } else {
                vec!["    ", "──██"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_rendering_viewport_content_length() {
        for i in 0..=16 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 2));
            let mut state = ScrollbarState::default()
                .position(i)
                .content_length(16)
                .viewport_content_length(4);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(Some(DOUBLE_HORIZONTAL.begin))
                .end_symbol(Some(DOUBLE_HORIZONTAL.end))
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec!["        ", "◄██════►"]
            } else if i <= 5 {
                vec!["        ", "◄═██═══►"]
            } else if i <= 9 {
                vec!["        ", "◄══██══►"]
            } else if i <= 13 {
                vec!["        ", "◄═══██═►"]
            } else {
                vec!["        ", "◄════██►"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }

        for i in 0..=16 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 2));
            let mut state = ScrollbarState::default()
                .position(i)
                .content_length(16)
                .viewport_content_length(1);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(Some(DOUBLE_HORIZONTAL.begin))
                .end_symbol(Some(DOUBLE_HORIZONTAL.end))
                .render(buffer.area, &mut buffer, &mut state);
            dbg!(i);
            let expected = if i <= 1 {
                vec!["        ", "◄█═════►"]
            } else if i <= 4 {
                vec!["        ", "◄═█════►"]
            } else if i <= 7 {
                vec!["        ", "◄══█═══►"]
            } else if i <= 11 {
                vec!["        ", "◄═══█══►"]
            } else if i <= 14 {
                vec!["        ", "◄════█═►"]
            } else {
                vec!["        ", "◄═════█►"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_rendering_begin_end_arrows_horizontal_bottom() {
        for i in 0..=16 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 2));
            let mut state = ScrollbarState::default().position(i).content_length(16);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(Some(DOUBLE_HORIZONTAL.begin))
                .end_symbol(Some(DOUBLE_HORIZONTAL.end))
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec!["        ", "◄██════►"]
            } else if i <= 5 {
                vec!["        ", "◄═██═══►"]
            } else if i <= 9 {
                vec!["        ", "◄══██══►"]
            } else if i <= 13 {
                vec!["        ", "◄═══██═►"]
            } else {
                vec!["        ", "◄════██►"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_rendering_begin_end_arrows_horizontal_top() {
        for i in 0..=16 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 2));
            let mut state = ScrollbarState::default().position(i).content_length(16);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalTop)
                .begin_symbol(Some(DOUBLE_HORIZONTAL.begin))
                .end_symbol(Some(DOUBLE_HORIZONTAL.end))
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec!["◄██════►", "        "]
            } else if i <= 5 {
                vec!["◄═██═══►", "        "]
            } else if i <= 9 {
                vec!["◄══██══►", "        "]
            } else if i <= 13 {
                vec!["◄═══██═►", "        "]
            } else {
                vec!["◄════██►", "        "]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_rendering_only_begin_arrow_horizontal_bottom() {
        for i in 0..=16 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 2));
            let mut state = ScrollbarState::default().position(i).content_length(16);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(Some(DOUBLE_HORIZONTAL.begin))
                .end_symbol(None)
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec!["        ", "◄███════"]
            } else if i <= 5 {
                vec!["        ", "◄═███═══"]
            } else if i <= 9 {
                vec!["        ", "◄══███══"]
            } else if i <= 13 {
                vec!["        ", "◄═══███═"]
            } else {
                vec!["        ", "◄════███"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }
}
