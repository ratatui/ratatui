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

        // Handle double-width glyphs that straddle the rect's boundaries.
        //
        // Left edge: a double-width glyph at `area.x - 1` occupies two visual
        // columns. Its right half spills into `area.x` and would render as
        // garbled overlap with whatever the caller draws on top of the cleared
        // area. Replace the glyph with a single-width space.
        if area.left() > 0 {
            for y in area.top()..area.bottom() {
                let cell = &mut buf[(area.left() - 1, y)];
                if cell.symbol().cell_width() > 1 {
                    cell.set_symbol(" ");
                }
            }
        }

        // Right edge: when `area.right() - 1` is the left half of a double-width
        // glyph, its continuation cell at `area.right()` holds the empty string
        // that ratatui uses for trailing cells. After `area.right() - 1` is
        // cleared below, the continuation becomes an orphan — the terminal
        // paints stale content there. Replace it with a space.
        if area.right() < buf.area().right() {
            for y in area.top()..area.bottom() {
                let prev = &buf[(area.right() - 1, y)];
                if prev.symbol().cell_width() > 1 {
                    buf[(area.right(), y)].reset();
                }
            }
        }

        // Clear the inner area.
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
    use ratatui_core::widgets::Widget;

    use super::*;
    use ratatui_core::style::Style;

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

    #[test]
    fn left_edge_double_width_replaced_with_space() {
        // 字 at col 3 (width 2) — its right half spills into col 4.
        // Clearing [4..6) should also replace 字 at col 3 with " ".
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 1));
        buf.set_string(0, 0, "abc字", Style::default());
        assert_eq!(buf[(3, 0)].symbol().cell_width(), 2);
        Clear.render(Rect::new(4, 0, 2, 1), &mut buf);
        assert_eq!(buf[(3, 0)].symbol(), " ");
        assert_eq!(buf[(4, 0)].symbol(), " ");
        assert_eq!(buf[(5, 0)].symbol(), " ");
    }

    #[test]
    fn left_edge_single_width_left_alone() {
        // ASCII 'd' at col 3 (width 1) — no spill.
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 1));
        buf.set_string(0, 0, "abcd", Style::default());
        Clear.render(Rect::new(4, 0, 2, 1), &mut buf);
        assert_eq!(buf[(3, 0)].symbol(), "d");
        assert_eq!(buf[(4, 0)].symbol(), " ");
        assert_eq!(buf[(5, 0)].symbol(), " ");
    }

    #[test]
    fn right_edge_continuation_cleared() {
        // 字 spans cols 3,4. Clear [0..4) → the orphan at col 4 should be " ".
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 1));
        buf.set_string(0, 0, "abc字d", Style::default());
        Clear.render(Rect::new(0, 0, 4, 1), &mut buf);
        assert_eq!(buf[(3, 0)].symbol(), " ");
        assert_eq!(buf[(4, 0)].symbol(), " ");
        assert_eq!(buf[(5, 0)].symbol(), "d");
    }

    #[test]
    fn right_edge_no_overhang_leaves_neighbour_alone() {
        // ASCII 'd' at col 4 (width 1), clear [0..4) — col 4 untouched.
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 1));
        buf.set_string(0, 0, "abcd", Style::default());
        Clear.render(Rect::new(0, 0, 4, 1), &mut buf);
        assert_eq!(buf[(3, 0)].symbol(), " ");
        assert_eq!(buf[(4, 0)].symbol(), " ");
    }

    #[test]
    fn area_at_buffer_edges_does_not_panic() {
        // Left edge: area.x == 0 → skip left-edge fix.
        let mut buf = Buffer::empty(Rect::new(0, 0, 2, 1));
        let clear = Clear;
        clear.render(Rect::new(0, 0, 1, 1), &mut buf);

        // Right edge: area.right() == buf.area().right() → skip right-edge fix.
        let mut buf = Buffer::empty(Rect::new(0, 0, 2, 1));
        let clear = Clear;
        clear.render(Rect::new(1, 0, 1, 1), &mut buf);
    }
}
