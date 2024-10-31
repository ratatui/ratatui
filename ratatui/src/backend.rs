#![warn(missing_docs)]
//! This module provides the backend implementations for different terminal libraries.
//!
//! It defines the [`Backend`] trait which is used to abstract over the specific terminal library
//! being used.
//!
//! Supported terminal backends:
//! - [Crossterm]: enable the `crossterm` feature (enabled by default) and use [`CrosstermBackend`]
//! - [Termion]: enable the `termion` feature and use [`TermionBackend`]
//! - [Termwiz]: enable the `termwiz` feature and use [`TermwizBackend`]
//!
//! Additionally, a [`TestBackend`] is provided for testing purposes.
//!
//! See the [Backend Comparison] section of the [Ratatui Website] for more details on the different
//! backends.
//!
//! Each backend supports a number of features, such as [raw mode](#raw-mode), [alternate
//! screen](#alternate-screen), and [mouse capture](#mouse-capture). These features are generally
//! not enabled by default, and must be enabled by the application before they can be used. See the
//! documentation for each backend for more details.
//!
//! Note: most applications should use the [`Terminal`] struct instead of directly calling methods
//! on the backend.
//!
//! # Example
//!
//! ```rust,no_run
//! use std::io::stdout;
//!
//! use ratatui::{backend::CrosstermBackend, Terminal};
//!
//! let backend = CrosstermBackend::new(stdout());
//! let mut terminal = Terminal::new(backend)?;
//! terminal.clear()?;
//! terminal.draw(|frame| {
//!     // -- snip --
//! })?;
//! # std::io::Result::Ok(())
//! ```
//!
//! See the the [Examples] directory for more examples.
//!
//! # Raw Mode
//!
//! Raw mode is a mode where the terminal does not perform any processing or handling of the input
//! and output. This means that features such as echoing input characters, line buffering, and
//! special character processing (e.g., CTRL-C for SIGINT) are disabled. This is useful for
//! applications that want to have complete control over the terminal input and output, processing
//! each keystroke themselves.
//!
//! For example, in raw mode, the terminal will not perform line buffering on the input, so the
//! application will receive each key press as it is typed, instead of waiting for the user to
//! press enter. This makes it suitable for real-time applications like text editors,
//! terminal-based games, and more.
//!
//! Each backend handles raw mode differently, so the behavior may vary depending on the backend
//! being used. Be sure to consult the backend's specific documentation for exact details on how it
//! implements raw mode.

//! # Alternate Screen
//!
//! The alternate screen is a separate buffer that some terminals provide, distinct from the main
//! screen. When activated, the terminal will display the alternate screen, hiding the current
//! content of the main screen. Applications can write to this screen as if it were the regular
//! terminal display, but when the application exits, the terminal will switch back to the main
//! screen, and the contents of the alternate screen will be cleared. This is useful for
//! applications like text editors or terminal games that want to use the full terminal window
//! without disrupting the command line or other terminal content.
//!
//! This creates a seamless transition between the application and the regular terminal session, as
//! the content displayed before launching the application will reappear after the application
//! exits.
//!
//! Note that not all terminal emulators support the alternate screen, and even those that do may
//! handle it differently. As a result, the behavior may vary depending on the backend being used.
//! Always consult the specific backend's documentation to understand how it implements the
//! alternate screen.
//!
//! # Mouse Capture
//!
//! Mouse capture is a mode where the terminal captures mouse events such as clicks, scrolls, and
//! movement, and sends them to the application as special sequences or events. This enables the
//! application to handle and respond to mouse actions, providing a more interactive and graphical
//! user experience within the terminal. It's particularly useful for applications like
//! terminal-based games, text editors, or other programs that require more direct interaction from
//! the user.
//!
//! Each backend handles mouse capture differently, with variations in the types of events that can
//! be captured and how they are represented. As such, the behavior may vary depending on the
//! backend being used, and developers should consult the specific backend's documentation to
//! understand how it implements mouse capture.
//!
//! [`TermionBackend`]: termion/struct.TermionBackend.html
//! [`Terminal`]: crate::terminal::Terminal
//! [`TermionBackend`]: termion/struct.TermionBackend.html
//! [Crossterm]: https://crates.io/crates/crossterm
//! [Termion]: https://crates.io/crates/termion
//! [Termwiz]: https://crates.io/crates/termwiz
//! [Examples]: https://github.com/ratatui/ratatui/tree/main/examples/README.md
//! [Backend Comparison]:
//!     https://ratatui.rs/concepts/backends/comparison/
//! [Ratatui Website]: https://ratatui.rs
use std::io;

use strum::{Display, EnumString};

use crate::{
    buffer::Cell,
    layout::{Position, Size},
};

#[cfg(all(not(windows), feature = "termion"))]
mod termion;
#[cfg(all(not(windows), feature = "termion"))]
pub use self::termion::TermionBackend;

#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "crossterm")]
pub use self::crossterm::CrosstermBackend;

#[cfg(feature = "termwiz")]
mod termwiz;
#[cfg(feature = "termwiz")]
pub use self::termwiz::TermwizBackend;

mod test;
pub use self::test::TestBackend;

/// Enum representing the different types of clearing operations that can be performed
/// on the terminal screen.
#[derive(Debug, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ClearType {
    /// Clear the entire screen.
    All,
    /// Clear everything after the cursor.
    AfterCursor,
    /// Clear everything before the cursor.
    BeforeCursor,
    /// Clear the current line.
    CurrentLine,
    /// Clear everything from the cursor until the next newline.
    UntilNewLine,
}

/// The window size in characters (columns / rows) as well as pixels.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct WindowSize {
    /// Size of the window in characters (columns / rows).
    pub columns_rows: Size,
    /// Size of the window in pixels.
    ///
    /// The `pixels` fields may not be implemented by all terminals and return `0,0`. See
    /// <https://man7.org/linux/man-pages/man4/tty_ioctl.4.html> under section "Get and set window
    /// size" / TIOCGWINSZ where the fields are commented as "unused".
    pub pixels: Size,
}

/// The `Backend` trait provides an abstraction over different terminal libraries. It defines the
/// methods required to draw content, manipulate the cursor, and clear the terminal screen.
///
/// Most applications should not need to interact with the `Backend` trait directly as the
/// [`Terminal`] struct provides a higher level interface for interacting with the terminal.
///
/// [`Terminal`]: crate::terminal::Terminal
pub trait Backend {
    /// Draw the given content to the terminal screen.
    ///
    /// The content is provided as an iterator over `(u16, u16, &Cell)` tuples, where the first two
    /// elements represent the x and y coordinates, and the third element is a reference to the
    /// [`Cell`] to be drawn.
    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>;

    /// Insert `n` line breaks to the terminal screen.
    ///
    /// This method is optional and may not be implemented by all backends.
    fn append_lines(&mut self, _n: u16) -> io::Result<()> {
        Ok(())
    }

    /// Hide the cursor on the terminal screen.
    ///
    ///
    /// See also [`show_cursor`].
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::backend::{TestBackend};
    /// # let mut backend = TestBackend::new(80, 25);
    /// use ratatui::backend::Backend;
    ///
    /// backend.hide_cursor()?;
    /// // do something with hidden cursor
    /// backend.show_cursor()?;
    /// # std::io::Result::Ok(())
    /// ```
    ///
    /// [`show_cursor`]: Self::show_cursor
    fn hide_cursor(&mut self) -> io::Result<()>;

    /// Show the cursor on the terminal screen.
    ///
    /// See [`hide_cursor`] for an example.
    ///
    /// [`hide_cursor`]: Self::hide_cursor
    fn show_cursor(&mut self) -> io::Result<()>;

    /// Get the current cursor position on the terminal screen.
    ///
    /// The returned tuple contains the x and y coordinates of the cursor.
    /// The origin (0, 0) is at the top left corner of the screen.
    ///
    /// See [`set_cursor_position`] for an example.
    ///
    /// [`set_cursor_position`]: Self::set_cursor_position
    fn get_cursor_position(&mut self) -> io::Result<Position>;

    /// Set the cursor position on the terminal screen to the given x and y coordinates.
    ///
    /// The origin (0, 0) is at the top left corner of the screen.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::backend::{TestBackend};
    /// # let mut backend = TestBackend::new(80, 25);
    /// use ratatui::{backend::Backend, layout::Position};
    ///
    /// backend.set_cursor_position(Position { x: 10, y: 20 })?;
    /// assert_eq!(backend.get_cursor_position()?, Position { x: 10, y: 20 });
    /// # std::io::Result::Ok(())
    /// ```
    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> io::Result<()>;

    /// Get the current cursor position on the terminal screen.
    ///
    /// The returned tuple contains the x and y coordinates of the cursor. The origin
    /// (0, 0) is at the top left corner of the screen.
    #[deprecated = "the method get_cursor_position indicates more clearly what about the cursor to get"]
    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        let Position { x, y } = self.get_cursor_position()?;
        Ok((x, y))
    }

    /// Set the cursor position on the terminal screen to the given x and y coordinates.
    ///
    /// The origin (0, 0) is at the top left corner of the screen.
    #[deprecated = "the method set_cursor_position indicates more clearly what about the cursor to set"]
    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.set_cursor_position(Position { x, y })
    }

    /// Clears the whole terminal screen
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ratatui::backend::{TestBackend};
    /// # let mut backend = TestBackend::new(80, 25);
    /// use ratatui::backend::Backend;
    ///
    /// backend.clear()?;
    /// # std::io::Result::Ok(())
    /// ```
    fn clear(&mut self) -> io::Result<()>;

    /// Clears a specific region of the terminal specified by the [`ClearType`] parameter
    ///
    /// This method is optional and may not be implemented by all backends. The default
    /// implementation calls [`clear`] if the `clear_type` is [`ClearType::All`] and returns an
    /// error otherwise.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ratatui::{backend::{TestBackend}};
    /// # let mut backend = TestBackend::new(80, 25);
    /// use ratatui::backend::{Backend, ClearType};
    ///
    /// backend.clear_region(ClearType::All)?;
    /// # std::io::Result::Ok(())
    /// ```
    ///
    /// # Errors
    ///
    /// This method will return an error if the terminal screen could not be cleared. It will also
    /// return an error if the `clear_type` is not supported by the backend.
    ///
    /// [`clear`]: Self::clear
    fn clear_region(&mut self, clear_type: ClearType) -> io::Result<()> {
        match clear_type {
            ClearType::All => self.clear(),
            ClearType::AfterCursor
            | ClearType::BeforeCursor
            | ClearType::CurrentLine
            | ClearType::UntilNewLine => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("clear_type [{clear_type:?}] not supported with this backend"),
            )),
        }
    }

    /// Get the size of the terminal screen in columns/rows as a [`Size`].
    ///
    /// The returned [`Size`] contains the width and height of the terminal screen.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::{backend::{TestBackend}};
    /// # let backend = TestBackend::new(80, 25);
    /// use ratatui::{backend::Backend, layout::Size};
    ///
    /// assert_eq!(backend.size()?, Size::new(80, 25));
    /// # std::io::Result::Ok(())
    /// ```
    fn size(&self) -> io::Result<Size>;

    /// Get the size of the terminal screen in columns/rows and pixels as a [`WindowSize`].
    ///
    /// The reason for this not returning only the pixel size, given the redundancy with the
    /// `size()` method, is that the underlying backends most likely get both values with one
    /// syscall, and the user is also most likely to need columns and rows along with pixel size.
    fn window_size(&mut self) -> io::Result<WindowSize>;

    /// Flush any buffered content to the terminal screen.
    fn flush(&mut self) -> io::Result<()>;

    /// Scroll a region of the screen upwards, where a region is specified by a (half-open) range
    /// of rows.
    ///
    /// Each row in the region is replaced by the row `line_count` rows below it, except the bottom
    /// `line_count` rows, which are replaced by empty rows. If `line_count` is equal to or larger
    /// than the number of rows in the region, then all rows are replaced with empty rows.
    ///
    /// If the region includes row 0, then `line_count` rows are copied into the bottom of the
    /// scrollback buffer. These rows are first taken from the old contents of the region, starting
    /// from the top. If there aren't sufficient rows in the region, then the remainder are empty
    /// rows.
    ///
    /// The position of the cursor afterwards is undefined.
    ///
    /// The behavior is designed to match what ANSI terminals do when scrolling regions are
    /// established. With ANSI terminals, a scrolling region can be established with the "^[[X;Yr"
    /// sequence, where X and Y define the lines of the region. The scrolling region can be reset
    /// to be the whole screen with the "^[[r" sequence.
    ///
    /// When a scrolling region is established in an ANSI terminal, various operations' behaviors
    /// are changed in such a way that the scrolling region acts like a "virtual screen". In
    /// particular, the scrolling sequence "^[[NS", which scrolls lines up by a count of N.
    ///
    /// On an ANSI terminal, this method will probably translate to something like:
    /// "^[[X;Yr^[[NS^[[r". That is, set the scrolling region, scroll up, then reset the scrolling
    /// region.
    ///
    /// For examples of how this function is expected to work, refer to the tests for
    /// [`TestBackend::scroll_region_up`].
    #[cfg(feature = "scrolling-regions")]
    fn scroll_region_up(&mut self, region: std::ops::Range<u16>, line_count: u16)
        -> io::Result<()>;

    /// Scroll a region of the screen downwards, where a region is specified by a (half-open) range
    /// of rows.
    ///
    /// Each row in the region is replaced by the row `line_count` rows above it, except the top
    /// `line_count` rows, which are replaced by empty rows. If `line_count` is equal to or larger
    /// than the number of rows in the region, then all rows are replaced with empty rows.
    ///
    /// The position of the cursor afterwards is undefined.
    ///
    /// See the documentation for [`Self::scroll_region_down`] for more information about how this
    /// is expected to be implemented for ANSI terminals. All of that applies, except the ANSI
    /// sequence to scroll down is "^[[NT".
    ///
    /// This function is asymmetrical with regards to the scrollback buffer. The reason is that
    /// this how terminals seem to implement things.
    ///
    /// For examples of how this function is expected to work, refer to the tests for
    /// [`TestBackend::scroll_region_down`].
    #[cfg(feature = "scrolling-regions")]
    fn scroll_region_down(
        &mut self,
        region: std::ops::Range<u16>,
        line_count: u16,
    ) -> io::Result<()>;
}

#[cfg(test)]
mod tests {
    use strum::ParseError;

    use super::*;

    #[test]
    fn clear_type_tostring() {
        assert_eq!(ClearType::All.to_string(), "All");
        assert_eq!(ClearType::AfterCursor.to_string(), "AfterCursor");
        assert_eq!(ClearType::BeforeCursor.to_string(), "BeforeCursor");
        assert_eq!(ClearType::CurrentLine.to_string(), "CurrentLine");
        assert_eq!(ClearType::UntilNewLine.to_string(), "UntilNewLine");
    }

    #[test]
    fn clear_type_from_str() {
        assert_eq!("All".parse::<ClearType>(), Ok(ClearType::All));
        assert_eq!(
            "AfterCursor".parse::<ClearType>(),
            Ok(ClearType::AfterCursor)
        );
        assert_eq!(
            "BeforeCursor".parse::<ClearType>(),
            Ok(ClearType::BeforeCursor)
        );
        assert_eq!(
            "CurrentLine".parse::<ClearType>(),
            Ok(ClearType::CurrentLine)
        );
        assert_eq!(
            "UntilNewLine".parse::<ClearType>(),
            Ok(ClearType::UntilNewLine)
        );
        assert_eq!("".parse::<ClearType>(), Err(ParseError::VariantNotFound));
    }
}
