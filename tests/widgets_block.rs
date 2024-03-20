use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{
        block::title::{Position, Title},
        Block, Borders,
    },
    Terminal,
};

#[test]
fn widgets_block_renders() {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let block = Block::default()
        .title(Span::styled("Title", Style::default().fg(Color::LightBlue)))
        .borders(Borders::ALL);
    terminal
        .draw(|frame| frame.render_widget(block, Rect::new(0, 0, 8, 8)))
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
fn widgets_block_titles_overlap() {
    #[allow(clippy::needless_pass_by_value)]
    #[track_caller]
    fn test_case(block: Block, area: Rect, expected: Buffer) {
        let backend = TestBackend::new(area.width, area.height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| frame.render_widget(block, area))
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    }

    // Left overrides the center
    test_case(
        Block::default()
            .title(Title::from("aaaaa").alignment(Alignment::Left))
            .title(Title::from("bbb").alignment(Alignment::Center))
            .title(Title::from("ccc").alignment(Alignment::Right)),
        Rect::new(0, 0, 10, 1),
        Buffer::with_lines(vec!["aaaaab ccc"]),
    );

    // Left alignment overrides the center alignment which overrides the right alignment
    test_case(
        Block::default()
            .title(Title::from("aaaaa").alignment(Alignment::Left))
            .title(Title::from("bbbbb").alignment(Alignment::Center))
            .title(Title::from("ccccc").alignment(Alignment::Right)),
        Rect::new(0, 0, 11, 1),
        Buffer::with_lines(vec!["aaaaabbbccc"]),
    );

    // Multiple left alignment overrides the center alignment and the right alignment
    test_case(
        Block::default()
            .title(Title::from("aaaaa").alignment(Alignment::Left))
            .title(Title::from("aaaaa").alignment(Alignment::Left))
            .title(Title::from("bbbbb").alignment(Alignment::Center))
            .title(Title::from("ccccc").alignment(Alignment::Right)),
        Rect::new(0, 0, 11, 1),
        Buffer::with_lines(vec!["aaaaabaaaaa"]),
    );

    // The right alignment doesn't override the center alignment, but pierces through it
    test_case(
        Block::default()
            .title(Title::from("bbbbb").alignment(Alignment::Center))
            .title(Title::from("ccccccccccc").alignment(Alignment::Right)),
        Rect::new(0, 0, 11, 1),
        Buffer::with_lines(vec!["cccbbbbbccc"]),
    );
}

#[test]
fn widgets_block_renders_on_small_areas() {
    #[allow(clippy::needless_pass_by_value)]
    #[track_caller]
    fn test_case(block: Block, area: Rect, expected: Buffer) {
        let backend = TestBackend::new(area.width, area.height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| frame.render_widget(block, area))
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    }

    let one_cell_test_cases = [
        (Borders::NONE, "T"),
        (Borders::LEFT, "│"),
        (Borders::TOP, "T"),
        (Borders::RIGHT, "│"),
        (Borders::BOTTOM, "T"),
        (Borders::ALL, "┌"),
    ];
    for (borders, symbol) in one_cell_test_cases {
        test_case(
            Block::default().title("Test").borders(borders),
            Rect::new(0, 0, 0, 0),
            Buffer::empty(Rect::new(0, 0, 0, 0)),
        );
        test_case(
            Block::default().title("Test").borders(borders),
            Rect::new(0, 0, 1, 0),
            Buffer::empty(Rect::new(0, 0, 1, 0)),
        );
        test_case(
            Block::default().title("Test").borders(borders),
            Rect::new(0, 0, 0, 1),
            Buffer::empty(Rect::new(0, 0, 0, 1)),
        );
        test_case(
            Block::default().title("Test").borders(borders),
            Rect::new(0, 0, 1, 1),
            Buffer::with_lines(vec![symbol]),
        );
    }
    test_case(
        Block::default().title("Test").borders(Borders::LEFT),
        Rect::new(0, 0, 4, 1),
        Buffer::with_lines(vec!["│Tes"]),
    );
    test_case(
        Block::default().title("Test").borders(Borders::RIGHT),
        Rect::new(0, 0, 4, 1),
        Buffer::with_lines(vec!["Tes│"]),
    );
    test_case(
        Block::default().title("Test").borders(Borders::RIGHT),
        Rect::new(0, 0, 4, 1),
        Buffer::with_lines(vec!["Tes│"]),
    );
    test_case(
        Block::default()
            .title("Test")
            .borders(Borders::LEFT | Borders::RIGHT),
        Rect::new(0, 0, 4, 1),
        Buffer::with_lines(vec!["│Te│"]),
    );
    test_case(
        Block::default().title("Test").borders(Borders::TOP),
        Rect::new(0, 0, 4, 1),
        Buffer::with_lines(vec!["Test"]),
    );
    test_case(
        Block::default().title("Test").borders(Borders::TOP),
        Rect::new(0, 0, 5, 1),
        Buffer::with_lines(vec!["Test─"]),
    );
    test_case(
        Block::default()
            .title("Test")
            .borders(Borders::LEFT | Borders::TOP),
        Rect::new(0, 0, 5, 1),
        Buffer::with_lines(vec!["┌Test"]),
    );
    test_case(
        Block::default()
            .title("Test")
            .borders(Borders::LEFT | Borders::TOP),
        Rect::new(0, 0, 6, 1),
        Buffer::with_lines(vec!["┌Test─"]),
    );
}

#[allow(clippy::too_many_lines)]
#[test]
fn widgets_block_title_alignment() {
    #[allow(clippy::needless_pass_by_value)]
    #[track_caller]
    fn test_case(alignment: Alignment, borders: Borders, expected: Buffer) {
        let backend = TestBackend::new(15, 3);
        let mut terminal = Terminal::new(backend).unwrap();

        let block1 = Block::default()
            .title(Title::from(Span::styled("Title", Style::default())).alignment(alignment))
            .borders(borders);

        let block2 = Block::default()
            .title("Title")
            .title_alignment(alignment)
            .borders(borders);

        let area = Rect::new(1, 0, 13, 3);

        for block in [block1, block2] {
            terminal
                .draw(|frame| frame.render_widget(block, area))
                .unwrap();
            terminal.backend().assert_buffer(&expected);
        }
    }

    // title top-left with all borders
    test_case(
        Alignment::Left,
        Borders::ALL,
        Buffer::with_lines(vec![
            " ┌Title──────┐ ",
            " │           │ ",
            " └───────────┘ ",
        ]),
    );

    // title top-left without top border
    test_case(
        Alignment::Left,
        Borders::LEFT | Borders::BOTTOM | Borders::RIGHT,
        Buffer::with_lines(vec![
            " │Title      │ ",
            " │           │ ",
            " └───────────┘ ",
        ]),
    );

    // title top-left with no left border
    test_case(
        Alignment::Left,
        Borders::TOP | Borders::RIGHT | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " Title───────┐ ",
            "             │ ",
            " ────────────┘ ",
        ]),
    );

    // title top-left without right border
    test_case(
        Alignment::Left,
        Borders::LEFT | Borders::TOP | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ┌Title─────── ",
            " │             ",
            " └──────────── ",
        ]),
    );

    // title top-left without borders
    test_case(
        Alignment::Left,
        Borders::NONE,
        Buffer::with_lines(vec![
            " Title         ",
            "               ",
            "               ",
        ]),
    );

    // title center with all borders
    test_case(
        Alignment::Center,
        Borders::ALL,
        Buffer::with_lines(vec![
            " ┌───Title───┐ ",
            " │           │ ",
            " └───────────┘ ",
        ]),
    );

    // title center without top border
    test_case(
        Alignment::Center,
        Borders::LEFT | Borders::BOTTOM | Borders::RIGHT,
        Buffer::with_lines(vec![
            " │   Title   │ ",
            " │           │ ",
            " └───────────┘ ",
        ]),
    );

    // title center with no left border
    test_case(
        Alignment::Center,
        Borders::TOP | Borders::RIGHT | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ───Title────┐ ",
            "             │ ",
            " ────────────┘ ",
        ]),
    );

    // title center without right border
    test_case(
        Alignment::Center,
        Borders::LEFT | Borders::TOP | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ┌───Title──── ",
            " │             ",
            " └──────────── ",
        ]),
    );

    // title center without borders
    test_case(
        Alignment::Center,
        Borders::NONE,
        Buffer::with_lines(vec![
            "     Title     ",
            "               ",
            "               ",
        ]),
    );

    // title top-right with all borders
    test_case(
        Alignment::Right,
        Borders::ALL,
        Buffer::with_lines(vec![
            " ┌──────Title┐ ",
            " │           │ ",
            " └───────────┘ ",
        ]),
    );

    // title top-right without top border
    test_case(
        Alignment::Right,
        Borders::LEFT | Borders::BOTTOM | Borders::RIGHT,
        Buffer::with_lines(vec![
            " │      Title│ ",
            " │           │ ",
            " └───────────┘ ",
        ]),
    );

    // title top-right with no left border
    test_case(
        Alignment::Right,
        Borders::TOP | Borders::RIGHT | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ───────Title┐ ",
            "             │ ",
            " ────────────┘ ",
        ]),
    );

    // title top-right without right border
    test_case(
        Alignment::Right,
        Borders::LEFT | Borders::TOP | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ┌───────Title ",
            " │             ",
            " └──────────── ",
        ]),
    );

    // title top-right without borders
    test_case(
        Alignment::Right,
        Borders::NONE,
        Buffer::with_lines(vec![
            "         Title ",
            "               ",
            "               ",
        ]),
    );
}

#[allow(clippy::too_many_lines)]
#[test]
fn widgets_block_title_alignment_bottom() {
    #[allow(clippy::needless_pass_by_value)]
    #[track_caller]
    fn test_case(alignment: Alignment, borders: Borders, expected: Buffer) {
        let backend = TestBackend::new(15, 3);
        let mut terminal = Terminal::new(backend).unwrap();

        let title = Title::from(Span::styled("Title", Style::default()))
            .alignment(alignment)
            .position(Position::Bottom);
        let block = Block::default().title(title).borders(borders);
        let area = Rect::new(1, 0, 13, 3);
        terminal
            .draw(|frame| frame.render_widget(block, area))
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    }

    // title bottom-left with all borders
    test_case(
        Alignment::Left,
        Borders::ALL,
        Buffer::with_lines(vec![
            " ┌───────────┐ ",
            " │           │ ",
            " └Title──────┘ ",
        ]),
    );

    // title bottom-left without bottom border
    test_case(
        Alignment::Left,
        Borders::LEFT | Borders::TOP | Borders::RIGHT,
        Buffer::with_lines(vec![
            " ┌───────────┐ ",
            " │           │ ",
            " │Title      │ ",
        ]),
    );

    // title bottom-left with no left border
    test_case(
        Alignment::Left,
        Borders::TOP | Borders::RIGHT | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ────────────┐ ",
            "             │ ",
            " Title───────┘ ",
        ]),
    );

    // title bottom-left without right border
    test_case(
        Alignment::Left,
        Borders::LEFT | Borders::TOP | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ┌──────────── ",
            " │             ",
            " └Title─────── ",
        ]),
    );

    // title bottom-left without borders
    test_case(
        Alignment::Left,
        Borders::NONE,
        Buffer::with_lines(vec![
            "               ",
            "               ",
            " Title         ",
        ]),
    );

    // title center with all borders
    test_case(
        Alignment::Center,
        Borders::ALL,
        Buffer::with_lines(vec![
            " ┌───────────┐ ",
            " │           │ ",
            " └───Title───┘ ",
        ]),
    );

    // title center without bottom border
    test_case(
        Alignment::Center,
        Borders::LEFT | Borders::TOP | Borders::RIGHT,
        Buffer::with_lines(vec![
            " ┌───────────┐ ",
            " │           │ ",
            " │   Title   │ ",
        ]),
    );

    // title center with no left border
    test_case(
        Alignment::Center,
        Borders::TOP | Borders::RIGHT | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ────────────┐ ",
            "             │ ",
            " ───Title────┘ ",
        ]),
    );

    // title center without right border
    test_case(
        Alignment::Center,
        Borders::LEFT | Borders::TOP | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ┌──────────── ",
            " │             ",
            " └───Title──── ",
        ]),
    );

    // title center without borders
    test_case(
        Alignment::Center,
        Borders::NONE,
        Buffer::with_lines(vec![
            "               ",
            "               ",
            "     Title     ",
        ]),
    );

    // title bottom-right with all borders
    test_case(
        Alignment::Right,
        Borders::ALL,
        Buffer::with_lines(vec![
            " ┌───────────┐ ",
            " │           │ ",
            " └──────Title┘ ",
        ]),
    );

    // title bottom-right without bottom border
    test_case(
        Alignment::Right,
        Borders::LEFT | Borders::TOP | Borders::RIGHT,
        Buffer::with_lines(vec![
            " ┌───────────┐ ",
            " │           │ ",
            " │      Title│ ",
        ]),
    );

    // title bottom-right with no left border
    test_case(
        Alignment::Right,
        Borders::TOP | Borders::RIGHT | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ────────────┐ ",
            "             │ ",
            " ───────Title┘ ",
        ]),
    );

    // title bottom-right without right border
    test_case(
        Alignment::Right,
        Borders::LEFT | Borders::TOP | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ┌──────────── ",
            " │             ",
            " └───────Title ",
        ]),
    );

    // title bottom-right without borders
    test_case(
        Alignment::Right,
        Borders::NONE,
        Buffer::with_lines(vec![
            "               ",
            "               ",
            "         Title ",
        ]),
    );
}

#[allow(clippy::too_many_lines)]
#[test]
fn widgets_block_multiple_titles() {
    #[allow(clippy::needless_pass_by_value)]
    #[track_caller]
    fn test_case(title_a: Title, title_b: Title, borders: Borders, expected: Buffer) {
        let backend = TestBackend::new(15, 3);
        let mut terminal = Terminal::new(backend).unwrap();

        let block = Block::default()
            .title(title_a)
            .title(title_b)
            .borders(borders);

        let area = Rect::new(1, 0, 13, 3);

        terminal
            .draw(|f| {
                f.render_widget(block, area);
            })
            .unwrap();

        terminal.backend().assert_buffer(&expected);
    }

    // title bottom-left with all borders
    test_case(
        Title::from("foo"),
        Title::from("bar"),
        Borders::ALL,
        Buffer::with_lines(vec![
            " ┌foo─bar────┐ ",
            " │           │ ",
            " └───────────┘ ",
        ]),
    );

    // title top-left without top border
    test_case(
        Title::from("foo"),
        Title::from("bar"),
        Borders::LEFT | Borders::BOTTOM | Borders::RIGHT,
        Buffer::with_lines(vec![
            " │foo bar    │ ",
            " │           │ ",
            " └───────────┘ ",
        ]),
    );

    // title top-left with no left border
    test_case(
        Title::from("foo"),
        Title::from("bar"),
        Borders::TOP | Borders::RIGHT | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " foo─bar─────┐ ",
            "             │ ",
            " ────────────┘ ",
        ]),
    );

    // title top-left without right border
    test_case(
        Title::from("foo"),
        Title::from("bar"),
        Borders::LEFT | Borders::TOP | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ┌foo─bar───── ",
            " │             ",
            " └──────────── ",
        ]),
    );

    // title top-left without borders
    test_case(
        Title::from("foo"),
        Title::from("bar"),
        Borders::NONE,
        Buffer::with_lines(vec![
            " foo bar       ",
            "               ",
            "               ",
        ]),
    );

    // title center with all borders
    test_case(
        Title::from("foo").alignment(Alignment::Center),
        Title::from("bar").alignment(Alignment::Center),
        Borders::ALL,
        Buffer::with_lines(vec![
            " ┌──foo─bar──┐ ",
            " │           │ ",
            " └───────────┘ ",
        ]),
    );

    // title center without top border
    test_case(
        Title::from("foo").alignment(Alignment::Center),
        Title::from("bar").alignment(Alignment::Center),
        Borders::LEFT | Borders::BOTTOM | Borders::RIGHT,
        Buffer::with_lines(vec![
            " │  foo bar  │ ",
            " │           │ ",
            " └───────────┘ ",
        ]),
    );

    // title center with no left border
    test_case(
        Title::from("foo").alignment(Alignment::Center),
        Title::from("bar").alignment(Alignment::Center),
        Borders::TOP | Borders::RIGHT | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ──foo─bar───┐ ",
            "             │ ",
            " ────────────┘ ",
        ]),
    );

    // title center without right border
    test_case(
        Title::from("foo").alignment(Alignment::Center),
        Title::from("bar").alignment(Alignment::Center),
        Borders::LEFT | Borders::TOP | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ┌──foo─bar─── ",
            " │             ",
            " └──────────── ",
        ]),
    );

    // title center without borders
    test_case(
        Title::from("foo").alignment(Alignment::Center),
        Title::from("bar").alignment(Alignment::Center),
        Borders::NONE,
        Buffer::with_lines(vec![
            "    foo bar    ",
            "               ",
            "               ",
        ]),
    );

    // title top-right with all borders
    test_case(
        Title::from("foo").alignment(Alignment::Right),
        Title::from("bar").alignment(Alignment::Right),
        Borders::ALL,
        Buffer::with_lines(vec![
            " ┌────foo─bar┐ ",
            " │           │ ",
            " └───────────┘ ",
        ]),
    );

    // title top-right without top border
    test_case(
        Title::from("foo").alignment(Alignment::Right),
        Title::from("bar").alignment(Alignment::Right),
        Borders::LEFT | Borders::BOTTOM | Borders::RIGHT,
        Buffer::with_lines(vec![
            " │    foo bar│ ",
            " │           │ ",
            " └───────────┘ ",
        ]),
    );

    // title top-right with no left border
    test_case(
        Title::from("foo").alignment(Alignment::Right),
        Title::from("bar").alignment(Alignment::Right),
        Borders::TOP | Borders::RIGHT | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ─────foo─bar┐ ",
            "             │ ",
            " ────────────┘ ",
        ]),
    );

    // title top-right without right border
    test_case(
        Title::from("foo").alignment(Alignment::Right),
        Title::from("bar").alignment(Alignment::Right),
        Borders::LEFT | Borders::TOP | Borders::BOTTOM,
        Buffer::with_lines(vec![
            " ┌─────foo─bar ",
            " │             ",
            " └──────────── ",
        ]),
    );

    // title top-right without borders
    test_case(
        Title::from("foo").alignment(Alignment::Right),
        Title::from("bar").alignment(Alignment::Right),
        Borders::NONE,
        Buffer::with_lines(vec![
            "       foo bar ",
            "               ",
            "               ",
        ]),
    );
}
