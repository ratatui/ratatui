use crate::backend::{Backend, ClearType};
use crate::buffer::{Buffer, Cell};
use crate::layout::{Position, Rect, Size};
use crate::terminal::{CompletedFrame, Frame, TerminalOptions, Viewport};

/// An interface to interact and draw [`Frame`]s on the user's terminal.
///
/// This is the main entry point for Ratatui. It is responsible for drawing and maintaining the
/// state of the buffers, cursor and viewport.
///
/// If you're building a fullscreen application with the `ratatui` crate's default backend
/// ([Crossterm]), prefer [`ratatui::run`] (or [`ratatui::init`] + [`ratatui::restore`]) over
/// constructing `Terminal` directly. These helpers enable common terminal modes (raw mode +
/// alternate screen) and restore them on exit and on panic.
///
/// ```rust,ignore
/// ratatui::run(|terminal| {
///     let mut should_quit = false;
///     while !should_quit {
///         terminal.draw(|frame| {
///             frame.render_widget("Hello, World!", frame.area());
///         })?;
///
///         // Handle events, update application state, and set `should_quit = true` to exit.
///     }
///     Ok(())
/// })?;
/// ```
///
/// # Typical Usage
///
/// In a typical application, the flow is: set up a terminal, run an event loop, update state, and
/// draw each frame.
///
/// 1. Choose a setup path for a `Terminal`. Most apps call [`ratatui::run`], which passes a
///    preconfigured `Terminal` into your callback. If you need more control, use [`ratatui::init`]
///    and [`ratatui::restore`], or construct a `Terminal` manually via [`Terminal::new`]
///    (fullscreen) or [`Terminal::with_options`] (select a [`Viewport`]).
/// 2. Enter your application's event loop and call [`Terminal::draw`] (or [`Terminal::try_draw`])
///    to render the current UI state into a [`Frame`].
/// 3. Handle input and application state updates between draw calls.
/// 4. If the terminal is resized, call [`Terminal::draw`] again. Ratatui automatically resizes
///    fullscreen and inline viewports during `draw`; fixed viewports require an explicit call to
///    [`Terminal::resize`] if you want the region to change.
///
/// # Rendering Pipeline
///
/// A single call to [`Terminal::draw`] (or [`Terminal::try_draw`]) represents one render pass. In
/// broad strokes, Ratatui:
///
/// 1. Checks whether the underlying terminal size changed (see [`Terminal::autoresize`]).
/// 2. Creates a [`Frame`] backed by the current buffer (see [`Terminal::get_frame`]).
/// 3. Runs your render callback to populate that buffer.
/// 4. Diffs the current buffer against the previous buffer and writes the changes (see
///    [`Terminal::flush`]).
/// 5. Applies cursor visibility and position requested by the frame (see
///    [`Frame::set_cursor_position`]).
/// 6. Swaps the buffers to prepare for the next render pass (see [`Terminal::swap_buffers`]).
/// 7. Flushes the backend (see [`Backend::flush`]).
///
/// Each render pass starts with an empty buffer for the current viewport. Your render callback
/// should render everything that should be visible in [`Frame::area`], even if it is unchanged
/// from the previous frame. Ratatui diffs the current and previous buffers and only writes the
/// changes; anything you don't render is treated as empty and may clear previously drawn content.
///
/// If the viewport size changes between render passes (for example via [`Terminal::autoresize`] or
/// an explicit [`Terminal::resize`]), Ratatui clears the viewport and resets the previous buffer so
/// the next `draw` is treated as a full redraw.
///
/// Most applications should use [`Terminal::draw`] / [`Terminal::try_draw`]. For manual rendering
/// (primarily for tests), you can build a frame with [`Terminal::get_frame`], write diffs with
/// [`Terminal::flush`], then call [`Terminal::swap_buffers`]. If your backend buffers output, also
/// call [`Backend::flush`].
///
/// ```rust,no_run
/// # mod ratatui {
/// #     pub use ratatui_core::backend;
/// #     pub use ratatui_core::terminal::Terminal;
/// # }
/// use ratatui::Terminal;
/// use ratatui::backend::{Backend, TestBackend};
///
/// let backend = TestBackend::new(10, 10);
/// let mut terminal = Terminal::new(backend)?;
///
/// // Manual render pass (roughly what `Terminal::draw` does internally).
/// {
///     let mut frame = terminal.get_frame();
///     frame.render_widget("Hello World!", frame.area());
/// }
///
/// terminal.flush()?;
/// terminal.swap_buffers();
/// terminal.backend_mut().flush()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Viewports
///
/// The viewport controls *where* Ratatui draws and therefore what [`Frame::area`] represents.
/// Most applications use [`Viewport::Fullscreen`], but Ratatui also supports [`Viewport::Inline`]
/// and [`Viewport::Fixed`].
///
/// Choose a viewport at initialization time with [`Terminal::with_options`] and
/// [`TerminalOptions`].
///
/// In [`Viewport::Fullscreen`], the viewport is the entire terminal and `Frame::area` starts at
/// (0, 0). Ratatui automatically resizes the internal buffers when the terminal size changes.
///
/// In [`Viewport::Fixed`], the viewport is a user-provided [`Rect`] in terminal coordinates.
/// `Frame::area` is that exact rectangle (including its `x`/`y` offset). Fixed viewports are not
/// automatically resized; if the region should change, call [`Terminal::resize`].
///
/// In [`Viewport::Inline`], Ratatui draws into a rectangle anchored to where the UI started. This
/// mode is described in more detail in the "Inline Viewport" section below.
///
/// ```rust,ignore
/// use ratatui::{layout::Rect, Terminal, TerminalOptions, Viewport};
/// use ratatui::backend::CrosstermBackend;
///
/// // Fullscreen (most common):
/// let fullscreen = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
///
/// // Fixed region (your app manages the coordinates):
/// let viewport = Viewport::Fixed(Rect::new(0, 0, 30, 10));
/// let fixed = Terminal::with_options(
///     CrosstermBackend::new(std::io::stdout()),
///     TerminalOptions { viewport },
/// )?;
/// ```
///
/// Applications should detect terminal resizes and call [`Terminal::draw`] to redraw the
/// application with the new size. This will automatically resize the internal buffers to match the
/// new size for inline and fullscreen viewports. Fixed viewports are not resized automatically.
///
/// # Initialization
///
/// For most applications, consider using the convenience functions `ratatui::run()`,
/// `ratatui::init()`, and `ratatui::restore()` (available since version 0.28.1) along with the
/// `DefaultTerminal` type alias instead of constructing `Terminal` instances manually. These
/// functions handle the common setup and teardown tasks automatically. Manual construction
/// using `Terminal::new()` or `Terminal::with_options()` is still supported for applications
/// that need fine-grained control over initialization.
///
/// # Examples
///
/// ## Using convenience functions (recommended for most applications)
///
/// ```rust,ignore
/// // Modern approach using convenience functions
/// ratatui::run(|terminal| {
///     terminal.draw(|frame| {
///         let area = frame.area();
///         frame.render_widget(Paragraph::new("Hello World!"), area);
///     })?;
///     Ok(())
/// })?;
/// ```
///
/// ## Manual construction (for fine-grained control)
///
/// ```rust,ignore
/// use std::io::stdout;
///
/// use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};
///
/// let backend = CrosstermBackend::new(stdout());
/// let mut terminal = Terminal::new(backend)?;
/// terminal.draw(|frame| {
///     let area = frame.area();
///     frame.render_widget(Paragraph::new("Hello World!"), area);
/// })?;
/// # std::io::Result::Ok(())
/// ```
///
/// [Crossterm]: https://crates.io/crates/crossterm
/// [Termion]: https://crates.io/crates/termion
/// [Termwiz]: https://crates.io/crates/termwiz
/// [`backend`]: crate::backend
/// [`Backend`]: crate::backend::Backend
/// [`Buffer`]: crate::buffer::Buffer
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Terminal<B>
where
    B: Backend,
{
    /// The backend used to interface with the terminal
    backend: B,
    /// Holds the results of the current and previous draw calls. The two are compared at the end
    /// of each draw pass to output the necessary updates to the terminal
    buffers: [Buffer; 2],
    /// Index of the current buffer in the previous array
    current: usize,
    /// Whether the cursor is currently hidden
    hidden_cursor: bool,
    /// Viewport
    viewport: Viewport,
    /// Area of the viewport
    viewport_area: Rect,
    /// Last known area of the terminal. Used to detect if the internal buffers have to be resized.
    last_known_area: Rect,
    /// Last known position of the cursor. Used to find the new area when the viewport is inlined
    /// and the terminal resized.
    last_known_cursor_pos: Position,
    /// Number of frames rendered up until current time.
    frame_count: usize,
}

/// Options to pass to [`Terminal::with_options`]
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Options {
    /// Viewport used to draw to the terminal
    pub viewport: Viewport,
}

impl<B> Drop for Terminal<B>
where
    B: Backend,
{
    fn drop(&mut self) {
        // Attempt to restore the cursor state
        if self.hidden_cursor {
            #[allow(unused_variables)]
            if let Err(err) = self.show_cursor() {
                #[cfg(feature = "std")]
                std::eprintln!("Failed to show the cursor: {err}");
            }
        }
    }
}

impl<B> Terminal<B>
where
    B: Backend,
{
    /// Creates a new [`Terminal`] with the given [`Backend`] with a full screen viewport.
    ///
    /// Note that unlike `ratatui::init`, this does not install a panic hook, so it is recommended
    /// to do that manually when using this function, otherwise any panic messages will be printed
    /// to the alternate screen and the terminal may be left in an unusable state.
    ///
    /// See [how to set up panic hooks](https://ratatui.rs/recipes/apps/panic-hooks/) and
    /// [`better-panic` example](https://ratatui.rs/recipes/apps/better-panic/) for more
    /// information.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use std::io::stdout;
    ///
    /// use ratatui::{backend::CrosstermBackend, Terminal};
    ///
    /// let backend = CrosstermBackend::new(stdout());
    /// let terminal = Terminal::new(backend)?;
    ///
    /// // Optionally set up a panic hook to restore the terminal on panic.
    /// let old_hook = std::panic::take_hook();
    /// std::panic::set_hook(Box::new(move |info| {
    ///     ratatui::restore();
    ///     old_hook(info);
    /// }));
    /// # std::io::Result::Ok(())
    /// ```
    pub fn new(backend: B) -> Result<Self, B::Error> {
        Self::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Fullscreen,
            },
        )
    }

    /// Creates a new [`Terminal`] with the given [`Backend`] and [`TerminalOptions`].
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use std::io::stdout;
    ///
    /// use ratatui::{backend::CrosstermBackend, layout::Rect, Terminal, TerminalOptions, Viewport};
    ///
    /// let backend = CrosstermBackend::new(stdout());
    /// let viewport = Viewport::Fixed(Rect::new(0, 0, 10, 10));
    /// let terminal = Terminal::with_options(backend, TerminalOptions { viewport })?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn with_options(mut backend: B, options: TerminalOptions) -> Result<Self, B::Error> {
        let area = match options.viewport {
            Viewport::Fullscreen | Viewport::Inline(_) => backend.size()?.into(),
            Viewport::Fixed(area) => area,
        };
        let (viewport_area, cursor_pos) = match options.viewport {
            Viewport::Fullscreen => (area, Position::ORIGIN),
            Viewport::Inline(height) => {
                compute_inline_size(&mut backend, height, area.as_size(), 0)?
            }
            Viewport::Fixed(area) => (area, area.as_position()),
        };
        Ok(Self {
            backend,
            buffers: [Buffer::empty(viewport_area), Buffer::empty(viewport_area)],
            current: 0,
            hidden_cursor: false,
            viewport: options.viewport,
            viewport_area,
            last_known_area: area,
            last_known_cursor_pos: cursor_pos,
            frame_count: 0,
        })
    }

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
    /// The [`Frame`] passed to the render callback represents the currently configured [`Viewport`]
    /// (see [`Frame::area`] and [`Terminal::with_options`]).
    ///
    /// Build layout relative to the [`Rect`] returned by [`Frame::area`] rather than assuming the
    /// origin is `(0, 0)`, so the same rendering code works for fixed and inline viewports.
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
    /// The [`Frame`] passed to the render callback represents the currently configured [`Viewport`]
    /// (see [`Frame::area`] and [`Terminal::with_options`]).
    ///
    /// Build layout relative to the [`Rect`] returned by [`Frame::area`] rather than assuming the
    /// origin is `(0, 0)`, so the same rendering code works for fixed and inline viewports.
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
    /// Getting the buffer and asserting on some cells after rendering a widget.
    ///
    /// ```rust,ignore
    /// use ratatui::{backend::TestBackend, Terminal};
    /// use ratatui::widgets::Paragraph;
    /// let backend = TestBackend::new(30, 5);
    /// let mut terminal = Terminal::new(backend).unwrap();
    /// {
    ///     let mut frame = terminal.get_frame();
    ///     frame.render_widget(Paragraph::new("Hello"), frame.area());
    /// }
    /// // When not using `draw`, present the buffer manually:
    /// terminal.flush().unwrap();
    /// terminal.swap_buffers();
    /// terminal.backend_mut().flush().unwrap();
    /// ```
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
    pub const fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current]
    }

    /// Gets the backend
    pub const fn backend(&self) -> &B {
        &self.backend
    }

    /// Gets the backend as a mutable reference
    pub const fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// Obtains a difference between the previous and the current buffer and passes it to the
    /// current backend for drawing.
    pub fn flush(&mut self) -> Result<(), B::Error> {
        let previous_buffer = &self.buffers[1 - self.current];
        let current_buffer = &self.buffers[self.current];
        let updates = previous_buffer.diff(current_buffer);
        if let Some((col, row, _)) = updates.last() {
            self.last_known_cursor_pos = Position { x: *col, y: *row };
        }
        self.backend.draw(updates.into_iter())
    }

    /// Updates the Terminal so that internal buffers match the requested area.
    ///
    /// Requested area will be saved to remain consistent when rendering. This leads to a full clear
    /// of the screen.
    pub fn resize(&mut self, area: Rect) -> Result<(), B::Error> {
        let next_area = match self.viewport {
            Viewport::Inline(height) => {
                let offset_in_previous_viewport = self
                    .last_known_cursor_pos
                    .y
                    .saturating_sub(self.viewport_area.top());
                compute_inline_size(
                    &mut self.backend,
                    height,
                    area.as_size(),
                    offset_in_previous_viewport,
                )?
                .0
            }
            Viewport::Fixed(_) | Viewport::Fullscreen => area,
        };
        self.set_viewport_area(next_area);
        self.clear()?;

        self.last_known_area = area;
        Ok(())
    }

    fn set_viewport_area(&mut self, area: Rect) {
        self.buffers[self.current].resize(area);
        self.buffers[1 - self.current].resize(area);
        self.viewport_area = area;
    }

    /// Queries the backend for size and resizes if it doesn't match the previous size.
    pub fn autoresize(&mut self) -> Result<(), B::Error> {
        // fixed viewports do not get autoresized
        if matches!(self.viewport, Viewport::Fullscreen | Viewport::Inline(_)) {
            let area = self.size()?.into();
            if area != self.last_known_area {
                self.resize(area)?;
            }
        }
        Ok(())
    }

    /// Hides the cursor.
    pub fn hide_cursor(&mut self) -> Result<(), B::Error> {
        self.backend.hide_cursor()?;
        self.hidden_cursor = true;
        Ok(())
    }

    /// Shows the cursor.
    pub fn show_cursor(&mut self) -> Result<(), B::Error> {
        self.backend.show_cursor()?;
        self.hidden_cursor = false;
        Ok(())
    }

    /// Gets the current cursor position.
    ///
    /// This is the position of the cursor after the last draw call and is returned as a tuple of
    /// `(x, y)` coordinates.
    #[deprecated = "use `get_cursor_position()` instead which returns `Result<Position>`"]
    pub fn get_cursor(&mut self) -> Result<(u16, u16), B::Error> {
        let Position { x, y } = self.get_cursor_position()?;
        Ok((x, y))
    }

    /// Sets the cursor position.
    #[deprecated = "use `set_cursor_position((x, y))` instead which takes `impl Into<Position>`"]
    pub fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), B::Error> {
        self.set_cursor_position(Position { x, y })
    }

    /// Gets the current cursor position.
    ///
    /// This is the position of the cursor after the last draw call.
    pub fn get_cursor_position(&mut self) -> Result<Position, B::Error> {
        self.backend.get_cursor_position()
    }

    /// Sets the cursor position.
    pub fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> Result<(), B::Error> {
        let position = position.into();
        self.backend.set_cursor_position(position)?;
        self.last_known_cursor_pos = position;
        Ok(())
    }

    /// Clear the terminal and force a full redraw on the next draw call.
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

    /// Clears the inactive buffer and swaps it with the current buffer
    pub fn swap_buffers(&mut self) {
        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;
    }

    /// Queries the real size of the backend.
    pub fn size(&self) -> Result<Size, B::Error> {
        self.backend.size()
    }

    /// Insert some content before the current inline viewport. This has no effect when the
    /// viewport is not inline.
    ///
    /// The `draw_fn` closure will be called to draw into a writable `Buffer` that is `height`
    /// lines tall. The content of that `Buffer` will then be inserted before the viewport.
    ///
    /// If the viewport isn't yet at the bottom of the screen, inserted lines will push it towards
    /// the bottom. Once the viewport is at the bottom of the screen, inserted lines will scroll
    /// the area of the screen above the viewport upwards.
    ///
    /// Before:
    /// ```ignore
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
    /// ```ignore
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
    /// ```ignore
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
    /// ```rust,ignore
    /// use ratatui::{
    ///     backend::TestBackend,
    ///     style::{Color, Style},
    ///     text::{Line, Span},
    ///     widgets::{Paragraph, Widget},
    ///     Terminal,
    /// };
    /// # let backend = TestBackend::new(10, 10);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// terminal.insert_before(1, |buf| {
    ///     Paragraph::new(Line::from(vec![
    ///         Span::raw("This line will be added "),
    ///         Span::styled("before", Style::default().fg(Color::Blue)),
    ///         Span::raw(" the current viewport"),
    ///     ]))
    ///     .render(buf.area, buf);
    /// });
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
    #[cfg(not(feature = "scrolling-regions"))]
    fn insert_before_no_scrolling_regions(
        &mut self,
        height: u16,
        draw_fn: impl FnOnce(&mut Buffer),
    ) -> Result<(), B::Error> {
        // The approach of this function is to first render all of the lines to insert into a
        // temporary buffer, and then to loop drawing chunks from the buffer to the screen. drawing
        // this buffer onto the screen.
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
        // The approach of this function is to first render all of the lines to insert into a
        // temporary buffer, and then to loop drawing chunks from the buffer to the screen. drawing
        // this buffer onto the screen.
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

fn compute_inline_size<B: Backend>(
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
