use std::collections::VecDeque;
use std::vec::{IntoIter, Vec};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::layout::Alignment;
use crate::text::StyledGrapheme;

const NBSP: &str = "\u{00a0}";

/// A state machine to pack styled symbols into lines.
/// Cannot implement it as Iterator since it yields slices of the internal buffer (need streaming
/// iterators for that).
pub trait LineComposer<'a> {
    fn next_line(&mut self) -> Option<(&[StyledGrapheme<'a>], u16, Alignment)>;

    /// Like [`std::iter::Iterator::advance_by`].
    #[inline]
    fn advance_by(&mut self, n: usize) -> Result<(), usize> {
        for i in 0..n {
            self.next_line().ok_or(i)?;
        }
        Ok(())
    }
}

/// A [`LineComposer`] that can be consumed from either end.
pub trait DoubleEndedLineComposer<'a>: LineComposer<'a> {
    fn next_line_back(&mut self) -> Option<(&[StyledGrapheme<'a>], u16, Alignment)>;

    #[must_use]
    #[inline]
    fn rev(self) -> RevLineComposer<Self>
    where
        Self: Sized,
    {
        RevLineComposer { inner: self }
    }
}

/// A state machine that wraps lines on word boundaries.
pub struct WordWrapper<'a, O, I>
where
    O: Iterator<Item = (I, Alignment)>, // Outer iterator providing the individual lines
    I: Iterator<Item = StyledGrapheme<'a>>, // Inner iterator providing the styled symbols of a line
                                        // Each line consists of an alignment and a series of symbols
{
    /// The given, unprocessed lines
    input_lines: O,
    max_line_width: u16,
    /// Removes the leading whitespace from lines
    trim: bool,
    front: Option<(IntoIter<Vec<StyledGrapheme<'a>>>, Alignment)>,
    back: Option<(IntoIter<Vec<StyledGrapheme<'a>>>, Alignment)>,
    current: Vec<StyledGrapheme<'a>>,
}

impl<'a, O, I> WordWrapper<'a, O, I>
where
    O: Iterator<Item = (I, Alignment)>,
    I: Iterator<Item = StyledGrapheme<'a>>,
{
    #[must_use]
    #[inline]
    pub fn new(lines: O, max_line_width: u16, trim: bool) -> WordWrapper<'a, O, I> {
        WordWrapper {
            input_lines: lines,
            max_line_width,
            trim,
            front: None,
            back: None,
            current: vec![],
        }
    }
}

fn width(line: &[StyledGrapheme<'_>]) -> u16 {
    line.iter()
        .map(|grapheme| grapheme.symbol.width())
        .sum::<usize>() as u16
}

impl<'a, O, I> WordWrapper<'a, O, I>
where
    O: Iterator<Item = (I, Alignment)>,
    I: Iterator<Item = StyledGrapheme<'a>>,
{
    fn wrap(&mut self, line_symbols: I) -> IntoIter<Vec<StyledGrapheme<'a>>> {
        let mut wrapped_lines = vec![]; // Saves the wrapped lines
        let (mut current_line, mut current_line_width) = (vec![], 0); // Saves the unfinished wrapped line
        let (mut unfinished_word, mut word_width) = (vec![], 0); // Saves the partially processed word
        let (mut unfinished_whitespaces, mut whitespace_width) =
            (VecDeque::<StyledGrapheme>::new(), 0); // Saves the whitespaces of the partially unfinished word

        let mut has_seen_non_whitespace = false;
        for StyledGrapheme { symbol, style } in line_symbols {
            let symbol_whitespace = symbol.chars().all(&char::is_whitespace) && symbol != NBSP;
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
                    // Also append whitespaces if not trimming or current line is not empty
                    current_line.extend(std::mem::take(&mut unfinished_whitespaces).into_iter());
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

            // Append the unfinished wrapped line to wrapped lines if it is as wide as max line width
            if current_line_width >= self.max_line_width
                // or if it would be too long with the current partially processed word added
                || current_line_width + whitespace_width + word_width >= self.max_line_width && symbol_width > 0
            {
                let mut remaining_width =
                    (self.max_line_width as i32 - current_line_width as i32).max(0) as u16;
                wrapped_lines.push(std::mem::take(&mut current_line));
                current_line_width = 0;

                // Remove all whitespaces till end of just appended wrapped line + next whitespace
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

        wrapped_lines.into_iter()
    }
}

impl<'a, O, I> LineComposer<'a> for WordWrapper<'a, O, I>
where
    O: Iterator<Item = (I, Alignment)>,
    I: Iterator<Item = StyledGrapheme<'a>>,
{
    fn next_line(&mut self) -> Option<(&[StyledGrapheme<'a>], u16, Alignment)> {
        if self.max_line_width == 0 {
            return None;
        }

        let mut current_line: Option<(Vec<StyledGrapheme<'a>>, Alignment)> = None;

        loop {
            if let Some((line, &mut align)) = self
                .front
                .as_mut()
                .and_then(|(ref mut f, align)| f.next().map(|l| (l, align)))
            {
                current_line = Some((line, align));
                break;
            } else if let Some((unwrapped, align)) = self.input_lines.next() {
                self.front = Some((self.wrap(unwrapped), align));
            } else {
                break;
            }
        }

        if current_line.is_none() {
            if let Some((line, &mut align)) = self
                .back
                .as_mut()
                .and_then(|(ref mut f, align)| f.next().map(|l| (l, align)))
            {
                current_line = Some((line, align));
            }
        }

        if let Some((line, align)) = current_line {
            self.current = line;
            Some((&self.current, width(&self.current), align))
        } else {
            None
        }
    }
}

impl<'a, O, I> DoubleEndedLineComposer<'a> for WordWrapper<'a, O, I>
where
    O: DoubleEndedIterator<Item = (I, Alignment)>,
    I: Iterator<Item = StyledGrapheme<'a>>,
{
    fn next_line_back(&mut self) -> Option<(&[StyledGrapheme<'a>], u16, Alignment)> {
        if self.max_line_width == 0 {
            return None;
        }

        let mut current_line: Option<(Vec<StyledGrapheme<'a>>, Alignment)> = None;

        loop {
            if let Some((line, &mut align)) = self
                .back
                .as_mut()
                .and_then(|(ref mut f, align)| f.next_back().map(|l| (l, align)))
            {
                current_line = Some((line, align));
                break;
            } else if let Some((unwrapped, align)) = self.input_lines.next_back() {
                self.back = Some((self.wrap(unwrapped), align));
            } else {
                break;
            }
        }

        if current_line.is_none() {
            if let Some((line, &mut align)) = self
                .front
                .as_mut()
                .and_then(|(ref mut f, align)| f.next_back().map(|l| (l, align)))
            {
                current_line = Some((line, align));
            }
        }

        if let Some((line, align)) = current_line {
            self.current = line;
            Some((&self.current, width(&self.current), align))
        } else {
            None
        }
    }
}

/// A state machine that truncates overhanging lines.
pub struct LineTruncator<'a, O, I>
where
    O: Iterator<Item = (I, Alignment)>, // Outer iterator providing the individual lines
    I: Iterator<Item = StyledGrapheme<'a>>, // Inner iterator providing the styled symbols of a line
                                        // Each line consists of an alignment and a series of symbols
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
    pub fn new(lines: O, max_line_width: u16) -> LineTruncator<'a, O, I> {
        LineTruncator {
            input_lines: lines,
            max_line_width,
            horizontal_offset: 0,
            current_line: vec![],
        }
    }

    pub fn set_horizontal_offset(&mut self, horizontal_offset: u16) {
        self.horizontal_offset = horizontal_offset;
    }

    fn truncate(&mut self, line: &mut I, align: Alignment) -> u16 {
        self.current_line.clear();
        let mut current_line_width = 0;
        let mut horizontal_offset: usize = self.horizontal_offset as usize;

        for StyledGrapheme { symbol, style } in line {
            // Ignore characters wider that the total max width.
            if symbol.width() as u16 > self.max_line_width {
                continue;
            }

            if current_line_width + symbol.width() as u16 > self.max_line_width {
                // Truncate line
                break;
            }

            let symbol = if horizontal_offset == 0 || Alignment::Left != align {
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
        current_line_width
    }
}

impl<'a, O, I> LineComposer<'a> for LineTruncator<'a, O, I>
where
    O: Iterator<Item = (I, Alignment)>,
    I: Iterator<Item = StyledGrapheme<'a>>,
{
    fn next_line(&mut self) -> Option<(&[StyledGrapheme<'a>], u16, Alignment)> {
        if self.max_line_width == 0 {
            return None;
        }

        if let Some((current_line, align)) = &mut self.input_lines.next() {
            let width = self.truncate(current_line, *align);
            Some((&self.current_line, width, *align))
        } else {
            None
        }
    }
}

impl<'a, O, I> DoubleEndedLineComposer<'a> for LineTruncator<'a, O, I>
where
    O: DoubleEndedIterator<Item = (I, Alignment)>,
    I: Iterator<Item = StyledGrapheme<'a>>,
{
    fn next_line_back(&mut self) -> Option<(&[StyledGrapheme<'a>], u16, Alignment)> {
        if self.max_line_width == 0 {
            return None;
        }

        if let Some((current_line, align)) = &mut self.input_lines.next_back() {
            let width = self.truncate(current_line, *align);
            Some((&self.current_line, width, *align))
        } else {
            None
        }
    }
}

/// A [`DoubleEndedLineComposer`] that reverses the order of the lines
/// of the inner composer.
pub struct RevLineComposer<I> {
    inner: I,
}

impl<'a, I> LineComposer<'a> for RevLineComposer<I>
where
    I: DoubleEndedLineComposer<'a>,
{
    #[inline]
    fn next_line(&mut self) -> Option<(&[StyledGrapheme<'a>], u16, Alignment)> {
        self.inner.next_line_back()
    }
}

impl<'a, I> DoubleEndedLineComposer<'a> for RevLineComposer<I>
where
    I: DoubleEndedLineComposer<'a>,
{
    #[inline]
    fn next_line_back(&mut self) -> Option<(&[StyledGrapheme<'a>], u16, Alignment)> {
        self.inner.next_line()
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
    use crate::style::Style;
    use crate::text::{Line, Text};
    use unicode_segmentation::UnicodeSegmentation;

    enum Composer {
        WordWrapper { trim: bool },
        LineTruncator,
    }

    fn run_composer<'a>(
        which: Composer,
        rev: bool,
        text: impl Into<Text<'a>>,
        text_area_width: u16,
    ) -> (Vec<String>, Vec<u16>, Vec<Alignment>) {
        let text = text.into();
        let styled_lines = text.lines.iter().map(|line| {
            (
                line.spans
                    .iter()
                    .flat_map(|span| span.styled_graphemes(Style::default())),
                line.alignment.unwrap_or(Alignment::Left),
            )
        });

        let mut composer: Box<dyn LineComposer> = match which {
            Composer::WordWrapper { trim } => {
                let wrapper = WordWrapper::new(styled_lines, text_area_width, trim);
                if rev {
                    Box::new(wrapper.rev())
                } else {
                    Box::new(wrapper)
                }
            }
            Composer::LineTruncator => {
                let truncator = LineTruncator::new(styled_lines, text_area_width);
                if rev {
                    Box::new(truncator.rev())
                } else {
                    Box::new(truncator)
                }
            }
        };
        let mut lines = vec![];
        let mut widths = vec![];
        let mut alignments = vec![];
        while let Some((styled, width, alignment)) = composer.next_line() {
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
            let (word_wrapper, _, _) = run_composer(
                Composer::WordWrapper { trim: true },
                false,
                &text[..],
                width as u16,
            );
            let (line_truncator, _, _) =
                run_composer(Composer::LineTruncator, false, &text[..], width as u16);
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
        let (word_wrapper, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, false, text, width);
        let (word_wrapper_rev, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, true, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, false, text, width);
        let (line_truncator_rev, _, _) = run_composer(Composer::LineTruncator, true, text, width);

        let wrapped: Vec<&str> = text.split('\n').collect();
        let wrapped_rev: Vec<&str> = text.split('\n').rev().collect();
        assert_eq!(word_wrapper, wrapped);
        assert_eq!(word_wrapper_rev, wrapped_rev);
        assert_eq!(line_truncator, wrapped);
        assert_eq!(line_truncator_rev, wrapped_rev);
    }

    #[test]
    fn line_composer_long_word() {
        let width = 20;
        let text = "abcdefghijklmnopabcdefghijklmnopabcdefghijklmnopabcdefghijklmno";
        let (word_wrapper, _, _) = run_composer(
            Composer::WordWrapper { trim: true },
            false,
            text,
            width as u16,
        );
        let (word_wrapper_rev, _, _) = run_composer(
            Composer::WordWrapper { trim: true },
            true,
            text,
            width as u16,
        );
        let (line_truncator, _, _) =
            run_composer(Composer::LineTruncator, false, text, width as u16);
        let (line_truncator_rev, _, _) =
            run_composer(Composer::LineTruncator, true, text, width as u16);

        let wrapped = vec![
            &text[..width],
            &text[width..width * 2],
            &text[width * 2..width * 3],
            &text[width * 3..],
        ];
        let mut wrapped_rev = wrapped.clone();
        wrapped_rev.reverse();
        assert_eq!(
            word_wrapper, wrapped,
            "WordWrapper should detect the line cannot be broken on word boundary and \
             break it at line width limit."
        );
        assert_eq!(word_wrapper_rev, wrapped_rev,);
        assert_eq!(line_truncator, [&text[..width]]);
        assert_eq!(line_truncator_rev, [&text[..width]]);
    }

    #[test]
    fn line_composer_long_sentence() {
        let width = 20;
        let text =
            "abcd efghij klmnopabcd efgh ijklmnopabcdefg hijkl mnopab c d e f g h i j k l m n o";
        let text_multi_space =
            "abcd efghij    klmnopabcd efgh     ijklmnopabcdefg hijkl mnopab c d e f g h i j k l \
             m n o";
        let (word_wrapper_single_space, _, _) = run_composer(
            Composer::WordWrapper { trim: true },
            false,
            text,
            width as u16,
        );
        let (word_wrapper_multi_space, _, _) = run_composer(
            Composer::WordWrapper { trim: true },
            false,
            text_multi_space,
            width as u16,
        );
        let (word_wrapper_rev, _, _) = run_composer(
            Composer::WordWrapper { trim: true },
            true,
            text_multi_space,
            width as u16,
        );
        let (line_truncator, _, _) =
            run_composer(Composer::LineTruncator, false, text, width as u16);
        let (line_truncator_rev, _, _) =
            run_composer(Composer::LineTruncator, true, text, width as u16);

        let word_wrapped = [
            "abcd efghij",
            "klmnopabcd efgh",
            "ijklmnopabcdefg",
            "hijkl mnopab c d e f",
            "g h i j k l m n o",
        ];
        let mut word_wrapped_rev = word_wrapped;
        word_wrapped_rev.reverse();

        assert_eq!(word_wrapper_single_space, word_wrapped);
        assert_eq!(word_wrapper_multi_space, word_wrapped);
        assert_eq!(word_wrapper_rev, word_wrapped_rev);

        assert_eq!(line_truncator, [&text[..width]]);
        assert_eq!(line_truncator_rev, [&text[..width]]);
    }

    #[test]
    fn line_composer_zero_width() {
        let width = 0;
        let text = "abcd efghij klmnopabcd efgh ijklmnopabcdefg hijkl mnopab ";
        let (word_wrapper, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, false, text, width);
        let (word_wrapper_rev, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, true, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, false, text, width);
        let (line_truncator_rev, _, _) = run_composer(Composer::LineTruncator, true, text, width);

        let expected: Vec<&str> = Vec::new();
        assert_eq!(word_wrapper, expected);
        assert_eq!(word_wrapper_rev, expected);
        assert_eq!(line_truncator, expected);
        assert_eq!(line_truncator_rev, expected);
    }

    #[test]
    fn line_composer_max_line_width_of_1() {
        let width = 1;
        let text = "abcd efghij klmnopabcd efgh ijklmnopabcdefg hijkl mnopab ";
        let (word_wrapper, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, false, text, width);
        let (word_wrapper_rev, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, true, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, false, text, width);
        let (line_truncator_rev, _, _) = run_composer(Composer::LineTruncator, true, text, width);

        let expected: Vec<&str> = UnicodeSegmentation::graphemes(text, true)
            .filter(|g| g.chars().any(|c| !c.is_whitespace()))
            .collect();
        let mut expected_rev: Vec<&str> = expected.clone();
        expected_rev.reverse();
        assert_eq!(word_wrapper, expected);
        assert_eq!(word_wrapper_rev, expected_rev);
        assert_eq!(line_truncator, ["a"]);
        assert_eq!(line_truncator_rev, ["a"]);
    }

    #[test]
    fn line_composer_max_line_width_of_1_double_width_characters() {
        let width = 1;
        let text =
            "コンピュータ上で文字を扱う場合、典型的には文字\naaa\naによる通信を行う場合にその\
                    両端点では、";
        let (word_wrapper, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, false, text, width);
        let (word_wrapper_rev, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, true, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, false, text, width);
        let (line_truncator_rev, _, _) = run_composer(Composer::LineTruncator, true, text, width);
        assert_eq!(word_wrapper, ["", "a", "a", "a", "a"]);
        assert_eq!(word_wrapper_rev, ["a", "a", "a", "a", ""]);
        assert_eq!(line_truncator, ["", "a", "a"]);
        assert_eq!(line_truncator_rev, ["a", "a", ""]);
    }

    /// Tests `WordWrapper` with words some of which exceed line length and some not.
    #[test]
    fn line_composer_word_wrapper_mixed_length() {
        let width = 20;
        let text = "abcd efghij klmnopabcdefghijklmnopabcdefghijkl mnopab cdefghi j klmno";
        let (word_wrapper, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, false, text, width);
        let (word_wrapper_rev, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, true, text, width);
        let expected = [
            "abcd efghij",
            "klmnopabcdefghijklmn",
            "opabcdefghijkl",
            "mnopab cdefghi j",
            "klmno",
        ];
        let mut expected_rev = expected;
        expected_rev.reverse();
        assert_eq!(word_wrapper, expected);
        assert_eq!(word_wrapper_rev, expected_rev);
    }

    #[test]
    fn line_composer_double_width_chars() {
        let width = 20;
        let text = "コンピュータ上で文字を扱う場合、典型的には文字による通信を行う場合にその両端点\
                    では、";
        let (word_wrapper, word_wrapper_width, _) =
            run_composer(Composer::WordWrapper { trim: true }, false, text, width);
        let (word_wrapper_rev, word_wrapper_rev_width, _) =
            run_composer(Composer::WordWrapper { trim: true }, true, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, false, text, width);
        let (line_truncator_rev, _, _) = run_composer(Composer::LineTruncator, true, text, width);
        assert_eq!(line_truncator, ["コンピュータ上で文字"]);
        assert_eq!(line_truncator_rev, ["コンピュータ上で文字"]);
        let wrapped = [
            "コンピュータ上で文字",
            "を扱う場合、典型的に",
            "は文字による通信を行",
            "う場合にその両端点で",
            "は、",
        ];
        let mut wrapped_rev = wrapped;
        wrapped_rev.reverse();
        assert_eq!(word_wrapper, wrapped);
        assert_eq!(word_wrapper_width, vec![width, width, width, width, 4]);
        assert_eq!(word_wrapper_rev, wrapped_rev);
        assert_eq!(word_wrapper_rev_width, vec![4, width, width, width, width]);
    }

    #[test]
    fn line_composer_leading_whitespace_removal() {
        let width = 20;
        let text = "AAAAAAAAAAAAAAAAAAAA    AAA";
        let (word_wrapper, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, false, text, width);
        let (word_wrapper_rev, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, true, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, false, text, width);
        let (line_truncator_rev, _, _) = run_composer(Composer::LineTruncator, true, text, width);
        assert_eq!(word_wrapper, ["AAAAAAAAAAAAAAAAAAAA", "AAA",]);
        assert_eq!(word_wrapper_rev, ["AAA", "AAAAAAAAAAAAAAAAAAAA",]);
        assert_eq!(line_truncator, ["AAAAAAAAAAAAAAAAAAAA"]);
        assert_eq!(line_truncator_rev, ["AAAAAAAAAAAAAAAAAAAA"]);
    }

    /// Tests truncation of leading whitespace.
    #[test]
    fn line_composer_lots_of_spaces() {
        let width = 20;
        let text = "                                                                     ";
        let (word_wrapper, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, false, text, width);
        let (word_wrapper_rev, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, true, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, false, text, width);
        let (line_truncator_rev, _, _) = run_composer(Composer::LineTruncator, true, text, width);
        assert_eq!(word_wrapper, [""]);
        assert_eq!(word_wrapper_rev, [""]);
        assert_eq!(line_truncator, ["                    "]);
        assert_eq!(line_truncator_rev, ["                    "]);
    }

    /// Tests an input starting with a letter, followed by spaces - some of the behaviour is
    /// incidental.
    #[test]
    fn line_composer_char_plus_lots_of_spaces() {
        let width = 20;
        let text = "a                                                                     ";
        let (word_wrapper, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, false, text, width);
        let (word_wrapper_rev, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, true, text, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, false, text, width);
        let (line_truncator_rev, _, _) = run_composer(Composer::LineTruncator, true, text, width);
        // What's happening below is: the first line gets consumed, trailing spaces discarded,
        // after 20 of which a word break occurs (probably shouldn't). The second line break
        // discards all whitespace. The result should probably be vec!["a"] but it doesn't matter
        // that much.
        assert_eq!(word_wrapper, ["a", ""]);
        assert_eq!(word_wrapper_rev, ["", "a"]);
        assert_eq!(line_truncator, ["a                   "]);
        assert_eq!(line_truncator_rev, ["a                   "]);
    }

    #[test]
    fn line_composer_word_wrapper_double_width_chars_mixed_with_spaces() {
        let width = 20;
        // Japanese seems not to use spaces but we should break on spaces anyway... We're using it
        // to test double-width chars.
        // You are more than welcome to add word boundary detection based of alterations of
        // hiragana and katakana...
        // This happens to also be a test case for mixed width because regular spaces are single width.
        let text = "コンピュ ータ上で文字を扱う場合、 典型的には文 字による 通信を行 う場合にその両端点では、";
        let (word_wrapper, word_wrapper_width, _) =
            run_composer(Composer::WordWrapper { trim: true }, false, text, width);
        let (word_wrapper_rev, word_wrapper_rev_width, _) =
            run_composer(Composer::WordWrapper { trim: true }, true, text, width);
        let expected = [
            "コンピュ",
            "ータ上で文字を扱う場",
            "合、 典型的には文",
            "字による 通信を行",
            "う場合にその両端点で",
            "は、",
        ];
        let mut expected_rev = expected;
        expected_rev.reverse();
        assert_eq!(word_wrapper, expected);
        assert_eq!(word_wrapper_rev, expected_rev);
        // Odd-sized lines have a space in them.
        let expected_width = [8, 20, 17, 17, 20, 4];
        let mut expected_width_rev = expected_width;
        expected_width_rev.reverse();
        assert_eq!(word_wrapper_width, expected_width);
        assert_eq!(word_wrapper_rev_width, expected_width_rev);
    }

    /// Ensure words separated by nbsp are wrapped as if they were a single one.
    #[test]
    fn line_composer_word_wrapper_nbsp() {
        let width = 20;
        let text = "AAAAAAAAAAAAAAA AAAA\u{00a0}AAA";
        let (word_wrapper, word_wrapper_widths, _) =
            run_composer(Composer::WordWrapper { trim: true }, false, text, width);
        let (word_wrapper_rev, word_wrapper_rev_widths, _) =
            run_composer(Composer::WordWrapper { trim: true }, true, text, width);
        assert_eq!(word_wrapper, ["AAAAAAAAAAAAAAA", "AAAA\u{00a0}AAA",]);
        assert_eq!(word_wrapper_rev, ["AAAA\u{00a0}AAA", "AAAAAAAAAAAAAAA"]);
        assert_eq!(word_wrapper_widths, [15, 8]);
        assert_eq!(word_wrapper_rev_widths, [8, 15]);

        // Ensure that if the character was a regular space, it would be wrapped differently.
        let text_space = text.replace('\u{00a0}', " ");
        let (word_wrapper_space, word_wrapper_widths, _) = run_composer(
            Composer::WordWrapper { trim: true },
            false,
            text_space.clone(),
            width,
        );
        let (word_wrapper_rev_space, word_wrapper_rev_widths, _) = run_composer(
            Composer::WordWrapper { trim: true },
            true,
            text_space,
            width,
        );
        assert_eq!(word_wrapper_space, ["AAAAAAAAAAAAAAA AAAA", "AAA",]);
        assert_eq!(word_wrapper_rev_space, ["AAA", "AAAAAAAAAAAAAAA AAAA"]);
        assert_eq!(word_wrapper_widths, [20, 3]);
        assert_eq!(word_wrapper_rev_widths, [3, 20]);
    }

    #[test]
    fn line_composer_word_wrapper_preserve_indentation() {
        let width = 20;
        let text = "AAAAAAAAAAAAAAAAAAAA    AAA";
        let (word_wrapper, _, _) =
            run_composer(Composer::WordWrapper { trim: false }, false, text, width);
        let (word_wrapper_rev, _, _) =
            run_composer(Composer::WordWrapper { trim: false }, true, text, width);
        assert_eq!(word_wrapper, ["AAAAAAAAAAAAAAAAAAAA", "   AAA",]);
        assert_eq!(word_wrapper_rev, ["   AAA", "AAAAAAAAAAAAAAAAAAAA",]);
    }

    #[test]
    fn line_composer_word_wrapper_preserve_indentation_with_wrap() {
        let width = 10;
        let text = "AAA AAA AAAAA AA AAAAAA\n B\n  C\n   D";
        let (word_wrapper, _, _) =
            run_composer(Composer::WordWrapper { trim: false }, false, text, width);
        let (word_wrapper_rev, _, _) =
            run_composer(Composer::WordWrapper { trim: false }, true, text, width);
        let expected = ["AAA AAA", "AAAAA AA", "AAAAAA", " B", "  C", "   D"];
        let mut expected_rev = expected;
        expected_rev.reverse();
        assert_eq!(word_wrapper, expected,);
        assert_eq!(word_wrapper_rev, expected_rev,);
    }

    #[test]
    fn line_composer_word_wrapper_preserve_indentation_lots_of_whitespace() {
        let width = 10;
        let text = "               4 Indent\n                 must wrap!";
        let (word_wrapper, _, _) =
            run_composer(Composer::WordWrapper { trim: false }, false, text, width);
        let (word_wrapper_rev, _, _) =
            run_composer(Composer::WordWrapper { trim: false }, true, text, width);
        let expected = [
            "          ",
            "    4",
            "Indent",
            "          ",
            "      must",
            "wrap!",
        ];
        let mut expected_rev = expected;
        expected_rev.reverse();
        assert_eq!(word_wrapper, expected);
        assert_eq!(word_wrapper_rev, expected_rev);
    }

    #[test]
    fn line_composer_zero_width_at_end() {
        let width = 3;
        let line = "foo\0";
        let (word_wrapper, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, false, line, width);
        let (word_wrapper_rev, _, _) =
            run_composer(Composer::WordWrapper { trim: true }, true, line, width);
        let (line_truncator, _, _) = run_composer(Composer::LineTruncator, false, line, width);
        let (line_truncator_rev, _, _) = run_composer(Composer::LineTruncator, true, line, width);
        assert_eq!(word_wrapper, ["foo\0"]);
        assert_eq!(word_wrapper_rev, ["foo\0"]);
        assert_eq!(line_truncator, ["foo\0"]);
        assert_eq!(line_truncator_rev, ["foo\0"]);
    }

    #[test]
    fn line_composer_preserves_line_alignment() {
        let width = 20;
        let lines = vec![
            Line::from("Something that is left aligned.").alignment(Alignment::Left),
            Line::from("This is right aligned and half short.").alignment(Alignment::Right),
            Line::from("This should sit in the center.").alignment(Alignment::Center),
        ];
        let (_, _, wrapped_alignments) = run_composer(
            Composer::WordWrapper { trim: true },
            false,
            lines.clone(),
            width,
        );
        let (_, _, wrapped_alignments_rev) = run_composer(
            Composer::WordWrapper { trim: true },
            true,
            lines.clone(),
            width,
        );
        let (_, _, truncated_alignments) =
            run_composer(Composer::LineTruncator, false, lines.clone(), width);
        let (_, _, truncated_alignments_rev) =
            run_composer(Composer::LineTruncator, true, lines, width);
        let expected_wrapped = [
            Alignment::Left,
            Alignment::Left,
            Alignment::Right,
            Alignment::Right,
            Alignment::Right,
            Alignment::Center,
            Alignment::Center,
        ];
        let mut expected_wrapped_rev = expected_wrapped;
        expected_wrapped_rev.reverse();
        assert_eq!(wrapped_alignments, expected_wrapped,);
        assert_eq!(wrapped_alignments_rev, expected_wrapped_rev,);
        assert_eq!(
            truncated_alignments,
            [Alignment::Left, Alignment::Right, Alignment::Center]
        );
        assert_eq!(
            truncated_alignments_rev,
            [Alignment::Center, Alignment::Right, Alignment::Left]
        );
    }
}
