use crate::backend::Backend;
use crate::buffer::{Buffer, Cell};
use crate::layout::{Position, Rect, Size};
use crate::terminal::{Terminal, Viewport};

impl<B: Backend> Terminal<B> {
    /// Insert some content before the current inline viewport. This has no effect when the
    /// viewport is not inline.
    ///
    /// This is intended for inline UIs that want to print output (e.g. logs or status messages)
    /// above the UI without breaking it. See [`Viewport::Inline`] for how inline viewports are
    /// anchored.
    ///
    /// The `draw_fn` closure will be called to draw into a writable `Buffer` that is `height`
    /// lines tall. The content of that `Buffer` will then be inserted before the viewport.
    ///
    /// When Ratatui is built with the `scrolling-regions` feature, this can be done without
    /// clearing and redrawing the viewport. Without `scrolling-regions`, Ratatui falls back to a
    /// more portable approach and clears the viewport so the next [`Terminal::draw`] repaints it.
    ///
    /// If the viewport isn't yet at the bottom of the screen, inserted lines will push it towards
    /// the bottom. Once the viewport is at the bottom of the screen, inserted lines will scroll
    /// the area of the screen above the viewport upwards.
    ///
    /// Before:
    /// ```text
    /// +---------------------+
    /// | pre-existing line 1 |
    /// | pre-existing line 2 |
    /// +---------------------+
    /// |       viewport      |
    /// +---------------------+
    /// |                     |
    /// |                     |
    /// +---------------------+
    /// ```
    ///
    /// After inserting 2 lines:
    /// ```text
    /// +---------------------+
    /// | pre-existing line 1 |
    /// | pre-existing line 2 |
    /// |   inserted line 1   |
    /// |   inserted line 2   |
    /// +---------------------+
    /// |       viewport      |
    /// +---------------------+
    /// +---------------------+
    /// ```
    ///
    /// After inserting 2 more lines:
    /// ```text
    /// +---------------------+
    /// | pre-existing line 2 |
    /// |   inserted line 1   |
    /// |   inserted line 2   |
    /// |   inserted line 3   |
    /// |   inserted line 4   |
    /// +---------------------+
    /// |       viewport      |
    /// +---------------------+
    /// ```
    ///
    /// If more lines are inserted than there is space on the screen, then the top lines will go
    /// directly into the terminal's scrollback buffer. At the limit, if the viewport takes up the
    /// whole screen, all lines will be inserted directly into the scrollback buffer.
    ///
    /// # Examples
    ///
    /// ## Insert a single line before the current viewport
    ///
    /// ```rust,no_run
    /// # mod ratatui {
    /// #     pub use ratatui_core::backend;
    /// #     pub use ratatui_core::layout;
    /// #     pub use ratatui_core::style;
    /// #     pub use ratatui_core::terminal::{Terminal, TerminalOptions, Viewport};
    /// #     pub use ratatui_core::text;
    /// #     pub use ratatui_core::widgets;
    /// # }
    /// use ratatui::backend::{Backend, TestBackend};
    /// use ratatui::layout::Position;
    /// use ratatui::style::{Color, Style};
    /// use ratatui::text::{Line, Span};
    /// use ratatui::widgets::Widget;
    /// use ratatui::{Terminal, TerminalOptions, Viewport};
    ///
    /// let mut backend = TestBackend::new(10, 10);
    /// // Simulate existing output above the inline UI.
    /// backend.set_cursor_position(Position::new(0, 3))?;
    /// let mut terminal = Terminal::with_options(
    ///     backend,
    ///     TerminalOptions {
    ///         viewport: Viewport::Inline(4),
    ///     },
    /// )?;
    ///
    /// terminal.insert_before(1, |buf| {
    ///     Line::from(vec![
    ///         Span::raw("This line will be added "),
    ///         Span::styled("before", Style::default().fg(Color::Blue)),
    ///         Span::raw(" the current viewport"),
    ///     ])
    ///     .render(buf.area, buf);
    /// })?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn insert_before<F>(&mut self, height: u16, draw_fn: F) -> Result<(), B::Error>
    where
        F: FnOnce(&mut Buffer),
    {
        match self.viewport {
            #[cfg(feature = "scrolling-regions")]
            Viewport::Inline(_) => self.insert_before_scrolling_regions(height, draw_fn),
            #[cfg(not(feature = "scrolling-regions"))]
            Viewport::Inline(_) => self.insert_before_no_scrolling_regions(height, draw_fn),
            _ => Ok(()),
        }
    }

    /// Implement `Self::insert_before` using standard backend capabilities.
    ///
    /// This is the fallback implementation when the `scrolling-regions` feature is disabled. It
    /// renders the inserted lines into a temporary [`Buffer`], then draws them directly to the
    /// backend in chunks, scrolling the terminal as needed.
    ///
    /// See [`Terminal::insert_before`] for the public API contract.
    #[cfg(not(feature = "scrolling-regions"))]
    fn insert_before_no_scrolling_regions(
        &mut self,
        height: u16,
        draw_fn: impl FnOnce(&mut Buffer),
    ) -> Result<(), B::Error> {
        let area = Rect {
            x: 0,
            y: 0,
            width: self.viewport_area.width,
            height,
        };
        let mut buffer = Buffer::empty(area);
        draw_fn(&mut buffer);
        let mut buffer = buffer.content.as_slice();

        // Use i32 variables so we don't have worry about overflowed u16s when adding, or about
        // negative results when subtracting.
        let mut drawn_height: i32 = self.viewport_area.top().into();
        let mut buffer_height: i32 = height.into();
        let viewport_height: i32 = self.viewport_area.height.into();
        let screen_height: i32 = self.last_known_area.height.into();

        // The algorithm here is to loop, drawing large chunks of text (up to a screen-full at a
        // time), until the remainder of the buffer plus the viewport fits on the screen. We choose
        // this loop condition because it guarantees that we can write the remainder of the buffer
        // with just one call to Self::draw_lines().
        while buffer_height + viewport_height > screen_height {
            // We will draw as much of the buffer as possible on this iteration in order to make
            // forward progress. So we have:
            //
            //     to_draw = min(buffer_height, screen_height)
            //
            // We may need to scroll the screen up to make room to draw. We choose the minimal
            // possible scroll amount so we don't end up with the viewport sitting in the middle of
            // the screen when this function is done. The amount to scroll by is:
            //
            //     scroll_up = max(0, drawn_height + to_draw - screen_height)
            //
            // We want `scroll_up` to be enough so that, after drawing, we have used the whole
            // screen (drawn_height - scroll_up + to_draw = screen_height). However, there might
            // already be enough room on the screen to draw without scrolling (drawn_height +
            // to_draw <= screen_height). In this case, we just don't scroll at all.
            let to_draw = buffer_height.min(screen_height);
            let scroll_up = 0.max(drawn_height + to_draw - screen_height);
            self.scroll_up(scroll_up as u16)?;
            buffer = self.draw_lines((drawn_height - scroll_up) as u16, to_draw as u16, buffer)?;
            drawn_height += to_draw - scroll_up;
            buffer_height -= to_draw;
        }

        // There is now enough room on the screen for the remaining buffer plus the viewport,
        // though we may still need to scroll up some of the existing text first. It's possible
        // that by this point we've drained the buffer, but we may still need to scroll up to make
        // room for the viewport.
        //
        // We want to scroll up the exact amount that will leave us completely filling the screen.
        // However, it's possible that the viewport didn't start on the bottom of the screen and
        // the added lines weren't enough to push it all the way to the bottom. We deal with this
        // case by just ensuring that our scroll amount is non-negative.
        //
        // We want:
        //   screen_height = drawn_height - scroll_up + buffer_height + viewport_height
        // Or, equivalently:
        //   scroll_up = drawn_height + buffer_height + viewport_height - screen_height
        let scroll_up = 0.max(drawn_height + buffer_height + viewport_height - screen_height);
        self.scroll_up(scroll_up as u16)?;
        self.draw_lines(
            (drawn_height - scroll_up) as u16,
            buffer_height as u16,
            buffer,
        )?;
        drawn_height += buffer_height - scroll_up;

        self.set_viewport_area(Rect {
            y: drawn_height as u16,
            ..self.viewport_area
        });

        // Clear the viewport off the screen. We didn't clear earlier for two reasons. First, it
        // wasn't necessary because the buffer we drew out of isn't sparse, so it overwrote
        // whatever was on the screen. Second, there is a weird bug with tmux where a full screen
        // clear plus immediate scrolling causes some garbage to go into the scrollback.
        self.clear()?;

        Ok(())
    }

    /// Implement `Self::insert_before` using scrolling regions.
    ///
    /// If a terminal supports scrolling regions, it means that we can define a subset of rows of
    /// the screen, and then tell the terminal to scroll up or down just within that region. The
    /// rows outside of the region are not affected.
    ///
    /// This function utilizes this feature to avoid having to redraw the viewport. This is done
    /// either by splitting the screen at the top of the viewport, and then creating a gap by
    /// either scrolling the viewport down, or scrolling the area above it up. The lines to insert
    /// are then drawn into the gap created.
    #[cfg(feature = "scrolling-regions")]
    fn insert_before_scrolling_regions(
        &mut self,
        mut height: u16,
        draw_fn: impl FnOnce(&mut Buffer),
    ) -> Result<(), B::Error> {
        let area = Rect {
            x: 0,
            y: 0,
            width: self.viewport_area.width,
            height,
        };
        let mut buffer = Buffer::empty(area);
        draw_fn(&mut buffer);
        let mut buffer = buffer.content.as_slice();

        // Handle the special case where the viewport takes up the whole screen.
        if self.viewport_area.height == self.last_known_area.height {
            // "Borrow" the top line of the viewport. Draw over it, then immediately scroll it into
            // scrollback. Do this repeatedly until the whole buffer has been put into scrollback.
            let mut first = true;
            while !buffer.is_empty() {
                buffer = if first {
                    self.draw_lines(0, 1, buffer)?
                } else {
                    self.draw_lines_over_cleared(0, 1, buffer)?
                };
                first = false;
                self.backend.scroll_region_up(0..1, 1)?;
            }

            // Redraw the top line of the viewport.
            let width = self.viewport_area.width as usize;
            let top_line = self.buffers[1 - self.current].content[0..width].to_vec();
            self.draw_lines_over_cleared(0, 1, &top_line)?;
            return Ok(());
        }

        // Handle the case where the viewport isn't yet at the bottom of the screen.
        {
            let viewport_top = self.viewport_area.top();
            let viewport_bottom = self.viewport_area.bottom();
            let screen_bottom = self.last_known_area.bottom();
            if viewport_bottom < screen_bottom {
                let to_draw = height.min(screen_bottom - viewport_bottom);
                self.backend
                    .scroll_region_down(viewport_top..viewport_bottom + to_draw, to_draw)?;
                buffer = self.draw_lines_over_cleared(viewport_top, to_draw, buffer)?;
                self.set_viewport_area(Rect {
                    y: viewport_top + to_draw,
                    ..self.viewport_area
                });
                height -= to_draw;
            }
        }

        let viewport_top = self.viewport_area.top();
        while height > 0 {
            let to_draw = height.min(viewport_top);
            self.backend.scroll_region_up(0..viewport_top, to_draw)?;
            buffer = self.draw_lines_over_cleared(viewport_top - to_draw, to_draw, buffer)?;
            height -= to_draw;
        }

        Ok(())
    }

    /// Draw lines at the given vertical offset. The slice of cells must contain enough cells
    /// for the requested lines. A slice of the unused cells are returned.
    ///
    /// This is a small internal helper used by [`Terminal::insert_before`]. It writes cells
    /// directly to the backend in terminal coordinates (not viewport coordinates).
    fn draw_lines<'a>(
        &mut self,
        y_offset: u16,
        lines_to_draw: u16,
        cells: &'a [Cell],
    ) -> Result<&'a [Cell], B::Error> {
        let width: usize = self.last_known_area.width.into();
        let (to_draw, remainder) = cells.split_at(width * lines_to_draw as usize);
        if lines_to_draw > 0 {
            let iter = to_draw
                .iter()
                .enumerate()
                .map(|(i, c)| ((i % width) as u16, y_offset + (i / width) as u16, c));
            self.backend.draw(iter)?;
            self.backend.flush()?;
        }
        Ok(remainder)
    }

    /// Draw lines at the given vertical offset, assuming that the lines they are replacing on the
    /// screen are cleared. The slice of cells must contain enough cells for the requested lines. A
    /// slice of the unused cells are returned.
    ///
    /// This is used by the `scrolling-regions` implementation of [`Terminal::insert_before`] to
    /// avoid relying on a full-screen clear while updating only part of the terminal.
    #[cfg(feature = "scrolling-regions")]
    fn draw_lines_over_cleared<'a>(
        &mut self,
        y_offset: u16,
        lines_to_draw: u16,
        cells: &'a [Cell],
    ) -> Result<&'a [Cell], B::Error> {
        let width: usize = self.last_known_area.width.into();
        let (to_draw, remainder) = cells.split_at(width * lines_to_draw as usize);
        if lines_to_draw > 0 {
            let area = Rect::new(0, y_offset, width as u16, y_offset + lines_to_draw);
            let old = Buffer::empty(area);
            let new = Buffer {
                area,
                content: to_draw.to_vec(),
            };
            self.backend.draw(old.diff(&new).into_iter())?;
            self.backend.flush()?;
        }
        Ok(remainder)
    }

    /// Scroll the whole screen up by the given number of lines.
    ///
    /// This is used by [`Terminal::insert_before`] when the `scrolling-regions` feature is
    /// disabled.
    /// It scrolls by moving the cursor to the last row and calling [`Backend::append_lines`].
    #[cfg(not(feature = "scrolling-regions"))]
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
/// - [`Terminal::autoresize`] (detects backend size changes during [`Terminal::draw`])
/// - [`Terminal::resize`] (recomputes the viewport and clears before the next draw)
pub(crate) fn compute_inline_size<B: Backend>(
    backend: &mut B,
    height: u16,
    size: Size,
    offset_in_previous_viewport: u16,
) -> Result<(Rect, Position), B::Error> {
    let pos = backend.get_cursor_position()?;
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
    use crate::style::Style;
    use crate::terminal::inline::compute_inline_size;
    use crate::terminal::{Terminal, TerminalOptions, Viewport};

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
            compute_inline_size(&mut backend, 4, Size::new(10, 10), 1).unwrap();

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
            compute_inline_size(&mut backend, 4, Size::new(10, 10), 5).unwrap();

        assert_eq!(area, Rect::new(0, 0, 10, 4));
    }

    #[cfg(not(feature = "scrolling-regions"))]
    mod no_scrolling_regions {
        use super::*;

        #[test]
        fn insert_before_is_noop_for_non_inline_viewports() {
            // Diagram:
            //
            // Viewport is fullscreen (not inline), so insert_before() is a no-op.
            //
            // Screen before:
            //   x..
            //   ...
            //
            // Screen after:
            //   x..
            //   ...
            let mut terminal = Terminal::new(TestBackend::new(3, 2)).unwrap();
            {
                let frame = terminal.get_frame();
                frame.buffer[(0, 0)].set_symbol("x");
            }
            terminal.flush().unwrap();

            let viewport_area = terminal.viewport_area;
            terminal
                .insert_before(1, |buf| {
                    buf.set_string(0, 0, "zzz", Style::default());
                })
                .unwrap();

            assert_eq!(terminal.viewport_area, viewport_area);
            terminal.backend().assert_buffer_lines(["x  ", "   "]);
        }

        #[test]
        fn insert_before_pushes_viewport_down_when_space_available() {
            // Diagram (screen height = 10, viewport height = 4, cursor row = 3):
            //
            // Before:
            //   0: 0000000000
            //   1: 1111111111
            //   2: 2222222222
            //   3: [viewport top] 3333333333
            //   4:               4444444444
            //   5:               5555555555
            //   6:               6666666666
            //   7: 7777777777
            //   8: 8888888888
            //   9: 9999999999
            //
            // After inserting 1 line above an inline viewport (no scrolling regions):
            // - A line is drawn at the old viewport top (y=3)
            // - The viewport moves down by 1 row (new top y=4)
            // - The viewport is cleared so it will be redrawn on the next draw()
            let mut backend = TestBackend::with_lines([
                "0000000000",
                "1111111111",
                "2222222222",
                "3333333333",
                "4444444444",
                "5555555555",
                "6666666666",
                "7777777777",
                "8888888888",
                "9999999999",
            ]);
            backend
                .set_cursor_position(Position { x: 0, y: 3 })
                .unwrap();
            let mut terminal = Terminal::with_options(
                backend,
                TerminalOptions {
                    viewport: Viewport::Inline(4),
                },
            )
            .unwrap();

            terminal
                .insert_before(1, |buf| {
                    buf.set_string(0, 0, "INSERTLINE", Style::default());
                })
                .unwrap();

            assert_eq!(terminal.viewport_area, Rect::new(0, 4, 10, 4));
            terminal.backend().assert_buffer_lines([
                "0000000000",
                "1111111111",
                "2222222222",
                "INSERTLINE",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
            ]);
        }

        #[test]
        fn insert_before_scrolls_when_viewport_is_at_bottom() {
            // Diagram (screen height = 10, viewport height = 4, cursor row = 6):
            //
            // Before:
            //   0: 0000000000
            //   1: 1111111111
            //   2: 2222222222
            //   3: 3333333333
            //   4: 4444444444
            //   5: 5555555555
            //   6: [viewport top] 6666666666
            //   7:               7777777777
            //   8:               8888888888
            //   9:               9999999999
            //
            // After inserting 2 lines:
            // - The area above the viewport scrolls up to make room
            // - Inserted lines appear immediately above the viewport
            // - The viewport is cleared so it will be redrawn on the next draw()
            let mut backend = TestBackend::with_lines([
                "0000000000",
                "1111111111",
                "2222222222",
                "3333333333",
                "4444444444",
                "5555555555",
                "6666666666",
                "7777777777",
                "8888888888",
                "9999999999",
            ]);
            backend
                .set_cursor_position(Position { x: 0, y: 6 })
                .unwrap();
            let mut terminal = Terminal::with_options(
                backend,
                TerminalOptions {
                    viewport: Viewport::Inline(4),
                },
            )
            .unwrap();

            terminal
                .insert_before(2, |buf| {
                    buf.set_string(0, 0, "INSERTED1", Style::default());
                    buf.set_string(0, 1, "INSERTED2", Style::default());
                })
                .unwrap();

            assert_eq!(terminal.viewport_area, Rect::new(0, 6, 10, 4));
            terminal.backend().assert_buffer_lines([
                "2222222222",
                "3333333333",
                "4444444444",
                "5555555555",
                "INSERTED1 ",
                "INSERTED2 ",
                "          ",
                "          ",
                "          ",
                "          ",
            ]);
        }

        #[test]
        fn insert_before_then_draw_repaints_cleared_viewport() {
            // Diagram (screen height = 10, viewport height = 4, cursor row = 6):
            //
            // 1) Draw a frame into the inline viewport at the bottom:
            //   6..9: AAAAAAAAAA
            //
            // 2) Insert 2 lines above the viewport:
            //   - Inserts appear at rows 4..5
            //   - Viewport is cleared (so it is blank on-screen until the next draw)
            //
            // 3) Draw again:
            //   6..9: BBBBBBBBBB
            //
            // Expected final screen:
            //   4: INSERTED00
            //   5: INSERTED01
            //   6..9: BBBBBBBBBB
            let mut backend = TestBackend::new(10, 10);
            backend
                .set_cursor_position(Position { x: 0, y: 6 })
                .unwrap();
            let mut terminal = Terminal::with_options(
                backend,
                TerminalOptions {
                    viewport: Viewport::Inline(4),
                },
            )
            .unwrap();

            terminal
                .draw(|frame| {
                    let area = frame.area();
                    for y in area.top()..area.bottom() {
                        frame
                            .buffer
                            .set_string(area.x, y, "AAAAAAAAAA", Style::default());
                    }
                })
                .unwrap();

            terminal
                .insert_before(2, |buf| {
                    buf.set_string(0, 0, "INSERTED00", Style::default());
                    buf.set_string(0, 1, "INSERTED01", Style::default());
                })
                .unwrap();

            terminal
                .draw(|frame| {
                    let area = frame.area();
                    for y in area.top()..area.bottom() {
                        frame
                            .buffer
                            .set_string(area.x, y, "BBBBBBBBBB", Style::default());
                    }
                })
                .unwrap();

            terminal.backend().assert_buffer_lines([
                "          ",
                "          ",
                "          ",
                "          ",
                "INSERTED00",
                "INSERTED01",
                "BBBBBBBBBB",
                "BBBBBBBBBB",
                "BBBBBBBBBB",
                "BBBBBBBBBB",
            ]);
        }
    }

    #[cfg(feature = "scrolling-regions")]
    mod scrolling_regions {
        use super::*;

        #[test]
        fn insert_before_moves_viewport_down_without_clearing() {
            // Diagram (screen height = 10, viewport height = 4, cursor row = 3):
            //
            // With scrolling regions enabled, we can create a gap and draw the inserted line
            // without clearing the viewport content.
            //
            // Before:
            //   2: 2222222222
            //   3: [viewport top] 3333333333
            //   4:               4444444444
            //
            // After:
            //   3: INSERTLINE
            //   4: 3333333333  (viewport content preserved)
            let mut backend = TestBackend::with_lines([
                "0000000000",
                "1111111111",
                "2222222222",
                "3333333333",
                "4444444444",
                "5555555555",
                "6666666666",
                "7777777777",
                "8888888888",
                "9999999999",
            ]);
            backend
                .set_cursor_position(Position { x: 0, y: 3 })
                .unwrap();
            let mut terminal = Terminal::with_options(
                backend,
                TerminalOptions {
                    viewport: Viewport::Inline(4),
                },
            )
            .unwrap();

            terminal
                .insert_before(1, |buf| {
                    buf.set_string(0, 0, "INSERTLINE", Style::default());
                })
                .unwrap();

            assert_eq!(terminal.viewport_area, Rect::new(0, 4, 10, 4));
            terminal.backend().assert_buffer_lines([
                "0000000000",
                "1111111111",
                "2222222222",
                "INSERTLINE",
                "3333333333",
                "4444444444",
                "5555555555",
                "6666666666",
                "8888888888",
                "9999999999",
            ]);
        }

        #[test]
        fn insert_before_when_viewport_is_at_bottom_preserves_viewport() {
            // Diagram (screen height = 10, viewport height = 4, viewport top = 6):
            //
            // With scrolling regions enabled and the viewport already at the bottom:
            // - The region above the viewport (rows 0..6) scrolls up to make room.
            // - Inserted lines are drawn into the cleared space immediately above the viewport.
            // - The viewport itself is not cleared and stays on-screen.
            //
            // Before (after drawing V into the viewport):
            //   0: 0000000000
            //   1: 1111111111
            //   2: 2222222222
            //   3: 3333333333
            //   4: 4444444444
            //   5: 5555555555
            //   6..9: VVVVVVVVVV
            //
            // After inserting 2 lines:
            //   0..3: previous 2..5
            //   4: AAAAAAAAAA
            //   5: BBBBBBBBBB
            //   6..9: VVVVVVVVVV
            //
            // The scrolled-off lines are appended to scrollback (previous 0 and 1).
            let mut backend = TestBackend::with_lines([
                "0000000000",
                "1111111111",
                "2222222222",
                "3333333333",
                "4444444444",
                "5555555555",
                "6666666666",
                "7777777777",
                "8888888888",
                "9999999999",
            ]);
            backend
                .set_cursor_position(Position { x: 0, y: 6 })
                .unwrap();
            let mut terminal = Terminal::with_options(
                backend,
                TerminalOptions {
                    viewport: Viewport::Inline(4),
                },
            )
            .unwrap();

            terminal
                .draw(|frame| {
                    let area = frame.area();
                    for y in area.top()..area.bottom() {
                        frame
                            .buffer
                            .set_string(area.x, y, "VVVVVVVVVV", Style::default());
                    }
                })
                .unwrap();

            terminal
                .insert_before(2, |buf| {
                    buf.set_string(0, 0, "AAAAAAAAAA", Style::default());
                    buf.set_string(0, 1, "BBBBBBBBBB", Style::default());
                })
                .unwrap();

            terminal.backend().assert_buffer_lines([
                "2222222222",
                "3333333333",
                "4444444444",
                "5555555555",
                "AAAAAAAAAA",
                "BBBBBBBBBB",
                "VVVVVVVVVV",
                "VVVVVVVVVV",
                "VVVVVVVVVV",
                "VVVVVVVVVV",
            ]);
            terminal
                .backend()
                .assert_scrollback_lines(["0000000000", "1111111111"]);
        }

        #[test]
        fn insert_before_when_viewport_is_fullscreen_appends_to_scrollback() {
            // Diagram (screen height = 4, viewport height = 4):
            //
            // When the viewport takes the whole screen, there is no visible "area above" it.
            // The scrolling-regions implementation handles this by repeatedly:
            // - drawing one line over the top row
            // - immediately scrolling that row into scrollback
            //
            // The viewport content stays on-screen; inserted lines end up in scrollback.
            let mut backend = TestBackend::new(10, 4);
            backend
                .set_cursor_position(Position { x: 0, y: 0 })
                .unwrap();
            let mut terminal = Terminal::with_options(
                backend,
                TerminalOptions {
                    viewport: Viewport::Inline(4),
                },
            )
            .unwrap();

            terminal
                .draw(|frame| {
                    let area = frame.area();
                    frame
                        .buffer
                        .set_string(area.x, area.y, "VIEWLINE00", Style::default());
                    frame
                        .buffer
                        .set_string(area.x, area.y + 1, "VIEWLINE01", Style::default());
                    frame
                        .buffer
                        .set_string(area.x, area.y + 2, "VIEWLINE02", Style::default());
                    frame
                        .buffer
                        .set_string(area.x, area.y + 3, "VIEWLINE03", Style::default());
                })
                .unwrap();

            terminal
                .insert_before(2, |buf| {
                    buf.set_string(0, 0, "INSERTED00", Style::default());
                    buf.set_string(0, 1, "INSERTED01", Style::default());
                })
                .unwrap();

            terminal.backend().assert_buffer_lines([
                "VIEWLINE00",
                "VIEWLINE01",
                "VIEWLINE02",
                "VIEWLINE03",
            ]);
            terminal
                .backend()
                .assert_scrollback_lines(["INSERTED00", "INSERTED01"]);
        }
    }
}
