//! The [`Clear`] widget allows you to clear a certain area to allow overdrawing (e.g. for popups).
use ratatui_core::buffer::{Buffer, CellWidth};
use ratatui_core::layout::Rect;
use ratatui_core::widgets::Widget;

/// A widget to clear/reset a certain area to allow overdrawing (e.g. for popups).
///
/// This widget **cannot be used to clear the terminal on the first render** as `ratatui` assumes
/// the render area is empty. Use `Terminal::clear` instead.
///
/// # Examples
///
/// ```
/// use ratatui::Frame;
/// use ratatui::layout::Rect;
/// use ratatui::widgets::{Block, Clear};
///
/// fn draw_on_clear(f: &mut Frame, area: Rect) {
///     let block = Block::bordered().title("Block");
///     f.render_widget(Clear, area); // <- this will clear/reset the area first
///     f.render_widget(block, area); // now render the block widget
/// }
/// ```
///
/// # Popup Example
///
/// For a more complete example how to utilize `Clear` to realize popups see
/// the example `examples/popup.rs`
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Clear;

impl Widget for Clear {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &Clear {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(*buf.area());
        if area.is_empty() {
            return;
        }

        // A double-width glyph sitting just to the left of the area keeps its full visual width
        // after the loop below resets the cells inside the area: its right half occupies
        // `area.x`, so it spills into whatever the caller draws on top of `Clear` (e.g. a popup's
        // left border). Shrink it to a space first, while we can still see its pre-clear width.
        if area.x > buf.area().x {
            for y in area.top()..area.bottom() {
                if buf[(area.x - 1, y)].cell_width() > 1 {
                    buf[(area.x - 1, y)].set_symbol(" ");
                }
            }
        }

        for x in area.left()..area.right() {
            for y in area.top()..area.bottom() {
                buf[(x, y)].reset();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui_core::buffer::Buffer;
    use ratatui_core::layout::Rect;
    use ratatui_core::style::Style;
    use ratatui_core::widgets::Widget;

    use super::*;

    #[test]
    fn render() {
        let mut buffer = Buffer::with_lines(["xxxxxxxxxxxxxxx"; 7]);
        let clear = Clear;
        clear.render(Rect::new(1, 2, 3, 4), &mut buffer);
        let expected = Buffer::with_lines([
            "xxxxxxxxxxxxxxx",
            "xxxxxxxxxxxxxxx",
            "x   xxxxxxxxxxx",
            "x   xxxxxxxxxxx",
            "x   xxxxxxxxxxx",
            "x   xxxxxxxxxxx",
            "xxxxxxxxxxxxxxx",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn render_partially_out_of_bounds() {
        let mut buffer = Buffer::with_lines(["xxxxxxxxxxxxxxx"; 7]);
        let clear = Clear;
        clear.render(Rect::new(2, 0, 100, 100), &mut buffer);
        let expected = Buffer::with_lines(["xx             "; 7]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn render_fully_out_of_bounds() {
        let mut buffer = Buffer::with_lines(["xxxxxxxxxxxxxxx"; 7]);
        let clear = Clear;
        clear.render(Rect::new(100, 0, 100, 100), &mut buffer);
        let expected = Buffer::with_lines(["xxxxxxxxxxxxxxx"; 7]);
        assert_eq!(buffer, expected);
    }

    /// Regression test for a double-width glyph immediately to the left of the cleared area.
    ///
    /// `字` occupies columns 3 and 4. Clearing `Rect::new(4, 0, 2, 1)` only resets columns 4-5,
    /// so `字` at column 3 kept its full two-cell width and its right half spilled into column 4,
    /// underneath whatever gets drawn on top of `Clear` (e.g. a popup's left border).
    #[test]
    fn render_shrinks_wide_glyph_straddling_left_edge() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 6, 1));
        buffer.set_string(0, 0, "abc字", Style::default());
        Clear.render(Rect::new(4, 0, 2, 1), &mut buffer);
        let expected = Buffer::with_lines(["abc   "]);
        assert_eq!(buffer, expected);
    }

    /// A single-width glyph at `area.x - 1` must be left untouched.
    #[test]
    fn render_left_edge_single_width_glyph_untouched() {
        let mut buffer = Buffer::with_lines(["abcde "]);
        Clear.render(Rect::new(4, 0, 2, 1), &mut buffer);
        let expected = Buffer::with_lines(["abcd  "]);
        assert_eq!(buffer, expected);
    }

    /// The boundary patch must not panic when the area's left edge is also the buffer's left
    /// edge (there is no `area.x - 1` cell to look at in that case).
    #[test]
    fn render_area_at_buffer_left_edge_does_not_panic() {
        let mut buffer = Buffer::with_lines(["abcd"]);
        Clear.render(Rect::new(0, 0, 4, 1), &mut buffer);
        let expected = Buffer::with_lines(["    "]);
        assert_eq!(buffer, expected);
    }
}
