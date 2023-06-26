//! This module provides the [`TermionBackend`] implementation for the [`Backend`] trait. It uses
//! the [Termion] crate to interact with the terminal.
//!
//! [`Backend`]: crate::backend::Backend
//! [`TermionBackend`]: crate::backend::TermionBackend
/// [Termion]: https://docs.rs/termion
use std::{
    fmt,
    io::{self, stdout, Stderr, Stdout, Write},
};

use termion::{
    input::MouseTerminal,
    raw::{IntoRawMode, RawTerminal},
    screen::{AlternateScreen, IntoAlternateScreen},
};

use crate::{
    backend::{Backend, ClearType},
    buffer::Cell,
    layout::Rect,
    style::{Color, Modifier},
};
/// A [`Backend`] implementation that uses [Termion] to render to the terminal.
///
/// The `TermionBackend` struct is a wrapper around a type implementing [`std::io::Write`], which
/// is used to send commands to the terminal. It provides methods for drawing content, manipulating
/// the cursor, and clearing the terminal screen.
///
/// # Examples
///
/// Generally apps should [`TermionBackend::on_stderr()`] or [`TermionBackend::on_stdout()`]
/// methods, which both enable raw mode and enter the alternate screen. Choosing `stderr` over
/// `stdout` ensures your app displays on the terminal even if the standard output stream is
/// redirected and makes it easy to write apps that can be piped to other programs.
///
/// ```no_run termion cannot set the terminal to raw mode in the doc tests
/// use ratatui::backend::TermionBackend;
///
/// let mut backend = TermionBackend::on_stdout()?;
/// // alternatively
/// let mut backend = TermionBackend::on_stderr()?;
/// # std::io::Result::Ok(())
/// ```
///
/// For more control over raw mode, mouse capture or the alternate screen, use
/// [`TermionBackend::new()`].
///
/// ```no_run termion cannot set the terminal to raw mode in the doc tests
/// use std::io::stdout;
/// use ratatui::backend::TermionBackend;
///
/// let mut backend = TermionBackend::new(stdout())
///     .with_raw_mode()?
///     .with_alternate_screen()?
///     .with_mouse_capture()?;
/// # std::io::Result::Ok(())
/// ```
///
/// [Termion]: https://docs.rs/termion
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TermionBackend<W>
where
    W: Write,
{
    stdout: W,
}

impl<W> TermionBackend<W>
where
    W: Write,
{
    /// Creates a new Termion backend with the given buffer.
    ///
    /// # Example
    ///
    /// ```no_run termion cannot set the terminal to raw mode in the doc tests
    /// # use std::io::stdout;
    /// # use ratatui::backend::TermionBackend;
    /// let mut backend = TermionBackend::new(stdout());
    /// # std::io::Result::Ok(())
    /// ```
    pub fn new(stdout: W) -> TermionBackend<W> {
        TermionBackend { stdout }
    }

    /// Builder pattern method to enable raw mode.
    ///
    /// See [`crate::backend#raw-mode`] for more information.
    ///
    /// # Example
    ///
    /// ```no_run termion cannot set the terminal to raw mode in the doc tests
    /// # use std::io::stdout;
    /// # use ratatui::backend::TermionBackend;
    /// let mut backend = TermionBackend::new(stdout()).with_raw_mode()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn with_raw_mode(self) -> io::Result<TermionBackend<RawTerminal<W>>> {
        let stdout = self.stdout.into_raw_mode()?;
        Ok(TermionBackend { stdout })
    }

    /// Builder pattern method to enter alternate screen.
    ///
    /// See [`crate::backend#alternate-screen`] for more information.
    ///
    /// # Example
    ///
    /// ```no_run termion cannot set the terminal to raw mode in the doc tests
    /// # use std::io::stdout;
    /// # use ratatui::backend::TermionBackend;
    /// let mut backend = TermionBackend::new(stdout()).with_alternate_screen()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn with_alternate_screen(self) -> io::Result<TermionBackend<AlternateScreen<W>>> {
        let stdout = self.stdout.into_alternate_screen()?;
        Ok(TermionBackend { stdout })
    }

    /// Builder pattern method to enable mouse capture.
    ///
    /// See [`crate::backend#mouse-capture`] for more information.
    ///
    /// The backend will disable mouse capture when dropped.
    ///
    /// # Example
    ///
    /// ```no_run termion cannot set the terminal to raw mode in the doc tests
    /// # use std::io::stdout;
    /// # use ratatui::backend::TermionBackend;
    /// let mut backend = TermionBackend::new(stdout()).with_mouse_capture()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn with_mouse_capture(self) -> io::Result<TermionBackend<MouseTerminal<W>>> {
        let stdout = MouseTerminal::from(self.stdout);
        Ok(TermionBackend { stdout })
    }

    /// Enters the alternate screen.
    ///
    /// Note that this does not get reverted when the backend is dropped unless the backend is one
    /// which wraps stdout or stderr with AlternateScreen. If you want to ensure the alternate
    /// screen is left when the backend is dropped, use [`TermionBackend::with_alternate_screen()`]
    /// instead.
    ///
    /// See [`crate::backend#alternate-screen`] for more information.
    ///
    /// # Example
    ///
    /// ```no_run termion cannot set the terminal to raw mode in the doc tests
    /// # use std::io::stdout;
    /// # use ratatui::backend::TermionBackend;
    /// let mut backend = TermionBackend::new(stdout());
    /// backend.enter_alternate_screen()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn enter_alternate_screen(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", termion::screen::ToAlternateScreen)
    }

    /// Leaves the alternate screen.
    ///
    /// This is necessary if you are using [`TermionBackend::enter_alternate_screen()`] and want
    /// to leave the alternate screen before the backend is dropped.
    ///
    /// See [`crate::backend#alternate-screen`] for more information.
    ///
    /// # Example
    ///
    /// ```no_run termion cannot set the terminal to raw mode in the doc tests
    /// # use std::io::stdout;
    /// # use ratatui::backend::TermionBackend;
    /// let mut backend = TermionBackend::new(stdout());
    /// backend.enter_alternate_screen()?;
    /// backend.leave_alternate_screen()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn leave_alternate_screen(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", termion::screen::ToMainScreen)
    }
}

impl TermionBackend<AlternateScreen<RawTerminal<Stdout>>> {
    /// Creates a new Termion backend on stdout.
    ///
    /// The backend is created with [raw mode] enabled and the [alternate screen] active.
    /// The backend will disable raw mode and leave the alternate screen when dropped.
    ///
    /// # Example
    ///
    /// ```no_run termion cannot set the terminal to raw mode in the doc tests
    /// # use ratatui::backend::TermionBackend;
    /// let mut backend = TermionBackend::on_stdout()?;
    /// # std::io::Result::Ok(())
    /// ```
    ///
    /// [raw mode]: crate::backend#raw-mode
    /// [alternate screen]: crate::backend#alternate-screen
    pub fn on_stdout() -> io::Result<TermionBackend<AlternateScreen<RawTerminal<Stdout>>>> {
        let stdout = stdout().into_raw_mode()?.into_alternate_screen()?;
        Ok(TermionBackend::new(stdout))
    }
}

impl TermionBackend<AlternateScreen<RawTerminal<Stderr>>> {
    /// Creates a new Termion backend on stderr.
    ///
    /// The backend is created with [raw mode] enabled and the [alternate screen] active.
    /// The backend will disable raw mode and leave the alternate screen when dropped.
    ///
    /// # Example
    ///
    /// ```no_run termion cannot set the terminal to raw mode in the doc tests
    /// # use ratatui::backend::TermionBackend;
    /// let mut backend = TermionBackend::on_stderr()?;
    /// # std::io::Result::Ok(())
    /// ```
    ///
    /// [raw mode]: crate::backend#raw-mode
    /// [alternate screen]: crate::backend#alternate-screen
    pub fn on_stderr() -> io::Result<TermionBackend<AlternateScreen<RawTerminal<Stderr>>>> {
        let stderr = io::stderr().into_raw_mode()?.into_alternate_screen()?;
        Ok(TermionBackend::new(stderr))
    }
}

impl<W> TermionBackend<AlternateScreen<RawTerminal<W>>>
where
    W: Write,
{
    /// Enable raw mode on the terminal.
    ///
    /// See [`crate::backend#raw-mode`] for more information.
    ///
    /// The backend will disable raw mode when it is dropped.
    ///
    /// # Example
    ///
    /// ```rust,no_run termion cannot set the terminal to raw mode in the doc tests
    /// # use ratatui::backend::{Backend, TermionBackend};
    /// let mut backend = TermionBackend::on_stdout()?;
    /// backend.disable_raw_mode()?;
    /// // do stuff outside of raw mode
    /// backend.enable_raw_mode()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn enable_raw_mode(&mut self) -> io::Result<()> {
        self.stdout.suspend_raw_mode()
    }

    /// Disable raw mode on the terminal.
    ///
    /// See [`crate::backend#raw-mode`] for more information.
    ///
    /// # Example
    ///
    /// ```rust,no_run termion cannot set the terminal to raw mode in the doc tests
    /// # use std::io::stdout;
    /// # use ratatui::backend::{Backend, TermionBackend};
    /// let mut backend = TermionBackend::on_stdout()?;
    /// backend.disable_raw_mode()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn disable_raw_mode(&mut self) -> io::Result<()> {
        self.stdout.activate_raw_mode()
    }
}

impl<W> TermionBackend<RawTerminal<W>>
where
    W: Write,
{
    /// Enables raw mode on the backend.
    ///
    /// See [`crate::backend#raw-mode`] for more information.
    ///
    /// The backend will disable raw mode when it is dropped.
    ///
    /// # Example
    ///
    /// ```rust,no_run termion cannot set the terminal to raw mode in the doc tests
    /// # use ratatui::backend::{Backend, TermionBackend};
    /// let mut backend = TermionBackend::new(std::io::stdout()).with_raw_mode()?;
    /// backend.disable_raw_mode()?;
    /// // do stuff outside of raw mode
    /// backend.enable_raw_mode()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn enable_raw_mode(&mut self) -> io::Result<()> {
        self.stdout.suspend_raw_mode()
    }

    /// Disables raw mode on the backend.
    ///
    /// See [`crate::backend#raw-mode`] for more information.
    ///
    /// # Example
    ///
    /// ```rust,no_run termion cannot set the terminal to raw mode in the doc tests
    /// # use ratatui::backend::{Backend, TermionBackend};
    /// let mut backend = TermionBackend::new(std::io::stdout()).with_raw_mode()?;
    /// backend.disable_raw_mode()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn disable_raw_mode(&mut self) -> io::Result<()> {
        self.stdout.activate_raw_mode()
    }
}

impl Default for TermionBackend<Stdout> {
    /// Creates a new Termion backend on stdout.
    ///
    /// It will usually be more convenient to use [`TermionBackend::on_stdout()`], which enables
    /// raw mode and enters the alternate screen.
    ///
    /// # Example
    ///
    /// ```no_run termion cannot set the terminal to raw mode in the doc tests
    /// # use std::io::Stdout;
    /// # use ratatui::backend::TermionBackend;
    /// let mut backend = TermionBackend::<Stdout>::default();
    /// ```
    fn default() -> TermionBackend<Stdout> {
        TermionBackend::new(stdout())
    }
}

impl Default for TermionBackend<Stderr> {
    /// Creates a new Termion backend on stderr.
    ///
    /// It will usually be more convenient to use [`TermionBackend::on_stderr()`], which enables
    /// raw mode and enters the alternate screen.
    ///
    /// # Example
    ///
    /// ```no_run termion cannot set the terminal to raw mode in the doc tests
    /// # use std::io::Stderr;
    /// # use ratatui::backend::TermionBackend;
    /// let mut backend = TermionBackend::<Stderr>::default();
    /// ```
    fn default() -> TermionBackend<Stderr> {
        TermionBackend::new(io::stderr())
    }
}

impl<W> Write for TermionBackend<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl<W> Backend for TermionBackend<W>
where
    W: Write,
{
    fn clear(&mut self) -> io::Result<()> {
        self.clear_region(ClearType::All)
    }

    fn clear_region(&mut self, clear_type: ClearType) -> io::Result<()> {
        match clear_type {
            ClearType::All => write!(self.stdout, "{}", termion::clear::All)?,
            ClearType::AfterCursor => write!(self.stdout, "{}", termion::clear::AfterCursor)?,
            ClearType::BeforeCursor => write!(self.stdout, "{}", termion::clear::BeforeCursor)?,
            ClearType::CurrentLine => write!(self.stdout, "{}", termion::clear::CurrentLine)?,
            ClearType::UntilNewLine => write!(self.stdout, "{}", termion::clear::UntilNewline)?,
        };
        self.stdout.flush()
    }

    fn append_lines(&mut self, n: u16) -> io::Result<()> {
        for _ in 0..n {
            writeln!(self.stdout)?;
        }
        self.stdout.flush()
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", termion::cursor::Hide)?;
        self.stdout.flush()
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", termion::cursor::Show)?;
        self.stdout.flush()
    }

    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        termion::cursor::DetectCursorPos::cursor_pos(&mut self.stdout).map(|(x, y)| (x - 1, y - 1))
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        write!(self.stdout, "{}", termion::cursor::Goto(x + 1, y + 1))?;
        self.stdout.flush()
    }

    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        use std::fmt::Write;

        let mut string = String::with_capacity(content.size_hint().0 * 3);
        let mut fg = Color::Reset;
        let mut bg = Color::Reset;
        let mut modifier = Modifier::empty();
        let mut last_pos: Option<(u16, u16)> = None;
        for (x, y, cell) in content {
            // Move the cursor if the previous location was not (x - 1, y)
            if !matches!(last_pos, Some(p) if x == p.0 + 1 && y == p.1) {
                write!(string, "{}", termion::cursor::Goto(x + 1, y + 1)).unwrap();
            }
            last_pos = Some((x, y));
            if cell.modifier != modifier {
                write!(
                    string,
                    "{}",
                    ModifierDiff {
                        from: modifier,
                        to: cell.modifier
                    }
                )
                .unwrap();
                modifier = cell.modifier;
            }
            if cell.fg != fg {
                write!(string, "{}", Fg(cell.fg)).unwrap();
                fg = cell.fg;
            }
            if cell.bg != bg {
                write!(string, "{}", Bg(cell.bg)).unwrap();
                bg = cell.bg;
            }
            string.push_str(&cell.symbol);
        }
        write!(
            self.stdout,
            "{string}{}{}{}",
            Fg(Color::Reset),
            Bg(Color::Reset),
            termion::style::Reset,
        )
    }

    fn size(&self) -> io::Result<Rect> {
        let terminal = termion::terminal_size()?;
        Ok(Rect::new(0, 0, terminal.0, terminal.1))
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
struct Fg(Color);

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
struct Bg(Color);

/// The `ModifierDiff` struct is used to calculate the difference between two `Modifier`
/// values. This is useful when updating the terminal display, as it allows for more
/// efficient updates by only sending the necessary changes.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
struct ModifierDiff {
    from: Modifier,
    to: Modifier,
}

impl fmt::Display for Fg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use termion::color::Color as TermionColor;
        match self.0 {
            Color::Reset => termion::color::Reset.write_fg(f),
            Color::Black => termion::color::Black.write_fg(f),
            Color::Red => termion::color::Red.write_fg(f),
            Color::Green => termion::color::Green.write_fg(f),
            Color::Yellow => termion::color::Yellow.write_fg(f),
            Color::Blue => termion::color::Blue.write_fg(f),
            Color::Magenta => termion::color::Magenta.write_fg(f),
            Color::Cyan => termion::color::Cyan.write_fg(f),
            Color::Gray => termion::color::White.write_fg(f),
            Color::DarkGray => termion::color::LightBlack.write_fg(f),
            Color::LightRed => termion::color::LightRed.write_fg(f),
            Color::LightGreen => termion::color::LightGreen.write_fg(f),
            Color::LightBlue => termion::color::LightBlue.write_fg(f),
            Color::LightYellow => termion::color::LightYellow.write_fg(f),
            Color::LightMagenta => termion::color::LightMagenta.write_fg(f),
            Color::LightCyan => termion::color::LightCyan.write_fg(f),
            Color::White => termion::color::LightWhite.write_fg(f),
            Color::Indexed(i) => termion::color::AnsiValue(i).write_fg(f),
            Color::Rgb(r, g, b) => termion::color::Rgb(r, g, b).write_fg(f),
        }
    }
}
impl fmt::Display for Bg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use termion::color::Color as TermionColor;
        match self.0 {
            Color::Reset => termion::color::Reset.write_bg(f),
            Color::Black => termion::color::Black.write_bg(f),
            Color::Red => termion::color::Red.write_bg(f),
            Color::Green => termion::color::Green.write_bg(f),
            Color::Yellow => termion::color::Yellow.write_bg(f),
            Color::Blue => termion::color::Blue.write_bg(f),
            Color::Magenta => termion::color::Magenta.write_bg(f),
            Color::Cyan => termion::color::Cyan.write_bg(f),
            Color::Gray => termion::color::White.write_bg(f),
            Color::DarkGray => termion::color::LightBlack.write_bg(f),
            Color::LightRed => termion::color::LightRed.write_bg(f),
            Color::LightGreen => termion::color::LightGreen.write_bg(f),
            Color::LightBlue => termion::color::LightBlue.write_bg(f),
            Color::LightYellow => termion::color::LightYellow.write_bg(f),
            Color::LightMagenta => termion::color::LightMagenta.write_bg(f),
            Color::LightCyan => termion::color::LightCyan.write_bg(f),
            Color::White => termion::color::LightWhite.write_bg(f),
            Color::Indexed(i) => termion::color::AnsiValue(i).write_bg(f),
            Color::Rgb(r, g, b) => termion::color::Rgb(r, g, b).write_bg(f),
        }
    }
}

impl fmt::Display for ModifierDiff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let remove = self.from - self.to;
        if remove.contains(Modifier::REVERSED) {
            write!(f, "{}", termion::style::NoInvert)?;
        }
        if remove.contains(Modifier::BOLD) {
            // XXX: the termion NoBold flag actually enables double-underline on ECMA-48 compliant
            // terminals, and NoFaint additionally disables bold... so we use this trick to get
            // the right semantics.
            write!(f, "{}", termion::style::NoFaint)?;

            if self.to.contains(Modifier::DIM) {
                write!(f, "{}", termion::style::Faint)?;
            }
        }
        if remove.contains(Modifier::ITALIC) {
            write!(f, "{}", termion::style::NoItalic)?;
        }
        if remove.contains(Modifier::UNDERLINED) {
            write!(f, "{}", termion::style::NoUnderline)?;
        }
        if remove.contains(Modifier::DIM) {
            write!(f, "{}", termion::style::NoFaint)?;

            // XXX: the NoFaint flag additionally disables bold as well, so we need to re-enable it
            // here if we want it.
            if self.to.contains(Modifier::BOLD) {
                write!(f, "{}", termion::style::Bold)?;
            }
        }
        if remove.contains(Modifier::CROSSED_OUT) {
            write!(f, "{}", termion::style::NoCrossedOut)?;
        }
        if remove.contains(Modifier::SLOW_BLINK) || remove.contains(Modifier::RAPID_BLINK) {
            write!(f, "{}", termion::style::NoBlink)?;
        }

        let add = self.to - self.from;
        if add.contains(Modifier::REVERSED) {
            write!(f, "{}", termion::style::Invert)?;
        }
        if add.contains(Modifier::BOLD) {
            write!(f, "{}", termion::style::Bold)?;
        }
        if add.contains(Modifier::ITALIC) {
            write!(f, "{}", termion::style::Italic)?;
        }
        if add.contains(Modifier::UNDERLINED) {
            write!(f, "{}", termion::style::Underline)?;
        }
        if add.contains(Modifier::DIM) {
            write!(f, "{}", termion::style::Faint)?;
        }
        if add.contains(Modifier::CROSSED_OUT) {
            write!(f, "{}", termion::style::CrossedOut)?;
        }
        if add.contains(Modifier::SLOW_BLINK) || add.contains(Modifier::RAPID_BLINK) {
            write!(f, "{}", termion::style::Blink)?;
        }

        Ok(())
    }
}
