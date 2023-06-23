use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::{Line, StyledGrapheme, Text};
use crate::{layout::Alignment, style::Style};

// NBSP is a non-breaking space which is essentially a whitespace character that is treated
// the same as non-whitespace characters in wrapping algorithms
const NBSP: &str = "\u{00a0}";

/// Describes how to wrap text across lines.
///
/// ## Examples
///
/// ```
/// # use ratatui::widgets::Paragraph;
/// # use ratatui::text::{Text, Wrap};
/// let bullet_points = Text::from(r#"Some indented points:
///     - First thing goes here and is long so that it wraps
///     - Here is another point that is long enough to wrap"#);
///
/// // Wrapping on char boundaries (window width of 30 chars):
/// Paragraph::new(bullet_points.clone()).wrap(Wrap::CharBoundary);
/// // Some indented points:
/// //     - First thing goes here an
/// // d is long so that it wraps
/// //     - Here is another point th
/// // at is long enough to wrap
///
/// // Wrapping on word boundaries
/// Paragraph::new(bullet_points).wrap(Wrap::WordBoundary);
/// // Some indented points:
/// //     - First thing goes here
/// // and is long so that it wraps
/// //     - Here is another point
/// // that is long enough to wrap
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Wrap {
    WordBoundary,
    CharBoundary,
}

impl<'a> Line<'a> {
    pub fn wrap_truncate(
        &'a self,
        base_style: Style,
        base_alignment: Alignment,
        viewport_width: usize,
        horizontal_scroll: usize,
        // vertical_scroll: usize,
    ) -> Vec<Line<'a>> {
        // If the area to draw the text is 0 wide, return an empty vector.
        if viewport_width == 0 {
            return vec![];
        }

        // Create an iterator over the styled graphemes in the line
        let styled_graphemes = self.styled_graphemes(base_style);

        // If the line has an alignment, use it. Otherwise, use the base alignment.
        let alignment = self.alignment.unwrap_or(base_alignment);

        let mut working_line = Vec::new();
        let mut working_line_width = 0;
        let mut remaining_scroll = horizontal_scroll;

        for StyledGrapheme { symbol, style } in styled_graphemes {
            // Ignore characters that are wider than the maximum width.
            if symbol.width() > viewport_width {
                continue;
            }

            // Truncate the line once the maximum width is reached.
            if working_line_width + symbol.width() > viewport_width {
                break;
            }

            // TODO: It seems that horizontal scroll is only supported on left-aligned lines.
            // Before adding the symbol to the line, adjust it for horizontal scroll.
            // This means that the symbol may or may not be truncated based on its position in the
            // line and whether it would be rendered outside (to the left) of the widget border.
            let scrolled_symbol =
                adjust_symbol_for_horizontal_scroll(symbol, &mut remaining_scroll, alignment);

            // Add the symbol to the line and update the line width.
            working_line_width += scrolled_symbol.width();
            working_line.push(StyledGrapheme {
                symbol: scrolled_symbol,
                style,
            });
        }

        // Convert the `Vec<StyledGrapheme>` into a `Line` so that it can be returned.
        vec![working_line
            .into_iter()
            .collect::<Line<'a>>()
            .alignment(alignment)]
    }

    pub fn wrap_char_boundary(
        &'a self,
        base_style: Style,
        base_alignment: Alignment,
        viewport_width: usize,
        trim: bool,
    ) -> Vec<Line<'a>> {
        // If the area to draw the text is 0 wide, return an empty vector.
        if viewport_width == 0 {
            return vec![];
        }

        // Create an iterator over the styled graphemes in the line
        let styled_graphemes = self.styled_graphemes(base_style);

        // If the line has an alignment, use it. Otherwise, use the base alignment.
        let alignment = self.alignment.unwrap_or(base_alignment);

        let mut wrapped_lines = Vec::new();
        let mut working_line = Vec::new();
        let mut working_line_width = 0;

        let mut has_seen_non_whitespace_this_line = false;

        for StyledGrapheme { symbol, style } in styled_graphemes {
            // Ignore characters that are wider than the maximum width.
            if symbol.width() > viewport_width {
                continue;
            }

            // Wrap the line once the maximum width is reached.
            if working_line_width + symbol.width() > viewport_width {
                // Save the working line to be returned, and reset the working line.
                wrapped_lines.push(
                    working_line
                        .into_iter()
                        .collect::<Line<'a>>()
                        .alignment(alignment),
                );
                working_line = Vec::new();
                working_line_width = 0;
                has_seen_non_whitespace_this_line = false;
            }

            let symbol_whitespace = is_whitespace(symbol);

            // If trimming is enabled, and the line has not started, and the symbol is whitespace,
            // trim the symbol from the line. If the symbol is not whitespace, mark the line as
            // having started so that whitespace will no longer be trimmed.
            if trim && !has_seen_non_whitespace_this_line && symbol_whitespace {
                continue;
            } else if trim && !has_seen_non_whitespace_this_line && !symbol_whitespace {
                has_seen_non_whitespace_this_line = true;
            }

            // Add the symbol to the line and update the line width.
            working_line_width += symbol.width();
            working_line.push(StyledGrapheme { symbol, style });
        }

        // Push the final line to the vector of lines.
        wrapped_lines.push(
            working_line
                .into_iter()
                .collect::<Line<'a>>()
                .alignment(alignment),
        );

        // Remove any trailing empty lines.
        match wrapped_lines.last() {
            Some(last_line) if wrapped_lines.len() != 1 && last_line.spans.is_empty() => {
                wrapped_lines.pop();
            }
            _ => {}
        }

        wrapped_lines
    }

    #[allow(unused)]
    pub fn wrap_word_boundary(
        &'a self,
        base_style: Style,
        base_alignment: Alignment,
        viewport_width: usize,
        trim: bool,
    ) -> Vec<Line<'a>> {
        todo!()
    }
}

impl<'a> Text<'a> {
    pub fn wrap_truncate(
        &'a self,
        base_style: Style,
        base_alignment: Alignment,
        viewport_width: usize,
        horizontal_scroll: usize,
    ) -> Vec<Line<'a>> {
        let mut wrapped_lines = Vec::new();
        for line in &self.lines {
            wrapped_lines.extend(line.wrap_truncate(
                base_style,
                base_alignment,
                viewport_width,
                horizontal_scroll,
            ));
        }

        wrapped_lines
    }

    pub fn wrap_char_boundary(
        &'a self,
        base_style: Style,
        base_alignment: Alignment,
        viewport_width: usize,
        trim: bool,
    ) -> Vec<Line<'a>> {
        let mut wrapped_lines = Vec::new();
        for line in &self.lines {
            wrapped_lines.extend(line.wrap_char_boundary(
                base_style,
                base_alignment,
                viewport_width,
                trim,
            ));
        }

        wrapped_lines
    }

    pub fn wrap_word_boundary(
        &'a self,
        base_style: Style,
        base_alignment: Alignment,
        viewport_width: usize,
        trim: bool,
    ) -> Vec<Line<'a>> {
        let mut wrapped_lines = Vec::new();
        for line in &self.lines {
            wrapped_lines.extend(line.wrap_word_boundary(
                base_style,
                base_alignment,
                viewport_width,
                trim,
            ));
        }

        wrapped_lines
    }
}

// This function trims a symbol by a given offset, allowing for only some parts of
// the symbol to be rendered based on horizontal scroll.
fn trim_symbol(symbol: &str, mut offset: usize) -> &str {
    let mut start = 0;
    for grapheme in UnicodeSegmentation::graphemes(symbol, true) {
        let grapheme_width = grapheme.width();
        if grapheme_width <= offset {
            offset -= grapheme_width;
            start += grapheme.len();
        } else {
            break;
        }
    }
    &symbol[start..]
}

// This function adjusts a symbol for horizontal scroll, based on whether it would be
// rendered outside of the left border of the widget.
fn adjust_symbol_for_horizontal_scroll<'a>(
    symbol: &'a str,
    remaining_scroll: &mut usize,
    alignment: Alignment,
) -> &'a str {
    let mut scrolled_symbol = "";

    // If the line is left-aligned and horizontal scroll is to be applied, skip symbols
    // until the horizontal scroll is reached.
    // Otherwise, just use the symbol without any modification.
    if alignment == Alignment::Left && *remaining_scroll > 0 {
        let symbol_width = symbol.width();

        // If the symbol is wider than the remaining horizontal scroll, this means that
        // it must be rendered, at least partially (the symbol is trimmed to fit).
        // If the symbol is not wider than the remaining horizontal scroll, this means that
        // it does not need to be rendered, so it is kept empty and the remaining scroll
        // is adjusted accordingly.
        if symbol_width > *remaining_scroll {
            scrolled_symbol = trim_symbol(symbol, *remaining_scroll);
            // If the symbol is being rendered, this means that scrolling has finished.
            *remaining_scroll = 0;
        } else {
            *remaining_scroll -= symbol_width;
        }
    } else {
        scrolled_symbol = symbol;
    }

    scrolled_symbol
}

// Determines if the given symbol is whitespace, except for non-breaking whitespace.
fn is_whitespace(symbol: &str) -> bool {
    symbol.chars().all(&char::is_whitespace) && symbol != NBSP
}

#[cfg(test)]
mod test {
    use unicode_segmentation::UnicodeSegmentation;

    use super::*;
    use crate::text::{Line, Text};

    fn widths(lines: &[Line]) -> Vec<usize> {
        lines
            .iter()
            .map(|line| line.width())
            .collect::<Vec<usize>>()
    }

    fn alignments(lines: &[Line]) -> Vec<Alignment> {
        lines
            .iter()
            .map(|line| line.alignment.unwrap_or(Alignment::Left))
            .collect::<Vec<Alignment>>()
    }

    #[test]
    fn wrap_one_line() {
        let width = 40;
        for i in 1..width {
            let line = Line::from("a".repeat(i));

            // let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
            // true);
            let char_wrapped =
                line.wrap_char_boundary(Style::default(), Alignment::Left, width, true);
            let truncated = line.wrap_truncate(Style::default(), Alignment::Left, width, 0);

            let expected = vec![line.clone().alignment(Alignment::Left)];

            assert_eq!(char_wrapped, expected);
            // assert_eq!(word_wrapped, expected);
            assert_eq!(truncated, expected);
        }
    }

    #[test]
    fn wrap_short_lines() {
        let width = 20;
        let source_text =
            "abcdefg\nhijklmno\npabcdefg\nhijklmn\nopabcdefghijk\nlmnopabcd\n\n\nefghijklmno";
        let text = Text::from(source_text);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, true);
        // let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
        // true);
        let truncated = text.wrap_truncate(Style::default(), Alignment::Left, width, 0);

        let expected: Vec<&str> = source_text.split('\n').collect();

        for (line, expected) in char_wrapped.iter().zip(expected.iter()) {
            assert_eq!(line, expected);
        }
        // for (line, expected) in word_wrapped.iter().zip(expected.iter()) {
        //     assert_eq!(line, expected);
        // }
        for (line, expected) in truncated.iter().zip(expected.iter()) {
            assert_eq!(line, expected);
        }
    }

    #[test]
    fn wrap_long_word() {
        let width = 20;
        let source_text = "abcdefghijklmnopabcdefghijklmnopabcdefghijklmnopabcdefghijklmno";
        let text = Text::from(source_text);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, true);
        // let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
        // true);
        let truncated = text.wrap_truncate(Style::default(), Alignment::Left, width, 0);

        let expected = vec![
            &source_text[..width],
            &source_text[width..width * 2],
            &source_text[width * 2..width * 3],
            &source_text[width * 3..],
        ];

        // assert_eq!(
        //     word_wrapped, wrapped,
        //     "CharWrapper should break the word at the line width limit."
        // );
        assert_eq!(
            char_wrapped, expected,
            "WordWrapper should detect the line cannot be broken on word boundary and \
             break it at line width limit."
        );
        assert_eq!(truncated, vec![&source_text[..width]]);
    }

    #[test]
    fn wrap_long_sentence() {
        let width = 20;
        let source_text_single_space =
            "abcd efghij klmnopabcd efgh ijklmnopabcdefg hijkl mnopab c d e f g h i j k l m n o";
        let source_text_multi_space =
            "abcd efghij    klmnopabcd efgh     ijklmnopabcdefg hijkl mnopab c d e f g h i j k l \
             m n o";
        let text_single_space = Text::from(source_text_single_space);
        let text_multi_space = Text::from(source_text_multi_space);

        let char_wrapped_single_space =
            text_single_space.wrap_char_boundary(Style::default(), Alignment::Left, width, true);
        let char_wrapped_multi_space =
            text_multi_space.wrap_char_boundary(Style::default(), Alignment::Left, width, true);
        // let word_wrapped_single_space =
        //     text_single_space.wrap_word_boundary(Style::default(), Alignment::Left, width, true);
        // let word_wrapped_multi_space =
        //     text_multi_space.wrap_word_boundary(Style::default(), Alignment::Left, width, true);
        let truncated =
            text_single_space.wrap_truncate(Style::default(), Alignment::Left, width, 0);

        let expected_char_single_space = vec![
            "abcd efghij klmnopab",
            "cd efgh ijklmnopabcd",
            "efg hijkl mnopab c d",
            "e f g h i j k l m n ",
            "o",
        ];
        let expected_char_multi_space = vec![
            "abcd efghij    klmno",
            "pabcd efgh     ijklm",
            "nopabcdefg hijkl mno",
            "pab c d e f g h i j ",
            "k l m n o",
        ];
        // Word wrapping should give the same result for multiple or single space due to trimming.
        let _expected_word_both = vec![
            "abcd efghij",
            "klmnopabcd efgh",
            "ijklmnopabcdefg",
            "hijkl mnopab c d e f",
            "g h i j k l m n o",
        ];

        assert_eq!(char_wrapped_single_space, expected_char_single_space);
        assert_eq!(char_wrapped_multi_space, expected_char_multi_space);
        // assert_eq!(word_wrapped_single_space, expected_word_both);
        // assert_eq!(word_wrapped_multi_space, expected_word_both);
        assert_eq!(truncated, vec![&source_text_single_space[..width]]);
    }

    #[test]
    fn wrap_zero_width() {
        let width = 0;
        let source_text = "abcd efghij klmnopabcd efgh ijklmnopabcdefg hijkl mnopab ";
        let text = Text::from(source_text);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, true);
        // let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
        // true);
        let truncated = text.wrap_truncate(Style::default(), Alignment::Left, width, 0);

        let expected: Vec<&str> = Vec::new();

        assert_eq!(char_wrapped, expected);
        // assert_eq!(word_wrapped, expected);
        assert_eq!(truncated, expected);
    }

    #[test]
    fn wrap_max_line_width_of_1() {
        let width = 1;
        let source_text = "abcd efghij klmnopabcd efgh ijklmnopabcdefg hijkl mnopab ";
        let text = Text::from(source_text);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, true);
        // let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
        // true);
        let truncated = text.wrap_truncate(Style::default(), Alignment::Left, width, 0);

        let expected: Vec<&str> = UnicodeSegmentation::graphemes(source_text, true)
            .filter(|g| g.chars().any(|c| !c.is_whitespace()))
            .collect();

        assert_eq!(char_wrapped, expected);
        // assert_eq!(word_wrapped, expected);
        assert_eq!(truncated, vec!["a"]);
    }

    #[test]
    fn wrap_max_line_width_of_1_double_width_characters() {
        let width = 1;
        let source_text =
            "コンピュータ上で文字を扱う場合、典型的には文字\naaa\naによる通信を行う場合にその\
                    両端点では、";
        let text = Text::from(source_text);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, true);
        // let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
        // true);
        let truncated = text.wrap_truncate(Style::default(), Alignment::Left, width, 0);

        assert_eq!(char_wrapped, vec!["", "a", "a", "a", "a"]);
        // assert_eq!(word_wrapped, vec!["", "a", "a", "a", "a"]);
        assert_eq!(truncated, vec!["", "a", "a"]);
    }

    /// Tests `CharWrapper` with words some of which exceed line length and some not.
    #[test]
    fn char_wrapper_mixed_length() {
        let width = 20;
        let source_text = "abcd efghij klmnopabcdefghijklmnopabcdefghijkl mnopab cdefghi j klmno";
        let text = Text::from(source_text);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, true);

        assert_eq!(
            char_wrapped,
            vec![
                "abcd efghij klmnopab",
                "cdefghijklmnopabcdef",
                "ghijkl mnopab cdefgh",
                "i j klmno",
            ]
        )
    }

    // /// Tests `WordWrapper` with words some of which exceed line length and some not.
    // #[test]
    // fn word_wrapper_mixed_length() {
    //     let width = 20;
    //     let source_text = "abcd efghij klmnopabcdefghijklmnopabcdefghijkl mnopab cdefghi j
    // klmno";     let text = Text::from(source_text);

    //     let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
    // true);

    //     assert_eq!(
    //         word_wrapped,
    //         vec![
    //             "abcd efghij",
    //             "klmnopabcdefghijklmn",
    //             "opabcdefghijkl",
    //             "mnopab cdefghi j",
    //             "klmno",
    //         ]
    //     )
    // }

    #[test]
    fn wrap_double_width_chars() {
        let width = 20;
        let source_text =
            "コンピュータ上で文字を扱う場合、典型的には文字による通信を行う場合にその両端点\
                    では、";
        let text = Text::from(source_text);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, true);
        // let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
        // true);
        let truncated = text.wrap_truncate(Style::default(), Alignment::Left, width, 0);

        let char_wrapped_widths = widths(&char_wrapped);
        // let word_wrapped_widths = withs(&word_wrapped);

        let expected_content = vec![
            "コンピュータ上で文字",
            "を扱う場合、典型的に",
            "は文字による通信を行",
            "う場合にその両端点で",
            "は、",
        ];
        let expected_widths = vec![width, width, width, width, 4];

        assert_eq!(char_wrapped, expected_content);
        // assert_eq!(word_wrapped, expected_content);
        assert_eq!(char_wrapped_widths, expected_widths);
        // assert_eq!(word_wrapped_widths, expected_widths);
        assert_eq!(truncated, vec!["コンピュータ上で文字"]);
    }

    #[test]
    fn wrap_leading_whitespace_removal() {
        let width = 20;
        let source_text = "AAAAAAAAAAAAAAAAAAAA    AAA";
        let text = Text::from(source_text);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, true);
        // let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
        // true);
        let truncated = text.wrap_truncate(Style::default(), Alignment::Left, width, 0);

        assert_eq!(char_wrapped, vec!["AAAAAAAAAAAAAAAAAAAA", "AAA",]);
        // assert_eq!(word_wrapped, vec!["AAAAAAAAAAAAAAAAAAAA", "AAA",]);
        assert_eq!(truncated, vec!["AAAAAAAAAAAAAAAAAAAA"]);
    }

    /// Tests truncation of leading whitespace.
    #[test]
    fn wrap_lots_of_spaces() {
        let width = 20;
        let source_text = "                                                                     ";
        let text = Text::from(source_text);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, true);
        // let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
        // true);
        let truncated = text.wrap_truncate(Style::default(), Alignment::Left, width, 0);

        assert_eq!(char_wrapped, vec![""]);
        // assert_eq!(word_wrapped, vec![""]);
        assert_eq!(truncated, vec!["                    "]);
    }

    /// Tests an input starting with a letter, followed by spaces - some of the behaviour is
    /// incidental.
    #[test]
    fn wrap_char_plus_lots_of_spaces() {
        let width = 20;
        let source_text = "a                                                                     ";
        let text = Text::from(source_text);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, true);
        // let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
        // true);
        let truncated = text.wrap_truncate(Style::default(), Alignment::Left, width, 0);

        // What's happening below is: the first line gets consumed, trailing spaces discarded,
        // after 20 of which a word break occurs (probably shouldn't). The second line break
        // discards all whitespace. The result should probably be vec!["a"] but it doesn't matter
        // that much.
        assert_eq!(char_wrapped, vec!["a                   "]);
        // assert_eq!(word_wrapped, vec!["a", ""]);
        assert_eq!(truncated, vec!["a                   "]);
    }

    #[test]
    fn wrap_double_width_chars_mixed_with_spaces() {
        let width = 20;
        // Japanese seems not to use spaces but we should break on spaces anyway... We're using it
        // to test double-width chars.
        // You are more than welcome to add word boundary detection based of alterations of
        // hiragana and katakana...
        // This happens to also be a test case for mixed width because regular spaces are single
        // width.
        let source_text = "コンピュ ータ上で文字を扱う場合、 典型的には文 字による 通信を行 う場合にその両端点では、";
        let text = Text::from(source_text);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, true);
        // let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
        // true);

        let char_wrapped_widths = widths(&char_wrapped);
        // let word_wrapped_widths = withs(&word_wrapped);

        assert_eq!(
            char_wrapped,
            vec![
                "コンピュ ータ上で文",
                "字を扱う場合、 典型",
                "的には文 字による 通",
                "信を行 う場合にその",
                "両端点では、",
            ]
        );
        // assert_eq!(
        //     word_wrapped,
        //     vec![
        //         "コンピュ",
        //         "ータ上で文字を扱う場",
        //         "合、 典型的には文",
        //         "字による 通信を行",
        //         "う場合にその両端点で",
        //         "は、",
        //     ]
        // );
        // Odd-sized lines have a space in them.
        assert_eq!(char_wrapped_widths, vec![19, 19, 20, 19, 12]);
        // assert_eq!(word_wrapped_widths, vec![8, 20, 17, 17, 20, 4]);
    }

    #[test]
    fn char_wrapper_preserve_indentation() {
        let width = 20;
        let source_text = "AAAAAAAAAAAAAAAAAAAA    AAA";
        let text = Text::from(source_text);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, false);

        assert_eq!(char_wrapped, vec!["AAAAAAAAAAAAAAAAAAAA", "    AAA",]);
    }

    #[test]
    fn char_wrapper_preserve_indentation_with_wrap() {
        let width = 10;
        let source_text = "AAA AAA AAAAA AA AAAAAA\n B\n  C\n   D";
        let text = Text::from(source_text);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, false);

        assert_eq!(
            char_wrapped,
            vec!["AAA AAA AA", "AAA AA AAA", "AAA", " B", "  C", "   D"]
        );
    }

    #[test]
    fn char_wrapper_preserve_indentation_lots_of_whitespace() {
        let width = 10;
        let source_text = "               4 Indent\n                 must wrap!";
        let text = Text::from(source_text);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, false);

        assert_eq!(
            char_wrapped,
            vec![
                "          ",
                "     4 Ind",
                "ent",
                "          ",
                "       mus",
                "t wrap!"
            ]
        );
    }

    // /// Ensure words separated by NBSP are wrapped as if they were a single one.
    // #[test]
    // fn word_wrapper_nbsp() {
    //     let width = 20;
    //     let source_text = "AAAAAAAAAAAAAAA AAAA\u{00a0}AAA";
    //     let text = Text::from(source_text);

    //     let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
    // true);     let word_wrapped_widths = widths(&word_wrapped);

    //     assert_eq!(word_wrapped, vec!["AAAAAAAAAAAAAAA", "AAAA\u{00a0}AAA",]);
    //     assert_eq!(word_wrapped_widths, vec![15, 8]);

    //     // Ensure that if the character was a regular space, it would be wrapped differently.
    //     let text_space = Text::from(source_text.replace('\u{00a0}', " "));

    //     let word_wrapped_space =
    //         text_space.wrap_word_boundary(Style::default(), Alignment::Left, width, true);
    //     let word_wrapped_space_widths = widths(&word_wrapped_space);

    //     assert_eq!(word_wrapped_space, vec!["AAAAAAAAAAAAAAA AAAA", "AAA",]);
    //     assert_eq!(word_wrapped_space_widths, vec![20, 3])
    // }

    // #[test]
    // fn word_wrapper_preserve_indentation() {
    //     let width = 20;
    //     let source_text = "AAAAAAAAAAAAAAAAAAAA    AAA";
    //     let text = Text::from(source_text);

    //     let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
    // false);

    //     assert_eq!(word_wrapped, vec!["AAAAAAAAAAAAAAAAAAAA", "   AAA",]);
    // }

    // #[test]
    // fn word_wrapper_preserve_indentation_with_wrap() {
    //     let width = 10;
    //     let source_text = "AAA AAA AAAAA AA AAAAAA\n B\n  C\n   D";
    //     let text = Text::from(source_text);

    //     let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
    // false);     assert_eq!(
    //         word_wrapped,
    //         vec!["AAA AAA", "AAAAA AA", "AAAAAA", " B", "  C", "   D"]
    //     );
    // }

    // #[test]
    // fn word_wrapper_preserve_indentation_lots_of_whitespace() {
    //     let width = 10;
    //     let source_text = "               4 Indent\n                 must wrap!";
    //     let text = Text::from(source_text);

    //     let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
    // false);

    //     assert_eq!(
    //         word_wrapped,
    //         vec![
    //             "          ",
    //             "    4",
    //             "Indent",
    //             "          ",
    //             "      must",
    //             "wrap!"
    //         ]
    //     );
    // }

    #[test]
    fn wrap_zero_width_at_end() {
        let width = 3;
        let line = "foo\0";
        let text = Text::from(line);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, true);
        // let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
        // true);
        let truncated = text.wrap_truncate(Style::default(), Alignment::Left, width, 0);

        assert_eq!(char_wrapped, vec!["foo\0"]);
        // assert_eq!(word_wrapped, vec!["foo\0"]);
        assert_eq!(truncated, vec!["foo\0"]);
    }

    #[test]
    fn wrap_preserves_line_alignment() {
        let width = 20;
        let text = Text::from(vec![
            Line::from("Something that is left aligned.").alignment(Alignment::Left),
            Line::from("This is right aligned and half short.").alignment(Alignment::Right),
            Line::from("This should sit in the center.").alignment(Alignment::Center),
        ]);

        let char_wrapped = text.wrap_char_boundary(Style::default(), Alignment::Left, width, true);
        // let word_wrapped = text.wrap_word_boundary(Style::default(), Alignment::Left, width,
        // true);
        let truncated = text.wrap_truncate(Style::default(), Alignment::Left, width, 0);

        let char_wrapped_alignments = alignments(&char_wrapped);
        // let word_wrapped_alignments = alignments(&word_wrapped);
        let truncated_alignments = alignments(&truncated);

        assert_eq!(
            char_wrapped_alignments,
            vec![
                Alignment::Left,
                Alignment::Left,
                Alignment::Right,
                Alignment::Right,
                Alignment::Center,
                Alignment::Center
            ]
        );
        // assert_eq!(
        //     word_wrapped_alignments,
        //     vec![
        //         Alignment::Left,
        //         Alignment::Left,
        //         Alignment::Right,
        //         Alignment::Right,
        //         Alignment::Right,
        //         Alignment::Center,
        //         Alignment::Center
        //     ]
        // );
        assert_eq!(
            truncated_alignments,
            vec![Alignment::Left, Alignment::Right, Alignment::Center]
        );
    }
}
