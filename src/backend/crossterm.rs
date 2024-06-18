//! This module provides the [`CrosstermBackend`] implementation for the [`Backend`] trait. It uses
//! the [Crossterm] crate to interact with the terminal.
//!
//! [Crossterm]: https://crates.io/crates/crossterm
use std::io::{self, Write};

#[cfg(feature = "underline-color")]
use crate::crossterm::style::SetUnderlineColor;
use crate::{
    backend::{Backend, ClearType, WindowSize},
    buffer::Cell,
    crossterm::{
        cursor::{Hide, MoveTo, Show},
        event::{
            DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
            EnableFocusChange, EnableMouseCapture, KeyboardEnhancementFlags,
            PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
        },
        execute, queue,
        style::{
            Attribute as CAttribute, Attributes as CAttributes, Color as CColor, Colors,
            ContentStyle, Print, SetAttribute, SetBackgroundColor, SetColors, SetForegroundColor,
        },
        terminal::{
            disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
        },
    },
    layout::{Rect, Size},
    style::{Color, Modifier, Style},
};

/// A [`Backend`] implementation that uses [Crossterm] to render to the terminal.
///
/// The `CrosstermBackend` struct is a wrapper around a writer implementing [`Write`], which is used
/// to send commands to the terminal. It provides methods for drawing content, manipulating the
/// cursor, and clearing the terminal screen.
///
/// Convenience methods ([`CrosstermBackend::stdout`] and [`CrosstermBackend::stderr`] are provided
/// to create a `CrosstermBackend` with [`std::io::stdout`] or [`std::io::stderr`] as the writer.
/// Additionally, these can be created with default settings to enable raw mode and switch to the
/// alternate screen using [`CrosstermBackend::stdout_with_defaults`] or
/// [`CrosstermBackend::stderr_with_defaults`].
///
/// If the default settings are not desired, the `CrosstermBackend` can be configured using the
/// `with_*` methods. These methods return an [`io::Result`] containing self so that they can be
/// chained with other methods. The settings are restored when the `CrosstermBackend` is dropped.
/// - [`CrosstermBackend::with_raw_mode`] enables raw mode for the terminal.
/// - [`CrosstermBackend::with_alternate_screen`] switches to the alternate screen.
/// - [`CrosstermBackend::with_mouse_capture`] enables mouse capture.
/// - [`CrosstermBackend::with_bracketed_paste`] enables bracketed paste.
/// - [`CrosstermBackend::with_focus_change`] enables focus change.
/// - [`CrosstermBackend::with_keyboard_enhancement_flags`] enables keyboard enhancement flags.
///
/// If a backend is configured using the `with_*` methods, the settings are restored when the
/// `CrosstermBackend` is dropped.
///
/// # Example
///
/// ```rust,no_run
/// use ratatui::{
///     backend::{Backend, CrosstermBackend},
///     crossterm::event::KeyboardEnhancementFlags,
/// };
///
/// let mut terminal = CrosstermBackend::stdout_with_defaults()?.to_terminal()?;
/// // or
/// let mut terminal = CrosstermBackend::stderr_with_defaults()?.to_terminal()?;
/// // or with custom settings
/// let mut terminal = CrosstermBackend::stdout()
///     .with_raw_mode()?
///     .with_alternate_screen()?
///     .with_mouse_capture()?
///     .with_bracketed_paste()?
///     .with_focus_change()?
///     .with_keyboard_enhancement_flags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)?
///     .to_terminal()?;
/// # std::io::Result::Ok(())
/// ```
///
/// See the the [Examples] directory for more examples. See the [`backend`] module documentation for
/// more details on raw mode and alternate screen.
///
/// [`Write`]: std::io::Write
/// [`Terminal`]: crate::terminal::Terminal
/// [`backend`]: crate::backend
/// [Crossterm]: https://crates.io/crates/crossterm
/// [Examples]: https://github.com/ratatui-org/ratatui/tree/main/examples/README.md
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
#[allow(clippy::struct_excessive_bools)]
pub struct CrosstermBackend<W: Write> {
    /// The writer used to send commands to the terminal.
    writer: W,
    restore_raw_mode_on_drop: bool,
    restore_alternate_screen_on_drop: bool,
    restore_mouse_capture_on_drop: bool,
    restore_bracketed_paste_on_drop: bool,
    restore_focus_change_on_drop: bool,
    restore_keyboard_enhancement_flags_on_drop: bool,
}

impl<W> CrosstermBackend<W>
where
    W: Write,
{
    /// Creates a new `CrosstermBackend` with the given writer.
    ///
    /// Applications will typically use [`CrosstermBackend::stdout`] or [`CrosstermBackend::stderr`]
    /// to create a `CrosstermBackend` with [`std::io::stdout`] or [`std::io::stderr`] as the
    /// writer.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ratatui::backend::CrosstermBackend;
    /// let backend = CrosstermBackend::new(std::io::stdout());
    /// ```
    pub const fn new(writer: W) -> Self {
        Self {
            writer,
            restore_raw_mode_on_drop: false,
            restore_alternate_screen_on_drop: false,
            restore_mouse_capture_on_drop: false,
            restore_bracketed_paste_on_drop: false,
            restore_focus_change_on_drop: false,
            restore_keyboard_enhancement_flags_on_drop: false,
        }
    }

    /// Gets the writer.
    #[stability::unstable(
        feature = "backend-writer",
        issue = "https://github.com/ratatui-org/ratatui/pull/991"
    )]
    pub const fn writer(&self) -> &W {
        &self.writer
    }

    /// Gets the writer as a mutable reference.
    ///
    /// Note: writing to the writer may cause incorrect output after the write. This is due to the
    /// way that the Terminal implements diffing Buffers.
    #[stability::unstable(
        feature = "backend-writer",
        issue = "https://github.com/ratatui-org/ratatui/pull/991"
    )]
    pub fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }
}

impl CrosstermBackend<io::Stdout> {
    /// Creates a new `CrosstermBackend` with `std::io::stdout`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ratatui::backend::CrosstermBackend;
    /// let backend = CrosstermBackend::stdout();
    /// ```
    pub fn stdout() -> Self {
        Self::new(io::stdout())
    }

    /// Creates a new `CrosstermBackend` with `std::io::stdout` and default settings.
    ///
    /// This enables raw mode and switches to the alternate screen. Mouse support is not enabled.
    ///
    /// Raw mode and alternate screen are restored when the `CrosstermBackend` is dropped.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ratatui::backend::CrosstermBackend;
    /// let backend = CrosstermBackend::stdout_with_defaults()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn stdout_with_defaults() -> io::Result<Self> {
        Self::stdout().with_raw_mode()?.with_alternate_screen()
    }
}

impl CrosstermBackend<io::Stderr> {
    /// Creates a new `CrosstermBackend` with `std::io::stderr`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ratatui::backend::CrosstermBackend;
    /// let backend = CrosstermBackend::stderr();
    /// ```
    pub fn stderr() -> Self {
        Self::new(io::stderr())
    }

    /// Creates a new `CrosstermBackend` with `std::io::stderr` and default settings.
    ///
    /// This enables raw mode and switches to the alternate screen. Mouse support is not enabled.
    ///
    /// Raw mode and alternate screen are restored when the `CrosstermBackend` is dropped.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ratatui::backend::CrosstermBackend;
    /// let backend = CrosstermBackend::stderr_with_defaults()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn stderr_with_defaults() -> io::Result<Self> {
        Self::stderr().with_raw_mode()?.with_alternate_screen()
    }
}

impl<W: Write> CrosstermBackend<W> {
    /// Enables raw mode for the terminal.
    ///
    /// Returns an [`io::Result`] containing self so that it can be chained with other methods.
    ///
    /// Raw mode is restored when the `CrosstermBackend` is dropped.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ratatui::backend::CrosstermBackend;
    /// let backend = CrosstermBackend::stdout().with_raw_mode()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn with_raw_mode(mut self) -> io::Result<Self> {
        enable_raw_mode()?;
        self.restore_raw_mode_on_drop = true;
        Ok(self)
    }

    /// Enables raw mode for the terminal and switches to the alternate screen.
    ///
    /// Returns an [`io::Result`] containing self so that it can be chained with other methods.
    ///
    /// Alternate screen is restored when the `CrosstermBackend` is dropped.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ratatui::backend::CrosstermBackend;
    /// let backend = CrosstermBackend::stdout().with_alternate_screen()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn with_alternate_screen(mut self) -> io::Result<Self> {
        execute!(self.writer, EnterAlternateScreen)?;
        self.restore_alternate_screen_on_drop = true;
        Ok(self)
    }

    /// Enables mouse capture for the terminal.
    ///
    /// Returns an [`io::Result`] containing self so that it can be chained with other methods.
    ///
    /// Mouse capture is disabled when the `CrosstermBackend` is dropped.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ratatui::backend::CrosstermBackend;
    /// let backend = CrosstermBackend::stdout().with_mouse_capture()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn with_mouse_capture(mut self) -> io::Result<Self> {
        execute!(self.writer, EnableMouseCapture)?;
        self.restore_mouse_capture_on_drop = true;
        Ok(self)
    }

    /// Enables bracketed paste for the terminal.
    ///
    /// Returns an [`io::Result`] containing self so that it can be chained with other methods.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ratatui::backend::CrosstermBackend;
    /// let backend = CrosstermBackend::stdout().with_bracketed_paste()?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn with_bracketed_paste(mut self) -> io::Result<Self> {
        execute!(self.writer, EnableBracketedPaste)?;
        self.restore_bracketed_paste_on_drop = true;
        Ok(self)
    }

    /// Enables focus change for the terminal.
    ///
    /// Returns an [`io::Result`] containing self so that it can be chained with other methods.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ratatui::backend::CrosstermBackend;
    /// let backend = CrosstermBackend::stdout().with_focus_change()?;
    /// # std::io::Result::Ok(())
    pub fn with_focus_change(mut self) -> io::Result<Self> {
        execute!(self.writer, EnableFocusChange)?;
        self.restore_focus_change_on_drop = true;
        Ok(self)
    }

    /// Enables keyboard enhancement flags for the terminal.
    ///
    /// Returns an [`io::Result`] containing self so that it can be chained with other methods.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ratatui::{backend::CrosstermBackend, crossterm::event::KeyboardEnhancementFlags};
    ///
    /// let backend = CrosstermBackend::stdout()
    ///     .with_keyboard_enhancement_flags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn with_keyboard_enhancement_flags(
        mut self,
        flags: KeyboardEnhancementFlags,
    ) -> io::Result<Self> {
        execute!(self.writer, PushKeyboardEnhancementFlags(flags))?;
        self.restore_keyboard_enhancement_flags_on_drop = true;
        Ok(self)
    }

    /// Resets the terminal to its default state.
    ///
    /// - Disables raw mode
    /// - Disables mouse capture
    /// - Leaves the alternate screen
    /// - Disables bracketed paste
    /// - Disables focus change
    /// - Pops keyboard enhancement flags
    ///
    /// This method is an associated method rather than an instance method to make it possible to
    /// call without having a `CrosstermBackend` instance. This is often useful in the context of
    /// error / panic handling.
    ///
    /// If you have created a `CrosstermBackend` using the `with_*` methods, the settings are
    /// restored when the `CrosstermBackend` is dropped, so you do not need to call this method
    /// manually.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ratatui::backend::CrosstermBackend;
    /// CrosstermBackend::reset(std::io::stderr())?;
    /// # std::io::Result::Ok(())
    /// ```
    pub fn reset(mut writer: W) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(
            writer,
            LeaveAlternateScreen,
            DisableMouseCapture,
            DisableBracketedPaste,
            DisableFocusChange,
            PopKeyboardEnhancementFlags
        )?;
        writer.flush()
    }
}

impl<W: Write> Drop for CrosstermBackend<W> {
    fn drop(&mut self) {
        // note that these are not checked for errors because there is nothing that can be done if
        // they fail. The terminal is likely in a bad state, and the application is exiting anyway.
        if self.restore_raw_mode_on_drop {
            let _ = disable_raw_mode();
        }
        if self.restore_mouse_capture_on_drop {
            let _ = execute!(self.writer, DisableMouseCapture);
        }
        if self.restore_alternate_screen_on_drop {
            let _ = execute!(self.writer, LeaveAlternateScreen);
        }
        if self.restore_bracketed_paste_on_drop {
            let _ = execute!(self.writer, DisableBracketedPaste);
        }
        if self.restore_focus_change_on_drop {
            let _ = execute!(self.writer, DisableFocusChange);
        }
        if self.restore_keyboard_enhancement_flags_on_drop {
            let _ = execute!(self.writer, PopKeyboardEnhancementFlags);
        }
        let _ = self.writer.flush();
    }
}

impl<W> Write for CrosstermBackend<W>
where
    W: Write,
{
    /// Writes a buffer of bytes to the underlying buffer.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    /// Flushes the underlying buffer.
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
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
        #[cfg(feature = "underline-color")]
        let mut underline_color = Color::Reset;
        let mut modifier = Modifier::empty();
        let mut last_pos: Option<(u16, u16)> = None;
        for (x, y, cell) in content {
            // Move the cursor if the previous location was not (x - 1, y)
            if !matches!(last_pos, Some(p) if x == p.0 + 1 && y == p.1) {
                queue!(self.writer, MoveTo(x, y))?;
            }
            last_pos = Some((x, y));
            if cell.modifier != modifier {
                let diff = ModifierDiff {
                    from: modifier,
                    to: cell.modifier,
                };
                diff.queue(&mut self.writer)?;
                modifier = cell.modifier;
            }
            if cell.fg != fg || cell.bg != bg {
                queue!(
                    self.writer,
                    SetColors(Colors::new(cell.fg.into(), cell.bg.into()))
                )?;
                fg = cell.fg;
                bg = cell.bg;
            }
            #[cfg(feature = "underline-color")]
            if cell.underline_color != underline_color {
                let color = CColor::from(cell.underline_color);
                queue!(self.writer, SetUnderlineColor(color))?;
                underline_color = cell.underline_color;
            }

            queue!(self.writer, Print(cell.symbol()))?;
        }

        #[cfg(feature = "underline-color")]
        return queue!(
            self.writer,
            SetForegroundColor(CColor::Reset),
            SetBackgroundColor(CColor::Reset),
            SetUnderlineColor(CColor::Reset),
            SetAttribute(CAttribute::Reset),
        );
        #[cfg(not(feature = "underline-color"))]
        return queue!(
            self.writer,
            SetForegroundColor(CColor::Reset),
            SetBackgroundColor(CColor::Reset),
            SetAttribute(CAttribute::Reset),
        );
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        execute!(self.writer, Hide)
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        execute!(self.writer, Show)
    }

    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        crossterm::cursor::position()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        execute!(self.writer, MoveTo(x, y))
    }

    fn clear(&mut self) -> io::Result<()> {
        self.clear_region(ClearType::All)
    }

    fn clear_region(&mut self, clear_type: ClearType) -> io::Result<()> {
        execute!(
            self.writer,
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
            queue!(self.writer, Print("\n"))?;
        }
        self.writer.flush()
    }

    fn size(&self) -> io::Result<Rect> {
        let (width, height) = crossterm::terminal::size()?;
        Ok(Rect::new(0, 0, width, height))
    }

    fn window_size(&mut self) -> io::Result<WindowSize> {
        let crossterm::terminal::WindowSize {
            columns,
            rows,
            width,
            height,
        } = crossterm::terminal::window_size()?;
        Ok(WindowSize {
            columns_rows: Size {
                width: columns,
                height: rows,
            },
            pixels: Size { width, height },
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl From<Color> for CColor {
    fn from(color: Color) -> Self {
        match color {
            Color::Reset => Self::Reset,
            Color::Black => Self::Black,
            Color::Red => Self::DarkRed,
            Color::Green => Self::DarkGreen,
            Color::Yellow => Self::DarkYellow,
            Color::Blue => Self::DarkBlue,
            Color::Magenta => Self::DarkMagenta,
            Color::Cyan => Self::DarkCyan,
            Color::Gray => Self::Grey,
            Color::DarkGray => Self::DarkGrey,
            Color::LightRed => Self::Red,
            Color::LightGreen => Self::Green,
            Color::LightBlue => Self::Blue,
            Color::LightYellow => Self::Yellow,
            Color::LightMagenta => Self::Magenta,
            Color::LightCyan => Self::Cyan,
            Color::White => Self::White,
            Color::Indexed(i) => Self::AnsiValue(i),
            Color::Rgb(r, g, b) => Self::Rgb { r, g, b },
        }
    }
}

impl From<CColor> for Color {
    fn from(value: CColor) -> Self {
        match value {
            CColor::Reset => Self::Reset,
            CColor::Black => Self::Black,
            CColor::DarkRed => Self::Red,
            CColor::DarkGreen => Self::Green,
            CColor::DarkYellow => Self::Yellow,
            CColor::DarkBlue => Self::Blue,
            CColor::DarkMagenta => Self::Magenta,
            CColor::DarkCyan => Self::Cyan,
            CColor::Grey => Self::Gray,
            CColor::DarkGrey => Self::DarkGray,
            CColor::Red => Self::LightRed,
            CColor::Green => Self::LightGreen,
            CColor::Blue => Self::LightBlue,
            CColor::Yellow => Self::LightYellow,
            CColor::Magenta => Self::LightMagenta,
            CColor::Cyan => Self::LightCyan,
            CColor::White => Self::White,
            CColor::Rgb { r, g, b } => Self::Rgb(r, g, b),
            CColor::AnsiValue(v) => Self::Indexed(v),
        }
    }
}

/// The `ModifierDiff` struct is used to calculate the difference between two `Modifier`
/// values. This is useful when updating the terminal display, as it allows for more
/// efficient updates by only sending the necessary changes.
struct ModifierDiff {
    pub from: Modifier,
    pub to: Modifier,
}

impl ModifierDiff {
    fn queue<W>(self, mut w: W) -> io::Result<()>
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

impl From<CAttribute> for Modifier {
    fn from(value: CAttribute) -> Self {
        // `Attribute*s*` (note the *s*) contains multiple `Attribute`
        // We convert `Attribute` to `Attribute*s*` (containing only 1 value) to avoid implementing
        // the conversion again
        Self::from(CAttributes::from(value))
    }
}

impl From<CAttributes> for Modifier {
    fn from(value: CAttributes) -> Self {
        let mut res = Self::empty();

        if value.has(CAttribute::Bold) {
            res |= Self::BOLD;
        }
        if value.has(CAttribute::Dim) {
            res |= Self::DIM;
        }
        if value.has(CAttribute::Italic) {
            res |= Self::ITALIC;
        }
        if value.has(CAttribute::Underlined)
            || value.has(CAttribute::DoubleUnderlined)
            || value.has(CAttribute::Undercurled)
            || value.has(CAttribute::Underdotted)
            || value.has(CAttribute::Underdashed)
        {
            res |= Self::UNDERLINED;
        }
        if value.has(CAttribute::SlowBlink) {
            res |= Self::SLOW_BLINK;
        }
        if value.has(CAttribute::RapidBlink) {
            res |= Self::RAPID_BLINK;
        }
        if value.has(CAttribute::Reverse) {
            res |= Self::REVERSED;
        }
        if value.has(CAttribute::Hidden) {
            res |= Self::HIDDEN;
        }
        if value.has(CAttribute::CrossedOut) {
            res |= Self::CROSSED_OUT;
        }

        res
    }
}

impl From<ContentStyle> for Style {
    fn from(value: ContentStyle) -> Self {
        let mut sub_modifier = Modifier::empty();

        if value.attributes.has(CAttribute::NoBold) {
            sub_modifier |= Modifier::BOLD;
        }
        if value.attributes.has(CAttribute::NoItalic) {
            sub_modifier |= Modifier::ITALIC;
        }
        if value.attributes.has(CAttribute::NotCrossedOut) {
            sub_modifier |= Modifier::CROSSED_OUT;
        }
        if value.attributes.has(CAttribute::NoUnderline) {
            sub_modifier |= Modifier::UNDERLINED;
        }
        if value.attributes.has(CAttribute::NoHidden) {
            sub_modifier |= Modifier::HIDDEN;
        }
        if value.attributes.has(CAttribute::NoBlink) {
            sub_modifier |= Modifier::RAPID_BLINK | Modifier::SLOW_BLINK;
        }
        if value.attributes.has(CAttribute::NoReverse) {
            sub_modifier |= Modifier::REVERSED;
        }

        Self {
            fg: value.foreground_color.map(Into::into),
            bg: value.background_color.map(Into::into),
            #[cfg(feature = "underline-color")]
            underline_color: value.underline_color.map(Into::into),
            add_modifier: value.attributes.into(),
            sub_modifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_crossterm_color() {
        assert_eq!(Color::from(CColor::Reset), Color::Reset);
        assert_eq!(Color::from(CColor::Black), Color::Black);
        assert_eq!(Color::from(CColor::DarkGrey), Color::DarkGray);
        assert_eq!(Color::from(CColor::Red), Color::LightRed);
        assert_eq!(Color::from(CColor::DarkRed), Color::Red);
        assert_eq!(Color::from(CColor::Green), Color::LightGreen);
        assert_eq!(Color::from(CColor::DarkGreen), Color::Green);
        assert_eq!(Color::from(CColor::Yellow), Color::LightYellow);
        assert_eq!(Color::from(CColor::DarkYellow), Color::Yellow);
        assert_eq!(Color::from(CColor::Blue), Color::LightBlue);
        assert_eq!(Color::from(CColor::DarkBlue), Color::Blue);
        assert_eq!(Color::from(CColor::Magenta), Color::LightMagenta);
        assert_eq!(Color::from(CColor::DarkMagenta), Color::Magenta);
        assert_eq!(Color::from(CColor::Cyan), Color::LightCyan);
        assert_eq!(Color::from(CColor::DarkCyan), Color::Cyan);
        assert_eq!(Color::from(CColor::White), Color::White);
        assert_eq!(Color::from(CColor::Grey), Color::Gray);
        assert_eq!(
            Color::from(CColor::Rgb { r: 0, g: 0, b: 0 }),
            Color::Rgb(0, 0, 0)
        );
        assert_eq!(
            Color::from(CColor::Rgb {
                r: 10,
                g: 20,
                b: 30
            }),
            Color::Rgb(10, 20, 30)
        );
        assert_eq!(Color::from(CColor::AnsiValue(32)), Color::Indexed(32));
        assert_eq!(Color::from(CColor::AnsiValue(37)), Color::Indexed(37));
    }

    mod modifier {
        use super::*;

        #[test]
        fn from_crossterm_attribute() {
            assert_eq!(Modifier::from(CAttribute::Reset), Modifier::empty());
            assert_eq!(Modifier::from(CAttribute::Bold), Modifier::BOLD);
            assert_eq!(Modifier::from(CAttribute::Italic), Modifier::ITALIC);
            assert_eq!(Modifier::from(CAttribute::Underlined), Modifier::UNDERLINED);
            assert_eq!(
                Modifier::from(CAttribute::DoubleUnderlined),
                Modifier::UNDERLINED
            );
            assert_eq!(
                Modifier::from(CAttribute::Underdotted),
                Modifier::UNDERLINED
            );
            assert_eq!(Modifier::from(CAttribute::Dim), Modifier::DIM);
            assert_eq!(
                Modifier::from(CAttribute::NormalIntensity),
                Modifier::empty()
            );
            assert_eq!(
                Modifier::from(CAttribute::CrossedOut),
                Modifier::CROSSED_OUT
            );
            assert_eq!(Modifier::from(CAttribute::NoUnderline), Modifier::empty());
            assert_eq!(Modifier::from(CAttribute::OverLined), Modifier::empty());
            assert_eq!(Modifier::from(CAttribute::SlowBlink), Modifier::SLOW_BLINK);
            assert_eq!(
                Modifier::from(CAttribute::RapidBlink),
                Modifier::RAPID_BLINK
            );
            assert_eq!(Modifier::from(CAttribute::Hidden), Modifier::HIDDEN);
            assert_eq!(Modifier::from(CAttribute::NoHidden), Modifier::empty());
            assert_eq!(Modifier::from(CAttribute::Reverse), Modifier::REVERSED);
        }

        #[test]
        fn from_crossterm_attributes() {
            assert_eq!(
                Modifier::from(CAttributes::from(CAttribute::Bold)),
                Modifier::BOLD
            );
            assert_eq!(
                Modifier::from(CAttributes::from(
                    [CAttribute::Bold, CAttribute::Italic].as_ref()
                )),
                Modifier::BOLD | Modifier::ITALIC
            );
            assert_eq!(
                Modifier::from(CAttributes::from(
                    [CAttribute::Bold, CAttribute::NotCrossedOut].as_ref()
                )),
                Modifier::BOLD
            );
            assert_eq!(
                Modifier::from(CAttributes::from(
                    [CAttribute::Dim, CAttribute::Underdotted].as_ref()
                )),
                Modifier::DIM | Modifier::UNDERLINED
            );
            assert_eq!(
                Modifier::from(CAttributes::from(
                    [CAttribute::Dim, CAttribute::SlowBlink, CAttribute::Italic].as_ref()
                )),
                Modifier::DIM | Modifier::SLOW_BLINK | Modifier::ITALIC
            );
            assert_eq!(
                Modifier::from(CAttributes::from(
                    [
                        CAttribute::Hidden,
                        CAttribute::NoUnderline,
                        CAttribute::NotCrossedOut
                    ]
                    .as_ref()
                )),
                Modifier::HIDDEN
            );
            assert_eq!(
                Modifier::from(CAttributes::from(CAttribute::Reverse)),
                Modifier::REVERSED
            );
            assert_eq!(
                Modifier::from(CAttributes::from(CAttribute::Reset)),
                Modifier::empty()
            );
            assert_eq!(
                Modifier::from(CAttributes::from(
                    [CAttribute::RapidBlink, CAttribute::CrossedOut].as_ref()
                )),
                Modifier::RAPID_BLINK | Modifier::CROSSED_OUT
            );
        }
    }

    #[test]
    fn from_crossterm_content_style() {
        assert_eq!(Style::from(ContentStyle::default()), Style::default());
        assert_eq!(
            Style::from(ContentStyle {
                foreground_color: Some(CColor::DarkYellow),
                ..Default::default()
            }),
            Style::default().fg(Color::Yellow)
        );
        assert_eq!(
            Style::from(ContentStyle {
                background_color: Some(CColor::DarkYellow),
                ..Default::default()
            }),
            Style::default().bg(Color::Yellow)
        );
        assert_eq!(
            Style::from(ContentStyle {
                attributes: CAttributes::from(CAttribute::Bold),
                ..Default::default()
            }),
            Style::default().add_modifier(Modifier::BOLD)
        );
        assert_eq!(
            Style::from(ContentStyle {
                attributes: CAttributes::from(CAttribute::NoBold),
                ..Default::default()
            }),
            Style::default().remove_modifier(Modifier::BOLD)
        );
        assert_eq!(
            Style::from(ContentStyle {
                attributes: CAttributes::from(CAttribute::Italic),
                ..Default::default()
            }),
            Style::default().add_modifier(Modifier::ITALIC)
        );
        assert_eq!(
            Style::from(ContentStyle {
                attributes: CAttributes::from(CAttribute::NoItalic),
                ..Default::default()
            }),
            Style::default().remove_modifier(Modifier::ITALIC)
        );
        assert_eq!(
            Style::from(ContentStyle {
                attributes: CAttributes::from([CAttribute::Bold, CAttribute::Italic].as_ref()),
                ..Default::default()
            }),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC)
        );
        assert_eq!(
            Style::from(ContentStyle {
                attributes: CAttributes::from([CAttribute::NoBold, CAttribute::NoItalic].as_ref()),
                ..Default::default()
            }),
            Style::default()
                .remove_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::ITALIC)
        );
    }

    #[test]
    #[cfg(feature = "underline-color")]
    fn from_crossterm_content_style_underline() {
        assert_eq!(
            Style::from(ContentStyle {
                underline_color: Some(CColor::DarkRed),
                ..Default::default()
            }),
            Style::default().underline_color(Color::Red)
        );
    }
}
