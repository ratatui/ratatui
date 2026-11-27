use crate::backend::ScrollByRegion;
use crate::buffer::{Buffer, Cell, CellWidth};
use crate::layout::Rect;
use crate::terminal::{Terminal, Viewport};

impl<B: ScrollByRegion> Terminal<B> {
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
    /// This uses the backend's scrolling regions (see [`ScrollByRegion`]), so the viewport is not
    /// cleared or redrawn. It is only available for backends that implement [`ScrollByRegion`].
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
    /// This is a no-op for non-inline viewports and when the terminal has no rows.
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
    pub fn insert_before<F>(&mut self, mut height: u16, draw_fn: F) -> Result<(), B::Error>
    where
        F: FnOnce(&mut Buffer),
    {
        if !matches!(self.viewport, Viewport::Inline(_)) || self.last_known_area.height == 0 {
            return Ok(());
        }

        // `insert_before` draws lines and scrolls the terminal directly on the backend, which
        // moves the physical cursor outside the `apply_buffer_with_cursor` flow. Invalidate
        // the next-frame `MoveTo` dedup tracking so a following `draw` always re-emits `Show`
        // + `MoveTo` to reposition the caret, even if it requests an otherwise unchanged
        // position.
        self.last_frame_cursor_position = None;

        // Scrolling regions let us scroll a subset of the screen rows while leaving the rest
        // untouched. We split the screen at the top of the viewport and open a gap by either
        // scrolling the viewport down or scrolling the area above it up, then draw the inserted
        // lines into that gap. This avoids clearing and redrawing the viewport.
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

    /// Draw lines at the given vertical offset, assuming that the lines they are replacing on the
    /// screen are cleared. The slice of cells must contain enough cells for the requested lines. A
    /// slice of the unused cells are returned.
    ///
    /// This is used by [`Terminal::insert_before`] to
    /// avoid relying on a full-screen clear while updating only part of the terminal.
    fn draw_lines_over_cleared<'a>(
        &mut self,
        y_offset: u16,
        lines_to_draw: u16,
        cells: &'a [Cell],
    ) -> Result<&'a [Cell], B::Error> {
        let width: usize = self.last_known_area.width.into();
        let (to_draw, remainder) = cells.split_at(width * lines_to_draw as usize);
        if lines_to_draw > 0 {
            // Skip the cells hidden behind wide glyphs: drawn after the glyph, they erase it.
            let mut hidden = 0;
            let iter = to_draw
                .iter()
                .enumerate()
                .filter(|(i, c)| {
                    // A glyph never hides cells on the next row.
                    let visible = hidden == 0 || i % width == 0;
                    hidden = if visible { c.cell_width() } else { hidden }.saturating_sub(1);
                    visible
                })
                // The screen is already cleared, so empty cells don't need drawing.
                .filter(|(_, c)| **c != Cell::EMPTY)
                .map(|(i, c)| ((i % width) as u16, y_offset + (i / width) as u16, c));
            self.backend.draw(iter)?;
            self.backend.flush()?;
        }
        Ok(remainder)
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
            // Skip the cells hidden behind wide glyphs: drawn after the glyph, they erase it.
            let mut hidden = 0;
            let iter = to_draw
                .iter()
                .enumerate()
                .filter(|(i, c)| {
                    // A glyph never hides cells on the next row.
                    let visible = hidden == 0 || i % width == 0;
                    hidden = if visible { c.cell_width() } else { hidden }.saturating_sub(1);
                    visible
                })
                .map(|(i, c)| ((i % width) as u16, y_offset + (i / width) as u16, c));
            self.backend.draw(iter)?;
            self.backend.flush()?;
        }
        Ok(remainder)
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::{Backend, TestBackend};
    use crate::layout::{Position, Rect};
    use crate::style::Style;
    use crate::terminal::{Terminal, TerminalOptions, Viewport};

    #[test]
    fn insert_before_is_noop_when_terminal_has_no_rows() {
        let backend = TestBackend::new(5, 0);
        let options = TerminalOptions {
            viewport: Viewport::Inline(3),
        };
        let mut terminal = Terminal::with_options(backend, options).unwrap();
        let scrollback = terminal.backend().scrollback().clone();

        terminal
            .insert_before(2, |buf| buf.set_string(0, 0, "aaaaa", Style::default()))
            .unwrap();

        assert_eq!(terminal.backend().scrollback(), &scrollback);
    }

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
    fn insert_before_skips_cells_hidden_behind_wide_glyphs() {
        let backend = TestBackend::new(3, 4);
        let options = TerminalOptions {
            viewport: Viewport::Inline(1),
        };
        let mut terminal = Terminal::with_options(backend, options).unwrap();

        terminal
            .insert_before(2, |buf| {
                buf[(0, 0)].set_symbol("界");
                // Hidden behind `界`: drawing it would erase the glyph.
                buf[(1, 0)].set_symbol("x");
                // A glyph in the last column hides nothing on the next row.
                buf[(2, 0)].set_symbol("界");
                buf[(0, 1)].set_symbol("b");
            })
            .unwrap();

        let screen = terminal.backend().buffer();
        assert_eq!(screen[(1, 0)].symbol(), " ");
        assert_eq!(screen[(0, 1)].symbol(), "b");
    }

    #[test]
    fn insert_before_skips_hidden_cells_when_viewport_is_fullscreen() {
        // A fullscreen viewport sends each inserted line straight to scrollback. The first line is
        // drawn with `draw_lines` and later ones with `draw_lines_over_cleared`, so put a wide
        // glyph on both.
        let backend = TestBackend::new(3, 2);
        let options = TerminalOptions {
            viewport: Viewport::Inline(2),
        };
        let mut terminal = Terminal::with_options(backend, options).unwrap();

        terminal
            .insert_before(2, |buf| {
                for y in 0..2 {
                    buf[(0, y)].set_symbol("界");
                    // Hidden behind `界`: drawing it would erase the glyph.
                    buf[(1, y)].set_symbol("x");
                    buf[(2, y)].set_symbol("b");
                }
            })
            .unwrap();

        let scrollback = terminal.backend().scrollback();
        for y in 0..2 {
            assert_eq!(scrollback[(0, y)].symbol(), "界");
            assert_eq!(scrollback[(1, y)].symbol(), " ");
            assert_eq!(scrollback[(2, y)].symbol(), "b");
        }
    }

    #[test]
    fn insert_before_then_draw_repaints_viewport() {
        // Diagram (screen height = 10, viewport height = 4, cursor row = 6):
        //
        // 1) Draw a frame into the inline viewport at the bottom:
        //   6..9: AAAAAAAAAA
        //
        // 2) Insert 2 lines above the viewport:
        //   - Inserts appear at rows 4..5
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
        // The scrolling region implementation handles this by repeatedly:
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

    #[test]
    fn insert_before_invalidates_move_dedup_tracking() {
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
                frame.set_cursor_position(Position { x: 3, y: 7 });
            })
            .unwrap();
        assert_eq!(
            terminal.last_frame_cursor_position,
            Some(Position { x: 3, y: 7 }),
            "a draw with a caret records the dedup position"
        );

        terminal
            .insert_before(2, |buf| {
                buf.set_string(0, 0, "INSERTED00", Style::default());
            })
            .unwrap();

        assert_eq!(
            terminal.last_frame_cursor_position, None,
            "insert_before must invalidate the MoveTo dedup tracking"
        );
    }

    #[test]
    fn insert_before_is_noop_for_non_inline_and_preserves_dedup() {
        let mut terminal = Terminal::new(TestBackend::new(3, 2)).unwrap();

        terminal
            .draw(|frame| {
                frame.set_cursor_position(Position { x: 2, y: 1 });
            })
            .unwrap();
        assert_eq!(
            terminal.last_frame_cursor_position,
            Some(Position { x: 2, y: 1 }),
            "a draw with a caret records the dedup position"
        );

        terminal.insert_before(1, |_buf| ()).unwrap();

        assert_eq!(
            terminal.last_frame_cursor_position,
            Some(Position { x: 2, y: 1 }),
            "a no-op insert_before on a non-inline viewport must not invalidate the tracking"
        );
    }
}
