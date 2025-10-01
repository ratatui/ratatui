#![warn(clippy::pedantic, clippy::nursery, clippy::arithmetic_side_effects)]
use alloc::borrow::{Cow, ToOwned};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::{fmt, iter};

use unicode_truncate::UnicodeTruncateStr;

use crate::buffer::Buffer;
use crate::layout::{Alignment, Rect};
use crate::style::{Style, Styled};
use crate::text::spacer::Spacer;
use crate::text::{Line, Span};
use crate::widgets::Widget;

/// Represents an inline block composed of one or more lines with shared styling and layout.
///
/// `InlineText` groups multiple [`Line`]s into a single block that is rendered **column-wise**
/// — that is, [`Line`]s are concatenated horizontally into a single visual line of text. This
/// contrasts with [`Text`], which renders [`Line`]s **row-wise**, stacking each line vertically.
/// The `InlineText` block is styled and aligned as a unit, and each line may contain its own
/// [`Span`]s and styles.
///
/// This is useful when you want to lay out multiple lines side-by-side with consistent alignment,
/// such as titles.
///
/// Lines within the block are separated by a configurable [`Spacer`], which inserts horizontal
/// gaps between flattened spans when rendered.
///
/// # Constructor Methods
///
/// - [`InlineText::raw`] creates a `InlineText` (potentially multiple lines) with no style.
/// - [`InlineText::default`] creates an empty `InlineText` (i.e. zero lines).
/// - [`InlineText::styled`] creates an `InlineText` with the given content and style.
///
/// # Conversion Methods
///
/// - [`InlineText::from`] creates a `InlineText` from a `String`.
/// - [`InlineText::from`] creates a `InlineText` from a `&str`.
/// - [`InlineText::from`] creates a `InlineText` from a `Cow<str>`.
/// - [`InlineText::from`] creates a `InlineText` from a [`Span`].
/// - [`InlineText::from`] creates a `InlineText` from a [`Line`].
/// - [`InlineText::from`] creates a `InlineText` from a `Vec<Into<Line>>`.
/// - [`InlineText::from_iter`] creates an `InlineText` from an iterator of items that are
///   convertible to [`Line`].
///
/// # Setter Methods
///
/// These methods are fluent setters. They return an `InlineText` with the property set.
///
/// - [`InlineText::lines`] sets the lines of the `InlineText`.
/// - [`InlineText::style`] sets the style of the `InlineText`.
/// - [`InlineText::alignment`] sets the alignment of the `InlineText`.
/// - [`InlineText::left_aligned`] sets the alignment to [`Alignment::Left`].
/// - [`InlineText::centered`] sets the alignment to [`Alignment::Center`].
/// - [`InlineText::right_aligned`] sets the alignment to [`Alignment::Right`].
/// - [`InlineText::spacer`] sets the [`Spacer`] between [`Line`]s.
///
/// # Iteration Methods
///
/// - [`InlineText::iter`] returns an iterator over the lines of the `InlineText`.
/// - [`InlineText::iter_mut`] returns a mutable iterator over the lines of the `InlineText`.
/// - [`InlineText::into_iter`] returns an iterator over the lines of the `InlineText`.
///
/// # Other Methods
///
/// - [`InlineText::patch_style`] patches the style of the `InlineText`, adding modifiers from the
///   given style.
/// - [`InlineText::reset_style`] resets the style of the `InlineText`.
/// - [`InlineText::width`] returns the unicode width of the content held by the `InlineText`.
/// - [`InlineText::push_line`] adds a line to the `InlineText`.
/// - [`InlineText::push_span`] adds a span to the last line of the `InlineText`.
///
/// [`Text`]: crate::text::Text
/// [`Span`]: crate::text::Span
/// [`Line`]: crate::text::Line
/// [`Style`]: crate::style::Style
/// [`Spacer`]: crate::text::Spacer
/// [`Alignment`]: crate::layout::Alignment
#[doc(hidden)]
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct InlineText<'a> {
    /// The style applied to the entire inline block.
    pub style: Style,

    /// The alignment of the inline block.
    pub alignment: Option<Alignment>,

    /// The spacer inserted between lines.
    pub spacer: Spacer,

    /// The lines that make up the inline block.
    pub lines: Vec<Line<'a>>,
}

impl fmt::Debug for InlineText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.lines.is_empty() {
            f.write_str("InlineText::default()")?;
        } else if self.lines.len() == 1 {
            write!(f, "InlineText::from({:?})", self.lines[0])?;
        } else {
            f.write_str("InlineText::from_iter(")?;
            f.debug_list().entries(self.lines.iter()).finish()?;
            f.write_str(")")?;
        }
        write!(f, ".with_space({})", self.spacer.width)?;
        self.style.fmt_stylize(f)?;
        match self.alignment {
            Some(Alignment::Left) => f.write_str(".left_aligned()")?,
            Some(Alignment::Center) => f.write_str(".centered()")?,
            Some(Alignment::Right) => f.write_str(".right_aligned()")?,
            _ => (),
        }
        Ok(())
    }
}

impl<'a> InlineText<'a> {
    /// Creates an `InlineText` block with the default style, alignment, and spacer.
    ///
    /// `content` can be any type that is convertible to [`Cow<str>`] (e.g. [`&str`], [`String`],
    /// [`Cow<str>`], or your own type that implements [`Into<Cow<str>>`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::text::{InlineText, Line};
    ///
    /// let inline = InlineText::raw("Hello, world!\nHello, Rustaceans!");
    /// assert_eq!(
    ///     inline.lines,
    ///     [
    ///         Line::from("Hello, world!"),
    ///         Line::from("Hello, Rustaceans!"),
    ///     ]
    /// );
    /// ```
    pub fn raw<T>(content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        let lines: Vec<_> = match content.into() {
            Cow::Borrowed("") => vec![Line::from("")],
            Cow::Borrowed(s) => s.lines().map(Line::from).collect(),
            Cow::Owned(s) if s.is_empty() => vec![Line::from("")],
            Cow::Owned(s) => s.lines().map(|l| Line::from(l.to_owned())).collect(),
        };
        Self::from(lines)
    }

    /// Creates an `InlineText` with the given [`Style`].
    ///
    /// `content` can be any type that is convertible to [`Cow<str>`] (e.g. [`&str`], [`String`],
    /// [`Cow<str>`], or your own type that implements [`Into<Cow<str>>`]).
    ///
    /// `style` can be any value that implements [`Into<Style>`], such as a [`Style`] literal or
    /// builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::style::{Style, Stylize};
    /// use ratatui_core::text::InlineText;
    ///
    /// let style = Style::new().yellow().italic();
    /// InlineText::styled("Hello, world!\nHello, Rustaceans!", style);
    /// ```
    pub fn styled<T, S>(content: T, style: S) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Style>,
    {
        Self::raw(content).patch_style(style)
    }

    /// Sets the lines of this `InlineText`.
    ///
    /// `lines` can be any iterable where each item is convertible into a [`Line`], such as a
    /// `Vec<Line>`, an array of `&str`, or any iterator yielding values that implement
    /// [`Into<Line>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::style::Stylize;
    /// use ratatui_core::text::InlineText;
    ///
    /// let inline = InlineText::default().lines(vec!["Hello, world!", "Hello, Rustaceans!"]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn lines<I>(mut self, lines: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Line<'a>>,
    {
        self.lines = lines.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the style of this `InlineText`.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::style::{Style, Stylize};
    /// use ratatui_core::text::InlineText;
    ///
    /// let mut inline =
    ///     InlineText::from(vec!["Hello, world!", "Hello, Rustaceans!"]).style(Style::new().red());
    /// ```
    ///
    /// [`Color`]: crate::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Patches the style of this `InlineText`, adding modifiers from the given style.
    ///
    /// This is useful for when you want to apply a style to a text that already has some styling.
    /// In contrast to [`InlineText::style`], this method will not overwrite the existing style, but
    /// instead will add the given style's modifiers to this `InlineText`'s style.
    ///
    /// `InlineText` also implements [`Styled`] which means you can use the methods of the
    /// [`Stylize`] trait.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::style::{Color, Modifier, Style};
    /// use ratatui_core::text::InlineText;
    ///
    /// let style = Style::new().italic();
    /// let raw_inline = InlineText::styled("Hello, world!", style);
    /// let styled_inline = InlineText::styled("Hello, world!", (Color::Yellow, Modifier::ITALIC));
    /// assert_ne!(raw_inline, styled_inline);
    ///
    /// let raw_inline = raw_inline.patch_style(Color::Yellow);
    /// assert_eq!(raw_inline, styled_inline);
    /// ```
    ///
    /// [`Color`]: crate::style::Color
    /// [`Stylize`]: crate::style::Stylize
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn patch_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = self.style.patch(style);
        self
    }

    /// Resets the style of this `InlineText`.
    ///
    /// Equivalent to calling [`patch_style(Style::reset())`](InlineText::patch_style).
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::style::{Color, Modifier, Style};
    /// use ratatui_core::text::InlineText;
    ///
    /// let inline = InlineText::styled("Hello, world!", (Color::Yellow, Modifier::ITALIC));
    ///
    /// let inline = inline.reset_style();
    /// assert_eq!(inline.style, Style::reset());
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn reset_style(self) -> Self {
        self.patch_style(Style::reset())
    }

    /// Sets the alignment for this `InlineText`.
    ///
    /// Defaults to: [`None`], in practice, this is equivalent to [`Alignment::Left`].
    ///
    /// Although [`Alignment`] can be set individually on each [`Line`], this is currently
    /// ignored. The [`Alignment`] defined on the `InlineText` itself is applied to all [`Line`]s
    /// as a whole. In effect, all [`Line`]s are aligned together as if they were a single [`Line`]
    /// separated by [`Spacer`]s, rather than being aligned independently per [`Line`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::layout::Alignment;
    /// use ratatui_core::text::InlineText;
    ///
    /// let mut inline = InlineText::from(vec!["Hello, world!", "Hello, Rustaceans!"]);
    /// assert_eq!(None, inline.alignment);
    /// assert_eq!(
    ///     Some(Alignment::Right),
    ///     inline.alignment(Alignment::Right).alignment,
    /// );
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn alignment(self, alignment: Alignment) -> Self {
        Self {
            alignment: Some(alignment),
            ..self
        }
    }

    /// Left-aligns this `InlineText`.
    ///
    /// Convenience shortcut for `InlineText::alignment(Alignment::Left)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::text::InlineText;
    ///
    /// let mut inline = InlineText::from(vec!["Hello, world!", "Hello, Rustaceans!"]).left_aligned();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn left_aligned(self) -> Self {
        self.alignment(Alignment::Left)
    }

    /// Center-aligns this `InlineText`.
    ///
    /// Convenience shortcut for `Line::alignment(Alignment::Center)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::text::InlineText;
    ///
    /// let mut inline = InlineText::from(vec!["Hello, world!", "Hello, Rustaceans!"]).centered();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn centered(self) -> Self {
        self.alignment(Alignment::Center)
    }

    /// Right-aligns this `InlineText`.
    ///
    /// Convenience shortcut for `Line::alignment(Alignment::Right)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::text::InlineText;
    ///
    /// let mut inline = InlineText::from(vec!["Hello, world!", "Hello, Rustaceans!"]).right_aligned();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn right_aligned(self) -> Self {
        self.alignment(Alignment::Right)
    }

    /// Sets the spacer of this `InlineText`.
    ///
    /// `spacer` accepts any type that is convertible to [`Spacer`] (e.g. [`Spacer`], [`usize`], or
    /// your own type that implements [`Into<Spacer>`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::text::InlineText;
    ///
    /// let mut inline = InlineText::from(vec!["Hello, world!", "Hello, Rustaceans!"]).spacer(1);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn spacer<S: Into<Spacer>>(mut self, spacer: S) -> Self {
        self.spacer = spacer.into();
        self
    }

    /// Returns the width of the underlying string.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::style::Stylize;
    /// use ratatui_core::text::{InlineText, Line};
    ///
    /// let inline = InlineText::from(vec![
    ///     Line::raw("Hello, world!").blue(),
    ///     Line::raw("Hello, Rustaceans!").green(),
    /// ])
    /// .spacer(1);
    /// assert_eq!(inline.width(), 32);
    /// ```
    #[must_use = "method returns the inline's width and should not be ignored"]
    pub fn width(&self) -> usize {
        self.span_or_spacer_iter()
            .map(|span_or_spacer| match span_or_spacer {
                SpanOrSpacer::Span(span, _) => span.width(),
                SpanOrSpacer::Spacer(spacer) => spacer.width,
            })
            .sum()
    }

    /// Returns an iterator over the [`Line`]s of this `InlineText`.
    pub fn iter(&self) -> core::slice::Iter<'_, Line<'a>> {
        self.lines.iter()
    }

    /// Returns an iterator that allows modifying each [`Line`].
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Line<'a>> {
        self.lines.iter_mut()
    }

    /// Adds a line to this `InlineText`.
    ///
    /// `line` can be any type that can be converted into a `Line`. For example, you can pass a
    /// `&str`, a `String`, a `Span`, or a `Line`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::text::{InlineText, Line, Span};
    ///
    /// let mut inline = InlineText::raw("Hello, world!");
    /// inline.push_line(Line::from("Hello, Rustaceans!"));
    /// inline.push_line(Span::from("Hello, Rustaceans!"));
    /// inline.push_line("Hello, Rustaceans!");
    /// ```
    pub fn push_line<T: Into<Line<'a>>>(&mut self, line: T) {
        self.lines.push(line.into());
    }

    /// Adds a span to the last line of this `InlineText`.
    ///
    /// `span` can be any type that is convertible into a `Span`. For example, you can pass a
    /// `&str`, a `String`, or a `Span`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::text::{InlineText, Span};
    ///
    /// let mut inline = InlineText::raw("Hello, world!");
    /// inline.push_span(Span::from("Hello, Rustaceans!"));
    /// inline.push_span("Hello, Rustaceans!");
    /// ```
    pub fn push_span<T: Into<Span<'a>>>(&mut self, span: T) {
        let span = span.into();
        if let Some(last) = self.lines.last_mut() {
            last.push_span(span);
        } else {
            self.lines.push(Line::from(span));
        }
    }
}

// Represents an item in an inline block: either a span of text or a spacer between lines.
//
// This enum is used when iterating over the contents of an inline via methods like
// `iter_spans_or_spacers()`, allowing each part—text or spacer—to be processed uniformly.
#[derive(Debug, Clone)]
enum SpanOrSpacer<'a> {
    // A span of styled text from a line.
    //
    // # Fields
    // - `&'a Span<'a>`: Reference to the span.
    // - `&'a Style`: Reference to the parent line style.
    Span(&'a Span<'a>, &'a Style),

    // A spacer inserted between lines in an inline block.
    Spacer(&'a Spacer),
}

impl<'a> InlineText<'a> {
    // Returns an iterator over all spans in all lines, with spacers inserted between lines.
    fn span_or_spacer_iter(&'a self) -> impl Iterator<Item = SpanOrSpacer<'a>> + 'a {
        self.lines.iter().enumerate().flat_map(move |(i, line)| {
            let iter = line
                .spans
                .iter()
                .map(move |span| SpanOrSpacer::Span(span, &line.style));
            if i < self.lines.len().saturating_sub(1) {
                Box::new(iter.chain(iter::once(SpanOrSpacer::Spacer(&self.spacer))))
                    as Box<dyn Iterator<Item = SpanOrSpacer<'a>>>
            } else {
                Box::new(iter) as Box<dyn Iterator<Item = SpanOrSpacer<'a>>>
            }
        })
    }
}

impl<'a> IntoIterator for InlineText<'a> {
    type Item = Line<'a>;
    type IntoIter = alloc::vec::IntoIter<Line<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter()
    }
}

impl<'a> IntoIterator for &'a InlineText<'a> {
    type Item = &'a Line<'a>;
    type IntoIter = core::slice::Iter<'a, Line<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut InlineText<'a> {
    type Item = &'a mut Line<'a>;
    type IntoIter = core::slice::IterMut<'a, Line<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl From<String> for InlineText<'_> {
    fn from(s: String) -> Self {
        Self::raw(s)
    }
}

impl<'a> From<&'a str> for InlineText<'a> {
    fn from(s: &'a str) -> Self {
        Self::raw(s)
    }
}

impl<'a> From<Cow<'a, str>> for InlineText<'a> {
    fn from(s: Cow<'a, str>) -> Self {
        Self::raw(s)
    }
}

impl<'a> From<Span<'a>> for InlineText<'a> {
    fn from(span: Span<'a>) -> Self {
        Self {
            lines: vec![Line::from(span)],
            ..Default::default()
        }
    }
}

impl<'a> From<Line<'a>> for InlineText<'a> {
    fn from(line: Line<'a>) -> Self {
        InlineText {
            lines: vec![line],
            ..Default::default()
        }
    }
}

impl<'a, T> From<Vec<T>> for InlineText<'a>
where
    T: Into<Line<'a>>,
{
    fn from(items: Vec<T>) -> Self {
        let lines = items.into_iter().map(Into::into).collect();
        InlineText {
            lines,
            ..Default::default()
        }
    }
}

impl<'a, T> FromIterator<T> for InlineText<'a>
where
    T: Into<Line<'a>>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            lines: iter.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

/// A trait for converting a value to a [`InlineText`].
///
/// This trait is automatically implemented for any type that implements the [`Display`] trait. As
/// such, `ToInlineText` shouldn't be implemented directly: [`Display`] should be implemented
/// instead, and you get the `ToInlineText` implementation for free.
///
/// [`Display`]: std::fmt::Display
#[doc(hidden)]
pub trait ToInlineText {
    /// Converts the value to a [`InlineText`].
    fn to_inline_text(&self) -> InlineText<'_>;
}

/// # Panics
///
/// In this implementation, the `to_inline_text` method panics if the `Display` implementation
/// returns an error. This indicates an incorrect `Display` implementation since `fmt::Write for
/// String` never returns an error itself.
impl<T: fmt::Display> ToInlineText for T {
    fn to_inline_text(&self) -> InlineText<'_> {
        InlineText::raw(self.to_string())
    }
}

impl<'a> core::ops::Add<Line<'a>> for InlineText<'a> {
    type Output = Self;

    fn add(mut self, line: Line<'a>) -> Self::Output {
        self.push_line(line);
        self
    }
}

impl<'a> core::ops::AddAssign<Line<'a>> for InlineText<'a> {
    fn add_assign(&mut self, line: Line<'a>) {
        self.push_line(line);
    }
}

impl<'a, T> Extend<T> for InlineText<'a>
where
    T: Into<Line<'a>>,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let lines = iter.into_iter().map(Into::into);
        self.lines.extend(lines);
    }
}

impl fmt::Display for SpanOrSpacer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpanOrSpacer::Span(span, _) => write!(f, "{span}"),
            SpanOrSpacer::Spacer(spacer) => write!(f, "{spacer}"),
        }
    }
}

impl fmt::Display for InlineText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for span_or_spacer in self.span_or_spacer_iter() {
            write!(f, "{span_or_spacer}")?;
        }
        Ok(())
    }
}

impl Widget for InlineText<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &InlineText<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        if area.is_empty() {
            return;
        }
        let area = Rect { height: 1, ..area };
        let area_width = usize::from(area.width);
        let inline_width = self.width();
        if inline_width == 0 {
            return;
        }
        buf.set_style(area, self.style);
        if inline_width <= area_width {
            let indent_width = match self.alignment {
                Some(Alignment::Center) => (area_width.saturating_sub(inline_width)) / 2,
                Some(Alignment::Right) => area_width.saturating_sub(inline_width),
                Some(Alignment::Left) | None => 0,
            };
            let indent_width = u16::try_from(indent_width).unwrap_or(u16::MAX);
            let area = area.indent_x(indent_width);
            self.render_fragments(area, buf, 0);
        } else {
            let skip_width = match self.alignment {
                Some(Alignment::Center) => (inline_width.saturating_sub(area_width)) / 2,
                Some(Alignment::Right) => inline_width.saturating_sub(area_width),
                Some(Alignment::Left) | None => 0,
            };
            self.render_fragments(area, buf, skip_width);
        }
    }
}

impl InlineText<'_> {
    // Renders all the fragments of the inline that should be visible.
    fn render_fragments(&self, mut area: Rect, buf: &mut Buffer, skip_width: usize) {
        for fragment in self.fragment_iter(skip_width) {
            match fragment {
                Fragment::Span(span, line_style) => {
                    if area.is_empty() {
                        break;
                    }
                    let width = u16::try_from(span.width()).unwrap_or_else(|err| {
                        panic!(
                            "failed to convert span width (usize) {} to u16: {}",
                            span.width(),
                            err
                        )
                    });
                    let span_area = Rect { width, ..area };
                    buf.set_style(span_area, *line_style);
                    span.render(area, buf);
                    area = area.indent_x(width);
                }
                Fragment::PartialSpan(span, line_style) => {
                    if area.is_empty() {
                        break;
                    }
                    let width = u16::try_from(span.width()).unwrap_or_else(|err| {
                        panic!(
                            "failed to convert span width (usize) {} to u16: {}",
                            span.width(),
                            err
                        )
                    });
                    let span_area = Rect { width, ..area };
                    buf.set_style(span_area, *line_style);
                    span.render(area, buf);
                    area = area.indent_x(width);
                }
                Fragment::Spacer(spacer) => {
                    spacer.apply(&mut area);
                }
                Fragment::PartialSpacer(spacer) => {
                    spacer.apply(&mut area);
                }
            }
        }
    }
}

// Represents a single fragment (span or spacer) of an inline text block as used during rendering.
//
// This enum is designed to express both fully visible and partially visible fragments of a block
// of inline text.
#[derive(Debug, Clone)]
enum Fragment<'a> {
    // A fully visible span, referencing the source data and style.
    //
    // # Fields
    // - `&'a Span<'a>`: Reference to the span.
    // - `&'a Style`: Reference to the parent line style.
    Span(&'a Span<'a>, &'a Style),

    // A partially visible span, holding owned data for the truncated fragment.
    //
    // # Fields
    // - `Span<'a>`: Owned span representing the visible part.
    // - `&'a Style`: Reference to the parent line style.
    PartialSpan(Span<'a>, &'a Style),

    // A fully visible spacer, referencing the source data.
    Spacer(&'a Spacer),

    // A partially visible spacer, holding owned data for the truncated fragment.
    PartialSpacer(Spacer),
}

impl<'a> InlineText<'a> {
    // Returns an iterator over the fragments of spans and spacers that lie after a given skip width
    // from the start of the inline (including a partially visible span and/or spacer if the
    // `skip_width` lands within a span and/or spacer).
    fn fragment_iter(&'a self, mut skip_width: usize) -> impl Iterator<Item = Fragment<'a>> + 'a {
        self.span_or_spacer_iter()
            .map(|span_or_spacer| match span_or_spacer {
                SpanOrSpacer::Span(span, _) => (span_or_spacer, span.width()),
                SpanOrSpacer::Spacer(spacer) => (span_or_spacer, spacer.width),
            })
            .filter_map(move |(span_or_spacer, mut width)| {
                if skip_width < width {
                    if skip_width != 0 {
                        width = width.saturating_sub(skip_width);
                        skip_width = 0;
                    }
                    Some((span_or_spacer, width))
                } else {
                    skip_width = skip_width.saturating_sub(width);
                    None
                }
            })
            .map(|(span_or_spacer, width)| match span_or_spacer {
                SpanOrSpacer::Span(span, line_style) => {
                    if span.width() == width {
                        Fragment::Span(span, line_style)
                    } else {
                        let (content, _) = span.content.unicode_truncate_start(width);
                        Fragment::PartialSpan(Span::styled(content, span.style), line_style)
                    }
                }
                SpanOrSpacer::Spacer(spacer) => {
                    if spacer.width == width {
                        Fragment::Spacer(spacer)
                    } else {
                        Fragment::PartialSpacer(Spacer::new(width))
                    }
                }
            })
    }
}

impl Styled for InlineText<'_> {
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
    use alloc::format;
    use alloc::string::String;

    use rstest::{fixture, rstest};

    use super::*;
    use crate::style::{Color, Modifier, Style, Stylize};

    #[test]
    fn raw() {
        let inline = InlineText::raw("Hello, world!\nHello, Rustaceans!");
        assert_eq!(
            inline.lines,
            [
                Line::from("Hello, world!"),
                Line::from("Hello, Rustaceans!"),
            ]
        );
        assert!(inline.style.fg.is_none());
        assert!(inline.style.bg.is_none());
        assert!(inline.style.add_modifier.is_empty());
        assert!(inline.style.sub_modifier.is_empty());
        assert!(inline.alignment.is_none());
        assert_eq!(inline.spacer.width, 0);
    }

    #[test]
    fn default() {
        let inline = InlineText::default();
        assert!(inline.style.fg.is_none());
        assert!(inline.style.bg.is_none());
        assert!(inline.style.add_modifier.is_empty());
        assert!(inline.style.sub_modifier.is_empty());
        assert!(inline.alignment.is_none());
        assert_eq!(inline.spacer.width, 0);
        assert!(inline.lines.is_empty());
    }

    #[test]
    fn styled() {
        let style = Style::new().yellow();
        let inline = InlineText::styled("Hello, world!\nHello, Rustaceans!", style);
        assert_eq!(
            inline.lines,
            [
                Line::from("Hello, world!"),
                Line::from("Hello, Rustaceans!"),
            ]
        );
        assert_eq!(inline.style, style);
    }

    #[test]
    fn lines() {
        let lines = ["Hello, world!", "Hello, Rustaceans!"];
        let inline = InlineText::default().lines(lines);
        assert_eq!(
            inline.lines,
            [
                Line::from("Hello, world!"),
                Line::from("Hello, Rustaceans!"),
            ]
        );
    }

    #[test]
    fn style() {
        let inline = InlineText::default().style(Style::new().red());
        assert_eq!(inline.style, Style::new().red());
    }

    #[test]
    fn patch_style() {
        let raw_inline = InlineText::styled("Hello, world!", Color::Yellow);
        let styled_inline = InlineText::styled("Hello, world!", (Color::Yellow, Modifier::ITALIC));
        assert_ne!(raw_inline, styled_inline);
        let raw_inline = raw_inline.patch_style(Modifier::ITALIC);
        assert_eq!(raw_inline, styled_inline);
    }

    #[test]
    fn reset_style() {
        let inline =
            InlineText::styled("Hello, world!", Style::default().yellow().on_red().italic())
                .reset_style();
        assert_eq!(inline.style, Style::reset());
    }

    #[test]
    fn alignment() {
        let inline = InlineText::raw("Hello, world!\nHello, Rustaceans!");
        assert_eq!(inline.alignment, None);
        assert_eq!(
            inline.alignment(Alignment::Right).alignment,
            Some(Alignment::Right),
        );
    }

    #[test]
    fn spacer() {
        let inline = InlineText::default().spacer(Spacer::new(1));
        assert_eq!(inline.spacer.width, 1);
    }

    #[test]
    fn width() {
        let inline = InlineText::raw("Hello, world!\nHello, Rustaceans!").spacer(1);
        assert_eq!(inline.width(), 32);
    }

    #[test]
    fn push_line() {
        let mut inline = InlineText::raw("A");
        inline.push_line(Line::from("B"));
        inline.push_line(Span::from("C"));
        inline.push_line("D");
        assert_eq!(
            inline.lines,
            vec![
                Line::raw("A"),
                Line::raw("B"),
                Line::raw("C"),
                Line::raw("D"),
            ]
        );
    }

    #[test]
    fn push_line_empty() {
        let mut inline = InlineText::default();
        inline.push_line(Line::from("Hello, world!"));
        assert_eq!(inline.lines, [Line::from("Hello, world!")]);
    }

    #[test]
    fn push_span() {
        let mut inline = InlineText::raw("A");
        inline.push_span(Span::raw("B"));
        inline.push_span("C");
        assert_eq!(
            inline.lines,
            vec![Line::from(vec![
                Span::raw("A"),
                Span::raw("B"),
                Span::raw("C"),
            ])],
        );
    }

    #[test]
    fn push_span_empty() {
        let mut inline = InlineText::default();
        inline.push_span(Span::raw("Hello, world!"));
        assert_eq!(inline.lines, [Line::from(Span::raw("Hello, world!"))]);
    }

    mod iterators {
        use super::*;

        #[fixture]
        fn greetings() -> InlineText<'static> {
            InlineText::from_iter([
                Span::styled("Hello, world!", Color::Blue),
                Span::styled("Hello, Rustaceans!", Color::Green),
            ])
        }

        #[rstest]
        fn iter(greetings: InlineText<'_>) {
            let mut iter = greetings.iter();
            assert_eq!(
                iter.next(),
                Some(&Line::from(Span::from("Hello, world!").blue())),
            );
            assert_eq!(
                iter.next(),
                Some(&Line::from(Span::from("Hello, Rustaceans!").green())),
            );
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn iter_mut(mut greetings: InlineText<'_>) {
            let mut iter = greetings.iter_mut();
            assert_eq!(
                iter.next(),
                Some(&mut Line::from(Span::from("Hello, world!").blue())),
            );
            assert_eq!(
                iter.next(),
                Some(&mut Line::from(Span::from("Hello, Rustaceans!").green())),
            );
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn into_iter(greetings: InlineText<'_>) {
            let mut iter = greetings.into_iter();
            assert_eq!(
                iter.next(),
                Some(Line::from(Span::from("Hello, world!").blue())),
            );
            assert_eq!(
                iter.next(),
                Some(Line::from(Span::from("Hello, Rustaceans!").green())),
            );
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn into_iter_ref(greetings: InlineText<'_>) {
            let mut iter = (&greetings).into_iter();
            assert_eq!(
                iter.next(),
                Some(&Line::from(Span::from("Hello, world!").blue())),
            );
            assert_eq!(
                iter.next(),
                Some(&Line::from(Span::from("Hello, Rustaceans!").green())),
            );
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn into_iter_mut_ref() {
            let mut greetings = InlineText::from_iter([
                Span::styled("Hello, world!", Color::Blue),
                Span::styled("Hello, Rustaceans!", Color::Green),
            ]);
            let mut iter = (&mut greetings).into_iter();
            assert_eq!(
                iter.next(),
                Some(&mut Line::from(Span::from("Hello, world!").blue())),
            );
            assert_eq!(
                iter.next(),
                Some(&mut Line::from(Span::from("Hello, Rustaceans!").green())),
            );
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn for_loop_ref(greetings: InlineText<'_>) {
            let mut result = String::new();
            for line in &greetings {
                result.push_str(&String::from(line.clone()));
            }
            assert_eq!(result, "Hello, world!Hello, Rustaceans!");
        }

        #[rstest]
        fn for_loop_mut_ref() {
            let mut greetings = InlineText::from_iter([
                Span::styled("Hello, world!", Color::Blue),
                Span::styled("Hello, Rustaceans!", Color::Green),
            ]);
            let mut result = String::new();
            for line in &mut greetings {
                result.push_str(&String::from(line.clone()));
            }
            assert_eq!(result, "Hello, world!Hello, Rustaceans!");
        }

        #[rstest]
        fn for_loop_into(greetings: InlineText<'_>) {
            let mut result = String::new();
            for line in greetings {
                result.push_str(&String::from(line.clone()));
            }
            assert_eq!(result, "Hello, world!Hello, Rustaceans!");
        }
    }

    mod conversions {
        use super::*;

        #[test]
        fn from_string() {
            let inline = InlineText::from(String::from("Hello, world!\nHello, Rustaceans!"));
            assert_eq!(
                inline.lines,
                vec![
                    Line::from("Hello, world!"),
                    Line::from("Hello, Rustaceans!"),
                ],
            );
        }

        #[test]
        fn from_str() {
            let inline = InlineText::from("Hello, world!\nHello, Rustaceans!");
            assert_eq!(
                inline.lines,
                vec![
                    Line::from("Hello, world!"),
                    Line::from("Hello, Rustaceans!"),
                ],
            );
        }

        #[test]
        fn from_cow() {
            let inline = InlineText::from(Cow::Borrowed("Hello, world!\nHello, Rustaceans!"));
            assert_eq!(
                inline.lines,
                vec![
                    Line::from("Hello, world!"),
                    Line::from("Hello, Rustaceans!"),
                ],
            );
        }

        #[test]
        fn from_span() {
            let style = Style::new().yellow().italic();
            let inline = InlineText::from(Span::styled("Hello, world!\nHello, Rustaceans!", style));
            assert_eq!(
                inline.lines,
                vec![Line::from(Span::styled(
                    "Hello, world!\nHello, Rustaceans!",
                    style,
                )),],
            );
        }

        #[test]
        fn from_line() {
            let line = Line::from("Hello, world!");
            let inline = InlineText::from(line);
            assert_eq!(inline.lines, [Line::from("Hello, world!")]);
        }

        #[test]
        fn from_line_vec() {
            let line1 = Line::from("Hello, world!");
            let line2 = Line::from("Hello, Rustaceans!");
            let inline = InlineText::from(vec![line1, line2]);
            assert_eq!(
                inline.lines,
                [
                    Line::from("Hello, world!"),
                    Line::from("Hello, Rustaceans!"),
                ]
            );
        }

        #[test]
        fn from_line_iter() {
            let lines = ["Hello, world!", "Hello, Rustaceans!"];
            let inline: InlineText = lines.into_iter().collect();
            assert_eq!(
                inline.lines,
                [
                    Line::from("Hello, world!"),
                    Line::from("Hello, Rustaceans!"),
                ]
            );
        }

        #[rstest]
        #[case(42, InlineText::from("42"))]
        #[case(
            "Hello, world!\nHello, Rustaceans!",
            InlineText::from("Hello, world!\nHello, Rustaceans!")
        )]
        #[case(true, InlineText::from("true"))]
        #[case(6.66, InlineText::from("6.66"))]
        #[case('a', InlineText::from("a"))]
        #[case(String::from("Hello, world!"), InlineText::from("Hello, world!"))]
        #[case(-1, InlineText::from("-1"))]
        #[case(
            "Hello, world!\nHello, Rustaceans!",
            InlineText::from("Hello, world!\nHello, Rustaceans!")
        )]
        #[case(
            "Hello, world!\nHello, Rustaceans!\nGreetings, fellow developers!",
            InlineText::from("Hello, world!\nHello, Rustaceans!\nGreetings, fellow developers!")
        )]
        #[case("Hello, world!\n", InlineText::from("Hello, world!\n"))]
        fn to_inline_text(#[case] value: impl fmt::Display, #[case] expected: InlineText) {
            assert_eq!(value.to_inline_text(), expected);
        }
    }

    mod operators {
        use super::*;

        #[test]
        fn add_line() {
            assert_eq!(
                InlineText::raw("Hello, world!").red() + Line::raw("Hello, Rustaceans!").blue(),
                InlineText {
                    style: Style::new().red(),
                    alignment: None,
                    spacer: Spacer::default(),
                    lines: vec![
                        Line::raw("Hello, world!"),
                        Line::raw("Hello, Rustaceans!").blue()
                    ],
                }
            );
        }

        #[test]
        fn add_assign_line() {
            let mut inline = InlineText::raw("Hello, world!").red();
            inline += Line::raw("Hello, Rustaceans!").blue();
            assert_eq!(
                inline,
                InlineText {
                    style: Style::new().red(),
                    alignment: None,
                    spacer: Spacer::default(),
                    lines: vec![
                        Line::raw("Hello, world!"),
                        Line::raw("Hello, Rustaceans!").blue()
                    ],
                }
            );
        }
    }

    mod collections {
        use super::*;

        #[test]
        fn extend() {
            let mut inline = InlineText::raw("Hello, world!").red();
            inline.extend(vec![
                Line::from("Hello, Rustaceans!"),
                Line::from("Greetings, fellow developers!"),
            ]);
            assert_eq!(
                inline.lines,
                vec![
                    Line::from("Hello, world!"),
                    Line::from("Hello, Rustaceans!"),
                    Line::from("Greetings, fellow developers!"),
                ]
            );
        }

        #[test]
        fn extend_from_iter_str() {
            let mut inline = InlineText::from("Hello, world!\nHello, Rustaceans!");
            inline.extend(vec![
                "Greetings, fellow developers!",
                "Greetings, fearless coders!",
            ]);
            assert_eq!(
                inline.lines,
                vec![
                    Line::from("Hello, world!"),
                    Line::from("Hello, Rustaceans!"),
                    Line::from("Greetings, fellow developers!"),
                    Line::from("Greetings, fearless coders!"),
                ]
            );
        }
    }

    #[rstest]
    #[case::one_line(
        InlineText::raw("Hello, world!").spacer(1),
        "Hello, world!",
    )]
    #[case::multiple_lines(
        InlineText::raw("Hello, world!\nHello, Rustaceans!").spacer(1),
        "Hello, world!.Hello, Rustaceans!",
    )]
    #[case::styled(
        InlineText::styled(
            "Hello, world!\nHello, Rustaceans!",
            Style::new().yellow().italic(),
        ).spacer(1),
        "Hello, world!.Hello, Rustaceans!",
    )]
    #[cfg(debug_assertions)]
    fn display(#[case] inline: InlineText, #[case] expected: &str) {
        assert_eq!(format!("{inline}"), expected);
    }

    #[rstest]
    #[case::one_line(
        InlineText::raw("Hello, world!").spacer(1),
        "Hello, world!",
    )]
    #[case::multiple_lines(
        InlineText::raw("Hello, world!\nHello, Rustaceans!").spacer(1),
        "Hello, world! Hello, Rustaceans!",
    )]
    #[case::styled(
        InlineText::styled(
            "Hello, world!\nHello, Rustaceans!",
            Style::new().yellow().italic(),
        ).spacer(1),
        "Hello, world! Hello, Rustaceans!",
    )]
    #[cfg(not(debug_assertions))]
    fn display(#[case] inline: InlineText, #[case] expected: &str) {
        assert_eq!(format!("{inline}"), expected);
    }

    #[rstest]
    #[case::raw(
        InlineText::raw("Hello, world!\nHello, Rustaceans!"),
        r#"InlineText::from_iter([Line::from("Hello, world!"), Line::from("Hello, Rustaceans!")]).with_space(0)"#,
    )]
    #[case::default(InlineText::default(), "InlineText::default().with_space(0)")]
    #[case::styled(
        InlineText::styled("Hello, world!\nHello, Rustaceans!", Color::Yellow),
        r#"InlineText::from_iter([Line::from("Hello, world!"), Line::from("Hello, Rustaceans!")]).with_space(0).yellow()"#,
    )]
    #[case::styled_complex(
        InlineText::from(vec![
            "Hello, world!",
            "Hello, Rustaceans!",
        ]).green().on_blue().bold().italic().not_dim(),
        r#"InlineText::from_iter([Line::from("Hello, world!"), Line::from("Hello, Rustaceans!")]).with_space(0).green().on_blue().bold().italic().not_dim()"#,
    )]
    #[case::styled_line(
        InlineText::from(Line::styled("Hello, world!", Color::Yellow)),
        r#"InlineText::from(Line::from("Hello, world!").yellow()).with_space(0)"#
    )]
    #[case::styled_inline_and_line(
        InlineText::from(vec![
            Line::styled("Hello, world!", Color::Yellow),
            Line::styled("Hello, Rustaceans!", Color::Green),
        ]).italic(),
        r#"InlineText::from_iter([Line::from("Hello, world!").yellow(), Line::from("Hello, Rustaceans!").green()]).with_space(0).italic()"#,
    )]
    #[case::left_aligned(
        InlineText::raw("Hello, world!").left_aligned(),
        r#"InlineText::from(Line::from("Hello, world!")).with_space(0).left_aligned()"#,
    )]
    #[case::centered(
        InlineText::raw("Hello, world!").centered(),
        r#"InlineText::from(Line::from("Hello, world!")).with_space(0).centered()"#,
    )]
    #[case::right_aligned(
        InlineText::raw("Hello, world!").right_aligned(),
        r#"InlineText::from(Line::from("Hello, world!")).with_space(0).right_aligned()"#,
    )]
    fn debug(#[case] inline: InlineText, #[case] expected: &str) {
        assert_eq!(format!("{inline:?}"), expected);
    }

    mod widget {
        use super::*;

        #[test]
        fn render() {
            let inline = InlineText::from("Hello, world!");
            let area = Rect::new(0, 0, 15, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello, world!  "]));
        }

        #[test]
        fn render_out_of_bounds() {
            let out_of_bounds_area = Rect::new(20, 20, 10, 1);
            let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
            InlineText::from("Hello, world!").render(out_of_bounds_area, &mut buf);
            assert_eq!(buf, Buffer::empty(buf.area));
        }

        #[test]
        fn render_left_aligned() {
            let inline = InlineText::from("Hello, world!").alignment(Alignment::Left);
            let area = Rect::new(0, 0, 15, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello, world!  "]));
        }

        #[test]
        fn render_right_aligned() {
            let inline = InlineText::from("Hello, world!").alignment(Alignment::Right);
            let area = Rect::new(0, 0, 15, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["  Hello, world!"]));
        }

        #[test]
        fn render_centered_odd() {
            let inline = InlineText::from("Hello, world!").alignment(Alignment::Center);
            let area = Rect::new(0, 0, 15, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([" Hello, world! "]));
        }

        #[test]
        fn render_centered_even() {
            let inline = InlineText::from("Hello, world!").alignment(Alignment::Center);
            let area = Rect::new(0, 0, 16, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([" Hello, world!  "]));
        }

        #[test]
        fn render_left_aligned_with_truncation() {
            let inline = InlineText::from("Hello, world!").alignment(Alignment::Left);
            let area = Rect::new(0, 0, 7, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello, "]));
        }

        #[test]
        fn render_right_aligned_with_truncation() {
            let inline = InlineText::from("Hello, world!").alignment(Alignment::Right);
            let area = Rect::new(0, 0, 7, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([" world!"]));
        }

        #[test]
        fn render_centered_odd_with_truncation() {
            let inline = InlineText::from("Hello, world!").alignment(Alignment::Center);
            let area = Rect::new(0, 0, 7, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["lo, wor"]));
        }

        #[test]
        fn render_centered_even_with_truncation() {
            let inline = InlineText::from("Hello, world!").alignment(Alignment::Center);
            let area = Rect::new(0, 0, 6, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["lo, wo"]));
        }

        #[test]
        fn render_with_spacer_left_aligned() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .spacer(1)
                .alignment(Alignment::Left);
            let area = Rect::new(0, 0, 15, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello, world!  "]));
        }

        #[test]
        fn render_with_spacer_right_aligned() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .spacer(1)
                .alignment(Alignment::Right);
            let area = Rect::new(0, 0, 15, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["  Hello, world!"]));
        }

        #[test]
        fn render_with_spacer_centered_odd() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .spacer(1)
                .alignment(Alignment::Center);
            let area = Rect::new(0, 0, 15, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([" Hello, world! "]));
        }

        #[test]
        fn render_with_spacer_centered_even() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .spacer(1)
                .alignment(Alignment::Center);
            let area = Rect::new(0, 0, 16, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([" Hello, world!  "]));
        }

        #[test]
        fn render_left_aligned_with_spacer_and_truncation() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .spacer(1)
                .alignment(Alignment::Left);
            let area = Rect::new(0, 0, 7, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello, "]));
        }

        #[test]
        fn render_right_aligned_with_spacer_and_truncation() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .spacer(1)
                .alignment(Alignment::Right);
            let area = Rect::new(0, 0, 7, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([" world!"]));
        }

        #[test]
        fn render_centered_odd_with_spacer_and_truncation() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .spacer(1)
                .alignment(Alignment::Center);
            let area = Rect::new(0, 0, 7, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["lo, wor"]));
        }

        #[test]
        fn render_centered_even_with_spacer_and_truncation() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .spacer(1)
                .alignment(Alignment::Center);
            let area = Rect::new(0, 0, 6, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["lo, wo"]));
        }
    }
}
