#![cfg(feature = "unstable-rendered-line-info")]

use ratatui_widgets::block::{Block, Padding};
use ratatui_widgets::borders::Borders;
use ratatui_widgets::paragraph::{Paragraph, Wrap};

#[test]
fn line_count_accounts_for_bordered_block_wrapping_issue_2823() {
    let text = "This is an example";
    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: false })
        .block(Block::bordered());

    // With 2 horizontal border columns, width 20 leaves inner width 18 (1 line + 2 borders = 3)
    assert_eq!(paragraph.line_count(20), 3);
    // Width 19 leaves inner width 17, wrapping 18-char text to 2 lines + 2 borders = 4
    assert_eq!(paragraph.line_count(19), 4);
    // Width 18 leaves inner width 16, wrapping to 2 lines + 2 borders = 4
    assert_eq!(paragraph.line_count(18), 4);

    let paragraph_trimmed = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(Block::bordered());
    assert_eq!(paragraph_trimmed.line_count(20), 3);
    assert_eq!(paragraph_trimmed.line_count(19), 4);
    assert_eq!(paragraph_trimmed.line_count(18), 4);
}

#[test]
fn line_count_accounts_for_single_side_borders_at_threshold() {
    let text = "Hello World"; // 11 characters

    let paragraph_left = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(Block::new().borders(Borders::LEFT));
    // 1 column left border: width 12 -> inner 11 (1 line), width 11 -> inner 10 (2 lines)
    assert_eq!(paragraph_left.line_count(12), 1);
    assert_eq!(paragraph_left.line_count(11), 2);

    let paragraph_right = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(Block::new().borders(Borders::RIGHT));
    // 1 column right border: width 12 -> inner 11 (1 line), width 11 -> inner 10 (2 lines)
    assert_eq!(paragraph_right.line_count(12), 1);
    assert_eq!(paragraph_right.line_count(11), 2);

    let paragraph_vertical = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(Block::new().borders(Borders::TOP | Borders::BOTTOM));
    // 0 horizontal border, 2 vertical borders: width 11 -> inner 11 (1 line + 2 = 3), width 10 ->
    // inner 10 (2 lines + 2 = 4)
    assert_eq!(paragraph_vertical.line_count(11), 3);
    assert_eq!(paragraph_vertical.line_count(10), 4);
}

#[test]
fn line_count_accounts_for_padding_at_threshold() {
    let text = "Hello World"; // 11 characters

    // Horizontal padding: 2 left + 2 right = 4 columns reduced
    let paragraph_pad = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(Block::new().padding(Padding::horizontal(2)));
    assert_eq!(paragraph_pad.line_count(15), 1); // inner width 11 -> 1 line
    assert_eq!(paragraph_pad.line_count(14), 2); // inner width 10 -> 2 lines

    // Asymmetric padding: left 1, right 2 (3 horizontal), top 3, bottom 4 (7 vertical)
    let paragraph_asym = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(Block::new().padding(Padding::new(1, 2, 3, 4)));
    assert_eq!(paragraph_asym.line_count(14), 8); // inner width 11 (1 line) + 7 vertical = 8
    assert_eq!(paragraph_asym.line_count(13), 9); // inner width 10 (2 lines) + 7 vertical = 9
}

#[test]
fn line_count_accounts_for_borders_and_padding_combined() {
    let text = "Hello World"; // 11 characters
    // Bordered (2 horizontal, 2 vertical) + 1 horizontal pad on each side (2 horizontal) = 4
    // horizontal, 2 vertical
    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(Block::bordered().padding(Padding::horizontal(1)));

    assert_eq!(paragraph.line_count(16), 3); // inner width 12 -> 1 line + 2 = 3
    assert_eq!(paragraph.line_count(15), 3); // inner width 11 -> 1 line + 2 = 3
    assert_eq!(paragraph.line_count(14), 4); // inner width 10 -> 2 lines + 2 = 4
}

#[test]
fn line_count_multiline_text_with_block() {
    let text = "Hello World\nSecond line"; // Two lines, each 11 chars
    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(Block::bordered());

    assert_eq!(paragraph.line_count(13), 4); // inner 11: 2 lines + 2 borders = 4
    assert_eq!(paragraph.line_count(12), 6); // inner 10: 2 lines wrapped into 2 each = 4 lines + 2 borders = 6
}

#[test]
fn line_count_edge_cases() {
    let text = "Hello World";
    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(Block::bordered());

    // Zero width
    assert_eq!(paragraph.line_count(0), 0);

    // Inner width zero: width <= 2 with 2 horizontal borders
    assert_eq!(paragraph.line_count(2), 2); // 0 text lines + 2 vertical borders
    assert_eq!(paragraph.line_count(1), 2); // 0 text lines + 2 vertical borders

    // Unwrapped paragraph should ignore horizontal border constraints
    let unwrapped = Paragraph::new(text).block(Block::bordered());
    assert_eq!(unwrapped.line_count(20), 3);
    assert_eq!(unwrapped.line_count(10), 3);
    assert_eq!(unwrapped.line_count(2), 3);
    assert_eq!(unwrapped.line_count(0), 0);
}
