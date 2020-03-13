use std::fmt;
use std::io;
use std::io::Write;

use super::Backend;
use crate::buffer::Cell;
use crate::layout::Rect;
use crate::style;

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
    pub fn new(stdout: W) -> TermionBackend<W> {
        TermionBackend { stdout }
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
    /// Clears the entire screen and move the cursor to the top left of the screen
    fn clear(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", termion::clear::All)?;
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1))?;
        self.stdout.flush()
    }

    /// Hides cursor
    fn hide_cursor(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", termion::cursor::Hide)?;
        self.stdout.flush()
    }

    /// Shows cursor
    fn show_cursor(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", termion::cursor::Show)?;
        self.stdout.flush()
    }

    /// Gets cursor position (0-based index)
    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        termion::cursor::DetectCursorPos::cursor_pos(&mut self.stdout).map(|(x, y)| (x - 1, y - 1))
    }

    /// Sets cursor position (0-based index)
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
        let mut style = style::Style::default();
        let mut last_y = 0;
        let mut last_x = 0;
        let mut inst = 0;
        for (x, y, cell) in content {
            if y != last_y || x != last_x + 1 || inst == 0 {
                write!(string, "{}", termion::cursor::Goto(x + 1, y + 1)).unwrap();
                inst += 1;
            }
            last_x = x;
            last_y = y;
            if cell.style.modifier != style.modifier {
                write!(
                    string,
                    "{}",
                    ModifierDiff {
                        from: style.modifier,
                        to: cell.style.modifier
                    }
                )
                .unwrap();
                style.modifier = cell.style.modifier;
                inst += 1;
            }
            if cell.style.fg != style.fg {
                write!(string, "{}", Fg(cell.style.fg)).unwrap();
                style.fg = cell.style.fg;
                inst += 1;
            }
            if cell.style.bg != style.bg {
                write!(string, "{}", Bg(cell.style.bg)).unwrap();
                style.bg = cell.style.bg;
                inst += 1;
            }
            string.push_str(&cell.symbol);
            inst += 1;
        }
        write!(
            self.stdout,
            "{}{}{}{}",
            string,
            Fg(style::Color::Reset),
            Bg(style::Color::Reset),
            termion::style::Reset,
        )
    }

    /// Return the size of the terminal
    fn size(&self) -> io::Result<Rect> {
        let terminal = termion::terminal_size()?;
        Ok(Rect::new(0, 0, terminal.0, terminal.1))
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

struct Fg(style::Color);

struct Bg(style::Color);

struct ModifierDiff {
    from: style::Modifier,
    to: style::Modifier,
}

impl fmt::Display for Fg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use termion::color::Color;
        match self.0 {
            style::Color::Reset => termion::color::Reset.write_fg(f),
            style::Color::Black => termion::color::Black.write_fg(f),
            style::Color::Red => termion::color::Red.write_fg(f),
            style::Color::Green => termion::color::Green.write_fg(f),
            style::Color::Yellow => termion::color::Yellow.write_fg(f),
            style::Color::Blue => termion::color::Blue.write_fg(f),
            style::Color::Magenta => termion::color::Magenta.write_fg(f),
            style::Color::Cyan => termion::color::Cyan.write_fg(f),
            style::Color::Gray => termion::color::White.write_fg(f),
            style::Color::DarkGray => termion::color::LightBlack.write_fg(f),
            style::Color::LightRed => termion::color::LightRed.write_fg(f),
            style::Color::LightGreen => termion::color::LightGreen.write_fg(f),
            style::Color::LightBlue => termion::color::LightBlue.write_fg(f),
            style::Color::LightYellow => termion::color::LightYellow.write_fg(f),
            style::Color::LightMagenta => termion::color::LightMagenta.write_fg(f),
            style::Color::LightCyan => termion::color::LightCyan.write_fg(f),
            style::Color::White => termion::color::LightWhite.write_fg(f),
            style::Color::Indexed(i) => termion::color::AnsiValue(i).write_fg(f),
            style::Color::Rgb(r, g, b) => termion::color::Rgb(r, g, b).write_fg(f),
        }
    }
}
impl fmt::Display for Bg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use termion::color::Color;
        match self.0 {
            style::Color::Reset => termion::color::Reset.write_bg(f),
            style::Color::Black => termion::color::Black.write_bg(f),
            style::Color::Red => termion::color::Red.write_bg(f),
            style::Color::Green => termion::color::Green.write_bg(f),
            style::Color::Yellow => termion::color::Yellow.write_bg(f),
            style::Color::Blue => termion::color::Blue.write_bg(f),
            style::Color::Magenta => termion::color::Magenta.write_bg(f),
            style::Color::Cyan => termion::color::Cyan.write_bg(f),
            style::Color::Gray => termion::color::White.write_bg(f),
            style::Color::DarkGray => termion::color::LightBlack.write_bg(f),
            style::Color::LightRed => termion::color::LightRed.write_bg(f),
            style::Color::LightGreen => termion::color::LightGreen.write_bg(f),
            style::Color::LightBlue => termion::color::LightBlue.write_bg(f),
            style::Color::LightYellow => termion::color::LightYellow.write_bg(f),
            style::Color::LightMagenta => termion::color::LightMagenta.write_bg(f),
            style::Color::LightCyan => termion::color::LightCyan.write_bg(f),
            style::Color::White => termion::color::LightWhite.write_bg(f),
            style::Color::Indexed(i) => termion::color::AnsiValue(i).write_bg(f),
            style::Color::Rgb(r, g, b) => termion::color::Rgb(r, g, b).write_bg(f),
        }
    }
}

impl fmt::Display for ModifierDiff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let remove = self.from - self.to;
        if remove.contains(style::Modifier::REVERSED) {
            write!(f, "{}", termion::style::NoInvert)?;
        }
        if remove.contains(style::Modifier::BOLD) {
            // XXX: the termion NoBold flag actually enables double-underline on ECMA-48 compliant
            // terminals, and NoFaint additionally disables bold... so we use this trick to get
            // the right semantics.
            write!(f, "{}", termion::style::NoFaint)?;

            if self.to.contains(style::Modifier::DIM) {
                write!(f, "{}", termion::style::Faint)?;
            }
        }
        if remove.contains(style::Modifier::ITALIC) {
            write!(f, "{}", termion::style::NoItalic)?;
        }
        if remove.contains(style::Modifier::UNDERLINED) {
            write!(f, "{}", termion::style::NoUnderline)?;
        }
        if remove.contains(style::Modifier::DIM) {
            write!(f, "{}", termion::style::NoFaint)?;

            // XXX: the NoFaint flag additionally disables bold as well, so we need to re-enable it
            // here if we want it.
            if self.to.contains(style::Modifier::BOLD) {
                write!(f, "{}", termion::style::Bold)?;
            }
        }
        if remove.contains(style::Modifier::CROSSED_OUT) {
            write!(f, "{}", termion::style::NoCrossedOut)?;
        }
        if remove.contains(style::Modifier::SLOW_BLINK)
            || remove.contains(style::Modifier::RAPID_BLINK)
        {
            write!(f, "{}", termion::style::NoBlink)?;
        }

        let add = self.to - self.from;
        if add.contains(style::Modifier::REVERSED) {
            write!(f, "{}", termion::style::Invert)?;
        }
        if add.contains(style::Modifier::BOLD) {
            write!(f, "{}", termion::style::Bold)?;
        }
        if add.contains(style::Modifier::ITALIC) {
            write!(f, "{}", termion::style::Italic)?;
        }
        if add.contains(style::Modifier::UNDERLINED) {
            write!(f, "{}", termion::style::Underline)?;
        }
        if add.contains(style::Modifier::DIM) {
            write!(f, "{}", termion::style::Faint)?;
        }
        if add.contains(style::Modifier::CROSSED_OUT) {
            write!(f, "{}", termion::style::CrossedOut)?;
        }
        if add.contains(style::Modifier::SLOW_BLINK) || add.contains(style::Modifier::RAPID_BLINK) {
            write!(f, "{}", termion::style::Blink)?;
        }

        Ok(())
    }
}
