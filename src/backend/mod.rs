//! This module provides the backend implementations for different terminal libraries.
//! It defines the [`Backend`] trait which is used to abstract over the specific
//! terminal library being used.
//!
//! The following terminal libraries are supported:
//! - Crossterm (with the `crossterm` feature)
//! - Termion (with the `termion` feature)
//! - Termwiz (with the `termwiz` feature)
//!
//! Additionally, a [`TestBackend`] is provided for testing purposes.
//!
//! # Example
//!
//! ```rust
//! use ratatui::backend::{Backend, CrosstermBackend};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let buffer = std::io::stdout();
//! let mut backend = CrosstermBackend::new(buffer);
//! backend.clear()?;
//! # Ok(())
//! # }
//! ```
//!
//! [`Backend`]: trait.Backend.html
//! [`TestBackend`]: struct.TestBackend.html

use std::io;

use strum::{Display, EnumString};

use crate::{buffer::Cell, layout::Rect};

#[cfg(feature = "termion")]
mod termion;
#[cfg(feature = "termion")]
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
    All,
    AfterCursor,
    BeforeCursor,
    CurrentLine,
    UntilNewLine,
}

/// The `Backend` trait provides an abstraction over different terminal libraries.
/// It defines the methods required to draw content, manipulate the cursor, and
/// clear the terminal screen.
pub trait Backend {
    /// Draw the given content to the terminal screen.
    ///
    /// The content is provided as an iterator over `(u16, u16, &Cell)` tuples,
    /// where the first two elements represent the x and y coordinates, and the
    /// third element is a reference to the [`Cell`] to be drawn.
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>;

    /// Insert `n` line breaks to the terminal screen.
    ///
    /// This method is optional and may not be implemented by all backends.
    fn append_lines(&mut self, _n: u16) -> io::Result<()> {
        Ok(())
    }

    /// Hide the cursor on the terminal screen.
    fn hide_cursor(&mut self) -> Result<(), io::Error>;

    /// Show the cursor on the terminal screen.
    fn show_cursor(&mut self) -> Result<(), io::Error>;

    /// Get the current cursor position on the terminal screen.
    fn get_cursor(&mut self) -> Result<(u16, u16), io::Error>;

    /// Set the cursor position on the terminal screen to the given x and y coordinates.
    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), io::Error>;

    /// Clears the whole terminal screen
    fn clear(&mut self) -> Result<(), io::Error>;

    /// Clears a specific region of the terminal specified by the [`ClearType`] parameter
    ///
    /// This method is optional and may not be implemented by all backends.
    fn clear_region(&mut self, clear_type: ClearType) -> Result<(), io::Error> {
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

    /// Get the size of the terminal screen as a [`Rect`].
    fn size(&self) -> Result<Rect, io::Error>;

    /// Flush any buffered content to the terminal screen.
    fn flush(&mut self) -> Result<(), io::Error>;
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
