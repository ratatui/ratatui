//! Tests for how `Paragraph`, `Block`, `Text`, `Line` and `Span` styles interact.
//!
//! See <https://github.com/ratatui/ratatui/issues/1015>.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Paragraph, Widget};

/// Renders `paragraph` onto a buffer the size of `expected` and asserts they match.
#[track_caller]
fn test_case(paragraph: &Paragraph, expected: &Buffer) {
    let mut buffer = Buffer::empty(expected.area);
    paragraph.render(buffer.area, &mut buffer);
    assert_eq!(buffer, *expected);
}

#[test]
fn prerendered_background_is_kept_under_unstyled_text() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 1));
    buffer.set_style(buffer.area, Style::new().on_blue());
    Paragraph::new("hi").render(buffer.area, &mut buffer);

    let mut expected = Buffer::with_lines(["hi  "]);
    expected.set_style(expected.area, Style::new().on_blue());
    assert_eq!(buffer, expected);
}

#[test]
fn paragraph_style_overrides_prerendered_background_and_fills_the_area() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 1));
    buffer.set_style(buffer.area, Style::new().red().on_green());
    Paragraph::new("hi")
        .style(Style::new().blue())
        .render(buffer.area, &mut buffer);

    // Paragraph fg overrides the prerendered fg on every cell; the prerendered bg
    // is inherited because Paragraph::style does not set one.
    let mut expected = Buffer::with_lines(["hi  "]);
    expected.set_style(expected.area, Style::new().blue().on_green());
    assert_eq!(buffer, expected);
}

#[test]
fn text_style_overrides_paragraph_style_on_text_cells_only() {
    let paragraph = Paragraph::new(Text::from("hi").blue()).style(Style::new().red().on_green());
    let mut expected = Buffer::with_lines(["hi  "]);
    expected.set_style(expected.area, Style::new().red().on_green());
    expected.set_style(Rect::new(0, 0, 2, 1), Style::new().blue());
    test_case(&paragraph, &expected);
}

#[test]
fn line_style_overrides_text_style_but_inherits_it() {
    let paragraph = Paragraph::new(Text::from(Line::from("hi").blue()).red().on_green());
    let mut expected = Buffer::with_lines(["hi  "]);
    expected.set_style(Rect::new(0, 0, 2, 1), Style::new().blue().on_green());
    test_case(&paragraph, &expected);
}

#[test]
fn span_style_overrides_line_style_but_inherits_it() {
    let paragraph = Paragraph::new(Line::from(Span::from("hi").blue()).red().on_green());
    let mut expected = Buffer::with_lines(["hi  "]);
    expected.set_style(Rect::new(0, 0, 2, 1), Style::new().blue().on_green());
    test_case(&paragraph, &expected);
}

#[test]
fn block_style_covers_the_border_but_paragraph_style_covers_the_whole_inside() {
    let paragraph = Paragraph::new("hi")
        .block(Block::bordered().on_green())
        .style(Style::new().on_blue());
    let mut expected = Buffer::with_lines(["┌────┐", "│hi  │", "└────┘"]);
    expected.set_style(expected.area, Style::new().on_green());
    // the whole inner rect, blanks included, is covered by the paragraph style
    expected.set_style(Rect::new(1, 1, 4, 1), Style::new().on_blue());
    test_case(&paragraph, &expected);
}

#[test]
fn all_styles_combine() {
    let text = Text::from(Line::from(Span::from("hi").red()).underlined()).bold();
    let paragraph = Paragraph::new(text)
        .block(Block::bordered().on_green())
        .style(Style::new().blue());

    let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 3));
    buffer.set_style(buffer.area, Style::new().italic());
    paragraph.render(buffer.area, &mut buffer);

    let mut expected = Buffer::with_lines(["┌──┐", "│hi│", "└──┘"]);
    expected.set_style(expected.area, Style::new().italic().blue().on_green());
    let inner = Style::new().red().on_green().italic().bold().underlined();
    expected[(1, 1)].set_style(inner);
    expected[(2, 1)].set_style(inner);
    assert_eq!(buffer, expected);
}
