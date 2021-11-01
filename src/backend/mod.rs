use std::io;

use crate::buffer::Cell;
use crate::layout::Rect;

#[cfg(feature = "termion")]
mod termion;
#[cfg(feature = "termion")]
pub use self::termion::TermionBackend;

#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "crossterm")]
pub use self::crossterm::CrosstermBackend;

mod test;
pub use self::test::TestBackend;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClearType {
    All,
    AfterCursor,
    BeforeCursor,
    CurrentLine,
    UntilNewLine,
}

pub trait Backend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>;
    fn append_lines(&mut self, n: u16) -> io::Result<()>;
    fn hide_cursor(&mut self) -> Result<(), io::Error>;
    fn show_cursor(&mut self) -> Result<(), io::Error>;
    fn get_cursor(&mut self) -> Result<(u16, u16), io::Error>;
    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), io::Error>;
    fn clear(&mut self, clear_type: ClearType) -> Result<(), io::Error>;
    fn size(&self) -> Result<Rect, io::Error>;
    fn flush(&mut self) -> Result<(), io::Error>;
}
