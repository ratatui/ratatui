use core::iter::{Enumerate, Zip};
use core::slice::Iter;

use unicode_width::UnicodeWidthStr;

use crate::buffer::{Buffer, Cell, CellDiffOption};

type CellIter<'next, 'prev> = Enumerate<Zip<Iter<'next, Cell>, Iter<'prev, Cell>>>;

/// A zero-allocation iterator over the differences between two buffers.
///
/// Yields `(x, y, &Cell)` tuples for each cell in `next` that differs from the corresponding cell
/// in `prev`. Handles multi-width characters (including VS16 emoji trailing cells) and
/// [`CellDiffOption`] directives.
pub struct BufferDiff<'prev, 'next> {
    /// Zipped iterator over (next, prev) cell pairs with index.
    iter: CellIter<'next, 'prev>,
    /// The next (current) buffer's cells — needed for trailing cell random access.
    next: &'next [Cell],
    /// The previous buffer's cells — needed for trailing cell comparison.
    prev: &'prev [Cell],
    /// Buffer width (for `pos_of` calculation).
    width: u16,
    x_offset: u16,
    y_offset: u16,
    /// Number of main-loop cells still to skip (from multi-width characters).
    to_skip: u16,
    /// When processing VS16 trailing cells, tracks the range of trailing indices still to yield.
    trailing: TrailingState,
}

/// Tracks pending trailing-cell yields for VS16 wide characters.
enum TrailingState {
    /// No trailing cells pending.
    None,
    /// Yielding trailing cells for indices `next_index..end` (then resume main loop with
    /// `resume_skip` cells to skip).
    Pending {
        next_index: usize,
        end: usize,
        resume_skip: u16,
    },
}

impl<'prev, 'next> BufferDiff<'prev, 'next> {
    /// Creates a new iterator over the differences between `prev` and `next` terminal cells.
    pub(crate) fn new(prev: &'prev Buffer, next: &'next Buffer) -> Self {
        Self {
            iter: next.content.iter().zip(prev.content.iter()).enumerate(),
            next: &next.content,
            prev: &prev.content,
            width: prev.area.width,
            x_offset: prev.area.x,
            y_offset: prev.area.y,
            to_skip: 0,
            trailing: TrailingState::None,
        }
    }

    /// Converts a flat index to (x, y) coordinates.
    const fn pos_of(&self, index: usize) -> (u16, u16) {
        let x = index % self.width as usize + self.x_offset as usize;
        let y = index / self.width as usize + self.y_offset as usize;
        (x as u16, y as u16)
    }
}

impl<'next> Iterator for BufferDiff<'_, 'next> {
    type Item = (u16, u16, &'next Cell);

    fn next(&mut self) -> Option<Self::Item> {
        // First, yield any pending VS16 trailing cells.
        if let TrailingState::Pending {
            ref mut next_index,
            end,
            resume_skip,
        } = self.trailing
        {
            while *next_index < end {
                let j = *next_index;
                *next_index += 1;

                // Make sure that we are still inside the buffer.
                if j >= self.next.len() || j >= self.prev.len() {
                    break;
                }

                if self.prev[j] != self.next[j] {
                    let (tx, ty) = self.pos_of(j);
                    return Some((tx, ty, &self.next[j]));
                }
            }

            // Done with trailing cells; resume main loop.
            self.to_skip = resume_skip;
            self.trailing = TrailingState::None;
        }

        // Cells from the current buffer to skip due to preceding multi-width characters taking
        // their place (the skipped cells should be blank anyway), or due to per-cell-skipping:
        for (i, (current, previous)) in self.iter.by_ref() {
            if self.to_skip > 0 {
                self.to_skip -= 1;
                continue;
            }

            match current.diff_option {
                CellDiffOption::Skip => {}
                CellDiffOption::ForcedWidth(width) => {
                    self.to_skip = width.get().saturating_sub(1) as u16;
                    if current != previous {
                        let (x, y) = self.pos_of(i);
                        return Some((x, y, &self.next[i]));
                    }
                }
                CellDiffOption::None => {
                    if current == previous {
                        // Equal cells still need to account for multi-width skip.
                        let cell_width = current.symbol().width();
                        if cell_width > 1 {
                            self.to_skip = cell_width.saturating_sub(1) as u16;
                        }
                        continue;
                    }

                    // If the current cell is multi-width, ensure the trailing cells are
                    // explicitly cleared when they previously contained non-blank content.
                    // Some terminals do not reliably clear the trailing cell(s) when printing
                    // a wide grapheme, which can result in visual artifacts (e.g., leftover
                    // characters). Emitting an explicit update for the trailing cells avoids
                    // this.
                    let symbol = current.symbol();
                    let cell_width = symbol.width();

                    // Work around terminals that fail to clear the trailing cell of certain
                    // emoji presentation sequences (those containing VS16 / U+FE0F).
                    // Only emit explicit clears for such sequences to avoid bloating diffs
                    // for standard wide characters (e.g., CJK), which terminals handle well.
                    let contains_vs16 = cell_width > 1 && symbol.chars().any(|c| c == '\u{FE0F}');

                    if contains_vs16 {
                        let trailing_start = i + 1;
                        let trailing_end =
                            (i + cell_width).min(self.next.len().min(self.prev.len()));

                        self.trailing = TrailingState::Pending {
                            next_index: trailing_start,
                            end: trailing_end,
                            resume_skip: cell_width.saturating_sub(1) as u16,
                        };
                        // Advance the inner iterator past the trailing cells so we
                        // don't visit them again in the main loop.
                        for _ in 1..cell_width {
                            self.iter.next();
                        }
                    }

                    if !contains_vs16 && cell_width > 1 {
                        self.to_skip = cell_width.saturating_sub(1) as u16;
                    }

                    let (x, y) = self.pos_of(i);
                    return Some((x, y, &self.next[i]));
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use core::num::NonZeroUsize;

    use super::*;
    use crate::buffer::Buffer;
    use crate::layout::Rect;

    /// Helper: collect diff iterator results into a Vec for comparison with `Buffer::diff`.
    fn collect_diff<'a>(prev: &'a Buffer, next: &'a Buffer) -> Vec<(u16, u16, &'a Cell)> {
        BufferDiff::new(prev, next).collect()
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
}
