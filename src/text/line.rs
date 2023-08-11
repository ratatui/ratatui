#![allow(deprecated)]
use std::borrow::Cow;

use super::{Span, Spans, Style, StyledGrapheme};
use crate::layout::Alignment;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Line<'a> {
    pub spans: Vec<Span<'a>>,
    pub alignment: Option<Alignment>,
}

impl<'a> Line<'a> {
    /// Create a line with a style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::text::Line;
    /// # use ratatui::style::{Color, Modifier, Style};
    /// let style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
    /// Line::styled("My text", style);
    /// Line::styled(String::from("My text"), style);
    /// ```
    pub fn styled<T>(content: T, style: Style) -> Line<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Line::from(Span::styled(content, style))
    }

    /// Returns the width of the underlying string.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::text::{Span, Line};
    /// # use ratatui::style::{Color, Style};
    /// let line = Line::from(vec![
    ///     Span::styled("My", Style::default().fg(Color::Yellow)),
    ///     Span::raw(" text"),
    /// ]);
    /// assert_eq!(7, line.width());
    /// ```
    pub fn width(&self) -> usize {
        self.spans.iter().map(Span::width).sum()
    }

    /// Returns an iterator over the graphemes held by this line.
    ///
    /// `base_style` is the [`Style`] that will be patched with each grapheme [`Style`] to get
    /// the resulting [`Style`].
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::text::{Line, StyledGrapheme};
    /// # use ratatui::style::{Color, Modifier, Style};
    /// # use std::iter::Iterator;
    /// let line = Line::styled("Text", Style::default().fg(Color::Yellow));
    /// let style = Style::default().fg(Color::Green).bg(Color::Black);
    /// assert_eq!(
    ///     line.styled_graphemes(style).collect::<Vec<StyledGrapheme>>(),
    ///     vec![
    ///         StyledGrapheme::new("T", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///         StyledGrapheme::new("e", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///         StyledGrapheme::new("x", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///         StyledGrapheme::new("t", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///     ]
    /// );
    /// ```
    pub fn styled_graphemes(
        &'a self,
        base_style: Style,
    ) -> impl Iterator<Item = StyledGrapheme<'a>> {
        self.spans
            .iter()
            .flat_map(move |span| span.styled_graphemes(base_style))
    }

    /// Patches the style of each Span in an existing Line, adding modifiers from the given style.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::text::{Span, Line};
    /// # use ratatui::style::{Color, Style, Modifier};
    /// let style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
    /// let mut raw_line = Line::from(vec![
    ///     Span::raw("My"),
    ///     Span::raw(" text"),
    /// ]);
    /// let mut styled_line = Line::from(vec![
    ///     Span::styled("My", style),
    ///     Span::styled(" text", style),
    /// ]);
    ///
    /// assert_ne!(raw_line, styled_line);
    ///
    /// raw_line.patch_style(style);
    /// assert_eq!(raw_line, styled_line);
    /// ```
    pub fn patch_style(&mut self, style: Style) {
        for span in &mut self.spans {
            span.patch_style(style);
        }
    }

    /// Resets the style of each Span in the Line.
    /// Equivalent to calling `patch_style(Style::reset())`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::text::{Span, Line};
    /// # use ratatui::style::{Color, Style, Modifier};
    /// let mut line = Line::from(vec![
    ///     Span::styled("My", Style::default().fg(Color::Yellow)),
    ///     Span::styled(" text", Style::default().add_modifier(Modifier::BOLD)),
    /// ]);
    ///
    /// line.reset_style();
    /// assert_eq!(Style::reset(), line.spans[0].style);
    /// assert_eq!(Style::reset(), line.spans[1].style);
    /// ```
    pub fn reset_style(&mut self) {
        for span in &mut self.spans {
            span.reset_style();
        }
    }

    /// Sets the target alignment for this line of text.
    /// Defaults to: [`None`], meaning the alignment is determined by the rendering widget.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use std::borrow::Cow;
    /// # use ratatui::layout::Alignment;
    /// # use ratatui::text::{Span, Line};
    /// # use ratatui::style::{Color, Style, Modifier};
    /// let mut line = Line::from("Hi, what's up?");
    /// assert_eq!(None, line.alignment);
    /// assert_eq!(Some(Alignment::Right), line.alignment(Alignment::Right).alignment)
    /// ```
    pub fn alignment(self, alignment: Alignment) -> Self {
        Self {
            alignment: Some(alignment),
            ..self
        }
    }
}

impl<'a> From<String> for Line<'a> {
    fn from(s: String) -> Self {
        Self::from(vec![Span::from(s)])
    }
}

impl<'a> From<&'a str> for Line<'a> {
    fn from(s: &'a str) -> Self {
        Self::from(vec![Span::from(s)])
    }
}

impl<'a> From<Vec<Span<'a>>> for Line<'a> {
    fn from(spans: Vec<Span<'a>>) -> Self {
        Self {
            spans,
            ..Default::default()
        }
    }
}

impl<'a> From<Span<'a>> for Line<'a> {
    fn from(span: Span<'a>) -> Self {
        Self::from(vec![span])
    }
}

impl<'a> From<Line<'a>> for String {
    fn from(line: Line<'a>) -> String {
        line.spans.iter().fold(String::new(), |mut acc, s| {
            acc.push_str(s.content.as_ref());
            acc
        })
    }
}

impl<'a> From<Spans<'a>> for Line<'a> {
    fn from(value: Spans<'a>) -> Self {
        Self::from(value.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        layout::Alignment,
        style::{Color, Modifier, Style},
        text::{Line, Span, Spans, StyledGrapheme},
    };

    #[test]
    fn test_width() {
        let line = Line::from(vec![
            Span::styled("My", Style::default().fg(Color::Yellow)),
            Span::raw(" text"),
        ]);
        assert_eq!(7, line.width());

        let empty_line = Line::default();
        assert_eq!(0, empty_line.width());
    }

    #[test]
    fn test_patch_style() {
        let style = Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::ITALIC);
        let mut raw_line = Line::from(vec![Span::raw("My"), Span::raw(" text")]);
        let styled_line = Line::from(vec![
            Span::styled("My", style),
            Span::styled(" text", style),
        ]);

        assert_ne!(raw_line, styled_line);

        raw_line.patch_style(style);
        assert_eq!(raw_line, styled_line);
    }

    #[test]
    fn test_reset_style() {
        let mut line = Line::from(vec![
            Span::styled("My", Style::default().fg(Color::Yellow)),
            Span::styled(" text", Style::default().add_modifier(Modifier::BOLD)),
        ]);

        line.reset_style();
        assert_eq!(Style::reset(), line.spans[0].style);
        assert_eq!(Style::reset(), line.spans[1].style);
    }

    #[test]
    fn test_from_string() {
        let s = String::from("Hello, world!");
        let line = Line::from(s);
        assert_eq!(vec![Span::from("Hello, world!")], line.spans);
    }

    #[test]
    fn test_from_str() {
        let s = "Hello, world!";
        let line = Line::from(s);
        assert_eq!(vec![Span::from("Hello, world!")], line.spans);
    }

    #[test]
    fn test_from_vec() {
        let spans = vec![
            Span::styled("Hello,", Style::default().fg(Color::Red)),
            Span::styled(" world!", Style::default().fg(Color::Green)),
        ];
        let line = Line::from(spans.clone());
        assert_eq!(spans, line.spans);
    }

    #[test]
    fn test_from_span() {
        let span = Span::styled("Hello, world!", Style::default().fg(Color::Yellow));
        let line = Line::from(span.clone());
        assert_eq!(vec![span], line.spans);
    }

    #[test]
    fn test_from_spans() {
        let spans = vec![
            Span::styled("Hello,", Style::default().fg(Color::Red)),
            Span::styled(" world!", Style::default().fg(Color::Green)),
        ];
        assert_eq!(Line::from(Spans::from(spans.clone())), Line::from(spans));
    }

    #[test]
    fn test_into_string() {
        let line = Line::from(vec![
            Span::styled("Hello,", Style::default().fg(Color::Red)),
            Span::styled(" world!", Style::default().fg(Color::Green)),
        ]);
        let s: String = line.into();
        assert_eq!("Hello, world!", s);
    }

    #[test]
    fn test_alignment() {
        let line = Line::from("This is left").alignment(Alignment::Left);
        assert_eq!(Some(Alignment::Left), line.alignment);

        let line = Line::from("This is default");
        assert_eq!(None, line.alignment);
    }

    #[test]
    fn styled_graphemes() {
        const RED: Style = Style::new().fg(Color::Red);
        const GREEN: Style = Style::new().fg(Color::Green);
        const BLUE: Style = Style::new().fg(Color::Blue);
        const RED_ON_WHITE: Style = Style::new().fg(Color::Red).bg(Color::White);
        const GREEN_ON_WHITE: Style = Style::new().fg(Color::Green).bg(Color::White);
        const BLUE_ON_WHITE: Style = Style::new().fg(Color::Blue).bg(Color::White);

        let line = Line::from(vec![
            Span::styled("He", RED),
            Span::styled("ll", GREEN),
            Span::styled("o!", BLUE),
        ]);
        let styled_graphemes = line
            .styled_graphemes(Style::new().bg(Color::White))
            .collect::<Vec<StyledGrapheme>>();
        assert_eq!(
            styled_graphemes,
            vec![
                StyledGrapheme::new("H", RED_ON_WHITE),
                StyledGrapheme::new("e", RED_ON_WHITE),
                StyledGrapheme::new("l", GREEN_ON_WHITE),
                StyledGrapheme::new("l", GREEN_ON_WHITE),
                StyledGrapheme::new("o", BLUE_ON_WHITE),
                StyledGrapheme::new("!", BLUE_ON_WHITE),
            ],
        );
    }
}
