use crate::backend::{Backend, ClearType};
use crate::buffer::{Buffer, Cell};
use crate::layout::{Position, Rect};
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
        let mut last_pos = None;

        let updates = previous_buffer
            .diff_iter(current_buffer)
            .inspect(|(col, row, _)| {
                last_pos = Some(Position { x: *col, y: *row });
            });
        self.backend.draw(updates)?;

        if let Some(pos) = last_pos {
            self.last_known_cursor_pos = pos;
        }

        Ok(())
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
    /// Current behavior: for [`Viewport::Inline`], clearing runs from the viewport origin through
    /// the end of the visible display area, not just the viewport's rectangle. This is an
    /// implementation detail rather than a contract; do not rely on it.
    ///
    /// This preserves the cursor position.
    ///
    /// This also resets the "previous" buffer so the next [`Terminal::flush`] redraws the full
    /// viewport. [`Terminal::resize`] calls this internally.
    ///
    /// Implementation note: this uses [`ClearType::AfterCursor`] starting at the viewport origin.
    pub fn clear(&mut self) -> Result<(), B::Error> {
        let original_cursor = self.backend.get_cursor_position()?;
        match self.viewport {
            Viewport::Fullscreen => self.backend.clear_region(ClearType::All)?,
            Viewport::Inline(_) => {
                self.backend
                    .set_cursor_position(self.viewport_area.as_position())?;
                self.backend.clear_region(ClearType::AfterCursor)?;
            }
            Viewport::Fixed(_) => {
                let area = self.viewport_area;
                self.clear_fixed_viewport(area)?;
            }
        }
        self.backend.set_cursor_position(original_cursor)?;
        // Reset the back buffer to make sure the next update will redraw everything.
        self.buffers[1 - self.current].reset();
        Ok(())
    }

    /// Clears a fixed viewport using terminal clear commands when possible.
    ///
    /// Terminal clear commands can be faster than per-cell updates.
    fn clear_fixed_viewport(&mut self, area: Rect) -> Result<(), B::Error> {
        if area.is_empty() {
            return Ok(());
        }
        let size = self.backend.size()?;
        let is_full_width = area.x == 0 && area.width == size.width;
        let ends_at_bottom = area.bottom() == size.height;
        if is_full_width && ends_at_bottom {
            self.backend.set_cursor_position(area.as_position())?;
            self.backend.clear_region(ClearType::AfterCursor)?;
        } else if is_full_width {
            self.clear_full_width_rows(area)?;
        } else {
            self.clear_region_cells(area)?;
        }
        Ok(())
    }

    /// Clears full-width rows using line clear commands.
    ///
    /// This avoids per-cell writes when the viewport spans the full width.
    fn clear_full_width_rows(&mut self, area: Rect) -> Result<(), B::Error> {
        for y in area.top()..area.bottom() {
            self.backend.set_cursor_position(Position { x: 0, y })?;
            self.backend.clear_region(ClearType::CurrentLine)?;
        }
        Ok(())
    }

    /// Clears a non-full-width region by writing empty cells directly.
    ///
    /// This is used when line-based clears would affect cells outside the viewport.
    fn clear_region_cells(&mut self, area: Rect) -> Result<(), B::Error> {
        let clear_cell = Cell::default();
        let updates = area.positions().map(|pos| (pos.x, pos.y, &clear_cell));
        self.backend.draw(updates)?;
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
        // Inline clear is implemented as:
        //   1) move the backend cursor to the viewport origin
        //   2) call ClearType::AfterCursor once
        let mut backend = TestBackend::with_lines([
            "before 1  ",
            "before 2  ",
            "viewport 1",
            "viewport 2",
            "after 1   ",
            "after 2   ",
        ]);
        backend
            .set_cursor_position(Position { x: 2, y: 2 })
            .unwrap();
        let options = TerminalOptions {
            viewport: Viewport::Inline(2),
        };
        let mut terminal = Terminal::with_options(backend, options).unwrap();
        terminal
            .backend_mut()
            .set_cursor_position(Position { x: 2, y: 2 })
            .unwrap();

        terminal.buffers[1][(2, 2)] = Cell::new("x");
        terminal.clear().unwrap();

        // Inline viewport is anchored to the cursor row (y = 2) with height 2. Clear runs from
        // the viewport origin through the end of the display, including the rows after it.
        terminal.backend().assert_buffer_lines([
            "before 1  ",
            "before 2  ",
            "          ",
            "          ",
            "          ",
            "          ",
        ]);
        assert_eq!(
            terminal.buffers[1 - terminal.current],
            Buffer::empty(terminal.viewport_area)
        );
        assert_eq!(
            terminal.backend().cursor_position(),
            Position { x: 2, y: 2 }
        );
    }

    #[test]
    fn clear_fixed_clears_viewport_rows_and_resets_back_buffer() {
        // For full-width fixed viewports that reach the terminal bottom, clear uses
        // ClearType::AfterCursor starting at the viewport origin.
        let mut backend = TestBackend::with_lines(["before 1  ", "viewport 1", "viewport 2"]);
        backend.set_cursor_position((2, 0)).unwrap();
        let options = TerminalOptions {
            viewport: Viewport::Fixed(Rect::new(0, 1, 10, 2)),
        };
        let mut terminal = Terminal::with_options(backend, options).unwrap();

        terminal.clear().unwrap();

        terminal
            .backend()
            .assert_buffer_lines(["before 1  ", "          ", "          "]);
        assert_eq!(
            terminal.buffers[1 - terminal.current],
            Buffer::empty(terminal.viewport_area)
        );
        assert_eq!(
            terminal.backend().cursor_position(),
            Position { x: 2, y: 0 }
        );
    }

    #[test]
    fn clear_fixed_full_width_not_at_bottom() {
        let mut backend =
            TestBackend::with_lines(["before 1  ", "viewport 1", "viewport 2", "after 1   "]);
        backend.set_cursor_position((1, 0)).unwrap();
        let options = TerminalOptions {
            viewport: Viewport::Fixed(Rect::new(0, 1, 10, 2)),
        };
        let mut terminal = Terminal::with_options(backend, options).unwrap();

        terminal.clear().unwrap();

        terminal.backend().assert_buffer_lines([
            "before 1  ",
            "          ",
            "          ",
            "after 1   ",
        ]);
        assert_eq!(
            terminal.backend().cursor_position(),
            Position { x: 1, y: 0 }
        );
    }

    #[test]
    fn clear_fixed_respects_non_full_width_viewport() {
        let mut backend =
            TestBackend::with_lines(["before 1  ", "viewport 1", "viewport 2", "after 1   "]);
        backend.set_cursor_position((3, 0)).unwrap();
        let options = TerminalOptions {
            viewport: Viewport::Fixed(Rect::new(1, 1, 3, 2)),
        };
        let mut terminal = Terminal::with_options(backend, options).unwrap();

        terminal.clear().unwrap();

        terminal.backend().assert_buffer_lines([
            "before 1  ",
            "v   port 1",
            "v   port 2",
            "after 1   ",
        ]);
        assert_eq!(
            terminal.backend().cursor_position(),
            Position { x: 3, y: 0 }
        );
    }
}
