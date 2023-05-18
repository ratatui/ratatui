use crate::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{StyledGrapheme, Text},
    widgets::{
        reflow::{LineComposer, LineTruncator, WordWrapper},
        Block, Widget,
    },
};
use std::iter;
use unicode_width::UnicodeWidthStr;

fn get_line_offset(line_width: u16, text_area_width: u16, alignment: Alignment) -> u16 {
    match alignment {
        Alignment::Center => (text_area_width / 2).saturating_sub(line_width / 2),
        Alignment::Right => text_area_width.saturating_sub(line_width),
        Alignment::Left => 0,
    }
}

/// A widget to display some text.
///
/// # Examples
///
/// ```
/// # use ratatui::text::{Text, Line, Span};
/// # use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
/// # use ratatui::style::{Style, Color, Modifier};
/// # use ratatui::layout::{Alignment};
/// let text = vec![
///     Line::from(vec![
///         Span::raw("First"),
///         Span::styled("line",Style::default().add_modifier(Modifier::ITALIC)),
///         Span::raw("."),
///     ]),
///     Line::from(Span::styled("Second line", Style::default().fg(Color::Red))),
/// ];
/// Paragraph::new(text)
///     .block(Block::default().title("Paragraph").borders(Borders::ALL))
///     .style(Style::default().fg(Color::White).bg(Color::Black))
///     .alignment(Alignment::Center)
///     .wrap(Wrap { trim: true });
/// ```
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, Copy)]
pub struct Wrap {
    /// Should leading whitespace be trimmed
    pub trim: bool,
}

impl<'a> Paragraph<'a> {
    pub fn new<T>(text: T) -> Paragraph<'a>
    where
        T: Into<Text<'a>>,
    {
        Paragraph {
            block: None,
            style: Default::default(),
            wrap: None,
            text: text.into(),
            scroll: (0, 0),
            alignment: Alignment::Left,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Paragraph<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Paragraph<'a> {
        self.style = style;
        self
    }

    pub fn wrap(mut self, wrap: Wrap) -> Paragraph<'a> {
        self.wrap = Some(wrap);
        self
    }

    pub fn scroll(mut self, offset: (u16, u16)) -> Paragraph<'a> {
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
        let mut styled = self.text.lines.iter().flat_map(|line| {
            line.spans
                .iter()
                .flat_map(|span| span.styled_graphemes(style))
                // Required given the way composers work but might be refactored out if we change
                // composers to operate on lines instead of a stream of graphemes.
                .chain(iter::once(StyledGrapheme {
                    symbol: "\n",
                    style: self.style,
                }))
        });

        let mut line_composer: Box<dyn LineComposer> = if let Some(Wrap { trim }) = self.wrap {
            Box::new(WordWrapper::new(&mut styled, text_area.width, trim))
        } else {
            let mut line_composer = Box::new(LineTruncator::new(&mut styled, text_area.width));
            if let Alignment::Left = self.alignment {
                line_composer.set_horizontal_offset(self.scroll.1);
            }
            line_composer
        };
        let mut y = 0;
        while let Some((current_line, current_line_width)) = line_composer.next_line() {
            if y >= self.scroll.0 {
                let mut x = get_line_offset(current_line_width, text_area.width, self.alignment);
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        style::Color,
        text::{Line, Span},
        widgets::Borders,
    };

    #[test]
    fn zero_width_char_at_end_of_line() {
        let line = "foo\0";
        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 1));

        Paragraph::new(line).render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec!["foo"]);
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_empty_paragraph() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));

        Paragraph::new("").render(Rect::new(0, 0, 10, 5), &mut buffer);

        let expected_buffer = Buffer::with_lines(vec!["          "; 5]);
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_single_line_paragraph() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 1));

        Paragraph::new("Hello, world!").render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec!["Hello, world!  "]);
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_multi_line_paragraph() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));

        Paragraph::new("This is a\nmultiline\nparagraph.").render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec![
            "This is a      ",
            "multiline      ",
            "paragraph.     ",
        ]);
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_paragraph_with_block() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));

        Paragraph::new("Hello, world!")
            .block(Block::default().title("Title").borders(Borders::ALL))
            .render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec![
            "┌Title────────┐",
            "│Hello, world!│",
            "└─────────────┘",
        ]);
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_paragraph_without_block() {
        let area = Rect::new(0, 0, 15, 1);
        let mut buffer = Buffer::empty(area);

        Paragraph::new("Hello, world!").render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec!["Hello, world!  "]);
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_paragraph_with_word_wrap() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 4));

        Paragraph::new("This is a long line of text that should wrap.")
            .wrap(Wrap { trim: true })
            .render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec![
            "This is a long ",
            "line of text   ",
            "that should    ",
            "wrap.          ",
        ]);
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_paragraph_with_line_truncation() {
        let paragraph = Paragraph::new("This is a long line of text that should be truncated.");
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        paragraph.render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec!["This is a "]);
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_paragraph_with_left_alignment() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 1));

        Paragraph::new("Hello, world!")
            .alignment(Alignment::Left)
            .render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec!["Hello, world!  "]);
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_paragraph_with_center_alignment() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 1));

        Paragraph::new("Hello, world!")
            .alignment(Alignment::Center)
            .render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec![" Hello, world! "]);
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_paragraph_with_right_alignment() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 1));

        Paragraph::new("Hello, world!")
            .alignment(Alignment::Right)
            .render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec!["  Hello, world!"]);
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_paragraph_with_scroll_offset() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));

        Paragraph::new("This is a\nmultiline\nparagraph.")
            .scroll((1, 0))
            .render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec![
            "multiline      ",
            "paragraph.     ",
            "               ",
        ]);
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_paragraph_with_zero_width_area() {
        let area = Rect::new(0, 0, 0, 3);
        let mut buffer = Buffer::empty(area);

        Paragraph::new("Hello, world!").render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::empty(area);
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_paragraph_with_zero_height_area() {
        let area = Rect::new(0, 0, 10, 0);
        let mut buffer = Buffer::empty(area);

        Paragraph::new("Hello, world!").render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::empty(area);
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_paragraph_with_styled_text() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 13, 1));

        Paragraph::new(Line::from(vec![
            Span::styled("Hello, ", Style::default().fg(Color::Red)),
            Span::styled("world!", Style::default().fg(Color::Blue)),
        ]))
        .render(buffer.area, &mut buffer);

        let mut expected_buffer = Buffer::with_lines(vec!["Hello, world!"]);
        expected_buffer.set_style(Rect::new(0, 0, 7, 1), Style::default().fg(Color::Red));
        expected_buffer.set_style(Rect::new(7, 0, 6, 1), Style::default().fg(Color::Blue));
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_paragraph_with_special_characters() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 1));

        Paragraph::new("Hello, <world>!").render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec!["Hello, <world>!"]);
        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn test_render_paragraph_with_unicode_characters() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 1));

        Paragraph::new("こんにちは, 世界! 😃").render(buffer.area, &mut buffer);

        let expected_buffer = Buffer::with_lines(vec!["こんにちは, 世界! 😃"]);
        assert_eq!(buffer, expected_buffer);
    }
}
