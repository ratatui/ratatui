#![deny(missing_docs)]
//! Provides the [`Terminal`], [`Frame`], [`CompletedFrame`], and [`Viewport`] types.
//!
//! This module contains Ratatui's rendering surface abstraction. [`Terminal`] ties together a
//! backend, a viewport, and a double-buffered renderer. In a typical application you create a
//! `Terminal`, render by calling [`Terminal::draw`] or [`Terminal::try_draw`] in a loop, and let
//! Ratatui diff successive frames so only changed cells are sent to the backend.
//!
//! [`Frame`] is the mutable view used during one render pass. Widgets write into the current
//! buffer through it, and cursor state for the end of the pass is requested through
//! [`Frame::set_cursor_position`]. After rendering completes, Ratatui applies the buffer diff,
//! updates the cursor, swaps buffers, and flushes any buffered backend output.
//!
//! This module focuses on rendering contracts. Process-wide terminal setup such as raw mode,
//! alternate screen handling, and panic restoration lives in the higher-level `ratatui` crate.
//!
//! # Example
//!
//! ```rust,no_run
//! # #![allow(unexpected_cfgs)]
//! # #[cfg(feature = "crossterm")]
//! # {
//! use std::io::stdout;
//!
//! use ratatui::Terminal;
//! use ratatui::backend::CrosstermBackend;
//! use ratatui::widgets::Paragraph;
//!
//! let backend = CrosstermBackend::new(stdout());
//! let mut terminal = Terminal::new(backend)?;
//! terminal.draw(|frame| {
//!     frame.render_widget(Paragraph::new("Hello world!"), frame.area());
//! })?;
//! # }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! [Crossterm]: https://crates.io/crates/crossterm
//! [Termion]: https://crates.io/crates/termion
//! [Termwiz]: https://crates.io/crates/termwiz
//! [`backend`]: crate::backend
//! [`Backend`]: crate::backend::Backend
//! [`Buffer`]: crate::buffer::Buffer

mod backend;
mod buffers;
mod cursor;
mod frame;
mod init;
mod inline;
mod render;
mod resize;
mod viewport;

pub use frame::{CompletedFrame, Frame};
pub use viewport::Viewport;

use crate::backend::Backend;
use crate::buffer::Buffer;
use crate::layout::{Position, Rect};

/// An interface to interact and draw [`Frame`]s on the user's terminal.
///
/// This is the main entry point for Ratatui's rendering subsystem. It owns the backend-facing
/// render state: double buffers, viewport bookkeeping, and cursor synchronization for each render
/// pass.
///
/// If you're building a fullscreen application with the `ratatui` crate's default backend
/// ([Crossterm]), prefer [`ratatui::run`] (or [`ratatui::init`] + [`ratatui::restore`]) over
/// constructing `Terminal` directly. These helpers enable common terminal modes (raw mode +
/// alternate screen) and restore them on exit and on panic.
///
/// ```rust,no_run
/// # #![allow(unexpected_cfgs)]
/// # #[cfg(feature = "crossterm")]
/// # {
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
/// # }
/// # Ok::<(), Box<dyn std::error::Error>>(())
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
/// The normal mental model is: redraw the whole UI each pass, let Ratatui compute the diff, and
/// treat `Frame::area` as the source of truth for where this pass can render. Most application
/// code can stay entirely within that model.
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
/// If [`Terminal::try_draw`] returns an error, the render pass ends early. Depending on where the
/// failure happened, Ratatui may have already resized internal buffers, written part of the diff,
/// or left cursor state unapplied. In most applications, treat that error as fatal for the current
/// terminal session and let higher-level setup code restore terminal state before continuing.
///
/// Most applications should use [`Terminal::draw`] / [`Terminal::try_draw`]. Manual rendering is a
/// separate, lower-level path intended primarily for tests and specialized integrations. In that
/// mode you build a frame with [`Terminal::get_frame`], apply the current buffer diff with
/// [`Terminal::flush`], then call [`Terminal::swap_buffers`]. If your backend buffers output, also
/// call [`Backend::flush`].
///
/// [`Terminal::flush`] only knows about Ratatui's two screen buffers. It does not know whether
/// you have changed terminal modes or switched display surfaces (for example by leaving the
/// alternate screen). If you call it after such a change, Ratatui may replay a diff computed for
/// the old surface onto the new one. When you need a complete draw pass that stays synchronized
/// with cursor updates and backend flushing, prefer [`Terminal::draw`] / [`Terminal::try_draw`].
///
/// The same caution applies to direct backend mutation and direct cursor manipulation. If you
/// write to the backend or move the cursor outside Ratatui's normal render pass, the next draw may
/// overwrite those changes or may diff against stale assumptions. Use those escape hatches only
/// when you intentionally manage resynchronization yourself, typically by calling
/// [`Terminal::clear`] or performing a full render pass afterward.
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
/// Choose a viewport based on how the app should fit into the terminal:
///
/// - [`Viewport::Fullscreen`]: the standard TUI case where Ratatui owns the whole terminal window.
/// - [`Viewport::Inline`]: embed the UI into a larger CLI flow with normal terminal output above
///   it.
/// - [`Viewport::Fixed`]: render into one region of a larger terminal layout managed elsewhere.
///
/// Choose a viewport at initialization time with [`Terminal::with_options`] and
/// [`TerminalOptions`].
///
/// `Frame::area` depends on the active viewport. In fullscreen mode it starts at (0, 0); in fixed
/// and inline mode it may have a non-zero origin, so prefer using `frame.area()` as your root
/// layout rectangle. The variant docs on [`Viewport`] describe each mode in more detail, and
/// inline-specific behavior is covered in the "Inline Viewport" section below.
///
/// ```rust,no_run
/// # #![allow(unexpected_cfgs)]
/// # #[cfg(feature = "crossterm")]
/// # {
/// use ratatui::backend::CrosstermBackend;
/// use ratatui::layout::{Constraint, Layout, Rect};
/// use ratatui::{Terminal, TerminalOptions, Viewport};
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
///
/// fixed.draw(|frame| {
///     // Split the fixed viewport itself instead of assuming the viewport starts at `(0, 0)`.
///     let [header, body] =
///         Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(frame.area());
///
///     frame.render_widget("Fixed panel header", header);
///     frame.render_widget("Render the panel body relative to frame.area()", body);
/// })?;
/// # }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Applications should redraw after terminal resizes with [`Terminal::draw`] /
/// [`Terminal::try_draw`]. Fullscreen and inline viewports resize automatically during those render
/// passes; fixed viewports do not.
///
/// If your event loop receives a resize event, treat that event as a signal to render again rather
/// than as a complete source of truth for layout. During a render pass, use [`Frame::area`] as the
/// rectangle that Ratatui has actually prepared for drawing. Ratatui checks the backend's current
/// size during `draw` / `try_draw` so layout reflects the terminal size that exists at render
/// time, even if resize events were coalesced, missed, or arrived before your app handled them.
///
/// # Inline Viewport
///
/// Inline mode is designed for applications that want to embed a UI into a larger CLI flow. In
/// [`Viewport::Inline`], Ratatui anchors the viewport to the backend cursor row and always starts
/// drawing at column 0.
///
/// To reserve vertical space for the requested height, Ratatui may append lines. When the cursor is
/// near the bottom edge, terminals scroll; Ratatui accounts for that scrolling by shifting the
/// computed viewport origin upward so the viewport stays fully visible.
///
/// While running in inline mode, [`Terminal::insert_before`] can be used to print output above the
/// viewport without disturbing the UI's logical position. When Ratatui is built with the
/// `scrolling-regions` feature, `insert_before` can do this without clearing and redrawing the
/// viewport.
///
/// ```rust,no_run
/// # #![allow(unexpected_cfgs)]
/// # #[cfg(feature = "crossterm")]
/// # {
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
///
/// terminal.draw(|frame| {
///     // Continue rendering the inline UI relative to the inline viewport.
///     frame.render_widget("inline ui", frame.area());
/// })?;
/// # }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # More Information
///
/// - Choosing a viewport: [`Terminal::with_options`], [`TerminalOptions`], and [`Viewport`]
/// - The rendering pipeline: [`Terminal::draw`] and [`Terminal::try_draw`]
/// - Resize handling: [`Terminal::autoresize`] and [`Terminal::resize`]
/// - Cursor behavior: [`Frame::set_cursor_position`], [`Terminal::set_cursor_position`], and
///   [`Terminal::show_cursor`]
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
    /// Double-buffered render state for the current viewport.
    ///
    /// [`Terminal::flush`] diffs `buffers[current]` against the other buffer to compute the next
    /// batch of cell updates to send to the backend.
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
    /// This is used by [`Terminal::autoresize`] to detect size changes and is reported via
    /// [`CompletedFrame::area`].
    last_known_area: Rect,
    /// Last known cursor position in terminal coordinates.
    ///
    /// This is updated when:
    ///
    /// - [`Terminal::set_cursor_position`] is called directly.
    /// - [`Frame::set_cursor_position`] is used during [`Terminal::draw`] /
    ///   [`Terminal::try_draw`].
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
pub struct TerminalOptions {
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
