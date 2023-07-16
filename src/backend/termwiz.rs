//! This module provides the `TermwizBackend` implementation for the [`Backend`] trait.
//! It uses the `termwiz` crate to interact with the terminal.
//!
//! [`Backend`]: trait.Backend.html
//! [`TermwizBackend`]: crate::backend::TermionBackend

use std::{error::Error, io};

use termwiz::{
    caps::Capabilities,
    cell::{AttributeChange, Blink, Intensity, Underline},
    color::{AnsiColor, ColorAttribute, SrgbaTuple},
    surface::{Change, CursorVisibility, Position},
    terminal::{buffered::BufferedTerminal, SystemTerminal, Terminal},
};

use crate::{
    backend::Backend,
    buffer::Cell,
    layout::Rect,
    style::{Color, Modifier},
};

/// Termwiz backend implementation for the [`Backend`] trait.
/// # Example
///
/// ```rust,no_run
/// use ratatui::backend::{Backend, TermwizBackend};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut backend = TermwizBackend::new()?;
/// backend.clear()?;
/// # Ok(())
/// # }
/// ```
pub struct TermwizBackend {
    buffered_terminal: BufferedTerminal<SystemTerminal>,
}

impl TermwizBackend {
    /// Creates a new Termwiz backend instance.
    pub fn new() -> Result<TermwizBackend, Box<dyn Error>> {
        let mut buffered_terminal =
            BufferedTerminal::new(SystemTerminal::new(Capabilities::new_from_env()?)?)?;
        buffered_terminal.terminal().set_raw_mode()?;
        buffered_terminal.terminal().enter_alternate_screen()?;
        Ok(TermwizBackend { buffered_terminal })
    }

    /// Creates a new Termwiz backend instance with the given buffered terminal.
    pub fn with_buffered_terminal(instance: BufferedTerminal<SystemTerminal>) -> TermwizBackend {
        TermwizBackend {
            buffered_terminal: instance,
        }
    }

    /// Returns a reference to the buffered terminal used by the backend.
    pub fn buffered_terminal(&self) -> &BufferedTerminal<SystemTerminal> {
        &self.buffered_terminal
    }

    /// Returns a mutable reference to the buffered terminal used by the backend.
    pub fn buffered_terminal_mut(&mut self) -> &mut BufferedTerminal<SystemTerminal> {
        &mut self.buffered_terminal
    }
}

impl Backend for TermwizBackend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, cell) in content {
            self.buffered_terminal.add_changes(vec![
                Change::CursorPosition {
                    x: Position::Absolute(x as usize),
                    y: Position::Absolute(y as usize),
                },
                Change::Attribute(AttributeChange::Foreground(cell.fg.into())),
                Change::Attribute(AttributeChange::Background(cell.bg.into())),
            ]);

            self.buffered_terminal
                .add_change(Change::Attribute(AttributeChange::Intensity(
                    if cell.modifier.contains(Modifier::BOLD) {
                        Intensity::Bold
                    } else if cell.modifier.contains(Modifier::DIM) {
                        Intensity::Half
                    } else {
                        Intensity::Normal
                    },
                )));

            self.buffered_terminal
                .add_change(Change::Attribute(AttributeChange::Italic(
                    cell.modifier.contains(Modifier::ITALIC),
                )));

            self.buffered_terminal
                .add_change(Change::Attribute(AttributeChange::Underline(
                    if cell.modifier.contains(Modifier::UNDERLINED) {
                        Underline::Single
                    } else {
                        Underline::None
                    },
                )));

            self.buffered_terminal
                .add_change(Change::Attribute(AttributeChange::Reverse(
                    cell.modifier.contains(Modifier::REVERSED),
                )));

            self.buffered_terminal
                .add_change(Change::Attribute(AttributeChange::Invisible(
                    cell.modifier.contains(Modifier::HIDDEN),
                )));

            self.buffered_terminal
                .add_change(Change::Attribute(AttributeChange::StrikeThrough(
                    cell.modifier.contains(Modifier::CROSSED_OUT),
                )));

            self.buffered_terminal
                .add_change(Change::Attribute(AttributeChange::Blink(
                    if cell.modifier.contains(Modifier::SLOW_BLINK) {
                        Blink::Slow
                    } else if cell.modifier.contains(Modifier::RAPID_BLINK) {
                        Blink::Rapid
                    } else {
                        Blink::None
                    },
                )));

            self.buffered_terminal.add_change(&cell.symbol);
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), io::Error> {
        self.buffered_terminal
            .add_change(Change::CursorVisibility(CursorVisibility::Hidden));
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), io::Error> {
        self.buffered_terminal
            .add_change(Change::CursorVisibility(CursorVisibility::Visible));
        Ok(())
    }

    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        let (x, y) = self.buffered_terminal.cursor_position();
        Ok((x as u16, y as u16))
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.buffered_terminal.add_change(Change::CursorPosition {
            x: Position::Absolute(x as usize),
            y: Position::Absolute(y as usize),
        });

        Ok(())
    }

    fn clear(&mut self) -> Result<(), io::Error> {
        self.buffered_terminal
            .add_change(Change::ClearScreen(termwiz::color::ColorAttribute::Default));
        Ok(())
    }

    fn size(&self) -> Result<Rect, io::Error> {
        let (term_width, term_height) = self.buffered_terminal.dimensions();
        let max = u16::max_value();
        Ok(Rect::new(
            0,
            0,
            if term_width > usize::from(max) {
                max
            } else {
                term_width as u16
            },
            if term_height > usize::from(max) {
                max
            } else {
                term_height as u16
            },
        ))
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.buffered_terminal
            .flush()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(())
    }
}

impl From<Color> for ColorAttribute {
    fn from(color: Color) -> ColorAttribute {
        match color {
            Color::Reset => ColorAttribute::Default,
            Color::Black => AnsiColor::Black.into(),
            Color::Gray | Color::DarkGray => AnsiColor::Grey.into(),
            Color::Red => AnsiColor::Maroon.into(),
            Color::LightRed => AnsiColor::Red.into(),
            Color::Green => AnsiColor::Green.into(),
            Color::LightGreen => AnsiColor::Lime.into(),
            Color::Yellow => AnsiColor::Olive.into(),
            Color::LightYellow => AnsiColor::Yellow.into(),
            Color::Magenta => AnsiColor::Purple.into(),
            Color::LightMagenta => AnsiColor::Fuchsia.into(),
            Color::Cyan => AnsiColor::Teal.into(),
            Color::LightCyan => AnsiColor::Aqua.into(),
            Color::White => AnsiColor::White.into(),
            Color::Blue => AnsiColor::Navy.into(),
            Color::LightBlue => AnsiColor::Blue.into(),
            Color::Indexed(i) => ColorAttribute::PaletteIndex(i),
            Color::Rgb(r, g, b) => {
                ColorAttribute::TrueColorWithDefaultFallback(SrgbaTuple::from((r, g, b)))
            }
        }
    }
}
