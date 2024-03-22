use std::{borrow::Cow, fmt::Debug};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::StyledGrapheme;
use crate::prelude::*;

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
/// let span = Span::raw(String::from("test content"))
///     .green()
///     .on_yellow()
///     .italic();
/// ```
///
/// `Span` implements the [`Widget`] trait, which allows it to be rendered to a [`Buffer`]. Usually
/// apps will use the [`Paragraph`] widget instead of rendering `Span` directly, as it handles text
/// wrapping and alignment for you.
///
/// ```rust
/// use ratatui::prelude::*;
///
/// # fn render_frame(frame: &mut Frame) {
/// frame.render_widget("test content".green().on_yellow().italic(), frame.size());
/// # }
/// ```
/// [`Line`]: crate::text::Line
/// [`Paragraph`]: crate::widgets::Paragraph
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
    pub fn raw<T>(content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            content: content.into(),
            style: Style::default(),
        }
    }

    /// Create a span with the specified style.
    ///
    /// `content` accepts any type that is convertible to [`Cow<str>`] (e.g. `&str`, `String`,
    /// `&String`, etc.).
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let style = Style::new().yellow().on_green().italic();
    /// Span::styled("test content", style);
    /// Span::styled(String::from("test content"), style);
    /// ```
    pub fn styled<T, S>(content: T, style: S) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Style>,
    {
        Self {
            content: content.into(),
            style: style.into(),
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
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let mut span = Span::default().style(Style::new().green());
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Patches the style of the Span, adding modifiers from the given style.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let span = Span::styled("test content", Style::new().green().italic())
    ///     .patch_style(Style::new().red().on_yellow().bold());
    /// assert_eq!(span.style, Style::new().red().on_yellow().italic().bold());
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn patch_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = self.style.patch(style);
        self
    }

    /// Resets the style of the Span.
    ///
    /// This is Equivalent to calling `patch_style(Style::reset())`.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let span = Span::styled(
    ///     "Test Content",
    ///     Style::new().dark_gray().on_yellow().italic(),
    /// )
    /// .reset_style();
    /// assert_eq!(span.style, Style::reset());
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn reset_style(self) -> Self {
        self.patch_style(Style::reset())
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
    /// `base_style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`],
    /// or your own type that implements [`Into<Style>`]).
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::iter::Iterator;
    ///
    /// use ratatui::{prelude::*, text::StyledGrapheme};
    ///
    /// let span = Span::styled("Test", Style::new().green().italic());
    /// let style = Style::new().red().on_yellow();
    /// assert_eq!(
    ///     span.styled_graphemes(style)
    ///         .collect::<Vec<StyledGrapheme>>(),
    ///     vec![
    ///         StyledGrapheme::new("T", Style::new().green().on_yellow().italic()),
    ///         StyledGrapheme::new("e", Style::new().green().on_yellow().italic()),
    ///         StyledGrapheme::new("s", Style::new().green().on_yellow().italic()),
    ///         StyledGrapheme::new("t", Style::new().green().on_yellow().italic()),
    ///     ],
    /// );
    /// ```
    pub fn styled_graphemes<S: Into<Style>>(
        &'a self,
        base_style: S,
    ) -> impl Iterator<Item = StyledGrapheme<'a>> {
        let style = base_style.into().patch(self.style);
        self.content
            .as_ref()
            .graphemes(true)
            .filter(|g| *g != "\n")
            .map(move |g| StyledGrapheme { symbol: g, style })
    }

    /// Converts this Span into a left-aligned [`Line`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let line = "Test Content".green().italic().into_left_aligned_line();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn into_left_aligned_line(self) -> Line<'a> {
        Line::from(self).left_aligned()
    }

    #[allow(clippy::wrong_self_convention)]
    #[deprecated = "use into_left_aligned_line"]
    pub fn to_left_aligned_line(self) -> Line<'a> {
        self.into_left_aligned_line()
    }

    /// Converts this Span into a center-aligned [`Line`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let line = "Test Content".green().italic().into_centered_line();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn into_centered_line(self) -> Line<'a> {
        Line::from(self).centered()
    }

    #[allow(clippy::wrong_self_convention)]
    #[deprecated = "use into_centered_line"]
    pub fn to_centered_line(self) -> Line<'a> {
        self.into_centered_line()
    }

    /// Converts this Span into a right-aligned [`Line`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let line = "Test Content".green().italic().into_right_aligned_line();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn into_right_aligned_line(self) -> Line<'a> {
        Line::from(self).right_aligned()
    }

    #[allow(clippy::wrong_self_convention)]
    #[deprecated = "use into_right_aligned_line"]
    pub fn to_right_aligned_line(self) -> Line<'a> {
        self.into_right_aligned_line()
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
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl Widget for Span<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

impl WidgetRef for Span<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let Rect {
            x: mut current_x,
            y,
            width,
            ..
        } = area;
        let max_x = Ord::min(current_x.saturating_add(width), buf.area.right());
        for g in self.styled_graphemes(Style::default()) {
            let symbol_width = g.symbol.width();
            let next_x = current_x.saturating_add(symbol_width as u16);
            if next_x > max_x {
                break;
            }
            buf.get_mut(current_x, y)
                .set_symbol(g.symbol)
                .set_style(g.style);

            // multi-width graphemes must clear the cells of characters that are hidden by the
            // grapheme, otherwise the hidden characters will be re-rendered if the grapheme is
            // overwritten.
            for i in (current_x + 1)..next_x {
                buf.get_mut(i, y).reset();
                // it may seem odd that the style of the hidden cells are not set to the style of
                // the grapheme, but this is how the existing buffer.set_span() method works.
                // buf.get_mut(i, y).set_style(g.style);
            }
            current_x = next_x;
        }
    }
}

impl std::fmt::Display for Span<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let span = Span::styled("test content", Style::new().green()).reset_style();
        assert_eq!(span.style, Style::reset());
    }

    #[test]
    fn patch_style() {
        let span = Span::styled("test content", Style::new().green().on_yellow())
            .patch_style(Style::new().red().bold());
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
    #[test]
    fn display_span() {
        let span = Span::raw("test content");

        assert_eq!(format!("{span}"), "test content");
    }

    #[test]
    fn display_styled_span() {
        let stylized_span = Span::styled("stylized test content", Style::new().green());

        assert_eq!(format!("{stylized_span}"), "stylized test content");
    }

    mod widget {
        use super::*;
        use crate::assert_buffer_eq;

        #[test]
        fn render() {
            let style = Style::new().green().on_yellow();
            let span = Span::styled("test content", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 1));
            span.render(buf.area, &mut buf);

            let expected = Buffer::with_lines(vec![Line::from(vec![
                "test content".green().on_yellow(),
                "   ".into(),
            ])]);
            assert_buffer_eq!(buf, expected);
        }

        /// When the content of the span is longer than the area passed to render, the content
        /// should be truncated
        #[test]
        fn render_truncates_too_long_content() {
            let style = Style::new().green().on_yellow();
            let span = Span::styled("test content", style);

            let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
            span.render(Rect::new(0, 0, 5, 1), &mut buf);

            let mut expected = Buffer::with_lines(vec![Line::from("test      ")]);
            expected.set_style(Rect::new(0, 0, 5, 1), (Color::Green, Color::Yellow));

            assert_buffer_eq!(buf, expected);
        }

        /// When there is already a style set on the buffer, the style of the span should be
        /// patched with the existing style
        #[test]
        fn render_patches_existing_style() {
            let style = Style::new().green().on_yellow();
            let span = Span::styled("test content", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 1));
            buf.set_style(buf.area, Style::new().italic());
            span.render(buf.area, &mut buf);

            let expected = Buffer::with_lines(vec![Line::from(vec![
                "test content".green().on_yellow().italic(),
                "   ".italic(),
            ])]);
            assert_buffer_eq!(buf, expected);
        }

        /// When the span contains a multi-width grapheme, the grapheme will ensure that the cells
        /// of the hidden characters are cleared.
        #[test]
        fn render_multi_width_symbol() {
            let style = Style::new().green().on_yellow();
            let span = Span::styled("test 😃 content", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 1));
            span.render(buf.area, &mut buf);

            // The existing code in buffer.set_line() handles multi-width graphemes by clearing the
            // cells of the hidden characters. This test ensures that the existing behavior is
            // preserved.
            let expected = Buffer::with_lines(vec!["test 😃 content".green().on_yellow()]);
            assert_buffer_eq!(buf, expected);
        }

        /// When the span contains a multi-width grapheme that does not fit in the area passed to
        /// render, the entire grapheme will be truncated.
        #[test]
        fn render_multi_width_symbol_truncates_entire_symbol() {
            // the 😃 emoji is 2 columns wide so it will be truncated
            let style = Style::new().green().on_yellow();
            let span = Span::styled("test 😃 content", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 6, 1));
            span.render(buf.area, &mut buf);

            let expected = Buffer::with_lines(vec![Line::from(vec![
                "test ".green().on_yellow(),
                " ".into(),
            ])]);
            assert_buffer_eq!(buf, expected);
        }

        /// When the area passed to render overflows the buffer, the content should be truncated
        /// to fit the buffer.
        #[test]
        fn render_overflowing_area_truncates() {
            let style = Style::new().green().on_yellow();
            let span = Span::styled("test content", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 1));
            span.render(Rect::new(10, 0, 20, 1), &mut buf);

            let expected = Buffer::with_lines(vec![Line::from(vec![
                "          ".into(),
                "test ".green().on_yellow(),
            ])]);
            assert_buffer_eq!(buf, expected);
        }
    }

    #[test]
    fn left_aligned() {
        let span = Span::styled("Test Content", Style::new().green().italic());
        let line = span.into_left_aligned_line();
        assert_eq!(line.alignment, Some(Alignment::Left));
    }

    #[test]
    fn centered() {
        let span = Span::styled("Test Content", Style::new().green().italic());
        let line = span.into_centered_line();
        assert_eq!(line.alignment, Some(Alignment::Center));
    }

    #[test]
    fn right_aligned() {
        let span = Span::styled("Test Content", Style::new().green().italic());
        let line = span.into_right_aligned_line();
        assert_eq!(line.alignment, Some(Alignment::Right));
    }
}
