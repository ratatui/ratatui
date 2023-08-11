#![allow(deprecated)]

use super::{Span, Style};
use crate::{layout::Alignment, text::Line};

/// A string composed of clusters of graphemes, each with their own style.
///
/// `Spans` has been deprecated in favor of `Line`, and will be removed in the
/// future. All methods that accept Spans have been replaced with methods that
/// accept Into<Line<'a>> (which is implemented on `Spans`) to allow users of
/// this crate to gradually transition to Line.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
#[deprecated(note = "Use `ratatui::text::Line` instead")]
pub struct Spans<'a>(pub Vec<Span<'a>>);

impl<'a> Spans<'a> {
    /// Returns the width of the underlying string.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::text::{Span, Spans};
    /// # use ratatui::style::{Color, Style};
    /// let spans = Spans::from(vec![
    ///     Span::styled("My", Style::default().fg(Color::Yellow)),
    ///     Span::raw(" text"),
    /// ]);
    /// assert_eq!(7, spans.width());
    /// ```
    pub fn width(&self) -> usize {
        self.0.iter().map(Span::width).sum()
    }

    /// Patches the style of each Span in an existing Spans, adding modifiers from the given style.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::text::{Span, Spans};
    /// # use ratatui::style::{Color, Style, Modifier};
    /// let style = Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
    /// let mut raw_spans = Spans::from(vec![
    ///     Span::raw("My"),
    ///     Span::raw(" text"),
    /// ]);
    /// let mut styled_spans = Spans::from(vec![
    ///     Span::styled("My", style),
    ///     Span::styled(" text", style),
    /// ]);
    ///
    /// assert_ne!(raw_spans, styled_spans);
    ///
    /// raw_spans.patch_style(style);
    /// assert_eq!(raw_spans, styled_spans);
    /// ```
    pub fn patch_style(&mut self, style: Style) {
        for span in &mut self.0 {
            span.patch_style(style);
        }
    }

    /// Resets the style of each Span in the Spans.
    /// Equivalent to calling `patch_style(Style::reset())`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::text::{Span, Spans};
    /// # use ratatui::style::{Color, Style, Modifier};
    /// let mut spans = Spans::from(vec![
    ///     Span::styled("My", Style::default().fg(Color::Yellow)),
    ///     Span::styled(" text", Style::default().add_modifier(Modifier::BOLD)),
    /// ]);
    ///
    /// spans.reset_style();
    /// assert_eq!(Style::reset(), spans.0[0].style);
    /// assert_eq!(Style::reset(), spans.0[1].style);
    /// ```
    pub fn reset_style(&mut self) {
        for span in &mut self.0 {
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
    /// # use ratatui::text::{Span, Spans};
    /// # use ratatui::style::{Color, Style, Modifier};
    /// let mut line = Spans::from("Hi, what's up?").alignment(Alignment::Right);
    /// assert_eq!(Some(Alignment::Right), line.alignment)
    /// ```
    pub fn alignment(self, alignment: Alignment) -> Line<'a> {
        let line = Line::from(self);
        line.alignment(alignment)
    }
}

impl<'a> From<String> for Spans<'a> {
    fn from(s: String) -> Spans<'a> {
        Spans(vec![Span::from(s)])
    }
}

impl<'a> From<&'a str> for Spans<'a> {
    fn from(s: &'a str) -> Spans<'a> {
        Spans(vec![Span::from(s)])
    }
}

impl<'a> From<Vec<Span<'a>>> for Spans<'a> {
    fn from(spans: Vec<Span<'a>>) -> Spans<'a> {
        Spans(spans)
    }
}

impl<'a> From<Span<'a>> for Spans<'a> {
    fn from(span: Span<'a>) -> Spans<'a> {
        Spans(vec![span])
    }
}

impl<'a> From<Spans<'a>> for String {
    fn from(line: Spans<'a>) -> String {
        line.0.iter().fold(String::new(), |mut acc, s| {
            acc.push_str(s.content.as_ref());
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        style::{Color, Modifier, Style},
        text::{Span, Spans},
    };

    #[test]
    fn test_width() {
        let spans = Spans::from(vec![
            Span::styled("My", Style::default().fg(Color::Yellow)),
            Span::raw(" text"),
        ]);
        assert_eq!(7, spans.width());

        let empty_spans = Spans::default();
        assert_eq!(0, empty_spans.width());
    }

    #[test]
    fn test_patch_style() {
        let style = Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::ITALIC);
        let mut raw_spans = Spans::from(vec![Span::raw("My"), Span::raw(" text")]);
        let styled_spans = Spans::from(vec![
            Span::styled("My", style),
            Span::styled(" text", style),
        ]);

        assert_ne!(raw_spans, styled_spans);

        raw_spans.patch_style(style);
        assert_eq!(raw_spans, styled_spans);
    }

    #[test]
    fn test_reset_style() {
        let mut spans = Spans::from(vec![
            Span::styled("My", Style::default().fg(Color::Yellow)),
            Span::styled(" text", Style::default().add_modifier(Modifier::BOLD)),
        ]);

        spans.reset_style();
        assert_eq!(Style::reset(), spans.0[0].style);
        assert_eq!(Style::reset(), spans.0[1].style);
    }

    #[test]
    fn test_from_string() {
        let s = String::from("Hello, world!");
        let spans = Spans::from(s);
        assert_eq!(vec![Span::from("Hello, world!")], spans.0);
    }

    #[test]
    fn test_from_str() {
        let s = "Hello, world!";
        let spans = Spans::from(s);
        assert_eq!(vec![Span::from("Hello, world!")], spans.0);
    }

    #[test]
    fn test_from_vec() {
        let spans_vec = vec![
            Span::styled("Hello,", Style::default().fg(Color::Red)),
            Span::styled(" world!", Style::default().fg(Color::Green)),
        ];
        let spans = Spans::from(spans_vec.clone());
        assert_eq!(spans_vec, spans.0);
    }

    #[test]
    fn test_from_span() {
        let span = Span::styled("Hello, world!", Style::default().fg(Color::Yellow));
        let spans = Spans::from(span.clone());
        assert_eq!(vec![span], spans.0);
    }

    #[test]
    fn test_into_string() {
        let spans = Spans::from(vec![
            Span::styled("Hello,", Style::default().fg(Color::Red)),
            Span::styled(" world!", Style::default().fg(Color::Green)),
        ]);
        let s: String = spans.into();
        assert_eq!("Hello, world!", s);
    }
}
