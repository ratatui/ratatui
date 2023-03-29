use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
    Terminal,
};

const SAMPLE_STRING: &str = "The library is based on the principle of immediate rendering with \
     intermediate buffers. This means that at each new frame you should build all widgets that are \
     supposed to be part of the UI. While providing a great flexibility for rich and \
     interactive UI, this may introduce overhead for highly dynamic content.";

#[test]
fn widgets_paragraph_can_wrap_its_content() {
    let test_case = |alignment, expected| {
        let backend = TestBackend::new(20, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        let text = vec![Spans::from(SAMPLE_STRING)];
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(alignment)
            .wrap(Wrap { trim: true });
        terminal.render_widget_on_viewport(paragraph, 0);
        terminal.flush().unwrap();
        terminal.backend().assert_buffer(&expected);
    };

    test_case(
        Alignment::Left,
        Buffer::with_lines(vec![
            "┌──────────────────┐",
            "│The library is    │",
            "│based on the      │",
            "│principle of      │",
            "│immediate         │",
            "│rendering with    │",
            "│intermediate      │",
            "│buffers. This     │",
            "│means that at each│",
            "└──────────────────┘",
        ]),
    );
    test_case(
        Alignment::Right,
        Buffer::with_lines(vec![
            "┌──────────────────┐",
            "│    The library is│",
            "│      based on the│",
            "│      principle of│",
            "│         immediate│",
            "│    rendering with│",
            "│      intermediate│",
            "│     buffers. This│",
            "│means that at each│",
            "└──────────────────┘",
        ]),
    );
    test_case(
        Alignment::Center,
        Buffer::with_lines(vec![
            "┌──────────────────┐",
            "│  The library is  │",
            "│   based on the   │",
            "│   principle of   │",
            "│     immediate    │",
            "│  rendering with  │",
            "│   intermediate   │",
            "│   buffers. This  │",
            "│means that at each│",
            "└──────────────────┘",
        ]),
    );
}

#[test]
fn widgets_paragraph_renders_double_width_graphemes() {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let s = "コンピュータ上で文字を扱う場合、典型的には文字による通信を行う場合にその両端点では、";

    let text = vec![Spans::from(s)];
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    terminal.render_widget_on_viewport(paragraph, 0);
    terminal.flush().unwrap();

    let expected = Buffer::with_lines(vec![
        "┌────────┐",
        "│コンピュ│",
        "│ータ上で│",
        "│文字を扱│",
        "│う場合、│",
        "│典型的に│",
        "│は文字に│",
        "│よる通信│",
        "│を行う場│",
        "└────────┘",
    ]);
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_paragraph_renders_mixed_width_graphemes() {
    let backend = TestBackend::new(10, 7);
    let mut terminal = Terminal::new(backend).unwrap();

    let s = "aコンピュータ上で文字を扱う場合、";

    let text = vec![Spans::from(s)];
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    terminal.render_widget_on_viewport(paragraph, 0);
    terminal.flush().unwrap();

    let expected = Buffer::with_lines(vec![
        // The internal width is 8 so only 4 slots for double-width characters.
        "┌────────┐",
        "│aコンピ │", // Here we have 1 latin character so only 3 double-width ones can fit.
        "│ュータ上│",
        "│で文字を│",
        "│扱う場合│",
        "│、      │",
        "└────────┘",
    ]);
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_paragraph_can_wrap_with_a_trailing_nbsp() {
    let nbsp: &str = "\u{00a0}";
    let line = Spans::from(vec![Span::raw("NBSP"), Span::raw(nbsp)]);
    let backend = TestBackend::new(20, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    let expected = Buffer::with_lines(vec![
        "┌──────────────────┐",
        "│NBSP\u{00a0}             │",
        "└──────────────────┘",
    ]);

    let paragraph = Paragraph::new(line).block(Block::default().borders(Borders::ALL));
    terminal.render_widget_on_viewport(paragraph, 0);
    terminal.flush().unwrap();
    terminal.backend().assert_buffer(&expected);
}
#[test]
fn widgets_paragraph_can_scroll_horizontally() {
    let test_case = |alignment, scroll, expected| {
        let backend = TestBackend::new(20, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        let text =
            Text::from("段落现在可以水平滚动了！\nParagraph can scroll horizontally!\nShort line");
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(alignment)
            .scroll(scroll);
        terminal.render_widget_on_viewport(paragraph, 0);
        terminal.flush().unwrap();
        terminal.backend().assert_buffer(&expected);
    };

    test_case(
        Alignment::Left,
        (0, 7),
        Buffer::with_lines(vec![
            "┌──────────────────┐",
            "│在可以水平滚动了！│",
            "│ph can scroll hori│",
            "│ine               │",
            "│                  │",
            "│                  │",
            "│                  │",
            "│                  │",
            "│                  │",
            "└──────────────────┘",
        ]),
    );
    // only support Alignment::Left
    test_case(
        Alignment::Right,
        (0, 7),
        Buffer::with_lines(vec![
            "┌──────────────────┐",
            "│段落现在可以水平滚│",
            "│Paragraph can scro│",
            "│        Short line│",
            "│                  │",
            "│                  │",
            "│                  │",
            "│                  │",
            "│                  │",
            "└──────────────────┘",
        ]),
    );
}

#[test]
fn zero_width_char_at_end_of_line() {
    let line = "foo\0";
    let paragraph = Paragraph::new(line);
    let mut buf = Buffer::with_lines(vec![line]);
    paragraph.render(
        &Rect::new(0, 0, buf.get_width(), buf.get_height()),
        &mut buf,
    );
}
