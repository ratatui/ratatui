use crate::buffer::{Buffer, Cell, CellDiffOption, CellWidth};
use crate::layout::Rect;
use crate::style::{Color, Modifier};

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
    /// Tracks trailing cells and a leading cell that may be deferred until after they are cleared.
    trailing: Option<TrailingState>,
}

/// Tracks pending trailing-cell yields for a wide character update.
#[derive(Debug)]
struct TrailingState {
    next_index: usize,
    end: usize,
    /// When `true`, all cells in the trailing range are emitted unconditionally: the previous
    /// wide character's style was visible on blank cells, so the terminal may show stale style
    /// there and every trailing cell must be refreshed.
    ///
    /// When `false`, only cells whose symbol changed are emitted.
    force: bool,
    /// Leading cell to repaint after its trailing cells have been processed.
    deferred_leading_cell: Option<usize>,
}

/// Modifiers that are visually apparent on a blank (space) cell.
const VISIBLE_ON_BLANK: Modifier = Modifier::REVERSED
    .union(Modifier::UNDERLINED)
    .union(Modifier::SLOW_BLINK)
    .union(Modifier::RAPID_BLINK)
    .union(Modifier::CROSSED_OUT);

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
        let len = self.next.len().min(self.prev.len());

        // First, yield any pending trailing cells.
        if let Some(TrailingState {
            next_index,
            end,
            force,
            deferred_leading_cell,
        }) = &mut self.trailing
        {
            while *next_index < *end {
                let j = *next_index;
                // Advance past this cell; if it is wide, also skip its own trailing column
                // so the main loop does not emit a spurious EMPTY write over it.
                let cell_width = self.next[j].cell_width().max(1) as usize;
                *next_index += cell_width;
                *end = (*end).max(*next_index).min(len);

                if !is_skip(&self.next[j])
                    && (*force || self.prev[j].symbol() != self.next[j].symbol())
                {
                    let (tx, ty) = self.pos_of(j);
                    return Some((tx, ty, &self.next[j]));
                }
            }

            // Resume after the wide glyph, repainting its deferred leading cell first.
            self.pos = *end;
            let deferred_leading_cell = deferred_leading_cell.take();
            self.trailing = None;
            if let Some(i) = deferred_leading_cell {
                let (x, y) = self.pos_of(i);
                return Some((x, y, &self.next[i]));
            }
        }
        while self.pos < len {
            let i = self.pos;
            self.pos += 1;

            let current = &self.next[i];
            let previous = &self.prev[i];

            match current.diff_option {
                CellDiffOption::Skip => {}
                _ if is_skip(current) => {}

                CellDiffOption::ForcedWidth(width) => {
                    self.pos = self
                        .pos
                        .saturating_add(width.get().saturating_sub(1) as usize);
                    if current != previous {
                        let (x, y) = self.pos_of(i);
                        return Some((x, y, &self.next[i]));
                    }
                }
                CellDiffOption::None | CellDiffOption::AlwaysUpdate => {
                    let cell_width = current.cell_width() as usize;
                    if matches!(current.diff_option, CellDiffOption::None) && current == previous {
                        // Equal cells still need to account for multi-width skip.
                        self.pos += cell_width.saturating_sub(1);
                        continue;
                    }

                    let previous_width = previous.cell_width() as usize;
                    let previous_style_is_visible_on_blank = previous.bg != Color::Reset
                        || previous.modifier.intersects(VISIBLE_ON_BLANK);

                    // Clear stale styles from an unchanged wide glyph's trailing cells before
                    // repainting it.
                    if cell_width > 1
                        && current.symbol() == previous.symbol()
                        && current.style() != previous.style()
                        && previous_style_is_visible_on_blank
                    {
                        self.trailing = Some(TrailingState {
                            next_index: i + 1,
                            end: (i + cell_width).min(len),
                            force: true,
                            deferred_leading_cell: Some(i),
                        });
                        return self.next();
                    }

                    // Clear reserved columns first so they cannot overwrite the uncertain-width
                    // glyph.
                    let uncertain_width = cell_width > 1 && has_uncertain_width(current.symbol());

                    if uncertain_width {
                        let trailing_end = (i + cell_width).min(len);
                        self.trailing = Some(TrailingState {
                            next_index: i + 1,
                            end: trailing_end,
                            force: false,
                            deferred_leading_cell: Some(i),
                        });
                        // Recurses once and returns the deferred leading cell.
                        return self.next();
                    } else if cell_width > 1 {
                        self.pos += cell_width.saturating_sub(1);
                    } else if previous_width > cell_width && previous_style_is_visible_on_blank {
                        // The previous wide character's style is visible on blank cells, so the
                        // terminal may still show it on the trailing columns even after the
                        // character is replaced. Force-emit every cell in the trailing range to
                        // refresh the terminal regardless of whether the buffer content changed.
                        // This cell is narrow, so those columns stay adjacent and need no move.
                        self.trailing = Some(TrailingState {
                            next_index: i + 1,
                            end: (i + previous_width).min(len),
                            force: true,
                            deferred_leading_cell: None,
                        });
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

/// Returns `true` if terminals disagree on how many columns `symbol` occupies.
///
/// Emoji presentation sequences (those containing VS16, `U+FE0F`) are the known case:
/// `unicode-width` and most modern terminals treat them as two columns, but some terminals draw
/// them in one. The diff cannot assume either, so it must emit updates that are correct under
/// both.
fn has_uncertain_width(symbol: &str) -> bool {
    symbol.chars().any(|c| c == '\u{FE0F}')
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
    use crate::style::Color;

    #[test]
    fn empty_buffers_yield_no_diffs() {
        let rect = Rect::new(0, 0, 5, 1);
        let buf = Buffer::empty(rect);
        let diff: Vec<_> = BufferDiff::new(&buf, &buf).collect();
        assert_eq!(diff, Vec::new());
    }

    #[test]
    fn identical_buffers_yield_no_diffs() {
        let buf = Buffer::with_lines(["hello"]);
        let diff: Vec<_> = BufferDiff::new(&buf, &buf).collect();
        assert_eq!(diff, Vec::new());
    }

    /// Regression for <https://github.com/ratatui/ratatui/issues/2685>: an image protocol cell
    /// keeps its whole escape sequence in one symbol, which measures hundreds of columns wide
    /// while covering a single column. Advancing the cursor by that width dropped every later
    /// cell in the buffer, so the frame kept whatever was on screen before.
    #[test]
    fn long_symbol_does_not_hide_later_cells() {
        let area = Rect::new(0, 0, 20, 5);
        let prev = Buffer::filled(area, Cell::new("S"));
        let mut next = Buffer::filled(area, Cell::new("L"));
        next[(10, 0)].set_symbol(&"\u{1b}[s".repeat(400));

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();
        assert_eq!(diff.len(), area.area() as usize);
    }

    /// The same cell, unchanged between the two frames. It still must not hide the cells after
    /// it, which is how the regression showed up once the image had been drawn once.
    #[test]
    fn unchanged_long_symbol_does_not_hide_later_cells() {
        let area = Rect::new(0, 0, 20, 5);
        let mut prev = Buffer::filled(area, Cell::new("S"));
        let mut next = Buffer::filled(area, Cell::new("L"));
        let payload = "\u{1b}[s".repeat(400);
        prev[(10, 0)].set_symbol(&payload);
        next[(10, 0)].set_symbol(&payload);

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();
        assert_eq!(diff.len(), area.area() as usize - 1);
    }

    /// A real wide glyph still hides its trailing cell: writing to the column covered by the
    /// right half of the glyph would overwrite the glyph itself.
    #[test]
    fn identical_wide_glyph_hides_trailing_cell_changes() {
        use crate::style::Style;

        let area = Rect::new(0, 0, 3, 1);
        let mut prev = Buffer::empty(area);
        prev.set_string(0, 0, "＋", Style::default());

        let mut next = prev.clone();
        // Column 1 is occupied by the right half of `＋`.
        next[(1, 0)].set_bg(Color::Red);

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();

        assert!(
            diff.is_empty(),
            "the hidden trailing cell of an unchanged wide glyph must not be emitted; got {diff:?}"
        );
    }

    /// Regression for <https://github.com/ratatui/ratatui/issues/2652>: removing a style that is
    /// visible on blank cells from an unchanged wide glyph must clear its trailing column before
    /// repainting the glyph. Clearing it afterwards can erase the glyph on some terminals.
    #[test]
    fn wide_glyph_style_change_clears_trailing_cell_before_repaint() {
        use crate::style::Style;

        let area = Rect::new(0, 0, 3, 1);
        for glyph in ["❤️", "😀"] {
            let mut prev = Buffer::empty(area);
            prev.set_string(0, 0, glyph, Style::new().bg(Color::Blue));

            let mut next = Buffer::empty(area);
            next.set_string(0, 0, glyph, Style::default());

            let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();

            assert_eq!(diff.len(), 2, "trailing cell and glyph must be redrawn");
            assert_eq!((diff[0].0, diff[0].1, diff[0].2.symbol()), (1, 0, " "));
            assert_eq!((diff[1].0, diff[1].1, diff[1].2.symbol()), (0, 0, glyph));
        }
    }

    #[test]
    fn wide_glyph_style_change_resumes_after_repaint() {
        use crate::style::Style;

        let area = Rect::new(0, 0, 3, 1);
        let mut prev = Buffer::empty(area);
        prev.set_string(0, 0, "한a", Style::new().bg(Color::Blue));

        let mut next = Buffer::empty(area);
        next.set_string(0, 0, "한b", Style::new().bg(Color::Red));

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();

        assert_eq!(diff.len(), 3);
        assert_eq!((diff[0].0, diff[0].1, diff[0].2.symbol()), (1, 0, " "));
        assert_eq!((diff[1].0, diff[1].1, diff[1].2.symbol()), (0, 0, "한"));
        assert_eq!((diff[2].0, diff[2].1, diff[2].2.symbol()), (2, 0, "b"));
        assert_eq!(diff[1].2.bg, Color::Red);
    }

    #[test]
    fn wide_glyph_foreground_change_does_not_clear_trailing_cell() {
        use crate::style::Style;

        let area = Rect::new(0, 0, 3, 1);
        let mut prev = Buffer::empty(area);
        prev.set_string(0, 0, "한", Style::new().fg(Color::Blue));

        let mut next = Buffer::empty(area);
        next.set_string(0, 0, "한", Style::new().fg(Color::Red));

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();

        assert_eq!(diff.len(), 1);
        assert_eq!((diff[0].0, diff[0].1, diff[0].2.symbol()), (0, 0, "한"));
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

    /// Regression test for the "uncovered trailing cell" bug.
    ///
    /// When a wide char with a non-default style (e.g. bg=Blue) is replaced by a narrow char,
    /// its trailing cell was `reset()` in the buffer and therefore has default style in both
    /// `prev` and `next`. The diff would skip it because symbol and style are identical.
    ///
    /// But the *terminal* rendered the trailing cell with the wide char's style (that's how
    /// terminals work: printing "，" with bg=Blue paints both columns blue). So an explicit
    /// update must be emitted even though the buffer cell looks unchanged.
    ///
    /// With "你好，世界！" (Blue) replaced by "Hello" (default), "Hello" covers columns 0–4.
    /// Columns 5, 7, 9, 11 are trailing cells of "，", "世", "界", "！" respectively — all
    /// EMPTY in both buffers, but rendered blue on the terminal.
    #[test]
    fn uncovered_trailing_cells_emitted_when_wide_char_style_changes() {
        use crate::style::{Color, Style};

        // Width 12 fits exactly "你好，世界！" (6 wide chars × 2 cols each).
        let rect = Rect::new(0, 0, 12, 1);
        let mut prev = Buffer::empty(rect);
        prev.set_string(0, 0, "你好，世界！", Style::new().bg(Color::Blue));

        let mut next = Buffer::empty(rect);
        next.set_string(0, 0, "Hello", Style::default());

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();

        // Columns 5, 7, 9, 11 are the trailing cells of "，", "世", "界", "！".
        // Both prev and next hold EMPTY there (same symbol, same default style), but the
        // previous cell did have a different style, so the terminal painted them blue.
        // Without an explicit update the blue background is never cleared, producing
        // "Hello ░ ░ ░ ░".
        for x in [5u16, 7, 9, 11] {
            assert!(
                diff.iter().any(|(dx, dy, _)| *dx == x && *dy == 0),
                "expected update for trailing cell at x={x} (blue bg not cleared otherwise)"
            );
        }
    }

    /// Regression for a multi-step scenario that produces "Hello  ░" artifacts.
    ///
    /// "你好，世界！"(Blue) → "喵呜www"(Blue): "世"(cols 6-7) is replaced by "w"(Blue). Both
    /// have the same background, so a guard like `previous.bg != current.bg` would not fire —
    /// but the terminal painted col 7 (the trailing cell of "世") blue when it rendered the
    /// wide char. Col 7 is EMPTY in both buffers, so without an explicit update the blue
    /// lingers. A subsequent "Hello" draw then shows "Hello  ░" because "Hello" covers cols
    /// 0-4, cols 5-6 are cleared (they held "ww"), but col 7 is never cleared.
    #[test]
    fn uncovered_trailing_cells_emitted_when_wide_chars_partially_replaced() {
        use crate::style::{Color, Style};

        let rect = Rect::new(0, 0, 12, 1);
        let mut prev = Buffer::empty(rect);
        prev.set_string(0, 0, "你好，世界！", Style::new().bg(Color::Blue));

        let mut next = Buffer::empty(rect);
        next.set_string(0, 0, "喵呜www", Style::new().bg(Color::Blue));

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();

        // Col 7 is the trailing cell of "世"(6-7). "世" and its replacement "w" both have
        // Blue bg, so a `previous.bg != current.bg` guard would not trigger — but the
        // terminal painted it blue and it must be cleared.
        assert!(
            diff.iter().any(|(dx, dy, _)| *dx == 7 && *dy == 0),
            "expected update for trailing cell at x=7 (blue bg not cleared otherwise)"
        );
    }

    #[test]
    fn no_force_update_when_wide_char_has_default_style() {
        use crate::style::Style;

        let rect = Rect::new(0, 0, 12, 1);
        let mut prev = Buffer::empty(rect);
        prev.set_string(0, 0, "你好，世界！", Style::default());

        let mut next = Buffer::empty(rect);
        next.set_string(0, 0, "Hello", Style::default());

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();

        // Columns 5..=11 include the trailing cells of "，", "世", "界", "！".
        // Both prev and next hold EMPTY there, and the previous wide chars had default style
        // (bg=Reset, no visible modifier), so `force=true` is never set and trailing cells
        // are not emitted.
        for x in [5u16, 7, 9, 11] {
            assert!(
                !diff.iter().any(|(dx, dy, _)| *dx == x && *dy == 0),
                "expected no update for trailing cell at x={x}"
            );
        }
    }

    #[test]
    fn shrinking_wide_glyph_clears_trailing_cell() {
        // Regression for https://github.com/ratatui/ratatui/issues/2585 (introduced by #1605):
        // the cell-diff-options refactor dropped the classic `invalidated` counter, so when a
        // multi-width glyph (here the full-width plus ＋, U+FF0B) is replaced by narrower content,
        // the trailing cell it physically painted over was left un-cleared. Because that trailing
        // cell is blank in both buffers (`current == previous`), only the previous glyph's width
        // can force the redraw — exactly what `invalidated` tracks.

        let rect = Rect::new(0, 0, 2, 1);

        let mut prev = Buffer::empty(rect);
        prev.set_string(0, 0, "＋", crate::style::Style::new());
        // Force some style that would leave artifacts (anything except foreground color).
        prev.cell_mut((0, 0)).unwrap().set_bg(Color::Red);

        assert_eq!(prev.content[0].symbol(), "＋");

        // The trailing cell is reset to a blank space when the wide glyph is written.
        assert_eq!(prev.content[1].symbol(), " ");

        // Next frame clears the glyph: both cells are blank.
        let next = Buffer::empty(rect);
        assert_eq!(next.content[1], prev.content[1]); // trailing cell is unchanged

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();

        assert_eq!(diff.len(), 2, "both columns must be redrawn, got {diff:?}");
        assert_eq!((diff[0].0, diff[0].1), (0, 0));
        assert_eq!(diff[0].2.symbol(), " ");

        // The trailing cell (1,0) must be emitted even though it is identical in both buffers,
        // otherwise the right half of the old glyph (and its background) lingers on screen.
        assert_eq!((diff[1].0, diff[1].1), (1, 0));
        assert_eq!(diff[1].2.symbol(), " ");
    }

    #[test]
    fn shrinking_wide_glyph_clears_trailing_background() {
        use crate::style::{Color, Style};

        // Same regression, framed as the reported symptom: a styled background painted across a
        // wide glyph must be cleared from the trailing cell when the glyph shrinks away.

        let rect = Rect::new(0, 0, 2, 1);

        let mut prev = Buffer::empty(rect);
        prev.set_string(0, 0, "＋", Style::new().bg(Color::Blue));

        let next = Buffer::empty(rect); // blank, default background

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();

        assert!(
            diff.iter().any(|(x, y, _)| *x == 1 && *y == 0),
            "trailing cell (1,0) must be redrawn to clear the leftover background, got {diff:?}"
        );
    }

    /// Regression: a non-blank cell inside the trailing range whose style changes between frames
    /// must still be emitted.
    ///
    /// The trailing loop replaces the main loop for indices in `[i+1, i+prev_width)`. When
    /// `force=true` every non-skip cell in the range is emitted unconditionally — not because the
    /// style change is specifically detected, but because `force=true` short-circuits the check
    /// entirely. This ensures style-only changes on trailing cells are never silently dropped.
    ///
    /// Scenario: widget A draws "你"(Blue) at col 0; widget B then writes "X"(Red) at col 1
    /// (overriding the trailing reset). Next frame: widget A writes "a"(default), widget B writes
    /// "X"(Green). Symbol at col 1 is unchanged, but fg changed Red→Green.
    /// `force=true` fires because prev "你" had bg=Blue (visible on blank), so col 1 is emitted
    /// regardless.
    #[test]
    fn trailing_range_cell_style_change_is_emitted() {
        use crate::style::{Color, Style};

        let rect = Rect::new(0, 0, 3, 1);
        let mut prev = Buffer::empty(rect);
        prev.set_string(0, 0, "你", Style::new().bg(Color::Blue));
        // Widget B overrides the trailing reset at col 1.
        prev.content[1]
            .set_symbol("X")
            .set_fg(Color::Red)
            .set_bg(Color::Reset);

        let mut next = Buffer::empty(rect);
        next.set_string(0, 0, "a", Style::default());
        // Same symbol, different fg — style-only change.
        next.content[1]
            .set_symbol("X")
            .set_fg(Color::Green)
            .set_bg(Color::Reset);

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();

        assert!(
            diff.iter().any(|(x, y, _)| *x == 1 && *y == 0),
            "col 1 style change (Red→Green) must be emitted even though symbol is unchanged; got {diff:?}"
        );
    }

    /// Regression: when the trailing loop emits a wide char, its own trailing column must be
    /// skipped so the main loop does not write EMPTY there and corrupt the display.
    ///
    /// Scenario: prev="你好"(Blue) → next="a好"(Blue) where 'a' replaces '你'.
    /// `force=true` is set (Blue bg), trailing range = [1, 2).
    /// At j=1 the trailing loop finds '好' (width 2) in next and emits it.
    /// Without the fix `next_index` advances to 2 = end; `self.pos` is set to 2.
    /// The main loop then sees next[2]=EMPTY vs prev[2]='好'(Blue) and emits
    /// EMPTY at col 2 — writing a space over the right half of the just-drawn '好'.
    #[test]
    fn trailing_range_wide_char_trailing_not_corrupted() {
        use crate::style::{Color, Style};

        let rect = Rect::new(0, 0, 4, 1);
        let mut prev = Buffer::empty(rect);
        prev.set_string(0, 0, "你好", Style::new().bg(Color::Blue));
        // prev: [你(Blue)][EMPTY][好(Blue)][EMPTY]

        let mut next = Buffer::empty(rect);
        next.set_string(0, 0, "a", Style::new().bg(Color::Blue));
        next.set_string(1, 0, "好", Style::new().bg(Color::Blue));
        // next: [a(Blue)][好(Blue)][EMPTY][EMPTY]

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();

        // col 2 is the trailing cell of '好' in next; writing EMPTY there would
        // overwrite the right half of '好' and corrupt the display.
        assert!(
            !diff.iter().any(|(x, y, _)| *x == 2 && *y == 0),
            "trailing cell of '好' must not receive a spurious EMPTY write; got {diff:?}"
        );
        // '好' itself must be emitted at col 1.
        assert!(
            diff.iter()
                .any(|(x, y, cell)| *x == 1 && *y == 0 && cell.symbol() == "好"),
            "'好' at col 1 must be emitted; got {diff:?}"
        );
    }

    /// Regression for <https://github.com/ratatui/ratatui/issues/2695> (introduced by #2587):
    /// the forced trailing range was not clamped to the buffer length, unlike the VS16 range
    /// above it. With a wide glyph carrying a visible-on-blank style in the *last* cell of the
    /// buffer, replaced by narrower content, the range extended one past the end and the
    /// iterator panicked with an index out of bounds while being consumed (i.e. inside
    /// `Terminal::flush`).
    #[test]
    fn wide_glyph_in_last_cell_does_not_overrun_buffer() {
        let rect = Rect::new(0, 0, 3, 1);
        let mut prev = Buffer::empty(rect);
        // set_string refuses to place a wide glyph in the last column, so write the cell
        // directly — the path third-party widgets and direct cell access take.
        prev.content[2].set_symbol("你").set_bg(Color::Red);
        let mut next = Buffer::empty(rect);
        next.content[2].set_symbol("a");

        let diff: Vec<_> = BufferDiff::new(&prev, &next).collect();

        assert_eq!(
            diff.len(),
            1,
            "only the replaced cell must be emitted, got {diff:?}"
        );
        assert_eq!((diff[0].0, diff[0].1), (2, 0));
        assert_eq!(diff[0].2.symbol(), "a");
    }

    #[test]
    #[should_panic(expected = "buffer areas must have the same x, y, and width")]
    fn mismatched_widths_panics() {
        let prev = Buffer::empty(Rect::new(0, 0, 5, 1));
        let next = Buffer::empty(Rect::new(0, 0, 10, 1));
        BufferDiff::new(&prev, &next);
    }
}

/// Replays diffs through a model backend with real cursor semantics, and checks the resulting
/// screen against what the buffer says the row should look like.
///
/// Backends skip a cursor move only when the previous symbol has a predictable terminal width.
/// VS16 clusters force a move because their actual cursor advance may differ from the buffer.
///
/// Terminals disagree on the width of a cluster carrying VS16 (`U+FE0F`), so the model is
/// parameterised over both observed behaviours; every scenario must pass under both.
#[cfg(test)]
mod terminal_replay {
    use alloc::string::{String, ToString};
    use alloc::vec;
    use alloc::vec::Vec;

    use rstest::rstest;

    use super::*;
    use crate::buffer::Buffer;
    use crate::style::Style;

    /// Marks a column painted over by the glyph starting to its left.
    const COVERED: &str = "";

    /// How a terminal renders a grapheme cluster carrying VS16 (`U+FE0F`).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum TerminalModel {
        /// VS16 clusters advance two columns (tmux, most modern terminals).
        Wide,
        /// VS16 clusters advance one column (the terminals
        /// <https://github.com/ratatui/ratatui/pull/2063> was filed against).
        Narrow,
    }

    const MODELS: [TerminalModel; 2] = [TerminalModel::Wide, TerminalModel::Narrow];

    impl TerminalModel {
        /// Columns the cursor advances when `symbol` is printed, given the width the buffer
        /// reserved for it.
        fn advance(self, symbol: &str, reserved: usize) -> usize {
            if self == Self::Narrow && reserved > 1 && has_uncertain_width(symbol) {
                1
            } else {
                reserved
            }
        }
    }

    /// A single row of terminal columns.
    #[derive(Debug)]
    struct Screen {
        cols: Vec<String>,
    }

    impl Screen {
        /// Prints `symbol` at `col`, advancing the cursor by `advance` columns.
        ///
        /// Overwriting either half of a multi-column glyph destroys the whole glyph, so its full
        /// footprint is blanked before the new symbol is written.
        fn print(&mut self, col: usize, symbol: &str, advance: usize) {
            let len = self.cols.len();
            if col >= len {
                return;
            }
            let end = (col + advance).min(len);

            let mut lo = col;
            while lo > 0 && self.cols[lo] == COVERED {
                lo -= 1;
            }
            let mut hi = end;
            while hi < len && self.cols[hi] == COVERED {
                hi += 1;
            }
            for c in lo..hi {
                self.cols[c] = " ".to_string();
            }

            self.cols[col] = symbol.to_string();
            for c in col + 1..end {
                self.cols[c] = COVERED.to_string();
            }
        }
    }

    /// The columns a buffer row occupies on screen under `model`.
    ///
    /// A glyph paints its own column plus every column it advances over; columns the buffer
    /// reserved but the terminal did not cover show the buffer cell's own content.
    fn expected_columns(model: TerminalModel, buf: &Buffer) -> Vec<String> {
        let width = buf.area.width as usize;
        let mut cols = vec![" ".to_string(); width];
        let mut i = 0;
        while i < width {
            let cell = &buf.content[i];
            let reserved = (cell.cell_width() as usize).max(1);
            let advance = model.advance(cell.symbol(), reserved);
            cols[i] = cell.symbol().to_string();
            for (c, col) in cols
                .iter_mut()
                .enumerate()
                .take((i + reserved).min(width))
                .skip(i + 1)
            {
                *col = if c < i + advance {
                    COVERED.to_string()
                } else {
                    buf.content[c].symbol().to_string()
                };
            }
            i += reserved;
        }
        cols
    }

    /// Replays `BufferDiff` the way a backend does and returns the resulting screen.
    ///
    /// The screen starts as `prev` rendered under `model`, so any column the diff never rewrites
    /// keeps the previous frame's content — a stale column surviving into the comparison *is* the
    /// artifact.
    fn replay(model: TerminalModel, prev: &Buffer, next: &Buffer) -> Vec<String> {
        assert_eq!(prev.area.height, 1, "harness models a single row");
        let mut screen = Screen {
            cols: expected_columns(model, prev),
        };
        let mut cursor = 0usize;
        let mut last: Option<(u16, usize)> = None;

        for (x, _y, cell) in BufferDiff::new(prev, next) {
            // Reposition unless the backend can predict the terminal's cursor position.
            let col = if matches!(last, Some((p, width)) if usize::from(p) + width == usize::from(x))
            {
                cursor
            } else {
                x as usize
            };
            let reserved = (cell.cell_width() as usize).max(1);
            let advance = model.advance(cell.symbol(), reserved);
            screen.print(col, cell.symbol(), advance);
            cursor = col + advance;
            let uncertain_width = reserved > 1 && has_uncertain_width(cell.symbol());
            last = (!uncertain_width).then_some((x, reserved));
        }
        screen.cols
    }

    #[track_caller]
    fn assert_no_artifacts(model: TerminalModel, prev: &Buffer, next: &Buffer) {
        let expected = expected_columns(model, next);
        let actual = replay(model, prev, next);
        assert_eq!(
            actual, expected,
            "{model:?} model: screen differs from buffer"
        );
    }

    #[track_caller]
    fn assert_no_artifacts_in_both_models(prev: &str, next: &str, width: u16) {
        for model in MODELS {
            assert_no_artifacts(model, &row(width, prev), &row(width, next));
        }
    }

    fn row(width: u16, content: &str) -> Buffer {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, 1));
        buf.set_string(0, 0, content, Style::default());
        buf
    }

    /// <https://github.com/ratatui/ratatui/issues/2357>: updating an ASCII row to VS16 emoji.
    #[rstest]
    #[case::heart("❤️❤️❤️❤️❤️❤️")]
    #[case::rocket("🚀️🚀️🚀️🚀️🚀️🚀️")]
    fn vs16_emoji_replacing_ascii_does_not_drift(#[case] next: &str) {
        assert_no_artifacts_in_both_models("int value8;;", next, 12);
    }

    /// Same as above with keycap sequences, which carry VS16 plus a combining enclosing keycap.
    #[test]
    fn vs16_keycaps_replacing_ascii_do_not_drift() {
        assert_no_artifacts_in_both_models("int value8;;", "1️⃣1️⃣1️⃣1️⃣1️⃣1️⃣", 12);
    }

    /// A VS16 row redrawn one column over: every glyph moves, so no update may inherit an offset.
    #[test]
    fn shifted_vs16_row_does_not_drift() {
        assert_no_artifacts_in_both_models("❤️❤️❤️❤️❤️", " ❤️❤️❤️❤️❤️", 12);
    }

    /// <https://github.com/ratatui/ratatui/pull/2063>: a terminal that draws a VS16 cluster in a
    /// single column leaves the buffer's reserved column showing the old content unless the diff
    /// writes it. Regression guard: deleting the VS16 special case to fix #2357 breaks this.
    #[test]
    fn vs16_emoji_replacing_narrower_content_leaves_no_stale_column() {
        assert_no_artifacts_in_both_models("ab", "❤️", 2);
        assert_no_artifacts_in_both_models("abcd", "❤️cd", 4);
        assert_no_artifacts_in_both_models("abcd", "ab❤️", 4);
    }

    /// Mixed rows: narrow, certain-width (`漢`, `😀`) and uncertain-width clusters interleaved.
    ///
    /// Also pins that certain-width clusters keep skipping their own reserved column: they are
    /// not routed through the clear-first path, and must not drift either.
    #[test]
    fn mixed_width_rows_do_not_drift() {
        assert_no_artifacts_in_both_models("int value8;;", "a❤️b漢c😀d", 12);
        assert_no_artifacts_in_both_models("a❤️b漢c😀d", "int value8;;", 12);
        assert_no_artifacts_in_both_models("a❤️b漢c😀d", "漢a❤️b😀c", 12);
    }
}
