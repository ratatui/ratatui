use compact_str::CompactString;

use crate::prelude::*;

/// A buffer cell
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cell {
    /// The string to be drawn in the cell.
    ///
    /// This accepts unicode grapheme clusters which might take up more than one cell.
    ///
    /// This is a [`CompactString`] which is a wrapper around [`String`] that uses a small inline
    /// buffer for short strings.
    ///
    /// See <https://github.com/ratatui-org/ratatui/pull/601> for more information.
    symbol: CompactString,

    /// The foreground color of the cell.
    pub fg: Color,

    /// The background color of the cell.
    pub bg: Color,

    /// The underline color of the cell.
    #[cfg(feature = "underline-color")]
    pub underline_color: Color,

    /// The modifier of the cell.
    pub modifier: Modifier,

    /// Whether the cell should be skipped when copying (diffing) the buffer to the screen.
    pub skip: bool,
}

impl Cell {
    /// Gets the symbol of the cell.
    pub fn symbol(&self) -> &str {
        self.symbol.as_str()
    }

    /// Sets the symbol of the cell.
    pub fn set_symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = CompactString::new(symbol);
        self
    }

    /// Sets the symbol of the cell to a single character.
    pub fn set_char(&mut self, ch: char) -> &mut Self {
        let mut buf = [0; 4];
        self.symbol = CompactString::new(ch.encode_utf8(&mut buf));
        self
    }

    /// Sets the foreground color of the cell.
    pub fn set_fg(&mut self, color: Color) -> &mut Self {
        self.fg = color;
        self
    }

    /// Sets the background color of the cell.
    pub fn set_bg(&mut self, color: Color) -> &mut Self {
        self.bg = color;
        self
    }

    /// Sets the style of the cell.
    ///
    ///  `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    pub fn set_style<S: Into<Style>>(&mut self, style: S) -> &mut Self {
        let style = style.into();
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

    /// Returns the style of the cell.
    pub fn style(&self) -> Style {
        #[cfg(feature = "underline-color")]
        return Style::default()
            .fg(self.fg)
            .bg(self.bg)
            .underline_color(self.underline_color)
            .add_modifier(self.modifier);

        #[cfg(not(feature = "underline-color"))]
        return Style::default()
            .fg(self.fg)
            .bg(self.bg)
            .add_modifier(self.modifier);
    }

    /// Sets the cell to be skipped when copying (diffing) the buffer to the screen.
    ///
    /// This is helpful when it is necessary to prevent the buffer from overwriting a cell that is
    /// covered by an image from some terminal graphics protocol (Sixel / iTerm / Kitty ...).
    pub fn set_skip(&mut self, skip: bool) -> &mut Self {
        self.skip = skip;
        self
    }

    /// Resets the cell to the default state.
    pub fn reset(&mut self) {
        self.symbol = CompactString::new_inline(" ");
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
    fn default() -> Self {
        Self {
            symbol: CompactString::new_inline(" "),
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
