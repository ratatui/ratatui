use tui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph, Text},
    Terminal,
};

const SAMPLE_STRING: &str = "The library is based on the principle of immediate rendering with \
     intermediate buffers. This means that at each new frame you should build all widgets that are \
     supposed to be part of the UI. While providing a great flexibility for rich and \
     interactive UI, this may introduce overhead for highly dynamic content.";

#[test]
fn paragraph_render_wrap() {
    let test_case = |alignment, expected| {
        let backend = TestBackend::new(20, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|mut f| {
                let size = f.size();
                let text = [Text::raw(SAMPLE_STRING)];
                let paragraph = Paragraph::new(text.iter())
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(alignment)
                    .wrap(true);
                f.render_widget(paragraph, size);
            })
            .unwrap();
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
fn paragraph_render_double_width() {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let s = "コンピュータ上で文字を扱う場合、典型的には文字による通信を行う場合にその両端点では、";
    terminal
        .draw(|mut f| {
            let size = f.size();
            let text = [Text::raw(s)];
            let paragraph = Paragraph::new(text.iter())
                .block(Block::default().borders(Borders::ALL))
                .wrap(true);
            f.render_widget(paragraph, size);
        })
        .unwrap();

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
fn paragraph_render_mixed_width() {
    let backend = TestBackend::new(10, 7);
    let mut terminal = Terminal::new(backend).unwrap();

    let s = "aコンピュータ上で文字を扱う場合、";
    terminal
        .draw(|mut f| {
            let size = f.size();
            let text = [Text::raw(s)];
            let paragraph = Paragraph::new(text.iter())
                .block(Block::default().borders(Borders::ALL))
                .wrap(true);
            f.render_widget(paragraph, size);
        })
        .unwrap();

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
