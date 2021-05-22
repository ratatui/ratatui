use tui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders},
    Terminal,
};

#[test]
fn widgets_block_renders() {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let block = Block::default()
                .title(Span::styled("Title", Style::default().fg(Color::LightBlue)))
                .borders(Borders::ALL);
            f.render_widget(
                block,
                Rect {
                    x: 0,
                    y: 0,
                    width: 8,
                    height: 8,
                },
            );
        })
        .unwrap();
    let mut expected = Buffer::with_lines(vec![
        "┌Title─┐  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "└──────┘  ",
        "          ",
        "          ",
    ]);
    for x in 1..=5 {
        expected.get_mut(x, 0).set_fg(Color::LightBlue);
    }
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_block_renders_on_small_areas() {
    let test_case = |block, area: Rect, expected| {
        let backend = TestBackend::new(area.width, area.height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                f.render_widget(block, area);
            })
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    };

    let one_cell_test_cases = [
        (Borders::NONE, "T"),
        (Borders::LEFT, "│"),
        (Borders::TOP, "T"),
        (Borders::RIGHT, "│"),
        (Borders::BOTTOM, "T"),
        (Borders::ALL, "┌"),
    ];
    for (borders, symbol) in one_cell_test_cases.iter().cloned() {
        test_case(
            Block::default().title("Test").borders(borders),
            Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            Buffer::empty(Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            }),
        );
        test_case(
            Block::default().title("Test").borders(borders),
            Rect {
                x: 0,
                y: 0,
                width: 1,
                height: 0,
            },
            Buffer::empty(Rect {
                x: 0,
                y: 0,
                width: 1,
                height: 0,
            }),
        );
        test_case(
            Block::default().title("Test").borders(borders),
            Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 1,
            },
            Buffer::empty(Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 1,
            }),
        );
        test_case(
            Block::default().title("Test").borders(borders),
            Rect {
                x: 0,
                y: 0,
                width: 1,
                height: 1,
            },
            Buffer::with_lines(vec![symbol]),
        );
    }
    test_case(
        Block::default().title("Test").borders(Borders::LEFT),
        Rect {
            x: 0,
            y: 0,
            width: 4,
            height: 1,
        },
        Buffer::with_lines(vec!["│Tes"]),
    );
    test_case(
        Block::default().title("Test").borders(Borders::RIGHT),
        Rect {
            x: 0,
            y: 0,
            width: 4,
            height: 1,
        },
        Buffer::with_lines(vec!["Tes│"]),
    );
    test_case(
        Block::default().title("Test").borders(Borders::RIGHT),
        Rect {
            x: 0,
            y: 0,
            width: 4,
            height: 1,
        },
        Buffer::with_lines(vec!["Tes│"]),
    );
    test_case(
        Block::default()
            .title("Test")
            .borders(Borders::LEFT | Borders::RIGHT),
        Rect {
            x: 0,
            y: 0,
            width: 4,
            height: 1,
        },
        Buffer::with_lines(vec!["│Te│"]),
    );
    test_case(
        Block::default().title("Test").borders(Borders::TOP),
        Rect {
            x: 0,
            y: 0,
            width: 4,
            height: 1,
        },
        Buffer::with_lines(vec!["Test"]),
    );
    test_case(
        Block::default().title("Test").borders(Borders::TOP),
        Rect {
            x: 0,
            y: 0,
            width: 5,
            height: 1,
        },
        Buffer::with_lines(vec!["Test─"]),
    );
    test_case(
        Block::default()
            .title("Test")
            .borders(Borders::LEFT | Borders::TOP),
        Rect {
            x: 0,
            y: 0,
            width: 5,
            height: 1,
        },
        Buffer::with_lines(vec!["┌Test"]),
    );
    test_case(
        Block::default()
            .title("Test")
            .borders(Borders::LEFT | Borders::TOP),
        Rect {
            x: 0,
            y: 0,
            width: 6,
            height: 1,
        },
        Buffer::with_lines(vec!["┌Test─"]),
    );
}

#[test]
fn widgets_block_title_alignment() {
    let test_case = |alignment, borders, expected| {
        let backend = TestBackend::new(15, 2);
        let mut terminal = Terminal::new(backend).unwrap();

        let block = Block::default()
            .title(Span::styled("Title", Style::default()))
            .title_alignment(alignment)
            .borders(borders);

        let area = Rect {
            x: 1,
            y: 0,
            width: 13,
            height: 2,
        };

        terminal
            .draw(|f| {
                f.render_widget(block, area);
            })
            .unwrap();

        terminal.backend().assert_buffer(&expected);
    };

    // title top-left with all borders
    test_case(
        Alignment::Left,
        Borders::ALL,
        Buffer::with_lines(vec![" ┌Title──────┐ ", " └───────────┘ "]),
    );

    // title top-left without top border
    test_case(
        Alignment::Left,
        Borders::LEFT | Borders::BOTTOM | Borders::RIGHT,
        Buffer::with_lines(vec![" │Title      │ ", " └───────────┘ "]),
    );

    // title top-left with no left border
    test_case(
        Alignment::Left,
        Borders::TOP | Borders::RIGHT | Borders::BOTTOM,
        Buffer::with_lines(vec![" Title───────┐ ", " ────────────┘ "]),
    );

    // title top-left without right border
    test_case(
        Alignment::Left,
        Borders::LEFT | Borders::TOP | Borders::BOTTOM,
        Buffer::with_lines(vec![" ┌Title─────── ", " └──────────── "]),
    );

    // title top-left without borders
    test_case(
        Alignment::Left,
        Borders::NONE,
        Buffer::with_lines(vec![" Title         ", "               "]),
    );

    // title center with all borders
    test_case(
        Alignment::Center,
        Borders::ALL,
        Buffer::with_lines(vec![" ┌───Title───┐ ", " └───────────┘ "]),
    );

    // title center without top border
    test_case(
        Alignment::Center,
        Borders::LEFT | Borders::BOTTOM | Borders::RIGHT,
        Buffer::with_lines(vec![" │   Title   │ ", " └───────────┘ "]),
    );

    // title center with no left border
    test_case(
        Alignment::Center,
        Borders::TOP | Borders::RIGHT | Borders::BOTTOM,
        Buffer::with_lines(vec![" ────Title───┐ ", " ────────────┘ "]),
    );

    // title center without right border
    test_case(
        Alignment::Center,
        Borders::LEFT | Borders::TOP | Borders::BOTTOM,
        Buffer::with_lines(vec![" ┌───Title──── ", " └──────────── "]),
    );

    // title center without borders
    test_case(
        Alignment::Center,
        Borders::NONE,
        Buffer::with_lines(vec!["     Title     ", "               "]),
    );

    // title top-right with all borders
    test_case(
        Alignment::Right,
        Borders::ALL,
        Buffer::with_lines(vec![" ┌──────Title┐ ", " └───────────┘ "]),
    );

    // title top-right without top border
    test_case(
        Alignment::Right,
        Borders::LEFT | Borders::BOTTOM | Borders::RIGHT,
        Buffer::with_lines(vec![" │      Title│ ", " └───────────┘ "]),
    );

    // title top-right with no left border
    test_case(
        Alignment::Right,
        Borders::TOP | Borders::RIGHT | Borders::BOTTOM,
        Buffer::with_lines(vec![" ───────Title┐ ", " ────────────┘ "]),
    );

    // title top-right without right border
    test_case(
        Alignment::Right,
        Borders::LEFT | Borders::TOP | Borders::BOTTOM,
        Buffer::with_lines(vec![" ┌───────Title ", " └──────────── "]),
    );

    // title top-right without borders
    test_case(
        Alignment::Right,
        Borders::NONE,
        Buffer::with_lines(vec!["         Title ", "               "]),
    );
}
