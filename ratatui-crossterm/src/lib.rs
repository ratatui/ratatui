//! This module provides the [`CrosstermBackend`] implementation for the [`Backend`] trait. It uses
//! the [Crossterm] crate to interact with the terminal.
//!
//! [Crossterm]: https://crates.io/crates/crossterm
use std::io::{self, Write};

#[cfg(feature = "underline-color")]
use crossterm::style::SetUnderlineColor;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::{
        Attribute as CrosstermAttribute, Attributes as CrosstermAttributes,
        Color as CrosstermColor, Colors, ContentStyle, Print, SetAttribute, SetBackgroundColor,
        SetColors, SetForegroundColor,
    },
    terminal::{self, Clear},
};
use ratatui_core::{
    backend::{Backend, ClearType, WindowSize},
    buffer::Cell,
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
/// use ratatui::{
///     crossterm::{
///         terminal::{
///             disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
///         },
///         ExecutableCommand,
///     },
///     prelude::*,
/// };
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
    /// # use std::io::stdout;
    /// # use ratatui::prelude::*;
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
                    SetColors(Colors::new(
                        from_ratatui_color(cell.fg),
                        from_ratatui_color(cell.bg)
                    ))
                )?;
                fg = cell.fg;
                bg = cell.bg;
            }
            #[cfg(feature = "underline-color")]
            if cell.underline_color != underline_color {
                let color = from_ratatui_color(cell.underline_color);
                queue!(self.writer, SetUnderlineColor(color))?;
                underline_color = cell.underline_color;
            }

            queue!(self.writer, Print(cell.symbol()))?;
        }

        #[cfg(feature = "underline-color")]
        return queue!(
            self.writer,
            SetForegroundColor(CrosstermColor::Reset),
            SetBackgroundColor(CrosstermColor::Reset),
            SetUnderlineColor(CrosstermColor::Reset),
            SetAttribute(CrosstermAttribute::Reset),
        );
        #[cfg(not(feature = "underline-color"))]
        return queue!(
            self.writer,
            SetForegroundColor(CrosstermColor::Reset),
            SetBackgroundColor(CrosstermColor::Reset),
            SetAttribute(CrosstermAttribute::Reset),
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
}

fn from_ratatui_color(color: Color) -> CrosstermColor {
    match color {
        Color::Reset => CrosstermColor::Reset,
        Color::Black => CrosstermColor::Black,
        Color::Red => CrosstermColor::DarkRed,
        Color::Green => CrosstermColor::DarkGreen,
        Color::Yellow => CrosstermColor::DarkYellow,
        Color::Blue => CrosstermColor::DarkBlue,
        Color::Magenta => CrosstermColor::DarkMagenta,
        Color::Cyan => CrosstermColor::DarkCyan,
        Color::Gray => CrosstermColor::Grey,
        Color::DarkGray => CrosstermColor::DarkGrey,
        Color::LightRed => CrosstermColor::Red,
        Color::LightGreen => CrosstermColor::Green,
        Color::LightBlue => CrosstermColor::Blue,
        Color::LightYellow => CrosstermColor::Yellow,
        Color::LightMagenta => CrosstermColor::Magenta,
        Color::LightCyan => CrosstermColor::Cyan,
        Color::White => CrosstermColor::White,
        Color::Indexed(i) => CrosstermColor::AnsiValue(i),
        Color::Rgb(r, g, b) => CrosstermColor::Rgb { r, g, b },
    }
}

fn from_crossterm_color(value: CrosstermColor) -> Color {
    match value {
        CrosstermColor::Reset => Color::Reset,
        CrosstermColor::Black => Color::Black,
        CrosstermColor::DarkRed => Color::Red,
        CrosstermColor::DarkGreen => Color::Green,
        CrosstermColor::DarkYellow => Color::Yellow,
        CrosstermColor::DarkBlue => Color::Blue,
        CrosstermColor::DarkMagenta => Color::Magenta,
        CrosstermColor::DarkCyan => Color::Cyan,
        CrosstermColor::Grey => Color::Gray,
        CrosstermColor::DarkGrey => Color::DarkGray,
        CrosstermColor::Red => Color::LightRed,
        CrosstermColor::Green => Color::LightGreen,
        CrosstermColor::Blue => Color::LightBlue,
        CrosstermColor::Yellow => Color::LightYellow,
        CrosstermColor::Magenta => Color::LightMagenta,
        CrosstermColor::Cyan => Color::LightCyan,
        CrosstermColor::White => Color::White,
        CrosstermColor::Rgb { r, g, b } => Color::Rgb(r, g, b),
        CrosstermColor::AnsiValue(v) => Color::Indexed(v),
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
            queue!(w, SetAttribute(CrosstermAttribute::NoReverse))?;
        }
        if removed.contains(Modifier::BOLD) {
            queue!(w, SetAttribute(CrosstermAttribute::NormalIntensity))?;
            if self.to.contains(Modifier::DIM) {
                queue!(w, SetAttribute(CrosstermAttribute::Dim))?;
            }
        }
        if removed.contains(Modifier::ITALIC) {
            queue!(w, SetAttribute(CrosstermAttribute::NoItalic))?;
        }
        if removed.contains(Modifier::UNDERLINED) {
            queue!(w, SetAttribute(CrosstermAttribute::NoUnderline))?;
        }
        if removed.contains(Modifier::DIM) {
            queue!(w, SetAttribute(CrosstermAttribute::NormalIntensity))?;
        }
        if removed.contains(Modifier::CROSSED_OUT) {
            queue!(w, SetAttribute(CrosstermAttribute::NotCrossedOut))?;
        }
        if removed.contains(Modifier::SLOW_BLINK) || removed.contains(Modifier::RAPID_BLINK) {
            queue!(w, SetAttribute(CrosstermAttribute::NoBlink))?;
        }

        let added = self.to - self.from;
        if added.contains(Modifier::REVERSED) {
            queue!(w, SetAttribute(CrosstermAttribute::Reverse))?;
        }
        if added.contains(Modifier::BOLD) {
            queue!(w, SetAttribute(CrosstermAttribute::Bold))?;
        }
        if added.contains(Modifier::ITALIC) {
            queue!(w, SetAttribute(CrosstermAttribute::Italic))?;
        }
        if added.contains(Modifier::UNDERLINED) {
            queue!(w, SetAttribute(CrosstermAttribute::Underlined))?;
        }
        if added.contains(Modifier::DIM) {
            queue!(w, SetAttribute(CrosstermAttribute::Dim))?;
        }
        if added.contains(Modifier::CROSSED_OUT) {
            queue!(w, SetAttribute(CrosstermAttribute::CrossedOut))?;
        }
        if added.contains(Modifier::SLOW_BLINK) {
            queue!(w, SetAttribute(CrosstermAttribute::SlowBlink))?;
        }
        if added.contains(Modifier::RAPID_BLINK) {
            queue!(w, SetAttribute(CrosstermAttribute::RapidBlink))?;
        }

        Ok(())
    }
}

fn from_crossterm_attribute(value: CrosstermAttribute) -> Modifier {
    // `Attribute*s*` (note the *s*) contains multiple `Attribute`
    // We convert `Attribute` to `Attribute*s*` (containing only 1 value) to avoid implementing
    // the conversion again
    from_crossterm_attributes(CrosstermAttributes::from(value))
}

fn from_crossterm_attributes(value: CrosstermAttributes) -> Modifier {
    let mut res = Modifier::empty();

    if value.has(CrosstermAttribute::Bold) {
        res |= Modifier::BOLD;
    }
    if value.has(CrosstermAttribute::Dim) {
        res |= Modifier::DIM;
    }
    if value.has(CrosstermAttribute::Italic) {
        res |= Modifier::ITALIC;
    }
    if value.has(CrosstermAttribute::Underlined)
        || value.has(CrosstermAttribute::DoubleUnderlined)
        || value.has(CrosstermAttribute::Undercurled)
        || value.has(CrosstermAttribute::Underdotted)
        || value.has(CrosstermAttribute::Underdashed)
    {
        res |= Modifier::UNDERLINED;
    }
    if value.has(CrosstermAttribute::SlowBlink) {
        res |= Modifier::SLOW_BLINK;
    }
    if value.has(CrosstermAttribute::RapidBlink) {
        res |= Modifier::RAPID_BLINK;
    }
    if value.has(CrosstermAttribute::Reverse) {
        res |= Modifier::REVERSED;
    }
    if value.has(CrosstermAttribute::Hidden) {
        res |= Modifier::HIDDEN;
    }
    if value.has(CrosstermAttribute::CrossedOut) {
        res |= Modifier::CROSSED_OUT;
    }

    res
}

fn from_crossterm_style(value: ContentStyle) -> Style {
    let mut sub_modifier = Modifier::empty();

    if value.attributes.has(CrosstermAttribute::NoBold) {
        sub_modifier |= Modifier::BOLD;
    }
    if value.attributes.has(CrosstermAttribute::NoItalic) {
        sub_modifier |= Modifier::ITALIC;
    }
    if value.attributes.has(CrosstermAttribute::NotCrossedOut) {
        sub_modifier |= Modifier::CROSSED_OUT;
    }
    if value.attributes.has(CrosstermAttribute::NoUnderline) {
        sub_modifier |= Modifier::UNDERLINED;
    }
    if value.attributes.has(CrosstermAttribute::NoHidden) {
        sub_modifier |= Modifier::HIDDEN;
    }
    if value.attributes.has(CrosstermAttribute::NoBlink) {
        sub_modifier |= Modifier::RAPID_BLINK | Modifier::SLOW_BLINK;
    }
    if value.attributes.has(CrosstermAttribute::NoReverse) {
        sub_modifier |= Modifier::REVERSED;
    }

    Style {
        fg: value.foreground_color.map(from_crossterm_color),
        bg: value.background_color.map(from_crossterm_color),
        #[cfg(feature = "underline-color")]
        underline_color: value.underline_color.map(from_crossterm_color),
        add_modifier: from_crossterm_attributes(value.attributes),
        sub_modifier,
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(CrosstermColor::Reset, Color::Reset)]
    #[case(CrosstermColor::Black, Color::Black)]
    #[case(CrosstermColor::DarkGrey, Color::DarkGray)]
    #[case(CrosstermColor::Red, Color::LightRed)]
    #[case(CrosstermColor::DarkRed, Color::Red)]
    #[case(CrosstermColor::Green, Color::LightGreen)]
    #[case(CrosstermColor::DarkGreen, Color::Green)]
    #[case(CrosstermColor::Yellow, Color::LightYellow)]
    #[case(CrosstermColor::DarkYellow, Color::Yellow)]
    #[case(CrosstermColor::Blue, Color::LightBlue)]
    #[case(CrosstermColor::DarkBlue, Color::Blue)]
    #[case(CrosstermColor::Magenta, Color::LightMagenta)]
    #[case(CrosstermColor::DarkMagenta, Color::Magenta)]
    #[case(CrosstermColor::Cyan, Color::LightCyan)]
    #[case(CrosstermColor::DarkCyan, Color::Cyan)]
    #[case(CrosstermColor::White, Color::White)]
    #[case(CrosstermColor::Grey, Color::Gray)]
    #[case(CrosstermColor::Rgb { r: 0, g: 0, b: 0 }, Color::Rgb(0, 0, 0))]
    #[case(CrosstermColor::Rgb { r: 10, g: 20, b: 30 }, Color::Rgb(10, 20, 30))]
    #[case(CrosstermColor::AnsiValue(32), Color::Indexed(32))]
    #[case(CrosstermColor::AnsiValue(37), Color::Indexed(37))]
    fn convert_from_crossterm_color(#[case] value: CrosstermColor, #[case] expected: Color) {
        assert_eq!(from_crossterm_color(value), expected);
    }

    mod modifier {
        use super::*;

        #[rstest]
        #[rstest]
        #[case(CrosstermAttribute::Reset, Modifier::empty())]
        #[case(CrosstermAttribute::Bold, Modifier::BOLD)]
        #[case(CrosstermAttribute::Italic, Modifier::ITALIC)]
        #[case(CrosstermAttribute::Underlined, Modifier::UNDERLINED)]
        #[case(CrosstermAttribute::DoubleUnderlined, Modifier::UNDERLINED)]
        #[case(CrosstermAttribute::Underdotted, Modifier::UNDERLINED)]
        #[case(CrosstermAttribute::Dim, Modifier::DIM)]
        #[case(CrosstermAttribute::NormalIntensity, Modifier::empty())]
        #[case(CrosstermAttribute::CrossedOut, Modifier::CROSSED_OUT)]
        #[case(CrosstermAttribute::NoUnderline, Modifier::empty())]
        #[case(CrosstermAttribute::OverLined, Modifier::empty())]
        #[case(CrosstermAttribute::SlowBlink, Modifier::SLOW_BLINK)]
        #[case(CrosstermAttribute::RapidBlink, Modifier::RAPID_BLINK)]
        #[case(CrosstermAttribute::Hidden, Modifier::HIDDEN)]
        #[case(CrosstermAttribute::NoHidden, Modifier::empty())]
        #[case(CrosstermAttribute::Reverse, Modifier::REVERSED)]
        fn convert_from_crossterm_attribute(
            #[case] value: CrosstermAttribute,
            #[case] expected: Modifier,
        ) {
            assert_eq!(from_crossterm_attribute(value), expected);
        }

        #[rstest]
        #[case(&[CrosstermAttribute::Bold], Modifier::BOLD)]
        #[case(
            &[CrosstermAttribute::Bold, CrosstermAttribute::Italic],
            Modifier::BOLD | Modifier::ITALIC
        )]
        #[case(
            &[CrosstermAttribute::Bold, CrosstermAttribute::NotCrossedOut],
            Modifier::BOLD
        )]
        #[case(
            &[CrosstermAttribute::Dim, CrosstermAttribute::Underdotted],
            Modifier::DIM | Modifier::UNDERLINED
        )]
        #[case(
            &[CrosstermAttribute::Dim, CrosstermAttribute::SlowBlink, CrosstermAttribute::Italic],
            Modifier::DIM | Modifier::SLOW_BLINK | Modifier::ITALIC
        )]
        #[case(
            &[CrosstermAttribute::Hidden, CrosstermAttribute::NoUnderline, CrosstermAttribute::NotCrossedOut],
            Modifier::HIDDEN
        )]
        #[case(
            &[CrosstermAttribute::Reverse],
            Modifier::REVERSED
        )]
        #[case(
            &[CrosstermAttribute::Reset],
            Modifier::empty()
        )]
        #[case(
            &[CrosstermAttribute::RapidBlink, CrosstermAttribute::CrossedOut],
            Modifier::RAPID_BLINK | Modifier::CROSSED_OUT
        )]
        #[case(
            &[CrosstermAttribute::DoubleUnderlined, CrosstermAttribute::OverLined],
            Modifier::UNDERLINED
        )]
        #[case(
            &[CrosstermAttribute::Undercurled, CrosstermAttribute::Underdashed],
            Modifier::UNDERLINED
        )]
        #[case(
            &[CrosstermAttribute::NoBold, CrosstermAttribute::NoItalic],
            Modifier::empty()
        )]
        #[case(
            &[CrosstermAttribute::NoBlink, CrosstermAttribute::NoReverse],
            Modifier::empty()
        )]
        fn convert_from_crossterm_attributes(
            #[case] value: &[CrosstermAttribute],
            #[case] expected: Modifier,
        ) {
            assert_eq!(
                from_crossterm_attributes(CrosstermAttributes::from(value)),
                expected
            );
        }
    }

    #[rstest]
    #[case(ContentStyle::default(), Style::default())]
    #[case(
        ContentStyle {
            foreground_color: Some(CrosstermColor::DarkYellow),
            ..Default::default()
        },
        Style::default().fg(Color::Yellow)
    )]
    #[case(
        ContentStyle {
            background_color: Some(CrosstermColor::DarkYellow),
            ..Default::default()
        },
        Style::default().bg(Color::Yellow)
    )]
    #[case(
        ContentStyle {
            attributes: CrosstermAttributes::from(CrosstermAttribute::Bold),
            ..Default::default()
        },
        Style::default().add_modifier(Modifier::BOLD)
    )]
    #[case(
        ContentStyle {
            attributes: CrosstermAttributes::from(CrosstermAttribute::NoBold),
            ..Default::default()
        },
        Style::default().remove_modifier(Modifier::BOLD)
    )]
    #[case(
        ContentStyle {
            attributes: CrosstermAttributes::from(CrosstermAttribute::Italic),
            ..Default::default()
        },
        Style::default().add_modifier(Modifier::ITALIC)
    )]
    #[case(
        ContentStyle {
            attributes: CrosstermAttributes::from(CrosstermAttribute::NoItalic),
            ..Default::default()
        },
        Style::default().remove_modifier(Modifier::ITALIC)
    )]
    #[case(
        ContentStyle {
            attributes: CrosstermAttributes::from(
                [CrosstermAttribute::Bold, CrosstermAttribute::Italic].as_ref()
            ),
            ..Default::default()
        },
        Style::default()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::ITALIC)
    )]
    #[case(
        ContentStyle {
            attributes: CrosstermAttributes::from(
                [CrosstermAttribute::NoBold, CrosstermAttribute::NoItalic].as_ref()
            ),
            ..Default::default()
        },
        Style::default()
            .remove_modifier(Modifier::BOLD)
            .remove_modifier(Modifier::ITALIC)
    )]
    #[cfg(feature = "underline-color")]
    #[case(
        ContentStyle {
            underline_color: Some(CrosstermColor::DarkRed),
            ..Default::default()
        },
        Style::default().underline_color(Color::Red)
    )]
    fn convert_from_crossterm_content_style(#[case] value: ContentStyle, #[case] expected: Style) {
        assert_eq!(from_crossterm_style(value), expected);
    }
}
