use unicode_width::UnicodeWidthStr;

/// Returns the display width of a value in terminal cells.
///
/// This trait provides a unified way to compute cell widths for both string content
/// and [`Cell`](super::Cell)s:
///
/// - **`str`**: width is derived from [`UnicodeWidthStr`], with a fast path for single-byte ASCII
///   characters.
/// - **[`Cell`](super::Cell)**: returns the
///   [`CellDiffOption::ForcedWidth`](super::CellDiffOption::ForcedWidth) when set, otherwise falls
///   back to the width of the cell's symbol.
// Public because ratatui-widgets needs access, but not part of the user-facing API.
#[doc(hidden)]
pub trait CellWidth {
    /// Returns the display width in terminal cells.
    fn cell_width(&self) -> u16;
}

impl CellWidth for str {
    fn cell_width(&self) -> u16 {
        if self.len() == 1 {
            // Control characters are filtered out by `Span::styled_graphemes()` and
            // `Buffer::set_stringn()` before reaching this point. `Cell::set_symbol()`
            // and `set_char()` do not filter, but those are low-level APIs where the
            // caller is responsible for providing valid content.
            debug_assert!(
                !self.as_bytes()[0].is_ascii_control(),
                "control character passed to cell_width without filtering"
            );
            1
        } else {
            self.width() as u16
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii() {
        assert_eq!("a".cell_width(), 1);
    }

    #[test]
    fn wide_char() {
        assert_eq!("あ".cell_width(), 2);
    }

    #[test]
    fn empty() {
        assert_eq!("".cell_width(), 0);
    }
}
