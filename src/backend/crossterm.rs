use crate::{
    backend::Backend,
    buffer::Cell,
    style::{Color, Modifier},
};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::{
        Attribute as CAttribute, Color as CColor, Print, SetAttribute, SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use std::{
    cell::RefCell,
    io::{self, Write},
};

pub struct CrosstermBackend<W: Write> {
    stdout: RefCell<W>,
}

impl<W: Write> CrosstermBackend<W> {
    pub fn new(buffer: W) -> CrosstermBackend<W> {
        CrosstermBackend {
            stdout: RefCell::new(buffer),
        }
    }
}

impl<W: Write> Write for CrosstermBackend<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.borrow_mut().flush()
    }
}

impl<W: Write> Backend for CrosstermBackend<W> {
    fn draw<'a, I>(&self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = &'a (u16, u16, &'a Cell)>,
    {
        let mut fg = Color::Reset;
        let mut bg = Color::Reset;
        let mut modifier = Modifier::empty();
        let mut last_pos: Option<(u16, u16)> = None;
        let mut stdout = self.stdout.borrow_mut();
        for (x, y, cell) in content {
            // Move the cursor if the previous location was not (x - 1, y)
            if !matches!(last_pos, Some(p) if *x == p.0 + 1 && *y == p.1) {
                map_result(queue!(stdout, MoveTo(*x, *y)))?;
            }
            last_pos = Some((*x, *y));
            if cell.modifier != modifier {
                let diff = ModifierDiff {
                    from: modifier,
                    to: cell.modifier,
                };
                diff.queue(&mut *stdout)?;
                modifier = cell.modifier;
            }
            if cell.fg != fg {
                let color = CColor::from(cell.fg);
                map_result(queue!(stdout, SetForegroundColor(color)))?;
                fg = cell.fg;
            }
            if cell.bg != bg {
                let color = CColor::from(cell.bg);
                map_result(queue!(stdout, SetBackgroundColor(color)))?;
                bg = cell.bg;
            }

            map_result(queue!(stdout, Print(&cell.symbol)))?;
        }

        map_result(queue!(
            stdout,
            SetForegroundColor(CColor::Reset),
            SetBackgroundColor(CColor::Reset),
            SetAttribute(CAttribute::Reset)
        ))
    }

    fn hide_cursor(&self) -> io::Result<()> {
        map_result(execute!(self.stdout.borrow_mut(), Hide))
    }

    fn show_cursor(&self) -> io::Result<()> {
        map_result(execute!(self.stdout.borrow_mut(), Show))
    }

    fn get_cursor(&self) -> io::Result<(u16, u16)> {
        crossterm::cursor::position()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn set_cursor(&self, x: u16, y: u16) -> io::Result<()> {
        map_result(execute!(self.stdout.borrow_mut(), MoveTo(x, y)))
    }

    fn clear(&self) -> io::Result<()> {
        map_result(execute!(self.stdout.borrow_mut(), Clear(ClearType::All)))
    }

    fn dimensions(&self) -> io::Result<(u16, u16)> {
        terminal::size().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn flush(&self) -> io::Result<()> {
        self.stdout.borrow_mut().flush()
    }
}

fn map_result(result: crossterm::Result<()>) -> io::Result<()> {
    result.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
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

#[derive(Debug)]
struct ModifierDiff {
    pub from: Modifier,
    pub to: Modifier,
}

impl ModifierDiff {
    fn queue<W: Write>(&self, w: &mut W) -> io::Result<()> {
        //use crossterm::Attribute;
        let removed = self.from - self.to;
        if removed.contains(Modifier::REVERSED) {
            map_result(queue!(w, SetAttribute(CAttribute::NoReverse)))?;
        }
        if removed.contains(Modifier::BOLD) {
            map_result(queue!(w, SetAttribute(CAttribute::NormalIntensity)))?;
            if self.to.contains(Modifier::DIM) {
                map_result(queue!(w, SetAttribute(CAttribute::Dim)))?;
            }
        }
        if removed.contains(Modifier::ITALIC) {
            map_result(queue!(w, SetAttribute(CAttribute::NoItalic)))?;
        }
        if removed.contains(Modifier::UNDERLINED) {
            map_result(queue!(w, SetAttribute(CAttribute::NoUnderline)))?;
        }
        if removed.contains(Modifier::DIM) {
            map_result(queue!(w, SetAttribute(CAttribute::NormalIntensity)))?;
        }
        if removed.contains(Modifier::CROSSED_OUT) {
            map_result(queue!(w, SetAttribute(CAttribute::NotCrossedOut)))?;
        }
        if removed.contains(Modifier::SLOW_BLINK) || removed.contains(Modifier::RAPID_BLINK) {
            map_result(queue!(w, SetAttribute(CAttribute::NoBlink)))?;
        }

        let added = self.to - self.from;
        if added.contains(Modifier::REVERSED) {
            map_result(queue!(w, SetAttribute(CAttribute::Reverse)))?;
        }
        if added.contains(Modifier::BOLD) {
            map_result(queue!(w, SetAttribute(CAttribute::Bold)))?;
        }
        if added.contains(Modifier::ITALIC) {
            map_result(queue!(w, SetAttribute(CAttribute::Italic)))?;
        }
        if added.contains(Modifier::UNDERLINED) {
            map_result(queue!(w, SetAttribute(CAttribute::Underlined)))?;
        }
        if added.contains(Modifier::DIM) {
            map_result(queue!(w, SetAttribute(CAttribute::Dim)))?;
        }
        if added.contains(Modifier::CROSSED_OUT) {
            map_result(queue!(w, SetAttribute(CAttribute::CrossedOut)))?;
        }
        if added.contains(Modifier::SLOW_BLINK) {
            map_result(queue!(w, SetAttribute(CAttribute::SlowBlink)))?;
        }
        if added.contains(Modifier::RAPID_BLINK) {
            map_result(queue!(w, SetAttribute(CAttribute::RapidBlink)))?;
        }

        Ok(())
    }
}
