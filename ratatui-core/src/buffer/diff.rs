use crate::buffer::{Buffer, Cell, CellDiffOption, CellWidth};
use crate::layout::Rect;

/// A zero-allocation iterator over the differences between two buffers of the same width.
///
/// Yields `(x, y, &Cell)` tuples for each cell in `next` that differs from the corresponding cell
/// in `prev`. Handles multi-width characters (including VS16 emoji trailing cells) and
/// [`CellDiffOption`] directives.
#[derive(Debug)]
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
#[derive(Debug)]
struct TrailingState {
    next_index: usize,
    end: usize,
}

impl<'prev, 'next> BufferDiff<'prev, 'next> {
    /// Creates a new iterator over the differences between `prev` and `next` terminal cells.
    ///
    /// Heights may differ; the iterator uses the minimum of the two.
    ///
    /// # Panics
    ///
    /// Panics if the buffers have different `x`, `y`, or `width` values.
    pub(crate) fn new(prev: &'prev Buffer, next: &'next Buffer) -> Self {
        assert!(
            prev.area.x == next.area.x
                && prev.area.y == next.area.y
                && prev.area.width == next.area.width,
            "buffer areas must have the same x, y, and width: prev={:?}, next={:?}",
            prev.area,
            next.area,
        );

        let mut area = prev.area;
        area.height = area.height.min(next.area.height);

        Self {
            next: &next.content,
            prev: &prev.content,
            area,
            pos: 0,
            trailing: None,
        }
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

                // Only emit update if the symbol has changed.
                // The style of hidden trailing cells is not visible, so style
                // differences alone should not trigger updates that can cause
                // cursor positioning issues on some terminals.
                if !is_skip(&self.next[j]) && self.prev[j].symbol() != self.next[j].symbol() {
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
                _ if is_skip(current) => {}

                CellDiffOption::ForcedWidth(width) => {
                    self.pos += width.get().saturating_sub(1) as usize;
                    if current != previous {
                        let (x, y) = self.pos_of(i);
                        return Some((x, y, &self.next[i]));
                    }
                }
                CellDiffOption::None | CellDiffOption::AlwaysUpdate => {
                    // If the current cell is multi-width, ensure the trailing cells are
                    // explicitly cleared when they previously contained non-blank content.
                    // Some terminals do not reliably clear the trailing cell(s) when printing
                    // a wide grapheme, which can result in visual artifacts (e.g., leftover
                    // characters). Emitting an explicit update for the trailing cells avoids
                    // this.
                    let cell_width = current.cell_width() as usize;
                    if matches!(current.diff_option, CellDiffOption::None) && current == previous {
                        // Equal cells still need to account for multi-width skip.
                        self.pos += cell_width.saturating_sub(1);
                        continue;
                    }

                    // Work around terminals that fail to clear the trailing cell of certain
                    // emoji presentation sequences (those containing VS16 / U+FE0F).
                    // Only emit explicit clears for such sequences to avoid bloating diffs
                    // for standard wide characters (e.g., CJK), which terminals handle well.
                    let contains_vs16 =
                        cell_width > 1 && current.symbol().chars().any(|c| c == '\u{FE0F}');

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

/// Returns `true` if this cell should be skipped during diffing.
#[allow(deprecated)]
const fn is_skip(cell: &Cell) -> bool {
    matches!(cell.diff_option, CellDiffOption::Skip)
        || (cell.skip && matches!(cell.diff_option, CellDiffOption::None))
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use core::num::NonZeroU16;

    use compact_str::CompactString;

    use super::*;
    use crate::buffer::Buffer;
    use crate::layout::Rect;

    #[test]
    fn empty_buffers_yield_no_diffs() {
        let rect = Rect::new(0, 0, 5, 1);
        let buf = Buffer::empty(rect);
        let diff: Vec<_> = BufferDiff::new(&buf, &buf).collect();
        assert!(diff.is_empty());
    }

    #[test]
    fn identical_buffers_yield_no_diffs() {
        let buf = Buffer::with_lines(["hello"]);
        let diff: Vec<_> = BufferDiff::new(&buf, &buf).collect();
        assert!(diff.is_empty());
    }

    #[test]
    fn single_cell_change() {
        let prev = Buffer::with_lines(["hello"]);
        let next = Buffer::with_lines(["hallo"]);
        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].0, 1); // x
        assert_eq!(diff[0].1, 0); // y
        assert_eq!(diff[0].2.symbol(), "a");
    }

    #[test]
    fn all_cells_changed() {
        let prev = Buffer::with_lines(["aaa"]);
        let next = Buffer::with_lines(["bbb"]);
        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();
        assert_eq!(diff.len(), 3);
    }

    #[test]
    fn skip_cells_are_skipped() {
        let prev = Buffer::with_lines(["abc"]);
        let mut next = Buffer::with_lines(["xyz"]);
        next.content[1].diff_option = CellDiffOption::Skip;

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();
        assert_eq!(diff.len(), 2);
        assert_eq!(diff[0].2.symbol(), "x");
        assert_eq!(diff[1].2.symbol(), "z");
    }

    #[test]
    fn always_update_cells_are_emitted_even_when_identical() {
        let mut prev = Buffer::with_lines(["abc"]);
        prev.content[1].diff_option = CellDiffOption::AlwaysUpdate;

        let mut next = Buffer::with_lines(["abc"]);
        next.content[1].diff_option = CellDiffOption::AlwaysUpdate;

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].0, 1);
        assert_eq!(diff[0].1, 0);
        assert_eq!(diff[0].2.symbol(), "b");
    }

    #[test]
    fn forced_width_skips_trailing() {
        let prev = Buffer::with_lines(["abcd"]);
        let mut next = Buffer::with_lines(["xbcd"]);
        next.content[0].diff_option = CellDiffOption::ForcedWidth(NonZeroU16::new(2).unwrap());

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();
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
        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].0, 0);
        assert_eq!(diff[0].1, 0);
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_skip_field_is_respected() {
        let prev = Buffer::with_lines(["abc"]);
        let mut next = Buffer::with_lines(["xyz"]);
        next.content[1].skip = true;

        let diff: CompactString = BufferDiff::new(&prev, &next)
            .map(|(_, _, cell)| cell.symbol())
            .collect();

        assert_eq!(diff, "xz");
    }

    #[test]
    #[allow(deprecated)]
    fn forced_width_takes_precedence_over_deprecated_skip() {
        let prev = Buffer::with_lines(["abcd"]);
        let mut next = Buffer::with_lines(["xbcd"]);
        next.content[0].skip = true;
        next.content[0].diff_option = CellDiffOption::ForcedWidth(NonZeroU16::new(2).unwrap());

        // ForcedWidth wins over skip=true, so the cell is diffed with forced width
        let diff: CompactString = BufferDiff::new(&prev, &next)
            .map(|(_, _, cell)| cell.symbol())
            .collect();

        assert_eq!(diff, "x");
    }

    #[test]
    #[should_panic(expected = "buffer areas must have the same x, y, and width")]
    fn mismatched_widths_panics() {
        let prev = Buffer::empty(Rect::new(0, 0, 5, 1));
        let next = Buffer::empty(Rect::new(0, 0, 10, 1));
        BufferDiff::new(&prev, &next);
    }
}
