#![allow(deprecated)]
use super::{Span, Spans, Style};

#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct Line<'a> {
    pub spans: Vec<Span<'a>>,
}

impl<'a> Line<'a> {
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
        Self { spans }
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
    use crate::style::{Color, Modifier, Style};
    use crate::text::{Line, Span, Spans};

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
}
