use std::{borrow::Cow, fmt::Debug};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::StyledGrapheme;
use crate::style::{Style, Styled};

/// Represents a part of a line that is contiguous and where all characters share the same style.
///
/// A `Span` is the smallest unit of text that can be styled. It is usually combined in the [`Line`]
/// type to represent a line of text where each `Span` may have a different style.
///
/// # Constructor Methods
///
/// - [`Span::default`] creates an span with empty content and the default style.
/// - [`Span::raw`] creates an span with the specified content and the default style.
/// - [`Span::styled`] creates an span with the specified content and style.
///
/// # Setter Methods
///
/// These methods are fluent setters. They return a new `Span` with the specified property set.
///
/// - [`Span::content`] sets the content of the span.
/// - [`Span::style`] sets the style of the span.
///
/// # Other Methods
///
/// - [`Span::patch_style`] patches the style of the span, adding modifiers from the given style.
/// - [`Span::reset_style`] resets the style of the span.
/// - [`Span::width`] returns the unicode width of the content held by this span.
/// - [`Span::styled_graphemes`] returns an iterator over the graphemes held by this span.
///
/// # Examples
///
/// A `Span` with `style` set to [`Style::default()`] can be created from a `&str`, a `String`, or
/// any type convertible to [`Cow<str>`].
///
/// ```rust
/// use ratatui::prelude::*;
///
/// let span = Span::raw("test content");
/// let span = Span::raw(String::from("test content"));
/// let span = Span::from("test content");
/// let span = Span::from(String::from("test content"));
/// let span: Span = "test content".into();
/// let span: Span = String::from("test content").into();
/// ```
///
/// Styled spans can be created using [`Span::styled`] or by converting strings using methods from
/// the [`Stylize`] trait.
///
/// ```rust
/// use ratatui::prelude::*;
///
/// let span = Span::styled("test content", Style::new().green());
/// let span = Span::styled(String::from("test content"), Style::new().green());
///
/// // using Stylize trait shortcuts
/// let span = "test content".green();
/// let span = String::from("test content").green();
/// ```
///
/// `Span` implements the [`Styled`] trait, which allows it to be styled using the shortcut methods
/// defined in the [`Stylize`] trait.
///
/// ```rust
/// use ratatui::prelude::*;
///
/// let span = Span::raw("test content").green().on_yellow().italic();
/// let span = Span::raw(String::from("test content")).green().on_yellow().italic();
/// ```
///
/// [`Line`]: crate::text::Line
/// [`Stylize`]: crate::style::Stylize
/// [`Cow<str>`]: std::borrow::Cow
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Span<'a> {
    /// The content of the span as a Clone-on-write string.
    pub content: Cow<'a, str>,
    /// The style of the span.
    pub style: Style,
}

impl<'a> Span<'a> {
    /// Create a span with the default style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// Span::raw("test content");
    /// Span::raw(String::from("test content"));
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

    /// Create a span with the specified style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let style = Style::new().yellow().on_green().italic();
    /// Span::styled("test content", style);
    /// Span::styled(String::from("test content"), style);
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

    /// Sets the content of the span.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// Accepts any type that can be converted to [`Cow<str>`] (e.g. `&str`, `String`, `&String`,
    /// etc.).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let mut span = Span::default().content("content");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn content<T>(mut self, content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.content = content.into();
        self
    }

    /// Sets the style of the span.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// In contrast to [`Span::patch_style`], this method replaces the style of the span instead of
    /// patching it.
    ///
    /// Accepts any type that can be converted to [`Style`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let mut span = Span::default().style(Style::new().green());
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<T>(mut self, style: T) -> Self
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Patches the style of the Span, adding modifiers from the given style.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let mut span = Span::styled("test content", Style::new().green().italic());
    /// span.patch_style(Style::new().red().on_yellow().bold());
    /// assert_eq!(span.style, Style::new().red().on_yellow().italic().bold());
    /// ```
    pub fn patch_style(&mut self, style: Style) {
        self.style = self.style.patch(style);
    }

    /// Resets the style of the Span.
    ///
    /// This is Equivalent to calling `patch_style(Style::reset())`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let mut span = Span::styled("Test Content", Style::new().green().on_yellow().italic());
    /// span.reset_style();
    /// assert_eq!(span.style, Style::reset());
    /// ```
    pub fn reset_style(&mut self) {
        self.patch_style(Style::reset());
    }

    /// Returns the unicode width of the content held by this span.
    pub fn width(&self) -> usize {
        self.content.width()
    }

    /// Returns an iterator over the graphemes held by this span.
    ///
    /// `base_style` is the [`Style`] that will be patched with the `Span`'s `style` to get the
    /// resulting [`Style`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::iter::Iterator;
    /// use ratatui::{prelude::*, text::StyledGrapheme};
    ///
    /// let span = Span::styled("Test", Style::new().green().italic());
    /// let style = Style::new().red().on_yellow();
    /// assert_eq!(
    ///     span.styled_graphemes(style).collect::<Vec<StyledGrapheme>>(),
    ///     vec![
    ///         StyledGrapheme::new("T", Style::new().green().on_yellow().italic()),
    ///         StyledGrapheme::new("e", Style::new().green().on_yellow().italic()),
    ///         StyledGrapheme::new("s", Style::new().green().on_yellow().italic()),
    ///         StyledGrapheme::new("t", Style::new().green().on_yellow().italic()),
    ///     ],
    /// );
    /// ```
    pub fn styled_graphemes(
        &'a self,
        base_style: Style,
    ) -> impl Iterator<Item = StyledGrapheme<'a>> {
        self.content
            .as_ref()
            .graphemes(true)
            .filter(|g| *g != "\n")
            .map(move |g| StyledGrapheme {
                symbol: g,
                style: base_style.patch(self.style),
            })
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
    use crate::style::Stylize;

    #[test]
    fn default() {
        let span = Span::default();
        assert_eq!(span.content, Cow::Borrowed(""));
        assert_eq!(span.style, Style::default());
    }

    #[test]
    fn raw_str() {
        let span = Span::raw("test content");
        assert_eq!(span.content, Cow::Borrowed("test content"));
        assert_eq!(span.style, Style::default());
    }

    #[test]
    fn raw_string() {
        let content = String::from("test content");
        let span = Span::raw(content.clone());
        assert_eq!(span.content, Cow::Owned::<str>(content));
        assert_eq!(span.style, Style::default());
    }

    #[test]
    fn styled_str() {
        let style = Style::new().red();
        let span = Span::styled("test content", style);
        assert_eq!(span.content, Cow::Borrowed("test content"));
        assert_eq!(span.style, Style::new().red());
    }

    #[test]
    fn styled_string() {
        let content = String::from("test content");
        let style = Style::new().green();
        let span = Span::styled(content.clone(), style);
        assert_eq!(span.content, Cow::Owned::<str>(content));
        assert_eq!(span.style, style);
    }

    #[test]
    fn set_content() {
        let span = Span::default().content("test content");
        assert_eq!(span.content, Cow::Borrowed("test content"));
    }

    #[test]
    fn set_style() {
        let span = Span::default().style(Style::new().green());
        assert_eq!(span.style, Style::new().green());
    }

    #[test]
    fn from_ref_str_borrowed_cow() {
        let content = "test content";
        let span = Span::from(content);
        assert_eq!(span.content, Cow::Borrowed(content));
        assert_eq!(span.style, Style::default());
    }

    #[test]
    fn from_string_ref_str_borrowed_cow() {
        let content = String::from("test content");
        let span = Span::from(content.as_str());
        assert_eq!(span.content, Cow::Borrowed(content.as_str()));
        assert_eq!(span.style, Style::default());
    }

    #[test]
    fn from_string_owned_cow() {
        let content = String::from("test content");
        let span = Span::from(content.clone());
        assert_eq!(span.content, Cow::Owned::<str>(content));
        assert_eq!(span.style, Style::default());
    }

    #[test]
    fn from_ref_string_borrowed_cow() {
        let content = String::from("test content");
        let span = Span::from(&content);
        assert_eq!(span.content, Cow::Borrowed(content.as_str()));
        assert_eq!(span.style, Style::default());
    }

    #[test]
    fn reset_style() {
        let mut span = Span::styled("test content", Style::new().green());
        span.reset_style();
        assert_eq!(span.style, Style::reset());
    }

    #[test]
    fn patch_style() {
        let mut span = Span::styled("test content", Style::new().green().on_yellow());
        span.patch_style(Style::new().red().bold());
        assert_eq!(span.style, Style::new().red().on_yellow().bold());
    }

    #[test]
    fn width() {
        assert_eq!(Span::raw("").width(), 0);
        assert_eq!(Span::raw("test").width(), 4);
        assert_eq!(Span::raw("test content").width(), 12);
    }

    #[test]
    fn stylize() {
        let span = Span::raw("test content").green();
        assert_eq!(span.content, Cow::Borrowed("test content"));
        assert_eq!(span.style, Style::new().green());

        let span = Span::styled("test content", Style::new().green());
        let stylized = span.on_yellow().bold();
        assert_eq!(stylized.content, Cow::Borrowed("test content"));
        assert_eq!(stylized.style, Style::new().green().on_yellow().bold());
    }
}
