use std::borrow::Cow;

#[allow(deprecated)]
use super::{Line, Span, Spans};
use crate::style::Style;

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
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
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
            Cow::Borrowed(s) => s.lines().map(Line::from).collect(),
            Cow::Owned(s) if s.is_empty() => vec![Line::from("")],
            Cow::Owned(s) => s.lines().map(|l| Line::from(l.to_owned())).collect(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Stylize;

    #[test]
    fn raw() {
        let text = Text::raw("The first line\nThe second line");
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn styled() {
        let style = Style::new().yellow().italic();
        let text = Text::styled("The first line\nThe second line", style);
        assert_eq!(
            text.lines,
            vec![
                Line::from(Span::styled("The first line", style)),
                Line::from(Span::styled("The second line", style))
            ]
        );
    }

    #[test]
    fn width() {
        let text = Text::from("The first line\nThe second line");
        assert_eq!(15, text.width());
    }

    #[test]
    fn height() {
        let text = Text::from("The first line\nThe second line");
        assert_eq!(2, text.height());
    }

    #[test]
    fn patch_style() {
        let style = Style::new().yellow().italic();
        let style2 = Style::new().red().underlined();
        let mut text = Text::styled("The first line\nThe second line", style);

        text.patch_style(style2);
        let expected_style = Style::new().red().italic().underlined();
        assert_eq!(
            text.lines,
            vec![
                Line::from(Span::styled("The first line", expected_style)),
                Line::from(Span::styled("The second line", expected_style))
            ]
        );
    }

    #[test]
    fn reset_style() {
        let style = Style::new().yellow().italic();
        let mut text = Text::styled("The first line\nThe second line", style);

        text.reset_style();
        assert_eq!(
            text.lines,
            vec![
                Line::from(Span::styled("The first line", Style::reset())),
                Line::from(Span::styled("The second line", Style::reset()))
            ]
        );
    }

    #[test]
    fn from_string() {
        let text = Text::from(String::from("The first line\nThe second line"));
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn from_str() {
        let text = Text::from("The first line\nThe second line");
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn from_cow() {
        let text = Text::from(Cow::Borrowed("The first line\nThe second line"));
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn from_span() {
        let style = Style::new().yellow().italic();
        let text = Text::from(Span::styled("The first line\nThe second line", style));
        assert_eq!(
            text.lines,
            vec![Line::from(Span::styled(
                "The first line\nThe second line",
                style
            ))]
        );
    }

    #[test]
    #[allow(deprecated)]
    fn from_spans() {
        let style = Style::new().yellow().italic();
        let text = Text::from(Spans::from(vec![
            Span::styled("The first line", style),
            Span::styled("The second line", style),
        ]));
        assert_eq!(
            text.lines,
            vec![Line::from(Spans::from(vec![
                Span::styled("The first line", style),
                Span::styled("The second line", style),
            ]))]
        );
    }

    #[test]
    fn from_line() {
        let text = Text::from(Line::from("The first line"));
        assert_eq!(text.lines, vec![Line::from("The first line")]);
    }

    #[test]
    #[allow(deprecated)]
    fn from_vec_spans() {
        let text = Text::from(vec![
            Spans::from("The first line"),
            Spans::from("The second line"),
        ]);
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line"),]
        );
    }

    #[test]
    fn from_vec_line() {
        let text = Text::from(vec![
            Line::from("The first line"),
            Line::from("The second line"),
        ]);
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn into_iter() {
        let text = Text::from("The first line\nThe second line");
        let mut iter = text.into_iter();
        assert_eq!(iter.next(), Some(Line::from("The first line")));
        assert_eq!(iter.next(), Some(Line::from("The second line")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn extend() {
        let mut text = Text::from("The first line\nThe second line");
        text.extend(vec![
            Line::from("The third line"),
            Line::from("The fourth line"),
        ]);
        assert_eq!(
            text.lines,
            vec![
                Line::from("The first line"),
                Line::from("The second line"),
                Line::from("The third line"),
                Line::from("The fourth line"),
            ]
        );
    }

    #[test]
    fn extend_from_iter() {
        let mut text = Text::from("The first line\nThe second line");
        text.extend(vec![
            Line::from("The third line"),
            Line::from("The fourth line"),
        ]);
        assert_eq!(
            text.lines,
            vec![
                Line::from("The first line"),
                Line::from("The second line"),
                Line::from("The third line"),
                Line::from("The fourth line"),
            ]
        );
    }

    #[test]
    fn extend_from_iter_str() {
        let mut text = Text::from("The first line\nThe second line");
        text.extend(vec!["The third line", "The fourth line"]);
        assert_eq!(
            text.lines,
            vec![
                Line::from("The first line"),
                Line::from("The second line"),
                Line::from("The third line"),
                Line::from("The fourth line"),
            ]
        );
    }
}
