use either::Either;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::buffer::Buffer;
use crate::layout::{Alignment, Rect};
use crate::style::Style;
use crate::widgets::reflow::{LineComposer, LineTruncator, Styled, WordWrapper};
use crate::widgets::{Block, Text, Widget};

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
/// # use tui::widgets::{Block, Borders, Paragraph, Text, Wrap};
/// # use tui::style::{Style, Color};
/// # use tui::layout::{Alignment};
/// let text = [
///     Text::raw("First line\n"),
///     Text::styled("Second line\n", Style::default().fg(Color::Red))
/// ];
/// Paragraph::new(text.iter())
///     .block(Block::default().title("Paragraph").borders(Borders::ALL))
///     .style(Style::default().fg(Color::White).bg(Color::Black))
///     .alignment(Alignment::Center)
///     .wrap(Wrap { trim: true });
/// ```
#[derive(Debug, Clone)]
pub struct Paragraph<'a, 't, T>
where
    T: Iterator<Item = &'t Text<'t>>,
{
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Widget style
    style: Style,
    /// How to wrap the text
    wrap: Option<Wrap>,
    /// The text to display
    text: T,
    /// Should we parse the text for embedded commands
    raw: bool,
    /// Scroll
    scroll: (u16, u16),
    /// Alignment of the text
    alignment: Alignment,
}

/// Describes how to wrap text across lines.
///
/// # Example
///
/// ```
/// # use tui::widgets::{Paragraph, Text, Wrap};
/// let bullet_points = [Text::raw(r#"Some indented points:
///     - First thing goes here and is long so that it wraps
///     - Here is another point that is long enough to wrap"#)];
///
/// // With leading spaces trimmed (window width of 30 chars):
/// Paragraph::new(bullet_points.iter()).wrap(Wrap { trim: true });
/// // Some indented points:
/// // - First thing goes here and is
/// // long so that it wraps
/// // - Here is another point that
/// // is long enough to wrap
///
/// // But without trimming, indentation is preserved:
/// Paragraph::new(bullet_points.iter()).wrap(Wrap { trim: false });
/// // Some indented points:
/// //     - First thing goes here
/// // and is long so that it wraps
/// //     - Here is another point
/// // that is long enough to wrap
#[derive(Debug, Clone, Copy)]
pub struct Wrap {
    /// Should leading whitespace be trimmed
    pub trim: bool,
}

impl<'a, 't, T> Paragraph<'a, 't, T>
where
    T: Iterator<Item = &'t Text<'t>>,
{
    pub fn new(text: T) -> Paragraph<'a, 't, T> {
        Paragraph {
            block: None,
            style: Default::default(),
            wrap: None,
            raw: false,
            text,
            scroll: (0, 0),
            alignment: Alignment::Left,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Paragraph<'a, 't, T> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Paragraph<'a, 't, T> {
        self.style = style;
        self
    }

    pub fn wrap(mut self, wrap: Wrap) -> Paragraph<'a, 't, T> {
        self.wrap = Some(wrap);
        self
    }

    pub fn raw(mut self, flag: bool) -> Paragraph<'a, 't, T> {
        self.raw = flag;
        self
    }

    pub fn scroll(mut self, offset: (u16, u16)) -> Paragraph<'a, 't, T> {
        self.scroll = offset;
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Paragraph<'a, 't, T> {
        self.alignment = alignment;
        self
    }
}

impl<'a, 't, 'b, T> Widget for Paragraph<'a, 't, T>
where
    T: Iterator<Item = &'t Text<'t>>,
{
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let text_area = match self.block {
            Some(ref mut b) => {
                b.render(area, buf);
                b.inner(area)
            }
            None => area,
        };

        if text_area.height < 1 {
            return;
        }

        buf.set_background(text_area, self.style.bg);

        let style = self.style;
        let mut styled = self.text.by_ref().flat_map(|t| match *t {
            Text::Raw(ref d) => {
                let data: &'t str = d; // coerce to &str
                Either::Left(UnicodeSegmentation::graphemes(data, true).map(|g| Styled(g, style)))
            }
            Text::Styled(ref d, s) => {
                let data: &'t str = d; // coerce to &str
                Either::Right(UnicodeSegmentation::graphemes(data, true).map(move |g| Styled(g, s)))
            }
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
                for Styled(symbol, style) in current_line {
                    buf.get_mut(text_area.left() + x, text_area.top() + y - self.scroll.0)
                        .set_symbol(if symbol.is_empty() {
                            // If the symbol is empty, the last char which rendered last time will
                            // leave on the line. It's a quick fix.
                            " "
                        } else {
                            symbol
                        })
                        .set_style(*style);
                    x += symbol.width() as u16;
                }
            }
            y += 1;
            if y >= text_area.height + self.scroll.0 {
                break;
            }
        }
    }
}
