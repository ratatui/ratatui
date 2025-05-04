// show the feature flags in the generated documentation
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/favicon.ico"
)]
#![warn(missing_docs)]
//! This module provides the [`ConsoleBackend`] implementation for the [`Backend`] trait. It uses
//! the [Console] crate to interact with the terminal.
//!
//! [`Backend`]: ratatui_core::backend::Backend
//! [Console]: https://docs.rs/console
#![cfg_attr(feature = "document-features", doc = "\n## Features")]
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]

use std::fmt;
use std::io::{self, Write};

pub use console;
use console::Style as ConsoleStyle;
use console::{Attribute, Color as ConsoleColor};
use ratatui_core::backend::{Backend, ClearType, WindowSize};
use ratatui_core::buffer::Cell;
use ratatui_core::layout::{Position, Size};
use ratatui_core::style::{Color, Modifier, Style};

/// A [`Backend`] implementation that uses [Console] to render to the terminal.
///
/// The `ConsoleBackend` struct is a wrapper around a writer implementing [`Write`], which is used
/// to send commands to the terminal. It provides methods for drawing content, manipulating the
/// cursor, and clearing the terminal screen.
///
/// Most applications should not call the methods on `ConsoleBackend` directly, but will instead
/// use the [`Terminal`] struct, which provides a more ergonomic interface.
///
/// Usually applications will enable raw mode and switch to alternate screen mode when starting.
/// This is done by calling [`IntoRawMode::into_raw_mode()`] and
/// [`IntoAlternateScreen::into_alternate_screen()`] on the writer before creating the backend.
/// This is not done automatically by the backend because it is possible that the application may
/// want to use the terminal for other purposes (like showing help text) before entering alternate
/// screen mode. This backend automatically disable raw mode and switches back to the primary
/// screen when the writer is dropped.
///
/// # Example
///
/// ```rust,no_run
/// use std::io::{stderr, stdout};
///
/// use ratatui::backend::ConsoleBackend;
/// use ratatui::termion::raw::IntoRawMode;
/// use ratatui::termion::screen::IntoAlternateScreen;
/// use ratatui::Terminal;
///
/// let writer = stdout().into_raw_mode()?.into_alternate_screen()?;
/// let mut backend = ConsoleBackend::new(writer);
/// // or
/// let writer = stderr().into_raw_mode()?.into_alternate_screen()?;
/// let backend = ConsoleBackend::new(stderr());
/// let mut terminal = Terminal::new(backend)?;
///
/// terminal.clear()?;
/// terminal.draw(|frame| {
///     // -- snip --
/// })?;
/// # std::io::Result::Ok(())
/// ```
///
/// [`IntoRawMode::into_raw_mode()`]: termion::raw::IntoRawMode
/// [`IntoAlternateScreen::into_alternate_screen()`]: termion::screen::IntoAlternateScreen
/// [`Terminal`]: ratatui_core::terminal::Terminal
/// [Console]: https://docs.rs/console
// #[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
// pub struct ConsoleBackend<W>
// where
//     W: Write,
// {
//     writer: W,
// }
#[derive(Debug, Clone)]
pub struct ConsoleBackend {
    term: console::Term,
}
impl Default for ConsoleBackend {
    fn default() -> Self {
        Self::new(true, false)
    }
}
impl ConsoleBackend {
    /// create a new [`ConsoleBackend`] with `term` [`console::Term`]
    pub fn with_term(term: console::Term) -> Self {
        Self { term }
    }
    /// if not `stdout`, then `stderr`, if not `buffered`, then `unbuffered`
    pub fn new(stdout: bool, buffered: bool) -> Self {
        if stdout {
            if buffered {
                Self::with_term(console::Term::buffered_stdout())
            } else {
                Self::with_term(console::Term::stdout())
            }
        } else {
            if buffered {
                Self::with_term(console::Term::buffered_stderr())
            } else {
                Self::with_term(console::Term::stderr())
            }
        }
    }
    /// get the underlying [`console::Term`]
    pub fn get_term(&self) -> &console::Term {
        &self.term
    }
    /// get the underlying [`console::Term`]
    pub fn get_term_mut(&mut self) -> &mut console::Term {
        &mut self.term
    }
}

impl Backend for ConsoleBackend {
    type Error = io::Error;

    fn clear(&mut self) -> io::Result<()> {
        self.clear_region(ClearType::All)
    }

    fn clear_region(&mut self, clear_type: ClearType) -> Result<(), Self::Error> {
        match clear_type {
            ClearType::All => self.term.clear_screen(),
            ClearType::AfterCursor => self.term.clear_to_end_of_screen(),
            ClearType::BeforeCursor => {
                unimplemented!("console doesn't support clearing BeforeCursor")
            }
            ClearType::CurrentLine => self.term.clear_line(),
            ClearType::UntilNewLine => {
                unimplemented!("console doesn't support clearing until newline")
            }
        }
        // self.term.flush()
    }

    // fn clear_region(&mut self, clear_type: ClearType) -> io::Result<()> {
    //     // let term = console::Term::
    //     match clear_type {
    //         ClearType::All => write!(self.writer, "{}", termion::clear::All)?,
    //         ClearType::AfterCursor => write!(self.writer, "{}", termion::clear::AfterCursor)?,
    //         ClearType::BeforeCursor => write!(self.writer, "{}", termion::clear::BeforeCursor)?,
    //         ClearType::CurrentLine => write!(self.writer, "{}", termion::clear::CurrentLine)?,
    //         ClearType::UntilNewLine => write!(self.writer, "{}", termion::clear::UntilNewline)?,
    //     }
    //     self.writer.flush()
    // }

    // fn append_lines(&mut self, n: u16) -> io::Result<()> {
    //     for _ in 0..n {
    //         self.term.write_line("")?;
    //     }
    //     self.term.flush()
    // }

    fn hide_cursor(&mut self) -> Result<(), Self::Error> {
        self.term.hide_cursor()
        // self.term.flush()
    }

    fn show_cursor(&mut self) -> Result<(), Self::Error> {
        self.term.show_cursor()
        // self.term.flush()
    }

    // fn get_cursor_position(&mut self) -> io::Result<Position> {
    //     termion::cursor::DetectCursorPos::cursor_pos(&mut self.writer)
    //         .map(|(x, y)| Position { x: x - 1, y: y - 1 })
    // }
    fn get_cursor_position(&mut self) -> Result<Position, Self::Error> {
        unimplemented!("console doesn't support getting cursor position")
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> Result<(), Self::Error> {
        let Position { x, y } = position.into();
        // TODO: do we need `x + 1`, `y + 1`?
        self.term.move_cursor_to(usize::from(x), usize::from(y))
        // self.term.flush()
    }

    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        use std::fmt::Write;

        let mut buf = String::with_capacity(content.size_hint().0 * 3);
        let mut fg = Color::Reset;
        let mut bg = Color::Reset;
        let mut modifier = Modifier::empty();
        let mut last_pos: Option<Position> = None;
        for (x, y, cell) in content {
            // Move the cursor if the previous location was not (x - 1, y)
            if !matches!(last_pos, Some(p) if x == p.x + 1 && y == p.y) {
                self.term
                    .move_cursor_to(usize::from(x + 1), usize::from(y + 1))?;
            }
            last_pos = Some(Position { x, y });
            if cell.modifier != modifier {
                let mod_diff = ModifierDiff {
                    from: modifier,
                    to: cell.modifier,
                };
                write!(buf, "{mod_diff}").unwrap();
                modifier = cell.modifier;
            }
            if cell.fg != fg {
                write!(buf, "{}", console::style("").fg(cell.fg.into_console())).unwrap();
                fg = cell.fg;
            }
            if cell.bg != bg {
                write!(buf, "{}", console::style("").fg(cell.bg.into_console())).unwrap();
                bg = cell.bg;
            }
            buf.push_str(cell.symbol());
        }
        // write!(self.term, "{buf}")?;
        self.term.write_line(&buf)
        // self.term.flush()
    }
    // fn draw<'a, I>(&mut self, content: I) -> Result<(), Self::Error>
    // where
    //     I: Iterator<Item = (u16, u16, &'a Cell)>,
    // {
    //     todo!()
    // }

    fn size(&self) -> Result<Size, Self::Error> {
        let console_size = self
            .term
            .size_checked()
            .ok_or(io::Error::new(io::ErrorKind::Other, "not a tty"))?;
        Ok(Size::from(console_size))
    }

    // fn window_size(&mut self) -> io::Result<WindowSize> {
    //     Ok(WindowSize {
    //         columns_rows: termion::terminal_size()?.into(),
    //         pixels: termion::terminal_size_pixels()?.into(),
    //     })
    // }
    fn window_size(&mut self) -> Result<WindowSize, Self::Error> {
        unimplemented!("console doesn't support querying window size in pixels")
    }

    fn flush(&mut self) -> io::Result<()> {
        self.term.flush()
    }

    // #[cfg(feature = "scrolling-regions")]
    // fn scroll_region_up(&mut self, region: std::ops::Range<u16>, amount: u16) -> io::Result<()> {
    //     write!(
    //         self.writer,
    //         "{}{}{}",
    //         SetRegion(region.start.saturating_add(1), region.end),
    //         termion::scroll::Up(amount),
    //         ResetRegion,
    //     )?;
    //     self.writer.flush()
    // }

    // #[cfg(feature = "scrolling-regions")]
    // fn scroll_region_down(&mut self, region: std::ops::Range<u16>, amount: u16) -> io::Result<()> {
    //     write!(
    //         self.writer,
    //         "{}{}{}",
    //         SetRegion(region.start.saturating_add(1), region.end),
    //         termion::scroll::Down(amount),
    //         ResetRegion,
    //     )?;
    //     self.writer.flush()
    // }
}

/// A trait for converting a Ratatui type to a Console type.
///
/// This trait is needed for avoiding the orphan rule when implementing `From` for console types
/// once these are moved to a separate crate.
pub trait IntoConsole<C> {
    /// Converts the ratatui type to a console type.
    fn into_console(self) -> C;
}

/// A trait for converting a Console type to a Ratatui type.
///
/// This trait is needed for avoiding the orphan rule when implementing `From` for console types
/// once these are moved to a separate crate.
pub trait FromConsole<C> {
    /// Converts the console type to a ratatui type.
    fn from_console(value: C) -> Self;
}

impl IntoConsole<ConsoleColor> for Color {
    fn into_console(self) -> ConsoleColor {
        match self {
            Self::Reset => ConsoleColor::Black,
            Self::Black => ConsoleColor::Black,
            Self::Gray | Self::DarkGray => ConsoleColor::Black,
            Self::Red | Self::LightRed => ConsoleColor::Red,
            Self::Green | Self::LightGreen => ConsoleColor::Green,
            Self::Blue | Self::LightBlue => ConsoleColor::Blue,
            Self::Yellow | Self::LightYellow => ConsoleColor::Yellow,
            Self::Magenta | Self::LightMagenta => ConsoleColor::Magenta,
            Self::Cyan | Self::LightCyan => ConsoleColor::Cyan,
            Self::White => ConsoleColor::White,
            Self::Indexed(i) => ConsoleColor::Color256(i),
            // Self::Rgb(r, g, b) => ConsoleColor::Rgb { r, g, b },
            col => unimplemented!("console doesn't support other colors such as: {col}"),
        }
    }
}

impl FromConsole<ConsoleColor> for Color {
    fn from_console(value: ConsoleColor) -> Self {
        match value {
            // ConsoleColor::Reset => Self::Reset,
            ConsoleColor::Black => Self::Black,
            // ConsoleColor::DarkRed => Self::Red,
            // ConsoleColor::DarkGreen => Self::Green,
            // ConsoleColor::DarkYellow => Self::Yellow,
            // ConsoleColor::DarkBlue => Self::Blue,
            // ConsoleColor::DarkMagenta => Self::Magenta,
            // ConsoleColor::DarkCyan => Self::Cyan,
            // ConsoleColor::Grey => Self::Gray,
            // ConsoleColor::DarkGrey => Self::DarkGray,
            ConsoleColor::Red => Self::LightRed,
            ConsoleColor::Green => Self::LightGreen,
            ConsoleColor::Blue => Self::LightBlue,
            ConsoleColor::Yellow => Self::LightYellow,
            ConsoleColor::Magenta => Self::LightMagenta,
            ConsoleColor::Cyan => Self::LightCyan,
            ConsoleColor::White => Self::White,
            // ConsoleColor::Rgb { r, g, b } => Self::Rgb(r, g, b),
            ConsoleColor::Color256(v) => Self::Indexed(v),
            _ => unimplemented!("console doesn't support more colors"),
        }
    }
}

/// The `ModifierDiff` struct is used to calculate the difference between two `Modifier`
/// values. This is useful when updating the terminal display, as it allows for more
/// efficient updates by only sending the necessary changes.
struct ModifierDiff {
    from: Modifier,
    to: Modifier,
}

impl fmt::Display for ModifierDiff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let remove = self.from - self.to;
        // todo!()

        let add = self.to - self.from;
        if add.contains(Modifier::REVERSED) {
            // write!(f, "{}", console::style("").attr(Attribute::) termion::style::Invert)?;
        }
        if add.contains(Modifier::BOLD) {
            write!(f, "{}", console::style("").attr(Attribute::Bold))?;
        }
        if add.contains(Modifier::ITALIC) {
            write!(f, "{}", console::style("").attr(Attribute::Italic))?;
        }
        if add.contains(Modifier::UNDERLINED) {
            write!(f, "{}", console::style("").attr(Attribute::Underlined))?;
        }
        if add.contains(Modifier::DIM) {
            write!(f, "{}", console::style("").attr(Attribute::Dim))?;
        }
        if add.contains(Modifier::CROSSED_OUT) {
            write!(f, "{}", console::style("").attr(Attribute::StrikeThrough))?;
        }
        if add.contains(Modifier::SLOW_BLINK) {
            write!(f, "{}", console::style("").attr(Attribute::Blink))?;
        }
        if add.contains(Modifier::RAPID_BLINK) {
            write!(f, "{}", console::style("").attr(Attribute::BlinkFast))?;
        }

        Ok(())
    }
}
