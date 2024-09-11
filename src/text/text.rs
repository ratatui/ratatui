#![warn(missing_docs)]
use std::{borrow::Cow, fmt};

use crate::{prelude::*, style::Styled};

/// A string split over one or more lines.
///
/// [`Text`] is used wherever text is displayed in the terminal and represents one or more [`Line`]s
/// of text. When a [`Text`] is rendered, each line is rendered as a single line of text from top to
/// bottom of the area. The text can be styled and aligned.
///
/// # Constructor Methods
///
/// - [`Text::raw`] creates a `Text` (potentially multiple lines) with no style.
/// - [`Text::styled`] creates a `Text` (potentially multiple lines) with a style.
/// - [`Text::default`] creates a `Text` with empty content and the default style.
///
/// # Conversion Methods
///
/// - [`Text::from`] creates a `Text` from a `String`.
/// - [`Text::from`] creates a `Text` from a `&str`.
/// - [`Text::from`] creates a `Text` from a `Cow<str>`.
/// - [`Text::from`] creates a `Text` from a [`Span`].
/// - [`Text::from`] creates a `Text` from a [`Line`].
/// - [`Text::from`] creates a `Text` from a `Vec<Line>`.
/// - [`Text::from_iter`] creates a `Text` from an iterator of items that can be converted into
///   `Line`.
///
/// # Setter Methods
///
/// These methods are fluent setters. They return a `Text` with the property set.
///
/// - [`Text::style`] sets the style of this `Text`.
/// - [`Text::alignment`] sets the alignment for this `Text`.
/// - [`Text::left_aligned`] sets the alignment to [`Alignment::Left`].
/// - [`Text::centered`] sets the alignment to [`Alignment::Center`].
/// - [`Text::right_aligned`] sets the alignment to [`Alignment::Right`].
///
/// # Iteration Methods
///
/// - [`Text::iter`] returns an iterator over the lines of the text.
/// - [`Text::iter_mut`] returns an iterator that allows modifying each line.
/// - [`Text::into_iter`] returns an iterator over the lines of the text.
///
/// # Other Methods
///
/// - [`Text::width`] returns the max width of all the lines.
/// - [`Text::height`] returns the height.
/// - [`Text::patch_style`] patches the style of this `Text`, adding modifiers from the given style.
/// - [`Text::reset_style`] resets the style of the `Text`.
/// - [`Text::push_line`] adds a line to the text.
/// - [`Text::push_span`] adds a span to the last line of the text.
///
/// # Examples
///
/// ## Creating Text
///
/// A [`Text`], like a [`Line`], can be constructed using one of the many `From` implementations or
/// via the [`Text::raw`] and [`Text::styled`] methods. Helpfully, [`Text`] also implements
/// [`core::iter::Extend`] which enables the concatenation of several [`Text`] blocks.
///
/// ```rust
/// use std::{borrow::Cow, iter};
///
/// use ratatui::prelude::*;
///
/// let style = Style::new().yellow().italic();
/// let text = Text::raw("The first line\nThe second line").style(style);
/// let text = Text::styled("The first line\nThe second line", style);
/// let text = Text::styled(
///     "The first line\nThe second line",
///     (Color::Yellow, Modifier::ITALIC),
/// );
///
/// let text = Text::from("The first line\nThe second line");
/// let text = Text::from(String::from("The first line\nThe second line"));
/// let text = Text::from(Cow::Borrowed("The first line\nThe second line"));
/// let text = Text::from(Span::styled("The first line\nThe second line", style));
/// let text = Text::from(Line::from("The first line"));
/// let text = Text::from(vec![
///     Line::from("The first line"),
///     Line::from("The second line"),
/// ]);
/// let text = Text::from_iter(iter::once("The first line").chain(iter::once("The second line")));
///
/// let mut text = Text::default();
/// text.extend(vec![
///     Line::from("The first line"),
///     Line::from("The second line"),
/// ]);
/// text.extend(Text::from("The third line\nThe fourth line"));
/// ```
///
/// ## Styling Text
///
/// The text's [`Style`] is used by the rendering widget to determine how to style the text. Each
/// [`Line`] in the text will be styled with the [`Style`] of the text, and then with its own
/// [`Style`]. `Text` also implements [`Styled`] which means you can use the methods of the
/// [`Stylize`] trait.
///
/// ```rust
/// # use ratatui::prelude::*;
/// let text = Text::from("The first line\nThe second line").style(Style::new().yellow().italic());
/// let text = Text::from("The first line\nThe second line")
///     .yellow()
///     .italic();
/// let text = Text::from(vec![
///     Line::from("The first line").yellow(),
///     Line::from("The second line").yellow(),
/// ])
/// .italic();
/// ```
///
/// ## Aligning Text
/// The text's [`Alignment`] can be set using [`Text::alignment`] or the related helper methods.
/// Lines composing the text can also be individually aligned with [`Line::alignment`].
///
/// ```rust
/// # use ratatui::prelude::*;
/// let text = Text::from("The first line\nThe second line").alignment(Alignment::Right);
/// let text = Text::from("The first line\nThe second line").right_aligned();
/// let text = Text::from(vec![
///     Line::from("The first line").left_aligned(),
///     Line::from("The second line").right_aligned(),
///     Line::from("The third line"),
/// ])
/// .centered();
/// ```
///
/// ## Rendering Text
/// `Text` implements the [`Widget`] trait, which means it can be rendered to a [`Buffer`] or to a
/// [`Frame`].
///
/// ```rust
/// # use ratatui::prelude::*;
/// // within another widget's `render` method:
/// # fn render(area: Rect, buf: &mut Buffer) {
/// let text = Text::from("The first line\nThe second line");
/// text.render(area, buf);
/// # }
///
/// // within a terminal.draw closure:
/// # fn draw(frame: &mut Frame, area: Rect) {
/// let text = Text::from("The first line\nThe second line");
/// frame.render_widget(text, area);
/// # }
/// ```
///
/// ## Rendering Text with a Paragraph Widget
///
/// Usually apps will use the [`Paragraph`] widget instead of rendering a `Text` directly as it
/// provides more functionality.
///
/// ```rust
/// # use ratatui::{prelude::*, widgets::*};
/// # fn render(area: Rect, buf: &mut Buffer) {
/// let text = Text::from("The first line\nThe second line");
/// let paragraph = Paragraph::new(text)
///     .wrap(Wrap { trim: true })
///     .scroll((1, 1))
///     .render(area, buf);
/// # }
/// ```
///
/// [`Paragraph`]: crate::widgets::Paragraph
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Text<'a> {
    /// The lines that make up this piece of text.
    pub lines: Vec<Line<'a>>,
    /// The style of this text.
    pub style: Style,
    /// The alignment of this text.
    pub alignment: Option<Alignment>,
}

impl<'a> Text<'a> {
    /// Create some text (potentially multiple lines) with no style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// Text::raw("The first line\nThe second line");
    /// Text::raw(String::from("The first line\nThe second line"));
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

    /// Create some text (potentially multiple lines) with a style.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let style = Style::default()
    ///     .fg(Color::Yellow)
    ///     .add_modifier(Modifier::ITALIC);
    /// Text::styled("The first line\nThe second line", style);
    /// Text::styled(String::from("The first line\nThe second line"), style);
    /// ```
    pub fn styled<T, S>(content: T, style: S) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Style>,
    {
        Self::raw(content).patch_style(style)
    }

    /// Returns the max width of all the lines.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let text = Text::from("The first line\nThe second line");
    /// assert_eq!(15, text.width());
    /// ```
    pub fn width(&self) -> usize {
        self.iter().map(Line::width).max().unwrap_or_default()
    }

    /// Returns the height.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let text = Text::from("The first line\nThe second line");
    /// assert_eq!(2, text.height());
    /// ```
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /// Sets the style of this text.
    ///
    /// Defaults to [`Style::default()`].
    ///
    /// Note: This field was added in v0.26.0. Prior to that, the style of a text was determined
    /// only by the style of each [`Line`] contained in the line. For this reason, this field may
    /// not be supported by all widgets (outside of the `ratatui` crate itself).
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// # Examples
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let mut line = Text::from("foo").style(Style::new().red());
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Patches the style of this Text, adding modifiers from the given style.
    ///
    /// This is useful for when you want to apply a style to a text that already has some styling.
    /// In contrast to [`Text::style`], this method will not overwrite the existing style, but
    /// instead will add the given style's modifiers to this text's style.
    ///
    /// `Text` also implements [`Styled`] which means you can use the methods of the [`Stylize`]
    /// trait.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let raw_text = Text::styled("The first line\nThe second line", Modifier::ITALIC);
    /// let styled_text = Text::styled(
    ///     String::from("The first line\nThe second line"),
    ///     (Color::Yellow, Modifier::ITALIC),
    /// );
    /// assert_ne!(raw_text, styled_text);
    ///
    /// let raw_text = raw_text.patch_style(Color::Yellow);
    /// assert_eq!(raw_text, styled_text);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn patch_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = self.style.patch(style);
        self
    }

    /// Resets the style of the Text.
    ///
    /// Equivalent to calling [`patch_style(Style::reset())`](Text::patch_style).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let text = Text::styled(
    ///     "The first line\nThe second line",
    ///     (Color::Yellow, Modifier::ITALIC),
    /// );
    ///
    /// let text = text.reset_style();
    /// assert_eq!(Style::reset(), text.style);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn reset_style(self) -> Self {
        self.patch_style(Style::reset())
    }

    /// Sets the alignment for this text.
    ///
    /// Defaults to: [`None`], meaning the alignment is determined by the rendering widget.
    /// Setting the alignment of a Text generally overrides the alignment of its
    /// parent Widget.
    ///
    /// Alignment can be set individually on each line to override this text's alignment.
    ///
    /// # Examples
    ///
    /// Set alignment to the whole text.
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let mut text = Text::from("Hi, what's up?");
    /// assert_eq!(None, text.alignment);
    /// assert_eq!(
    ///     Some(Alignment::Right),
    ///     text.alignment(Alignment::Right).alignment
    /// )
    /// ```
    ///
    /// Set a default alignment and override it on a per line basis.
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let text = Text::from(vec![
    ///     Line::from("left").alignment(Alignment::Left),
    ///     Line::from("default"),
    ///     Line::from("default"),
    ///     Line::from("right").alignment(Alignment::Right),
    /// ])
    /// .alignment(Alignment::Center);
    /// ```
    ///
    /// Will render the following
    ///
    /// ```plain
    /// left
    ///   default
    ///   default
    ///       right
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn alignment(self, alignment: Alignment) -> Self {
        Self {
            alignment: Some(alignment),
            ..self
        }
    }

    /// Left-aligns the whole text.
    ///
    /// Convenience shortcut for `Text::alignment(Alignment::Left)`.
    /// Setting the alignment of a Text generally overrides the alignment of its
    /// parent Widget, with the default alignment being inherited from the parent.
    ///
    /// Alignment can be set individually on each line to override this text's alignment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let text = Text::from("Hi, what's up?").left_aligned();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn left_aligned(self) -> Self {
        self.alignment(Alignment::Left)
    }

    /// Center-aligns the whole text.
    ///
    /// Convenience shortcut for `Text::alignment(Alignment::Center)`.
    /// Setting the alignment of a Text generally overrides the alignment of its
    /// parent Widget, with the default alignment being inherited from the parent.
    ///
    /// Alignment can be set individually on each line to override this text's alignment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let text = Text::from("Hi, what's up?").centered();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn centered(self) -> Self {
        self.alignment(Alignment::Center)
    }

    /// Right-aligns the whole text.
    ///
    /// Convenience shortcut for `Text::alignment(Alignment::Right)`.
    /// Setting the alignment of a Text generally overrides the alignment of its
    /// parent Widget, with the default alignment being inherited from the parent.
    ///
    /// Alignment can be set individually on each line to override this text's alignment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let text = Text::from("Hi, what's up?").right_aligned();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn right_aligned(self) -> Self {
        self.alignment(Alignment::Right)
    }

    /// Returns an iterator over the lines of the text.
    pub fn iter(&self) -> std::slice::Iter<Line<'a>> {
        self.lines.iter()
    }

    /// Returns an iterator that allows modifying each line.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Line<'a>> {
        self.lines.iter_mut()
    }

    /// Adds a line to the text.
    ///
    /// `line` can be any type that can be converted into a `Line`. For example, you can pass a
    /// `&str`, a `String`, a `Span`, or a `Line`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let mut text = Text::from("Hello, world!");
    /// text.push_line(Line::from("How are you?"));
    /// text.push_line(Span::from("How are you?"));
    /// text.push_line("How are you?");
    /// ```
    pub fn push_line<T: Into<Line<'a>>>(&mut self, line: T) {
        self.lines.push(line.into());
    }

    /// Adds a span to the last line of the text.
    ///
    /// `span` can be any type that is convertible into a `Span`. For example, you can pass a
    /// `&str`, a `String`, or a `Span`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let mut text = Text::from("Hello, world!");
    /// text.push_span(Span::from("How are you?"));
    /// text.push_span("How are you?");
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

impl<'a> IntoIterator for Text<'a> {
    type Item = Line<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter()
    }
}

impl<'a> IntoIterator for &'a Text<'a> {
    type Item = &'a Line<'a>;
    type IntoIter = std::slice::Iter<'a, Line<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Text<'a> {
    type Item = &'a mut Line<'a>;
    type IntoIter = std::slice::IterMut<'a, Line<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a> From<String> for Text<'a> {
    fn from(s: String) -> Self {
        Self::raw(s)
    }
}

impl<'a> From<&'a str> for Text<'a> {
    fn from(s: &'a str) -> Self {
        Self::raw(s)
    }
}

impl<'a> From<Cow<'a, str>> for Text<'a> {
    fn from(s: Cow<'a, str>) -> Self {
        Self::raw(s)
    }
}

impl<'a> From<Span<'a>> for Text<'a> {
    fn from(span: Span<'a>) -> Self {
        Self {
            lines: vec![Line::from(span)],
            ..Default::default()
        }
    }
}

impl<'a> From<Line<'a>> for Text<'a> {
    fn from(line: Line<'a>) -> Self {
        Self {
            lines: vec![line],
            ..Default::default()
        }
    }
}

impl<'a> From<Vec<Line<'a>>> for Text<'a> {
    fn from(lines: Vec<Line<'a>>) -> Self {
        Self {
            lines,
            ..Default::default()
        }
    }
}

impl<'a, T> FromIterator<T> for Text<'a>
where
    T: Into<Line<'a>>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let lines = iter.into_iter().map(Into::into).collect();
        Self {
            lines,
            ..Default::default()
        }
    }
}

impl<'a> std::ops::Add<Line<'a>> for Text<'a> {
    type Output = Self;

    fn add(mut self, line: Line<'a>) -> Self::Output {
        self.push_line(line);
        self
    }
}

/// Adds two `Text` together.
///
/// This ignores the style and alignment of the second `Text`.
impl<'a> std::ops::Add<Self> for Text<'a> {
    type Output = Self;

    fn add(mut self, text: Self) -> Self::Output {
        self.lines.extend(text.lines);
        self
    }
}

impl<'a> std::ops::AddAssign<Line<'a>> for Text<'a> {
    fn add_assign(&mut self, line: Line<'a>) {
        self.push_line(line);
    }
}

impl<'a, T> Extend<T> for Text<'a>
where
    T: Into<Line<'a>>,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let lines = iter.into_iter().map(Into::into);
        self.lines.extend(lines);
    }
}

/// A trait for converting a value to a [`Text`].
///
/// This trait is automatically implemented for any type that implements the [`Display`] trait. As
/// such, `ToText` shouldn't be implemented directly: [`Display`] should be implemented instead, and
/// you get the `ToText` implementation for free.
///
/// [`Display`]: std::fmt::Display
pub trait ToText {
    /// Converts the value to a [`Text`].
    fn to_text(&self) -> Text<'_>;
}

/// # Panics
///
/// In this implementation, the `to_text` method panics if the `Display` implementation returns an
/// error. This indicates an incorrect `Display` implementation since `fmt::Write for String` never
/// returns an error itself.
impl<T: fmt::Display> ToText for T {
    fn to_text(&self) -> Text {
        Text::raw(self.to_string())
    }
}

impl fmt::Display for Text<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((last, rest)) = self.lines.split_last() {
            for line in rest {
                writeln!(f, "{line}")?;
            }
            write!(f, "{last}")?;
        }
        Ok(())
    }
}

impl Widget for Text<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

impl WidgetRef for Text<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        buf.set_style(area, self.style);
        for (line, row) in self.iter().zip(area.rows()) {
            let line_width = line.width() as u16;

            let x_offset = match (self.alignment, line.alignment) {
                (Some(Alignment::Center), None) => area.width.saturating_sub(line_width) / 2,
                (Some(Alignment::Right), None) => area.width.saturating_sub(line_width),
                _ => 0,
            };

            let line_area = Rect {
                x: area.x + x_offset,
                y: row.y,
                width: area.width - x_offset,
                height: 1,
            };

            line.render(line_area, buf);
        }
    }
}

impl<'a> Styled for Text<'a> {
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

    #[fixture]
    fn small_buf() -> Buffer {
        Buffer::empty(Rect::new(0, 0, 10, 1))
    }

    #[test]
    fn raw() {
        let text = Text::raw("The first line\nThe second line");
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn styled() {
        let style = Style::new().yellow().italic();
        let styled_text = Text::styled("The first line\nThe second line", style);

        let mut text = Text::raw("The first line\nThe second line");
        text.style = style;

        assert_eq!(styled_text, text);
    }

    #[test]
    fn width() {
        let text = Text::from("The first line\nThe second line");
        assert_eq!(15, text.width());
    }

    #[test]
    fn height() {
        let text = Text::from("The first line\nThe second line");
        assert_eq!(2, text.height());
    }

    #[test]
    fn patch_style() {
        let style = Style::new().yellow().italic();
        let style2 = Style::new().red().underlined();
        let text = Text::styled("The first line\nThe second line", style).patch_style(style2);

        let expected_style = Style::new().red().italic().underlined();
        let expected_text = Text::styled("The first line\nThe second line", expected_style);

        assert_eq!(text, expected_text);
    }

    #[test]
    fn reset_style() {
        let style = Style::new().yellow().italic();
        let text = Text::styled("The first line\nThe second line", style).reset_style();

        assert_eq!(text.style, Style::reset());
    }

    #[test]
    fn from_string() {
        let text = Text::from(String::from("The first line\nThe second line"));
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn from_str() {
        let text = Text::from("The first line\nThe second line");
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn from_cow() {
        let text = Text::from(Cow::Borrowed("The first line\nThe second line"));
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn from_span() {
        let style = Style::new().yellow().italic();
        let text = Text::from(Span::styled("The first line\nThe second line", style));
        assert_eq!(
            text.lines,
            vec![Line::from(Span::styled(
                "The first line\nThe second line",
                style
            ))]
        );
    }

    #[test]
    fn from_line() {
        let text = Text::from(Line::from("The first line"));
        assert_eq!(text.lines, [Line::from("The first line")]);
    }

    #[rstest]
    #[case(42, Text::from("42"))]
    #[case("just\ntesting", Text::from("just\ntesting"))]
    #[case(true, Text::from("true"))]
    #[case(6.66, Text::from("6.66"))]
    #[case('a', Text::from("a"))]
    #[case(String::from("hello"), Text::from("hello"))]
    #[case(-1, Text::from("-1"))]
    #[case("line1\nline2", Text::from("line1\nline2"))]
    #[case(
        "first line\nsecond line\nthird line",
        Text::from("first line\nsecond line\nthird line")
    )]
    #[case("trailing newline\n", Text::from("trailing newline\n"))]
    fn to_text(#[case] value: impl fmt::Display, #[case] expected: Text) {
        assert_eq!(value.to_text(), expected);
    }

    #[test]
    fn from_vec_line() {
        let text = Text::from(vec![
            Line::from("The first line"),
            Line::from("The second line"),
        ]);
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn from_iterator() {
        let text = Text::from_iter(vec!["The first line", "The second line"]);
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn collect() {
        let text: Text = iter::once("The first line")
            .chain(iter::once("The second line"))
            .collect();
        assert_eq!(
            text.lines,
            vec![Line::from("The first line"), Line::from("The second line")]
        );
    }

    #[test]
    fn into_iter() {
        let text = Text::from("The first line\nThe second line");
        let mut iter = text.into_iter();
        assert_eq!(iter.next(), Some(Line::from("The first line")));
        assert_eq!(iter.next(), Some(Line::from("The second line")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn add_line() {
        assert_eq!(
            Text::raw("Red").red() + Line::raw("Blue").blue(),
            Text {
                lines: vec![Line::raw("Red"), Line::raw("Blue").blue()],
                style: Style::new().red(),
                alignment: None,
            }
        );
    }

    #[test]
    fn add_text() {
        assert_eq!(
            Text::raw("Red").red() + Text::raw("Blue").blue(),
            Text {
                lines: vec![Line::raw("Red"), Line::raw("Blue")],
                style: Style::new().red(),
                alignment: None,
            }
        );
    }

    #[test]
    fn add_assign_line() {
        let mut text = Text::raw("Red").red();
        text += Line::raw("Blue").blue();
        assert_eq!(
            text,
            Text {
                lines: vec![Line::raw("Red"), Line::raw("Blue").blue()],
                style: Style::new().red(),
                alignment: None,
            }
        );
    }

    #[test]
    fn extend() {
        let mut text = Text::from("The first line\nThe second line");
        text.extend(vec![
            Line::from("The third line"),
            Line::from("The fourth line"),
        ]);
        assert_eq!(
            text.lines,
            vec![
                Line::from("The first line"),
                Line::from("The second line"),
                Line::from("The third line"),
                Line::from("The fourth line"),
            ]
        );
    }

    #[test]
    fn extend_from_iter() {
        let mut text = Text::from("The first line\nThe second line");
        text.extend(vec![
            Line::from("The third line"),
            Line::from("The fourth line"),
        ]);
        assert_eq!(
            text.lines,
            vec![
                Line::from("The first line"),
                Line::from("The second line"),
                Line::from("The third line"),
                Line::from("The fourth line"),
            ]
        );
    }

    #[test]
    fn extend_from_iter_str() {
        let mut text = Text::from("The first line\nThe second line");
        text.extend(vec!["The third line", "The fourth line"]);
        assert_eq!(
            text.lines,
            vec![
                Line::from("The first line"),
                Line::from("The second line"),
                Line::from("The third line"),
                Line::from("The fourth line"),
            ]
        );
    }

    #[rstest]
    #[case::one_line("The first line")]
    #[case::multiple_lines("The first line\nThe second line")]
    fn display_raw_text(#[case] value: &str) {
        let text = Text::raw(value);
        assert_eq!(format!("{text}"), value);
    }

    #[test]
    fn display_styled_text() {
        let styled_text = Text::styled(
            "The first line\nThe second line",
            Style::new().yellow().italic(),
        );

        assert_eq!(format!("{styled_text}"), "The first line\nThe second line");
    }

    #[test]
    fn display_text_from_vec() {
        let text_from_vec = Text::from(vec![
            Line::from("The first line"),
            Line::from("The second line"),
        ]);

        assert_eq!(
            format!("{text_from_vec}"),
            "The first line\nThe second line"
        );
    }

    #[test]
    fn display_extended_text() {
        let mut text = Text::from("The first line\nThe second line");

        assert_eq!(format!("{text}"), "The first line\nThe second line");

        text.extend(vec![
            Line::from("The third line"),
            Line::from("The fourth line"),
        ]);

        assert_eq!(
            format!("{text}"),
            "The first line\nThe second line\nThe third line\nThe fourth line"
        );
    }

    #[test]
    fn stylize() {
        assert_eq!(Text::default().green().style, Color::Green.into());
        assert_eq!(
            Text::default().on_green().style,
            Style::new().bg(Color::Green)
        );
        assert_eq!(Text::default().italic().style, Modifier::ITALIC.into());
    }

    #[test]
    fn left_aligned() {
        let text = Text::from("Hello, world!").left_aligned();
        assert_eq!(text.alignment, Some(Alignment::Left));
    }

    #[test]
    fn centered() {
        let text = Text::from("Hello, world!").centered();
        assert_eq!(text.alignment, Some(Alignment::Center));
    }

    #[test]
    fn right_aligned() {
        let text = Text::from("Hello, world!").right_aligned();
        assert_eq!(text.alignment, Some(Alignment::Right));
    }

    #[test]
    fn push_line() {
        let mut text = Text::from("A");
        text.push_line(Line::from("B"));
        text.push_line(Span::from("C"));
        text.push_line("D");
        assert_eq!(
            text.lines,
            vec![
                Line::raw("A"),
                Line::raw("B"),
                Line::raw("C"),
                Line::raw("D")
            ]
        );
    }

    #[test]
    fn push_line_empty() {
        let mut text = Text::default();
        text.push_line(Line::from("Hello, world!"));
        assert_eq!(text.lines, [Line::from("Hello, world!")]);
    }

    #[test]
    fn push_span() {
        let mut text = Text::from("A");
        text.push_span(Span::raw("B"));
        text.push_span("C");
        assert_eq!(
            text.lines,
            vec![Line::from(vec![
                Span::raw("A"),
                Span::raw("B"),
                Span::raw("C")
            ])],
        );
    }

    #[test]
    fn push_span_empty() {
        let mut text = Text::default();
        text.push_span(Span::raw("Hello, world!"));
        assert_eq!(text.lines, [Line::from(Span::raw("Hello, world!"))]);
    }

    mod widget {
        use super::*;

        #[test]
        fn render() {
            let text = Text::from("foo");
            let area = Rect::new(0, 0, 5, 1);
            let mut buf = Buffer::empty(area);
            text.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["foo  "]));
        }

        #[rstest]
        fn render_out_of_bounds(mut small_buf: Buffer) {
            let out_of_bounds_area = Rect::new(20, 20, 10, 1);
            Text::from("Hello, world!").render(out_of_bounds_area, &mut small_buf);
            assert_eq!(small_buf, Buffer::empty(small_buf.area));
        }

        #[test]
        fn render_right_aligned() {
            let text = Text::from("foo").alignment(Alignment::Right);
            let area = Rect::new(0, 0, 5, 1);
            let mut buf = Buffer::empty(area);
            text.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["  foo"]));
        }

        #[test]
        fn render_centered_odd() {
            let text = Text::from("foo").alignment(Alignment::Center);
            let area = Rect::new(0, 0, 5, 1);
            let mut buf = Buffer::empty(area);
            text.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([" foo "]));
        }

        #[test]
        fn render_centered_even() {
            let text = Text::from("foo").alignment(Alignment::Center);
            let area = Rect::new(0, 0, 6, 1);
            let mut buf = Buffer::empty(area);
            text.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines([" foo  "]));
        }

        #[test]
        fn render_one_line_right() {
            let text = Text::from(vec![
                "foo".into(),
                Line::from("bar").alignment(Alignment::Center),
            ])
            .alignment(Alignment::Right);
            let area = Rect::new(0, 0, 5, 2);
            let mut buf = Buffer::empty(area);
            text.render(area, &mut buf);
            assert_eq!(buf, Buffer::with_lines(["  foo", " bar "]));
        }

        #[test]
        fn render_only_styles_line_area() {
            let area = Rect::new(0, 0, 5, 1);
            let mut buf = Buffer::empty(area);
            Text::from("foo".on_blue()).render(area, &mut buf);

            let mut expected = Buffer::with_lines(["foo  "]);
            expected.set_style(Rect::new(0, 0, 3, 1), Style::new().bg(Color::Blue));
            assert_eq!(buf, expected);
        }

        #[test]
        fn render_truncates() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 6, 1));
            Text::from("foobar".on_blue()).render(Rect::new(0, 0, 3, 1), &mut buf);

            let mut expected = Buffer::with_lines(["foo   "]);
            expected.set_style(Rect::new(0, 0, 3, 1), Style::new().bg(Color::Blue));
            assert_eq!(buf, expected);
        }
    }

    mod iterators {
        use super::*;

        /// a fixture used in the tests below to avoid repeating the same setup
        #[fixture]
        fn hello_world() -> Text<'static> {
            Text::from(vec![
                Line::styled("Hello ", Color::Blue),
                Line::styled("world!", Color::Green),
            ])
        }

        #[rstest]
        fn iter(hello_world: Text<'_>) {
            let mut iter = hello_world.iter();
            assert_eq!(iter.next(), Some(&Line::styled("Hello ", Color::Blue)));
            assert_eq!(iter.next(), Some(&Line::styled("world!", Color::Green)));
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn iter_mut(mut hello_world: Text<'_>) {
            let mut iter = hello_world.iter_mut();
            assert_eq!(iter.next(), Some(&mut Line::styled("Hello ", Color::Blue)));
            assert_eq!(iter.next(), Some(&mut Line::styled("world!", Color::Green)));
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn into_iter(hello_world: Text<'_>) {
            let mut iter = hello_world.into_iter();
            assert_eq!(iter.next(), Some(Line::styled("Hello ", Color::Blue)));
            assert_eq!(iter.next(), Some(Line::styled("world!", Color::Green)));
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn into_iter_ref(hello_world: Text<'_>) {
            let mut iter = (&hello_world).into_iter();
            assert_eq!(iter.next(), Some(&Line::styled("Hello ", Color::Blue)));
            assert_eq!(iter.next(), Some(&Line::styled("world!", Color::Green)));
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn into_iter_mut_ref() {
            let mut hello_world = Text::from(vec![
                Line::styled("Hello ", Color::Blue),
                Line::styled("world!", Color::Green),
            ]);
            let mut iter = (&mut hello_world).into_iter();
            assert_eq!(iter.next(), Some(&mut Line::styled("Hello ", Color::Blue)));
            assert_eq!(iter.next(), Some(&mut Line::styled("world!", Color::Green)));
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn for_loop_ref(hello_world: Text<'_>) {
            let mut result = String::new();
            for line in &hello_world {
                result.push_str(line.to_string().as_ref());
            }
            assert_eq!(result, "Hello world!");
        }

        #[rstest]
        fn for_loop_mut_ref() {
            let mut hello_world = Text::from(vec![
                Line::styled("Hello ", Color::Blue),
                Line::styled("world!", Color::Green),
            ]);
            let mut result = String::new();
            for line in &mut hello_world {
                result.push_str(line.to_string().as_ref());
            }
            assert_eq!(result, "Hello world!");
        }

        #[rstest]
        fn for_loop_into(hello_world: Text<'_>) {
            let mut result = String::new();
            for line in hello_world {
                result.push_str(line.to_string().as_ref());
            }
            assert_eq!(result, "Hello world!");
        }
    }
}
