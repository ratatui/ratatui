use std::fmt::Debug;

use crate::prelude::*;

/// A buffer cell
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cell {
    #[deprecated(
        since = "0.24.1",
        note = "This field will be hidden at next major version. Use `Cell::symbol` method to get \
                the value. Use `Cell::set_symbol` to update the field. Use `Cell::default` to \
                create `Cell` instance"
    )]
    pub symbol: String,
    pub fg: Color,
    pub bg: Color,
    #[cfg(feature = "underline-color")]
    pub underline_color: Color,
    pub modifier: Modifier,
    pub skip: bool,
}

#[allow(deprecated)] // For Cell::symbol
impl Cell {
    pub fn symbol(&self) -> &str {
        self.symbol.as_str()
    }

    pub fn set_symbol(&mut self, symbol: &str) -> &mut Cell {
        self.symbol.clear();
        self.symbol.push_str(symbol);
        self
    }

    pub fn set_char(&mut self, ch: char) -> &mut Cell {
        self.symbol.clear();
        self.symbol.push(ch);
        self
    }

    pub fn set_fg(&mut self, color: Color) -> &mut Cell {
        self.fg = color;
        self
    }

    pub fn set_bg(&mut self, color: Color) -> &mut Cell {
        self.bg = color;
        self
    }

    pub fn set_style(&mut self, style: Style) -> &mut Cell {
        if let Some(c) = style.fg {
            self.fg = c;
        }
        if let Some(c) = style.bg {
            self.bg = c;
        }
        #[cfg(feature = "underline-color")]
        if let Some(c) = style.underline_color {
            self.underline_color = c;
        }
        self.modifier.insert(style.add_modifier);
        self.modifier.remove(style.sub_modifier);
        self
    }

    #[cfg(feature = "underline-color")]
    pub fn style(&self) -> Style {
        Style::default()
            .fg(self.fg)
            .bg(self.bg)
            .underline_color(self.underline_color)
            .add_modifier(self.modifier)
    }

    #[cfg(not(feature = "underline-color"))]
    pub fn style(&self) -> Style {
        Style::default()
            .fg(self.fg)
            .bg(self.bg)
            .add_modifier(self.modifier)
    }

    /// Sets the cell to be skipped when copying (diffing) the buffer to the screen.
    ///
    /// This is helpful when it is necessary to prevent the buffer from overwriting a cell that is
    /// covered by an image from some terminal graphics protocol (Sixel / iTerm / Kitty ...).
    pub fn set_skip(&mut self, skip: bool) -> &mut Cell {
        self.skip = skip;
        self
    }

    pub fn reset(&mut self) {
        self.symbol.clear();
        self.symbol.push(' ');
        self.fg = Color::Reset;
        self.bg = Color::Reset;
        #[cfg(feature = "underline-color")]
        {
            self.underline_color = Color::Reset;
        }
        self.modifier = Modifier::empty();
        self.skip = false;
    }
}

impl Default for Cell {
    fn default() -> Cell {
        #[allow(deprecated)] // For Cell::symbol
        Cell {
            symbol: " ".into(),
            fg: Color::Reset,
            bg: Color::Reset,
            #[cfg(feature = "underline-color")]
            underline_color: Color::Reset,
            modifier: Modifier::empty(),
            skip: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_field() {
        let mut cell = Cell::default();
        assert_eq!(cell.symbol(), " ");
        cell.set_symbol("あ"); // Multi-byte character
        assert_eq!(cell.symbol(), "あ");
        cell.set_symbol("👨‍👩‍👧‍👦"); // Multiple code units combined with ZWJ
        assert_eq!(cell.symbol(), "👨‍👩‍👧‍👦");
    }
}
