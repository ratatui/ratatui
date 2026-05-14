use unicode_width::UnicodeWidthStr;

/// Halfwidth Katakana Voiced Sound Mark (dakuten).
const HALFWIDTH_KATAKANA_VOICED_SOUND_MARK: char = '\u{FF9E}';
/// Halfwidth Katakana Semi-Voiced Sound Mark (handakuten).
const HALFWIDTH_KATAKANA_SEMI_VOICED_SOUND_MARK: char = '\u{FF9F}';

/// Returns the display width of a value in terminal cells.
///
/// This trait provides a unified way to compute cell widths for both string content
/// and [`Cell`](super::Cell)s:
///
/// - **`str`**: width is derived from [`UnicodeWidthStr`], with a fast path for single-byte ASCII
///   characters and a terminal-compatibility adjustment for halfwidth katakana dakuten/handakuten
///   (`U+FF9E`/`U+FF9F`).
/// - **[`Cell`](super::Cell)**: returns the
///   [`CellDiffOption::ForcedWidth`](super::CellDiffOption::ForcedWidth) when set, otherwise falls
///   back to the width of the cell's symbol.
pub trait CellWidth {
    /// Returns the display width in terminal cells.
    fn cell_width(&self) -> u16;
}

impl CellWidth for str {
    /// Returns the display width in terminal cells.
    ///
    /// ## Note
    ///
    /// Control characters are filtered out by `Span::styled_graphemes()` and
    /// `Buffer::set_stringn()` before reaching this point. `Cell::set_symbol()`
    /// and `set_char()` do not filter, but those are low-level APIs where the
    /// caller is responsible for providing valid content. Single-byte control
    /// characters that slip through will be reported as width 1.
    fn cell_width(&self) -> u16 {
        if self.len() == 1 {
            debug_assert!(
                !self.as_bytes()[0].is_ascii_control(),
                "control character passed to cell_width without filtering"
            );
            1
        } else {
            let width = self.width() as u16;
            width.saturating_add(count_halfwidth_sound_marks(self))
        }
    }
}

/// Returns how many halfwidth dakuten/handakuten marks are present.
///
/// `unicode-width` reports U+FF9E (ﾞ) and U+FF9F (ﾟ) as zero-width because
/// they have the `Grapheme_Extend` property, but terminals typically render
/// them as independent halfwidth characters occupying one cell each.
///
/// We compensate for that terminal behavior by adding `+1` for each occurrence.
/// This does not affect the combining variants U+3099 and U+309A, which keep
/// their normal combining behavior and width handling through `unicode-width`.
///
/// # References
///
/// - Ruby reline PR [#832](https://github.com/ruby/reline/pull/832): Fix cursor positioning for
///   invalid halfwidth dakuten/handakuten
/// - Microsoft Terminal Issue [#18087](https://github.com/microsoft/terminal/issues/18087):
///   Half-width Katakana and (han)dakuten should not overlap
/// - [Unicode L2/19-039](https://www.unicode.org/L2/L2019/19039-grapheme-break.pdf): Grapheme break
///   property for U+FF9E and U+FF9F
fn count_halfwidth_sound_marks(s: &str) -> u16 {
    s.chars()
        .filter(|c| {
            matches!(
                *c,
                HALFWIDTH_KATAKANA_VOICED_SOUND_MARK | HALFWIDTH_KATAKANA_SEMI_VOICED_SOUND_MARK
            )
        })
        .count() as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    fn width(s: &str) -> u16 {
        s.cell_width()
    }

    fn width_char(c: char) -> u16 {
        let mut buf = [0; 4];
        width(c.encode_utf8(&mut buf))
    }

    #[test]
    fn wide_char() {
        assert_eq!("あ".cell_width(), 2);
    }

    #[test]
    fn empty() {
        assert_eq!("".cell_width(), 0);
    }

    #[test]
    fn halfwidth_dakuten_alone() {
        assert_eq!(width_char(HALFWIDTH_KATAKANA_VOICED_SOUND_MARK), 1); // ﾞ
    }

    #[test]
    fn halfwidth_handakuten_alone() {
        assert_eq!(width_char(HALFWIDTH_KATAKANA_SEMI_VOICED_SOUND_MARK), 1); // ﾟ
    }

    #[test]
    fn halfwidth_katakana_with_dakuten() {
        // Valid combinations (halfwidth katakana + non-combining dakuten)
        assert_eq!(width("ｶﾞ"), 2); // U+FF76 + U+FF9E
        assert_eq!(width("ｻﾞ"), 2); // U+FF7B + U+FF9E
    }

    #[test]
    fn halfwidth_katakana_with_handakuten() {
        // Valid combinations (halfwidth katakana + non-combining handakuten)
        assert_eq!(width("ﾊﾟ"), 2); // U+FF8A + U+FF9F
        assert_eq!(width("ﾋﾟ"), 2); // U+FF8B + U+FF9F
    }

    #[test]
    fn non_katakana_with_halfwidth_dakuten() {
        // Non-katakana characters + halfwidth dakuten.
        // These form valid grapheme clusters but are linguistically incorrect.
        // The dakuten still takes 1 column width regardless.
        assert_eq!(width("aﾞ"), 2); // ASCII (1) + dakuten (1)
        assert_eq!(width("1ﾟ"), 2); // Digit (1) + handakuten (1)
        assert_eq!(width("あﾞ"), 3); // Hiragana (2) + dakuten (1)
        assert_eq!(width("紅ﾞ"), 3); // Kanji (2) + dakuten (1)
    }

    #[test]
    #[allow(clippy::unicode_not_nfc)]
    fn combining_dakuten_no_special_handling() {
        // Combining dakuten (U+3099) should follow unicode-width behavior.
        assert_eq!(width("ｶ゙"), 1); // U+FF76 + U+3099
        assert_eq!(width("ガ"), 2); // U+30AB + U+3099
    }

    #[test]
    #[allow(clippy::unicode_not_nfc)]
    fn combining_handakuten_no_special_handling() {
        // Combining handakuten (U+309A) should follow unicode-width behavior.
        assert_eq!(width("ﾊ゚"), 1); // U+FF8A + U+309A
        assert_eq!(width("パ"), 2); // U+30CF + U+309A
    }

    #[test]
    fn mixed_text_unchanged() {
        assert_eq!(width("a"), 1);
        assert_eq!(width("あ"), 2);
        assert_eq!(width("ｶ"), 1);
        assert_eq!(width("カ"), 2);
        assert_eq!(width("aｶﾞb"), 4); // a(1) + ｶﾞ(2) + b(1)
        assert_eq!(width("あｶﾞ"), 4); // あ(2) + ｶﾞ(2)
    }
}
