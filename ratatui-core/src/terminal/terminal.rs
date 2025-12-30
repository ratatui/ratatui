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
/// # Inline Viewport
///
/// Inline mode is designed for applications that want to embed a UI into a larger CLI flow. In
/// [`Viewport::Inline`], Ratatui anchors the viewport to the backend cursor row at initialization
/// time and always starts drawing at column 0.
///
/// To reserve vertical space for the requested height, Ratatui may append lines. When the cursor is
/// near the bottom edge, terminals scroll; Ratatui accounts for that scrolling by shifting the
/// computed viewport origin upward so the viewport stays fully visible.
///
/// While running in inline mode, [`Terminal::insert_before`] can be used to print output above the
/// viewport without disturbing the UI.
/// When Ratatui is built with the `scrolling-regions` feature, `insert_before` can do this without
/// clearing and redrawing the viewport.
///
/// ```rust,ignore
/// use ratatui::{TerminalOptions, Viewport};
///
/// println!("Some output above the UI");
///
/// let options = TerminalOptions {
///     viewport: Viewport::Inline(10),
/// };
/// let mut terminal = ratatui::try_init_with_options(options)?;
///
/// terminal.insert_before(1, |buf| {
///     // Render a single line of output into `buf` before the UI.
///     // (For example: logs, status updates, or command output.)
/// })?;
/// ```
///
/// # More Information
///
/// - Choosing a viewport: [`Terminal::with_options`], [`TerminalOptions`], and [`Viewport`]
/// - The rendering pipeline: [`Terminal::draw`] and [`Terminal::try_draw`]
/// - Resize handling: [`Terminal::autoresize`] and [`Terminal::resize`]
/// - Manual rendering and testing: [`Terminal::get_frame`], [`Terminal::flush`], and
///   [`Terminal::swap_buffers`]
/// - Printing above an inline UI: [`Terminal::insert_before`]
///
/// # Initialization
///
/// Most interactive TUIs need process-wide terminal setup (for example: raw mode and an alternate
/// screen) and matching teardown on exit and on panic. In Ratatui, that setup lives in the
/// `ratatui` crate; `Terminal` itself focuses on rendering and does not implicitly change those
/// modes.
///
/// If you're using the `ratatui` crate with its default backend ([Crossterm]), there are three
/// common entry points:
///
/// - [`ratatui::run`]: recommended for most applications. Provides a [`ratatui::DefaultTerminal`],
///   runs your closure, and restores terminal state on exit and on panic.
/// - [`ratatui::init`] + [`ratatui::restore`]: like `run`, but you control the event loop and
///   decide when to restore.
/// - [`Terminal::new`] / [`Terminal::with_options`]: manual construction (for example: custom
///   backends such as [Termion] / [Termwiz], inline UIs, or fixed viewports). You are responsible
///   for terminal mode setup and teardown.
///
/// [`ratatui::run`] was introduced in Ratatui 0.30, so older tutorials may use `init`/`restore` or
/// manual construction.
///
/// Some applications install a custom panic hook to log a crash report, print a friendlier error,
/// or integrate with error reporting. If you do, install it before calling [`ratatui::init`] /
/// [`ratatui::run`]. Ratatui wraps the current hook so it can restore terminal state first (for
/// example: leaving the alternate screen and disabling raw mode) and then delegate to your hook.
///
/// Crossterm is cross-platform and is what most Ratatui applications use by default. Ratatui also
/// supports other backends such as [Termion] and [Termwiz], and third-party backends can integrate
/// by implementing [`Backend`].
///
/// # How it works
///
/// `Terminal` ties together a [`Backend`], a [`Viewport`], and a double-buffered diffing renderer.
/// The high-level flow is described in the "Rendering Pipeline" section above; this section focuses
/// on how that pipeline is implemented.
///
/// `Terminal` is generic over a [`Backend`] implementation and does not depend on a particular
/// terminal library. It relies on the backend to:
///
/// - report the current screen size (used by [`Terminal::autoresize`])
/// - draw cell updates (used by [`Terminal::flush`])
/// - clear regions (used by [`Terminal::clear`] and [`Terminal::resize`])
/// - move and show/hide the cursor (used by [`Terminal::try_draw`])
/// - optionally append lines (used by inline viewports and by [`Terminal::insert_before`])
///
/// ## Buffers and diffing
///
/// The `Terminal` maintains two [`Buffer`]s sized to the current viewport. During a render pass,
/// widgets draw into the "current" buffer via the [`Frame`] passed to your callback. At the end of
/// the pass, [`Terminal::flush`] diffs the current buffer against the previous buffer and sends
/// only the changed cells to the backend.
///
/// After flushing, [`Terminal::swap_buffers`] flips which buffer is considered "current" and resets
/// the next buffer. This is why each render pass starts from an empty buffer: your callback is
/// expected to fully redraw the viewport every time.
///
/// The [`CompletedFrame`] returned from [`Terminal::draw`] / [`Terminal::try_draw`] provides a
/// reference to the buffer that was just rendered, which can be useful for assertions in tests.
///
/// ## Viewport state and resizing
///
/// The active [`Viewport`] controls how the viewport area is computed:
///
/// - Fullscreen: `Frame::area` covers the full backend size.
/// - Fixed: `Frame::area` is the exact rectangle you provided in terminal coordinates.
/// - Inline: `Frame::area` is a rectangle anchored to the backend cursor row.
///
/// For fullscreen and inline viewports, [`Terminal::autoresize`] checks the backend size during
/// every render pass and calls [`Terminal::resize`] when it changes. Resizing updates the internal
/// buffer sizes and clears the affected region; it also resets the previous buffer so the next draw
/// is treated as a full redraw.
///
/// ## Cursor tracking
///
/// The cursor position requested by [`Frame::set_cursor_position`] is applied after
/// [`Terminal::flush`] so the cursor ends up on top of the rendered UI. `Terminal` also tracks a
/// "last known cursor position" as a best-effort record of where it last wrote, and uses that
/// information when recomputing inline viewports on resize.
///
/// ## Inline-specific behavior
///
/// Inline viewports reserve vertical space by calling [`Backend::append_lines`]. If the cursor is
/// close enough to the bottom edge, terminals scroll as lines are appended. Ratatui accounts for
/// that scrolling by shifting the computed viewport origin upward so the viewport remains fully
/// visible. On resize, Ratatui recomputes the inline origin while trying to keep the cursor at the
/// same relative row inside the viewport.
///
/// When Ratatui is built with the `scrolling-regions` feature, [`Terminal::insert_before`] uses
/// terminal scrolling regions to insert content above an inline viewport without clearing and
/// redrawing it.
///
/// [Crossterm]: https://crates.io/crates/crossterm
/// [Termion]: https://crates.io/crates/termion
/// [Termwiz]: https://crates.io/crates/termwiz
/// [`backend`]: crate::backend
/// [`Backend`]: crate::backend::Backend
/// [`Backend::flush`]: crate::backend::Backend::flush
/// [`Buffer`]: crate::buffer::Buffer
/// [`ratatui::DefaultTerminal`]: https://docs.rs/ratatui/latest/ratatui/type.DefaultTerminal.html
/// [`ratatui::init`]: https://docs.rs/ratatui/latest/ratatui/fn.init.html
/// [`ratatui::restore`]: https://docs.rs/ratatui/latest/ratatui/fn.restore.html
/// [`ratatui::run`]: https://docs.rs/ratatui/latest/ratatui/fn.run.html
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Terminal<B>
where
    B: Backend,
{
    /// The backend used to write updates to the terminal.
    ///
    /// Most application code does not need to interact with the backend directly; see
    /// [`Terminal::draw`]. Accessing the backend can be useful for backend-specific testing and
    /// inspection (see [`Terminal::backend`]).
    backend: B,
    /// Double-buffered render state.
    ///
    /// [`Terminal::flush`] diffs `buffers[current]` against the other buffer to compute a minimal
    /// set of updates to send to the backend.
    buffers: [Buffer; 2],
    /// Index of the "current" buffer in [`Terminal::buffers`].
    ///
    /// This toggles between 0 and 1 and is updated by [`Terminal::swap_buffers`].
    current: usize,
    /// Whether Ratatui believes it has hidden the cursor.
    ///
    /// This is tracked so [`Drop`] can attempt to restore cursor visibility.
    hidden_cursor: bool,
    /// The configured [`Viewport`] mode.
    ///
    /// This determines how the initial viewport area is computed during construction, whether
    /// [`Terminal::autoresize`] runs, how [`Terminal::clear`] behaves, and whether operations like
    /// [`Terminal::insert_before`] have any effect.
    viewport: Viewport,
    /// The current viewport rectangle in terminal coordinates.
    ///
    /// This is the area returned by [`Frame::area`] and the size of the internal buffers. It is
    /// set during construction and updated by [`Terminal::resize`]. In inline mode, calls to
    /// [`Terminal::insert_before`] can also move the viewport vertically.
    viewport_area: Rect,
    /// Last known renderable "screen" area.
    ///
    /// For fullscreen and inline viewports this tracks the backend-reported terminal size. For
    /// fixed viewports, this tracks the user-provided fixed area.
    ///
    /// This is used by [`Terminal::autoresize`] and is reported via [`CompletedFrame::area`].
    last_known_area: Rect,
    /// Last known cursor position in terminal coordinates.
    ///
    /// This is updated when:
    ///
    /// - [`Terminal::set_cursor_position`] is called directly.
    /// - [`Frame::set_cursor_position`] is used during [`Terminal::draw`].
    /// - [`Terminal::flush`] observes a diff update (used as a proxy for the "last written" cell).
    ///
    /// Inline viewports use this during [`Terminal::resize`] to preserve the cursor's relative
    /// position within the viewport.
    last_known_cursor_pos: Position,
    /// Number of frames rendered so far.
    ///
    /// This increments after each successful [`Terminal::draw`] / [`Terminal::try_draw`] and wraps
    /// at `usize::MAX`.
    frame_count: usize,
}

/// Options to pass to [`Terminal::with_options`]
///
/// Most applications can use [`Terminal::new`]. Use `TerminalOptions` when you need to configure a
/// non-default [`Viewport`] at initialization time (see [`Terminal`] for an overview).
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Options {
    /// Viewport used to draw to the terminal.
    ///
    /// See [`Terminal`] for a higher-level overview, and [`Viewport`] for the per-variant
    /// definition.
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
    /// This is a convenience for [`Terminal::with_options`] with [`Viewport::Fullscreen`].
    ///
    /// After creating a terminal, call [`Terminal::draw`] (or [`Terminal::try_draw`]) in a loop to
    /// render your UI.
    ///
    /// Note that unlike [`ratatui::init`], this does not install a panic hook, so it is
    /// recommended to do that manually when using this function, otherwise any panic messages will
    /// be printed to the alternate screen and the terminal may be left in an unusable state.
    ///
    /// See [how to set up panic hooks](https://ratatui.rs/recipes/apps/panic-hooks/) and
    /// [`better-panic` example](https://ratatui.rs/recipes/apps/better-panic/) for more
    /// information.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # #![allow(unexpected_cfgs)]
    /// # #[cfg(feature = "crossterm")]
    /// # {
    /// use std::io::stdout;
    ///
    /// use ratatui::Terminal;
    /// use ratatui::backend::CrosstermBackend;
    ///
    /// let backend = CrosstermBackend::new(stdout());
    /// let _terminal = Terminal::new(backend)?;
    ///
    /// // Optionally set up a panic hook to restore the terminal on panic.
    /// let old_hook = std::panic::take_hook();
    /// std::panic::set_hook(Box::new(move |info| {
    ///     ratatui::restore();
    ///     old_hook(info);
    /// }));
    /// # }
    /// # #[cfg(not(feature = "crossterm"))]
    /// # {
    /// # use ratatui_core::{backend::TestBackend, terminal::Terminal};
    /// # let backend = TestBackend::new(10, 10);
    /// # let _terminal = Terminal::new(backend)?;
    /// # }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`ratatui::init`]: https://docs.rs/ratatui/latest/ratatui/fn.init.html
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
    /// The viewport determines what area is exposed to widgets via [`Frame::area`]. See
    /// [`Viewport`] for an overview of the available modes.
    ///
    /// After creating a terminal, call [`Terminal::draw`] (or [`Terminal::try_draw`]) in a loop to
    /// render your UI.
    ///
    /// Resize behavior depends on the selected viewport:
    ///
    /// - [`Viewport::Fullscreen`] and [`Viewport::Inline`] are automatically resized during
    ///   [`Terminal::draw`] (via [`Terminal::autoresize`]).
    /// - [`Viewport::Fixed`] is not automatically resized; call [`Terminal::resize`] if the region
    ///   should change.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # #![allow(unexpected_cfgs)]
    /// # #[cfg(feature = "crossterm")]
    /// # {
    /// use std::io::stdout;
    ///
    /// use ratatui::backend::CrosstermBackend;
    /// use ratatui::layout::Rect;
    /// use ratatui::{Terminal, TerminalOptions, Viewport};
    ///
    /// let backend = CrosstermBackend::new(stdout());
    /// let viewport = Viewport::Fixed(Rect::new(0, 0, 10, 10));
    /// let _terminal = Terminal::with_options(backend, TerminalOptions { viewport })?;
    /// # }
    /// # #[cfg(not(feature = "crossterm"))]
    /// # {
    /// # use ratatui_core::{
    /// #     backend::TestBackend,
    /// #     layout::Rect,
    /// #     terminal::{Terminal, TerminalOptions, Viewport},
    /// # };
    /// # let backend = TestBackend::new(10, 10);
    /// # let viewport = Viewport::Fixed(Rect::new(0, 0, 10, 10));
    /// # let _terminal = Terminal::with_options(backend, TerminalOptions { viewport })?;
    /// # }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// When the viewport is [`Viewport::Inline`], Ratatui anchors the viewport to the current
    /// cursor row at initialization time (always starting at column 0). Ratatui may scroll the
    /// terminal to make enough room for the requested height so the viewport stays fully visible.
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

    /// Returns a shared reference to the backend.
    ///
    /// This is primarily useful for backend-specific inspection in tests (e.g. reading
    /// [`TestBackend`]'s buffer). Most applications should interact with the terminal via
    /// [`Terminal::draw`] rather than calling backend methods directly.
    ///
    /// [`TestBackend`]: crate::backend::TestBackend
    pub const fn backend(&self) -> &B {
        &self.backend
    }

    /// Returns a mutable reference to the backend.
    ///
    /// This is an advanced escape hatch. Mutating the backend directly can desynchronize Ratatui's
    /// internal buffers from what's on-screen; if you do this, you may need to call
    /// [`Terminal::clear`] to force a full redraw.
    pub const fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
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

    /// Updates the Terminal so that internal buffers match the requested area.
    ///
    /// This updates the buffer size used for rendering and triggers a full clear so the next
    /// [`Terminal::draw`] paints into a consistent area.
    ///
    /// When the viewport is [`Viewport::Inline`], the `area` argument is treated as the new
    /// terminal size and the viewport origin is recomputed relative to the current cursor position.
    /// Ratatui attempts to keep the cursor at the same relative row within the viewport across
    /// resizes.
    ///
    /// See also: [`Terminal::autoresize`] (automatic resizing during [`Terminal::draw`]).
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

    /// Resize internal buffers and update the current viewport area.
    ///
    /// This is an internal helper used by [`Terminal::with_options`] and [`Terminal::resize`].
    fn set_viewport_area(&mut self, area: Rect) {
        self.buffers[self.current].resize(area);
        self.buffers[1 - self.current].resize(area);
        self.viewport_area = area;
    }

    /// Queries the backend for size and resizes if it doesn't match the previous size.
    ///
    /// This is called automatically during [`Terminal::draw`] for fullscreen and inline viewports.
    /// Fixed viewports are not automatically resized.
    ///
    /// If the size changed, this calls [`Terminal::resize`] (which clears the screen).
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

     ///     frame.set_cursor_position(Position { x: 0, y: 0 });
-    ///     io::Result::Ok(())
+    ///     Ok(())
     /// })?;
     ///
-    /// // or with a function
+    /// // Or with a function.
     /// terminal.try_draw(render)?;
     ///
-    /// fn render(frame: &mut ratatui::Frame) -> io::Result<()> {
-    ///     let value: u8 = "not a number".parse().map_err(io::Error::other)?;
-    ///     frame.render_widget(Paragraph::new("Hello World!"), frame.area());
+    /// fn render(frame: &mut Frame<'_>) -> io::Result<()> {
+    ///     frame.render_widget("Hello World!", frame.area());
     ///     Ok(())
     /// }
-    /// # io::Result::Ok(())
+    /// # }
+    /// # #[cfg(not(feature = "crossterm"))]
+    /// # {
+    /// # use ratatui_core::{backend::TestBackend, terminal::Terminal};
+    /// # let backend = TestBackend::new(10, 10);
+    /// # let mut terminal = Terminal::new(backend)?;
+    /// # terminal
+    /// #     .try_draw(|frame| {
+    /// #         frame.render_widget("Hello World!", frame.area());
+    /// #         Ok::<(), core::convert::Infallible>(())
+    /// #     })
+    /// #     ?;
+    /// # }
    /// Hides the cursor.
    ///
    /// When using [`Terminal::draw`], prefer controlling the cursor with
    /// [`Frame::set_cursor_position`]. Mixing the APIs can lead to surprising results.
    pub fn hide_cursor(&mut self) -> Result<(), B::Error> {
        self.backend.hide_cursor()?;
        self.hidden_cursor = true;
        Ok(())
    }

    /// Shows the cursor.
    ///
    /// When using [`Terminal::draw`], prefer controlling the cursor with
    /// [`Frame::set_cursor_position`]. Mixing the APIs can lead to surprising results.
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
    /// This queries the backend for the current cursor position.
    ///
    /// When using [`Terminal::draw`], prefer controlling the cursor with
    /// [`Frame::set_cursor_position`]. For direct control, see [`Terminal::set_cursor_position`].
    pub fn get_cursor_position(&mut self) -> Result<Position, B::Error> {
        self.backend.get_cursor_position()
    }

    /// Sets the cursor position.
    ///
    /// This updates the backend cursor and Ratatui's internal cursor tracking. Inline viewports
    /// use that tracking when recomputing the viewport on resize.
    ///
    /// When using [`Terminal::draw`], consider using [`Frame::set_cursor_position`] instead so the
    /// cursor is updated as part of the normal rendering flow.
    pub fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> Result<(), B::Error> {
        let position = position.into();
        self.backend.set_cursor_position(position)?;
        self.last_known_cursor_pos = position;
        Ok(())
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

    /// Clears the inactive buffer and swaps it with the current buffer.
    ///
    /// This is part of the standard rendering flow (see [`Terminal::try_draw`]). If you render
    /// manually using [`Terminal::get_frame`] and [`Terminal::flush`], call this afterward so the
    /// next flush can compute diffs against the correct "previous" buffer.
    pub fn swap_buffers(&mut self) {
        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;
    }

    /// Queries the real size of the backend.
    ///
    /// This returns the size of the underlying terminal. The current renderable area depends on
    /// the configured [`Viewport`]; use [`Frame::area`] inside [`Terminal::draw`] if you want the
    /// area you should render into.
    pub fn size(&self) -> Result<Size, B::Error> {
        self.backend.size()
    }

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
