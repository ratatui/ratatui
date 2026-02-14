use core::num::NonZeroU8;
use crate::style::{Color, Modifier, Style};
use crate::symbols::merge::MergeStrategy;

/// Cell diffing options
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CellDiffOption {
    #[default]
    /// No special option.
    None,
    /// Skip this cell when diffing.
    ///
    /// This is helpful when it is necessary to prevent the buffer from overwriting a cell that is
    /// covered by something from an escape sequence, such as graphics or links.
    Skip,
    /// Force a width regardless of the symbol text width.
    ///
    /// Escape sequences will have some computed width that does match what is written to the
    /// screen.
    ForcedWidth(NonZeroU8),
}

/// A 4-byte inline string that stores a single Unicode codepoint (up to 4-byte UTF-8).
///
/// The length is derived from the UTF-8 leading byte via a branchless lookup table,
/// eliminating the need for a separate length field. Zero-padded trailing bytes enable
/// raw byte comparison for equality checks without computing the length.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct EmbeddedStr {
    bytes: [u8; 4],
}

/// Branchless UTF-8 length lookup indexed by the high nibble of the leading byte.
const UTF8_LEN: [u8; 16] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 3, 4];

impl EmbeddedStr {
    /// Creates a new `EmbeddedStr` from a char at compile time.
    pub const fn new(symbol: char) -> Self {
        let mut buf = [0u8; 4];
        let encoded = symbol.encode_utf8(&mut buf);
        let len = encoded.len();
        // copy only the encoded bytes (clear any trailing garbage)
        let mut bytes = [0u8; 4];
        let mut i = 0;
        while i < len {
            bytes[i] = buf[i];
            i += 1;
        }
        Self { bytes }
    }

    /// Returns the byte length of the stored UTF-8 sequence.
    pub fn len(&self) -> usize {
        UTF8_LEN[(self.bytes[0] >> 4) as usize] as usize
    }

    /// Returns the stored string as a `&str`.
    pub fn as_str(&self) -> &str {
        #[allow(unsafe_code)]
        unsafe {
            core::str::from_utf8_unchecked(&self.bytes[..self.len()])
        }
    }
}

impl Default for EmbeddedStr {
    fn default() -> Self {
        Self {
            bytes: [b' ', 0, 0, 0],
        }
    }
}

impl From<char> for EmbeddedStr {
    fn from(c: char) -> Self {
        let c = c as u32;

        // fast path for ASCII
        if c < 0x80 {
            return Self {
                bytes: [c as u8, 0, 0, 0],
            };
        }

        let mut bytes = [0u8; 4];
        if c < 0x800 {
            bytes[0] = 0xC0 | ((c >> 6) as u8);
            bytes[1] = 0x80 | ((c & 0x3F) as u8);
        } else if c < 0x10000 {
            bytes[0] = 0xE0 | ((c >> 12) as u8);
            bytes[1] = 0x80 | (((c >> 6) & 0x3F) as u8);
            bytes[2] = 0x80 | ((c & 0x3F) as u8);
        } else {
            bytes[0] = 0xF0 | ((c >> 18) as u8);
            bytes[1] = 0x80 | (((c >> 12) & 0x3F) as u8);
            bytes[2] = 0x80 | (((c >> 6) & 0x3F) as u8);
            bytes[3] = 0x80 | ((c & 0x3F) as u8);
        }
        Self { bytes }
    }
}

impl From<&str> for EmbeddedStr {
    fn from(s: &str) -> Self {
        let bytes = s.as_bytes();
        if bytes.len() <= 4 {
            let mut result_bytes = [0u8; 4];
            result_bytes[..bytes.len()].copy_from_slice(bytes);
            Self {
                bytes: result_bytes,
            }
        } else {
            Self {
                bytes: [b' ', 0, 0, 0],
            }
        }
    }
}

impl AsRef<str> for EmbeddedStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// A buffer cell
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cell {
    /// The string to be drawn in the cell.
    ///
    /// Stores a single Unicode codepoint inline as 4 bytes of UTF-8. Defaults to a space.
    symbol: EmbeddedStr,

    /// The foreground color of the cell.
    pub fg: Color,

    /// The background color of the cell.
    pub bg: Color,

    /// The underline color of the cell.
    #[cfg(feature = "underline-color")]
    pub underline_color: Color,

    /// The modifier of the cell.
    pub modifier: Modifier,

    /// Special option applied when copying (diffing) the buffer to the screen (or another buffer).
    pub diff_option: CellDiffOption,
}

impl Default for Cell {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Cell {
    /// An empty `Cell`
    pub const EMPTY: Self = Self {
        symbol: EmbeddedStr {
            bytes: [b' ', 0, 0, 0],
        },
        fg: Color::Reset,
        bg: Color::Reset,
        #[cfg(feature = "underline-color")]
        underline_color: Color::Reset,
        modifier: Modifier::empty(),
        diff_option: CellDiffOption::None,
    };

    /// Creates a new `Cell` with the given symbol.
    pub fn new(symbol: &'static str) -> Self {
        Self {
            symbol: symbol.into(),
            ..Self::EMPTY
        }
    }

    /// Gets the symbol of the cell.
    #[must_use]
    pub fn symbol(&self) -> &str {
        self.symbol.as_str()
    }

    /// Merges the symbol of the cell with the one already on the cell, using the provided
    /// [`MergeStrategy`].
    ///
    /// Merges [Box Drawing Unicode block] characters to create a single character representing
    /// their combination, useful for [border collapsing]. Currently limited to box drawing
    /// characters, with potential future support for others.
    ///
    /// Merging may not be perfect due to Unicode limitations; some symbol combinations might not
    /// produce a valid character. [`MergeStrategy`] defines how to handle such cases, e.g.,
    /// `Exact` for valid merges only, or `Fuzzy` for close matches.
    ///
    /// # Example
    ///
    /// ```
    /// # use ratatui_core::buffer::Cell;
    /// use ratatui_core::symbols::merge::MergeStrategy;
    ///
    /// assert_eq!(
    ///     Cell::new("┘")
    ///         .merge_symbol("┏", MergeStrategy::Exact)
    ///         .symbol(),
    ///     "╆",
    /// );
    ///
    /// assert_eq!(
    ///     Cell::new("╭")
    ///         .merge_symbol("┘", MergeStrategy::Fuzzy)
    ///         .symbol(),
    ///     "┼",
    /// );
    /// ```
    ///
    /// [border collapsing]: https://ratatui.rs/recipes/layout/collapse-borders/
    /// [Box Drawing Unicode block]: https://en.wikipedia.org/wiki/Box_Drawing
    pub fn merge_symbol(&mut self, symbol: &str, strategy: MergeStrategy) -> &mut Self {
        let merged_symbol = if self.symbol.bytes[0] == b' ' {
            symbol
        } else {
            strategy.merge(self.symbol.as_str(), symbol)
        };
        self.symbol = merged_symbol.into();
        self
    }

    /// Sets the symbol of the cell.
    pub fn set_symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = symbol.into();
        self
    }

    /// Appends a symbol to the cell.
    ///
    /// This is particularly useful for adding zero-width characters to the cell.
    pub(crate) fn append_symbol(&mut self, _symbol: &str) -> &mut Self {
        // todo: not supported for EmbeddedStr (4-byte inline storage)
        self
    }

    /// Sets the symbol of the cell to a single character.
    pub fn set_char(&mut self, ch: char) -> &mut Self {
        self.symbol = ch.into();
        self
    }

    /// Sets the foreground color of the cell.
    pub const fn set_fg(&mut self, color: Color) -> &mut Self {
        self.fg = color;
        self
    }

    /// Sets the background color of the cell.
    pub const fn set_bg(&mut self, color: Color) -> &mut Self {
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

    #[deprecated(
        since = "0.30.0",
        note = "use `set_diff_option(CellDiffOption::Skip)` instead"
    )]
    /// Set cell diffing option to [`CellDiffOption::Skip`].
    pub const fn set_skip(&mut self, skip: bool) -> &mut Self {
        self.diff_option = if skip {
            CellDiffOption::Skip
        } else {
            CellDiffOption::None
        };
        self
    }

    /// Sets cell [`CellDiffOption`].
    ///
    /// The diff options are for dealing with cells that are wider than a unit, or that should not
    /// be updated at all (skip output due to preceding wider cells).
    pub const fn set_diff_option(&mut self, diff_option: CellDiffOption) -> &mut Self {
        self.diff_option = diff_option;
        self
    }

    /// Resets the cell to the empty state.
    pub fn reset(&mut self) {
        *self = Self::EMPTY;
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.symbol.bytes == other.symbol.bytes
            && self.fg == other.fg
            && self.bg == other.bg
            && {
                #[cfg(feature = "underline-color")]
                {
                    self.underline_color == other.underline_color
                }
                #[cfg(not(feature = "underline-color"))]
                true
            }
            && self.modifier == other.modifier
            && self.diff_option == other.diff_option
    }
}

impl Eq for Cell {}

impl core::hash::Hash for Cell {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.symbol.bytes.hash(state);
        self.fg.hash(state);
        self.bg.hash(state);
        #[cfg(feature = "underline-color")]
        self.underline_color.hash(state);
        self.modifier.hash(state);
        self.diff_option.hash(state);
    }
}

impl From<char> for Cell {
    fn from(ch: char) -> Self {
        let mut cell = Self::EMPTY;
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
                symbol: "あ".into(),
                fg: Color::Reset,
                bg: Color::Reset,
                #[cfg(feature = "underline-color")]
                underline_color: Color::Reset,
                modifier: Modifier::empty(),
                diff_option: CellDiffOption::None,
            }
        );
    }

    #[test]
    fn empty() {
        let cell = Cell::EMPTY;
        assert_eq!(cell.symbol(), " ");
    }

    #[test]
    fn set_symbol_multibyte() {
        let mut cell = Cell::EMPTY;
        cell.set_symbol("あ"); // 3-byte character
        assert_eq!(cell.symbol(), "あ");
    }

    #[test]
    fn set_symbol_4byte() {
        let mut cell = Cell::EMPTY;
        cell.set_char('🦀'); // 4-byte character
        assert_eq!(cell.symbol(), "🦀");
    }

    #[test]
    fn set_symbol_overflow_falls_back_to_space() {
        let mut cell = Cell::EMPTY;
        cell.set_symbol("👨‍👩‍👧‍👦"); // >4 bytes, falls back to space
        assert_eq!(cell.symbol(), " ");
    }

    #[test]
    fn set_char() {
        let mut cell = Cell::EMPTY;
        cell.set_char('あ'); // Multi-byte character
        assert_eq!(cell.symbol(), "あ");
    }

    #[test]
    fn set_fg() {
        let mut cell = Cell::EMPTY;
        cell.set_fg(Color::Red);
        assert_eq!(cell.fg, Color::Red);
    }

    #[test]
    fn set_bg() {
        let mut cell = Cell::EMPTY;
        cell.set_bg(Color::Red);
        assert_eq!(cell.bg, Color::Red);
    }

    #[test]
    fn set_style() {
        let mut cell = Cell::EMPTY;
        cell.set_style(Style::new().fg(Color::Red).bg(Color::Blue));
        assert_eq!(cell.fg, Color::Red);
        assert_eq!(cell.bg, Color::Blue);
    }

    #[test]
    fn set_skip() {
        let mut cell = Cell::EMPTY;
        cell.set_diff_option(CellDiffOption::Skip);
        assert_eq!(cell.diff_option, CellDiffOption::Skip);
    }

    #[test]
    fn reset() {
        let mut cell = Cell::EMPTY;
        cell.set_symbol("あ");
        cell.set_fg(Color::Red);
        cell.set_bg(Color::Blue);
        cell.set_diff_option(CellDiffOption::Skip);
        cell.reset();
        assert_eq!(cell.symbol(), " ");
        assert_eq!(cell.fg, Color::Reset);
        assert_eq!(cell.bg, Color::Reset);
        assert_eq!(cell.diff_option, CellDiffOption::None);
    }

    #[test]
    fn style() {
        let cell = Cell::EMPTY;
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
    fn embedded_str_ascii() {
        let e = EmbeddedStr::from('A');
        assert_eq!(e.as_str(), "A");
        assert_eq!(e.len(), 1);
    }

    #[test]
    fn embedded_str_cjk() {
        let e = EmbeddedStr::from('你');
        assert_eq!(e.as_str(), "你");
        assert_eq!(e.len(), 3);
    }

    #[test]
    fn embedded_str_4byte() {
        let e = EmbeddedStr::from('🦀');
        assert_eq!(e.as_str(), "🦀");
        assert_eq!(e.len(), 4);
    }

    #[test]
    fn embedded_str_default_is_space() {
        let e = EmbeddedStr::default();
        assert_eq!(e.as_str(), " ");
    }
}
