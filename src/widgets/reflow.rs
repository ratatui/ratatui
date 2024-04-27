use std::{collections::VecDeque, vec::IntoIter};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{layout::Alignment, text::StyledGrapheme};

const NBSP: &str = "\u{00a0}";
const ZWSP: &str = "\u{200b}";

/// A state machine to pack styled symbols into lines.
/// Cannot implement it as Iterator since it yields slices of the internal buffer (need streaming
/// iterators for that).
pub trait LineComposer<'a> {
    fn next_line<'lend>(&'lend mut self) -> Option<WrappedLine<'lend, 'a>>;
}

pub struct WrappedLine<'lend, 'text> {
    /// One line reflowed to the correct width
    pub line: &'lend [StyledGrapheme<'text>],
    /// The width of the line
    pub width: u16,
    /// Whether the line was aligned left or right
    pub alignment: Alignment,
}

/// A state machine that wraps lines on word boundaries.
#[derive(Debug, Default, Clone)]
pub struct WordWrapper<'a, O, I>
where
    // Outer iterator providing the individual lines
    O: Iterator<Item = (I, Alignment)>,
    // Inner iterator providing the styled symbols of a line Each line consists of an alignment and
    // a series of symbols
    I: Iterator<Item = StyledGrapheme<'a>>,
{
    /// The given, unprocessed lines
    input_lines: O,
    max_line_width: u16,
    wrapped_lines: Option<IntoIter<Vec<StyledGrapheme<'a>>>>,
    current_alignment: Alignment,
    current_line: Vec<StyledGrapheme<'a>>,
    /// Removes the leading whitespace from lines
    trim: bool,
}

impl<'a, O, I> WordWrapper<'a, O, I>
where
    O: Iterator<Item = (I, Alignment)>,
    I: Iterator<Item = StyledGrapheme<'a>>,
{
    pub fn new(lines: O, max_line_width: u16, trim: bool) -> Self {
        Self {
            input_lines: lines,
            max_line_width,
            wrapped_lines: None,
            current_alignment: Alignment::Left,
            current_line: vec![],
            trim,
        }
    }
}

impl<'a, O, I> LineComposer<'a> for WordWrapper<'a, O, I>
where
    O: Iterator<Item = (I, Alignment)>,
    I: Iterator<Item = StyledGrapheme<'a>>,
{
    #[allow(clippy::too_many_lines)]
    fn next_line<'lend>(&'lend mut self) -> Option<WrappedLine<'lend, 'a>> {
        if self.max_line_width == 0 {
            return None;
        }

        let mut current_line: Option<Vec<StyledGrapheme<'a>>> = None;
        let mut line_width: u16 = 0;

        // Try to repeatedly retrieve next line
        while current_line.is_none() {
            // Retrieve next preprocessed wrapped line
            if let Some(line_iterator) = &mut self.wrapped_lines {
                if let Some(line) = line_iterator.next() {
                    line_width = line
                        .iter()
                        .map(|grapheme| grapheme.symbol.width())
                        .sum::<usize>() as u16;
                    current_line = Some(line);
                }
            }

            // When no more preprocessed wrapped lines
            if current_line.is_none() {
                // Try to calculate next wrapped lines based on current whole line
                if let Some((line_symbols, line_alignment)) = &mut self.input_lines.next() {
                    // Save the whole line's alignment
                    self.current_alignment = *line_alignment;
                    let mut wrapped_lines = vec![]; // Saves the wrapped lines
                                                    // Saves the unfinished wrapped line
                    let (mut current_line, mut current_line_width) = (vec![], 0);
                    // Saves the partially processed word
                    let (mut unfinished_word, mut word_width) = (vec![], 0);
                    // Saves the whitespaces of the partially unfinished word
                    let (mut unfinished_whitespaces, mut whitespace_width) =
                        (VecDeque::<StyledGrapheme>::new(), 0);

                    let mut has_seen_non_whitespace = false;
                    for StyledGrapheme { symbol, style } in line_symbols {
                        let symbol_whitespace = symbol == ZWSP
                            || (symbol.chars().all(&char::is_whitespace) && symbol != NBSP);
                        let symbol_width = symbol.width() as u16;
                        // Ignore characters wider than the total max width
                        if symbol_width > self.max_line_width {
                            continue;
                        }

                        // Append finished word to current line
                        if has_seen_non_whitespace && symbol_whitespace
                            // Append if trimmed (whitespaces removed) word would overflow
                            || word_width + symbol_width > self.max_line_width && current_line.is_empty() && self.trim
                            // Append if removed whitespace would overflow -> reset whitespace counting to prevent overflow
                            || whitespace_width + symbol_width > self.max_line_width && current_line.is_empty() && self.trim
                            // Append if complete word would overflow
                            || word_width + whitespace_width + symbol_width > self.max_line_width && current_line.is_empty() && !self.trim
                        {
                            if !current_line.is_empty() || !self.trim {
                                // Also append whitespaces if not trimming or current line is not
                                // empty
                                current_line.extend(
                                    std::mem::take(&mut unfinished_whitespaces).into_iter(),
                                );
                                current_line_width += whitespace_width;
                            }
                            // Append trimmed word
                            current_line.append(&mut unfinished_word);
                            current_line_width += word_width;

                            // Clear whitespace buffer
                            unfinished_whitespaces.clear();
                            whitespace_width = 0;
                            word_width = 0;
                        }

                        // Append the unfinished wrapped line to wrapped lines if it is as wide as
                        // max line width
                        if current_line_width >= self.max_line_width
                            // or if it would be too long with the current partially processed word added
                            || current_line_width + whitespace_width + word_width >= self.max_line_width && symbol_width > 0
                        {
                            let mut remaining_width = (i32::from(self.max_line_width)
                                - i32::from(current_line_width))
                            .max(0) as u16;
                            wrapped_lines.push(std::mem::take(&mut current_line));
                            current_line_width = 0;

                            // Remove all whitespaces till end of just appended wrapped line + next
                            // whitespace
                            let mut first_whitespace = unfinished_whitespaces.pop_front();
                            while let Some(grapheme) = first_whitespace.as_ref() {
                                let symbol_width = grapheme.symbol.width() as u16;
                                whitespace_width -= symbol_width;

                                if symbol_width > remaining_width {
                                    break;
                                }
                                remaining_width -= symbol_width;
                                first_whitespace = unfinished_whitespaces.pop_front();
                            }
                            // In case all whitespaces have been exhausted
                            if symbol_whitespace && first_whitespace.is_none() {
                                // Prevent first whitespace to count towards next word
                                continue;
                            }
                        }

                        // Append symbol to unfinished, partially processed word
                        if symbol_whitespace {
                            whitespace_width += symbol_width;
                            unfinished_whitespaces.push_back(StyledGrapheme { symbol, style });
                        } else {
                            word_width += symbol_width;
                            unfinished_word.push(StyledGrapheme { symbol, style });
                        }

                        has_seen_non_whitespace = !symbol_whitespace;
                    }

                    // Append remaining text parts
                    if !unfinished_word.is_empty() || !unfinished_whitespaces.is_empty() {
                        if current_line.is_empty() && unfinished_word.is_empty() {
                            wrapped_lines.push(vec![]);
                        } else if !self.trim || !current_line.is_empty() {
                            current_line.extend(unfinished_whitespaces.into_iter());
                        } else {
                            // TODO: explain why this else branch is ok.
                            // See clippy::else_if_without_else
                        }
                        current_line.append(&mut unfinished_word);
                    }
                    if !current_line.is_empty() {
                        wrapped_lines.push(current_line);
                    }
                    if wrapped_lines.is_empty() {
                        // Append empty line if there was nothing to wrap in the first place
                        wrapped_lines.push(vec![]);
                    }

                    self.wrapped_lines = Some(wrapped_lines.into_iter());
                } else {
                    // No more whole lines available -> stop repeatedly retrieving next wrapped line
                    break;
                }
            }
        }

        if let Some(line) = current_line {
            self.current_line = line;
            Some(WrappedLine {
                line: &self.current_line,
                width: line_width,
                alignment: self.current_alignment,
            })
        } else {
            None
        }
    }
}

/// A state machine that truncates overhanging lines.
#[derive(Debug, Default, Clone)]
pub struct LineTruncator<'a, O, I>
where
    // Outer iterator providing the individual lines
    O: Iterator<Item = (I, Alignment)>,
    // Inner iterator providing the styled symbols of a line Each line consists of an alignment and
    // a series of symbols
    I: Iterator<Item = StyledGrapheme<'a>>,
{
    /// The given, unprocessed lines
    input_lines: O,
    max_line_width: u16,
    current_line: Vec<StyledGrapheme<'a>>,
    /// Record the offset to skip render
    horizontal_offset: u16,
}

impl<'a, O, I> LineTruncator<'a, O, I>
where
    O: Iterator<Item = (I, Alignment)>,
    I: Iterator<Item = StyledGrapheme<'a>>,
{
    pub fn new(lines: O, max_line_width: u16) -> Self {
        Self {
            input_lines: lines,
            max_line_width,
            horizontal_offset: 0,
            current_line: vec![],
        }
    }

    pub fn set_horizontal_offset(&mut self, horizontal_offset: u16) {
        self.horizontal_offset = horizontal_offset;
    }
}

impl<'a, O, I> LineComposer<'a> for LineTruncator<'a, O, I>
where
    O: Iterator<Item = (I, Alignment)>,
    I: Iterator<Item = StyledGrapheme<'a>>,
{
    fn next_line<'lend>(&'lend mut self) -> Option<WrappedLine<'lend, 'a>> {
        if self.max_line_width == 0 {
            return None;
        }

        self.current_line.truncate(0);
        let mut current_line_width = 0;

        let mut lines_exhausted = true;
        let mut horizontal_offset = self.horizontal_offset as usize;
        let mut current_alignment = Alignment::Left;
        if let Some((current_line, alignment)) = &mut self.input_lines.next() {
            lines_exhausted = false;
            current_alignment = *alignment;

            for StyledGrapheme { symbol, style } in current_line {
                // Ignore characters wider that the total max width.
                if symbol.width() as u16 > self.max_line_width {
                    continue;
                }

                if current_line_width + symbol.width() as u16 > self.max_line_width {
                    // Truncate line
                    break;
                }

                let symbol = if horizontal_offset == 0 || Alignment::Left != *alignment {
                    symbol
                } else {
                    let w = symbol.width();
                    if w > horizontal_offset {
                        let t = trim_offset(symbol, horizontal_offset);
                        horizontal_offset = 0;
                        t
                    } else {
                        horizontal_offset -= w;
                        ""
                    }
                };
                current_line_width += symbol.width() as u16;
                self.current_line.push(StyledGrapheme { symbol, style });
            }
        }

        if lines_exhausted {
            None
        } else {
            Some(WrappedLine {
                line: &self.current_line,
                width: current_line_width,
                alignment: current_alignment,
            })
        }
    }
}

/// This function will return a str slice which start at specified offset.
/// As src is a unicode str, start offset has to be calculated with each character.
fn trim_offset(src: &str, mut offset: usize) -> &str {
    let mut start = 0;
    for c in UnicodeSegmentation::graphemes(src, true) {
        let w = c.width();
        if w <= offset {
            offset -= w;
            start += c.len();
        } else {
            break;
        }
    }
    &src[start..]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        style::Style,
        text::{Line, Text},
    };

    #[derive(Clone, Copy)]
    enum Composer {
        WordWrapper { trim: bool },
        LineTruncator,
    }

    fn run_composer<'a>(
        which: Composer,
        text: impl Into<Text<'a>>,
        text_area_width: u16,
    ) -> (Vec<String>, Vec<u16>, Vec<Alignment>) {
        let text = text.into();
        let styled_lines = text.iter().map(|line| {
            (
                line.iter()
                    .flat_map(|span| span.styled_graphemes(Style::default())),
                line.alignment.unwrap_or(Alignment::Left),
            )
        });

        let mut composer: Box<dyn LineComposer> = match which {
            Composer::WordWrapper { trim } => {
                Box::new(WordWrapper::new(styled_lines, text_area_width, trim))
            }
            Composer::LineTruncator => Box::new(LineTruncator::new(styled_lines, text_area_width)),
        };
        let mut lines = vec![];
        let mut widths = vec![];
        let mut alignments = vec![];
        while let Some(WrappedLine {
            line: styled,
            width,
            alignment,
        }) = composer.next_line()
        {
            let line = styled
                .iter()
                .map(|StyledGrapheme { symbol, .. }| *symbol)
                .collect::<String>();
            assert!(width <= text_area_width);
            lines.push(line);
            widths.push(width);
            alignments.push(alignment);
        }
        (lines, widths, alignments)
    }

    #[test]
    fn line_composer_one_line() {
        let width = 40;
        for i in 1..width {
            let text = "a".repeat(i);
            let (word_wrapper, _, _) =
                run_composer(Composer::WordWrapper { trim: true }, &*text, width as u16);
            let (line_truncator, _, _) =
                run_composer(Composer::LineTruncator, &*text, width as u16);
            let expected = vec![text];
            assert_eq!(word_wrapper, expected);
            assert_eq!(line_truncator, expected);
        }
    }

    #[test]
    fn line_composer_short_lines() {
        let width = 20;
        let text =
            "abcdefg\nhijklmno\npabcdefg\nhijklmn\nopabcdefghijk\nlmnopabcd\n\n\nefghijklmno";
        let (word_wrapper, _, _) = run_composer(Composer::WordWrapper { trim: true }, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, text, width);

        let wrapped: Vec<&str> = text.split('\n').collect();
        assert_eq!(word_wrapper, wrapped);
        assert_eq!(line_truncator, wrapped);
    }

    #[test]
    fn line_composer_long_word() {
        let width = 20;
        let text = "abcdefghijklmnopabcdefghijklmnopabcdefghijklmnopabcdefghijklmno";
        let (word_wrapper, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, text, width as u16);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, text, width as u16);

        let wrapped = vec![
            &text[..width],
            &text[width..width * 2],
            &text[width * 2..width * 3],
            &text[width * 3..],
        ];
        assert_eq!(
            word_wrapper, wrapped,
            "WordWrapper should detect the line cannot be broken on word boundary and \
             break it at line width limit."
        );
        assert_eq!(line_truncator, vec![&text[..width]]);
    }

    #[test]
    fn line_composer_long_sentence() {
        let width = 20;
        let text =
            "abcd efghij klmnopabcd efgh ijklmnopabcdefg hijkl mnopab c d e f g h i j k l m n o";
        let text_multi_space =
            "abcd efghij    klmnopabcd efgh     ijklmnopabcdefg hijkl mnopab c d e f g h i j k l \
             m n o";
        let (word_wrapper_single_space, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, text, width as u16);
        let (word_wrapper_multi_space, _, _) = run_composer(
            Composer::WordWrapper { trim: true },
            text_multi_space,
            width as u16,
        );
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, text, width as u16);

        let word_wrapped = vec![
            "abcd efghij",
            "klmnopabcd efgh",
            "ijklmnopabcdefg",
            "hijkl mnopab c d e f",
            "g h i j k l m n o",
        ];
        assert_eq!(word_wrapper_single_space, word_wrapped);
        assert_eq!(word_wrapper_multi_space, word_wrapped);

        assert_eq!(line_truncator, vec![&text[..width]]);
    }

    #[test]
    fn line_composer_zero_width() {
        let width = 0;
        let text = "abcd efghij klmnopabcd efgh ijklmnopabcdefg hijkl mnopab ";
        let (word_wrapper, _, _) = run_composer(Composer::WordWrapper { trim: true }, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, text, width);

        let expected: Vec<&str> = Vec::new();
        assert_eq!(word_wrapper, expected);
        assert_eq!(line_truncator, expected);
    }

    #[test]
    fn line_composer_max_line_width_of_1() {
        let width = 1;
        let text = "abcd efghij klmnopabcd efgh ijklmnopabcdefg hijkl mnopab ";
        let (word_wrapper, _, _) = run_composer(Composer::WordWrapper { trim: true }, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, text, width);

        let expected: Vec<&str> = UnicodeSegmentation::graphemes(text, true)
            .filter(|g| g.chars().any(|c| !c.is_whitespace()))
            .collect();
        assert_eq!(word_wrapper, expected);
        assert_eq!(line_truncator, vec!["a"]);
    }

    #[test]
    fn line_composer_max_line_width_of_1_double_width_characters() {
        let width = 1;
        let text =
            "コンピュータ上で文字を扱う場合、典型的には文字\naaa\naによる通信を行う場合にその\
                    両端点では、";
        let (word_wrapper, _, _) = run_composer(Composer::WordWrapper { trim: true }, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, text, width);
        assert_eq!(word_wrapper, vec!["", "a", "a", "a", "a"]);
        assert_eq!(line_truncator, vec!["", "a", "a"]);
    }

    /// Tests `WordWrapper` with words some of which exceed line length and some not.
    #[test]
    fn line_composer_word_wrapper_mixed_length() {
        let width = 20;
        let text = "abcd efghij klmnopabcdefghijklmnopabcdefghijkl mnopab cdefghi j klmno";
        let (word_wrapper, _, _) = run_composer(Composer::WordWrapper { trim: true }, text, width);
        assert_eq!(
            word_wrapper,
            vec![
                "abcd efghij",
                "klmnopabcdefghijklmn",
                "opabcdefghijkl",
                "mnopab cdefghi j",
                "klmno",
            ]
        );
    }

    #[test]
    fn line_composer_double_width_chars() {
        let width = 20;
        let text = "コンピュータ上で文字を扱う場合、典型的には文字による通信を行う場合にその両端点\
                    では、";
        let (word_wrapper, word_wrapper_width, _) =
            run_composer(Composer::WordWrapper { trim: true }, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, text, width);
        assert_eq!(line_truncator, vec!["コンピュータ上で文字"]);
        let wrapped = vec![
            "コンピュータ上で文字",
            "を扱う場合、典型的に",
            "は文字による通信を行",
            "う場合にその両端点で",
            "は、",
        ];
        assert_eq!(word_wrapper, wrapped);
        assert_eq!(word_wrapper_width, vec![width, width, width, width, 4]);
    }

    #[test]
    fn line_composer_leading_whitespace_removal() {
        let width = 20;
        let text = "AAAAAAAAAAAAAAAAAAAA    AAA";
        let (word_wrapper, _, _) = run_composer(Composer::WordWrapper { trim: true }, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, text, width);
        assert_eq!(word_wrapper, vec!["AAAAAAAAAAAAAAAAAAAA", "AAA",]);
        assert_eq!(line_truncator, vec!["AAAAAAAAAAAAAAAAAAAA"]);
    }

    /// Tests truncation of leading whitespace.
    #[test]
    fn line_composer_lots_of_spaces() {
        let width = 20;
        let text = "                                                                     ";
        let (word_wrapper, _, _) = run_composer(Composer::WordWrapper { trim: true }, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, text, width);
        assert_eq!(word_wrapper, vec![""]);
        assert_eq!(line_truncator, vec!["                    "]);
    }

    /// Tests an input starting with a letter, followed by spaces - some of the behaviour is
    /// incidental.
    #[test]
    fn line_composer_char_plus_lots_of_spaces() {
        let width = 20;
        let text = "a                                                                     ";
        let (word_wrapper, _, _) = run_composer(Composer::WordWrapper { trim: true }, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, text, width);
        // What's happening below is: the first line gets consumed, trailing spaces discarded,
        // after 20 of which a word break occurs (probably shouldn't). The second line break
        // discards all whitespace. The result should probably be vec!["a"] but it doesn't matter
        // that much.
        assert_eq!(word_wrapper, vec!["a", ""]);
        assert_eq!(line_truncator, vec!["a                   "]);
    }

    #[test]
    fn line_composer_word_wrapper_double_width_chars_mixed_with_spaces() {
        let width = 20;
        // Japanese seems not to use spaces but we should break on spaces anyway... We're using it
        // to test double-width chars.
        // You are more than welcome to add word boundary detection based of alterations of
        // hiragana and katakana...
        // This happens to also be a test case for mixed width because regular spaces are single
        // width.
        let text = "コンピュ ータ上で文字を扱う場合、 典型的には文 字による 通信を行 う場合にその両端点では、";
        let (word_wrapper, word_wrapper_width, _) =
            run_composer(Composer::WordWrapper { trim: true }, text, width);
        assert_eq!(
            word_wrapper,
            vec![
                "コンピュ",
                "ータ上で文字を扱う場",
                "合、 典型的には文",
                "字による 通信を行",
                "う場合にその両端点で",
                "は、",
            ]
        );
        // Odd-sized lines have a space in them.
        assert_eq!(word_wrapper_width, vec![8, 20, 17, 17, 20, 4]);
    }

    /// Ensure words separated by nbsp are wrapped as if they were a single one.
    #[test]
    fn line_composer_word_wrapper_nbsp() {
        let width = 20;
        let text = "AAAAAAAAAAAAAAA AAAA\u{00a0}AAA";
        let (word_wrapper, word_wrapper_widths, _) =
            run_composer(Composer::WordWrapper { trim: true }, text, width);
        assert_eq!(word_wrapper, vec!["AAAAAAAAAAAAAAA", "AAAA\u{00a0}AAA",]);
        assert_eq!(word_wrapper_widths, vec![15, 8]);

        // Ensure that if the character was a regular space, it would be wrapped differently.
        let text_space = text.replace('\u{00a0}', " ");
        let (word_wrapper_space, word_wrapper_widths, _) =
            run_composer(Composer::WordWrapper { trim: true }, text_space, width);
        assert_eq!(word_wrapper_space, vec!["AAAAAAAAAAAAAAA AAAA", "AAA",]);
        assert_eq!(word_wrapper_widths, vec![20, 3]);
    }

    #[test]
    fn line_composer_word_wrapper_preserve_indentation() {
        let width = 20;
        let text = "AAAAAAAAAAAAAAAAAAAA    AAA";
        let (word_wrapper, _, _) = run_composer(Composer::WordWrapper { trim: false }, text, width);
        assert_eq!(word_wrapper, vec!["AAAAAAAAAAAAAAAAAAAA", "   AAA",]);
    }

    #[test]
    fn line_composer_word_wrapper_preserve_indentation_with_wrap() {
        let width = 10;
        let text = "AAA AAA AAAAA AA AAAAAA\n B\n  C\n   D";
        let (word_wrapper, _, _) = run_composer(Composer::WordWrapper { trim: false }, text, width);
        assert_eq!(
            word_wrapper,
            vec!["AAA AAA", "AAAAA AA", "AAAAAA", " B", "  C", "   D"]
        );
    }

    #[test]
    fn line_composer_word_wrapper_preserve_indentation_lots_of_whitespace() {
        let width = 10;
        let text = "               4 Indent\n                 must wrap!";
        let (word_wrapper, _, _) = run_composer(Composer::WordWrapper { trim: false }, text, width);
        assert_eq!(
            word_wrapper,
            vec![
                "          ",
                "    4",
                "Indent",
                "          ",
                "      must",
                "wrap!"
            ]
        );
    }

    #[test]
    fn line_composer_zero_width_at_end() {
        let width = 3;
        let line = "foo\0";
        let (word_wrapper, _, _) = run_composer(Composer::WordWrapper { trim: true }, line, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, line, width);
        assert_eq!(word_wrapper, vec!["foo\0"]);
        assert_eq!(line_truncator, vec!["foo\0"]);
    }

    #[test]
    fn line_composer_preserves_line_alignment() {
        let width = 20;
        let lines = vec![
            Line::from("Something that is left aligned.").alignment(Alignment::Left),
            Line::from("This is right aligned and half short.").alignment(Alignment::Right),
            Line::from("This should sit in the center.").alignment(Alignment::Center),
        ];
        let (_, _, wrapped_alignments) =
            run_composer(Composer::WordWrapper { trim: true }, lines.clone(), width);
        let (_, _, truncated_alignments) = run_composer(Composer::LineTruncator, lines, width);
        assert_eq!(
            wrapped_alignments,
            vec![
                Alignment::Left,
                Alignment::Left,
                Alignment::Right,
                Alignment::Right,
                Alignment::Right,
                Alignment::Center,
                Alignment::Center
            ]
        );
        assert_eq!(
            truncated_alignments,
            vec![Alignment::Left, Alignment::Right, Alignment::Center]
        );
    }

    #[test]
    fn line_composer_zero_width_white_space() {
        let width = 3;
        let line = "foo\u{200b}bar";
        let (word_wrapper, _, _) = run_composer(Composer::WordWrapper { trim: true }, line, width);
        assert_eq!(word_wrapper, vec!["foo", "bar"]);
    }
}
