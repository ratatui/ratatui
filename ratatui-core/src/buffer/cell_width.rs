use unicode_width::UnicodeWidthStr;

/// Returns the display width of a value in terminal cells.
///
/// This trait provides a unified way to compute cell widths for both string content
/// and [`Cell`](super::Cell)s. For strings, the width is derived from
/// [`UnicodeWidthStr`]. For cells, [`CellDiffOption::ForcedWidth`](super::CellDiffOption::ForcedWidth)
/// is respected when set.
pub trait StrCellWidth {
    /// Returns the display width in terminal cells.
    fn cell_width(&self) -> u16;
}

impl StrCellWidth for str {
    fn cell_width(&self) -> u16 {
        if self.len() == 1 {
            1
        } else {
            self.width() as u16
        }
    }
}
