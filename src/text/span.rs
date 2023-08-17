use std::{borrow::Cow, fmt::Debug};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::StyledGrapheme;
use crate::style::{Style, Styled};

/// A string where all graphemes have the same style.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
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
    /// let span = Span::styled("Text", Style::default().fg(Color::Yellow));
    /// let style = Style::default().fg(Color::Green).bg(Color::Black);
    /// assert_eq!(
    ///     span.styled_graphemes(style).collect::<Vec<StyledGrapheme>>(),
    ///     vec![
    ///         StyledGrapheme::new("T", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///         StyledGrapheme::new("e", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///         StyledGrapheme::new("x", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///         StyledGrapheme::new("t", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///     ],
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

impl<'a, T> From<T> for Span<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(s: T) -> Self {
        Span::raw(s.into())
    }
}

impl<'a> Styled for Span<'a> {
    type Item = Span<'a>;
    fn style(&self) -> Style {
        self.style
    }

    fn set_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_ref_str_borrowed_cow() {
        let content = "some string";
        let span = Span::from(content);
        assert_eq!(span.content, Cow::Borrowed(content));
    }

    #[test]
    fn from_string_ref_str_borrowed_cow() {
        let content = String::from("some string");
        let span = Span::from(content.as_str());
        assert_eq!(span.content, Cow::Borrowed(content.as_str()));
    }

    #[test]
    fn from_string_owned_cow() {
        let content = String::from("some string");
        let content_clone = content.clone();
        let span = Span::from(content);
        assert_eq!(span.content, Cow::Owned::<str>(content_clone));
    }

    #[test]
    fn from_ref_string_borrowed_cow() {
        let content = String::from("some string");
        let span = Span::from(&content);
        assert_eq!(span.content, Cow::Borrowed(content.as_str()));
    }

    #[test]
    fn reset_should_set_style_reset() {
        let mut span = Span::styled("test", Style::default().fg(crate::style::Color::Green));

        assert_eq!(span.style, Style::default().fg(crate::style::Color::Green));

        span.reset_style();

        assert_eq!(span.style, Style::reset());
        assert_ne!(span.style, Style::default());
    }

    #[test]
    fn patch_style() {
        let mut span = Span::styled("test", Style::default().bg(crate::style::Color::Cyan));

        assert_eq!(span.style, Style::default().bg(crate::style::Color::Cyan));

        span.patch_style(Style::default().fg(crate::style::Color::Green));

        assert_eq!(
            span.style,
            Style::default()
                .bg(crate::style::Color::Cyan)
                .fg(crate::style::Color::Green)
        );
    }
}
