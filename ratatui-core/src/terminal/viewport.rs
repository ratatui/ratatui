use core::fmt;

use crate::layout::Rect;

/// The area of the terminal that Ratatui draws into.
///
/// A [`Viewport`] controls where widgets render and what [`Frame::area`] returns.
///
/// For a higher-level overview of viewports in the context of an application (including
/// examples), see [`Terminal`].
///
/// Most applications use [`Viewport::Fullscreen`]. Use [`Viewport::Inline`] when you want to embed
/// a UI into a larger CLI flow (for example: print some text, then start an interactive UI below
/// it). Use [`Viewport::Fixed`] when you want Ratatui to render into a specific region of the
/// terminal.
///
/// In fullscreen mode, the viewport starts at (0, 0). In inline and fixed mode, the viewport may
/// have a non-zero `x`/`y` origin; prefer using `Frame::area()` as your root layout rectangle.
///
/// See [`Terminal::with_options`] for how to select a viewport, and [`Terminal::resize`] /
/// [`Terminal::autoresize`] for resize behavior.
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
    /// When the terminal size changes, Ratatui automatically resizes internal buffers during
    /// [`Terminal::draw`].
    ///
    /// `Frame::area()` always starts at (0, 0).
    ///
    /// [`Terminal::new`]: crate::terminal::Terminal::new
    /// [`Terminal::draw`]: crate::terminal::Terminal::draw
    #[default]
    Fullscreen,
    /// Draw the application inline with the rest of the terminal output.
    ///
    /// The viewport spans the full terminal width and its top-left corner is anchored to column 0
    /// of the current cursor row when the terminal is created (and when it is resized). Ratatui
    /// reserves space for the requested height; if the cursor is near the bottom of the screen,
    /// this may scroll the terminal so the viewport remains fully visible.
    ///
    /// The height is specified in rows and is clamped to the current terminal height.
    Inline(u16),
    /// Draw into a fixed region of the terminal.
    ///
    /// This can be useful when Ratatui is responsible for only part of the screen (for example, a
    /// status panel beside another renderer), or when you want to manage the overall layout
    /// yourself.
    ///
    /// Fixed viewports are not automatically resized. If the region should change (for example, on
    /// terminal resize), call [`Terminal::resize`] yourself.
    ///
    /// The area is specified as a [`Rect`] in terminal coordinates.
    ///
    /// `Frame::area()` returns this rectangle as-is (including its `x`/`y` offset).
    ///
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
