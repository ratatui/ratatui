use std::hash::{Hash, Hasher};

use compact_str::CompactString;
use unicode_width::UnicodeWidthStr;

use crate::style::{Color, Modifier, Style};

/// A buffer cell
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cell {
    /// The string to be drawn in the cell.
    ///
    /// This accepts unicode grapheme clusters which might take up more than one cell.
    ///
    /// This is a [`CompactString`] which is a wrapper around [`String`] that uses a small inline
    /// buffer for short strings.
    ///
    /// See <https://github.com/ratatui/ratatui/pull/601> for more information.
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

    /// Cache the width of the cell.
    width: std::cell::Cell<Option<usize>>,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        let eq = self.symbol == other.symbol
            && self.fg == other.fg
            && self.bg == other.bg
            && self.modifier == other.modifier
            && self.skip == other.skip;
        // explicitly not comparing width, as it is a cache and may be not set
        // && self.width == other.width

        #[cfg(feature = "underline-color")]
        return eq && self.underline_color == other.underline_color;
        #[cfg(not(feature = "underline-color"))]
        return eq;
    }
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
        self.fg.hash(state);
        self.bg.hash(state);
        #[cfg(feature = "underline-color")]
        self.underline_color.hash(state);
        self.modifier.hash(state);
        self.skip.hash(state);

        // explicitly not hashing width, as it is a cache and not part of the cell's identity
        // self.width.hash(state);
    }
}

impl Cell {
    /// An empty `Cell`
    pub const fn empty() -> Self {
        Self {
            symbol: CompactString::const_new(" "),
            fg: Color::Reset,
            bg: Color::Reset,
            #[cfg(feature = "underline-color")]
            underline_color: Color::Reset,
            modifier: Modifier::empty(),
            skip: false,
            width: std::cell::Cell::new(Some(1)),
        }
    }

    /// Creates a new `Cell` with the given symbol.
    ///
    /// This works at compile time and puts the symbol onto the stack. Fails to build when the
    /// symbol doesnt fit onto the stack and requires to be placed on the heap. Use
    /// `Self::default().set_symbol()` in that case. See [`CompactString::const_new`] for more
    /// details on this.
    pub const fn new(symbol: &'static str) -> Self {
        Self {
            symbol: CompactString::const_new(symbol),
            fg: Color::Reset,
            bg: Color::Reset,
            #[cfg(feature = "underline-color")]
            underline_color: Color::Reset,
            modifier: Modifier::empty(),
            skip: false,
            width: std::cell::Cell::new(None),
        }
    }

    /// Gets the symbol of the cell.
    #[must_use]
    pub fn symbol(&self) -> &str {
        self.symbol.as_str()
    }

    /// Sets the symbol of the cell.
    pub fn set_symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = CompactString::new(symbol);
        self.width.set(None);
        self
    }

    /// Appends a symbol to the cell.
    ///
    /// This is particularly useful for adding zero-width characters to the cell.
    pub(crate) fn append_symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol.push_str(symbol);
        self.width.set(None);
        self
    }

    /// Sets the symbol of the cell to a single character.
    pub fn set_char(&mut self, ch: char) -> &mut Self {
        let mut buf = [0; 4];
        self.symbol = CompactString::new(ch.encode_utf8(&mut buf));
        self.width.set(None);
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
    #[must_use]
    pub const fn style(&self) -> Style {
        Style {
            fg: Some(self.fg),
            bg: Some(self.bg),
            #[cfg(feature = "underline-color")]
            underline_color: Some(self.underline_color),
            add_modifier: self.modifier,
            sub_modifier: Modifier::empty(),
        }
    }

    /// Sets the cell to be skipped when copying (diffing) the buffer to the screen.
    ///
    /// This is helpful when it is necessary to prevent the buffer from overwriting a cell that is
    /// covered by an image from some terminal graphics protocol (Sixel / iTerm / Kitty ...).
    pub fn set_skip(&mut self, skip: bool) -> &mut Self {
        self.skip = skip;
        self
    }

    /// Resets the cell to the empty state.
    pub fn reset(&mut self) {
        self.symbol = CompactString::const_new(" ");
        self.fg = Color::Reset;
        self.bg = Color::Reset;
        #[cfg(feature = "underline-color")]
        {
            self.underline_color = Color::Reset;
        }
        self.modifier = Modifier::empty();
        self.skip = false;
        self.width.set(Some(1));
    }

    /// Returns the width of the cell.
    ///
    /// This value is cached and will only be recomputed when the cell is modified.
    #[must_use]
    pub fn width(&self) -> usize {
        if let Some(width) = self.width.get() {
            width
        } else {
            let width = self.symbol().width();
            self.width.set(Some(width));
            width
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<char> for Cell {
    fn from(ch: char) -> Self {
        let mut cell = Self::empty();
        cell.set_char(ch);
        cell
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let cell = Cell::new("あ");
        assert_eq!(
            cell,
            Cell {
                symbol: CompactString::const_new("あ"),
                fg: Color::Reset,
                bg: Color::Reset,
                #[cfg(feature = "underline-color")]
                underline_color: Color::Reset,
                modifier: Modifier::empty(),
                skip: false,
                width: std::cell::Cell::new(None),
            }
        );
    }

    #[test]
    fn empty() {
        let cell = Cell::empty();
        assert_eq!(cell.symbol(), " ");
    }

    #[test]
    fn set_symbol() {
        let mut cell = Cell::empty();
        cell.set_symbol("あ"); // Multi-byte character
        assert_eq!(cell.symbol(), "あ");
        cell.set_symbol("👨‍👩‍👧‍👦"); // Multiple code units combined with ZWJ
        assert_eq!(cell.symbol(), "👨‍👩‍👧‍👦");
    }

    #[test]
    fn append_symbol() {
        let mut cell = Cell::empty();
        cell.set_symbol("あ"); // Multi-byte character
        cell.append_symbol("\u{200B}"); // zero-width space
        assert_eq!(cell.symbol(), "あ\u{200B}");
    }

    #[test]
    fn set_char() {
        let mut cell = Cell::empty();
        cell.set_char('あ'); // Multi-byte character
        assert_eq!(cell.symbol(), "あ");
    }

    #[test]
    fn set_fg() {
        let mut cell = Cell::empty();
        cell.set_fg(Color::Red);
        assert_eq!(cell.fg, Color::Red);
    }

    #[test]
    fn set_bg() {
        let mut cell = Cell::empty();
        cell.set_bg(Color::Red);
        assert_eq!(cell.bg, Color::Red);
    }

    #[test]
    fn set_style() {
        let mut cell = Cell::empty();
        cell.set_style(Style::new().fg(Color::Red).bg(Color::Blue));
        assert_eq!(cell.fg, Color::Red);
        assert_eq!(cell.bg, Color::Blue);
    }

    #[test]
    fn set_skip() {
        let mut cell = Cell::empty();
        cell.set_skip(true);
        assert!(cell.skip);
    }

    #[test]
    fn reset() {
        let mut cell = Cell::empty();
        cell.set_symbol("あ");
        cell.set_fg(Color::Red);
        cell.set_bg(Color::Blue);
        cell.set_skip(true);
        cell.reset();
        assert_eq!(cell.symbol(), " ");
        assert_eq!(cell.fg, Color::Reset);
        assert_eq!(cell.bg, Color::Reset);
        assert!(!cell.skip);
    }

    #[test]
    fn style() {
        let cell = Cell::empty();
        assert_eq!(
            cell.style(),
            Style {
                fg: Some(Color::Reset),
                bg: Some(Color::Reset),
                #[cfg(feature = "underline-color")]
                underline_color: Some(Color::Reset),
                add_modifier: Modifier::empty(),
                sub_modifier: Modifier::empty(),
            }
        );
    }

    #[test]
    fn default() {
        let cell = Cell::default();
        assert_eq!(cell.symbol(), " ");
    }

    #[test]
    fn cell_eq() {
        let cell1 = Cell::new("あ");
        let cell2 = Cell::new("あ");
        assert_eq!(cell1, cell2);
    }

    #[test]
    fn cell_ne() {
        let cell1 = Cell::new("あ");
        let cell2 = Cell::new("い");
        assert_ne!(cell1, cell2);
    }

    #[test]
    fn width() {
        let cell = Cell::new("あ");
        assert_eq!(cell.width, std::cell::Cell::new(None)); // not yet cached
        assert_eq!(cell.width(), 2);
        assert_eq!(cell.width, std::cell::Cell::new(Some(2))); // cached
    }
}
