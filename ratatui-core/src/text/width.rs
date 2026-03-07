use unicode_segmentation::UnicodeSegmentation;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Halfwidth Katakana Voiced Sound Mark (dakuten)
const HALFWIDTH_DAKUTEN: char = '\u{FF9E}';
/// Halfwidth Katakana Semi-Voiced Sound Mark (handakuten)
const HALFWIDTH_HANDAKUTEN: char = '\u{FF9F}';

/// Checks if a character is a halfwidth dakuten or handakuten (non-combining).
///
/// These characters (U+FF9E ﾞ and U+FF9F ﾟ) are non-combining marks used with halfwidth
/// katakana. Despite being classified as having `Grapheme_Extend` property in Unicode,
/// they are displayed as separate characters in terminals, each taking 1 column width.
///
/// This is distinct from the combining variants (U+3099 and U+309A), which are true
/// combining characters with zero width.
///
/// # References
/// - Ruby reline PR #832: Fix cursor positioning for invalid halfwidth dakuten/handakuten
/// - Microsoft Terminal Issue #18087: Half-width Katakana and (han)dakuten should not overlap
/// - Unicode L2/19-039: Grapheme break property for U+FF9E and U+FF9F
const fn is_halfwidth_dakuten_handakuten(c: char) -> bool {
    matches!(c, HALFWIDTH_DAKUTEN | HALFWIDTH_HANDAKUTEN)
}

/// Calculates the width of a grapheme cluster as displayed in a terminal.
///
/// This function addresses a specific issue with halfwidth katakana dakuten and handakuten
/// marks (U+FF9E ﾞ and U+FF9F ﾟ). While `unicode-width` reports these as width 0 (due to
/// their `Grapheme_Extend` property), terminals display them as independent characters with
/// width 1 each, inherited from legacy Shift-JIS encoding where they were separate characters.
///
/// # Background
///
/// In legacy Shift-JIS encoding (JIS X 0201), halfwidth katakana and dakuten/handakuten
/// were completely separate 1-byte characters. This behavior persists in terminal emulators
/// for compatibility, where `ｶﾞ` is rendered as two distinct character cells, not as a
/// combined single character.
///
/// The combining variants (U+3099 and U+309A) properly combine with preceding characters
/// and report correct width 0, so they don't need special handling.
fn width(grapheme: &str) -> usize {
    // Check only the last character because halfwidth dakuten/handakuten (U+FF9E/U+FF9F)
    // always appear at the END of a grapheme cluster, never at the beginning or middle.
    // This is guaranteed by Unicode normalization rules:
    // - Characters with Grapheme_Extend property (including these marks) must follow their base
    //   character
    // - If a dakuten appears at the start, it forms a separate grapheme cluster
    // Example: "ﾞｶ" becomes two clusters: "ﾞ" (alone) + "ｶ" (alone), not "ﾞｶ" (combined)
    if let Some(c) = grapheme.chars().last() {
        if is_halfwidth_dakuten_handakuten(c) {
            // Sum up the width of each character individually
            return grapheme
                .chars()
                .map(|c| {
                    if is_halfwidth_dakuten_handakuten(c) {
                        // Halfwidth dakuten/handakuten are width 1 in terminals
                        1
                    } else {
                        // Use unicode-width for other characters
                        // unwrap_or(1) handles control characters and other edge cases
                        c.width().unwrap_or(1)
                    }
                })
                .sum();
        }
    }

    // For all other cases, use unicode-width as-is
    UnicodeWidthStr::width(grapheme)
}

/// Extension trait to calculate terminal display width for strings.
///
/// This trait provides a `.width()` method for `&str` that correctly handles
/// halfwidth katakana dakuten and handakuten marks, similar to how `UnicodeWidthStr`
/// provides `.width()`.
///
/// # Examples
///
/// ```
/// use ratatui_core::text::TerminalWidthStr;
///
/// assert_eq!("hello".width(), 5);
/// assert_eq!("ｶﾞｷﾞｸﾞ".width(), 6); // 3 graphemes × 2 width each
/// assert_eq!("あいう".width(), 6); // 3 graphemes × 2 width each
/// ```
pub trait TerminalWidthStr {
    /// Calculates the terminal display width of the string.
    ///
    /// This correctly handles halfwidth katakana dakuten/handakuten marks
    /// by treating them as width 1 each, unlike `unicode-width` which reports them as width 0.
    fn width(&self) -> usize;
}

impl TerminalWidthStr for str {
    fn width(&self) -> usize {
        self.graphemes(true).map(width).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn halfwidth_dakuten_alone() {
        assert_eq!(width("\u{FF9E}"), 1); // ﾞ
    }

    #[test]
    fn halfwidth_handakuten_alone() {
        assert_eq!(width("\u{FF9F}"), 1); // ﾟ
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
        // Non-katakana characters + halfwidth dakuten
        // These form valid grapheme clusters but are linguistically incorrect.
        // The dakuten still takes 1 column width regardless.
        assert_eq!(width("aﾞ"), 2); // ASCII (width 1) + dakuten (width 1)
        assert_eq!(width("1ﾟ"), 2); // Digit (width 1) + handakuten (width 1)
        assert_eq!(width("あﾞ"), 3); // Hiragana (width 2) + dakuten (width 1)
        assert_eq!(width("紅ﾞ"), 3); // Kanji (width 2) + dakuten (width 1)
    }

    #[test]
    #[allow(clippy::unicode_not_nfc)]
    fn combining_dakuten_no_special_handling() {
        // Combining dakuten (U+3099) - unicode-width handles correctly
        assert_eq!(width("ｶ゙"), 1); // U+FF76 + U+3099
        assert_eq!(width("ガ"), 2); // U+30AB + U+3099
    }

    #[test]
    #[allow(clippy::unicode_not_nfc)]
    fn combining_handakuten_no_special_handling() {
        // Combining handakuten (U+309A) - unicode-width handles correctly
        assert_eq!(width("ﾊ゚"), 1); // U+FF8A + U+309A
        assert_eq!(width("パ"), 2); // U+30CF + U+309A
    }

    #[test]
    fn normal_text_unchanged() {
        // Regular text should use unicode-width as-is
        assert_eq!(TerminalWidthStr::width("a"), 1);
        assert_eq!(TerminalWidthStr::width("あ"), 2);
        assert_eq!(TerminalWidthStr::width("ｶ"), 1);
        assert_eq!(TerminalWidthStr::width("カ"), 2);
    }

    #[test]
    fn terminal_width_str_trait_ascii() {
        assert_eq!(TerminalWidthStr::width(""), 0);
        assert_eq!(TerminalWidthStr::width("a"), 1);
        assert_eq!(TerminalWidthStr::width("hello"), 5);
        assert_eq!(TerminalWidthStr::width("hello world"), 11);
    }

    #[test]
    fn terminal_width_str_trait_fullwidth() {
        assert_eq!(TerminalWidthStr::width("あ"), 2);
        assert_eq!(TerminalWidthStr::width("あいう"), 6);
        assert_eq!(TerminalWidthStr::width("こんにちは"), 10);
    }

    #[test]
    fn terminal_width_str_trait_halfwidth_katakana_dakuten() {
        assert_eq!(TerminalWidthStr::width("ｶﾞ"), 2);
        assert_eq!(TerminalWidthStr::width("ｶﾞｷﾞｸﾞ"), 6);
        assert_eq!(TerminalWidthStr::width("ﾊﾟﾋﾟﾌﾟ"), 6);
    }

    #[test]
    fn terminal_width_str_trait_mixed() {
        assert_eq!(TerminalWidthStr::width("aｶﾞb"), 4); // a(1) + ｶﾞ(2) + b(1)
        assert_eq!(TerminalWidthStr::width("あｶﾞ"), 4); // あ(2) + ｶﾞ(2)
    }
}
