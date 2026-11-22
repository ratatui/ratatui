use core::cmp::Ordering;

use crate::backend::Backend;
use crate::layout::{Position, Rect, Size};
use crate::terminal::{Terminal, Viewport};

impl<B: Backend> Terminal<B> {
    /// Sets the height of an inline viewport and resizes it accordingly.
    ///
    /// This method only works with inline viewports. For other viewport types, it has no effect.
    /// The viewport will be resized to the new height, and the buffers will be cleared and
    /// reallocated to match the new size.
    ///
    /// Unlike [`Terminal::resize`], which handles changes to the terminal's dimensions, this
    /// changes the inline viewport's requested height. It keeps the current top row when possible;
    /// when growing would extend past the bottom of the terminal, it scrolls the screen to make
    /// room. Shrinking keeps the current top row. The visible viewport is capped at the terminal
    /// height, but the requested height is retained if the terminal is later resized larger.
    ///
    /// # Arguments
    ///
    /// * `new_height` - The new height for the inline viewport in lines
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ratatui::{Terminal, TerminalOptions, Viewport};
    ///
    /// let mut terminal = Terminal::with_options(backend, TerminalOptions {
    ///     viewport: Viewport::Inline(8),
    /// })?;
    ///
    /// // Later, resize the viewport to 12 lines
    /// terminal.set_viewport_height(12)?;
    /// ```
    pub fn set_viewport_height(&mut self, new_height: u16) -> Result<(), B::Error> {
        let Viewport::Inline(height) = &mut self.viewport else {
            return Ok(());
        };
        if *height == new_height {
            return Ok(());
        }

        *height = new_height;
        let new_height = new_height.min(self.last_known_area.height);
        let old_height = self.viewport_area.height;
        self.clear()?;

        let new_y = match new_height.cmp(&old_height) {
            Ordering::Greater => {
                let overflow = self
                    .viewport_area
                    .y
                    .saturating_add(new_height)
                    .saturating_sub(self.last_known_area.height);
                if overflow > 0 {
                    self.scroll_up(overflow)?;
                    self.viewport_area.y.saturating_sub(overflow)
                } else {
                    self.viewport_area.y
                }
            }
            _ => self.viewport_area.y,
        };

        self.set_viewport_area(Rect {
            height: new_height,
            y: new_y,
            ..self.viewport_area
        });
        self.clear()
    }

    /// Scroll the whole screen up by the given number of lines.
    ///
    /// This is used when growing an inline viewport. It scrolls by moving the cursor to the last
    /// row and calling [`Backend::append_lines`].
    fn scroll_up(&mut self, lines_to_scroll: u16) -> Result<(), B::Error> {
        if lines_to_scroll > 0 {
            self.set_cursor_position(Position::new(
                0,
                self.last_known_area.height.saturating_sub(1),
            ))?;
            self.backend.append_lines(lines_to_scroll)?;
        }
        Ok(())
    }
}

/// Compute the on-screen area for an inline viewport.
///
/// This helper is used by [`Terminal::with_options`] (initialization) and [`Terminal::resize`]
/// (after a terminal resize) to translate `Viewport::Inline(height)` into a concrete [`Rect`].
///
/// This returns the computed viewport area and the cursor position observed at the start of the
/// call.
///
/// Inline viewports always start at column 0, span the full terminal width, and are anchored to the
/// backend cursor row at the time of the call. The requested height is clamped to the current
/// terminal height.
///
/// Ratatui reserves vertical space for the requested height by calling [`Backend::append_lines`].
/// If the cursor is close enough to the bottom that appending would run past the last row,
/// terminals scroll; in that case we shift the computed `y` upward by the number of rows scrolled
/// so the viewport remains fully visible.
///
/// `offset_in_previous_viewport` is used by [`Terminal::resize`] to keep the cursor at the same
/// relative row within the viewport across resizes.
///
/// Related viewport code lives in:
///
/// - [`Terminal::with_options`] (selects the viewport and computes the initial area)
/// - [`Terminal::autoresize`] (detects backend size changes during [`Terminal::draw`] /
///   [`Terminal::try_draw`])
/// - [`Terminal::resize`] (recomputes the viewport and clears before the next draw)
pub(crate) fn compute_inline_size<B: Backend>(
    backend: &mut B,
    height: u16,
    size: Size,
    offset_in_previous_viewport: u16,
    cursor_position: Option<Position>,
) -> Result<(Rect, Position), B::Error> {
    let pos = match cursor_position {
        Some(position) => position,
        None => backend.get_cursor_position()?,
    };
    let mut row = pos.y;

    let max_height = size.height.min(height);

    let lines_after_cursor = height
        .saturating_sub(offset_in_previous_viewport)
        .saturating_sub(1);

    backend.append_lines(lines_after_cursor)?;

    let available_lines = size.height.saturating_sub(row).saturating_sub(1);
    let missing_lines = lines_after_cursor.saturating_sub(available_lines);
    if missing_lines > 0 {
        row = row.saturating_sub(missing_lines);
    }
    row = row.saturating_sub(offset_in_previous_viewport);

    Ok((
        Rect {
            x: 0,
            y: row,
            width: size.width,
            height: max_height,
        },
        pos,
    ))
}

#[cfg(test)]
mod tests {
    use crate::backend::{Backend, TestBackend};
    use crate::layout::{Position, Rect, Size};
    use crate::terminal::inline::compute_inline_size;
    use crate::terminal::{Terminal, TerminalOptions, Viewport};

    #[test]
    fn set_viewport_height_does_nothing_for_non_inline_viewports() {
        let backend = TestBackend::new(10, 10);
        let area = Rect::new(1, 2, 3, 4);
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Fixed(area),
            },
        )
        .unwrap();

        terminal.set_viewport_height(8).unwrap();

        assert_eq!(terminal.viewport, Viewport::Fixed(area));
        assert_eq!(terminal.viewport_area, area);
    }

    #[test]
    fn set_viewport_height_resizes_inline_viewport_and_buffers() {
        let mut backend = TestBackend::new(10, 10);
        backend.set_cursor_position(Position::new(0, 2)).unwrap();
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(3),
            },
        )
        .unwrap();

        terminal.set_viewport_height(5).unwrap();

        let expected_area = Rect::new(0, 2, 10, 5);
        assert_eq!(terminal.viewport, Viewport::Inline(5));
        assert_eq!(terminal.viewport_area, expected_area);
        assert_eq!(terminal.buffers[terminal.current].area, expected_area);
        assert_eq!(terminal.buffers[1 - terminal.current].area, expected_area);
    }

    #[test]
    fn set_viewport_height_clears_the_old_viewport_before_redrawing() {
        let mut backend = TestBackend::with_lines([
            "before    ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
        ]);
        backend.set_cursor_position(Position::new(0, 1)).unwrap();
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(3),
            },
        )
        .unwrap();

        terminal
            .draw(|frame| frame.render_widget("old", frame.area()))
            .unwrap();
        terminal.backend().assert_buffer_lines([
            "before    ",
            "old       ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
        ]);

        terminal.set_viewport_height(5).unwrap();

        terminal.backend().assert_buffer_lines([
            "before    ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
        ]);
        terminal
            .draw(|frame| frame.render_widget("new", frame.area()))
            .unwrap();
        terminal.backend().assert_buffer_lines([
            "before    ",
            "new       ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
        ]);
    }

    #[test]
    fn set_viewport_height_scrolls_to_make_room() {
        let mut backend = TestBackend::new(10, 10);
        backend.set_cursor_position(Position::new(0, 7)).unwrap();
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(3),
            },
        )
        .unwrap();

        terminal.set_viewport_height(5).unwrap();

        assert_eq!(terminal.viewport, Viewport::Inline(5));
        assert_eq!(terminal.viewport_area, Rect::new(0, 5, 10, 5));
    }

    #[test]
    fn set_viewport_height_caps_area_to_terminal_height() {
        let mut backend = TestBackend::new(10, 5);
        backend.set_cursor_position(Position::new(0, 2)).unwrap();
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(3),
            },
        )
        .unwrap();

        terminal.set_viewport_height(20).unwrap();

        assert_eq!(terminal.viewport, Viewport::Inline(20));
        assert_eq!(terminal.viewport_area, Rect::new(0, 0, 10, 5));
        assert_eq!(
            terminal.buffers[terminal.current].area,
            terminal.viewport_area
        );
        assert_eq!(
            terminal.buffers[1 - terminal.current].area,
            terminal.viewport_area
        );
    }

    #[test]
    fn set_viewport_height_keeps_origin_when_shrinking() {
        let mut backend = TestBackend::new(10, 10);
        backend.set_cursor_position(Position::new(0, 2)).unwrap();
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(5),
            },
        )
        .unwrap();

        terminal.set_viewport_height(3).unwrap();

        assert_eq!(terminal.viewport, Viewport::Inline(3));
        assert_eq!(terminal.viewport_area, Rect::new(0, 2, 10, 3));
    }

    #[test]
    fn compute_inline_size_uses_cursor_offset_when_space_available() {
        // Diagram (terminal height = 10, requested viewport height = 4):
        //
        // Cursor at y=6, previous cursor offset within viewport = 1.
        //
        // Before (conceptually):
        //   0
        //   1
        //   2
        //   3
        //   4
        //   5  <- viewport top (expected)
        //   6  <- cursor row (observed_pos.y)
        //   7
        //   8
        //   9
        //
        // After: viewport top y = 5 (6 - 1), height = 4 => rows 5..9 (exclusive).
        let mut backend = TestBackend::new(10, 10);
        backend
            .set_cursor_position(Position { x: 0, y: 6 })
            .unwrap();

        let (area, observed_pos) =
            compute_inline_size(&mut backend, 4, Size::new(10, 10), 1, None).unwrap();

        assert_eq!(observed_pos, Position { x: 0, y: 6 });
        assert_eq!(area, Rect::new(0, 5, 10, 4));
    }

    #[test]
    fn compute_inline_size_saturates_when_offset_exceeds_cursor_row() {
        // Diagram (terminal height = 10, requested viewport height = 4):
        //
        // Cursor at y=0, previous cursor offset within viewport = 5 (nonsensical but possible if
        // callers pass a stale/oversized offset).
        //
        // We saturate so the computed viewport top cannot go negative:
        //   top = cursor_y.saturating_sub(offset) = 0.saturating_sub(5) = 0
        //
        // Expected viewport area:
        //   y=0..4 (fully pinned to the top)
        let mut backend = TestBackend::new(10, 10);
        backend
            .set_cursor_position(Position { x: 0, y: 0 })
            .unwrap();

        let (area, _observed_pos) =
            compute_inline_size(&mut backend, 4, Size::new(10, 10), 5, None).unwrap();

        assert_eq!(area, Rect::new(0, 0, 10, 4));
    }
}
