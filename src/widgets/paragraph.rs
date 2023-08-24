use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Style, Styled},
    text::{StyledGrapheme, Text},
    widgets::{
        reflow::{LineComposer, LineTruncator, WordWrapper},
        Block, Widget,
    },
};

fn get_line_offset(line_width: u16, text_area_width: u16, alignment: Alignment) -> u16 {
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
/// # use ratatui::prelude::*;
/// # use ratatui::widgets::*;
/// let text = vec![
///     Line::from(vec![
///         Span::raw("First"),
///         Span::styled("line",Style::new().green().italic()),
///         ".".into(),
///     ]),
///     Line::from("Second line".red()),
///     "Third line".into(),
/// ];
/// Paragraph::new(text)
///     .block(Block::new()
///         .title("Paragraph")
///         .borders(Borders::ALL))
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
/// # use ratatui::widgets::{Paragraph, Wrap};
/// # use ratatui::text::Text;
/// let bullet_points = Text::from(r#"Some indented points:
///     - First thing goes here and is long so that it wraps
///     - Here is another point that is long enough to wrap"#);
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
    /// # use ratatui::prelude::*;
    /// # use ratatui::widgets::Paragraph;
    /// let paragraph = Paragraph::new("Hello, world!");
    /// let paragraph = Paragraph::new(String::from("Hello, world!"));
    /// let paragraph = Paragraph::new(Text::raw("Hello, world!"));
    /// let paragraph = Paragraph::new(
    ///     Text::styled("Hello, world!", Style::default()));
    /// let paragraph = Paragraph::new(
    ///     Line::from(vec!["Hello, ".into(), "world!".red()]));
    /// ```
    pub fn new<T>(text: T) -> Paragraph<'a>
    where
        T: Into<Text<'a>>,
    {
        Paragraph {
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
    /// # use ratatui::prelude::*;
    /// # use ratatui::widgets::{Block, Borders, Paragraph};
    /// let paragraph = Paragraph::new("Hello, world!")
    ///    .block(Block::default()
    ///         .title("Paragraph")
    ///         .borders(Borders::ALL));
    /// ```
    pub fn block(mut self, block: Block<'a>) -> Paragraph<'a> {
        self.block = Some(block);
        self
    }

    /// Sets the style of the entire widget.
    ///
    /// This applies to the entire widget, including the block if one is present. Any style set on
    /// the block or text will be added to this style.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # use ratatui::widgets::Paragraph;
    /// let paragraph = Paragraph::new("Hello, world!")
    ///    .style(Style::new().red().on_white());
    /// ```
    pub fn style(mut self, style: Style) -> Paragraph<'a> {
        self.style = style;
        self
    }

    /// Sets the wrapping configuration for the widget.
    ///
    /// See [`Wrap`] for more information on the different options.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # use ratatui::widgets::{Paragraph, Wrap};
    /// let paragraph = Paragraph::new("Hello, world!")
    ///   .wrap(Wrap { trim: true });
    /// ```
    pub fn wrap(mut self, wrap: Wrap) -> Paragraph<'a> {
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
    pub fn scroll(mut self, offset: (Vertical, Horizontal)) -> Paragraph<'a> {
        self.scroll = offset;
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Paragraph<'a> {
        self.alignment = alignment;
        self
    }
}

impl<'a> Widget for Paragraph<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let text_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if text_area.height < 1 {
            return;
        }

        let style = self.style;
        let styled = self.text.lines.iter().map(|line| {
            (
                line.spans
                    .iter()
                    .flat_map(|span| span.styled_graphemes(style)),
                line.alignment.unwrap_or(self.alignment),
            )
        });

        let mut line_composer: Box<dyn LineComposer> = if let Some(Wrap { trim }) = self.wrap {
            Box::new(WordWrapper::new(styled, text_area.width, trim))
        } else {
            let mut line_composer = Box::new(LineTruncator::new(styled, text_area.width));
            line_composer.set_horizontal_offset(self.scroll.1);
            line_composer
        };
        let mut y = 0;
        while let Some((current_line, current_line_width, current_line_alignment)) =
            line_composer.next_line()
        {
            if y >= self.scroll.0 {
                let mut x =
                    get_line_offset(current_line_width, text_area.width, current_line_alignment);
                for StyledGrapheme { symbol, style } in current_line {
                    let width = symbol.width();
                    if width == 0 {
                        continue;
                    }
                    buf.get_mut(text_area.left() + x, text_area.top() + y - self.scroll.0)
                        .set_symbol(if symbol.is_empty() {
                            // If the symbol is empty, the last char which rendered last time will
                            // leave on the line. It's a quick fix.
                            " "
                        } else {
                            symbol
                        })
                        .set_style(*style);
                    x += width as u16;
                }
            }
            y += 1;
            if y >= text_area.height + self.scroll.0 {
                break;
            }
        }
    }
}

impl<'a> Styled for Paragraph<'a> {
    type Item = Paragraph<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        backend::TestBackend,
        style::{Color, Modifier, Stylize},
        text::{Line, Span},
        widgets::Borders,
        Terminal,
    };

    /// Tests the [`Paragraph`] widget against the expected [`Buffer`] by rendering it onto an equal
    /// area and comparing the rendered and expected content.
    /// This can be used for easy testing of varying configured paragraphs with the same expected
    /// buffer or any other test case really.
    fn test_case(paragraph: &Paragraph, expected: Buffer) {
        let backend = TestBackend::new(expected.area.width, expected.area.height);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let size = f.size();
                f.render_widget(paragraph.clone(), size);
            })
            .unwrap();

        terminal.backend().assert_buffer(&expected);
    }

    #[test]
    fn zero_width_char_at_end_of_line() {
        let line = "foo\0";
        let paragraphs = vec![
            Paragraph::new(line),
            Paragraph::new(line).wrap(Wrap { trim: false }),
            Paragraph::new(line).wrap(Wrap { trim: true }),
        ];

        for paragraph in paragraphs {
            test_case(&paragraph, Buffer::with_lines(vec!["foo"]));
            test_case(&paragraph, Buffer::with_lines(vec!["foo   "]));
            test_case(&paragraph, Buffer::with_lines(vec!["foo   ", "      "]));
            test_case(&paragraph, Buffer::with_lines(vec!["foo", "   "]));
        }
    }

    #[test]
    fn test_render_empty_paragraph() {
        let paragraphs = vec![
            Paragraph::new(""),
            Paragraph::new("").wrap(Wrap { trim: false }),
            Paragraph::new("").wrap(Wrap { trim: true }),
        ];

        for paragraph in paragraphs {
            test_case(&paragraph, Buffer::with_lines(vec![" "]));
            test_case(&paragraph, Buffer::with_lines(vec!["          "]));
            test_case(&paragraph, Buffer::with_lines(vec!["     "; 10]));
            test_case(&paragraph, Buffer::with_lines(vec![" ", " "]));
        }
    }

    #[test]
    fn test_render_single_line_paragraph() {
        let text = "Hello, world!";
        let truncated_paragraph = Paragraph::new(text);
        let wrapped_paragraph = Paragraph::new(text).wrap(Wrap { trim: false });
        let trimmed_paragraph = Paragraph::new(text).wrap(Wrap { trim: true });

        let paragraphs = vec![&truncated_paragraph, &wrapped_paragraph, &trimmed_paragraph];

        for paragraph in paragraphs {
            test_case(paragraph, Buffer::with_lines(vec!["Hello, world!  "]));
            test_case(paragraph, Buffer::with_lines(vec!["Hello, world!"]));
            test_case(
                paragraph,
                Buffer::with_lines(vec!["Hello, world!  ", "               "]),
            );
            test_case(
                paragraph,
                Buffer::with_lines(vec!["Hello, world!", "             "]),
            );
        }
    }

    #[test]
    fn test_render_multi_line_paragraph() {
        let text = "This is a\nmultiline\nparagraph.";

        let paragraphs = vec![
            Paragraph::new(text),
            Paragraph::new(text).wrap(Wrap { trim: false }),
            Paragraph::new(text).wrap(Wrap { trim: true }),
        ];

        for paragraph in paragraphs {
            test_case(
                &paragraph,
                Buffer::with_lines(vec!["This is a ", "multiline ", "paragraph."]),
            );
            test_case(
                &paragraph,
                Buffer::with_lines(vec![
                    "This is a      ",
                    "multiline      ",
                    "paragraph.     ",
                ]),
            );
            test_case(
                &paragraph,
                Buffer::with_lines(vec![
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
        let truncated_paragraph =
            Paragraph::new(text).block(Block::default().title("Title").borders(Borders::ALL));
        let wrapped_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: false });
        let trimmed_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: true });

        let paragraphs = vec![&truncated_paragraph, &wrapped_paragraph, &trimmed_paragraph];

        for paragraph in paragraphs {
            test_case(
                paragraph,
                Buffer::with_lines(vec![
                    "┌Title─────────┐",
                    "│Hello, worlds!│",
                    "└──────────────┘",
                ]),
            );
            test_case(
                paragraph,
                Buffer::with_lines(vec![
                    "┌Title───────────┐",
                    "│Hello, worlds!  │",
                    "└────────────────┘",
                ]),
            );
            test_case(
                paragraph,
                Buffer::with_lines(vec![
                    "┌Title────────────┐",
                    "│Hello, worlds!   │",
                    "│                 │",
                    "└─────────────────┘",
                ]),
            );
        }

        test_case(
            &truncated_paragraph,
            Buffer::with_lines(vec![
                "┌Title───────┐",
                "│Hello, world│",
                "│            │",
                "└────────────┘",
            ]),
        );
        test_case(
            &wrapped_paragraph,
            Buffer::with_lines(vec![
                "┌Title──────┐",
                "│Hello,     │",
                "│worlds!    │",
                "└───────────┘",
            ]),
        );
        test_case(
            &trimmed_paragraph,
            Buffer::with_lines(vec![
                "┌Title──────┐",
                "│Hello,     │",
                "│worlds!    │",
                "└───────────┘",
            ]),
        );
    }

    #[test]
    fn test_render_paragraph_with_word_wrap() {
        let text = "This is a long line of text that should wrap      and contains a superultramegagigalong word.";
        let wrapped_paragraph = Paragraph::new(text).wrap(Wrap { trim: false });
        let trimmed_paragraph = Paragraph::new(text).wrap(Wrap { trim: true });

        test_case(
            &wrapped_paragraph,
            Buffer::with_lines(vec![
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
            Buffer::with_lines(vec![
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
            Buffer::with_lines(vec![
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
            Buffer::with_lines(vec![
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
            Buffer::with_lines(vec!["This is a long line of"]),
        );
        test_case(
            &truncated_paragraph,
            Buffer::with_lines(vec!["This is a long line of te"]),
        );
        test_case(
            &truncated_paragraph,
            Buffer::with_lines(vec!["This is a long line of "]),
        );
        test_case(
            &truncated_paragraph.clone().scroll((0, 2)),
            Buffer::with_lines(vec!["is is a long line of te"]),
        );
    }

    #[test]
    fn test_render_paragraph_with_left_alignment() {
        let text = "Hello, world!";
        let truncated_paragraph = Paragraph::new(text).alignment(Alignment::Left);
        let wrapped_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: false });
        let trimmed_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: true });

        let paragraphs = vec![&truncated_paragraph, &wrapped_paragraph, &trimmed_paragraph];

        for paragraph in paragraphs {
            test_case(paragraph, Buffer::with_lines(vec!["Hello, world!  "]));
            test_case(paragraph, Buffer::with_lines(vec!["Hello, world!"]));
        }

        test_case(&truncated_paragraph, Buffer::with_lines(vec!["Hello, wor"]));
        test_case(
            &wrapped_paragraph,
            Buffer::with_lines(vec!["Hello,    ", "world!    "]),
        );
        test_case(
            &trimmed_paragraph,
            Buffer::with_lines(vec!["Hello,    ", "world!    "]),
        );
    }

    #[test]
    fn test_render_paragraph_with_center_alignment() {
        let text = "Hello, world!";
        let truncated_paragraph = Paragraph::new(text).alignment(Alignment::Center);
        let wrapped_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: false });
        let trimmed_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: true });

        let paragraphs = vec![&truncated_paragraph, &wrapped_paragraph, &trimmed_paragraph];

        for paragraph in paragraphs {
            test_case(paragraph, Buffer::with_lines(vec![" Hello, world! "]));
            test_case(paragraph, Buffer::with_lines(vec!["  Hello, world! "]));
            test_case(paragraph, Buffer::with_lines(vec!["  Hello, world!  "]));
            test_case(paragraph, Buffer::with_lines(vec!["Hello, world!"]));
        }

        test_case(&truncated_paragraph, Buffer::with_lines(vec!["Hello, wor"]));
        test_case(
            &wrapped_paragraph,
            Buffer::with_lines(vec!["  Hello,  ", "  world!  "]),
        );
        test_case(
            &trimmed_paragraph,
            Buffer::with_lines(vec!["  Hello,  ", "  world!  "]),
        );
    }

    #[test]
    fn test_render_paragraph_with_right_alignment() {
        let text = "Hello, world!";
        let truncated_paragraph = Paragraph::new(text).alignment(Alignment::Right);
        let wrapped_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: false });
        let trimmed_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: true });

        let paragraphs = vec![&truncated_paragraph, &wrapped_paragraph, &trimmed_paragraph];

        for paragraph in paragraphs {
            test_case(paragraph, Buffer::with_lines(vec!["  Hello, world!"]));
            test_case(paragraph, Buffer::with_lines(vec!["Hello, world!"]));
        }

        test_case(&truncated_paragraph, Buffer::with_lines(vec!["Hello, wor"]));
        test_case(
            &wrapped_paragraph,
            Buffer::with_lines(vec!["    Hello,", "    world!"]),
        );
        test_case(
            &trimmed_paragraph,
            Buffer::with_lines(vec!["    Hello,", "    world!"]),
        );
    }

    #[test]
    fn test_render_paragraph_with_scroll_offset() {
        let text = "This is a\ncool\nmultiline\nparagraph.";
        let truncated_paragraph = Paragraph::new(text).scroll((2, 0));
        let wrapped_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: false });
        let trimmed_paragraph = truncated_paragraph.clone().wrap(Wrap { trim: true });

        let paragraphs = vec![&truncated_paragraph, &wrapped_paragraph, &trimmed_paragraph];

        for paragraph in paragraphs {
            test_case(
                paragraph,
                Buffer::with_lines(vec!["multiline   ", "paragraph.  ", "            "]),
            );
            test_case(paragraph, Buffer::with_lines(vec!["multiline   "]));
        }

        test_case(
            &truncated_paragraph.clone().scroll((2, 4)),
            Buffer::with_lines(vec!["iline   ", "graph.  "]),
        );
        test_case(
            &wrapped_paragraph,
            Buffer::with_lines(vec!["cool   ", "multili", "ne     "]),
        );
    }

    #[test]
    fn test_render_paragraph_with_zero_width_area() {
        let text = "Hello, world!";

        let paragraphs = vec![
            Paragraph::new(text),
            Paragraph::new(text).wrap(Wrap { trim: false }),
            Paragraph::new(text).wrap(Wrap { trim: true }),
        ];

        let area = Rect::new(0, 0, 0, 3);
        for paragraph in paragraphs {
            test_case(&paragraph, Buffer::empty(area));
            test_case(&paragraph.clone().scroll((2, 4)), Buffer::empty(area));
        }
    }

    #[test]
    fn test_render_paragraph_with_zero_height_area() {
        let text = "Hello, world!";

        let paragraphs = vec![
            Paragraph::new(text),
            Paragraph::new(text).wrap(Wrap { trim: false }),
            Paragraph::new(text).wrap(Wrap { trim: true }),
        ];

        let area = Rect::new(0, 0, 10, 0);
        for paragraph in paragraphs {
            test_case(&paragraph, Buffer::empty(area));
            test_case(&paragraph.clone().scroll((2, 4)), Buffer::empty(area));
        }
    }

    #[test]
    fn test_render_paragraph_with_styled_text() {
        let text = Line::from(vec![
            Span::styled("Hello, ", Style::default().fg(Color::Red)),
            Span::styled("world!", Style::default().fg(Color::Blue)),
        ]);

        let paragraphs = vec![
            Paragraph::new(text.clone()),
            Paragraph::new(text.clone()).wrap(Wrap { trim: false }),
            Paragraph::new(text.clone()).wrap(Wrap { trim: true }),
        ];

        let mut expected_buffer = Buffer::with_lines(vec!["Hello, world!"]);
        expected_buffer.set_style(
            Rect::new(0, 0, 7, 1),
            Style::default().fg(Color::Red).bg(Color::Green),
        );
        expected_buffer.set_style(
            Rect::new(7, 0, 6, 1),
            Style::default().fg(Color::Blue).bg(Color::Green),
        );
        for paragraph in paragraphs {
            test_case(
                &paragraph.style(Style::default().bg(Color::Green)),
                expected_buffer.clone(),
            );
        }
    }

    #[test]
    fn test_render_paragraph_with_special_characters() {
        let text = "Hello, <world>!";
        let paragraphs = vec![
            Paragraph::new(text),
            Paragraph::new(text).wrap(Wrap { trim: false }),
            Paragraph::new(text).wrap(Wrap { trim: true }),
        ];

        for paragraph in paragraphs {
            test_case(&paragraph, Buffer::with_lines(vec!["Hello, <world>!"]));
            test_case(&paragraph, Buffer::with_lines(vec!["Hello, <world>!     "]));
            test_case(
                &paragraph,
                Buffer::with_lines(vec!["Hello, <world>!     ", "                    "]),
            );
            test_case(
                &paragraph,
                Buffer::with_lines(vec!["Hello, <world>!", "               "]),
            );
        }
    }

    #[test]
    fn test_render_paragraph_with_unicode_characters() {
        let text = "こんにちは, 世界! 😃";
        let truncated_paragraph = Paragraph::new(text);
        let wrapped_paragraph = Paragraph::new(text).wrap(Wrap { trim: false });
        let trimmed_paragraph = Paragraph::new(text).wrap(Wrap { trim: true });

        let paragraphs = vec![&truncated_paragraph, &wrapped_paragraph, &trimmed_paragraph];

        for paragraph in paragraphs {
            test_case(paragraph, Buffer::with_lines(vec!["こんにちは, 世界! 😃"]));
            test_case(
                paragraph,
                Buffer::with_lines(vec!["こんにちは, 世界! 😃     "]),
            );
        }

        test_case(
            &truncated_paragraph,
            Buffer::with_lines(vec!["こんにちは, 世 "]),
        );
        test_case(
            &wrapped_paragraph,
            Buffer::with_lines(vec!["こんにちは,    ", "世界! 😃      "]),
        );
        test_case(
            &trimmed_paragraph,
            Buffer::with_lines(vec!["こんにちは,    ", "世界! 😃      "]),
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
        )
    }
}
