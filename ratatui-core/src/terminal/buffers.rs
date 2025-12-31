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
                self.backend.clear_region(ClearType::AfterCursor)?;
            }
            Viewport::Fixed(_) => {
                let area = self.viewport_area;
                for y in area.top()..area.bottom() {
                    self.backend.set_cursor_position(Position { x: 0, y })?;
                    self.backend.clear_region(ClearType::AfterCursor)?;
                }
            }
        }
        // Reset the back buffer to make sure the next update will redraw everything.
        self.buffers[1 - self.current].reset();
        Ok(())
    }
}
