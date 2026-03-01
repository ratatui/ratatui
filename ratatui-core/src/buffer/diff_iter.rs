use core::fmt;

use unicode_width::UnicodeWidthStr;

use crate::buffer::{Buffer, Cell, CellDiffOption};
use crate::layout::Rect;

/// A zero-allocation iterator over the differences between two buffers of the same width.
///
/// Yields `(x, y, &Cell)` tuples for each cell in `next` that differs from the corresponding cell
/// in `prev`. Handles multi-width characters (including VS16 emoji trailing cells) and
/// [`CellDiffOption`] directives.
pub struct BufferDiff<'prev, 'next> {
    /// The next (current) buffer's cells.
    next: &'next [Cell],
    /// The previous buffer's cells.
    prev: &'prev [Cell],
    /// Buffer width (for `pos_of` calculation).
    area: Rect,
    /// Current position in the flat cell array.
    pos: usize,
    /// When processing VS16 trailing cells, tracks the range of trailing indices still to yield.
    trailing: Option<TrailingState>,
}

/// Tracks pending trailing-cell yields for VS16 wide characters.
struct TrailingState {
    next_index: usize,
    end: usize,
}

impl<'prev, 'next> BufferDiff<'prev, 'next> {
    /// Creates a new iterator over the differences between `prev` and `next` terminal cells.
    ///
    /// Returns an error if the buffers have different `x`, `y`, or `width` values.
    /// Heights may differ; the iterator uses the minimum of the two.
    pub(crate) fn new(prev: &'prev Buffer, next: &'next Buffer) -> Result<Self, BufferDiffError> {
        if prev.area.x != next.area.x
            || prev.area.y != next.area.y
            || prev.area.width != next.area.width
        {
            return Err(BufferDiffError {
                prev: prev.area,
                next: next.area,
            });
        }

        let mut area = prev.area;
        area.height = area.height.min(next.area.height);

        Ok(Self {
            next: &next.content,
            prev: &prev.content,
            area,
            pos: 0,
            trailing: None,
        })
    }

    /// Converts a flat index to (x, y) coordinates.
    const fn pos_of(&self, index: usize) -> (u16, u16) {
        let w = self.area.width as usize;

        let x = index % w + self.area.x as usize;
        let y = index / w + self.area.y as usize;

        (x as u16, y as u16)
    }
}

impl<'next> Iterator for BufferDiff<'_, 'next> {
    type Item = (u16, u16, &'next Cell);

    fn next(&mut self) -> Option<Self::Item> {
        // First, yield any pending VS16 trailing cells.
        if let Some(TrailingState {
            ref mut next_index,
            end,
        }) = self.trailing
        {
            while *next_index < end {
                let j = *next_index;
                *next_index += 1;

                if self.prev[j] != self.next[j] {
                    let (tx, ty) = self.pos_of(j);
                    return Some((tx, ty, &self.next[j]));
                }
            }

            // Done with trailing cells; resume main loop past the wide character.
            self.pos = end;
            self.trailing = None;
        }

        let len = self.next.len().min(self.prev.len());
        while self.pos < len {
            let i = self.pos;
            self.pos += 1;

            let current = &self.next[i];
            let previous = &self.prev[i];

            match current.diff_option {
                CellDiffOption::Skip => {}
                CellDiffOption::ForcedWidth(width) => {
                    self.pos += width.get().saturating_sub(1);
                    if current != previous {
                        let (x, y) = self.pos_of(i);
                        return Some((x, y, &self.next[i]));
                    }
                }
                CellDiffOption::None => {
                    // If the current cell is multi-width, ensure the trailing cells are
                    // explicitly cleared when they previously contained non-blank content.
                    // Some terminals do not reliably clear the trailing cell(s) when printing
                    // a wide grapheme, which can result in visual artifacts (e.g., leftover
                    // characters). Emitting an explicit update for the trailing cells avoids
                    // this.
                    let symbol = current.symbol();
                    let cell_width = if symbol.len() == 1 { 1 } else { symbol.width() };

                    if current == previous {
                        // Equal cells still need to account for multi-width skip.
                        self.pos += cell_width.saturating_sub(1);
                        continue;
                    }

                    // Work around terminals that fail to clear the trailing cell of certain
                    // emoji presentation sequences (those containing VS16 / U+FE0F).
                    // Only emit explicit clears for such sequences to avoid bloating diffs
                    // for standard wide characters (e.g., CJK), which terminals handle well.
                    let contains_vs16 = cell_width > 1 && symbol.chars().any(|c| c == '\u{FE0F}');
                    if contains_vs16 {
                        let trailing_end = (i + cell_width).min(len);
                        self.trailing = Some(TrailingState {
                            next_index: i + 1,
                            end: trailing_end,
                        });
                    } else if cell_width > 1 {
                        self.pos += cell_width.saturating_sub(1);
                    } else {
                        // single-width character, no position adjustment needed
                    }

                    let (x, y) = self.pos_of(i);
                    return Some((x, y, &self.next[i]));
                }
            }
        }

        None
    }
}

/// Error returned when two buffers have incompatible areas for diffing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferDiffError {
    prev: Rect,
    next: Rect,
}

impl fmt::Display for BufferDiffError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "buffer areas must have the same x, y, and width: prev={:?}, next={:?}",
            self.prev, self.next,
        )
    }
}

impl core::error::Error for BufferDiffError {}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use core::num::NonZeroUsize;

    use super::*;
    use crate::buffer::Buffer;
    use crate::layout::Rect;

    /// Helper: collect diff iterator results into a Vec for comparison with `Buffer::diff`.
    fn collect_diff<'a>(prev: &'a Buffer, next: &'a Buffer) -> Vec<(u16, u16, &'a Cell)> {
        BufferDiff::new(prev, next).unwrap().collect()
    }

    #[test]
    fn empty_buffers_yield_no_diffs() {
        let rect = Rect::new(0, 0, 5, 1);
        let buf = Buffer::empty(rect);
        assert!(collect_diff(&buf, &buf).is_empty());
    }

    #[test]
    fn identical_buffers_yield_no_diffs() {
        let buf = Buffer::with_lines(["hello"]);
        assert!(collect_diff(&buf, &buf).is_empty());
    }

    #[test]
    fn single_cell_change() {
        let prev = Buffer::with_lines(["hello"]);
        let next = Buffer::with_lines(["hallo"]);
        let diff: Vec<_> = collect_diff(&prev, &next);
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].0, 1); // x
        assert_eq!(diff[0].1, 0); // y
        assert_eq!(diff[0].2.symbol(), "a");
    }

    #[test]
    fn all_cells_changed() {
        let prev = Buffer::with_lines(["aaa"]);
        let next = Buffer::with_lines(["bbb"]);
        let diff: Vec<_> = collect_diff(&prev, &next);
        assert_eq!(diff.len(), 3);
    }

    #[test]
    fn skip_cells_are_skipped() {
        let prev = Buffer::with_lines(["abc"]);
        let mut next = Buffer::with_lines(["xyz"]);
        next.content[1].diff_option = CellDiffOption::Skip;

        let diff: Vec<_> = collect_diff(&prev, &next);
        assert_eq!(diff.len(), 2);
        assert_eq!(diff[0].2.symbol(), "x");
        assert_eq!(diff[1].2.symbol(), "z");
    }

    #[test]
    fn forced_width_skips_trailing() {
        let prev = Buffer::with_lines(["abcd"]);
        let mut next = Buffer::with_lines(["xbcd"]);
        next.content[0].diff_option = CellDiffOption::ForcedWidth(NonZeroUsize::new(2).unwrap());

        let diff: Vec<_> = collect_diff(&prev, &next);
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].2.symbol(), "x");
    }

    #[test]
    fn vs16_trailing_cell_unchanged() {
        use crate::style::{Color, Style};

        let rect = Rect::new(0, 0, 4, 1);
        let mut prev = Buffer::empty(rect);
        prev.set_string(0, 0, "⌨️", Style::new());
        prev.set_string(2, 0, "ab", Style::new());

        let mut next = Buffer::empty(rect);
        next.set_string(0, 0, "⌨️", Style::new().fg(Color::Red));
        next.set_string(2, 0, "ab", Style::new());

        // Only the main emoji cell (0,0) differs (different style);
        // the trailing cell (1,0) is identical in both buffers.
        let diff: Vec<_> = collect_diff(&prev, &next);
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].0, 0);
        assert_eq!(diff[0].1, 0);
    }
}
