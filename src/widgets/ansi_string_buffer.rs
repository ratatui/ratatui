use std::fmt::{self, Display, Formatter};

use crate::prelude::*;

/// A buffer that allows widgets to easily be rendered to a string with ANSI escape sequences.
pub struct AnsiStringBuffer {
    pub area: Rect,
    buf: Buffer,
}

impl AnsiStringBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            area: Rect::new(0, 0, width, height),
            buf: Buffer::empty(Rect::new(0, 0, width, height)),
        }
    }

    pub fn render_ref(&mut self, widget: &impl WidgetRef) {
        widget.render_ref(self.area, &mut self.buf);
    }
}

impl Display for AnsiStringBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut last_style = None;
        for y in 0..self.area.height {
            if y > 0 {
                f.write_str("\n")?;
            }
            for x in 0..self.area.width {
                let cell = self.buf.get(x, y);
                let style = (cell.fg, cell.bg, cell.modifier);
                if last_style.is_none() || last_style != Some(style) {
                    write_cell_style(cell, f)?;
                    last_style = Some(style);
                }
                f.write_str(cell.symbol())?;
            }
        }
        f.write_str("\u{1b}[0m")
    }
}

fn write_cell_style(cell: &buffer::Cell, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str("\u{1b}[")?;
    write_modifier(cell.modifier, f)?;
    write_fg(cell.fg, f)?;
    write_bg(cell.bg, f)?;
    f.write_str("m")
}

fn write_modifier(modifier: Modifier, f: &mut Formatter<'_>) -> fmt::Result {
    if modifier.contains(Modifier::BOLD) {
        f.write_str("1;")?;
    }
    if modifier.contains(Modifier::DIM) {
        f.write_str("2;")?;
    }
    if modifier.contains(Modifier::ITALIC) {
        f.write_str("3;")?;
    }
    if modifier.contains(Modifier::UNDERLINED) {
        f.write_str("4;")?;
    }
    if modifier.contains(Modifier::SLOW_BLINK) {
        f.write_str("5;")?;
    }
    if modifier.contains(Modifier::RAPID_BLINK) {
        f.write_str("6;")?;
    }
    if modifier.contains(Modifier::REVERSED) {
        f.write_str("7;")?;
    }
    if modifier.contains(Modifier::HIDDEN) {
        f.write_str("8;")?;
    }
    if modifier.contains(Modifier::CROSSED_OUT) {
        f.write_str("9;")?;
    }
    Ok(())
}

fn write_fg(color: Color, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str(match color {
        Color::Reset => "39",
        Color::Black => "30",
        Color::Red => "31",
        Color::Green => "32",
        Color::Yellow => "33",
        Color::Blue => "34",
        Color::Magenta => "35",
        Color::Cyan => "36",
        Color::Gray => "37",
        Color::DarkGray => "90",
        Color::LightRed => "91",
        Color::LightGreen => "92",
        Color::LightYellow => "93",
        Color::LightBlue => "94",
        Color::LightMagenta => "95",
        Color::LightCyan => "96",
        Color::White => "97",
        _ => "",
    })?;
    if let Color::Rgb(red, green, blue) = color {
        f.write_fmt(format_args!("38;2;{red};{green};{blue}"))?;
    }
    if let Color::Indexed(i) = color {
        f.write_fmt(format_args!("38;5;{i}"))?;
    }
    f.write_str(";")
}

fn write_bg(color: Color, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str(match color {
        Color::Reset => "49",
        Color::Black => "40",
        Color::Red => "41",
        Color::Green => "42",
        Color::Yellow => "43",
        Color::Blue => "44",
        Color::Magenta => "45",
        Color::Cyan => "46",
        Color::Gray => "47",
        Color::DarkGray => "100",
        Color::LightRed => "101",
        Color::LightGreen => "102",
        Color::LightYellow => "103",
        Color::LightBlue => "104",
        Color::LightMagenta => "105",
        Color::LightCyan => "106",
        Color::White => "107",
        _ => "",
    })?;
    if let Color::Rgb(red, green, blue) = color {
        f.write_fmt(format_args!("48;2;{red};{green};{blue}"))?;
    }
    if let Color::Indexed(i) = color {
        f.write_fmt(format_args!("48;5;{i}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Greeting;

    impl WidgetRef for Greeting {
        fn render_ref(&self, area: Rect, buf: &mut Buffer) {
            let text = Text::from(vec![
                Line::styled("Hello", Color::Blue),
                Line::styled("World ", Color::Green),
            ]);
            text.render(area, buf);
        }
    }

    #[test]
    fn to_string() {
        let mut buffer = AnsiStringBuffer::new(10, 2);
        buffer.render_ref(&Greeting);
        let ansi_string = buffer.to_string();
        println!("{ansi_string}");
        assert_eq!(
            ansi_string,
            "\u{1b}[34;49mHello     \n\u{1b}[32;49mWorld     \u{1b}[0m"
        );
    }
}
