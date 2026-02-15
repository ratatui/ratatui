use unicode_width::UnicodeWidthStr;

/// Returns the display width of a value in terminal cells.
///
/// This trait provides a unified way to compute cell widths for both string content
/// and [`Cell`](super::Cell)s. For strings, the width is derived from
/// [`UnicodeWidthStr`]. For cells, [`CellDiffOption::ForcedWidth`](super::CellDiffOption::ForcedWidth)
/// is respected when set.
// Public because ratatui-widgets needs access, but not part of the user-facing API.
#[doc(hidden)]
pub trait CellWidth {
    /// Returns the display width in terminal cells.
    fn cell_width(&self) -> u16;
}

impl CellWidth for str {
    fn cell_width(&self) -> u16 {
        if self.len() == 1 {
            // Single-byte strings are always printable ASCII (width 1) in practice:
            // control characters are filtered out before reaching this point by
            // `Buffer::set_stringn` and `Span::styled_graphemes`.
            1
        } else {
            self.width() as u16
        }
    }
}
