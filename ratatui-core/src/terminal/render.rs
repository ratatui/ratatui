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

        // Draw to stdout
        self.flush()?;

        match cursor_position {
            None => self.hide_cursor()?,
            Some(position) => {
                self.show_cursor()?;
                self.set_cursor_position(position)?;
            }
        }

        self.swap_buffers();

        // Flush
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
