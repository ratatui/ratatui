//! This module provides the [`CrosstermBackend`] implementation for the [`Backend`] trait. It uses
//! the [Crossterm] crate to interact with the terminal.
//!
//! [Crossterm]: https://crates.io/crates/crossterm
use std::io::{self, Write};

#[cfg(feature = "underline-color")]
use crossterm::style::SetUnderlineColor;

use crate::{
    backend::{Backend, ClearType, WindowSize},
    buffer::Cell,
    crossterm::{
        cursor::{Hide, MoveTo, Show},
        execute, queue,
        style::{
            Attribute as CAttribute, Attributes as CAttributes, Color as CColor, Colors,
            ContentStyle, Print, SetAttribute, SetBackgroundColor, SetColors, SetForegroundColor,
        },
        terminal::{self, Clear},
    },
    layout::{Position, Size},
    style::{Color, Modifier, Style},
};

/// A [`Backend`] implementation that uses [Crossterm] to render to the terminal.
///
/// The `CrosstermBackend` struct is a wrapper around a writer implementing [`Write`], which is
/// used to send commands to the terminal. It provides methods for drawing content, manipulating
/// the cursor, and clearing the terminal screen.
///
/// Most applications should not call the methods on `CrosstermBackend` directly, but will instead
/// use the [`Terminal`] struct, which provides a more ergonomic interface.
///
/// Usually applications will enable raw mode and switch to alternate screen mode after creating
/// a `CrosstermBackend`. This is done by calling [`crossterm::terminal::enable_raw_mode`] and
/// [`crossterm::terminal::EnterAlternateScreen`] (and the corresponding disable/leave functions
/// when the application exits). This is not done automatically by the backend because it is
/// possible that the application may want to use the terminal for other purposes (like showing
/// help text) before entering alternate screen mode.
///
/// # Example
///
/// ```rust,no_run
/// use std::io::{stderr, stdout};
///
/// use crossterm::{
///     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
///     ExecutableCommand,
/// };
/// use ratatui::{backend::CrosstermBackend, Terminal};
///
/// let mut backend = CrosstermBackend::new(stdout());
/// // or
/// let backend = CrosstermBackend::new(stderr());
/// let mut terminal = Terminal::new(backend)?;
///
/// enable_raw_mode()?;
/// stdout().execute(EnterAlternateScreen)?;
///
/// terminal.clear()?;
/// terminal.draw(|frame| {
///     // -- snip --
/// })?;
///
/// stdout().execute(LeaveAlternateScreen)?;
/// disable_raw_mode()?;
///
/// # std::io::Result::Ok(())
/// ```
///
/// See the the [Examples] directory for more examples. See the [`backend`] module documentation
/// for more details on raw mode and alternate screen.
///
/// [`Write`]: std::io::Write
/// [`Terminal`]: crate::terminal::Terminal
/// [`backend`]: crate::backend
/// [Crossterm]: https://crates.io/crates/crossterm
/// [Examples]: https://github.com/ratatui/ratatui/tree/main/examples/README.md
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct CrosstermBackend<W: Write> {
    /// The writer used to send commands to the terminal.
    writer: W,
}

impl<W> CrosstermBackend<W>
where
    W: Write,
{
    /// Creates a new `CrosstermBackend` with the given writer.
    ///
    /// Most applications will use either [`stdout`](std::io::stdout) or
    /// [`stderr`](std::io::stderr) as writer. See the [FAQ] to determine which one to use.
    ///
    /// [FAQ]: https://ratatui.rs/faq/#should-i-use-stdout-or-stderr
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::io::stdout;
    ///
    /// use ratatui::backend::CrosstermBackend;
    ///
    /// let backend = CrosstermBackend::new(stdout());
    /// ```
    pub const fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Gets the writer.
    #[instability::unstable(
        feature = "backend-writer",
        issue = "https://github.com/ratatui/ratatui/pull/991"
    )]
    pub const fn writer(&self) -> &W {
        &self.writer
    }

    /// Gets the writer as a mutable reference.
    ///
    /// Note: writing to the writer may cause incorrect output after the write. This is due to the
    /// way that the Terminal implements diffing Buffers.
    #[instability::unstable(
        feature = "backend-writer",
        issue = "https://github.com/ratatui/ratatui/pull/991"
    )]
    pub fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
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
        let mut last_pos: Option<Position> = None;
        for (x, y, cell) in content {
            // Move the cursor if the previous location was not (x - 1, y)
            if !matches!(last_pos, Some(p) if x == p.x + 1 && y == p.y) {
                queue!(self.writer, MoveTo(x, y))?;
            }
            last_pos = Some(Position { x, y });
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

    fn get_cursor_position(&mut self) -> io::Result<Position> {
        crossterm::cursor::position()
            .map(|(x, y)| Position { x, y })
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> io::Result<()> {
        let Position { x, y } = position.into();
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

    fn size(&self) -> io::Result<Size> {
        let (width, height) = terminal::size()?;
        Ok(Size { width, height })
    }

    fn window_size(&mut self) -> io::Result<WindowSize> {
        let crossterm::terminal::WindowSize {
            columns,
            rows,
            width,
            height,
        } = terminal::window_size()?;
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

    #[cfg(feature = "scrolling-regions")]
    fn scroll_region_up(&mut self, region: std::ops::Range<u16>, amount: u16) -> io::Result<()> {
        queue!(
            self.writer,
            ScrollUpInRegion {
                first_row: region.start,
                last_row: region.end.saturating_sub(1),
                lines_to_scroll: amount,
            }
        )?;
        self.writer.flush()
    }

    #[cfg(feature = "scrolling-regions")]
    fn scroll_region_down(&mut self, region: std::ops::Range<u16>, amount: u16) -> io::Result<()> {
        queue!(
            self.writer,
            ScrollDownInRegion {
                first_row: region.start,
                last_row: region.end.saturating_sub(1),
                lines_to_scroll: amount,
            }
        )?;
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
        //use crossterm::Attribute;
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

/// A command that scrolls the terminal screen a given number of rows up in a specific scrolling
/// region.
///
/// This will hopefully be replaced by a struct in crossterm proper. There are two outstanding
/// crossterm PRs that will address this:
///   - [918](https://github.com/crossterm-rs/crossterm/pull/918)
///   - [923](https://github.com/crossterm-rs/crossterm/pull/923)
#[cfg(feature = "scrolling-regions")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScrollUpInRegion {
    /// The first row of the scrolling region.
    pub first_row: u16,

    /// The last row of the scrolling region.
    pub last_row: u16,

    /// The number of lines to scroll up by.
    pub lines_to_scroll: u16,
}

#[cfg(feature = "scrolling-regions")]
impl crate::crossterm::Command for ScrollUpInRegion {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        if self.lines_to_scroll != 0 {
            // Set a scrolling region that contains just the desired lines.
            write!(
                f,
                crate::crossterm::csi!("{};{}r"),
                self.first_row.saturating_add(1),
                self.last_row.saturating_add(1)
            )?;
            // Scroll the region by the desired count.
            write!(f, crate::crossterm::csi!("{}S"), self.lines_to_scroll)?;
            // Reset the scrolling region to be the whole screen.
            write!(f, crate::crossterm::csi!("r"))?;
        }
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "ScrollUpInRegion command not supported for winapi",
        ))
    }
}

/// A command that scrolls the terminal screen a given number of rows down in a specific scrolling
/// region.
///
/// This will hopefully be replaced by a struct in crossterm proper. There are two outstanding
/// crossterm PRs that will address this:
///   - [918](https://github.com/crossterm-rs/crossterm/pull/918)
///   - [923](https://github.com/crossterm-rs/crossterm/pull/923)
#[cfg(feature = "scrolling-regions")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScrollDownInRegion {
    /// The first row of the scrolling region.
    pub first_row: u16,

    /// The last row of the scrolling region.
    pub last_row: u16,

    /// The number of lines to scroll down by.
    pub lines_to_scroll: u16,
}

#[cfg(feature = "scrolling-regions")]
impl crate::crossterm::Command for ScrollDownInRegion {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        if self.lines_to_scroll != 0 {
            // Set a scrolling region that contains just the desired lines.
            write!(
                f,
                crate::crossterm::csi!("{};{}r"),
                self.first_row.saturating_add(1),
                self.last_row.saturating_add(1)
            )?;
            // Scroll the region by the desired count.
            write!(f, crate::crossterm::csi!("{}T"), self.lines_to_scroll)?;
            // Reset the scrolling region to be the whole screen.
            write!(f, crate::crossterm::csi!("r"))?;
        }
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "ScrollDownInRegion command not supported for winapi",
        ))
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
