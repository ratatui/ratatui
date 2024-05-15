use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders},
    Terminal,
};
use rstest::rstest;

#[test]
fn widgets_block_renders() {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let block =
        Block::bordered().title_top(Span::styled("Line", Style::default().fg(Color::LightBlue)));
    terminal
        .draw(|frame| frame.render_widget(block, Rect::new(0, 0, 8, 8)))
        .unwrap();
    let mut expected = Buffer::with_lines([
        "┌Line──┐  ",
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
    for x in 1..=4 {
        expected.get_mut(x, 0).set_fg(Color::LightBlue);
    }
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_block_titles_overlap() {
    #[track_caller]
    fn test_case<'line, Lines>(block: Block, area: Rect, expected: Lines)
    where
        Lines: IntoIterator,
        Lines::Item: Into<ratatui::text::Line<'line>>,
    {
        let backend = TestBackend::new(area.width, area.height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| frame.render_widget(block, area))
            .unwrap();
        terminal.backend().assert_buffer_lines(expected);
    }

    // Left overrides the center
    test_case(
        Block::new()
            .title_top(Line::from("aaaaa").left_aligned())
            .title_top(Line::from("bbb").centered())
            .title_top(Line::from("ccc").right_aligned()),
        Rect::new(0, 0, 10, 1),
        ["aaaaab ccc"],
    );

    // Left alignment overrides the center alignment which overrides the right alignment
    test_case(
        Block::new()
            .title_top(Line::from("aaaaa").left_aligned())
            .title_top(Line::from("bbbbb").centered())
            .title_top(Line::from("ccccc").right_aligned()),
        Rect::new(0, 0, 11, 1),
        ["aaaaabbbccc"],
    );

    // Multiple left alignment overrides the center alignment and the right alignment
    test_case(
        Block::new()
            .title_top(Line::from("aaaaa").left_aligned())
            .title_top(Line::from("aaaaa").left_aligned())
            .title_top(Line::from("bbbbb").centered())
            .title_top(Line::from("ccccc").right_aligned()),
        Rect::new(0, 0, 11, 1),
        ["aaaaabaaaaa"],
    );

    // The right alignment doesn't override the center alignment, but pierces through it
    test_case(
        Block::new()
            .title_top(Line::from("bbbbb").centered())
            .title_top(Line::from("ccccccccccc").right_aligned()),
        Rect::new(0, 0, 11, 1),
        ["cccbbbbbccc"],
    );
}

#[test]
fn widgets_block_renders_on_small_areas() {
    #[track_caller]
    fn test_case(block: Block, area: Rect, expected: &Buffer) {
        let backend = TestBackend::new(area.width, area.height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| frame.render_widget(block, area))
            .unwrap();
        terminal.backend().assert_buffer(expected);
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
            Block::new().borders(borders).title_top("Test"),
            Rect::new(0, 0, 0, 0),
            &Buffer::empty(Rect::new(0, 0, 0, 0)),
        );
        test_case(
            Block::new().borders(borders).title_top("Test"),
            Rect::new(0, 0, 1, 0),
            &Buffer::empty(Rect::new(0, 0, 1, 0)),
        );
        test_case(
            Block::new().borders(borders).title_top("Test"),
            Rect::new(0, 0, 0, 1),
            &Buffer::empty(Rect::new(0, 0, 0, 1)),
        );
        test_case(
            Block::new().borders(borders).title_top("Test"),
            Rect::new(0, 0, 1, 1),
            &Buffer::with_lines([symbol]),
        );
    }
    test_case(
        Block::new().borders(Borders::LEFT).title_top("Test"),
        Rect::new(0, 0, 4, 1),
        &Buffer::with_lines(["│Tes"]),
    );
    test_case(
        Block::new().borders(Borders::RIGHT).title_top("Test"),
        Rect::new(0, 0, 4, 1),
        &Buffer::with_lines(["Tes│"]),
    );
    test_case(
        Block::new().borders(Borders::RIGHT).title_top("Test"),
        Rect::new(0, 0, 4, 1),
        &Buffer::with_lines(["Tes│"]),
    );
    test_case(
        Block::new()
            .borders(Borders::LEFT | Borders::RIGHT)
            .title_top("Test"),
        Rect::new(0, 0, 4, 1),
        &Buffer::with_lines(["│Te│"]),
    );
    test_case(
        Block::new().borders(Borders::TOP).title_top("Test"),
        Rect::new(0, 0, 4, 1),
        &Buffer::with_lines(["Test"]),
    );
    test_case(
        Block::new().borders(Borders::TOP).title_top("Test"),
        Rect::new(0, 0, 5, 1),
        &Buffer::with_lines(["Test─"]),
    );
    test_case(
        Block::new()
            .borders(Borders::LEFT | Borders::TOP)
            .title_top("Test"),
        Rect::new(0, 0, 5, 1),
        &Buffer::with_lines(["┌Test"]),
    );
    test_case(
        Block::new()
            .borders(Borders::LEFT | Borders::TOP)
            .title_top("Test"),
        Rect::new(0, 0, 6, 1),
        &Buffer::with_lines(["┌Test─"]),
    );
}

#[rstest]
#[case::left_with_all_borders(Alignment::Left, Borders::ALL, [
    " ┌Line───────┐ ",
    " │           │ ",
    " └───────────┘ ",
])]
#[case::left_without_top_border(Alignment::Left, Borders::LEFT | Borders::BOTTOM | Borders::RIGHT, [
    " │Line       │ ",
    " │           │ ",
    " └───────────┘ ",
])]
#[case::left_without_left_border(Alignment::Left, Borders::TOP | Borders::RIGHT | Borders::BOTTOM, [
    " Line────────┐ ",
    "             │ ",
    " ────────────┘ ",
])]
#[case::left_without_right_border(Alignment::Left, Borders::LEFT | Borders::TOP | Borders::BOTTOM, [
    " ┌Line──────── ",
    " │             ",
    " └──────────── ",
])]
#[case::left_without_borders(Alignment::Left, Borders::NONE, [
    " Line         ",
    "               ",
    "               ",
])]
#[case::center_with_all_borders(Alignment::Center, Borders::ALL, [
    " ┌───Line────┐ ",
    " │           │ ",
    " └───────────┘ ",
])]
#[case::center_without_top_border(Alignment::Center, Borders::LEFT | Borders::BOTTOM | Borders::RIGHT, [
    " │   Line    │ ",
    " │           │ ",
    " └───────────┘ ",
])]
#[case::center_without_left_border(Alignment::Center, Borders::TOP | Borders::RIGHT | Borders::BOTTOM, [
    " ────Line────┐ ",
    "             │ ",
    " ────────────┘ ",
])]
#[case::center_without_right_border(Alignment::Center, Borders::LEFT | Borders::TOP | Borders::BOTTOM, [
    " ┌────Line──── ",
    " │             ",
    " └──────────── ",
])]
#[case::center_without_borders(Alignment::Center, Borders::NONE, [
    "     Line      ",
    "               ",
    "               ",
])]
#[case::right_with_all_borders(Alignment::Right, Borders::ALL, [
    " ┌───────Line┐ ",
    " │           │ ",
    " └───────────┘ ",
])]
#[case::right_without_top_border(Alignment::Right, Borders::LEFT | Borders::BOTTOM | Borders::RIGHT, [
    " │       Line│ ",
    " │           │ ",
    " └───────────┘ ",
])]
#[case::right_without_left_border(Alignment::Right, Borders::TOP | Borders::RIGHT | Borders::BOTTOM, [
    " ────────Line┐ ",
    "             │ ",
    " ────────────┘ ",
])]
#[case::right_without_right_border(Alignment::Right, Borders::LEFT | Borders::TOP | Borders::BOTTOM, [
    " ┌────────Line ",
    " │             ",
    " └──────────── ",
])]
#[case::right_without_borders(Alignment::Right, Borders::NONE, [
    "          Line ",
    "               ",
    "               ",
])]
fn widgets_block_title_alignment_top<'line, Lines>(
    #[case] alignment: Alignment,
    #[case] borders: Borders,
    #[case] expected: Lines,
) where
    Lines: IntoIterator,
    Lines::Item: Into<ratatui::text::Line<'line>>,
{
    let backend = TestBackend::new(15, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    let block1 = Block::new()
        .borders(borders)
        .title_top(Line::from(Span::raw("Line")).alignment(alignment));

    let block2 = Block::new()
        .borders(borders)
        .title_alignment(alignment)
        .title_top("Line");
    let area = Rect::new(1, 0, 13, 3);
    let expected = Buffer::with_lines(expected);
    for block in [block1, block2] {
        terminal
            .draw(|frame| frame.render_widget(block, area))
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    }
}

#[rstest]
#[case::left(Alignment::Left, Borders::ALL, [
    " ┌───────────┐ ",
    " │           │ ",
    " └Line───────┘ ",
])]
#[case::left(Alignment::Left, Borders::LEFT | Borders::TOP | Borders::RIGHT, [
    " ┌───────────┐ ",
    " │           │ ",
    " │Line       │ ",
])]
#[case::left(Alignment::Left, Borders::TOP | Borders::RIGHT | Borders::BOTTOM, [
    " ────────────┐ ",
    "             │ ",
    " Line────────┘ ",
])]
#[case::left(Alignment::Left, Borders::LEFT | Borders::TOP | Borders::BOTTOM, [
    " ┌──────────── ",
    " │             ",
    " └Line──────── ",
])]
#[case::left(Alignment::Left, Borders::NONE, [
    "               ",
    "               ",
    " Line          ",
])]
#[case::left(Alignment::Center, Borders::ALL, [
    " ┌───────────┐ ",
    " │           │ ",
    " └───Line────┘ ",
])]
#[case::left(Alignment::Center, Borders::LEFT | Borders::TOP | Borders::RIGHT, [
    " ┌───────────┐ ",
    " │           │ ",
    " │   Line    │ ",
])]
#[case::left(Alignment::Center, Borders::TOP | Borders::RIGHT | Borders::BOTTOM, [
    " ────────────┐ ",
    "             │ ",
    " ────Line────┘ ",
])]
#[case::left(Alignment::Center, Borders::LEFT | Borders::TOP | Borders::BOTTOM, [
    " ┌──────────── ",
    " │             ",
    " └────Line──── ",
])]
#[case::left(Alignment::Center, Borders::NONE, [
    "               ",
    "               ",
    "     Line      ",
])]
#[case::left(Alignment::Right, Borders::ALL, [
    " ┌───────────┐ ",
    " │           │ ",
    " └───────Line┘ ",
])]
#[case::left(Alignment::Right, Borders::LEFT | Borders::TOP | Borders::RIGHT, [
    " ┌───────────┐ ",
    " │           │ ",
    " │       Line│ ",
])]
#[case::left(Alignment::Right, Borders::TOP | Borders::RIGHT | Borders::BOTTOM, [
    " ────────────┐ ",
    "             │ ",
    " ────────Line┘ ",
])]
#[case::left(Alignment::Right, Borders::LEFT | Borders::TOP | Borders::BOTTOM, [
    " ┌──────────── ",
    " │             ",
    " └────────Line ",
])]
#[case::left(Alignment::Right, Borders::NONE, [
    "               ",
    "               ",
    "          Line ",
])]
fn widgets_block_title_alignment_bottom<'line, Lines>(
    #[case] alignment: Alignment,
    #[case] borders: Borders,
    #[case] expected: Lines,
) where
    Lines: IntoIterator,
    Lines::Item: Into<ratatui::text::Line<'line>>,
{
    let backend = TestBackend::new(15, 3);
    let mut terminal = Terminal::new(backend).unwrap();

    let title = Line::from(Span::styled("Line", Style::default())).alignment(alignment);
    let block = Block::default().title_bottom(title).borders(borders);
    let area = Rect::new(1, 0, 13, 3);
    terminal
        .draw(|frame| frame.render_widget(block, area))
        .unwrap();
    terminal.backend().assert_buffer_lines(expected);
}

#[rstest]
#[case::left_with_all_borders(Line::from("foo"), Line::from("bar"), Borders::ALL, [
    " ┌foo─bar────┐ ",
    " │           │ ",
    " └───────────┘ ",
])]
#[case::left_without_top_border(Line::from("foo"), Line::from("bar"), Borders::LEFT | Borders::BOTTOM | Borders::RIGHT, [
    " │foo bar    │ ",
    " │           │ ",
    " └───────────┘ ",
])]
#[case::left_without_left_border(Line::from("foo"), Line::from("bar"), Borders::TOP | Borders::RIGHT | Borders::BOTTOM, [
    " foo─bar─────┐ ",
    "             │ ",
    " ────────────┘ ",
])]
#[case::left_without_right_border(Line::from("foo"), Line::from("bar"), Borders::LEFT | Borders::TOP | Borders::BOTTOM, [
    " ┌foo─bar───── ",
    " │             ",
    " └──────────── ",
])]
#[case::left_without_borders(Line::from("foo"), Line::from("bar"), Borders::NONE, [
    " foo bar       ",
    "               ",
    "               ",
])]
#[case::center_with_borders(Line::from("foo").alignment(Alignment::Center), Line::from("bar").alignment(Alignment::Center), Borders::ALL, [
    " ┌──foo─bar──┐ ",
    " │           │ ",
    " └───────────┘ ",
])]
#[case::center_without_top_border(Line::from("foo").alignment(Alignment::Center), Line::from("bar").alignment(Alignment::Center), Borders::LEFT | Borders::BOTTOM | Borders::RIGHT, [
    " │  foo bar  │ ",
    " │           │ ",
    " └───────────┘ ",
])]
#[case::center_without_left_border(Line::from("foo").alignment(Alignment::Center), Line::from("bar").alignment(Alignment::Center), Borders::TOP | Borders::RIGHT | Borders::BOTTOM, [
    " ──foo─bar───┐ ",
    "             │ ",
    " ────────────┘ ",
])]
#[case::center_without_right_border(Line::from("foo").alignment(Alignment::Center), Line::from("bar").alignment(Alignment::Center), Borders::LEFT | Borders::TOP | Borders::BOTTOM, [
    " ┌──foo─bar─── ",
    " │             ",
    " └──────────── ",
])]
#[case::center_without_borders(Line::from("foo").alignment(Alignment::Center), Line::from("bar").alignment(Alignment::Center), Borders::NONE, [
    "    foo bar    ",
    "               ",
    "               ",
])]
#[case::right_with_all_borders(Line::from("foo").alignment(Alignment::Right), Line::from("bar").alignment(Alignment::Right), Borders::ALL, [
    " ┌────foo─bar┐ ",
    " │           │ ",
    " └───────────┘ ",
])]
#[case::right_without_top_border(Line::from("foo").alignment(Alignment::Right), Line::from("bar").alignment(Alignment::Right), Borders::LEFT | Borders::BOTTOM | Borders::RIGHT, [
    " │    foo bar│ ",
    " │           │ ",
    " └───────────┘ ",
])]
#[case::right_without_left_border(Line::from("foo").alignment(Alignment::Right), Line::from("bar").alignment(Alignment::Right), Borders::TOP | Borders::RIGHT | Borders::BOTTOM, [
    " ─────foo─bar┐ ",
    "             │ ",
    " ────────────┘ ",
])]
#[case::right_without_right_border(Line::from("foo").alignment(Alignment::Right), Line::from("bar").alignment(Alignment::Right), Borders::LEFT | Borders::TOP | Borders::BOTTOM, [
    " ┌─────foo─bar ",
    " │             ",
    " └──────────── ",
])]
#[case::right_without_borders(Line::from("foo").alignment(Alignment::Right), Line::from("bar").alignment(Alignment::Right), Borders::NONE, [
    "       foo bar ",
    "               ",
    "               ",
])]
fn widgets_block_multiple_titles<'line, Lines>(
    #[case] title_a: Line,
    #[case] title_b: Line,
    #[case] borders: Borders,
    #[case] expected: Lines,
) where
    Lines: IntoIterator,
    Lines::Item: Into<ratatui::text::Line<'line>>,
{
    let backend = TestBackend::new(15, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    let block = Block::default()
        .title_top(title_a)
        .title_top(title_b)
        .borders(borders);
    let area = Rect::new(1, 0, 13, 3);
    terminal
        .draw(|f| {
            f.render_widget(block, area);
        })
        .unwrap();
    terminal.backend().assert_buffer_lines(expected);
}
