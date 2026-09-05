use crate::backend::Backend;
use crate::layout::Position;
use crate::terminal::{CompletedFrame, Frame, Terminal};

impl<B: Backend> Terminal<B> {
    /// Draws a single frame to the terminal.
    ///
    /// Returns a [`CompletedFrame`] if successful, otherwise a backend error (`B::Error`).
    ///
    /// If the render callback passed to this method can fail, use [`try_draw`] instead.
    ///
    /// Applications should call `draw` or [`try_draw`] in a loop to continuously render the
    /// terminal. These methods are the main entry points for drawing to the terminal.
    ///
    /// [`try_draw`]: Terminal::try_draw
    ///
    /// The [`Frame`] passed to the render callback represents the currently configured
    /// [`Viewport`] (see [`Frame::area`] and [`Terminal::with_options`]).
    ///
    /// Build layout relative to the [`Rect`] returned by [`Frame::area`] rather than assuming the
    /// origin is `(0, 0)`, so the same rendering code works for fixed and inline viewports.
    ///
    /// [`Frame::area`]: crate::terminal::Frame::area
    /// [`Rect`]: crate::layout::Rect
    /// [`Viewport`]: crate::terminal::Viewport
    ///
    /// This method will:
    ///
    /// - call [`Terminal::autoresize`] if necessary
    /// - call the render callback, passing it a [`Frame`] reference to render to
    /// - call [`Terminal::flush`] to apply the current buffer diff to the backend
    /// - show/hide the cursor based on [`Frame::set_cursor_position`]
    /// - call [`Terminal::swap_buffers`] to prepare for the next render pass
    /// - call [`Backend::flush`] to flush any buffered backend output
    /// - return a [`CompletedFrame`] with the current buffer and the area used for rendering
    ///
    /// If any backend step fails, the error is returned immediately and later steps in the render
    /// pass are skipped.
    ///
    /// The [`CompletedFrame`] returned by this method can be useful for debugging or testing
    /// purposes, but it is often not used in regular applications.
    ///
    /// The render callback should fully render the entire frame when called, including areas that
    /// are unchanged from the previous frame. This is because each frame is compared to the
    /// previous frame to determine what has changed, and only the changes are written to the
    /// terminal. If the render callback does not fully render the frame, the terminal will not be
    /// in a consistent state.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # mod ratatui {
    /// #     pub use ratatui_core::backend;
    /// #     pub use ratatui_core::layout;
    /// #     pub use ratatui_core::terminal::{Frame, Terminal};
    /// # }
    /// use ratatui::backend::TestBackend;
    /// use ratatui::layout::Position;
    /// use ratatui::{Frame, Terminal};
    ///
    /// let backend = TestBackend::new(10, 10);
    /// let mut terminal = Terminal::new(backend)?;
    ///
    /// // With a closure.
    /// terminal.draw(|frame| {
    ///     let area = frame.area();
    ///     frame.render_widget("Hello World!", area);
    ///     frame.set_cursor_position(Position { x: 0, y: 0 });
    /// })?;
    ///
    /// // Or with a function.
    /// terminal.draw(render)?;
    ///
    /// fn render(frame: &mut Frame<'_>) {
    ///     frame.render_widget("Hello World!", frame.area());
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`Backend::flush`]: crate::backend::Backend::flush
    pub fn draw<F>(&mut self, render_callback: F) -> Result<CompletedFrame<'_>, B::Error>
    where
        F: FnOnce(&mut Frame),
    {
        self.try_draw(|frame| {
            render_callback(frame);
            Ok::<(), B::Error>(())
        })
    }

    /// Tries to draw a single frame to the terminal.
    ///
    /// Returns [`Result::Ok`] containing a [`CompletedFrame`] if successful, otherwise
    /// [`Result::Err`] containing the backend error (`B::Error`) that caused the failure.
    ///
    /// This is the equivalent of [`Terminal::draw`] but the render callback is a function or
    /// closure that returns a `Result` instead of nothing.
    ///
    /// Applications should call `try_draw` or [`draw`] in a loop to continuously render the
    /// terminal. These methods are the main entry points for drawing to the terminal.
    ///
    /// [`draw`]: Terminal::draw
    ///
    /// The [`Frame`] passed to the render callback represents the currently configured
    /// [`Viewport`] (see [`Frame::area`] and [`Terminal::with_options`]).
    ///
    /// Build layout relative to the [`Rect`] returned by [`Frame::area`] rather than assuming the
    /// origin is `(0, 0)`, so the same rendering code works for fixed and inline viewports.
    ///
    /// [`Frame::area`]: crate::terminal::Frame::area
    /// [`Rect`]: crate::layout::Rect
    /// [`Viewport`]: crate::terminal::Viewport
    ///
    /// This method will:
    ///
    /// - call [`Terminal::autoresize`] if necessary
    /// - call the render callback, passing it a [`Frame`] reference to render to
    /// - call [`Terminal::flush`] to apply the current buffer diff to the backend
    /// - show/hide the cursor based on [`Frame::set_cursor_position`]
    /// - call [`Terminal::swap_buffers`] to prepare for the next render pass
    /// - call [`Backend::flush`] to flush any buffered backend output
    /// - return a [`CompletedFrame`] with the current buffer and the area used for rendering
    ///
    /// If the render callback returns an error, Ratatui leaves the backend, buffers, cursor state,
    /// and frame count unchanged.
    ///
    /// The render callback passed to `try_draw` can return any [`Result`] with an error type that
    /// can be converted into `B::Error` using the [`Into`] trait. This makes it possible to use the
    /// `?` operator to propagate errors that occur during rendering. If the render callback returns
    /// an error, the error will be returned from `try_draw` and the terminal will not be updated.
    ///
    /// The [`CompletedFrame`] returned by this method can be useful for debugging or testing
    /// purposes, but it is often not used in regular applications.
    ///
    /// The render callback should fully render the entire frame when called, including areas that
    /// are unchanged from the previous frame. This is because each frame is compared to the
    /// previous frame to determine what has changed, and only the changes are written to the
    /// terminal. If the render function does not fully render the frame, the terminal will not be
    /// in a consistent state.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # #![allow(unexpected_cfgs)]
    /// # #[cfg(feature = "crossterm")]
    /// # {
    /// use std::io;
    ///
    /// use ratatui::backend::CrosstermBackend;
    /// use ratatui::layout::Position;
    /// use ratatui::{Frame, Terminal};
    ///
    /// let backend = CrosstermBackend::new(std::io::stdout());
    /// let mut terminal = Terminal::new(backend)?;
    ///
    /// // With a closure that returns `Result`.
    /// terminal.try_draw(|frame| -> io::Result<()> {
    ///     let _value: u8 = "42".parse().map_err(io::Error::other)?;
    ///     let area = frame.area();
    ///     frame.render_widget("Hello World!", area);
    ///     frame.set_cursor_position(Position { x: 0, y: 0 });
    ///     Ok(())
    /// })?;
    ///
    /// // Or with a function.
    /// terminal.try_draw(render)?;
    ///
    /// fn render(frame: &mut Frame<'_>) -> io::Result<()> {
    ///     frame.render_widget("Hello World!", frame.area());
    ///     Ok(())
    /// }
    /// # }
    /// # #[cfg(not(feature = "crossterm"))]
    /// # {
    /// # use ratatui_core::{backend::TestBackend, terminal::Terminal};
    /// # let backend = TestBackend::new(10, 10);
    /// # let mut terminal = Terminal::new(backend)?;
    /// # terminal
    /// #     .try_draw(|frame| {
    /// #         frame.render_widget("Hello World!", frame.area());
    /// #         Ok::<(), core::convert::Infallible>(())
    /// #     })
    /// #     ?;
    /// # }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`Backend::flush`]: crate::backend::Backend::flush
    pub fn try_draw<F, E>(&mut self, render_callback: F) -> Result<CompletedFrame<'_>, B::Error>
    where
        F: FnOnce(&mut Frame) -> Result<(), E>,
        E: Into<B::Error>,
    {
        // Autoresize - otherwise we get glitches if shrinking or potential desync between widgets
        // and the terminal (if growing), which may OOB.
        self.autoresize()?;

        let mut frame = self.get_frame();

        render_callback(&mut frame).map_err(Into::into)?;

        let cursor_position = frame.cursor_position;

        self.apply_buffer_with_cursor(cursor_position)
    }

    /// A low-level function that applies and flushes the current buffer to the backend.
    ///
    /// This calls [`Terminal::apply_buffer_with_cursor`] with [`None`], which hides the cursor.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # #![allow(unexpected_cfgs)]
    /// # #[cfg(feature = "crossterm")]
    /// # {
    /// use std::io;
    ///
    /// use ratatui::Terminal;
    /// use ratatui::backend::CrosstermBackend;
    /// use ratatui::buffer::Buffer;
    /// use ratatui::widgets::Widget;
    ///
    /// let backend = CrosstermBackend::new(io::stdout());
    /// let mut terminal = Terminal::new(backend)?;
    ///
    /// terminal.autoresize()?;
    ///
    /// let mut custom_buffer = Buffer::default();
    /// custom_buffer.resize(terminal.get_frame().area());
    /// custom_buffer.reset();
    ///
    /// "Hello World!".render(custom_buffer.area, &mut custom_buffer);
    ///
    /// terminal.current_buffer_mut().merge(&custom_buffer);
    /// terminal.apply_buffer()?;
    /// # }
    /// ```
    pub fn apply_buffer(&mut self) -> Result<CompletedFrame<'_>, B::Error> {
        self.apply_buffer_with_cursor(None)
    }

    /// A low-level function that applies and flushes the current buffer to the backend and
    /// re-positions the cursor. This function is useful if you need to manage your own custom
    /// draw lifecycle and buffer.
    ///
    /// Returns [`Result::Ok`] containing a [`CompletedFrame`] if successful, otherwise
    /// [`Result::Err`] containing the backend error (`B::Error`) that caused the failure.
    ///
    /// This method will:
    ///
    /// - show/hide the cursor based on `cursor_position` ([`None`] will hide the cursor). When a
    ///   position is given, the redundant `Show` + `MoveTo` are skipped only when the cursor is
    ///   provably already at that position and the frame's diff was empty (so the cursor did not
    ///   move). When it does reposition the caret, it emits `MoveTo` and, only if the cursor is
    ///   currently hidden, `Show`
    /// - call [`Terminal::swap_buffers`] to prepare for the next render pass
    /// - call [`Backend::flush`] to flush any buffered backend output
    /// - return a [`CompletedFrame`] with the current buffer and the area used for rendering
    ///
    /// The [`CompletedFrame`] returned by this method can be useful for debugging or testing
    /// purposes, but it is often not used in regular applications.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # #![allow(unexpected_cfgs)]
    /// # #[cfg(feature = "crossterm")]
    /// # {
    /// use std::io;
    ///
    /// use ratatui::Terminal;
    /// use ratatui::backend::CrosstermBackend;
    /// use ratatui::buffer::Buffer;
    /// use ratatui::widgets::Widget;
    ///
    /// let backend = CrosstermBackend::new(io::stdout());
    /// let mut terminal = Terminal::new(backend)?;
    ///
    /// terminal.autoresize()?;
    ///
    /// let mut custom_buffer = Buffer::default();
    /// custom_buffer.resize(terminal.get_frame().area());
    /// custom_buffer.reset();
    ///
    /// "Hello World!".render(custom_buffer.area, &mut custom_buffer);
    ///
    /// terminal.current_buffer_mut().merge(&custom_buffer);
    /// terminal.apply_buffer_with_cursor(None)?;
    /// # }
    /// ```
    pub fn apply_buffer_with_cursor(
        &mut self,
        cursor_position: Option<Position>,
    ) -> Result<CompletedFrame<'_>, B::Error> {
        // Apply the buffer diff to the backend (this is the terminal's "flush" step, distinct
        // from `Backend::flush` below which flushes the backend's output).
        self.flush()?;

        // The cursor position can only be changed after the frame is flushed to stdout.
        match cursor_position {
            None => self.hide_cursor()?,
            Some(position) => {
                // Only emit `Show` + `MoveTo` when the cursor is not already visible at this
                // exact position. Consecutive frames that request an unchanged caret then emit no
                // redundant escape sequences. This avoids repeatedly re-showing the cursor,
                // which some terminals use to re-arm the cursor blink, and trims per-frame
                // output.
                //
                // The cursor can only be skipped when it is provably still at `position`:
                // - the cursor is not hidden (`hidden_cursor`), and
                // - no external operation (`insert_before`, `resize`, direct cursor calls)
                //   invalidated `last_frame_cursor_position` since the last draw, and
                // - this frame's `flush` wrote nothing (`last_flush_had_updates` is `false`).
                //
                // The `flush` condition is essential: on a real terminal, writing any cell
                // advances the physical cursor to just past the cell drawn. So any non-empty diff
                // moves the cursor away from `position`, and the `MoveTo` must be re-emitted to
                // bring the caret back. This is why `last_known_cursor_pos` alone cannot be used:
                // it records the cell coordinate, not the cursor position after drawing it.
                if self.hidden_cursor
                    || self.last_frame_cursor_position != Some(position)
                    || self.last_flush_had_updates
                {
                    // Only emit `Show` when the cursor is actually hidden; when it is already
                    // visible we only need to reposition it with `MoveTo`. Re-showing an already
                    // visible cursor is a redundant escape that some terminals treat as a hint to
                    // re-arm the blink phase.
                    if self.hidden_cursor {
                        self.show_cursor()?;
                    }
                    self.set_cursor_position(position)?;
                }
                self.last_frame_cursor_position = Some(position);
            }
        }

        self.swap_buffers();

        // Flush any buffered backend output.
        self.backend.flush()?;

        let completed_frame = CompletedFrame {
            buffer: &self.buffers[1 - self.current],
            area: self.last_known_area,
            count: self.frame_count,
        };

        // increment frame count before returning from draw
        self.frame_count = self.frame_count.wrapping_add(1);

        Ok(completed_frame)
    }
}

#[cfg(test)]
mod tests {
    use core::fmt;

    use crate::backend::{Backend, ClearType, TestBackend, WindowSize};
    use crate::buffer::{Buffer, Cell};
    use crate::layout::{Position, Rect};
    use crate::terminal::{Terminal, TerminalOptions, Viewport};

    #[derive(Debug, Clone, Eq, PartialEq)]
    struct TestError(&'static str);

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl core::error::Error for TestError {}

    /// A thin wrapper around [`TestBackend`] with a fallible error type.
    ///
    /// [`TestBackend`] uses [`core::convert::Infallible`] as its associated `Backend::Error`, which
    /// is ideal for most tests but makes it impossible to write a `try_draw` callback that returns
    /// an error (because `E: Into<B::Error>` would require converting a real error into
    /// `Infallible`). This wrapper keeps the same observable backend behavior (buffer + cursor)
    /// while allowing tests to exercise `Terminal::try_draw`'s error path.
    #[derive(Debug, Clone, Eq, PartialEq)]
    struct FallibleTestBackend {
        inner: TestBackend,
    }

    impl FallibleTestBackend {
        fn new(inner: TestBackend) -> Self {
            Self { inner }
        }
    }

    impl Backend for FallibleTestBackend {
        type Error = TestError;

        fn draw<'a, I>(&mut self, content: I) -> Result<(), Self::Error>
        where
            I: Iterator<Item = (u16, u16, &'a crate::buffer::Cell)>,
        {
            self.inner.draw(content).map_err(|err| match err {})
        }

        fn append_lines(&mut self, n: u16) -> Result<(), Self::Error> {
            self.inner.append_lines(n).map_err(|err| match err {})
        }

        fn hide_cursor(&mut self) -> Result<(), Self::Error> {
            self.inner.hide_cursor().map_err(|err| match err {})
        }

        fn show_cursor(&mut self) -> Result<(), Self::Error> {
            self.inner.show_cursor().map_err(|err| match err {})
        }

        fn get_cursor_position(&mut self) -> Result<Position, Self::Error> {
            self.inner.get_cursor_position().map_err(|err| match err {})
        }

        fn set_cursor_position<P: Into<Position>>(
            &mut self,
            position: P,
        ) -> Result<(), Self::Error> {
            self.inner
                .set_cursor_position(position)
                .map_err(|err| match err {})
        }

        fn clear(&mut self) -> Result<(), Self::Error> {
            self.inner.clear().map_err(|err| match err {})
        }

        fn clear_region(&mut self, clear_type: ClearType) -> Result<(), Self::Error> {
            self.inner
                .clear_region(clear_type)
                .map_err(|err| match err {})
        }

        fn size(&self) -> Result<crate::layout::Size, Self::Error> {
            self.inner.size().map_err(|err| match err {})
        }

        fn window_size(&mut self) -> Result<WindowSize, Self::Error> {
            self.inner.window_size().map_err(|err| match err {})
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            self.inner.flush().map_err(|err| match err {})
        }

        #[cfg(feature = "scrolling-regions")]
        fn scroll_region_up(
            &mut self,
            region: core::ops::Range<u16>,
            line_count: u16,
        ) -> Result<(), Self::Error> {
            self.inner
                .scroll_region_up(region, line_count)
                .map_err(|err| match err {})
        }

        #[cfg(feature = "scrolling-regions")]
        fn scroll_region_down(
            &mut self,
            region: core::ops::Range<u16>,
            line_count: u16,
        ) -> Result<(), Self::Error> {
            self.inner
                .scroll_region_down(region, line_count)
                .map_err(|err| match err {})
        }
    }

    /// `draw` hides the cursor when the frame does not request a cursor position.
    ///
    /// This asserts the end-to-end effect on the backend (buffer contents + cursor state) as well
    /// as internal frame counting.
    #[test]
    fn draw_hides_cursor_when_frame_cursor_is_not_set() {
        let backend = TestBackend::new(3, 2);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.show_cursor().unwrap();

        let completed = terminal
            .draw(|frame| {
                // Ensure the frame produces updates so `Terminal::flush` writes to the backend.
                frame.buffer_mut()[(0, 0)] = Cell::new("x");
            })
            .unwrap();

        assert_eq!(completed.count, 0, "first draw returns count 0");
        assert_eq!(
            completed.area,
            Rect::new(0, 0, 3, 2),
            "completed area matches terminal size in fullscreen mode"
        );
        assert_eq!(
            completed.buffer,
            &Buffer::with_lines(["x  ", "   "]),
            "completed buffer contains the rendered content"
        );

        assert!(terminal.hidden_cursor);
        assert!(!terminal.backend().cursor_visible());
        assert_eq!(
            terminal.frame_count, 1,
            "successful draw increments frame_count"
        );
    }

    /// `draw` applies the cursor requested by `Frame::set_cursor_position`.
    ///
    /// The cursor is updated after rendering has been flushed, so it appears on top of the drawn
    /// UI.
    #[test]
    fn draw_shows_and_positions_cursor_when_frame_cursor_is_set() {
        let backend = TestBackend::new(3, 2);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.hide_cursor().unwrap();

        terminal
            .draw(|frame| {
                // The cursor is applied after the frame is flushed.
                frame.set_cursor_position(Position { x: 2, y: 1 });
                frame.buffer_mut()[(1, 0)] = Cell::new("y");
            })
            .unwrap();

        assert!(!terminal.hidden_cursor);
        assert!(terminal.backend().cursor_visible());
        assert_eq!(
            terminal.backend().cursor_position(),
            Position { x: 2, y: 1 },
            "backend cursor is positioned after flushing"
        );
        assert_eq!(
            terminal.last_known_cursor_pos,
            Position { x: 2, y: 1 },
            "terminal cursor tracking matches the final cursor position"
        );
    }

    /// When the render callback returns an error, `try_draw` does not update the terminal.
    ///
    /// This is a characterization of the "no partial updates" behavior: backend contents and
    /// cursor state are unchanged and `frame_count` does not advance.
    #[test]
    fn try_draw_propagates_render_errors_without_updating_backend() {
        let backend = FallibleTestBackend::new(TestBackend::with_lines(["aaa", "bbb"]));
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.show_cursor().unwrap();

        let was_hidden = terminal.hidden_cursor;
        let cursor_visible = terminal.backend().inner.cursor_visible();
        let cursor_position = terminal.backend().inner.cursor_position();

        let result = terminal.try_draw(|_frame| Err::<(), _>(TestError("render failed")));

        assert_eq!(
            result.unwrap_err(),
            TestError("render failed"),
            "try_draw returns the render callback error"
        );

        assert_eq!(terminal.frame_count, 0, "frame_count is unchanged on error");
        assert_eq!(
            terminal.backend().inner.buffer(),
            &Buffer::with_lines(["aaa", "bbb"]),
            "backend buffer is unchanged on error"
        );
        assert_eq!(
            terminal.hidden_cursor, was_hidden,
            "terminal cursor state is unchanged on error"
        );
        assert_eq!(
            terminal.backend().inner.cursor_visible(),
            cursor_visible,
            "backend cursor visibility is unchanged on error"
        );
        assert_eq!(
            terminal.backend().inner.cursor_position(),
            cursor_position,
            "backend cursor position is unchanged on error"
        );
    }

    /// `draw` autoresizes fullscreen terminals and clears before rendering.
    ///
    /// This simulates the backend resizing between draw calls; `draw` runs `autoresize()` first
    /// (which calls `resize()` and clears) so the frame renders into a fresh, correctly-sized
    /// region.
    #[test]
    fn draw_clears_on_fullscreen_resize_before_rendering() {
        let backend = TestBackend::with_lines(["xxx", "yyy"]);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.backend_mut().resize(4, 3);

        terminal
            .draw(|frame| {
                // Render a marker to show we rendered after the clear.
                frame.buffer_mut()[(0, 0)] = Cell::new("x");
            })
            .unwrap();

        assert_eq!(
            terminal.viewport_area,
            Rect::new(0, 0, 4, 3),
            "viewport area tracks the resized terminal size"
        );
        assert_eq!(
            terminal.last_known_area,
            Rect::new(0, 0, 4, 3),
            "last_known_area tracks the resized terminal size"
        );
        terminal
            .backend()
            .assert_buffer_lines(["x   ", "    ", "    "]);
    }

    /// In fixed viewports, `Frame::area` is an absolute terminal rectangle.
    ///
    /// This asserts that rendering at `frame.area().x/y` updates the backend at that absolute
    /// position.
    #[test]
    fn draw_uses_fixed_viewport_coordinates() {
        let backend = TestBackend::new(5, 3);
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Fixed(Rect::new(2, 1, 2, 1)),
            },
        )
        .unwrap();

        terminal
            .draw(|frame| {
                assert_eq!(
                    frame.area(),
                    Rect::new(2, 1, 2, 1),
                    "frame area matches the configured fixed viewport"
                );
                let area = frame.area();
                frame.buffer_mut()[(area.x, area.y)] = Cell::new("z");
            })
            .unwrap();

        terminal
            .backend()
            .assert_buffer_lines(["     ", "  z  ", "     "]);
    }

    /// Inline viewports render into a sub-rectangle, but `CompletedFrame::area` reports terminal
    /// size.
    ///
    /// This asserts that the `CompletedFrame` returned from `draw` reports the full terminal
    /// size while its buffer is sized to the inline viewport, and that rendering uses the inline
    /// viewport's absolute origin.
    #[test]
    fn draw_inline_completed_frame_reports_terminal_size() {
        let mut inner = TestBackend::new(6, 5);
        inner.set_cursor_position((0, 2)).unwrap();
        let mut terminal = Terminal::with_options(
            inner,
            TerminalOptions {
                viewport: Viewport::Inline(3),
            },
        )
        .unwrap();

        let viewport_area = terminal.viewport_area;
        {
            // `CompletedFrame` borrows the terminal, so backend assertions happen after it drops.
            let completed = terminal
                .draw(|frame| {
                    assert_eq!(
                        frame.area(),
                        viewport_area,
                        "inline frame area matches the computed viewport"
                    );
                    frame.buffer_mut()[(viewport_area.x, viewport_area.y)] = Cell::new("i");
                })
                .unwrap();

            assert_eq!(
                completed.area,
                Rect::new(0, 0, 6, 5),
                "completed area reports the full terminal size"
            );
            assert_eq!(
                completed.buffer.area, viewport_area,
                "completed buffer is sized to the inline viewport"
            );
        }

        assert_eq!(
            terminal.backend().buffer()[(viewport_area.x, viewport_area.y)].symbol(),
            "i"
        );
    }

    /// Inline viewports are autoresized during `draw`.
    ///
    /// This asserts that when the backend reports a different terminal size, `draw` recomputes the
    /// inline viewport rectangle and renders into the new viewport area.
    #[test]
    fn draw_inline_autoresize_recomputes_viewport_on_grow() {
        let mut backend = TestBackend::new(6, 5);
        backend
            .set_cursor_position(Position { x: 0, y: 2 })
            .unwrap();
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(3),
            },
        )
        .unwrap();

        terminal
            .draw(|frame| {
                let area = frame.area();
                frame.set_cursor_position(Position {
                    x: area.x,
                    y: area.y.saturating_add(1),
                });
                frame.buffer_mut()[(area.x, area.y)] = Cell::new("a");
            })
            .unwrap();

        terminal.backend_mut().resize(8, 7);
        let new_area = Rect::new(0, 0, 8, 7);

        let previous_viewport = terminal.viewport_area;
        terminal
            .draw(|frame| {
                let area = frame.area();
                frame.buffer_mut()[(area.x, area.y)] = Cell::new("g");
            })
            .unwrap();

        assert_eq!(
            terminal.last_known_area, new_area,
            "inline last_known_area tracks the resized terminal size"
        );
        assert_eq!(
            terminal.viewport_area.width, 8,
            "inline viewport width tracks the resized terminal width"
        );
        assert_eq!(
            terminal.viewport_area.height, 3,
            "inline viewport height is capped by the configured inline height"
        );
        assert_eq!(
            terminal.viewport_area.y, previous_viewport.y,
            "inline viewport stays anchored relative to the cursor across a grow"
        );
        assert_eq!(
            terminal.backend().buffer()[(terminal.viewport_area.x, terminal.viewport_area.y)]
                .symbol(),
            "g",
            "render output lands at the recomputed viewport origin"
        );
    }

    /// Inline viewports are autoresized during `draw`.
    ///
    /// This asserts that shrinking the backend terminal size causes `draw` to recompute the inline
    /// viewport origin so it stays visible, and that rendering uses the new viewport origin.
    #[test]
    fn draw_inline_autoresize_recomputes_viewport_on_shrink() {
        let mut backend = TestBackend::new(6, 6);
        backend
            .set_cursor_position(Position { x: 0, y: 4 })
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
                frame.set_cursor_position(Position {
                    x: area.x,
                    y: area.y.saturating_add(2),
                });
                frame.buffer_mut()[(area.x, area.y)] = Cell::new("a");
            })
            .unwrap();

        terminal.backend_mut().resize(6, 5);
        let new_area = Rect::new(0, 0, 6, 5);

        terminal
            .draw(|frame| {
                let area = frame.area();
                frame.buffer_mut()[(area.x, area.y)] = Cell::new("s");
            })
            .unwrap();

        assert_eq!(
            terminal.last_known_area, new_area,
            "inline last_known_area tracks the resized terminal size"
        );
        assert_eq!(
            terminal.viewport_area,
            Rect::new(0, 1, 6, 4),
            "inline viewport is recomputed to stay visible after a shrink"
        );
        assert_eq!(
            terminal.backend().buffer()[(terminal.viewport_area.x, terminal.viewport_area.y)]
                .symbol(),
            "s",
            "render output lands at the recomputed viewport origin"
        );
    }

    /// `CompletedFrame` is only valid until the next draw call.
    ///
    /// This asserts that each `draw` returns the buffer for the frame that was just rendered
    /// and that the count increments after each successful draw.
    #[test]
    fn draw_returns_completed_frame_for_current_render_pass() {
        let backend = TestBackend::new(3, 2);
        let mut terminal = Terminal::new(backend).unwrap();

        {
            // `CompletedFrame` borrows the terminal, and is only valid until the next draw call.
            let first = terminal
                .draw(|frame| {
                    frame.buffer_mut()[(0, 0)] = Cell::new("a");
                })
                .unwrap();

            assert_eq!(first.count, 0, "first CompletedFrame has count 0");
            assert_eq!(
                first.buffer,
                &Buffer::with_lines(["a  ", "   "]),
                "first frame's buffer contains the first render output"
            );
        }

        let second = terminal
            .draw(|frame| {
                frame.buffer_mut()[(0, 0)] = Cell::new("b");
            })
            .unwrap();

        assert_eq!(second.count, 1, "second CompletedFrame has count 1");
        assert_eq!(
            second.buffer,
            &Buffer::with_lines(["b  ", "   "]),
            "second frame's buffer contains the second render output"
        );
    }

    #[test]
    fn apply_buffer_hides_cursor() {
        let backend = TestBackend::new(3, 2);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.show_cursor().unwrap();
        terminal.autoresize().unwrap();

        let mut external_buffer = Buffer::default();
        external_buffer.resize(terminal.get_frame().area());
        external_buffer[(0, 0)] = Cell::new("b");

        terminal.current_buffer_mut().merge(&external_buffer);
        let completed = terminal.apply_buffer().unwrap();

        assert_eq!(completed.count, 0, "first draw returns count 0");
        assert_eq!(
            completed.area,
            Rect::new(0, 0, 3, 2),
            "completed area matches terminal size in fullscreen mode"
        );
        assert_eq!(
            completed.buffer,
            &Buffer::with_lines(["b  ", "   "]),
            "completed buffer contains the rendered content"
        );

        assert!(terminal.hidden_cursor);
        assert!(!terminal.backend().cursor_visible());
        assert_eq!(
            terminal.frame_count, 1,
            "successful draw increments frame_count"
        );
    }

    /// A [`TestBackend`] wrapper that records cursor operations and simulates real-terminal cursor
    /// behavior so tests can assert both that redundant `Show`/`MoveTo` sequences are skipped and
    /// that the caret ends at the requested position.
    ///
    /// Like a real terminal, writing cells with [`Backend::draw`] advances the logical cursor to
    /// just past the last cell written, and [`Backend::set_cursor_position`] moves it to the
    /// requested position.
    #[derive(Debug)]
    struct RecordingCursorBackend {
        inner: TestBackend,
        hide_cursor_calls: usize,
        show_cursor_calls: usize,
        set_cursor_position_calls: usize,
        /// Simulated physical cursor position, advanced by `draw` like a real terminal.
        simulated_cursor: Position,
        /// Simulated cursor visibility, toggled by `show_cursor`/`hide_cursor`.
        simulated_visible: bool,
    }

    impl RecordingCursorBackend {
        fn new(inner: TestBackend) -> Self {
            Self {
                inner,
                hide_cursor_calls: 0,
                show_cursor_calls: 0,
                set_cursor_position_calls: 0,
                simulated_cursor: Position::ORIGIN,
                simulated_visible: true,
            }
        }
    }

    impl Backend for RecordingCursorBackend {
        type Error = TestError;

        fn draw<'a, I>(&mut self, content: I) -> Result<(), Self::Error>
        where
            I: Iterator<Item = (u16, u16, &'a crate::buffer::Cell)>,
        {
            // Consume the iterator, tracking the last cell written so we can simulate the
            // terminal cursor advancing just past it (like a real terminal after printing a
            // cell), then forward the collected cells to TestBackend.
            let mut last = None;
            let mut cells = alloc::vec::Vec::new();
            for (x, y, cell) in content {
                // The test cells are single-width ASCII, so a `Print` advances the terminal
                // cursor by one column. Real multi-width glyphs advance by more.
                last = Some(Position {
                    x: x.saturating_add(1),
                    y,
                });
                cells.push((x, y, cell));
            }
            if let Some(pos) = last {
                self.simulated_cursor = pos;
            }
            self.inner
                .draw(cells.into_iter())
                .map_err(|err| match err {})
        }

        fn append_lines(&mut self, n: u16) -> Result<(), Self::Error> {
            self.inner.append_lines(n).map_err(|err| match err {})
        }

        fn hide_cursor(&mut self) -> Result<(), Self::Error> {
            self.hide_cursor_calls += 1;
            self.simulated_visible = false;
            self.inner.hide_cursor().map_err(|err| match err {})
        }

        fn show_cursor(&mut self) -> Result<(), Self::Error> {
            self.show_cursor_calls += 1;
            self.simulated_visible = true;
            self.inner.show_cursor().map_err(|err| match err {})
        }

        fn get_cursor_position(&mut self) -> Result<Position, Self::Error> {
            // Return the simulated cursor so the wrapper is internally consistent: a real
            // terminal query reflects the same cursor state that `set_cursor_position`/`draw`
            // maintain.
            Ok(self.simulated_cursor)
        }

        fn set_cursor_position<P: Into<Position>>(
            &mut self,
            position: P,
        ) -> Result<(), Self::Error> {
            let position = position.into();
            self.set_cursor_position_calls += 1;
            self.simulated_cursor = position;
            self.inner
                .set_cursor_position(position)
                .map_err(|err| match err {})
        }

        fn clear(&mut self) -> Result<(), Self::Error> {
            self.inner.clear().map_err(|err| match err {})
        }

        fn clear_region(&mut self, clear_type: ClearType) -> Result<(), Self::Error> {
            self.inner
                .clear_region(clear_type)
                .map_err(|err| match err {})
        }

        fn size(&self) -> Result<crate::layout::Size, Self::Error> {
            self.inner.size().map_err(|err| match err {})
        }

        fn window_size(&mut self) -> Result<WindowSize, Self::Error> {
            self.inner.window_size().map_err(|err| match err {})
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            self.inner.flush().map_err(|err| match err {})
        }

        #[cfg(feature = "scrolling-regions")]
        fn scroll_region_up(
            &mut self,
            region: core::ops::Range<u16>,
            line_count: u16,
        ) -> Result<(), Self::Error> {
            self.inner
                .scroll_region_up(region, line_count)
                .map_err(|err| match err {})
        }

        #[cfg(feature = "scrolling-regions")]
        fn scroll_region_down(
            &mut self,
            region: core::ops::Range<u16>,
            line_count: u16,
        ) -> Result<(), Self::Error> {
            self.inner
                .scroll_region_down(region, line_count)
                .map_err(|err| match err {})
        }
    }


    /// Consecutive frames that request an unchanged cursor position must not re-emit `Show` +
    /// `MoveTo` when nothing changed on screen, while a changed position, changed content, or a
    /// hidden cursor must.
    ///
    /// On a real terminal, drawing a diff advances the cursor just past the last cell written, so
    /// the `MoveTo` may only be skipped when the physical cursor provably remains at the requested
    /// position, which requires the frame's diff to be empty.
    #[test]
    fn draw_skips_redundant_cursor_emission_when_position_unchanged() {
        let backend = RecordingCursorBackend::new(TestBackend::new(3, 2));
        let mut terminal = Terminal::new(backend).unwrap();

        // First frame places the caret at (2, 1); the cursor is already visible, so only a
        // `MoveTo` is emitted, not a redundant `Show`.
        terminal
            .draw(|frame| {
                frame.set_cursor_position(Position { x: 2, y: 1 });
                frame.buffer_mut()[(0, 0)] = Cell::new("x");
            })
            .unwrap();
        assert_eq!(
            terminal.backend().show_cursor_calls,
            0,
            "first frame with a visible cursor must not emit a redundant Show"
        );
        assert_eq!(
            terminal.backend().set_cursor_position_calls,
            1,
            "first frame moves the cursor"
        );

        // A second frame at the same caret with no cell changes (redrawing the same "x" content)
        // must not re-emit `Show`/`MoveTo`: the diff is empty so the physical cursor stays
        // at (2, 1).
        terminal
            .draw(|frame| {
                frame.set_cursor_position(Position { x: 2, y: 1 });
                frame.buffer_mut()[(0, 0)] = Cell::new("x");
            })
            .unwrap();
        assert_eq!(
            terminal.backend().show_cursor_calls,
            0,
            "unchanged caret with empty diff must not re-show the cursor"
        );
        assert_eq!(
            terminal.backend().set_cursor_position_calls,
            1,
            "unchanged caret with empty diff must not re-MoveTo the cursor"
        );

        // A third frame at the same caret but with changed content must re-emit `MoveTo`: drawing
        // the diff leaves the terminal cursor on the changed cell, not at the caret.
        terminal
            .draw(|frame| {
                frame.set_cursor_position(Position { x: 2, y: 1 });
                frame.buffer_mut()[(0, 0)] = Cell::new("y");
            })
            .unwrap();
        assert_eq!(
            terminal.backend().set_cursor_position_calls,
            2,
            "an unchanged caret with a changed cell must re-MoveTo the cursor"
        );

        // A moved caret must emit.
        terminal
            .draw(|frame| {
                frame.set_cursor_position(Position { x: 1, y: 1 });
            })
            .unwrap();
        assert_eq!(
            terminal.backend().set_cursor_position_calls,
            3,
            "a moved caret must re-emit MoveTo"
        );
    }

    /// On a real terminal the cursor follows the last cell drawn by the frame's flush. When a
    /// frame changes content but keeps the caret unchanged, the `MoveTo` must still be emitted so
    /// the physical cursor ends at the caret rather than stranded on a changed cell.
    #[test]
    fn content_change_keeps_caret_positioned_at_requested_spot() {
        let backend = RecordingCursorBackend::new(TestBackend::new(3, 2));
        let mut terminal = Terminal::new(backend).unwrap();

        // First frame places the caret at (2, 1).
        terminal
            .draw(|frame| {
                frame.set_cursor_position(Position { x: 2, y: 1 });
                frame.buffer_mut()[(0, 0)] = Cell::new("x");
            })
            .unwrap();
        assert_eq!(
            terminal.backend().simulated_cursor,
            Position { x: 2, y: 1 },
            "after the first frame the cursor must be at the caret"
        );

        // Second frame changes a cell but keeps the caret unchanged. The flush draws the changed
        // cell, which advances the terminal cursor just past it (to (1, 0) in the simulated real
        // terminal); the guard must re-emit the `MoveTo` so the caret ends back at (2, 1).
        terminal
            .draw(|frame| {
                frame.set_cursor_position(Position { x: 2, y: 1 });
                frame.buffer_mut()[(0, 0)] = Cell::new("y");
            })
            .unwrap();
        assert_eq!(
            terminal.backend().simulated_cursor,
            Position { x: 2, y: 1 },
            "after a content change the cursor must still be at the caret"
        );
        assert_eq!(
            terminal.get_cursor_position().unwrap(),
            Position { x: 2, y: 1 },
            "querying the cursor via the public API reflects the caret after a content change"
        );
    }

    /// Any non-empty frame diff (even one whose last written cell is at the caret) must re-emit
    /// `MoveTo`. On a real terminal, writing a cell advances the physical cursor to just past that
    /// cell, so it is never exactly at the caret position after a non-empty `flush`.
    #[test]
    fn draw_reemits_when_diff_is_nonempty_even_if_it_ends_at_caret() {
        let backend = RecordingCursorBackend::new(TestBackend::new(3, 2));
        let mut terminal = Terminal::new(backend).unwrap();

        // First frame places the caret at (1, 0) and writes a cell there too.
        terminal
            .draw(|frame| {
                frame.set_cursor_position(Position { x: 1, y: 0 });
                frame.buffer_mut()[(1, 0)] = Cell::new("x");
            })
            .unwrap();
        assert_eq!(terminal.backend().set_cursor_position_calls, 1);

        // Second frame keeps the caret at (1, 0) and changes two cells, the last of which is the
        // caret cell. Because the diff is non-empty, the `MoveTo` must be re-emitted to bring the
        // caret back to (1, 0).
        terminal
            .draw(|frame| {
                frame.set_cursor_position(Position { x: 1, y: 0 });
                frame.buffer_mut()[(0, 0)] = Cell::new("y");
                frame.buffer_mut()[(1, 0)] = Cell::new("z");
            })
            .unwrap();
        assert_eq!(
            terminal.backend().set_cursor_position_calls,
            2,
            "a non-empty diff must re-MoveTo even if it ends at the caret"
        );
        assert_eq!(
            terminal.backend().simulated_cursor,
            Position { x: 1, y: 0 },
            "the simulated cursor is brought back to the caret after a non-empty diff"
        );
    }

    /// A hidden cursor followed by a shown frame must still re-emit `Show` + `MoveTo`, even if the
    /// requested position matches the stale hidden-frame tracking.
    #[test]
    fn draw_emits_cursor_when_transitioning_from_hidden_to_shown() {
        let backend = RecordingCursorBackend::new(TestBackend::new(3, 2));
        let mut terminal = Terminal::new(backend).unwrap();

        // First frame hides the cursor.
        terminal
            .draw(|frame| {
                frame.buffer_mut()[(0, 0)] = Cell::new("x");
            })
            .unwrap();
        assert_eq!(
            terminal.backend().hide_cursor_calls,
            1,
            "a frame without a cursor hides the cursor"
        );
        assert!(terminal.hidden_cursor);
        assert!(
            !terminal.backend().simulated_visible,
            "the simulated cursor is hidden after the hide frame"
        );

        // A shown frame at a position must re-emit `Show` + `MoveTo` even though it is the first
        // shown caret. The `Some` position differs from the stale `None` (and `hidden_cursor` is
        // true), so both guards cooperate to force the emission.
        terminal
            .draw(|frame| {
                frame.set_cursor_position(Position { x: 2, y: 1 });
            })
            .unwrap();
        assert_eq!(
            terminal.backend().show_cursor_calls,
            1,
            "shown-after-hidden must re-show the cursor"
        );
        assert_eq!(
            terminal.backend().set_cursor_position_calls,
            1,
            "shown-after-hidden must re-MoveTo the cursor"
        );
        assert!(
            terminal.backend().simulated_visible,
            "the simulated cursor is visible again after the shown frame"
        );
    }

    /// A direct [`Terminal::show_cursor`] call only changes visibility, not position, so the
    /// dedup tracking is unaffected and a following draw at the same position skips re-emitting
    /// `Show` + `MoveTo`.
    #[test]
    fn draw_skips_cursor_after_direct_show_cursor() {
        let backend = RecordingCursorBackend::new(TestBackend::new(3, 2));
        let mut terminal = Terminal::new(backend).unwrap();

        // One draw places a visible caret at (2, 1); the cursor is already visible, so only a
        // `MoveTo` is emitted, not a redundant `Show`.
        terminal
            .draw(|frame| {
                frame.set_cursor_position(Position { x: 2, y: 1 });
            })
            .unwrap();
        assert_eq!(terminal.backend().show_cursor_calls, 0);
        assert_eq!(terminal.backend().set_cursor_position_calls, 1);

        // Re-showing directly touches the backend once (counter -> 1) but doesn't change the
        // position, so the next draw at the same spot must not emit any further `Show` or
        // `MoveTo`.
        terminal.show_cursor().unwrap();
        assert_eq!(terminal.backend().show_cursor_calls, 1);
        terminal
            .draw(|frame| {
                frame.set_cursor_position(Position { x: 2, y: 1 });
            })
            .unwrap();
        assert_eq!(
            terminal.backend().show_cursor_calls,
            1,
            "a draw after a direct show_cursor at the same spot must not re-show the cursor"
        );
        assert_eq!(
            terminal.backend().set_cursor_position_calls,
            1,
            "a draw after a direct show_cursor at the same spot must not re-MoveTo the cursor"
        );
        assert_eq!(
            terminal.backend().simulated_cursor,
            Position { x: 2, y: 1 },
            "a direct show_cursor does not move the simulated cursor"
        );
    }

    /// A direct [`Terminal::set_cursor_position`] call keeps the dedup tracking accurate, so a
    /// following draw at the same position must not re-emit `MoveTo`.
    #[test]
    fn draw_skips_moveto_after_direct_set_cursor_position() {
        let backend = RecordingCursorBackend::new(TestBackend::new(3, 2));
        let mut terminal = Terminal::new(backend).unwrap();

        // Position the cursor directly at (2, 1).
        terminal
            .set_cursor_position(Position { x: 2, y: 1 })
            .unwrap();

        // A draw requesting the same position must not re-emit `MoveTo`.
        terminal
            .draw(|frame| {
                frame.set_cursor_position(Position { x: 2, y: 1 });
            })
            .unwrap();
        assert_eq!(
            terminal.backend().set_cursor_position_calls,
            1,
            "a draw after a direct set_cursor_position at the same spot must not re-MoveTo"
        );
        assert_eq!(
            terminal.backend().simulated_cursor,
            Position { x: 2, y: 1 },
            "the simulated cursor stays where the direct set_cursor_position placed it"
        );
    }
}
