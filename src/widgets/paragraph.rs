use unicode_width::UnicodeWidthStr;

use crate::{
    prelude::*,
    style::Styled,
    text::StyledGrapheme,
    widgets::{
        reflow::{LineComposer, LineTruncator, WordWrapper, WrappedLine},
        Block,
    },
};

const fn get_line_offset(line_width: u16, text_area_width: u16, alignment: Alignment) -> u16 {
    match alignment {
        Alignment::Center => (text_area_width / 2).saturating_sub(line_width / 2),
        Alignment::Right => text_area_width.saturating_sub(line_width),
        Alignment::Left => 0,
    }
}

/// A widget to display some text.
///
/// # Example
///
/// ```
/// use ratatui::{prelude::*, widgets::*};
///
/// let text = vec![
///     Line::from(vec![
///         Span::raw("First"),
///         Span::styled("line", Style::new().green().italic()),
///         ".".into(),
///     ]),
///     Line::from("Second line".red()),
///     "Third line".into(),
/// ];
/// Paragraph::new(text)
///     .block(Block::bordered().title("Paragraph"))
///     .style(Style::new().white().on_black())
///     .alignment(Alignment::Center)
///     .wrap(Wrap { trim: true });
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Paragraph<'a> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Widget style
    style: Style,
    /// How to wrap the text
    wrap: Option<Wrap>,
    /// The text to display
    text: Text<'a>,
    /// Scroll
    scroll: (u16, u16),
    /// Alignment of the text
    alignment: Alignment,
}

/// Describes how to wrap text across lines.
///
/// ## Examples
///
/// ```
/// use ratatui::{prelude::*, widgets::*};
///
/// let bullet_points = Text::from(
///     r#"Some indented points:
///     - First thing goes here and is long so that it wraps
///     - Here is another point that is long enough to wrap"#,
/// );
///
/// // With leading spaces trimmed (window width of 30 chars):
/// Paragraph::new(bullet_points.clone()).wrap(Wrap { trim: true });
/// // Some indented points:
/// // - First thing goes here and is
/// // long so that it wraps
/// // - Here is another point that
/// // is long enough to wrap
///
/// // But without trimming, indentation is preserved:
/// Paragraph::new(bullet_points).wrap(Wrap { trim: false });
/// // Some indented points:
/// //     - First thing goes here
/// // and is long so that it wraps
/// //     - Here is another point
/// // that is long enough to wrap
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Wrap {
    /// Should leading whitespace be trimmed
    pub trim: bool,
}

type Horizontal = u16;
type Vertical = u16;

impl<'a> Paragraph<'a> {
    /// Creates a new [`Paragraph`] widget with the given text.
    ///
    /// The `text` parameter can be a [`Text`] or any type that can be converted into a [`Text`]. By
    /// default, the text is styled with [`Style::default()`], not wrapped, and aligned to the left.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let paragraph = Paragraph::new("Hello, world!");
    /// let paragraph = Paragraph::new(String::from("Hello, world!"));
    /// let paragraph = Paragraph::new(Text::raw("Hello, world!"));
    /// let paragraph = Paragraph::new(Text::styled("Hello, world!", Style::default()));
    /// let paragraph = Paragraph::new(Line::from(vec!["Hello, ".into(), "world!".red()]));
    /// ```
    pub fn new<T>(text: T) -> Self
    where
        T: Into<Text<'a>>,
    {
        Self {
            block: None,
            style: Style::default(),
            wrap: None,
            text: text.into(),
            scroll: (0, 0),
            alignment: Alignment::Left,
        }
    }

    /// Surrounds the [`Paragraph`] widget with a [`Block`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let paragraph = Paragraph::new("Hello, world!").block(Block::bordered().title("Paragraph"));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Sets the style of the entire widget.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This applies to the entire widget, including the block if one is present. Any style set on
    /// the block or text will be added to this style.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let paragraph = Paragraph::new("Hello, world!").style(Style::new().red().on_white());
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the wrapping configuration for the widget.
    ///
    /// See [`Wrap`] for more information on the different options.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let paragraph = Paragraph::new("Hello, world!").wrap(Wrap { trim: true });
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = Some(wrap);
        self
    }

    /// Set the scroll offset for the given paragraph
    ///
    /// The scroll offset is a tuple of (y, x) offset. The y offset is the number of lines to
    /// scroll, and the x offset is the number of characters to scroll. The scroll offset is applied
    /// after the text is wrapped and aligned.
    ///
    /// Note: the order of the tuple is (y, x) instead of (x, y), which is different from general
    /// convention across the crate.
    ///
    /// For more information about future scrolling design and concerns, see [RFC: Design of
    /// Scrollable Widgets](https://github.com/ratatui-org/ratatui/issues/174) on GitHub.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn scroll(mut self, offset: (Vertical, Horizontal)) -> Self {
        self.scroll = offset;
        self
    }

    /// Set the text alignment for the given paragraph
    ///
    /// The alignment is a variant of the [`Alignment`] enum which can be one of Left, Right, or
    /// Center. If no alignment is specified, the text in a paragraph will be left-aligned.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let paragraph = Paragraph::new("Hello World").alignment(Alignment::Center);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Left-aligns the text in the given paragraph.
    ///
    /// Convenience shortcut for `Paragraph::alignment(Alignment::Left)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let paragraph = Paragraph::new("Hello World").left_aligned();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn left_aligned(self) -> Self {
        self.alignment(Alignment::Left)
    }

    /// Center-aligns the text in the given paragraph.
    ///
    /// Convenience shortcut for `Paragraph::alignment(Alignment::Center)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let paragraph = Paragraph::new("Hello World").centered();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn centered(self) -> Self {
        self.alignment(Alignment::Center)
    }

    /// Right-aligns the text in the given paragraph.
    ///
    /// Convenience shortcut for `Paragraph::alignment(Alignment::Right)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::{prelude::*, widgets::*};
    /// let paragraph = Paragraph::new("Hello World").right_aligned();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn right_aligned(self) -> Self {
        self.alignment(Alignment::Right)
    }

    /// Calculates the number of lines needed to fully render.
    ///
    /// Given a max line width, this method calculates the number of lines that a paragraph will
    /// need in order to be fully rendered. For paragraphs that do not use wrapping, this count is
    /// simply the number of lines present in the paragraph.
    ///
    /// Note: The design for text wrapping is not stable and might affect this API.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use ratatui::{prelude::*, widgets::*};
    /// let paragraph = Paragraph::new("Hello World")
    ///     .wrap(Wrap { trim: false });
    /// assert_eq!(paragraph.line_count(20), 1);
    /// assert_eq!(paragraph.line_count(10), 2);
    /// ```
    #[instability::unstable(
        feature = "rendered-line-info",
        issue = "https://github.com/ratatui-org/ratatui/issues/293"
    )]
    pub fn line_count(&self, width: u16) -> usize {
        if width < 1 {
            return 0;
        }

        if let Some(Wrap { trim }) = self.wrap {
            let styled = self.text.iter().map(|line| {
                let graphemes = line
                    .spans
                    .iter()
                    .flat_map(|span| span.styled_graphemes(self.style));
                let alignment = line.alignment.unwrap_or(self.alignment);
                (graphemes, alignment)
            });
            let mut line_composer = WordWrapper::new(styled, width, trim);
            let mut count = 0;
            while line_composer.next_line().is_some() {
                count += 1;
            }
            count
        } else {
            self.text.height()
        }
    }

    /// Calculates the shortest line width needed to avoid any word being wrapped or truncated.
    ///
    /// Note: The design for text wrapping is not stable and might affect this API.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use ratatui::{prelude::*, widgets::*};
    /// let paragraph = Paragraph::new("Hello World");
    /// assert_eq!(paragraph.line_width(), 11);
    ///
    /// let paragraph = Paragraph::new("Hello World\nhi\nHello World!!!");
    /// assert_eq!(paragraph.line_width(), 14);
    /// ```
    #[instability::unstable(
        feature = "rendered-line-info",
        issue = "https://github.com/ratatui-org/ratatui/issues/293"
    )]
    pub fn line_width(&self) -> usize {
        self.text.iter().map(Line::width).max().unwrap_or_default()
    }
}

impl Widget for Paragraph<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

impl WidgetRef for Paragraph<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        self.block.render_ref(area, buf);
        let inner = self.block.inner_if_some(area);
        self.render_paragraph(inner, buf);
    }
}

impl Paragraph<'_> {
    fn render_paragraph(&self, text_area: Rect, buf: &mut Buffer) {
        if text_area.is_empty() {
            return;
        }

        buf.set_style(text_area, self.style);
        let styled = self.text.iter().map(|line| {
            let graphemes = line.styled_graphemes(self.text.style);
            let alignment = line.alignment.unwrap_or(self.alignment);
            (graphemes, alignment)
        });

        if let Some(Wrap { trim }) = self.wrap {
            let line_composer = WordWrapper::new(styled, text_area.width, trim);
            self.render_text(line_composer, text_area, buf);
        } else {
            let mut line_composer = LineTruncator::new(styled, text_area.width);
            line_composer.set_horizontal_offset(self.scroll.1);
            self.render_text(line_composer, text_area, buf);
        }
    }
}

impl<'a> Paragraph<'a> {
    fn render_text<C: LineComposer<'a>>(&self, mut composer: C, area: Rect, buf: &mut Buffer) {
        let mut y = 0;
        while let Some(WrappedLine {
            line: current_line,
            width: current_line_width,
            alignment: current_line_alignment,
        }) = composer.next_line()
        {
            if y >= self.scroll.0 {
                let mut x = get_line_offset(current_line_width, area.width, current_line_alignment);
                for StyledGrapheme { symbol, style } in current_line {
                    let width = symbol.width();
                    if width == 0 {
                        continue;
                    }
                    // If the symbol is empty, the last char which rendered last time will
                    // leave on the line. It's a quick fix.
                    let symbol = if symbol.is_empty() { " " } else { symbol };
                    buf.get_mut(area.left() + x, area.top() + y - self.scroll.0)
                        .set_symbol(symbol)
                        .set_style(*style);
                    x += width as u16;
                }
            }
            y += 1;
            if y >= area.height + self.scroll.0 {
                break;
            }
        }
    }
}

impl<'a> Styled for Paragraph<'a> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        backend::TestBackend,
        widgets::{block::Position, Borders},
    };

    /// Tests the [`Paragraph`] widget against the expected [`Buffer`] by rendering it onto an equal
    /// area and comparing the rendered and expected content.
    /// This can be used for easy testing of varying configured paragraphs with the same expected
    /// buffer or any other test case really.
    #[track_caller]
    fn test_case(paragraph: &Paragraph, expected: &Buffer) {
        let backend = TestBackend::new(expected.area.width, expected.area.height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                let size = f.size();
                f.render_widget(paragraph.clone(), size);
            })
            .unwrap();
        terminal.backend().assert_buffer(expected);
    }

    #[test]
    fn zero_width_char_at_end_of_line() {
        let line = "foo\u{200B}";
        for paragraph in [
            Paragraph::new(line),
            Paragraph::new(line).wrap(Wrap { trim: false }),
            Paragraph::new(line).wrap(Wrap { trim: true }),
        ] {
            test_case(&paragraph, &Buffer::with_lines(["foo"]));
            test_case(&paragraph, &Buffer::with_lines(["foo   "]));
            test_case(&paragraph, &Buffer::with_lines(["foo   ", "      "]));
            test_case(&paragraph, &Buffer::with_lines(["foo", "   "]));
        }
    }

    #[test]
    fn test_render_empty_paragraph() {
        for paragraph in [
            Paragraph::new(""),
            Paragraph::new("").wrap(Wrap { trim: false }),
            Paragraph::new("").wrap(Wrap { trim: true }),
        ] {
            test_case(&paragraph, &Buffer::with_lines([" "]));
            test_case(&paragraph, &Buffer::with_lines(["          "]));
            test_case(&paragraph, &Buffer::with_lines(["     "; 10]));
            test_case(&paragraph, &Buffer::with_lines([" ", " "]));
        }
    }

    #[test]
    fn test_render_single_line_paragraph() {
        let text = "Hello, world!";
        for paragraph in [
            Paragraph::new(text),
            Paragraph::new(text).wrap(Wrap { trim: false }),
            Paragraph::new(text).wrap(Wrap { trim: true }),
        ] {
            test_case(&paragraph, &Buffer::with_lines(["Hello, world!  "]));
            test_case(&paragraph, &Buffer::with_lines(["Hello, world!"]));
            test_case(
                &paragraph,
                &Buffer::with_lines(["Hello, world!  ", "               "]),
            );
            test_case(
                &paragraph,
                &Buffer::with_lines(["Hello, world!", "             "]),
            );
        }
    }

    #[test]
    fn test_render_multi_line_paragraph() {
        let text = "This is a\nmultiline\nparagraph.";
        for paragraph in [
            Paragraph::new(text),
            Paragraph::new(text).wrap(Wrap { trim: false }),
            Paragraph::new(text).wrap(Wrap { trim: true }),
        ] {
            test_case(
                &paragraph,
                &Buffer::with_lines(["This is a ", "multiline ", "paragraph."]),
            );
            test_case(
                &paragraph,
                &Buffer::with_lines(["This is a      ", "multiline      ", "paragraph.     "]),
            );
            test_case(
                &paragraph,
                &Buffer::with_lines([
                    "This is a      ",
                    "multiline      ",
                    "paragraph.     ",
                    "               ",
                    "               ",
                ]),
            );
        }
    }

    #[test]
    fn test_render_paragraph_with_block() {
        // We use the slightly unconventional "worlds" instead of "world" here to make sure when we
        // can truncate this without triggering the typos linter.
        let text = "Hello, worlds!";
        let truncated_paragraph = Paragraph::new(text).block(Block::bordered().title("Title"));
        let wrapped_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: false });
        let trimmed_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: true });

        for paragraph in [&truncated_paragraph, &wrapped_paragraph, &trimmed_paragraph] {
            #[rustfmt::skip]
            test_case(
                paragraph,
                &Buffer::with_lines([
                    "┌Title─────────┐",
                    "│Hello, worlds!│",
                    "└──────────────┘",
                ]),
            );
            test_case(
                paragraph,
                &Buffer::with_lines([
                    "┌Title───────────┐",
                    "│Hello, worlds!  │",
                    "└────────────────┘",
                ]),
            );
            test_case(
                paragraph,
                &Buffer::with_lines([
                    "┌Title────────────┐",
                    "│Hello, worlds!   │",
                    "│                 │",
                    "└─────────────────┘",
                ]),
            );
        }

        test_case(
            &truncated_paragraph,
            &Buffer::with_lines([
                "┌Title───────┐",
                "│Hello, world│",
                "│            │",
                "└────────────┘",
            ]),
        );
        test_case(
            &wrapped_paragraph,
            &Buffer::with_lines([
                "┌Title──────┐",
                "│Hello,     │",
                "│worlds!    │",
                "└───────────┘",
            ]),
        );
        test_case(
            &trimmed_paragraph,
            &Buffer::with_lines([
                "┌Title──────┐",
                "│Hello,     │",
                "│worlds!    │",
                "└───────────┘",
            ]),
        );
    }

    #[test]
    fn test_render_line_styled() {
        let l0 = Line::raw("unformatted");
        let l1 = Line::styled("bold text", Style::new().bold());
        let l2 = Line::styled("cyan text", Style::new().cyan());
        let l3 = Line::styled("dim text", Style::new().dim());
        let paragraph = Paragraph::new(vec![l0, l1, l2, l3]);

        let mut expected =
            Buffer::with_lines(["unformatted", "bold text", "cyan text", "dim text"]);
        expected.set_style(Rect::new(0, 1, 9, 1), Style::new().bold());
        expected.set_style(Rect::new(0, 2, 9, 1), Style::new().cyan());
        expected.set_style(Rect::new(0, 3, 8, 1), Style::new().dim());

        test_case(&paragraph, &expected);
    }

    #[test]
    fn test_render_line_spans_styled() {
        let l0 = Line::default().spans([
            Span::styled("bold", Style::new().bold()),
            Span::raw(" and "),
            Span::styled("cyan", Style::new().cyan()),
        ]);
        let l1 = Line::default().spans([Span::raw("unformatted")]);
        let paragraph = Paragraph::new(vec![l0, l1]);

        let mut expected = Buffer::with_lines(["bold and cyan", "unformatted"]);
        expected.set_style(Rect::new(0, 0, 4, 1), Style::new().bold());
        expected.set_style(Rect::new(9, 0, 4, 1), Style::new().cyan());

        test_case(&paragraph, &expected);
    }

    #[test]
    fn test_render_paragraph_with_block_with_bottom_title_and_border() {
        let block = Block::new()
            .borders(Borders::BOTTOM)
            .title_position(Position::Bottom)
            .title("Title");
        let paragraph = Paragraph::new("Hello, world!").block(block);
        test_case(
            &paragraph,
            &Buffer::with_lines(["Hello, world!  ", "Title──────────"]),
        );
    }

    #[test]
    fn test_render_paragraph_with_word_wrap() {
        let text = "This is a long line of text that should wrap      and contains a superultramegagigalong word.";
        let wrapped_paragraph = Paragraph::new(text).wrap(Wrap { trim: false });
        let trimmed_paragraph = Paragraph::new(text).wrap(Wrap { trim: true });

        test_case(
            &wrapped_paragraph,
            &Buffer::with_lines([
                "This is a long line",
                "of text that should",
                "wrap      and      ",
                "contains a         ",
                "superultramegagigal",
                "ong word.          ",
            ]),
        );
        test_case(
            &wrapped_paragraph,
            &Buffer::with_lines([
                "This is a   ",
                "long line of",
                "text that   ",
                "should wrap ",
                "    and     ",
                "contains a  ",
                "superultrame",
                "gagigalong  ",
                "word.       ",
            ]),
        );

        test_case(
            &trimmed_paragraph,
            &Buffer::with_lines([
                "This is a long line",
                "of text that should",
                "wrap      and      ",
                "contains a         ",
                "superultramegagigal",
                "ong word.          ",
            ]),
        );
        test_case(
            &trimmed_paragraph,
            &Buffer::with_lines([
                "This is a   ",
                "long line of",
                "text that   ",
                "should wrap ",
                "and contains",
                "a           ",
                "superultrame",
                "gagigalong  ",
                "word.       ",
            ]),
        );
    }

    #[test]
    fn test_render_paragraph_with_line_truncation() {
        let text = "This is a long line of text that should be truncated.";
        let truncated_paragraph = Paragraph::new(text);

        test_case(
            &truncated_paragraph,
            &Buffer::with_lines(["This is a long line of"]),
        );
        test_case(
            &truncated_paragraph,
            &Buffer::with_lines(["This is a long line of te"]),
        );
        test_case(
            &truncated_paragraph,
            &Buffer::with_lines(["This is a long line of "]),
        );
        test_case(
            &truncated_paragraph.clone().scroll((0, 2)),
            &Buffer::with_lines(["is is a long line of te"]),
        );
    }

    #[test]
    fn test_render_paragraph_with_left_alignment() {
        let text = "Hello, world!";
        let truncated_paragraph = Paragraph::new(text).alignment(Alignment::Left);
        let wrapped_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: false });
        let trimmed_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: true });

        for paragraph in [&truncated_paragraph, &wrapped_paragraph, &trimmed_paragraph] {
            test_case(paragraph, &Buffer::with_lines(["Hello, world!  "]));
            test_case(paragraph, &Buffer::with_lines(["Hello, world!"]));
        }

        test_case(&truncated_paragraph, &Buffer::with_lines(["Hello, wor"]));
        test_case(
            &wrapped_paragraph,
            &Buffer::with_lines(["Hello,    ", "world!    "]),
        );
        test_case(
            &trimmed_paragraph,
            &Buffer::with_lines(["Hello,    ", "world!    "]),
        );
    }

    #[test]
    fn test_render_paragraph_with_center_alignment() {
        let text = "Hello, world!";
        let truncated_paragraph = Paragraph::new(text).alignment(Alignment::Center);
        let wrapped_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: false });
        let trimmed_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: true });

        for paragraph in [&truncated_paragraph, &wrapped_paragraph, &trimmed_paragraph] {
            test_case(paragraph, &Buffer::with_lines([" Hello, world! "]));
            test_case(paragraph, &Buffer::with_lines(["  Hello, world! "]));
            test_case(paragraph, &Buffer::with_lines(["  Hello, world!  "]));
            test_case(paragraph, &Buffer::with_lines(["Hello, world!"]));
        }

        test_case(&truncated_paragraph, &Buffer::with_lines(["Hello, wor"]));
        test_case(
            &wrapped_paragraph,
            &Buffer::with_lines(["  Hello,  ", "  world!  "]),
        );
        test_case(
            &trimmed_paragraph,
            &Buffer::with_lines(["  Hello,  ", "  world!  "]),
        );
    }

    #[test]
    fn test_render_paragraph_with_right_alignment() {
        let text = "Hello, world!";
        let truncated_paragraph = Paragraph::new(text).alignment(Alignment::Right);
        let wrapped_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: false });
        let trimmed_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: true });

        for paragraph in [&truncated_paragraph, &wrapped_paragraph, &trimmed_paragraph] {
            test_case(paragraph, &Buffer::with_lines(["  Hello, world!"]));
            test_case(paragraph, &Buffer::with_lines(["Hello, world!"]));
        }

        test_case(&truncated_paragraph, &Buffer::with_lines(["Hello, wor"]));
        test_case(
            &wrapped_paragraph,
            &Buffer::with_lines(["    Hello,", "    world!"]),
        );
        test_case(
            &trimmed_paragraph,
            &Buffer::with_lines(["    Hello,", "    world!"]),
        );
    }

    #[test]
    fn test_render_paragraph_with_scroll_offset() {
        let text = "This is a\ncool\nmultiline\nparagraph.";
        let truncated_paragraph = Paragraph::new(text).scroll((2, 0));
        let wrapped_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: false });
        let trimmed_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: true });

        for paragraph in [&truncated_paragraph, &wrapped_paragraph, &trimmed_paragraph] {
            test_case(
                paragraph,
                &Buffer::with_lines(["multiline   ", "paragraph.  ", "            "]),
            );
            test_case(paragraph, &Buffer::with_lines(["multiline   "]));
        }

        test_case(
            &truncated_paragraph.clone().scroll((2, 4)),
            &Buffer::with_lines(["iline   ", "graph.  "]),
        );
        test_case(
            &wrapped_paragraph,
            &Buffer::with_lines(["cool   ", "multili", "ne     "]),
        );
    }

    #[test]
    fn test_render_paragraph_with_zero_width_area() {
        let text = "Hello, world!";
        let area = Rect::new(0, 0, 0, 3);

        for paragraph in [
            Paragraph::new(text),
            Paragraph::new(text).wrap(Wrap { trim: false }),
            Paragraph::new(text).wrap(Wrap { trim: true }),
        ] {
            test_case(&paragraph, &Buffer::empty(area));
            test_case(&paragraph.clone().scroll((2, 4)), &Buffer::empty(area));
        }
    }

    #[test]
    fn test_render_paragraph_with_zero_height_area() {
        let text = "Hello, world!";
        let area = Rect::new(0, 0, 10, 0);

        for paragraph in [
            Paragraph::new(text),
            Paragraph::new(text).wrap(Wrap { trim: false }),
            Paragraph::new(text).wrap(Wrap { trim: true }),
        ] {
            test_case(&paragraph, &Buffer::empty(area));
            test_case(&paragraph.clone().scroll((2, 4)), &Buffer::empty(area));
        }
    }

    #[test]
    fn test_render_paragraph_with_styled_text() {
        let text = Line::from(vec![
            Span::styled("Hello, ", Style::default().fg(Color::Red)),
            Span::styled("world!", Style::default().fg(Color::Blue)),
        ]);

        let mut expected_buffer = Buffer::with_lines(["Hello, world!"]);
        expected_buffer.set_style(
            Rect::new(0, 0, 7, 1),
            Style::default().fg(Color::Red).bg(Color::Green),
        );
        expected_buffer.set_style(
            Rect::new(7, 0, 6, 1),
            Style::default().fg(Color::Blue).bg(Color::Green),
        );

        for paragraph in [
            Paragraph::new(text.clone()),
            Paragraph::new(text.clone()).wrap(Wrap { trim: false }),
            Paragraph::new(text.clone()).wrap(Wrap { trim: true }),
        ] {
            test_case(
                &paragraph.style(Style::default().bg(Color::Green)),
                &expected_buffer,
            );
        }
    }

    #[test]
    fn test_render_paragraph_with_special_characters() {
        let text = "Hello, <world>!";
        for paragraph in [
            Paragraph::new(text),
            Paragraph::new(text).wrap(Wrap { trim: false }),
            Paragraph::new(text).wrap(Wrap { trim: true }),
        ] {
            test_case(&paragraph, &Buffer::with_lines(["Hello, <world>!"]));
            test_case(&paragraph, &Buffer::with_lines(["Hello, <world>!     "]));
            test_case(
                &paragraph,
                &Buffer::with_lines(["Hello, <world>!     ", "                    "]),
            );
            test_case(
                &paragraph,
                &Buffer::with_lines(["Hello, <world>!", "               "]),
            );
        }
    }

    #[test]
    fn test_render_paragraph_with_unicode_characters() {
        let text = "こんにちは, 世界! 😃";
        let truncated_paragraph = Paragraph::new(text);
        let wrapped_paragraph = Paragraph::new(text).wrap(Wrap { trim: false });
        let trimmed_paragraph = Paragraph::new(text).wrap(Wrap { trim: true });

        for paragraph in [&truncated_paragraph, &wrapped_paragraph, &trimmed_paragraph] {
            test_case(paragraph, &Buffer::with_lines(["こんにちは, 世界! 😃"]));
            test_case(
                paragraph,
                &Buffer::with_lines(["こんにちは, 世界! 😃     "]),
            );
        }

        test_case(
            &truncated_paragraph,
            &Buffer::with_lines(["こんにちは, 世 "]),
        );
        test_case(
            &wrapped_paragraph,
            &Buffer::with_lines(["こんにちは,    ", "世界! 😃      "]),
        );
        test_case(
            &trimmed_paragraph,
            &Buffer::with_lines(["こんにちは,    ", "世界! 😃      "]),
        );
    }

    #[test]
    fn can_be_stylized() {
        assert_eq!(
            Paragraph::new("").black().on_white().bold().not_dim().style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::DIM)
        );
    }

    #[test]
    fn widgets_paragraph_count_rendered_lines() {
        let paragraph = Paragraph::new("Hello World");
        assert_eq!(paragraph.line_count(20), 1);
        assert_eq!(paragraph.line_count(10), 1);
        let paragraph = Paragraph::new("Hello World").wrap(Wrap { trim: false });
        assert_eq!(paragraph.line_count(20), 1);
        assert_eq!(paragraph.line_count(10), 2);
        let paragraph = Paragraph::new("Hello World").wrap(Wrap { trim: true });
        assert_eq!(paragraph.line_count(20), 1);
        assert_eq!(paragraph.line_count(10), 2);

        let text = "Hello World ".repeat(100);
        let paragraph = Paragraph::new(text.trim());
        assert_eq!(paragraph.line_count(11), 1);
        assert_eq!(paragraph.line_count(6), 1);
        let paragraph = paragraph.wrap(Wrap { trim: false });
        assert_eq!(paragraph.line_count(11), 100);
        assert_eq!(paragraph.line_count(6), 200);
        let paragraph = paragraph.wrap(Wrap { trim: true });
        assert_eq!(paragraph.line_count(11), 100);
        assert_eq!(paragraph.line_count(6), 200);
    }

    #[test]
    fn widgets_paragraph_line_width() {
        let paragraph = Paragraph::new("Hello World");
        assert_eq!(paragraph.line_width(), 11);
        let paragraph = Paragraph::new("Hello World").wrap(Wrap { trim: false });
        assert_eq!(paragraph.line_width(), 11);
        let paragraph = Paragraph::new("Hello World").wrap(Wrap { trim: true });
        assert_eq!(paragraph.line_width(), 11);

        let text = "Hello World ".repeat(100);
        let paragraph = Paragraph::new(text);
        assert_eq!(paragraph.line_width(), 1200);
        let paragraph = paragraph.wrap(Wrap { trim: false });
        assert_eq!(paragraph.line_width(), 1200);
        let paragraph = paragraph.wrap(Wrap { trim: true });
        assert_eq!(paragraph.line_width(), 1200);
    }

    #[test]
    fn left_aligned() {
        let p = Paragraph::new("Hello, world!").left_aligned();
        assert_eq!(p.alignment, Alignment::Left);
    }

    #[test]
    fn centered() {
        let p = Paragraph::new("Hello, world!").centered();
        assert_eq!(p.alignment, Alignment::Center);
    }

    #[test]
    fn right_aligned() {
        let p = Paragraph::new("Hello, world!").right_aligned();
        assert_eq!(p.alignment, Alignment::Right);
    }

    /// Regression test for <https://github.com/ratatui-org/ratatui/issues/990>
    ///
    /// This test ensures that paragraphs with a block and styled text are rendered correctly.
    /// It has been simplified from the original issue but tests the same functionality.
    #[test]
    fn paragraph_block_text_style() {
        let text = Text::styled("Styled text", Color::Green);
        let paragraph = Paragraph::new(text).block(Block::bordered());

        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 3));
        paragraph.render(Rect::new(0, 0, 20, 3), &mut buf);

        let mut expected = Buffer::with_lines([
            "┌──────────────────┐",
            "│Styled text       │",
            "└──────────────────┘",
        ]);
        expected.set_style(Rect::new(1, 1, 11, 1), Style::default().fg(Color::Green));
        assert_eq!(buf, expected);
    }
}
