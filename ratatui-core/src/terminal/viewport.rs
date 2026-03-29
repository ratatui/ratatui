use core::fmt;

use crate::layout::Rect;

/// The area of the terminal that Ratatui draws into.
///
/// A [`Viewport`] controls where widgets render and what [`Frame::area`] returns.
///
/// For a higher-level overview of viewports in the context of an application (including
/// examples), see [`Terminal`].
///
/// Choose a viewport based on how the Ratatui UI should fit into the terminal:
///
/// - [`Viewport::Fullscreen`] for the standard case: your app owns the whole terminal surface.
/// - [`Viewport::Inline`] when the UI should live inside a larger CLI flow, with normal terminal
///   output above it.
/// - [`Viewport::Fixed`] when Ratatui should render into one region of a terminal layout managed
///   elsewhere.
///
/// In fullscreen mode, the viewport starts at (0, 0). In inline and fixed mode, the viewport may
/// have a non-zero `x`/`y` origin; prefer using `Frame::area()` as your root layout rectangle.
/// Code that assumes `(0, 0)` as the origin is therefore only correct for fullscreen viewports.
///
/// See [`Terminal::with_options`] for how to select a viewport, and [`Terminal::resize`] /
/// [`Terminal::autoresize`] for resize behavior.
///
/// # Example
///
/// ```rust,no_run
/// # #![allow(unexpected_cfgs)]
/// # #[cfg(feature = "crossterm")]
/// # {
/// use ratatui::backend::CrosstermBackend;
/// use ratatui::layout::{Constraint, Layout, Rect};
/// use ratatui::{Terminal, TerminalOptions, Viewport};
///
/// let mut terminal = Terminal::with_options(
///     CrosstermBackend::new(std::io::stdout()),
///     TerminalOptions {
///         viewport: Viewport::Fixed(Rect::new(10, 5, 20, 4)),
///     },
/// )?;
///
/// terminal.draw(|frame| {
///     // `frame.area()` is `Rect::new(10, 5, 20, 4)`, not `(0, 0, 20, 4)`.
///     let [title, body] =
///         Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(frame.area());
///
///     frame.render_widget("panel title", title);
///     frame.render_widget("render the body relative to the fixed viewport", body);
/// })?;
/// # }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// [`Frame::area`]: crate::terminal::Frame::area
/// [`Terminal`]: crate::terminal::Terminal
/// [`Terminal::with_options`]: crate::terminal::Terminal::with_options
/// [`Terminal::resize`]: crate::terminal::Terminal::resize
/// [`Terminal::autoresize`]: crate::terminal::Terminal::autoresize
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum Viewport {
    /// Draw into the entire terminal.
    ///
    /// This is the default viewport used by [`Terminal::new`].
    ///
    /// Choose this when the Ratatui app should own the whole terminal window.
    ///
    /// When the terminal size changes, Ratatui automatically resizes internal buffers during
    /// [`Terminal::draw`] / [`Terminal::try_draw`].
    ///
    /// `Frame::area()` always starts at (0, 0).
    ///
    /// [`Terminal::new`]: crate::terminal::Terminal::new
    /// [`Terminal::draw`]: crate::terminal::Terminal::draw
    #[default]
    Fullscreen,
    /// Draw the application inline with the rest of the terminal output.
    ///
    /// Choose this when the UI should appear inside a larger command-line flow instead of taking
    /// over the entire terminal.
    ///
    /// The viewport spans the full terminal width and its top-left corner is anchored to column 0
    /// of the current cursor row when the terminal is created and whenever it is recomputed during
    /// resize. Ratatui reserves space for the requested height; if the cursor is near the bottom
    /// of the screen, this may scroll the terminal so the viewport remains fully visible.
    ///
    /// The height is specified in rows and is clamped to the current terminal height.
    ///
    /// Inline viewports always span the full terminal width.
    ///
    /// For the full inline rendering model, including output inserted above the UI, see the
    /// "Inline Viewport" section on [`Terminal`] and [`Terminal::insert_before`].
    Inline(u16),
    /// Draw into a fixed region of the terminal.
    ///
    /// Choose this when Ratatui is responsible for only part of the screen, for example a panel in
    /// a larger terminal layout managed by another renderer or by surrounding application code.
    ///
    /// Fixed viewports are not automatically resized. If the region should change (for example, on
    /// terminal resize), call [`Terminal::resize`] yourself.
    ///
    /// The area is specified as a [`Rect`] in terminal coordinates.
    ///
    /// `Frame::area()` returns this rectangle as-is (including its `x`/`y` offset).
    /// Ratatui does not keep this rectangle synchronized with backend resizes unless you call
    /// [`Terminal::resize`] yourself.
    ///
    /// See also [`Terminal::with_options`] for initialization behavior.
    ///
    /// [`Terminal::with_options`]: crate::terminal::Terminal::with_options
    /// [`Terminal::resize`]: crate::terminal::Terminal::resize
    Fixed(Rect),
}

impl fmt::Display for Viewport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fullscreen => write!(f, "Fullscreen"),
            Self::Inline(height) => write!(f, "Inline({height})"),
            Self::Fixed(area) => write!(f, "Fixed({area})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn viewport_to_string() {
        assert_eq!(Viewport::Fullscreen.to_string(), "Fullscreen");
        assert_eq!(Viewport::Inline(5).to_string(), "Inline(5)");
        assert_eq!(
            Viewport::Fixed(Rect::new(0, 0, 5, 5)).to_string(),
            "Fixed(5x5+0+0)"
        );
    }
}
