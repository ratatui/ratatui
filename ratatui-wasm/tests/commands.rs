use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_wasm::wit::{Alignment, Color, Line, RenderCommand, Span, Style};
use ratatui_wasm::blit_commands;

#[test]
fn blit_left_aligned_line() {
    let mut buf = Buffer::empty(Rect::new(0, 0, 20, 3));
    let line = Line {
        y: 1,
        spans: vec![
            Span {
                content: "hi".to_string(),
                style: Some(Style {
                    fg: Some(Color::Green),
                    bg: None,
                    bold: false,
                    italic: false,
                    underline: false,
                }),
            },
            Span {
                content: " there".to_string(),
                style: None,
            },
        ],
        alignment: Some(Alignment::Left),
    };

    blit_commands(Rect::new(0, 0, 20, 3), &[RenderCommand::Line(line)], &mut buf);

    let text: String = buf.content()[20..40]
        .iter()
        .map(ratatui_core::buffer::Cell::symbol)
        .collect();
    assert_eq!(text.trim_end(), "hi there");
}

#[test]
fn blit_centered_line() {
    let mut buf = Buffer::empty(Rect::new(0, 0, 20, 3));
    let line = Line {
        y: 0,
        spans: vec![Span {
            content: "center".to_string(),
            style: None,
        }],
        alignment: Some(Alignment::Center),
    };

    blit_commands(Rect::new(0, 0, 20, 3), &[RenderCommand::Line(line)], &mut buf);

    let text: String = buf.content()[0..20]
        .iter()
        .map(ratatui_core::buffer::Cell::symbol)
        .collect();
    assert!(text.contains("center"));
    assert!(text.starts_with("  "));
    assert!(text.trim_end().len() < 20);
}
