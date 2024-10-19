#![deny(missing_docs)]
#![warn(clippy::pedantic, clippy::nursery, clippy::arithmetic_side_effects)]
use std::{borrow::Cow, fmt};

use unicode_truncate::UnicodeTruncateStr;

use crate::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Style, Styled},
    text::{Span, StyledGrapheme, Text},
    widgets::{Widget, WidgetRef},
};

/// A line of text, consisting of one or more [`Span`]s.
///
/// [`Line`]s are used wherever text is displayed in the terminal and represent a single line of
/// text. When a [`Line`] is rendered, it is rendered as a single line of text, with each [`Span`]
/// being rendered in order (left to right).
///
/// Any newlines in the content are removed when creating a [`Line`] using the constructor or
/// conversion methods.
///
/// # Constructor Methods
///
/// - [`Line::default`] creates a line with empty content and the default style.
/// - [`Line::raw`] creates a line with the given content and the default style.
/// - [`Line::styled`] creates a line with the given content and style.
///
/// # Conversion Methods
///
/// - [`Line::from`] creates a `Line` from a [`String`].
/// - [`Line::from`] creates a `Line` from a [`&str`].
/// - [`Line::from`] creates a `Line` from a [`Vec`] of [`Span`]s.
/// - [`Line::from`] creates a `Line` from single [`Span`].
/// - [`String::from`] converts a line into a [`String`].
/// - [`Line::from_iter`] creates a line from an iterator of items that are convertible to [`Span`].
///
/// # Setter Methods
///
/// These methods are fluent setters. They return a `Line` with the property set.
///
/// - [`Line::spans`] sets the content of the line.
/// - [`Line::style`] sets the style of the line.
/// - [`Line::alignment`] sets the alignment of the line.
/// - [`Line::left_aligned`] sets the alignment of the line to [`Alignment::Left`].
/// - [`Line::centered`] sets the alignment of the line to [`Alignment::Center`].
/// - [`Line::right_aligned`] sets the alignment of the line to [`Alignment::Right`].
///
/// # Iteration Methods
///
/// - [`Line::iter`] returns an iterator over the spans of this line.
/// - [`Line::iter_mut`] returns a mutable iterator over the spans of this line.
/// - [`Line::into_iter`] returns an iterator over the spans of this line.
///
/// # Other Methods
///
/// - [`Line::patch_style`] patches the style of the line, adding modifiers from the given style.
/// - [`Line::reset_style`] resets the style of the line.
/// - [`Line::width`] returns the unicode width of the content held by this line.
/// - [`Line::styled_graphemes`] returns an iterator over the graphemes held by this line.
/// - [`Line::push_span`] adds a span to the line.
///
/// # Compatibility Notes
///
/// Before v0.26.0, [`Line`] did not have a `style` field and instead relied on only the styles that
/// were set on each [`Span`] contained in the `spans` field. The [`Line::patch_style`] method was
/// the only way to set the overall style for individual lines. For this reason, this field may not
/// be supported yet by all widgets (outside of the `ratatui` crate itself).
///
/// # Examples
///
/// ## Creating Lines
/// [`Line`]s can be created from [`Span`]s, [`String`]s, and [`&str`]s. They can be styled with a
/// [`Style`].
///
/// ```rust
/// use ratatui::{
///     style::{Color, Modifier, Style, Stylize},
///     text::{Line, Span},
/// };
///
/// let style = Style::new().yellow();
/// let line = Line::raw("Hello, world!").style(style);
/// let line = Line::styled("Hello, world!", style);
/// let line = Line::styled("Hello, world!", (Color::Yellow, Modifier::BOLD));
///
/// let line = Line::from("Hello, world!");
/// let line = Line::from(String::from("Hello, world!"));
/// let line = Line::from(vec![
///     Span::styled("Hello", Style::new().blue()),
///     Span::raw(" world!"),
/// ]);
/// ```
///
/// ## Styling Lines
///
/// The line's [`Style`] is used by the rendering widget to determine how to style the line. Each
/// [`Span`] in the line will be styled with the [`Style`] of the line, and then with its own
/// [`Style`]. If the line is longer than the available space, the style is applied to the entire
/// line, and the line is truncated. `Line` also implements [`Styled`] which means you can use the
/// methods of the [`Stylize`] trait.
///
/// ```rust
/// use ratatui::{
///     style::{Color, Modifier, Style, Stylize},
///     text::Line,
/// };
///
/// let line = Line::from("Hello world!").style(Style::new().yellow().italic());
/// let line = Line::from("Hello world!").style(Color::Yellow);
/// let line = Line::from("Hello world!").style((Color::Yellow, Color::Black));
/// let line = Line::from("Hello world!").style((Color::Yellow, Modifier::ITALIC));
/// let line = Line::from("Hello world!").yellow().italic();
/// ```
///
/// ## Aligning Lines
///
/// The line's [`Alignment`] is used by the rendering widget to determine how to align the line
/// within the available space. If the line is longer than the available space, the alignment is
/// ignored and the line is truncated.
///
/// ```rust
/// use ratatui::{layout::Alignment, text::Line};
///
/// let line = Line::from("Hello world!").alignment(Alignment::Right);
/// let line = Line::from("Hello world!").centered();
/// let line = Line::from("Hello world!").left_aligned();
/// let line = Line::from("Hello world!").right_aligned();
/// ```
///
/// ## Rendering Lines
///
/// `Line` implements the [`Widget`] trait, which means it can be rendered to a [`Buffer`].
///
/// ```rust
/// use ratatui::{
///     buffer::Buffer,
///     layout::Rect,
///     style::{Style, Stylize},
///     text::Line,
///     widgets::Widget,
///     Frame,
/// };
///
/// # fn render(area: Rect, buf: &mut Buffer) {
/// // in another widget's render method
/// let line = Line::from("Hello world!").style(Style::new().yellow().italic());
/// line.render(area, buf);
/// # }
///
/// # fn draw(frame: &mut Frame, area: Rect) {
/// // in a terminal.draw closure
/// let line = Line::from("Hello world!").style(Style::new().yellow().italic());
/// frame.render_widget(line, area);
/// # }
/// ```
/// ## Rendering Lines with a Paragraph widget
///
/// Usually apps will use the [`Paragraph`] widget instead of rendering a [`Line`] directly as it
/// provides more functionality.
///
/// ```rust
/// use ratatui::{
///     buffer::Buffer,
///     layout::Rect,
///     style::Stylize,
///     text::Line,
///     widgets::{Paragraph, Widget, Wrap},
/// };
///
/// # fn render(area: Rect, buf: &mut Buffer) {
/// let line = Line::from("Hello world!").yellow().italic();
/// Paragraph::new(line)
///     .wrap(Wrap { trim: true })
///     .render(area, buf);
/// # }
/// ```
///
/// [`Paragraph`]: crate::widgets::Paragraph
/// [`Stylize`]: crate::style::Stylize
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Line<'a> {
    /// The style of this line of text.
    pub style: Style,

    /// The alignment of this line of text.
    pub alignment: Option<Alignment>,

    /// The spans that make up this line of text.
    pub spans: Vec<Span<'a>>,
}

impl fmt::Debug for Line<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.spans.is_empty() {
            f.write_str("Line::default()")?;
        } else if self.spans.len() == 1 && self.spans[0].style == Style::default() {
            f.write_str(r#"Line::from(""#)?;
            f.write_str(&self.spans[0].content)?;
            f.write_str(r#"")"#)?;
        } else if self.spans.len() == 1 {
            f.write_str("Line::from(")?;
            self.spans[0].fmt(f)?;
            f.write_str(")")?;
        } else {
            f.write_str("Line::from_iter(")?;
            f.debug_list().entries(&self.spans).finish()?;
            f.write_str(")")?;
        }
        self.style.fmt_stylize(f)?;
        match self.alignment {
            Some(Alignment::Left) => write!(f, ".left_aligned()"),
            Some(Alignment::Center) => write!(f, ".centered()"),
            Some(Alignment::Right) => write!(f, ".right_aligned()"),
            None => Ok(()),
        }
    }
}

fn cow_to_spans<'a>(content: impl Into<Cow<'a, str>>) -> Vec<Span<'a>> {
    match content.into() {
        Cow::Borrowed(s) => s.lines().map(Span::raw).collect(),
        Cow::Owned(s) => s.lines().map(|v| Span::raw(v.to_string())).collect(),
    }
}

impl<'a> Line<'a> {
    /// Create a line with the default style.
    ///
    /// `content` can be any type that is convertible to [`Cow<str>`] (e.g. [`&str`], [`String`],
    /// [`Cow<str>`], or your own type that implements [`Into<Cow<str>>`]).
    ///
    /// A [`Line`] can specify a [`Style`], which will be applied before the style of each [`Span`]
    /// in the line.
    ///
    /// Any newlines in the content are removed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::borrow::Cow;
    ///
    /// use ratatui::text::Line;
    ///
    /// Line::raw("test content");
    /// Line::raw(String::from("test content"));
    /// Line::raw(Cow::from("test content"));
    /// ```
    pub fn raw<T>(content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            spans: cow_to_spans(content),
            ..Default::default()
        }
    }

    /// Create a line with the given style.
    ///
    /// `content` can be any type that is convertible to [`Cow<str>`] (e.g. [`&str`], [`String`],
    /// [`Cow<str>`], or your own type that implements [`Into<Cow<str>>`]).
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// # Examples
    ///
    /// Any newlines in the content are removed.
    ///
    /// ```rust
    /// use std::borrow::Cow;
    ///
    /// use ratatui::{
    ///     style::{Style, Stylize},
    ///     text::Line,
    /// };
    ///
    /// let style = Style::new().yellow().italic();
    /// Line::styled("My text", style);
    /// Line::styled(String::from("My text"), style);
    /// Line::styled(Cow::from("test content"), style);
    /// ```
    ///
    /// [`Color`]: crate::style::Color
    pub fn styled<T, S>(content: T, style: S) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Style>,
    {
        Self {
            spans: cow_to_spans(content),
            style: style.into(),
            ..Default::default()
        }
    }

    /// Sets the spans of this line of text.
    ///
    /// `spans` accepts any iterator that yields items that are convertible to [`Span`] (e.g.
    /// [`&str`], [`String`], [`Span`], or your own type that implements [`Into<Span>`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::{style::Stylize, text::Line};
    ///
    /// let line = Line::default().spans(vec!["Hello".blue(), " world!".green()]);
    /// let line = Line::default().spans([1, 2, 3].iter().map(|i| format!("Item {}", i)));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn spans<I>(mut self, spans: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Span<'a>>,
    {
        self.spans = spans.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the style of this line of text.
    ///
    /// Defaults to [`Style::default()`].
    ///
    /// Note: This field was added in v0.26.0. Prior to that, the style of a line was determined
    /// only by the style of each [`Span`] contained in the line. For this reason, this field may
    /// not be supported by all widgets (outside of the `ratatui` crate itself).
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// # Examples
    /// ```rust
    /// use ratatui::{
    ///     style::{Style, Stylize},
    ///     text::Line,
    /// };
    ///
    /// let mut line = Line::from("foo").style(Style::new().red());
    /// ```
    ///
    /// [`Color`]: crate::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the target alignment for this line of text.
    ///
    /// Defaults to: [`None`], meaning the alignment is determined by the rendering widget.
    /// Setting the alignment of a Line generally overrides the alignment of its
    /// parent Text or Widget.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::{layout::Alignment, text::Line};
    ///
    /// let mut line = Line::from("Hi, what's up?");
    /// assert_eq!(None, line.alignment);
    /// assert_eq!(
    ///     Some(Alignment::Right),
    ///     line.alignment(Alignment::Right).alignment
    /// )
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn alignment(self, alignment: Alignment) -> Self {
        Self {
            alignment: Some(alignment),
            ..self
        }
    }

    /// Left-aligns this line of text.
    ///
    /// Convenience shortcut for `Line::alignment(Alignment::Left)`.
    /// Setting the alignment of a Line generally overrides the alignment of its
    /// parent Text or Widget, with the default alignment being inherited from the parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::text::Line;
    ///
    /// let line = Line::from("Hi, what's up?").left_aligned();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn left_aligned(self) -> Self {
        self.alignment(Alignment::Left)
    }

    /// Center-aligns this line of text.
    ///
    /// Convenience shortcut for `Line::alignment(Alignment::Center)`.
    /// Setting the alignment of a Line generally overrides the alignment of its
    /// parent Text or Widget, with the default alignment being inherited from the parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::text::Line;
    ///
    /// let line = Line::from("Hi, what's up?").centered();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn centered(self) -> Self {
        self.alignment(Alignment::Center)
    }

    /// Right-aligns this line of text.
    ///
    /// Convenience shortcut for `Line::alignment(Alignment::Right)`.
    /// Setting the alignment of a Line generally overrides the alignment of its
    /// parent Text or Widget, with the default alignment being inherited from the parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::text::Line;
    ///
    /// let line = Line::from("Hi, what's up?").right_aligned();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn right_aligned(self) -> Self {
        self.alignment(Alignment::Right)
    }

    /// Returns the width of the underlying string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::{style::Stylize, text::Line};
    ///
    /// let line = Line::from(vec!["Hello".blue(), " world!".green()]);
    /// assert_eq!(12, line.width());
    /// ```
    pub fn width(&self) -> usize {
        self.spans.iter().map(Span::width).sum()
    }

    /// Returns an iterator over the graphemes held by this line.
    ///
    /// `base_style` is the [`Style`] that will be patched with each grapheme [`Style`] to get
    /// the resulting [`Style`].
    ///
    /// `base_style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`],
    /// or your own type that implements [`Into<Style>`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::iter::Iterator;
    ///
    /// use ratatui::{
    ///     style::{Color, Style},
    ///     text::{Line, StyledGrapheme},
    /// };
    ///
    /// let line = Line::styled("Text", Style::default().fg(Color::Yellow));
    /// let style = Style::default().fg(Color::Green).bg(Color::Black);
    /// assert_eq!(
    ///     line.styled_graphemes(style)
    ///         .collect::<Vec<StyledGrapheme>>(),
    ///     vec![
    ///         StyledGrapheme::new("T", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///         StyledGrapheme::new("e", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///         StyledGrapheme::new("x", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///         StyledGrapheme::new("t", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///     ]
    /// );
    /// ```
    ///
    /// [`Color`]: crate::style::Color
    pub fn styled_graphemes<S: Into<Style>>(
        &'a self,
        base_style: S,
    ) -> impl Iterator<Item = StyledGrapheme<'a>> {
        let style = base_style.into().patch(self.style);
        self.spans
            .iter()
            .flat_map(move |span| span.styled_graphemes(style))
    }

    /// Patches the style of this Line, adding modifiers from the given style.
    ///
    /// This is useful for when you want to apply a style to a line that already has some styling.
    /// In contrast to [`Line::style`], this method will not overwrite the existing style, but
    /// instead will add the given style's modifiers to this Line's style.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::{
    ///     style::{Color, Modifier},
    ///     text::Line,
    /// };
    ///
    /// let line = Line::styled("My text", Modifier::ITALIC);
    ///
    /// let styled_line = Line::styled("My text", (Color::Yellow, Modifier::ITALIC));
    ///
    /// assert_eq!(styled_line, line.patch_style(Color::Yellow));
    /// ```
    ///
    /// [`Color`]: crate::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn patch_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = self.style.patch(style);
        self
    }

    /// Resets the style of this Line.
    ///
    /// Equivalent to calling `patch_style(Style::reset())`.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # let style = Style::default().yellow();
    /// use ratatui::{
    ///     style::{Style, Stylize},
    ///     text::Line,
    /// };
    ///
    /// let line = Line::styled("My text", style);
    ///
    /// assert_eq!(Style::reset(), line.reset_style().style);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn reset_style(self) -> Self {
        self.patch_style(Style::reset())
    }

    /// Returns an iterator over the spans of this line.
    pub fn iter(&self) -> std::slice::Iter<Span<'a>> {
        self.spans.iter()
    }

    /// Returns a mutable iterator over the spans of this line.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Span<'a>> {
        self.spans.iter_mut()
    }

    /// Adds a span to the line.
    ///
    /// `span` can be any type that is convertible into a `Span`. For example, you can pass a
    /// `&str`, a `String`, or a `Span`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui::text::{Line, Span};
    ///
    /// let mut line = Line::from("Hello, ");
    /// line.push_span(Span::raw("world!"));
    /// line.push_span(" How are you?");
    /// ```
    pub fn push_span<T: Into<Span<'a>>>(&mut self, span: T) {
        self.spans.push(span.into());
    }
}

impl<'a> IntoIterator for Line<'a> {
    type Item = Span<'a>;
    type IntoIter = std::vec::IntoIter<Span<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.spans.into_iter()
    }
}

impl<'a> IntoIterator for &'a Line<'a> {
    type Item = &'a Span<'a>;
    type IntoIter = std::slice::Iter<'a, Span<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Line<'a> {
    type Item = &'a mut Span<'a>;
    type IntoIter = std::slice::IterMut<'a, Span<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a> From<String> for Line<'a> {
    fn from(s: String) -> Self {
        Self::raw(s)
    }
}

impl<'a> From<&'a str> for Line<'a> {
    fn from(s: &'a str) -> Self {
        Self::raw(s)
    }
}

impl<'a> From<Cow<'a, str>> for Line<'a> {
    fn from(s: Cow<'a, str>) -> Self {
        Self::raw(s)
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
    fn from(line: Line<'a>) -> Self {
        line.iter().fold(Self::new(), |mut acc, s| {
            acc.push_str(s.content.as_ref());
            acc
        })
    }
}

impl<'a, T> FromIterator<T> for Line<'a>
where
    T: Into<Span<'a>>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from(iter.into_iter().map(Into::into).collect::<Vec<_>>())
    }
}

/// Adds a `Span` to a `Line`, returning a new `Line` with the `Span` added.
impl<'a> std::ops::Add<Span<'a>> for Line<'a> {
    type Output = Self;

    fn add(mut self, rhs: Span<'a>) -> Self::Output {
        self.spans.push(rhs);
        self
    }
}

/// Adds two `Line`s together, returning a new `Text` with the contents of the two `Line`s.
impl<'a> std::ops::Add<Self> for Line<'a> {
    type Output = Text<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        Text::from(vec![self, rhs])
    }
}

impl<'a> std::ops::AddAssign<Span<'a>> for Line<'a> {
    fn add_assign(&mut self, rhs: Span<'a>) {
        self.spans.push(rhs);
    }
}

impl<'a> Extend<Span<'a>> for Line<'a> {
    fn extend<T: IntoIterator<Item = Span<'a>>>(&mut self, iter: T) {
        self.spans.extend(iter);
    }
}

impl Widget for Line<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

impl WidgetRef for Line<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.render_with_alignment(area, buf, None);
    }
}

impl Line<'_> {
    /// An internal implementation method for `WidgetRef::render_ref`
    ///
    /// Allows the parent widget to define a default alignment, to be
    /// used if `Line::alignment` is `None`.
    pub(crate) fn render_with_alignment(
        &self,
        area: Rect,
        buf: &mut Buffer,
        parent_alignment: Option<Alignment>,
    ) {
        let area = area.intersection(buf.area);
        if area.is_empty() {
            return;
        }
        let area = Rect { height: 1, ..area };
        let line_width = self.width();
        if line_width == 0 {
            return;
        }

        buf.set_style(area, self.style);

        let alignment = self.alignment.or(parent_alignment);

        let area_width = usize::from(area.width);
        let can_render_complete_line = line_width <= area_width;
        if can_render_complete_line {
            let indent_width = match alignment {
                Some(Alignment::Center) => (area_width.saturating_sub(line_width)) / 2,
                Some(Alignment::Right) => area_width.saturating_sub(line_width),
                Some(Alignment::Left) | None => 0,
            };
            let indent_width = u16::try_from(indent_width).unwrap_or(u16::MAX);
            let area = area.indent_x(indent_width);
            render_spans(&self.spans, area, buf, 0);
        } else {
            // There is not enough space to render the whole line. As the right side is truncated by
            // the area width, only truncate the left.
            let skip_width = match alignment {
                Some(Alignment::Center) => (line_width.saturating_sub(area_width)) / 2,
                Some(Alignment::Right) => line_width.saturating_sub(area_width),
                Some(Alignment::Left) | None => 0,
            };
            render_spans(&self.spans, area, buf, skip_width);
        };
    }
}

/// Renders all the spans of the line that should be visible.
fn render_spans(spans: &[Span], mut area: Rect, buf: &mut Buffer, span_skip_width: usize) {
    for (span, span_width, offset) in spans_after_width(spans, span_skip_width) {
        area = area.indent_x(offset);
        if area.is_empty() {
            break;
        }
        span.render_ref(area, buf);
        let span_width = u16::try_from(span_width).unwrap_or(u16::MAX);
        area = area.indent_x(span_width);
    }
}

/// Returns an iterator over the spans that lie after a given skip widtch from the start of the
/// `Line` (including a partially visible span if the `skip_width` lands within a span).
fn spans_after_width<'a>(
    spans: &'a [Span],
    mut skip_width: usize,
) -> impl Iterator<Item = (Span<'a>, usize, u16)> {
    spans
        .iter()
        .map(|span| (span, span.width()))
        // Filter non visible spans out.
        .filter_map(move |(span, span_width)| {
            // Ignore spans that are completely before the offset. Decrement `span_skip_width` by
            // the span width until we find a span that is partially or completely visible.
            if skip_width >= span_width {
                skip_width = skip_width.saturating_sub(span_width);
                return None;
            }

            // Apply the skip from the start of the span, not the end as the end will be trimmed
            // when rendering the span to the buffer.
            let available_width = span_width.saturating_sub(skip_width);
            skip_width = 0; // ensure the next span is rendered in full
            Some((span, span_width, available_width))
        })
        .map(|(span, span_width, available_width)| {
            if span_width <= available_width {
                // Span is fully visible. Clone here is fast as the underlying content is `Cow`.
                return (span.clone(), span_width, 0u16);
            }
            // Span is only partially visible. As the end is truncated by the area width, only
            // truncate the start of the span.
            let (content, actual_width) = span.content.unicode_truncate_start(available_width);

            // When the first grapheme of the span was truncated, start rendering from a position
            // that takes that into account by indenting the start of the area
            let first_grapheme_offset = available_width.saturating_sub(actual_width);
            let first_grapheme_offset = u16::try_from(first_grapheme_offset).unwrap_or(u16::MAX);
            (
                Span::styled(content, span.style),
                actual_width,
                first_grapheme_offset,
            )
        })
}

/// A trait for converting a value to a [`Line`].
///
/// This trait is automatically implemented for any type that implements the [`Display`] trait. As
/// such, `ToLine` shouln't be implemented directly: [`Display`] should be implemented instead, and
/// you get the `ToLine` implementation for free.
///
/// [`Display`]: std::fmt::Display
pub trait ToLine {
    /// Converts the value to a [`Line`].
    fn to_line(&self) -> Line<'_>;
}

/// # Panics
///
/// In this implementation, the `to_line` method panics if the `Display` implementation returns an
/// error. This indicates an incorrect `Display` implementation since `fmt::Write for String` never
/// returns an error itself.
impl<T: fmt::Display> ToLine for T {
    fn to_line(&self) -> Line<'_> {
        Line::from(self.to_string())
    }
}

impl fmt::Display for Line<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for span in &self.spans {
            write!(f, "{span}")?;
        }
        Ok(())
    }
}

impl<'a> Styled for Line<'a> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use rstest::{fixture, rstest};

    use super::*;
    use crate::style::{Color, Modifier, Stylize};

    #[fixture]
    fn small_buf() -> Buffer {
        Buffer::empty(Rect::new(0, 0, 10, 1))
    }

    #[test]
    fn raw_str() {
        let line = Line::raw("test content");
        assert_eq!(line.spans, [Span::raw("test content")]);
        assert_eq!(line.alignment, None);

        let line = Line::raw("a\nb");
        assert_eq!(line.spans, [Span::raw("a"), Span::raw("b")]);
        assert_eq!(line.alignment, None);
    }

    #[test]
    fn styled_str() {
        let style = Style::new().yellow();
        let content = "Hello, world!";
        let line = Line::styled(content, style);
        assert_eq!(line.spans, [Span::raw(content)]);
        assert_eq!(line.style, style);
    }

    #[test]
    fn styled_string() {
        let style = Style::new().yellow();
        let content = String::from("Hello, world!");
        let line = Line::styled(content.clone(), style);
        assert_eq!(line.spans, [Span::raw(content)]);
        assert_eq!(line.style, style);
    }

    #[test]
    fn styled_cow() {
        let style = Style::new().yellow();
        let content = Cow::from("Hello, world!");
        let line = Line::styled(content.clone(), style);
        assert_eq!(line.spans, [Span::raw(content)]);
        assert_eq!(line.style, style);
    }

    #[test]
    fn spans_vec() {
        let line = Line::default().spans(vec!["Hello".blue(), " world!".green()]);
        assert_eq!(
            line.spans,
            vec![
                Span::styled("Hello", Style::new().blue()),
                Span::styled(" world!", Style::new().green()),
            ]
        );
    }

    #[test]
    fn spans_iter() {
        let line = Line::default().spans([1, 2, 3].iter().map(|i| format!("Item {i}")));
        assert_eq!(
            line.spans,
            vec![
                Span::raw("Item 1"),
                Span::raw("Item 2"),
                Span::raw("Item 3"),
            ]
        );
    }

    #[test]
    fn style() {
        let line = Line::default().style(Style::new().red());
        assert_eq!(line.style, Style::new().red());
    }

    #[test]
    fn alignment() {
        let line = Line::from("This is left").alignment(Alignment::Left);
        assert_eq!(Some(Alignment::Left), line.alignment);

        let line = Line::from("This is default");
        assert_eq!(None, line.alignment);
    }

    #[test]
    fn width() {
        let line = Line::from(vec![
            Span::styled("My", Style::default().fg(Color::Yellow)),
            Span::raw(" text"),
        ]);
        assert_eq!(7, line.width());

        let empty_line = Line::default();
        assert_eq!(0, empty_line.width());
    }

    #[test]
    fn patch_style() {
        let raw_line = Line::styled("foobar", Color::Yellow);
        let styled_line = Line::styled("foobar", (Color::Yellow, Modifier::ITALIC));

        assert_ne!(raw_line, styled_line);

        let raw_line = raw_line.patch_style(Modifier::ITALIC);
        assert_eq!(raw_line, styled_line);
    }

    #[test]
    fn reset_style() {
        let line =
            Line::styled("foobar", Style::default().yellow().on_red().italic()).reset_style();

        assert_eq!(Style::reset(), line.style);
    }

    #[test]
    fn stylize() {
        assert_eq!(Line::default().green().style, Color::Green.into());
        assert_eq!(
            Line::default().on_green().style,
            Style::new().bg(Color::Green)
        );
        assert_eq!(Line::default().italic().style, Modifier::ITALIC.into());
    }

    #[test]
    fn from_string() {
        let s = String::from("Hello, world!");
        let line = Line::from(s);
        assert_eq!(line.spans, [Span::from("Hello, world!")]);

        let s = String::from("Hello\nworld!");
        let line = Line::from(s);
        assert_eq!(line.spans, [Span::from("Hello"), Span::from("world!")]);
    }

    #[test]
    fn from_str() {
        let s = "Hello, world!";
        let line = Line::from(s);
        assert_eq!(line.spans, [Span::from("Hello, world!")]);

        let s = "Hello\nworld!";
        let line = Line::from(s);
        assert_eq!(line.spans, [Span::from("Hello"), Span::from("world!")]);
    }

    #[test]
    fn to_line() {
        let line = 42.to_line();
        assert_eq!(line.spans, [Span::from("42")]);
    }

    #[test]
    fn from_vec() {
        let spans = vec![
            Span::styled("Hello,", Style::default().fg(Color::Red)),
            Span::styled(" world!", Style::default().fg(Color::Green)),
        ];
        let line = Line::from(spans.clone());
        assert_eq!(line.spans, spans);
    }

    #[test]
    fn from_iter() {
        let line = Line::from_iter(vec!["Hello".blue(), " world!".green()]);
        assert_eq!(
            line.spans,
            vec![
                Span::styled("Hello", Style::new().blue()),
                Span::styled(" world!", Style::new().green()),
            ]
        );
    }

    #[test]
    fn collect() {
        let line: Line = iter::once("Hello".blue())
            .chain(iter::once(" world!".green()))
            .collect();
        assert_eq!(
            line.spans,
            vec![
                Span::styled("Hello", Style::new().blue()),
                Span::styled(" world!", Style::new().green()),
            ]
        );
    }

    #[test]
    fn from_span() {
        let span = Span::styled("Hello, world!", Style::default().fg(Color::Yellow));
        let line = Line::from(span.clone());
        assert_eq!(line.spans, [span]);
    }

    #[test]
    fn add_span() {
        assert_eq!(
            Line::raw("Red").red() + Span::raw("blue").blue(),
            Line {
                spans: vec![Span::raw("Red"), Span::raw("blue").blue()],
                style: Style::new().red(),
                alignment: None,
            },
        );
    }

    #[test]
    fn add_line() {
        assert_eq!(
            Line::raw("Red").red() + Line::raw("Blue").blue(),
            Text {
                lines: vec![Line::raw("Red").red(), Line::raw("Blue").blue()],
                style: Style::default(),
                alignment: None,
            }
        );
    }

    #[test]
    fn add_assign_span() {
        let mut line = Line::raw("Red").red();
        line += Span::raw("Blue").blue();
        assert_eq!(
            line,
            Line {
                spans: vec![Span::raw("Red"), Span::raw("Blue").blue()],
                style: Style::new().red(),
                alignment: None,
            },
        );
    }

    #[test]
    fn extend() {
        let mut line = Line::from("Hello, ");
        line.extend([Span::raw("world!")]);
        assert_eq!(line.spans, [Span::raw("Hello, "), Span::raw("world!")]);

        let mut line = Line::from("Hello, ");
        line.extend([Span::raw("world! "), Span::raw("How are you?")]);
        assert_eq!(
            line.spans,
            [
                Span::raw("Hello, "),
                Span::raw("world! "),
                Span::raw("How are you?")
            ]
        );
    }

    #[test]
    fn into_string() {
        let line = Line::from(vec![
            Span::styled("Hello,", Style::default().fg(Color::Red)),
            Span::styled(" world!", Style::default().fg(Color::Green)),
        ]);
        let s: String = line.into();
        assert_eq!(s, "Hello, world!");
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

    #[test]
    fn display_line_from_vec() {
        let line_from_vec = Line::from(vec![Span::raw("Hello,"), Span::raw(" world!")]);

        assert_eq!(format!("{line_from_vec}"), "Hello, world!");
    }

    #[test]
    fn display_styled_line() {
        let styled_line = Line::styled("Hello, world!", Style::new().green().italic());

        assert_eq!(format!("{styled_line}"), "Hello, world!");
    }

    #[test]
    fn display_line_from_styled_span() {
        let styled_span = Span::styled("Hello, world!", Style::new().green().italic());
        let line_from_styled_span = Line::from(styled_span);

        assert_eq!(format!("{line_from_styled_span}"), "Hello, world!");
    }

    #[test]
    fn left_aligned() {
        let line = Line::from("Hello, world!").left_aligned();
        assert_eq!(line.alignment, Some(Alignment::Left));
    }

    #[test]
    fn centered() {
        let line = Line::from("Hello, world!").centered();
        assert_eq!(line.alignment, Some(Alignment::Center));
    }

    #[test]
    fn right_aligned() {
        let line = Line::from("Hello, world!").right_aligned();
        assert_eq!(line.alignment, Some(Alignment::Right));
    }

    #[test]
    pub fn push_span() {
        let mut line = Line::from("A");
        line.push_span(Span::raw("B"));
        line.push_span("C");
        assert_eq!(
            line.spans,
            vec![Span::raw("A"), Span::raw("B"), Span::raw("C")]
        );
    }

    mod widget {
        use unicode_segmentation::UnicodeSegmentation;
        use unicode_width::UnicodeWidthStr;

        use super::*;
        use crate::buffer::Cell;

        const BLUE: Style = Style::new().fg(Color::Blue);
        const GREEN: Style = Style::new().fg(Color::Green);
        const ITALIC: Style = Style::new().add_modifier(Modifier::ITALIC);

        #[fixture]
        fn hello_world() -> Line<'static> {
            Line::from(vec![
                Span::styled("Hello ", BLUE),
                Span::styled("world!", GREEN),
            ])
            .style(ITALIC)
        }

        #[test]
        fn render() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 1));
            hello_world().render(Rect::new(0, 0, 15, 1), &mut buf);
            let mut expected = Buffer::with_lines(["Hello world!   "]);
            expected.set_style(Rect::new(0, 0, 15, 1), ITALIC);
            expected.set_style(Rect::new(0, 0, 6, 1), BLUE);
            expected.set_style(Rect::new(6, 0, 6, 1), GREEN);
            assert_eq!(buf, expected);
        }

        #[rstest]
        fn render_out_of_bounds(hello_world: Line<'static>, mut small_buf: Buffer) {
            let out_of_bounds = Rect::new(20, 20, 10, 1);
            hello_world.render(out_of_bounds, &mut small_buf);
            assert_eq!(small_buf, Buffer::empty(small_buf.area));
        }

        #[test]
        fn render_only_styles_line_area() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 20, 1));
            hello_world().render(Rect::new(0, 0, 15, 1), &mut buf);
            let mut expected = Buffer::with_lines(["Hello world!        "]);
            expected.set_style(Rect::new(0, 0, 15, 1), ITALIC);
            expected.set_style(Rect::new(0, 0, 6, 1), BLUE);
            expected.set_style(Rect::new(6, 0, 6, 1), GREEN);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_only_styles_first_line() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 20, 2));
            hello_world().render(buf.area, &mut buf);
            let mut expected = Buffer::with_lines(["Hello world!        ", "                    "]);
            expected.set_style(Rect::new(0, 0, 20, 1), ITALIC);
            expected.set_style(Rect::new(0, 0, 6, 1), BLUE);
            expected.set_style(Rect::new(6, 0, 6, 1), GREEN);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_truncates() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
            Line::from("Hello world!").render(Rect::new(0, 0, 5, 1), &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello     "]));
        }

        #[test]
        fn render_centered() {
            let line = hello_world().alignment(Alignment::Center);
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 1));
            line.render(Rect::new(0, 0, 15, 1), &mut buf);
            let mut expected = Buffer::with_lines([" Hello world!  "]);
            expected.set_style(Rect::new(0, 0, 15, 1), ITALIC);
            expected.set_style(Rect::new(1, 0, 6, 1), BLUE);
            expected.set_style(Rect::new(7, 0, 6, 1), GREEN);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_right_aligned() {
            let line = hello_world().alignment(Alignment::Right);
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 1));
            line.render(Rect::new(0, 0, 15, 1), &mut buf);
            let mut expected = Buffer::with_lines(["   Hello world!"]);
            expected.set_style(Rect::new(0, 0, 15, 1), ITALIC);
            expected.set_style(Rect::new(3, 0, 6, 1), BLUE);
            expected.set_style(Rect::new(9, 0, 6, 1), GREEN);
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_truncates_left() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
            Line::from("Hello world")
                .left_aligned()
                .render(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello"]));
        }

        #[test]
        fn render_truncates_right() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
            Line::from("Hello world")
                .right_aligned()
                .render(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["world"]));
        }

        #[test]
        fn render_truncates_center() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
            Line::from("Hello world")
                .centered()
                .render(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["lo wo"]));
        }

        /// Part of a regression test for <https://github.com/ratatui/ratatui/issues/1032> which
        /// found panics with truncating lines that contained multi-byte characters.
        #[test]
        fn regression_1032() {
            let line = Line::from(
                "🦀 RFC8628 OAuth 2.0 Device Authorization GrantでCLIからGithubのaccess tokenを取得する"
            );
            let mut buf = Buffer::empty(Rect::new(0, 0, 83, 1));
            line.render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([
                "🦀 RFC8628 OAuth 2.0 Device Authorization GrantでCLIからGithubのaccess tokenを取得 "
            ]));
        }

        /// Documentary test to highlight the crab emoji width / length discrepancy
        ///
        /// Part of a regression test for <https://github.com/ratatui/ratatui/issues/1032> which
        /// found panics with truncating lines that contained multi-byte characters.
        #[test]
        fn crab_emoji_width() {
            let crab = "🦀";
            assert_eq!(crab.len(), 4); // bytes
            assert_eq!(crab.chars().count(), 1);
            assert_eq!(crab.graphemes(true).count(), 1);
            assert_eq!(crab.width(), 2); // display width
        }

        /// Part of a regression test for <https://github.com/ratatui/ratatui/issues/1032> which
        /// found panics with truncating lines that contained multi-byte characters.
        #[rstest]
        #[case::left_4(Alignment::Left, 4, "1234")]
        #[case::left_5(Alignment::Left, 5, "1234 ")]
        #[case::left_6(Alignment::Left, 6, "1234🦀")]
        #[case::left_7(Alignment::Left, 7, "1234🦀7")]
        #[case::right_4(Alignment::Right, 4, "7890")]
        #[case::right_5(Alignment::Right, 5, " 7890")]
        #[case::right_6(Alignment::Right, 6, "🦀7890")]
        #[case::right_7(Alignment::Right, 7, "4🦀7890")]
        fn render_truncates_emoji(
            #[case] alignment: Alignment,
            #[case] buf_width: u16,
            #[case] expected: &str,
        ) {
            let line = Line::from("1234🦀7890").alignment(alignment);
            let mut buf = Buffer::empty(Rect::new(0, 0, buf_width, 1));
            line.render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([expected]));
        }

        /// Part of a regression test for <https://github.com/ratatui/ratatui/issues/1032> which
        /// found panics with truncating lines that contained multi-byte characters.
        ///
        /// centering is tricky because there's an ambiguity about whether to take one more char
        /// from the left or the right when the line width is odd. This interacts with the width of
        /// the crab emoji, which is 2 characters wide by hitting the left or right side of the
        /// emoji.
        #[rstest]
        #[case::center_6_0(6, 0, "")]
        #[case::center_6_1(6, 1, " ")] // lef side of "🦀"
        #[case::center_6_2(6, 2, "🦀")]
        #[case::center_6_3(6, 3, "b🦀")]
        #[case::center_6_4(6, 4, "b🦀c")]
        #[case::center_7_0(7, 0, "")]
        #[case::center_7_1(7, 1, " ")] // right side of "🦀"
        #[case::center_7_2(7, 2, "🦀")]
        #[case::center_7_3(7, 3, "🦀c")]
        #[case::center_7_4(7, 4, "b🦀c")]
        #[case::center_8_0(8, 0, "")]
        #[case::center_8_1(8, 1, " ")] // right side of "🦀"
        #[case::center_8_2(8, 2, " c")] // right side of "🦀c"
        #[case::center_8_3(8, 3, "🦀c")]
        #[case::center_8_4(8, 4, "🦀cd")]
        #[case::center_8_5(8, 5, "b🦀cd")]
        #[case::center_9_0(9, 0, "")]
        #[case::center_9_1(9, 1, "c")]
        #[case::center_9_2(9, 2, " c")] // right side of "🦀c"
        #[case::center_9_3(9, 3, " cd")]
        #[case::center_9_4(9, 4, "🦀cd")]
        #[case::center_9_5(9, 5, "🦀cde")]
        #[case::center_9_6(9, 6, "b🦀cde")]
        fn render_truncates_emoji_center(
            #[case] line_width: u16,
            #[case] buf_width: u16,
            #[case] expected: &str,
        ) {
            // because the crab emoji is 2 characters wide, it will can cause the centering tests
            // intersect with either the left or right part of the emoji, which causes the emoji to
            // be not rendered. Checking for four different widths of the line is enough to cover
            // all the possible cases.
            let value = match line_width {
                6 => "ab🦀cd",
                7 => "ab🦀cde",
                8 => "ab🦀cdef",
                9 => "ab🦀cdefg",
                _ => unreachable!(),
            };
            let line = Line::from(value).centered();
            let mut buf = Buffer::empty(Rect::new(0, 0, buf_width, 1));
            line.render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([expected]));
        }

        /// Ensures the rendering also works away from the 0x0 position.
        ///
        /// Particularly of note is that an emoji that is truncated will not overwrite the
        /// characters that are already in the buffer. This is inentional (consider how a line
        /// that is rendered on a border should not overwrite the border with a partial emoji).
        #[rstest]
        #[case::left(Alignment::Left, "XXa🦀bcXXX")]
        #[case::center(Alignment::Center, "XX🦀bc🦀XX")]
        #[case::right(Alignment::Right, "XXXbc🦀dXX")]
        fn render_truncates_away_from_0x0(#[case] alignment: Alignment, #[case] expected: &str) {
            let line = Line::from(vec![Span::raw("a🦀b"), Span::raw("c🦀d")]).alignment(alignment);
            // Fill buffer with stuff to ensure the output is indeed padded
            let mut buf = Buffer::filled(Rect::new(0, 0, 10, 1), Cell::new("X"));
            let area = Rect::new(2, 0, 6, 1);
            line.render_ref(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([expected]));
        }

        /// When two spans are rendered after each other the first needs to be padded in accordance
        /// to the skipped unicode width. In this case the first crab does not fit at width 6 which
        /// takes a front white space.
        #[rstest]
        #[case::right_4(4, "c🦀d")]
        #[case::right_5(5, "bc🦀d")]
        #[case::right_6(6, "Xbc🦀d")]
        #[case::right_7(7, "🦀bc🦀d")]
        #[case::right_8(8, "a🦀bc🦀d")]
        fn render_right_aligned_multi_span(#[case] buf_width: u16, #[case] expected: &str) {
            let line = Line::from(vec![Span::raw("a🦀b"), Span::raw("c🦀d")]).right_aligned();
            let area = Rect::new(0, 0, buf_width, 1);
            // Fill buffer with stuff to ensure the output is indeed padded
            let mut buf = Buffer::filled(area, Cell::new("X"));
            line.render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([expected]));
        }

        /// Part of a regression test for <https://github.com/ratatui/ratatui/issues/1032> which
        /// found panics with truncating lines that contained multi-byte characters.
        ///
        /// Flag emoji are actually two independent characters, so they can be truncated in the
        /// middle of the emoji. This test documents just the emoji part of the test.
        #[test]
        fn flag_emoji() {
            let str = "🇺🇸1234";
            assert_eq!(str.len(), 12); // flag is 4 bytes
            assert_eq!(str.chars().count(), 6); // flag is 2 chars
            assert_eq!(str.graphemes(true).count(), 5); // flag is 1 grapheme
            assert_eq!(str.width(), 6); // flag is 2 display width
        }

        /// Part of a regression test for <https://github.com/ratatui/ratatui/issues/1032> which
        /// found panics with truncating lines that contained multi-byte characters.
        #[rstest]
        #[case::flag_1(1, " ")]
        #[case::flag_2(2, "🇺🇸")]
        #[case::flag_3(3, "🇺🇸1")]
        #[case::flag_4(4, "🇺🇸12")]
        #[case::flag_5(5, "🇺🇸123")]
        #[case::flag_6(6, "🇺🇸1234")]
        #[case::flag_7(7, "🇺🇸1234 ")]
        fn render_truncates_flag(#[case] buf_width: u16, #[case] expected: &str) {
            let line = Line::from("🇺🇸1234");
            let mut buf = Buffer::empty(Rect::new(0, 0, buf_width, 1));
            line.render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([expected]));
        }

        // Buffer width is `u16`. A line can be longer.
        #[rstest]
        #[case::left(Alignment::Left, "This is some content with a some")]
        #[case::right(Alignment::Right, "horribly long Line over u16::MAX")]
        fn render_truncates_very_long_line_of_many_spans(
            #[case] alignment: Alignment,
            #[case] expected: &str,
        ) {
            let part = "This is some content with a somewhat long width to be repeated over and over again to create horribly long Line over u16::MAX";
            let min_width = usize::from(u16::MAX).saturating_add(1);

            // width == len as only ASCII is used here
            let factor = min_width.div_ceil(part.len());

            let line = Line::from(vec![Span::raw(part); factor]).alignment(alignment);

            dbg!(line.width());
            assert!(line.width() >= min_width);

            let mut buf = Buffer::empty(Rect::new(0, 0, 32, 1));
            line.render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([expected]));
        }

        // Buffer width is `u16`. A single span inside a line can be longer.
        #[rstest]
        #[case::left(Alignment::Left, "This is some content with a some")]
        #[case::right(Alignment::Right, "horribly long Line over u16::MAX")]
        fn render_truncates_very_long_single_span_line(
            #[case] alignment: Alignment,
            #[case] expected: &str,
        ) {
            let part = "This is some content with a somewhat long width to be repeated over and over again to create horribly long Line over u16::MAX";
            let min_width = usize::from(u16::MAX).saturating_add(1);

            // width == len as only ASCII is used here
            let factor = min_width.div_ceil(part.len());

            let line = Line::from(vec![Span::raw(part.repeat(factor))]).alignment(alignment);

            dbg!(line.width());
            assert!(line.width() >= min_width);

            let mut buf = Buffer::empty(Rect::new(0, 0, 32, 1));
            line.render_ref(buf.area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([expected]));
        }

        #[test]
        fn render_with_newlines() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 11, 1));
            Line::from("Hello\nworld!").render(Rect::new(0, 0, 11, 1), &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Helloworld!"]));
        }
    }

    mod iterators {
        use super::*;

        /// a fixture used in the tests below to avoid repeating the same setup
        #[fixture]
        fn hello_world() -> Line<'static> {
            Line::from(vec![
                Span::styled("Hello ", Color::Blue),
                Span::styled("world!", Color::Green),
            ])
        }

        #[rstest]
        fn iter(hello_world: Line<'_>) {
            let mut iter = hello_world.iter();
            assert_eq!(iter.next(), Some(&Span::styled("Hello ", Color::Blue)));
            assert_eq!(iter.next(), Some(&Span::styled("world!", Color::Green)));
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn iter_mut(mut hello_world: Line<'_>) {
            let mut iter = hello_world.iter_mut();
            assert_eq!(iter.next(), Some(&mut Span::styled("Hello ", Color::Blue)));
            assert_eq!(iter.next(), Some(&mut Span::styled("world!", Color::Green)));
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn into_iter(hello_world: Line<'_>) {
            let mut iter = hello_world.into_iter();
            assert_eq!(iter.next(), Some(Span::styled("Hello ", Color::Blue)));
            assert_eq!(iter.next(), Some(Span::styled("world!", Color::Green)));
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn into_iter_ref(hello_world: Line<'_>) {
            let mut iter = (&hello_world).into_iter();
            assert_eq!(iter.next(), Some(&Span::styled("Hello ", Color::Blue)));
            assert_eq!(iter.next(), Some(&Span::styled("world!", Color::Green)));
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn into_iter_mut_ref() {
            let mut hello_world = Line::from(vec![
                Span::styled("Hello ", Color::Blue),
                Span::styled("world!", Color::Green),
            ]);
            let mut iter = (&mut hello_world).into_iter();
            assert_eq!(iter.next(), Some(&mut Span::styled("Hello ", Color::Blue)));
            assert_eq!(iter.next(), Some(&mut Span::styled("world!", Color::Green)));
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn for_loop_ref(hello_world: Line<'_>) {
            let mut result = String::new();
            for span in &hello_world {
                result.push_str(span.content.as_ref());
            }
            assert_eq!(result, "Hello world!");
        }

        #[rstest]
        fn for_loop_mut_ref() {
            let mut hello_world = Line::from(vec![
                Span::styled("Hello ", Color::Blue),
                Span::styled("world!", Color::Green),
            ]);
            let mut result = String::new();
            for span in &mut hello_world {
                result.push_str(span.content.as_ref());
            }
            assert_eq!(result, "Hello world!");
        }

        #[rstest]
        fn for_loop_into(hello_world: Line<'_>) {
            let mut result = String::new();
            for span in hello_world {
                result.push_str(span.content.as_ref());
            }
            assert_eq!(result, "Hello world!");
        }
    }

    #[rstest]
    #[case::empty(Line::default(), "Line::default()")]
    #[case::raw(Line::raw("Hello, world!"), r#"Line::from("Hello, world!")"#)]
    #[case::styled(
        Line::styled("Hello, world!", Color::Yellow),
        r#"Line::from("Hello, world!").yellow()"#
    )]
    #[case::styled_complex(
        Line::from(String::from("Hello, world!")).green().on_blue().bold().italic().not_dim(),
        r#"Line::from("Hello, world!").green().on_blue().bold().italic().not_dim()"#
    )]
    #[case::styled_span(
        Line::from(Span::styled("Hello, world!", Color::Yellow)),
        r#"Line::from(Span::from("Hello, world!").yellow())"#
    )]
    #[case::styled_line_and_span(
        Line::from(vec![
            Span::styled("Hello", Color::Yellow),
            Span::styled(" world!", Color::Green),
        ]).italic(),
        r#"Line::from_iter([Span::from("Hello").yellow(), Span::from(" world!").green()]).italic()"#
    )]
    #[case::spans_vec(
        Line::from(vec![
            Span::styled("Hello", Color::Blue),
            Span::styled(" world!", Color::Green),
        ]),
        r#"Line::from_iter([Span::from("Hello").blue(), Span::from(" world!").green()])"#,
    )]
    #[case::left_aligned(
        Line::from("Hello, world!").left_aligned(),
        r#"Line::from("Hello, world!").left_aligned()"#
    )]
    #[case::centered(
        Line::from("Hello, world!").centered(),
        r#"Line::from("Hello, world!").centered()"#
    )]
    #[case::right_aligned(
        Line::from("Hello, world!").right_aligned(),
        r#"Line::from("Hello, world!").right_aligned()"#
    )]
    fn debug(#[case] line: Line, #[case] expected: &str) {
        assert_eq!(format!("{line:?}"), expected);
    }
}
