#![warn(clippy::pedantic, clippy::nursery, clippy::arithmetic_side_effects)]
use alloc::borrow::{Cow, ToOwned};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::option::Option;
use core::{fmt, iter};

use unicode_truncate::UnicodeTruncateStr;
use unicode_width::UnicodeWidthStr;

use crate::buffer::Buffer;
use crate::layout::{Alignment, Position, Rect};
use crate::style::{Style, Styled};
use crate::text::grapheme::StyledGrapheme;
use crate::text::{Line, Span};
use crate::widgets::Widget;

/// Represents an inline block composed of one or more lines with shared styling and layout.
///
/// `InlineText` groups multiple [`Line`]s into a single block that is rendered **column-wise**
/// — that is, [`Line`]s are concatenated horizontally into a single visual line of text. This
/// contrasts with [`Text`], which renders [`Line`]s **row-wise**, stacking each line vertically.
/// The `InlineText` block is treated as a single visual unit for styling and alignment, while
/// each contained [`Line`] can define its own [`Span`]s and styles.
///
/// When rendered within a given [`Rect`], `InlineText` automatically performs **text wrapping**
/// to ensure that the content fits within the available horizontal space. Long lines are truncated
/// or wrapped accordingly to the layout rules, preventing overlap with adjacent text or boundaries.
///
/// When no [`Alignment`] is specified for the `InlineText` itself, lines are grouped by their
/// individual [`Line`] alignments and rendered according to each group's alignment within the
/// inline area. Conversely, when the `InlineText` has an explicit [`Alignment`], all lines are
/// treated as a single group and rendered according to that alignment.
///
/// [`Line`]s within the `InlineText` are separated by a space, which inserts horizontal gaps
/// between lines when rendered.
///
/// This is useful when you want to lay out multiple lines side-by-side with consistent alignment,
/// such as titles.
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
/// [`Alignment`]: crate::layout::Alignment
#[doc(hidden)]
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct InlineText<'a> {
    /// The style applied to the entire inline block.
    pub style: Style,

    /// The alignment applied to the entire inline block.
    pub alignment: Option<Alignment>,

    /// The space inserted between lines.
    pub space: usize,

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
        write!(f, ".with_space({})", self.space)?;
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
    /// Creates an `InlineText` block with the default style, alignment, and space.
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
    /// When an alignment is specified for the `InlineText`, all contained [`Line`]s are treated
    /// as a single group and rendered according to that alignment.
    ///
    /// When no alignment is explicitly set on the `InlineText` (`None`), lines are grouped by
    /// their individual [`Line`] alignments and rendered per-group within the inline area.
    /// For any `Line` that does not have an explicit alignment, `Alignment::Left` is assumed.
    ///
    /// Defaults to [`None`].
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

    /// Sets the horizontal space between [`Line`]s within each alignment group of this
    /// `InlineText`.
    ///
    /// Each alignment group is rendered according to either the `InlineText` alignment (if set)
    /// or per-`Line` alignment (if `InlineText` alignment is `None`). The `space` is applied
    /// between consecutive lines within the same group.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui_core::text::InlineText;
    ///
    /// let mut inline = InlineText::from(vec!["Hello, world!", "Hello, Rustaceans!"]).space(1);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn space(mut self, space: usize) -> Self {
        self.space = space;
        self
    }

    /// Returns the total width of all lines if concatenated horizontally,
    /// including the spaces set by [`InlineText::space`] between consecutive lines.
    ///
    /// Alignment groups are ignored — this measures the width as if all lines
    /// were placed sequentially in a single row with the configured spacing.
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
    /// .space(1);
    /// assert_eq!(inline.width(), 32);
    /// ```
    #[must_use = "method returns the inline's width and should not be ignored"]
    pub fn width(&self) -> usize {
        self.span_or_space_iter(None)
            .map(|span_or_space| match span_or_space {
                SpanOrSpace::Span(span, _) => span.width(),
                SpanOrSpace::Space(space, _) => space,
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

// Represents an item in an inline block: either a span of text or a space between lines.
//
// This enum is used when iterating over the contents of an inline via methods like
// `iter_spans_or_spaces()`, allowing each part—text or space—to be processed uniformly.
#[derive(Debug, Clone)]
enum SpanOrSpace<'a> {
    // A span of styled text from a line.
    //
    // # Fields
    // - `&'a Span<'a>`: Reference to the span.
    // - `&'a Style`: Reference to the parent line style.
    Span(&'a Span<'a>, &'a Style),

    // A space inserted between lines in an inline block.
    //
    // # Fields
    // - `usize`: Owned space width.
    // - `&'a Style`: Reference to the parent line style.
    Space(usize, &'a Style),
}

impl<'a> InlineText<'a> {
    // Returns an iterator over all spans in all lines, with spaces inserted between lines.
    fn span_or_space_iter(
        &'a self,
        maybe_alignment: Option<Alignment>,
    ) -> impl Iterator<Item = SpanOrSpace<'a>> + 'a {
        self.lines
            .iter()
            .filter(move |line| {
                match (maybe_alignment, line.alignment.unwrap_or(Alignment::Left)) {
                    (Some(alignment), line_alignment) => alignment == line_alignment,
                    (None, _) => true,
                }
            })
            .enumerate()
            .flat_map(move |(i, line)| {
                let iter = line
                    .spans
                    .iter()
                    .map(move |span| SpanOrSpace::Span(span, &line.style));
                if i < self.lines.len().saturating_sub(1) {
                    Box::new(iter.chain(iter::once(SpanOrSpace::Space(self.space, &line.style))))
                        as Box<dyn Iterator<Item = SpanOrSpace<'a>>>
                } else {
                    Box::new(iter) as Box<dyn Iterator<Item = SpanOrSpace<'a>>>
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

impl fmt::Display for SpanOrSpace<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpanOrSpace::Span(span, _) => write!(f, "{span}"),
            SpanOrSpace::Space(space, _) => {
                let width = f.precision().map_or(*space, |p| *space.min(&p));
                for _ in 0..width {
                    f.write_str(" ")?;
                }
                Ok(())
            }
        }
    }
}

impl fmt::Display for InlineText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for span_or_space in self.span_or_space_iter(None) {
            write!(f, "{span_or_space}")?;
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
        buf.set_style(area, self.style);
        // The total inline width.
        let inline_width = area.width.saturating_mul(area.height);
        // If `alignment` is not set, each `Line`'s own alignment is used to group, reorder, and
        // align individually.
        let Some(alignment) = self.alignment else {
            // Computes the rendering range with skip width for each alignment within the given
            // `inline_width`.
            let (left, center, right) = self.alignment_widths();
            let (left, center, right) = [
                (left, Alignment::Left),
                (center, Alignment::Center),
                (right, Alignment::Right),
            ]
            .map(|(width, alignment)| {
                let width = u16::try_from(width).unwrap_or(u16::MAX);
                InlineText::alignment_bounds(width, inline_width, alignment)
            })
            .into();
            let mut position = area.as_position();
            // If there is a left-aligned lines, render it.
            if let Some((range, skip_width)) = left {
                let candidates = [center, right];
                let other = candidates.iter().flatten().next();
                let end = if let Some((other_range, _)) = other {
                    InlineText::overlap(range, *other_range)
                        .map_or(range.1, |overlap| (overlap.0.saturating_add(overlap.1)) / 2)
                } else {
                    range.1
                };
                position.move_to_mut(range.0, area);
                self.render_fragments(
                    &mut position,
                    area,
                    buf,
                    skip_width,
                    end.saturating_sub(range.0),
                    Some(Alignment::Left),
                );
            }
            // If there is a center-aligned lines, render it.
            if let Some((range, skip_width)) = center {
                let start = if let Some((left_range, _)) = left {
                    InlineText::overlap(left_range, range)
                        .map_or(range.0, |overlap| (overlap.0.saturating_add(overlap.1)) / 2)
                } else {
                    range.0
                };
                let end = if let Some((right_range, _)) = right {
                    InlineText::overlap(range, right_range)
                        .map_or(range.1, |overlap| (overlap.0.saturating_add(overlap.1)) / 2)
                } else {
                    range.1
                };
                position.move_to_mut(start, area);
                self.render_fragments(
                    &mut position,
                    area,
                    buf,
                    skip_width.saturating_add(start.saturating_sub(range.0)),
                    end.saturating_sub(start),
                    Some(Alignment::Center),
                );
            }
            // If there is a right-aligned lines, render it.
            if let Some((range, skip_width)) = right {
                let candidates = [center, left];
                let other = candidates.iter().flatten().next();
                let start = if let Some((other_range, _)) = other {
                    InlineText::overlap(*other_range, range)
                        .map_or(range.0, |overlap| (overlap.0.saturating_add(overlap.1)) / 2)
                } else {
                    range.0
                };
                position.move_to_mut(start, area);
                self.render_fragments(
                    &mut position,
                    area,
                    buf,
                    skip_width.saturating_add(start.saturating_sub(range.0)),
                    range.1.saturating_sub(start),
                    Some(Alignment::Right),
                );
            }
            return;
        };
        // If `alignment` is set, all Lines are concatenated and rendered according to the specified
        // alignment.
        let width = u16::try_from(self.width()).unwrap_or(u16::MAX);
        if width == 0 {
            return;
        }
        if let Some(((indent_width, rendering_width), skip_width)) =
            InlineText::alignment_bounds(width, inline_width, alignment)
        {
            let mut position = area.as_position();
            position.step_wrapping_mut(indent_width, area);
            self.render_fragments(&mut position, area, buf, skip_width, rendering_width, None);
        }
    }
}

impl InlineText<'_> {
    // Renders all the fragments of the inline that should be visible.
    fn render_fragments(
        &self,
        position: &mut Position,
        area: Rect,
        buf: &mut Buffer,
        offset: u16,
        width: u16,
        maybe_alignment: Option<Alignment>,
    ) {
        for fragment in self
            .span_or_space_iter(maybe_alignment)
            .into_fragment_iter(offset, width)
        {
            match fragment {
                Fragment::Span(span, line_style) => {
                    span.render_wrapping(position, *line_style, area, buf);
                }
                Fragment::PartialSpan(span, line_style) => {
                    span.render_wrapping(position, *line_style, area, buf);
                }
                Fragment::Space(space, line_style) => {
                    // NOTE: Should be the style reset here instead?
                    let space = u16::try_from(space).unwrap_or(u16::MAX);
                    for spaced_position in &mut *position
                        .iter_wrapping_to(*position.step_wrapping_mut(space, area), area)
                    {
                        buf[(spaced_position.x, spaced_position.y)].set_style(*line_style);
                    }
                }
            }
        }
    }
}

// Represents a single fragment (span or space) of an inline text block as used during rendering.
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

    // A fully visible or partially visible space, referencing the source data.
    //
    // # Fields
    // - `usize`: Owned space width.
    // - `&'a Style`: Reference to the parent line style.
    Space(usize, &'a Style),
}

trait FragmentIteratorExt<'a>: Iterator<Item = SpanOrSpace<'a>> + Sized {
    // Returns an iterator over the fragments of spans and spaces that lie within a given range.
    //
    // This iterator includes partially visible spans and/or spaces if the specified `offset` lands
    // within a span or space. The iteration will stop once the `remaining` width has been fully
    // consumed.
    fn into_fragment_iter(
        self,
        mut offset: u16,
        remaining: u16,
    ) -> impl Iterator<Item = Fragment<'a>> {
        self
            // Attach width to each `SpanOrSpace`.
            .map(|span_or_space| match span_or_space {
                SpanOrSpace::Span(span, _) => (
                    span_or_space,
                    u16::try_from(span.width()).unwrap_or(u16::MAX),
                ),
                SpanOrSpace::Space(space, _) => {
                    (span_or_space, u16::try_from(space).unwrap_or(u16::MAX))
                }
            })
            // Skip elements until the starting offset is reached.
            .skip_while(move |(_, width)| {
                if offset > *width {
                    offset = offset.saturating_sub(*width);
                    true
                } else {
                    false
                }
            })
            // Compute the visible width after applying left-side offset.
            .map(move |(span_or_space, width)| {
                let mut left_trimmed_width = width;
                if offset > 0 {
                    left_trimmed_width = width.saturating_sub(offset);
                    offset = 0;
                }
                (span_or_space, width, left_trimmed_width)
            })
            // Limit iteration to the requested `remaining` width and compute the final visible
            // width.
            .scan(
                remaining,
                move |remaining, (span_or_space, width, left_trimmed_width)| {
                    if *remaining == 0 {
                        None
                    } else {
                        let content_width = left_trimmed_width.min(*remaining);
                        *remaining = remaining.saturating_sub(content_width);
                        Some((span_or_space, width, left_trimmed_width, content_width))
                    }
                },
            )
            // Convert width metadata back into renderable `Fragment`s.
            .map(
                |(span_or_space, width, left_trimmed_width, content_width)| match span_or_space {
                    SpanOrSpace::Span(span, line_style) => {
                        if width == content_width {
                            Fragment::Span(span, line_style)
                        } else {
                            let (content, _) = span
                                .content
                                .unicode_truncate_start(left_trimmed_width.into());
                            let (content, _) = content.unicode_truncate(content_width.into());
                            Fragment::PartialSpan(Span::styled(content, span.style), line_style)
                        }
                    }
                    SpanOrSpace::Space(_, line_style) => {
                        Fragment::Space(content_width.into(), line_style)
                    }
                },
            )
    }
}

impl<'a, I> FragmentIteratorExt<'a> for I where I: Iterator<Item = SpanOrSpace<'a>> {}

impl Span<'_> {
    // Renders this `Span` within the given `area` and `buf`, advancing `position` and wrapping
    // text as needed.
    //
    // This method is similar to the `render` implementation in `Widget for &Span<'_>`, but modified
    // to properly handle grapheme-wise wrapping. The provided `position` is updated to reflect the
    // final cursor location after rendering.
    fn render_wrapping<S: Into<Style>>(
        &self,
        position: &mut Position,
        line_style: S,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let area = area.intersection(buf.area);
        if area.is_empty() || !area.contains(*position) {
            return;
        }
        let line_style = line_style.into();
        let mut graphemes = self.styled_graphemes(Style::default()).peekable();
        // Writes a grapheme into a buffer with the specified position and the line style.
        // If the `append` flag set to be true, the grapheme will be appended to the existing
        // grapheme.
        let write_grapheme = |buf: &mut Buffer,
                              position: &Position,
                              grapheme: &StyledGrapheme,
                              line_style: Style,
                              append: bool| {
            let cell = &mut buf[(position.x, position.y)];
            if append {
                cell.append_symbol(grapheme.symbol);
            } else {
                cell.set_symbol(grapheme.symbol);
            }
            cell.set_style(line_style).set_style(grapheme.style);
        };
        // Multi-width graphemes must clear the cells of characters that are hidden by the
        // grapheme, otherwise the hidden characters will be re-rendered if the grapheme is
        // overwritten.
        let clear_hidden = |buf: &mut Buffer, position: &mut Position, next_position: Position| {
            for hidden_position in position
                .clone()
                .step_wrapping_mut(1, area)
                .iter_wrapping_to(next_position, area)
            {
                buf[(hidden_position.x, hidden_position.y)].reset();
            }
        };
        // Collects any zero-width graphemes that appear at the start of the cell.
        // These zero-width graphemes form a "prefix" for the first visible grapheme.
        // Examples include characters like Left-to-Right Mark (LRM, U+200E) that may appear
        // at the start of a grapheme cluster.
        // See: https://github.com/ratatui/ratatui/issues/1160
        let zero_width_prefix: Vec<_> =
            core::iter::from_fn(|| graphemes.next_if(|g| g.symbol.width() == 0)).collect();
        for (i, grapheme) in zero_width_prefix.iter().enumerate() {
            write_grapheme(buf, position, grapheme, line_style, i != 0);
        }
        // Renders the first grapheme, handling zero-width prefix if present.
        if let Some(first) = graphemes.next() {
            let symbol_width = u16::try_from(first.symbol.width()).unwrap_or(u16::MAX);
            // Advances the cursor; stop rendering if the current position is out of bounds.
            let Some(next_position) = position.try_step_wrapping_mut(symbol_width, area) else {
                return;
            };
            write_grapheme(
                buf,
                position,
                &first,
                line_style,
                !zero_width_prefix.is_empty(),
            );
            clear_hidden(buf, position, next_position);
            let mut prev_position = *position;
            *position = next_position;
            // Processes the remaining graphemes.
            for grapheme in graphemes {
                let symbol_width = u16::try_from(grapheme.symbol.width()).unwrap_or(u16::MAX);
                // Continues the same cursor; zero-width graphemes are appended to the previous
                // cell.
                if symbol_width == 0 {
                    write_grapheme(buf, &prev_position, &grapheme, line_style, true);
                    continue;
                }
                // Advances the cursor; stop rendering if the current position is out of bounds.
                let Some(next_position) = position.try_step_wrapping_mut(symbol_width, area) else {
                    break;
                };
                write_grapheme(buf, position, &grapheme, line_style, false);
                clear_hidden(buf, position, next_position);
                prev_position = *position;
                *position = next_position;
            }
        }
    }
}

impl Position {
    // Calculates the linear index of `position` within `area`.
    // Validation (e.g., ensuring `position` is inside `area`) is the caller's responsibility.
    const fn index(self, area: Rect) -> u16 {
        let dx = self.x.saturating_sub(area.x);
        let dy = self.y.saturating_sub(area.y);
        dy.saturating_mul(area.width).saturating_add(dx)
    }

    // Moves this position to the coordinates corresponding to the given linear `index` within
    // `area`.
    const fn move_to_mut(&mut self, index: u16, area: Rect) -> &Self {
        let dx = index
            .checked_rem(area.width)
            .expect("division by zero while computing remainder for wrapped position");
        let dy = index
            .checked_div(area.width)
            .expect("division by zero while computing wrapped line count");
        self.x = area.x.saturating_add(dx);
        self.y = area.y.saturating_add(dy);
        self
    }

    // Increments this position by `width` coordinate-wise, wrapping within `area` if needed.
    const fn step_wrapping_mut(&mut self, width: u16, area: Rect) -> &Self {
        if area.is_empty() || !area.contains(*self) {
            return self;
        }
        let i = self.index(area).saturating_add(width);
        let dx = i
            .checked_rem(area.width)
            .expect("division by zero while computing remainder for wrapped position");
        let dy = i
            .checked_div(area.width)
            .expect("division by zero while computing wrapped line count");
        self.x = area.x.saturating_add(dx);
        self.y = area.y.saturating_add(dy);
        self
    }

    // Increments this position by `width` grapheme-wise, wrapping within `area` if needed,
    // preserving grapheme context, and returns the next position or `None` if it overflows.
    const fn try_step_wrapping_mut(&mut self, width: u16, area: Rect) -> Option<Self> {
        if area.is_empty() || !area.contains(*self) || width > area.width {
            return None;
        }
        let mut next = *self;
        next.x = next.x.saturating_add(width);
        #[allow(clippy::else_if_without_else)]
        if next.x == area.right() {
            // When `next.x == area.right()`, the current (x, y) position is still valid for
            // rendering the grapheme.
            next.x = area.left();
            next.y = self.y.saturating_add(1);
        } else if next.x > area.right() {
            // When `next.x > area.right()`, the current grapheme does not fit in the remaining
            // width and must be wrapped to the next line.
            self.x = area.left();
            self.y = self.y.saturating_add(1);
            // Unlike the condition above, this check uses `self.y`; when `self.y == area.bottom()`,
            // rendering is no longer possible.
            if self.y >= area.bottom() {
                return None;
            }
            next = *self;
            next.x = next.x.saturating_add(width);
        }
        Some(next)
    }

    // Iterates from self (inclusive) to `end` (exclusive) coordinate-wise, wrapping within `area`
    // if needed.
    fn iter_wrapping_to(self, other: Self, area: Rect) -> Box<dyn Iterator<Item = Self>> {
        if area.is_empty() || !area.contains(self) || self.index(area) >= other.index(area) {
            return Box::new(core::iter::empty());
        }
        Box::new(
            (self.index(area)
                ..other
                    .index(area)
                    .min(area.width.saturating_mul(area.height)))
                .map(move |i| {
                    let dx = i
                        .checked_rem(area.width)
                        .expect("division by zero while computing remainder for wrapped position");
                    let dy = i
                        .checked_div(area.width)
                        .expect("division by zero while computing wrapped line count");
                    Self {
                        x: area.x.saturating_add(dx),
                        y: area.y.saturating_add(dy),
                    }
                }),
        )
    }
}

impl InlineText<'_> {
    // Calculates the total width of spans for each alignment (Left, Center, Right).
    fn alignment_widths(&self) -> (usize, usize, usize) {
        // Tracks (total width, number of spans) for each alignment as an array [Left, Center,
        // Right].
        let acc = {
            let mut acc: [(usize, usize); 3] = [(0, 0); 3];
            self.lines
                .iter()
                .flat_map(|line| {
                    let alignment = line.alignment.unwrap_or(Alignment::Left);
                    line.spans.iter().map(move |span| (span, alignment))
                })
                .for_each(|(span, alignment)| {
                    let i = match alignment {
                        Alignment::Left => 0,
                        Alignment::Center => 1,
                        Alignment::Right => 2,
                    };
                    acc[i].0 = acc[i].0.saturating_add(span.width());
                    acc[i].1 = acc[i].1.saturating_add(1);
                });
            acc
        };
        // Adds spacing and convert to final widths.
        acc.map(|(width, count): (usize, usize)| {
            width.saturating_add(count.saturating_sub(1).saturating_mul(self.space))
        })
        .into()
    }

    // Returns ([start_index, end_index), skip_width) for the given alignment within `inline_width`.
    fn alignment_bounds(
        width: u16,
        inline_width: u16,
        alignment: Alignment,
    ) -> Option<((u16, u16), u16)> {
        (width > 0).then(|| match alignment {
            Alignment::Left => ((0, width.min(inline_width)), 0),
            Alignment::Center => {
                if width > inline_width {
                    let overflow = width.saturating_sub(inline_width);
                    let skip_width = overflow / 2;
                    ((0, inline_width), skip_width)
                } else {
                    let space = inline_width.saturating_sub(width);
                    let indent_width = space / 2;
                    ((indent_width, indent_width.saturating_add(width)), 0)
                }
            }
            Alignment::Right => {
                if width > inline_width {
                    ((0, inline_width), width.saturating_sub(inline_width))
                } else {
                    ((inline_width.saturating_sub(width), inline_width), 0)
                }
            }
        })
    }

    // Calculates the overlapping range between two ranges.
    fn overlap(lhs: (u16, u16), rhs: (u16, u16)) -> Option<(u16, u16)> {
        let overlap = (lhs.0.max(rhs.0), lhs.1.min(rhs.1));
        (overlap.0 < overlap.1).then_some(overlap)
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
    use crate::buffer::Cell;
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
        assert_eq!(inline.space, 0);
    }

    #[test]
    fn default() {
        let inline = InlineText::default();
        assert!(inline.style.fg.is_none());
        assert!(inline.style.bg.is_none());
        assert!(inline.style.add_modifier.is_empty());
        assert!(inline.style.sub_modifier.is_empty());
        assert!(inline.alignment.is_none());
        assert_eq!(inline.space, 0);
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
    fn space() {
        let inline = InlineText::default().space(1);
        assert_eq!(inline.space, 1);
    }

    #[test]
    fn width() {
        let inline = InlineText::raw("Hello, world!\nHello, Rustaceans!").space(1);
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
                    space: Default::default(),
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
                    space: Default::default(),
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

    mod span {
        use super::*;

        #[test]
        fn render_wrapping_with_sufficient_area() {
            let style = Style::new().green().on_yellow();
            let span = Span::styled("Hello, world! Hello, Rustaceans!", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 10, 4));
            let mut position = buf.area.as_position();
            span.render_wrapping(&mut position, style, buf.area, &mut buf);
            let expected = Buffer::with_lines([
                Line::from(Span::from("Hello, wor").green().on_yellow()),
                Line::from(Span::from("ld! Hello,").green().on_yellow()),
                Line::from(Span::from(" Rustacean").green().on_yellow()),
                Line::from(Span::from("s!").green().on_yellow()),
            ]);
            assert_eq!(buf, expected);
            assert_eq!(position, Position { x: 2, y: 3 });
        }

        #[test]
        fn render_wrapping_without_sufficient_area() {
            let style = Style::new().green().on_yellow();
            let span = Span::styled("Hello, world! Hello, Rustaceans!", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 10, 2));
            let mut position = buf.area.as_position();
            span.render_wrapping(&mut position, style, buf.area, &mut buf);
            let expected = Buffer::with_lines([
                Line::from(Span::from("Hello, wor").green().on_yellow()),
                Line::from(Span::from("ld! Hello,").green().on_yellow()),
            ]);
            assert_eq!(buf, expected);
            assert_eq!(position, Position { x: 0, y: 2 });
        }

        #[test]
        fn render_wrapping_out_of_bounds() {
            let style = Style::new().green().on_yellow();
            let span = Span::styled("Hello, world! Hello, Rustaceans!", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 10, 2));
            let out_of_bounds = Rect::new(20, 20, 10, 2);
            let mut position = out_of_bounds.as_position();
            span.render_wrapping(&mut position, style, buf.area, &mut buf);
            assert_eq!(buf, Buffer::empty(buf.area));
            assert_eq!(position, Position { x: 20, y: 20 });
        }

        #[test]
        fn render_wrapping_patches_existing_style() {
            let style = Style::new().green().on_yellow();
            let span = Span::styled("Hello, world! Hello, Rustaceans!", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 10, 4));
            buf.set_style(buf.area, Style::new().italic());
            let mut position = buf.area.as_position();
            span.render_wrapping(&mut position, style, buf.area, &mut buf);
            let expected = Buffer::with_lines([
                Line::from(Span::from("Hello, wor").green().on_yellow().italic()),
                Line::from(Span::from("ld! Hello,").green().on_yellow().italic()),
                Line::from(Span::from(" Rustacean").green().on_yellow().italic()),
                Line::from(vec!["s!".green().on_yellow().italic(), "        ".italic()]),
            ]);
            assert_eq!(buf, expected);
            assert_eq!(position, Position { x: 2, y: 3 });
        }

        #[test]
        fn render_wrapping_multi_width_symbol() {
            let style = Style::new().green().on_yellow();
            let span = Span::styled("Hello, world 😃 Hello, Rustaceans 😃", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 10, 4));
            let mut position = buf.area.as_position();
            span.render_wrapping(&mut position, style, buf.area, &mut buf);
            let expected = Buffer::with_lines([
                Line::from(Span::from("Hello, wor").green().on_yellow()),
                Line::from(Span::from("ld 😃 Hell").green().on_yellow()),
                Line::from(Span::from("o, Rustace").green().on_yellow()),
                Line::from(Span::from("ans 😃").green().on_yellow()),
            ]);
            assert_eq!(buf, expected);
            assert_eq!(position, Position { x: 6, y: 3 });
        }

        #[test]
        fn render_wrapping_multi_width_symbol_with_wrapping() {
            let style = Style::new().green().on_yellow();
            let span = Span::styled("Hello, world 😃", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 14, 2));
            let mut position = buf.area.as_position();
            span.render_wrapping(&mut position, style, buf.area, &mut buf);
            let expected = Buffer::with_lines([
                Line::from(vec!["Hello, world ".green().on_yellow(), " ".into()]),
                Line::from(Span::from("😃").green().on_yellow()),
            ]);
            assert_eq!(buf, expected);
            assert_eq!(position, Position { x: 2, y: 1 });
        }

        #[test]
        fn render_wrapping_multi_width_symbol_truncates_entire_symbol() {
            let style = Style::new().green().on_yellow();
            let span = Span::styled("Hello, world 😃", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 14, 1));
            let mut position = buf.area.as_position();
            span.render_wrapping(&mut position, style, buf.area, &mut buf);
            let expected = Buffer::with_lines([Line::from(vec![
                "Hello, world ".green().on_yellow(),
                " ".into(),
            ])]);
            assert_eq!(buf, expected);
            assert_eq!(position, Position { x: 0, y: 1 });
        }

        #[test]
        fn render_wrapping_first_zero_width() {
            let style = Style::new();
            let span = Span::styled("\u{200B}Hello", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
            let mut position = buf.area.as_position();
            span.render_wrapping(&mut position, style, buf.area, &mut buf);
            assert_eq!(
                buf.content(),
                [
                    Cell::new("\u{200B}H"),
                    Cell::new("e"),
                    Cell::new("l"),
                    Cell::new("l"),
                    Cell::new("o"),
                ]
            );
            assert_eq!(position, Position { x: 0, y: 1 });
        }

        #[test]
        fn render_wrapping_second_zero_width() {
            let style = Style::new();
            let span = Span::styled("H\u{200B}ello", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
            let mut position = buf.area.as_position();
            span.render_wrapping(&mut position, style, buf.area, &mut buf);
            assert_eq!(
                buf.content(),
                [
                    Cell::new("H\u{200B}"),
                    Cell::new("e"),
                    Cell::new("l"),
                    Cell::new("l"),
                    Cell::new("o"),
                ]
            );
            assert_eq!(position, Position { x: 0, y: 1 });
        }

        #[test]
        fn render_wrapping_middle_zero_width() {
            let style = Style::new();
            let span = Span::styled("He\u{200B}l\u{200B}l\u{200B}o", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
            let mut position = buf.area.as_position();
            span.render_wrapping(&mut position, style, buf.area, &mut buf);
            assert_eq!(
                buf.content(),
                [
                    Cell::new("H"),
                    Cell::new("e\u{200B}"),
                    Cell::new("l\u{200B}"),
                    Cell::new("l\u{200B}"),
                    Cell::new("o"),
                ]
            );
            assert_eq!(position, Position { x: 0, y: 1 });
        }

        #[test]
        fn render_wrapping_last_zero_width() {
            let style = Style::new();
            let span = Span::styled("Hello\u{200B}", style);
            let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
            let mut position = buf.area.as_position();
            span.render_wrapping(&mut position, style, buf.area, &mut buf);
            assert_eq!(
                buf.content(),
                [
                    Cell::new("H"),
                    Cell::new("e"),
                    Cell::new("l"),
                    Cell::new("l"),
                    Cell::new("o\u{200B}"),
                ]
            );
            assert_eq!(position, Position { x: 0, y: 1 });
        }
    }

    mod position {
        use super::*;

        #[rstest]
        #[case(Position { x: 0, y: 0 }, 3, Rect::new(0, 0, 5, 3), Position { x: 3, y: 0 })]
        #[case(Position { x: 4, y: 0 }, 2, Rect::new(0, 0, 5, 3), Position { x: 1, y: 1 })]
        #[case(Position { x: 3, y: 0 }, 13, Rect::new(0, 0, 5, 3), Position { x: 1, y: 3 })]
        #[case(Position { x: 6, y: 8 }, 13, Rect::new(0, 0, 5, 3), Position { x: 6, y: 8 })]
        fn step_wrapping_mut(
            #[case] mut position: Position,
            #[case] width: u16,
            #[case] area: Rect,
            #[case] expected: Position,
        ) {
            position.step_wrapping_mut(width, area);
            assert_eq!(position, expected);
        }

        #[rstest]
        #[case(Position { x: 0, y: 0 }, 3, Rect::new(0, 0, 5, 3), Some(Position { x: 3, y: 0 }), Position { x: 0, y: 0 })]
        #[case(Position { x: 4, y: 0 }, 2, Rect::new(0, 0, 5, 3), Some(Position { x: 2, y: 1 }), Position { x: 0, y: 1 })]
        #[case(Position { x: 4, y: 2 }, 3, Rect::new(0, 0, 5, 3), None, Position { x: 0, y: 3 })]
        #[case(Position { x: 0, y: 0 }, 0, Rect::new(0, 0, 5, 3), Some(Position { x: 0, y: 0 }), Position { x: 0, y: 0 })]
        #[case(Position { x: 2, y: 1 }, 2, Rect::new(0, 0, 5, 3), Some(Position { x: 4, y: 1 }), Position { x: 2, y: 1 })]
        fn try_step_wrapping_mut(
            #[case] mut position: Position,
            #[case] symbol_width: u16,
            #[case] area: Rect,
            #[case] expected_result: Option<Position>,
            #[case] expected_position: Position,
        ) {
            let result = position.try_step_wrapping_mut(symbol_width, area);
            assert_eq!(result, expected_result);
            assert_eq!(position, expected_position);
        }

        #[rstest]
        #[case(
            Position { x: 0, y: 0 },
            Position { x: 3, y: 0 },
            Rect::new(0, 0, 5, 3),
            vec![
                Position { x: 0, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 2, y: 0 },
            ]
        )]
        #[case(
            Position { x: 4, y: 0 },
            Position { x: 2, y: 1 },
            Rect::new(0, 0, 5, 3),
            vec![
                Position { x: 4, y: 0 },
                Position { x: 0, y: 1 },
                Position { x: 1, y: 1 },
            ]
        )]
        #[case(
            Position { x: 4, y: 2 },
            Position { x: 3, y: 3 },
            Rect::new(0, 0, 5, 3),
            vec![
                Position { x: 4, y: 2 },
            ]
        )]
        #[case(
            Position { x: 0, y: 0 },
            Position { x: 0, y: 0 },
            Rect::new(0, 0, 5, 3),
            vec![]
        )]
        fn iter_wrapping_to(
            #[case] start: Position,
            #[case] end: Position,
            #[case] area: Rect,
            #[case] expected: Vec<Position>,
        ) {
            let result: Vec<_> = start.iter_wrapping_to(end, area).collect();
            assert_eq!(result, expected);
        }
    }

    #[rstest]
    #[case::one_line(
        InlineText::raw("Hello, world!").space(1),
        "Hello, world!",
    )]
    #[case::multiple_lines(
        InlineText::raw("Hello, world!\nHello, Rustaceans!").space(1),
        "Hello, world! Hello, Rustaceans!",
    )]
    #[case::styled(
        InlineText::styled(
            "Hello, world!\nHello, Rustaceans!",
            Style::new().yellow().italic(),
        ).space(1),
        "Hello, world! Hello, Rustaceans!",
    )]
    #[cfg(debug_assertions)]
    fn display(#[case] inline: InlineText, #[case] expected: &str) {
        assert_eq!(format!("{inline}"), expected);
    }

    #[rstest]
    #[case::one_line(
        InlineText::raw("Hello, world!").space(1),
        "Hello, world!",
    )]
    #[case::multiple_lines(
        InlineText::raw("Hello, world!\nHello, Rustaceans!").space(1),
        "Hello, world! Hello, Rustaceans!",
    )]
    #[case::styled(
        InlineText::styled(
            "Hello, world!\nHello, Rustaceans!",
            Style::new().yellow().italic(),
        ).space(1),
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
        fn render_with_space_left_aligned() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .space(1)
                .alignment(Alignment::Left);
            let area = Rect::new(0, 0, 15, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello, world!  "]));
        }

        #[test]
        fn render_with_space_right_aligned() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .space(1)
                .alignment(Alignment::Right);
            let area = Rect::new(0, 0, 15, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["  Hello, world!"]));
        }

        #[test]
        fn render_with_space_centered_odd() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .space(1)
                .alignment(Alignment::Center);
            let area = Rect::new(0, 0, 15, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([" Hello, world! "]));
        }

        #[test]
        fn render_with_space_centered_even() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .space(1)
                .alignment(Alignment::Center);
            let area = Rect::new(0, 0, 16, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([" Hello, world!  "]));
        }

        #[test]
        fn render_left_aligned_with_space_and_truncation() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .space(1)
                .alignment(Alignment::Left);
            let area = Rect::new(0, 0, 7, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello, "]));
        }

        #[test]
        fn render_right_aligned_with_space_and_truncation() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .space(1)
                .alignment(Alignment::Right);
            let area = Rect::new(0, 0, 7, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([" world!"]));
        }

        #[test]
        fn render_centered_odd_with_space_and_truncation() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .space(1)
                .alignment(Alignment::Center);
            let area = Rect::new(0, 0, 7, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["lo, wor"]));
        }

        #[test]
        fn render_centered_even_with_space_and_truncation() {
            let inline = InlineText::from(vec!["Hello,", "world!"])
                .space(1)
                .alignment(Alignment::Center);
            let area = Rect::new(0, 0, 6, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["lo, wo"]));
        }

        #[test]
        fn render_multiple_aligned_left_and_center() {
            let inline = InlineText::from(vec![
                Line::from("Hello, world!").alignment(Alignment::Left),
                Line::from("Hello, Rustaceans!").alignment(Alignment::Center),
            ]);
            let area = Rect::new(0, 0, 100, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines([
                    "Hello, world!                            Hello, Rustaceans!                                         "
                ])
            );
        }

        #[test]
        fn render_multiple_aligned_left_and_right() {
            let inline = InlineText::from(vec![
                Line::from("Hello, world!").alignment(Alignment::Left),
                Line::from("Greetings, fellow developers!").alignment(Alignment::Right),
            ]);
            let area = Rect::new(0, 0, 100, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines([
                    "Hello, world!                                                          Greetings, fellow developers!"
                ])
            );
        }

        #[test]
        fn render_multiple_aligned_center_and_right() {
            let inline = InlineText::from(vec![
                Line::from("Hello, Rustaceans!").alignment(Alignment::Center),
                Line::from("Greetings, fellow developers!").alignment(Alignment::Right),
            ]);
            let area = Rect::new(0, 0, 100, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines([
                    "                                         Hello, Rustaceans!            Greetings, fellow developers!"
                ])
            );
        }

        #[test]
        fn render_multiple_aligned_left_and_center_and_right() {
            let inline = InlineText::from(vec![
                Line::from("Hello, world!").alignment(Alignment::Left),
                Line::from("Hello, Rustaceans!").alignment(Alignment::Center),
                Line::from("Greetings, fellow developers!").alignment(Alignment::Right),
            ]);
            let area = Rect::new(0, 0, 100, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines([
                    "Hello, world!                            Hello, Rustaceans!            Greetings, fellow developers!"
                ])
            );
        }

        #[test]
        fn render_multiple_aligned_left_and_center_with_truncation() {
            let inline = InlineText::from(vec![
                Line::from("Hello, world!").alignment(Alignment::Left),
                Line::from("Hello, Rustaceans!").alignment(Alignment::Center),
            ]);
            let area = Rect::new(0, 0, 30, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello, wolo, Rustaceans!      "]));
        }

        #[test]
        fn render_multiple_aligned_left_and_right_with_truncation() {
            let inline = InlineText::from(vec![
                Line::from("Hello, world!").alignment(Alignment::Left),
                Line::from("Greetings, fellow developers!").alignment(Alignment::Right),
            ]);
            let area = Rect::new(0, 0, 30, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello, ngs, fellow developers!"]));
        }

        #[test]
        fn render_multiple_aligned_center_and_right_with_truncation() {
            let inline = InlineText::from(vec![
                Line::from("Hello, Rustaceans!").alignment(Alignment::Center),
                Line::from("Greetings, fellow developers!").alignment(Alignment::Right),
            ]);
            let area = Rect::new(0, 0, 30, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["      Hello, Rulow developers!"]));
        }

        #[test]
        fn render_multiple_aligned_left_and_center_and_right_with_truncation() {
            let inline = InlineText::from(vec![
                Line::from("Hello, world!").alignment(Alignment::Left),
                Line::from("Hello, Rustaceans!").alignment(Alignment::Center),
                Line::from("Greetings, fellow developers!").alignment(Alignment::Right),
            ]);
            let area = Rect::new(0, 0, 30, 1);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["Hello, wolo, Rulow developers!"]));
        }

        #[test]
        fn render_render_left_aligned_vertical() {
            let inline = InlineText::from(vec!["Hello, world!"]).alignment(Alignment::Left);
            let area = Rect::new(0, 0, 1, 15);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines([
                    "H", "e", "l", "l", "o", ",", " ", "w", "o", "r", "l", "d", "!", " ", " ",
                ])
            );
        }

        #[test]
        fn render_right_aligned_vertical() {
            let inline = InlineText::from("Hello, world!").alignment(Alignment::Right);
            let area = Rect::new(0, 0, 1, 15);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines([
                    " ", " ", "H", "e", "l", "l", "o", ",", " ", "w", "o", "r", "l", "d", "!",
                ])
            );
        }

        #[test]
        fn render_centered_odd_vertical() {
            let inline = InlineText::from("Hello, world!").alignment(Alignment::Center);
            let area = Rect::new(0, 0, 1, 15);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines([
                    " ", "H", "e", "l", "l", "o", ",", " ", "w", "o", "r", "l", "d", "!", " ",
                ])
            );
        }

        #[test]
        fn render_centered_even_vertical() {
            let inline = InlineText::from("Hello, world!").alignment(Alignment::Center);
            let area = Rect::new(0, 0, 1, 16);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines([
                    " ", "H", "e", "l", "l", "o", ",", " ", "w", "o", "r", "l", "d", "!", " ", " ",
                ])
            );
        }

        #[test]
        fn render_left_aligned_with_truncation_vertical() {
            let inline = InlineText::from("Hello, world!").alignment(Alignment::Left);
            let area = Rect::new(0, 0, 1, 7);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines(["H", "e", "l", "l", "o", ",", " ",])
            );
        }

        #[test]
        fn render_right_aligned_with_truncation_vertical() {
            let inline = InlineText::from("Hello, world!").alignment(Alignment::Right);
            let area = Rect::new(0, 0, 1, 7);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines([" ", "w", "o", "r", "l", "d", "!",])
            );
        }

        #[test]
        fn render_centered_odd_with_truncation_vertical() {
            let inline = InlineText::from("Hello, world!").alignment(Alignment::Center);
            let area = Rect::new(0, 0, 1, 7);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines(["l", "o", ",", " ", "w", "o", "r",])
            );
        }

        #[test]
        fn render_centered_even_with_truncation_vertical() {
            let inline = InlineText::from("Hello, world!").alignment(Alignment::Center);
            let area = Rect::new(0, 0, 1, 6);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["l", "o", ",", " ", "w", "o",]));
        }

        #[test]
        fn render_multiple_aligned_left_and_center_vertical() {
            let inline = InlineText::from(vec![
                Line::from("Hello").alignment(Alignment::Left),
                Line::from("World").alignment(Alignment::Center),
            ]);
            let area = Rect::new(0, 0, 1, 30);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines([
                    "H", "e", "l", "l", "o", " ", " ", " ", " ", " ", " ", " ", "W", "o", "r", "l",
                    "d", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ",
                ])
            );
        }

        #[test]
        fn render_multiple_aligned_left_and_right_vertical() {
            let inline = InlineText::from(vec![
                Line::from("Hello").alignment(Alignment::Left),
                Line::from("Greetings").alignment(Alignment::Right),
            ]);
            let area = Rect::new(0, 0, 1, 30);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines([
                    "H", "e", "l", "l", "o", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ",
                    " ", " ", " ", " ", " ", "G", "r", "e", "e", "t", "i", "n", "g", "s",
                ])
            );
        }

        #[test]
        fn render_multiple_aligned_center_and_right_vertical() {
            let inline = InlineText::from(vec![
                Line::from("World").alignment(Alignment::Center),
                Line::from("Greetings").alignment(Alignment::Right),
            ]);
            let area = Rect::new(0, 0, 1, 30);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines([
                    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "W", "o", "r", "l",
                    "d", " ", " ", " ", " ", "G", "r", "e", "e", "t", "i", "n", "g", "s",
                ])
            );
        }

        #[test]
        fn render_multiple_aligned_left_and_center_and_right_vertical() {
            let inline = InlineText::from(vec![
                Line::from("Hello").alignment(Alignment::Left),
                Line::from("World").alignment(Alignment::Center),
                Line::from("Greetings").alignment(Alignment::Right),
            ]);
            let area = Rect::new(0, 0, 1, 30);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines([
                    "H", "e", "l", "l", "o", " ", " ", " ", " ", " ", " ", " ", "W", "o", "r", "l",
                    "d", " ", " ", " ", " ", "G", "r", "e", "e", "t", "i", "n", "g", "s",
                ])
            );
        }

        #[test]
        fn render_multiple_aligned_left_and_center_vertical_with_truncation() {
            let inline = InlineText::from(vec![
                Line::from("Hello").alignment(Alignment::Left),
                Line::from("World").alignment(Alignment::Center),
            ]);
            let area = Rect::new(0, 0, 1, 10);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines(["H", "e", "l", "o", "r", "l", "d", " ", " ", " ",])
            );
        }

        #[test]
        fn render_multiple_aligned_center_and_right_vertical_with_truncation() {
            let inline = InlineText::from(vec![
                Line::from("World").alignment(Alignment::Center),
                Line::from("Greetings").alignment(Alignment::Right),
            ]);
            let area = Rect::new(0, 0, 1, 10);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines([" ", " ", "W", "o", "e", "t", "i", "n", "g", "s",])
            );
        }

        #[test]
        fn render_multiple_aligned_left_and_center_and_right_vertical_with_truncation() {
            let inline = InlineText::from(vec![
                Line::from("Hello").alignment(Alignment::Left),
                Line::from("World").alignment(Alignment::Center),
                Line::from("Greetings").alignment(Alignment::Right),
            ]);
            let area = Rect::new(0, 0, 1, 10);
            let mut buf = Buffer::empty(area);
            inline.render(area, &mut buf);
            assert_eq!(
                buf,
                Buffer::with_lines(["H", "e", "l", "o", "e", "t", "i", "n", "g", "s",])
            );
        }
    }
}
