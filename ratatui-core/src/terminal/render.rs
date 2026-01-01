use crate::backend::Backend;
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
    /// - call [`Terminal::flush`] to write changes to the backend
    /// - show/hide the cursor based on [`Frame::set_cursor_position`]
    /// - call [`Terminal::swap_buffers`] to prepare for the next render pass
    /// - call [`Backend::flush`]
    /// - return a [`CompletedFrame`] with the current buffer and the area used for rendering
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
    /// - call [`Terminal::flush`] to write changes to the backend
    /// - show/hide the cursor based on [`Frame::set_cursor_position`]
    /// - call [`Terminal::swap_buffers`] to prepare for the next render pass
    /// - call [`Backend::flush`]
    /// - return a [`CompletedFrame`] with the current buffer and the area used for rendering
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

        // We can't change the cursor position right away because we have to flush the frame to
        // stdout first. But we also can't keep the frame around, since it holds a &mut to
        // Buffer. Thus, we're taking the important data out of the Frame and dropping it.
        let cursor_position = frame.cursor_position;

        // Apply the buffer diff to the backend (this is the terminal's "flush" step, distinct
        // from `Backend::flush` below which flushes the backend's output).
        self.flush()?;

        match cursor_position {
            None => self.hide_cursor()?,
            Some(position) => {
                self.show_cursor()?;
                self.set_cursor_position(position)?;
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
}
