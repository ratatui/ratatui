use super::StyledGrapheme;
use crate::style::Style;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// A string where all graphemes have the same style.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub content: String,
    pub style: Style,
}

impl Span {
    /// Create a span with no style.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::text::Span;
    /// Span::raw("My text");
    /// Span::raw(String::from("My text"));
    /// ```
    pub fn raw<T>(content: T) -> Span
    where
        T: Into<String>,
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
    pub fn styled<T>(content: T, style: Style) -> Span
    where
        T: Into<String>,
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
    pub fn styled_graphemes(&self, base_style: Style) -> impl Iterator<Item = StyledGrapheme> + '_ {
        self.content
            .graphemes(true)
            .map(move |g| StyledGrapheme {
                symbol: g.to_owned(),
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

impl From<String> for Span {
    fn from(s: String) -> Span {
        Span::raw(s)
    }
}

impl From<&str> for Span {
    fn from(s: &str) -> Span {
        Span::raw(s)
    }
}
