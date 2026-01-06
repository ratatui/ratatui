use crate::backend::{Backend, ClearType};
use crate::buffer::Buffer;
use crate::layout::Position;
use crate::terminal::{Frame, Terminal, Viewport};

impl<B: Backend> Terminal<B> {
    /// Returns a [`Frame`] for manual rendering.
    ///
    /// Most applications should render via [`Terminal::draw`] / [`Terminal::try_draw`]. This method
    /// exposes the frame construction step used by [`Terminal::try_draw`] so tests and advanced
    /// callers can render without running the full draw pipeline.
    ///
    /// Unlike `draw` / `try_draw`, this does not call [`Terminal::autoresize`], does not write
    /// updates to the backend, and does not apply any cursor changes. After rendering, you
    /// typically call [`Terminal::flush`], [`Terminal::swap_buffers`], and [`Backend::flush`].
    ///
    /// The returned `Frame` mutably borrows the current buffer, so it must be dropped before you
    /// can call methods like [`Terminal::flush`]. The example below uses a scope to make that
    /// explicit.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # mod ratatui {
    /// #     pub use ratatui_core::backend;
    /// #     pub use ratatui_core::terminal::Terminal;
    /// # }
    /// use ratatui::Terminal;
    /// use ratatui::backend::{Backend, TestBackend};
    ///
    /// let backend = TestBackend::new(30, 5);
    /// let mut terminal = Terminal::new(backend)?;
    /// {
    ///     let mut frame = terminal.get_frame();
    ///     frame.render_widget("Hello", frame.area());
    /// }
    /// // When not using `draw`, present the buffer manually:
    /// terminal.flush()?;
    /// terminal.swap_buffers();
    /// terminal.backend_mut().flush()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`Backend::flush`]: crate::backend::Backend::flush
    pub const fn get_frame(&mut self) -> Frame<'_> {
        let count = self.frame_count;
        Frame {
            cursor_position: None,
            viewport_area: self.viewport_area,
            buffer: self.current_buffer_mut(),
            count,
        }
    }

    /// Gets the current buffer as a mutable reference.
    ///
    /// This is the buffer that the next [`Frame`] will render into (see [`Terminal::get_frame`]).
    /// Most applications should render inside [`Terminal::draw`] and access the buffer via
    /// [`Frame::buffer_mut`] instead.
    pub const fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current]
    }

    /// Writes the current buffer to the backend using a diff against the previous buffer.
    ///
    /// This is one of the building blocks used by [`Terminal::draw`] / [`Terminal::try_draw`]. It
    /// does not swap buffers or flush the backend; see [`Terminal::swap_buffers`] and
    /// [`Backend::flush`].
    ///
    /// Implementation note: when there are updates, Ratatui records the position of the last
    /// updated cell as the "last known cursor position". Inline viewports use this to preserve the
    /// cursor's relative position within the viewport across resizes.
    ///
    /// [`Backend::flush`]: crate::backend::Backend::flush
    pub fn flush(&mut self) -> Result<(), B::Error> {
        let previous_buffer = &self.buffers[1 - self.current];
        let current_buffer = &self.buffers[self.current];
        let updates = previous_buffer.diff(current_buffer);
        if let Some((col, row, _)) = updates.last() {
            self.last_known_cursor_pos = Position { x: *col, y: *row };
        }
        self.backend.draw(updates.into_iter())
    }

    /// Clears the inactive buffer and swaps it with the current buffer.
    ///
    /// This is part of the standard rendering flow (see [`Terminal::try_draw`]). If you render
    /// manually using [`Terminal::get_frame`] and [`Terminal::flush`], call this afterward so the
    /// next flush can compute diffs against the correct "previous" buffer.
    pub fn swap_buffers(&mut self) {
        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;
    }

    /// Clear the terminal and force a full redraw on the next draw call.
    ///
    /// What gets cleared depends on the active [`Viewport`]:
    ///
    /// - [`Viewport::Fullscreen`]: clears the entire terminal.
    /// - [`Viewport::Fixed`]: clears only the viewport region.
    /// - [`Viewport::Inline`]: clears after the viewport's origin, leaving any content above the
    ///   viewport untouched.
    ///
    /// This also resets the "previous" buffer so the next [`Terminal::flush`] redraws the full
    /// viewport. [`Terminal::resize`] calls this internally.
    ///
    /// Implementation note: this uses [`ClearType::AfterCursor`] starting at the viewport origin.
    pub fn clear(&mut self) -> Result<(), B::Error> {
        match self.viewport {
            Viewport::Fullscreen => self.backend.clear_region(ClearType::All)?,
            Viewport::Inline(_) => {
                self.backend
                    .set_cursor_position(self.viewport_area.as_position())?;
                // TODO: `ClearType::AfterCursor` is exclusive of the cursor cell in `TestBackend`
                // (and in terminals that interpret this as "after" rather than "from"), which can
                // leave the viewport origin cell uncleared. Consider switching to a clear that
                // includes the cursor cell when fixing clear semantics.
                self.backend.clear_region(ClearType::AfterCursor)?;
            }
            Viewport::Fixed(_) => {
                let area = self.viewport_area;
                for y in area.top()..area.bottom() {
                    // TODO: Fixed viewports can start at x > 0 and have a limited width. Clearing
                    // from x = 0 clears outside the viewport. Consider clearing only within
                    // `viewport_area` (respecting both x offset and width) when fixing clear
                    // semantics.
                    self.backend.set_cursor_position(Position { x: 0, y })?;
                    // TODO: `ClearType::AfterCursor` is exclusive of the cursor cell in
                    // `TestBackend`, so the first cell of each cleared row can remain. Consider a
                    // clear mode that includes the cursor cell when fixing clear semantics.
                    self.backend.clear_region(ClearType::AfterCursor)?;
                }
            }
        }
        // Reset the back buffer to make sure the next update will redraw everything.
        self.buffers[1 - self.current].reset();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::{Backend, TestBackend};
    use crate::buffer::{Buffer, Cell};
    use crate::layout::{Position, Rect};
    use crate::terminal::{Terminal, TerminalOptions, Viewport};

    #[test]
    fn get_frame_uses_current_viewport_and_frame_count() {
        let backend = TestBackend::new(5, 3);
        let mut terminal = Terminal::new(backend).unwrap();

        let frame = terminal.get_frame();
        assert_eq!(frame.count, 0);
        assert_eq!(frame.area().width, 5);
        assert_eq!(frame.area().height, 3);
        assert_eq!(frame.buffer.area, frame.area());
    }

    #[test]
    fn flush_writes_updates_and_tracks_last_updated_cell() {
        let backend = TestBackend::new(3, 2);
        let mut terminal = Terminal::new(backend).unwrap();

        {
            let frame = terminal.get_frame();
            frame.buffer[(1, 0)].set_symbol("x");
        }

        terminal.flush().unwrap();
        terminal.backend().assert_buffer_lines([" x ", "   "]);
        assert_eq!(terminal.last_known_cursor_pos, Position { x: 1, y: 0 });
    }

    #[test]
    fn flush_with_no_updates_does_not_change_last_known_cursor_pos() {
        let backend = TestBackend::new(3, 2);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.set_cursor_position((2, 1)).unwrap();

        terminal.flush().unwrap();

        assert_eq!(terminal.last_known_cursor_pos, Position { x: 2, y: 1 });
    }

    #[test]
    fn swap_buffers_resets_new_current_buffer() {
        let backend = TestBackend::new(3, 2);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.buffers[1][(0, 0)].set_symbol("x");
        terminal.swap_buffers();

        assert_eq!(terminal.current, 1);
        assert_eq!(
            terminal.buffers[terminal.current],
            Buffer::empty(terminal.viewport_area)
        );
    }

    #[test]
    fn clear_fullscreen_clears_backend_and_resets_back_buffer() {
        let backend = TestBackend::new(3, 2);
        let mut terminal = Terminal::new(backend).unwrap();

        {
            let frame = terminal.get_frame();
            frame.buffer[(0, 0)] = Cell::new("x");
        }
        terminal.flush().unwrap();
        terminal.backend().assert_buffer_lines(["x  ", "   "]);

        terminal.buffers[1][(2, 1)] = Cell::new("y");
        terminal.clear().unwrap();

        terminal.backend().assert_buffer_lines(["   ", "   "]);
        assert_eq!(
            terminal.buffers[1 - terminal.current],
            Buffer::empty(terminal.viewport_area)
        );
    }

    #[test]
    fn clear_inline_clears_after_viewport_origin_and_resets_back_buffer() {
        // Characterization test:
        // The current implementation clears using ClearType::AfterCursor, which is exclusive of
        // the cursor cell. This yields somewhat surprising results (the origin cell is left
        // untouched). We'll fix the clear semantics later; this test locks down current behavior.
        //
        // Inline clear is implemented as:
        //   1) move the backend cursor to the viewport origin
        //   2) call ClearType::AfterCursor once
        //
        // Note: TestBackend's ClearType::AfterCursor clears *after the cursor position*, keeping
        // the cell at the cursor intact, and clears through the end of the screen buffer.
        let mut backend = TestBackend::with_lines(["aaa", "bbb", "ccc"]);
        backend.set_cursor_position((0, 1)).unwrap();
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(1),
            },
        )
        .unwrap();

        terminal.buffers[1][(2, 1)] = Cell::new("x");
        terminal.clear().unwrap();

        terminal
            .backend()
            .assert_buffer_lines(["aaa", "b  ", "   "]);
        assert_eq!(
            terminal.buffers[1 - terminal.current],
            Buffer::empty(terminal.viewport_area)
        );
        // The inline branch also explicitly sets the cursor to the viewport origin before
        // clearing, so the backend cursor ends up at that origin.
        assert_eq!(
            terminal.backend().cursor_position(),
            Position { x: 0, y: 1 }
        );
    }

    #[test]
    fn clear_fixed_clears_viewport_rows_and_resets_back_buffer() {
        // Characterization test:
        // The current implementation clears using ClearType::AfterCursor, which is exclusive of
        // the cursor cell. This yields somewhat surprising results (each row's first cell is left
        // untouched, and TestBackend clears through the end of the screen). We'll fix the clear
        // semantics later; this test locks down current behavior.
        //
        // Fixed clear is implemented as: for each viewport row, set the cursor to the start of
        // the row (x = 0) and call ClearType::AfterCursor.
        //
        // Note: TestBackend's ClearType::AfterCursor clears from *after the cursor* through the
        // end of the screen buffer (not just the current line). That means the first iteration
        // clears everything below the viewport's first row too.
        let backend = TestBackend::with_lines(["aaa", "bbb", "ccc"]);
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Fixed(Rect::new(0, 1, 3, 2)),
            },
        )
        .unwrap();

        terminal.buffers[1][(2, 1)] = Cell::new("x");
        terminal.clear().unwrap();

        terminal
            .backend()
            .assert_buffer_lines(["aaa", "b  ", "   "]);
        assert_eq!(
            terminal.buffers[1 - terminal.current],
            Buffer::empty(terminal.viewport_area)
        );
        // The fixed branch sets the cursor for each row it processes; after the loop, the cursor
        // is left at the start of the last processed row.
        assert_eq!(
            terminal.backend().cursor_position(),
            Position { x: 0, y: 2 }
        );
    }
}
