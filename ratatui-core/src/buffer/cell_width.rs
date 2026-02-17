use unicode_width::UnicodeWidthStr;

/// Returns the display width of a value in terminal cells.
///
/// This trait provides a unified way to compute cell widths for both string content
/// and [`Cell`](super::Cell)s. For strings, the width is derived from
/// [`UnicodeWidthStr`]. For cells,
/// [`CellDiffOption::ForcedWidth`](super::CellDiffOption::ForcedWidth) is respected when set.
// Public because ratatui-widgets needs access, but not part of the user-facing API.
#[doc(hidden)]
pub trait CellWidth {
    /// Returns the display width in terminal cells.
    fn cell_width(&self) -> u16;
}

impl CellWidth for str {
    fn cell_width(&self) -> u16 {
        if self.len() == 1 {
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
