use crate::buffer::Cell;
use std::io;

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

pub trait Backend {
    fn draw<'a, I>(&self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = &'a (u16, u16, &'a Cell)>;
    fn hide_cursor(&self) -> io::Result<()>;
    fn show_cursor(&self) -> io::Result<()>;
    fn get_cursor(&self) -> io::Result<(u16, u16)>;
    fn set_cursor(&self, x: u16, y: u16) -> io::Result<()>;
    fn clear(&self) -> io::Result<()>;
    fn clear_region(&self) -> io::Result<()>;
    fn dimensions(&self) -> io::Result<(u16, u16)>;
    /// Return the size of the terminal
    fn size(&self) -> io::Result<usize> {
        let (w, h) = self.dimensions()?;
        Ok(w as usize * h as usize)
    }
    fn flush(&self) -> io::Result<()>;
}
