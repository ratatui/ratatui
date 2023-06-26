//! This module provides the [`CrosstermBackend`] implementation for the [`Backend`] trait. It uses
//! the [Crossterm] crate to interact with the terminal.
//!
//! [`Backend`]: super::Backend
/// [Crossterm]: https://docs.rs/crossterm
use std::io::{self, Stderr, Stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute, queue,
    style::{
        Attribute as CAttribute, Color as CColor, Print, SetAttribute, SetBackgroundColor,
        SetForegroundColor, SetUnderlineColor,
    },
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::{
    backend::{Backend, ClearType},
    buffer::Cell,
    layout::Rect,
    style::{Color, Modifier},
};

/// A [`Backend`] implementation that uses [Crossterm] to render to the terminal.
///
/// The `CrosstermBackend` struct is a wrapper around a type implementing [`std::io::Write`], which
/// is used to send commands to the terminal. It provides methods for drawing content, manipulating
/// the cursor, and clearing the terminal screen.
///
/// # Examples
///
/// Generally, apps should use [`CrosstermBackend::on_stderr()`] or
/// [`CrosstermBackend::on_stdout()`] methods, which both enable raw mode and enter the alternate
/// screen. Choosing `stderr` over `stdout` ensures your app displays on the terminal even if the
/// standard output stream is redirected and makes it easy to write apps that can be piped to other
/// programs.
///
/// ```rust,no_run
/// use ratatui::backend::CrosstermBackend;
///
/// let mut backend = CrosstermBackend::on_stdout()?;
/// // alternatively
/// let mut backend = CrosstermBackend::on_stderr()?;
/// # std::io::Result::Ok(())
/// ```
///
/// For more control over raw mode, the alternate screen, or mouse capture use
/// [`CrosstermBackend::new()`].
///
/// ```rust,no_run
/// use std::io::stdout;
/// use ratatui::backend::CrosstermBackend;
///
/// let mut backend = CrosstermBackend::new(stdout())
///     .with_raw_mode()?
///     .with_alternate_screen()?
///     .with_mouse_capture()?;
/// # std::io::Result::Ok(())
/// ```
///
/// [Crossterm]: https://docs.rs/crossterm
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CrosstermBackend<W: Write> {
    /// The buffer being written to.
    buffer: W,
    /// Whether raw mode is enabled. This is used to determine whether to disable raw mode when
    /// dropping the backend.
    raw_mode_enabled: bool,
    /// Whether the alternate screen is entered. This is used to determine whether to leave the
    /// alternate screen when dropping the backend.
    alternate_screen_entered: bool,
    /// Whether mouse capture is enabled. This is used to determine whether to disable mouse
    /// capture when dropping the backend.
    mouse_capture_enabled: bool,
}

impl<W> CrosstermBackend<W>
where
    W: Write,
{
    /// Creates a new `CrosstermBackend` with the given buffer.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::io::stdout;
    /// # use ratatui::backend::CrosstermBackend;
    /// let mut backend = CrosstermBackend::new(stdout());
    /// # std::io::Result::Ok(())
    /// ```
    pub fn new(buffer: W) -> CrosstermBackend<W> {
        CrosstermBackend {
            buffer,
            raw_mode_enabled: false,
            alternate_screen_entered: false,
            mouse_capture_enabled: false,
        }
    }

    /// Builder method to enable raw mode.
    ///
    /// See [`crate::backend#raw-mode`] for more information.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use std::io::stdout;
    /// # use ratatui::backend::CrosstermBackend;
    /// let mut backend = CrosstermBackend::new(stdout()).with_raw_mode()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn with_raw_mode(mut self) -> io::Result<Self> {
        self.enable_raw_mode()?;
        Ok(self)
    }

    /// Builder method to enter alternate screen.
    ///
    /// See [`crate::backend#alternate-screen`] for more information.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::io::stdout;
    /// # use ratatui::backend::CrosstermBackend;
    /// let mut backend = CrosstermBackend::new(stdout()).with_alternate_screen()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn with_alternate_screen(mut self) -> io::Result<Self> {
        self.enter_alternate_screen()?;
        Ok(self)
    }

    /// Builder method to enable mouse capture.
    ///
    /// See [`crate::backend#mouse-capture`] for more information.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::io::stdout;
    /// # use ratatui::backend::CrosstermBackend;
    /// let mut backend = CrosstermBackend::new(stdout()).with_mouse_capture()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn with_mouse_capture(mut self) -> io::Result<Self> {
        self.enable_mouse_capture()?;
        Ok(self)
    }

    /// Enables raw mode.
    ///
    /// The backend will automatically disable raw mode when it is dropped.
    ///
    /// See [`crate::backend#raw-mode`] for more information.
    pub fn enable_raw_mode(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        self.raw_mode_enabled = true;
        Ok(())
    }

    /// Disables raw mode.
    ///
    /// The backend will automatically disable raw mode when it is dropped.
    ///
    /// See [`crate::backend#raw-mode`] for more information.
    pub fn disable_raw_mode(&mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        self.raw_mode_enabled = false;
        Ok(())
    }

    /// Enters the alternate screen.
    ///
    /// The backend will automatically leave the alternate screen when it is dropped.
    ///
    /// See [`crate::backend#alternate-screen`] for more information.
    pub fn enter_alternate_screen(&mut self) -> io::Result<()> {
        execute!(self.buffer, EnterAlternateScreen)?;
        self.alternate_screen_entered = true;
        Ok(())
    }

    /// Leaves the alternate screen.
    ///
    /// The backend will automatically leave the alternate screen when it is dropped.
    ///
    /// See [`crate::backend#alternate-screen`] for more information.
    pub fn leave_alternate_screen(&mut self) -> io::Result<()> {
        execute!(self.buffer, LeaveAlternateScreen)?;
        self.alternate_screen_entered = false;
        Ok(())
    }

    /// Enables mouse capture.
    ///
    /// The backend will automatically disable mouse capture when it is dropped.
    ///
    /// See [`crate::backend#mouse-capture`] for more information.
    pub fn enable_mouse_capture(&mut self) -> io::Result<()> {
        execute!(self.buffer, EnableMouseCapture)?;
        self.mouse_capture_enabled = true;
        Ok(())
    }

    /// Disables mouse capture.
    ///
    /// The backend will automatically disable mouse capture when it is dropped.
    ///
    /// See [`crate::backend#mouse-capture`] for more information.
    pub fn disable_mouse_capture(&mut self) -> io::Result<()> {
        execute!(self.buffer, DisableMouseCapture)?;
        self.mouse_capture_enabled = false;
        Ok(())
    }
}

impl CrosstermBackend<Stderr> {
    /// Creates a new `CrosstermBackend` using [`std::io::stderr()`] as the buffer.
    ///
    /// This method also enables [raw mode] and enters the [alternate screen].
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ratatui::backend::{Backend, CrosstermBackend};
    /// let mut backend = CrosstermBackend::on_stderr()?;
    /// # std::io::Result::Ok(())
    /// ```
    ///
    /// [raw mode]: crate::backend#raw-mode
    /// [alternate screen]: crate::backend#alternate-screen
    pub fn on_stderr() -> Result<CrosstermBackend<Stderr>, io::Error> {
        CrosstermBackend::new(io::stderr())
            .with_raw_mode()?
            .with_alternate_screen()
    }
}

impl CrosstermBackend<Stdout> {
    /// Creates a new `CrosstermBackend` using [`std::io::stdout()`] as the buffer.
    ///
    /// This method also enables [raw mode] and enters the [alternate screen].
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ratatui::backend::{Backend, CrosstermBackend};
    /// let mut backend = CrosstermBackend::on_stdout()?;
    /// # std::io::Result::Ok(())
    /// ```
    ///
    /// [raw mode]: crate::backend#raw-mode
    /// [alternate screen]: crate::backend#alternate-screen
    pub fn on_stdout() -> Result<CrosstermBackend<Stdout>, io::Error> {
        CrosstermBackend::new(io::stdout())
            .with_raw_mode()?
            .with_alternate_screen()
    }
}

impl Default for CrosstermBackend<Stderr> {
    // impl Default for CrosstermBackend {
    /// Creates a new `CrosstermBackend` using [`std::io::stderr()`] as the buffer.
    ///
    /// It will usually be more convenient to use [`CrosstermBackend::on_stderr()`], which enables
    /// raw mode and enters the alternate screen.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use std::io::Stderr;
    /// # use ratatui::backend::CrosstermBackend;
    /// let mut backend = CrosstermBackend::<Stderr>::default()
    ///     .with_raw_mode()?
    ///     .with_alternate_screen()?;
    /// # std::io::Result::Ok(())
    /// ```
    fn default() -> Self {
        Self::new(io::stderr())
    }
}

impl Default for CrosstermBackend<Stdout> {
    // impl Default for CrosstermBackend {
    /// Creates a new `CrosstermBackend` using [`std::io::stdout()`] as the buffer.
    ///
    /// It will usually be more convenient to use [`CrosstermBackend::on_stdout()`], which enables
    /// raw mode and enters the alternate screen.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use std::io::Stdout;
    /// # use ratatui::backend::{Backend, CrosstermBackend};
    /// let mut backend = CrosstermBackend::<Stdout>::default()
    ///     .with_raw_mode()?
    ///     .with_alternate_screen()?;
    /// # std::io::Result::Ok(())
    /// ```
    fn default() -> Self {
        Self::new(io::stdout())
    }
}

impl<W> Drop for CrosstermBackend<W>
where
    W: Write,
{
    /// Disables raw mode, and mouse capture, leaves the alternate screen and shows the cursor.
    ///
    /// # Panics
    ///
    /// This method panics if it fails to disable raw mode, leave the alternate screen, disable
    /// mouse capture, or show the cursor. Panicking is necessary because there is no way to recover
    /// from these errors. The terminal will be in an inconsistent state, and the only way to fix it
    /// is to reset the terminal manually using `reset` or `stty sane`. A user application can hook
    /// into the panic hook if they want to handle the panic by logging the error or performing
    /// some other action.
    fn drop(&mut self) {
        [
            self.disable_raw_mode()
                .map_err(|_| "failed to disable raw mode"),
            self.disable_mouse_capture()
                .map_err(|_| "failed to disable mouse capture"),
            self.show_cursor().map_err(|_| "failed to show cursor"),
            self.leave_alternate_screen()
                .map_err(|_| "failed to leave alternate screen"),
        ]
        .into_iter()
        .collect::<Result<Vec<()>, &str>>() // only captures the first error
        .expect("dropping CrosstermBackend failed. Reset the terminal manually using `reset` or `stty sane`");
    }
}

impl<W> Write for CrosstermBackend<W>
where
    W: Write,
{
    /// Writes a buffer of bytes to the underlying buffer.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    /// Flushes the underlying buffer.
    fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}

impl<W> Backend for CrosstermBackend<W>
where
    W: Write,
{
    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        let mut fg = Color::Reset;
        let mut bg = Color::Reset;
        let mut underline_color = Color::Reset;
        let mut modifier = Modifier::empty();
        let mut last_pos: Option<(u16, u16)> = None;
        for (x, y, cell) in content {
            // Move the cursor if the previous location was not (x - 1, y)
            if !matches!(last_pos, Some(p) if x == p.0 + 1 && y == p.1) {
                queue!(self.buffer, MoveTo(x, y))?;
            }
            last_pos = Some((x, y));
            if cell.modifier != modifier {
                let diff = ModifierDiff {
                    from: modifier,
                    to: cell.modifier,
                };
                diff.queue(&mut self.buffer)?;
                modifier = cell.modifier;
            }
            if cell.fg != fg {
                let color = CColor::from(cell.fg);
                queue!(self.buffer, SetForegroundColor(color))?;
                fg = cell.fg;
            }
            if cell.bg != bg {
                let color = CColor::from(cell.bg);
                queue!(self.buffer, SetBackgroundColor(color))?;
                bg = cell.bg;
            }
            if cell.underline_color != underline_color {
                let color = CColor::from(cell.underline_color);
                queue!(self.buffer, SetUnderlineColor(color))?;
                underline_color = cell.underline_color;
            }

            queue!(self.buffer, Print(&cell.symbol))?;
        }

        queue!(
            self.buffer,
            SetForegroundColor(CColor::Reset),
            SetBackgroundColor(CColor::Reset),
            SetUnderlineColor(CColor::Reset),
            SetAttribute(CAttribute::Reset)
        )
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        execute!(self.buffer, Hide)
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        execute!(self.buffer, Show)
    }

    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        crossterm::cursor::position()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        execute!(self.buffer, MoveTo(x, y))
    }

    fn clear(&mut self) -> io::Result<()> {
        self.clear_region(ClearType::All)
    }

    fn clear_region(&mut self, clear_type: ClearType) -> io::Result<()> {
        execute!(
            self.buffer,
            Clear(match clear_type {
                ClearType::All => crossterm::terminal::ClearType::All,
                ClearType::AfterCursor => crossterm::terminal::ClearType::FromCursorDown,
                ClearType::BeforeCursor => crossterm::terminal::ClearType::FromCursorUp,
                ClearType::CurrentLine => crossterm::terminal::ClearType::CurrentLine,
                ClearType::UntilNewLine => crossterm::terminal::ClearType::UntilNewLine,
            })
        )
    }

    fn append_lines(&mut self, n: u16) -> io::Result<()> {
        for _ in 0..n {
            queue!(self.buffer, Print("\n"))?;
        }
        self.buffer.flush()
    }

    fn size(&self) -> io::Result<Rect> {
        let (width, height) =
            terminal::size().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        Ok(Rect::new(0, 0, width, height))
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}

impl From<Color> for CColor {
    fn from(color: Color) -> Self {
        match color {
            Color::Reset => CColor::Reset,
            Color::Black => CColor::Black,
            Color::Red => CColor::DarkRed,
            Color::Green => CColor::DarkGreen,
            Color::Yellow => CColor::DarkYellow,
            Color::Blue => CColor::DarkBlue,
            Color::Magenta => CColor::DarkMagenta,
            Color::Cyan => CColor::DarkCyan,
            Color::Gray => CColor::Grey,
            Color::DarkGray => CColor::DarkGrey,
            Color::LightRed => CColor::Red,
            Color::LightGreen => CColor::Green,
            Color::LightBlue => CColor::Blue,
            Color::LightYellow => CColor::Yellow,
            Color::LightMagenta => CColor::Magenta,
            Color::LightCyan => CColor::Cyan,
            Color::White => CColor::White,
            Color::Indexed(i) => CColor::AnsiValue(i),
            Color::Rgb(r, g, b) => CColor::Rgb { r, g, b },
        }
    }
}

/// The `ModifierDiff` struct is used to calculate the difference between two `Modifier`
/// values. This is useful when updating the terminal display, as it allows for more
/// efficient updates by only sending the necessary changes.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
struct ModifierDiff {
    pub from: Modifier,
    pub to: Modifier,
}

impl ModifierDiff {
    fn queue<W>(&self, mut w: W) -> io::Result<()>
    where
        W: io::Write,
    {
        let removed = self.from - self.to;
        if removed.contains(Modifier::REVERSED) {
            queue!(w, SetAttribute(CAttribute::NoReverse))?;
        }
        if removed.contains(Modifier::BOLD) {
            queue!(w, SetAttribute(CAttribute::NormalIntensity))?;
            if self.to.contains(Modifier::DIM) {
                queue!(w, SetAttribute(CAttribute::Dim))?;
            }
        }
        if removed.contains(Modifier::ITALIC) {
            queue!(w, SetAttribute(CAttribute::NoItalic))?;
        }
        if removed.contains(Modifier::UNDERLINED) {
            queue!(w, SetAttribute(CAttribute::NoUnderline))?;
        }
        if removed.contains(Modifier::DIM) {
            queue!(w, SetAttribute(CAttribute::NormalIntensity))?;
        }
        if removed.contains(Modifier::CROSSED_OUT) {
            queue!(w, SetAttribute(CAttribute::NotCrossedOut))?;
        }
        if removed.contains(Modifier::SLOW_BLINK) || removed.contains(Modifier::RAPID_BLINK) {
            queue!(w, SetAttribute(CAttribute::NoBlink))?;
        }

        let added = self.to - self.from;
        if added.contains(Modifier::REVERSED) {
            queue!(w, SetAttribute(CAttribute::Reverse))?;
        }
        if added.contains(Modifier::BOLD) {
            queue!(w, SetAttribute(CAttribute::Bold))?;
        }
        if added.contains(Modifier::ITALIC) {
            queue!(w, SetAttribute(CAttribute::Italic))?;
        }
        if added.contains(Modifier::UNDERLINED) {
            queue!(w, SetAttribute(CAttribute::Underlined))?;
        }
        if added.contains(Modifier::DIM) {
            queue!(w, SetAttribute(CAttribute::Dim))?;
        }
        if added.contains(Modifier::CROSSED_OUT) {
            queue!(w, SetAttribute(CAttribute::CrossedOut))?;
        }
        if added.contains(Modifier::SLOW_BLINK) {
            queue!(w, SetAttribute(CAttribute::SlowBlink))?;
        }
        if added.contains(Modifier::RAPID_BLINK) {
            queue!(w, SetAttribute(CAttribute::RapidBlink))?;
        }

        Ok(())
    }
}
