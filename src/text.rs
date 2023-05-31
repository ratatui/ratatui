//! Primitives for styled text.
//!
//! A terminal UI is at its root a lot of strings. In order to make it accessible and stylish,
//! those strings may be associated to a set of styles. `ratatui` has three ways to represent them:
//! - A single line string where all graphemes have the same style is represented by a [`Span`].
//! - A single line string where each grapheme may have its own style is represented by [`Line`].
//! - A multiple line string where each grapheme may have its own style is represented by a
//! [`Text`].
//!
//! These types form a hierarchy: [`Line`] is a collection of [`Span`] and each line of [`Text`]
//! is a [`Line`].
//!
//! Keep it mind that a lot of widgets will use those types to advertise what kind of string is
//! supported for their properties. Moreover, `ratatui` provides convenient `From` implementations so
//! that you can start by using simple `String` or `&str` and then promote them to the previous
//! primitives when you need additional styling capabilities.
//!
//! For example, for the [`crate::widgets::Block`] widget, all the following calls are valid to set
//! its `title` property (which is a [`Line`] under the hood):
//!
//! ```rust
//! # use ratatui::widgets::Block;
//! # use ratatui::text::{Span, Line};
//! # use ratatui::style::{Color, Style};
//! // A simple string with no styling.
//! // Converted to Line(vec![
//! //   Span { content: Cow::Borrowed("My title"), style: Style { .. } }
//! // ])
//! let block = Block::default().title("My title");
//!
//! // A simple string with a unique style.
//! // Converted to Line(vec![
//! //   Span { content: Cow::Borrowed("My title"), style: Style { fg: Some(Color::Yellow), .. }
//! // ])
//! let block = Block::default().title(
//!     Span::styled("My title", Style::default().fg(Color::Yellow))
//! );
//!
//! // A string with multiple styles.
//! // Converted to Line(vec![
//! //   Span { content: Cow::Borrowed("My"), style: Style { fg: Some(Color::Yellow), .. } },
//! //   Span { content: Cow::Borrowed(" title"), .. }
//! // ])
//! let block = Block::default().title(vec![
//!     Span::styled("My", Style::default().fg(Color::Yellow)),
//!     Span::raw(" title"),
//! ]);
//! ```
use crate::style::Style;
use std::{
    borrow::Cow,
    fmt::{self, Debug},
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

mod line;
mod masked;
mod spans;
#[allow(deprecated)]
pub use {line::Line, masked::Masked, spans::Spans};

/// A grapheme associated to a style.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyledGrapheme<'a> {
    pub symbol: &'a str,
    pub style: Style,
}

/// A string where all graphemes have the same style.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span<'a> {
    pub content: Cow<'a, str>,
    pub style: Style,
}

impl<'a> Span<'a> {
    /// Create a span with no style.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::text::Span;
    /// Span::raw("My text");
    /// Span::raw(String::from("My text"));
    /// ```
    pub fn raw<T>(content: T) -> Span<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Span {
            content: content.into(),
            style: Style::default(),
        }
    }

    /// Create a span with a style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::text::Span;
    /// # use ratatui::style::{Color, Modifier, Style};
    /// let style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
    /// Span::styled("My text", style);
    /// Span::styled(String::from("My text"), style);
    /// ```
    pub fn styled<T>(content: T, style: Style) -> Span<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Span {
            content: content.into(),
            style,
        }
    }

    /// Returns the width of the content held by this span.
    pub fn width(&self) -> usize {
        self.content.width()
    }

    /// Returns an iterator over the graphemes held by this span.
    ///
    /// `base_style` is the [`Style`] that will be patched with each grapheme [`Style`] to get
    /// the resulting [`Style`].
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::text::{Span, StyledGrapheme};
    /// # use ratatui::style::{Color, Modifier, Style};
    /// # use std::iter::Iterator;
    /// let style = Style::default().fg(Color::Yellow);
    /// let span = Span::styled("Text", style);
    /// let style = Style::default().fg(Color::Green).bg(Color::Black);
    /// let styled_graphemes = span.styled_graphemes(style);
    /// assert_eq!(
    ///     vec![
    ///         StyledGrapheme {
    ///             symbol: "T",
    ///             style: Style {
    ///                 fg: Some(Color::Yellow),
    ///                 bg: Some(Color::Black),
    ///                 add_modifier: Modifier::empty(),
    ///                 sub_modifier: Modifier::empty(),
    ///             },
    ///         },
    ///         StyledGrapheme {
    ///             symbol: "e",
    ///             style: Style {
    ///                 fg: Some(Color::Yellow),
    ///                 bg: Some(Color::Black),
    ///                 add_modifier: Modifier::empty(),
    ///                 sub_modifier: Modifier::empty(),
    ///             },
    ///         },
    ///         StyledGrapheme {
    ///             symbol: "x",
    ///             style: Style {
    ///                 fg: Some(Color::Yellow),
    ///                 bg: Some(Color::Black),
    ///                 add_modifier: Modifier::empty(),
    ///                 sub_modifier: Modifier::empty(),
    ///             },
    ///         },
    ///         StyledGrapheme {
    ///             symbol: "t",
    ///             style: Style {
    ///                 fg: Some(Color::Yellow),
    ///                 bg: Some(Color::Black),
    ///                 add_modifier: Modifier::empty(),
    ///                 sub_modifier: Modifier::empty(),
    ///             },
    ///         },
    ///     ],
    ///     styled_graphemes.collect::<Vec<StyledGrapheme>>()
    /// );
    /// ```
    pub fn styled_graphemes(
        &'a self,
        base_style: Style,
    ) -> impl Iterator<Item = StyledGrapheme<'a>> {
        UnicodeSegmentation::graphemes(self.content.as_ref(), true)
            .map(move |g| StyledGrapheme {
                symbol: g,
                style: base_style.patch(self.style),
            })
            .filter(|s| s.symbol != "\n")
    }

    /// Patches the style an existing Span, adding modifiers from the given style.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::text::Span;
    /// # use ratatui::style::{Color, Style, Modifier};
    /// let style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
    /// let mut raw_span = Span::raw("My text");
    /// let mut styled_span = Span::styled("My text", style);
    ///
    /// assert_ne!(raw_span, styled_span);
    ///
    /// raw_span.patch_style(style);
    /// assert_eq!(raw_span, styled_span);
    /// ```
    pub fn patch_style(&mut self, style: Style) {
        self.style = self.style.patch(style);
    }

    /// Resets the style of the Span.
    /// Equivalent to calling `patch_style(Style::reset())`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::text::Span;
    /// # use ratatui::style::{Color, Style, Modifier};
    /// let mut span = Span::styled("My text", Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC));
    ///
    /// span.reset_style();
    /// assert_eq!(Style::reset(), span.style);
    /// ```
    pub fn reset_style(&mut self) {
        self.patch_style(Style::reset());
    }
}

impl<'a> From<String> for Span<'a> {
    fn from(s: String) -> Span<'a> {
        Span::raw(s)
    }
}

impl<'a> From<&'a str> for Span<'a> {
    fn from(s: &'a str) -> Span<'a> {
        Span::raw(s)
    }
}

/// A string split over multiple lines where each line is composed of several clusters, each with
/// their own style.
///
/// A [`Text`], like a [`Span`], can be constructed using one of the many `From` implementations
/// or via the [`Text::raw`] and [`Text::styled`] methods. Helpfully, [`Text`] also implements
/// [`core::iter::Extend`] which enables the concatenation of several [`Text`] blocks.
///
/// ```rust
/// # use ratatui::text::Text;
/// # use ratatui::style::{Color, Modifier, Style};
/// let style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
///
/// // An initial two lines of `Text` built from a `&str`
/// let mut text = Text::from("The first line\nThe second line");
/// assert_eq!(2, text.height());
///
/// // Adding two more unstyled lines
/// text.extend(Text::raw("These are two\nmore lines!"));
/// assert_eq!(4, text.height());
///
/// // Adding a final two styled lines
/// text.extend(Text::styled("Some more lines\nnow with more style!", style));
/// assert_eq!(6, text.height());
/// ```
#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct Text<'a> {
    pub lines: Vec<Line<'a>>,
}

impl<'a> Text<'a> {
    /// Create some text (potentially multiple lines) with no style.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::text::Text;
    /// Text::raw("The first line\nThe second line");
    /// Text::raw(String::from("The first line\nThe second line"));
    /// ```
    pub fn raw<T>(content: T) -> Text<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        let lines: Vec<_> = match content.into() {
            Cow::Borrowed("") => vec![Line::from("")],
            Cow::Owned(s) if s.is_empty() => vec![Line::from("")],
            // `.lines()` eats trailing newlines, which we don't want - hence this custom solution.
            Cow::Borrowed(s) => s
                .split('\n')
                .map(|s| Line::from(s.strip_suffix('\r').unwrap_or(s)))
                .collect(),
            Cow::Owned(s) => s
                .split('\n')
                .map(|s| Line::from(s.strip_suffix('\r').unwrap_or(s).to_owned()))
                .collect(),
        };

        Text::from(lines)
    }

    /// Create some text (potentially multiple lines) with a style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::text::Text;
    /// # use ratatui::style::{Color, Modifier, Style};
    /// let style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
    /// Text::styled("The first line\nThe second line", style);
    /// Text::styled(String::from("The first line\nThe second line"), style);
    /// ```
    pub fn styled<T>(content: T, style: Style) -> Text<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        let mut text = Text::raw(content);
        text.patch_style(style);
        text
    }

    /// Returns the max width of all the lines.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ratatui::text::Text;
    /// let text = Text::from("The first line\nThe second line");
    /// assert_eq!(15, text.width());
    /// ```
    pub fn width(&self) -> usize {
        self.lines.iter().map(Line::width).max().unwrap_or_default()
    }

    /// Returns the height.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ratatui::text::Text;
    /// let text = Text::from("The first line\nThe second line");
    /// assert_eq!(2, text.height());
    /// ```
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /// Patches the style of each line in an existing Text, adding modifiers from the given style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::text::Text;
    /// # use ratatui::style::{Color, Modifier, Style};
    /// let style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
    /// let mut raw_text = Text::raw("The first line\nThe second line");
    /// let styled_text = Text::styled(String::from("The first line\nThe second line"), style);
    /// assert_ne!(raw_text, styled_text);
    ///
    /// raw_text.patch_style(style);
    /// assert_eq!(raw_text, styled_text);
    /// ```
    pub fn patch_style(&mut self, style: Style) {
        for line in &mut self.lines {
            line.patch_style(style);
        }
    }

    /// Resets the style of the Text.
    /// Equivalent to calling `patch_style(Style::reset())`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::text::{Span, Line, Text};
    /// # use ratatui::style::{Color, Style, Modifier};
    /// let style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
    /// let mut text = Text::styled("The first line\nThe second line", style);
    ///
    /// text.reset_style();
    /// for line in &text.lines {
    ///     for span in &line.spans {
    ///         assert_eq!(Style::reset(), span.style);
    ///     }
    /// }
    /// ```
    pub fn reset_style(&mut self) {
        for line in &mut self.lines {
            line.reset_style();
        }
    }
}

impl<'a> From<String> for Text<'a> {
    fn from(s: String) -> Text<'a> {
        Text::raw(s)
    }
}

impl<'a> From<&'a str> for Text<'a> {
    fn from(s: &'a str) -> Text<'a> {
        Text::raw(s)
    }
}

impl<'a> From<Cow<'a, str>> for Text<'a> {
    fn from(s: Cow<'a, str>) -> Text<'a> {
        Text::raw(s)
    }
}

impl<'a> From<Span<'a>> for Text<'a> {
    fn from(span: Span<'a>) -> Text<'a> {
        Text {
            lines: vec![Line::from(span)],
        }
    }
}

#[allow(deprecated)]
impl<'a> From<Spans<'a>> for Text<'a> {
    fn from(spans: Spans<'a>) -> Text<'a> {
        Text {
            lines: vec![spans.into()],
        }
    }
}

impl<'a> From<Line<'a>> for Text<'a> {
    fn from(line: Line<'a>) -> Text<'a> {
        Text { lines: vec![line] }
    }
}

#[allow(deprecated)]
impl<'a> From<Vec<Spans<'a>>> for Text<'a> {
    fn from(lines: Vec<Spans<'a>>) -> Text<'a> {
        Text {
            lines: lines.into_iter().map(|l| l.0.into()).collect(),
        }
    }
}

impl<'a> From<Vec<Line<'a>>> for Text<'a> {
    fn from(lines: Vec<Line<'a>>) -> Text<'a> {
        Text { lines }
    }
}

impl<'a> IntoIterator for Text<'a> {
    type Item = Line<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter()
    }
}

impl<'a, T> Extend<T> for Text<'a>
where
    T: Into<Line<'a>>,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let lines = iter.into_iter().map(Into::into);
        self.lines.extend(lines);
    }
}

impl fmt::Write for Text<'_> {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        /// We would like the compiler to inline the return line,
        /// and therefore see that the result is always `Ok(())`,
        /// allowing it to turn `unwrap()` calls into a no-op.
        /// However, we don't want to inline
        /// the entire `write_str` function. Hence this split.
        #[inline(never)]
        fn write_str_inner(text: &mut Text<'_>, s: &str) {
            // Split on newlines.
            let mut lines = s.split('\n').map(|s| s.strip_suffix('\r').unwrap_or(s));

            // Append the start of the to-be-written text
            // (before the first newline) to the end of the previous line,
            // if there is one.
            if let Some(prev_end) = text.lines.last_mut() {
                if let Some(first) = lines.next() {
                    if !first.is_empty() {
                        // Only make a new `String` if the trailing one is not suitable to push onto.
                        match prev_end.spans.last_mut() {
                            Some(Span {
                                content: Cow::Owned(content),
                                style,
                            }) if *style == Style::default() => content.push_str(first),
                            _ => prev_end.spans.push(Span::from(first.to_owned())),
                        }
                    }
                } else {
                    return;
                }
            }

            // Append the rest of the lines to the `Text`,
            // one-by-one.
            for line in lines {
                text.lines.push(line.to_owned().into());
            }
        }

        write_str_inner(self, s);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fmt::Write;

    #[test]
    fn text_format() {
        let mut text = Text::from("\nThis is a test.\n");
        write!(text, "To be precise, a test is what this is. ").unwrap();
        writeln!(text, "Indeed.\nYes.\nClearly.").unwrap();

        let text_2 = Text::raw(
            "\nThis is a test.\nTo be precise, a test is what this is. Indeed.\nYes.\nClearly.\n",
        );

        assert_eq!(text, text_2);
    }
}
