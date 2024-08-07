use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    symbols,
    text::{self, Span},
    widgets::{Axis, Block, Chart, Dataset, GraphType::Line},
    Terminal,
};
use rstest::rstest;

fn create_labels<'a>(labels: &'a [&'a str]) -> Vec<Span<'a>> {
    labels.iter().map(|l| Span::from(*l)).collect()
}

#[track_caller]
fn axis_test_case<'line, Lines>(
    width: u16,
    height: u16,
    x_axis: Axis,
    y_axis: Axis,
    expected: Lines,
) where
    Lines: IntoIterator,
    Lines::Item: Into<text::Line<'line>>,
{
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let chart = Chart::new(vec![]).x_axis(x_axis).y_axis(y_axis);
            f.render_widget(chart, f.area());
        })
        .unwrap();
    terminal.backend().assert_buffer_lines(expected);
}

#[rstest]
#[case(0, 0)]
#[case(0, 1)]
#[case(1, 0)]
#[case(1, 1)]
#[case(2, 2)]
fn widgets_chart_can_render_on_small_areas(#[case] width: u16, #[case] height: u16) {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let datasets = vec![Dataset::default()
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Magenta))
                .data(&[(0.0, 0.0)])];
            let chart = Chart::new(datasets)
                .block(Block::bordered().title("Plot"))
                .x_axis(
                    Axis::default()
                        .bounds([0.0, 0.0])
                        .labels(create_labels(&["0.0", "1.0"])),
                )
                .y_axis(
                    Axis::default()
                        .bounds([0.0, 0.0])
                        .labels(create_labels(&["0.0", "1.0"])),
                );
            f.render_widget(chart, f.area());
        })
        .unwrap();
}

#[rstest]
#[case(
    Some(("AAAA", "B")),
    None,
    Alignment::Left,
    vec![
        "          ",
        "          ",
        "          ",
        "   ───────",
        "AAA      B",
    ],
)]
#[case(
    Some(("A", "BBBB")),
    None,
    Alignment::Left,
    vec![
        "          ",
        "          ",
        "          ",
        " ─────────",
        "A     BBBB",
    ],
)]
#[case(
    Some(("AAAAAAAAAAA", "B")),
    None,
    Alignment::Left,
    vec![
        "          ",
        "          ",
        "          ",
        "   ───────",
        "AAA      B",
    ],
)]
#[case(
    Some(("A", "B")),
    Some(("CCCCCCC", "D")),
    Alignment::Left,
    vec![
        "D  │      ",
        "   │      ",
        "CCC│      ",
        "   └──────",
        "   A     B",
    ],
)]
#[case(
    Some(("AAAAAAAAAA", "B")),
    Some(("C", "D")),
    Alignment::Center,
    vec![
        "D  │      ",
        "   │      ",
        "C  │      ",
        "   └──────",
        "AAAAAAA  B",
    ],
)]
#[case(
    Some(("AAAAAAA", "B")),
    Some(("C", "D")),
    Alignment::Right,
    vec![
        "D│        ",
        " │        ",
        "C│        ",
        " └────────",
        " AAAAA   B",
    ],
)]
#[case(
    Some(("AAAAAAA", "BBBBBBB")),
    Some(("C", "D")),
    Alignment::Right,
    vec![
        "D│        ",
        " │        ",
        "C│        ",
        " └────────",
        " AAAAABBBB",
    ],
)]
fn widgets_chart_handles_long_labels<'line, Lines>(
    #[case] x_labels: Option<(&str, &str)>,
    #[case] y_labels: Option<(&str, &str)>,
    #[case] x_alignment: Alignment,
    #[case] expected: Lines,
) where
    Lines: IntoIterator,
    Lines::Item: Into<text::Line<'line>>,
{
    let mut x_axis = Axis::default().bounds([0.0, 1.0]);
    if let Some((left_label, right_label)) = x_labels {
        x_axis = x_axis
            .labels([left_label, right_label])
            .labels_alignment(x_alignment);
    }
    let mut y_axis = Axis::default().bounds([0.0, 1.0]);
    if let Some((left_label, right_label)) = y_labels {
        y_axis = y_axis.labels([left_label, right_label]);
    }
    axis_test_case(10, 5, x_axis, y_axis, expected);
}

#[rstest]
#[case::left(
    Alignment::Left,
    vec![
        "          ",
        "          ",
        "          ",
        "   ───────",
        "AAA   B  C",
    ],
)]
#[case::center(
    Alignment::Center,
    vec![
        "          ",
        "          ",
        "          ",
        "  ────────",
        "AAAA B   C",
    ],
)]
#[case::right(
    Alignment::Right,
    vec![
        "          ",
        "          ",
        "          ",
        "──────────",
        "AAA B    C",
    ],
)]
fn widgets_chart_handles_x_axis_labels_alignments<'line, Lines>(
    #[case] y_alignment: Alignment,
    #[case] expected: Lines,
) where
    Lines: IntoIterator,
    Lines::Item: Into<text::Line<'line>>,
{
    let x_axis = Axis::default()
        .labels(["AAAA", "B", "C"])
        .labels_alignment(y_alignment);
    let y_axis = Axis::default();
    axis_test_case(10, 5, x_axis, y_axis, expected);
}

#[rstest]
#[case::left(Alignment::Left, [
    "D   │               ",
    "    │               ",
    "C   │               ",
    "    └───────────────",
    "AAAAA              B",
])]
#[case::center(Alignment::Center, [
    " D  │               ",
    "    │               ",
    " C  │               ",
    "    └───────────────",
    "AAAAA              B",
])]
#[case::right(Alignment::Right, [
    "   D│               ",
    "    │               ",
    "   C│               ",
    "    └───────────────",
    "AAAAA              B",
])]
fn widgets_chart_handles_y_axis_labels_alignments<'line, Lines>(
    #[case] y_alignment: Alignment,
    #[case] expected: Lines,
) where
    Lines: IntoIterator,
    Lines::Item: Into<text::Line<'line>>,
{
    let x_axis = Axis::default().labels(create_labels(&["AAAAA", "B"]));
    let y_axis = Axis::default()
        .labels(create_labels(&["C", "D"]))
        .labels_alignment(y_alignment);
    axis_test_case(20, 5, x_axis, y_axis, expected);
}

#[test]
fn widgets_chart_can_have_axis_with_zero_length_bounds() {
    let backend = TestBackend::new(100, 100);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let datasets = vec![Dataset::default()
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Magenta))
                .data(&[(0.0, 0.0)])];
            let chart = Chart::new(datasets)
                .block(Block::bordered().title("Plot"))
                .x_axis(
                    Axis::default()
                        .bounds([0.0, 0.0])
                        .labels(create_labels(&["0.0", "1.0"])),
                )
                .y_axis(
                    Axis::default()
                        .bounds([0.0, 0.0])
                        .labels(create_labels(&["0.0", "1.0"])),
                );
            f.render_widget(
                chart,
                Rect {
                    x: 0,
                    y: 0,
                    width: 100,
                    height: 100,
                },
            );
        })
        .unwrap();
}

#[test]
fn widgets_chart_handles_overflows() {
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let datasets = vec![Dataset::default()
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Magenta))
                .data(&[
                    (1_588_298_471.0, 1.0),
                    (1_588_298_473.0, 0.0),
                    (1_588_298_496.0, 1.0),
                ])];
            let chart = Chart::new(datasets)
                .block(Block::bordered().title("Plot"))
                .x_axis(
                    Axis::default()
                        .bounds([1_588_298_471.0, 1_588_992_600.0])
                        .labels(create_labels(&["1588298471.0", "1588992600.0"])),
                )
                .y_axis(
                    Axis::default()
                        .bounds([0.0, 1.0])
                        .labels(create_labels(&["0.0", "1.0"])),
                );
            f.render_widget(
                chart,
                Rect {
                    x: 0,
                    y: 0,
                    width: 80,
                    height: 30,
                },
            );
        })
        .unwrap();
}

#[test]
fn widgets_chart_can_have_empty_datasets() {
    let backend = TestBackend::new(100, 100);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let datasets = vec![Dataset::default().data(&[]).graph_type(Line)];
            let chart = Chart::new(datasets)
                .block(Block::bordered().title("Empty Dataset With Line"))
                .x_axis(
                    Axis::default()
                        .bounds([0.0, 0.0])
                        .labels(create_labels(&["0.0", "1.0"])),
                )
                .y_axis(
                    Axis::default()
                        .bounds([0.0, 1.0])
                        .labels(create_labels(&["0.0", "1.0"])),
                );
            f.render_widget(
                chart,
                Rect {
                    x: 0,
                    y: 0,
                    width: 100,
                    height: 100,
                },
            );
        })
        .unwrap();
}

#[allow(clippy::too_many_lines)]
#[test]
fn widgets_chart_can_have_a_legend() {
    let backend = TestBackend::new(60, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let datasets = vec![
                Dataset::default()
                    .name("Dataset 1")
                    .style(Style::default().fg(Color::Blue))
                    .data(&[
                        (0.0, 0.0),
                        (10.0, 1.0),
                        (20.0, 2.0),
                        (30.0, 3.0),
                        (40.0, 4.0),
                        (50.0, 5.0),
                        (60.0, 6.0),
                        (70.0, 7.0),
                        (80.0, 8.0),
                        (90.0, 9.0),
                        (100.0, 10.0),
                    ])
                    .graph_type(Line),
                Dataset::default()
                    .name("Dataset 2")
                    .style(Style::default().fg(Color::Green))
                    .data(&[
                        (0.0, 10.0),
                        (10.0, 9.0),
                        (20.0, 8.0),
                        (30.0, 7.0),
                        (40.0, 6.0),
                        (50.0, 5.0),
                        (60.0, 4.0),
                        (70.0, 3.0),
                        (80.0, 2.0),
                        (90.0, 1.0),
                        (100.0, 0.0),
                    ])
                    .graph_type(Line),
            ];
            let chart = Chart::new(datasets)
                .style(Style::default().bg(Color::White))
                .block(Block::bordered().title("Chart Test"))
                .x_axis(
                    Axis::default()
                        .bounds([0.0, 100.0])
                        .title(Span::styled("X Axis", Style::default().fg(Color::Yellow)))
                        .labels(create_labels(&["0.0", "50.0", "100.0"])),
                )
                .y_axis(
                    Axis::default()
                        .bounds([0.0, 10.0])
                        .title("Y Axis")
                        .labels(create_labels(&["0.0", "5.0", "10.0"])),
                );
            f.render_widget(
                chart,
                Rect {
                    x: 0,
                    y: 0,
                    width: 60,
                    height: 30,
                },
            );
        })
        .unwrap();
    let mut expected = Buffer::with_lines([
        "┌Chart Test────────────────────────────────────────────────┐",
        "│10.0│Y Axis                                    ┌─────────┐│",
        "│    │  ••                                      │Dataset 1││",
        "│    │    ••                                    │Dataset 2││",
        "│    │      ••                                  └─────────┘│",
        "│    │        ••                                ••         │",
        "│    │          ••                            ••           │",
        "│    │            ••                        ••             │",
        "│    │              ••                    ••               │",
        "│    │                ••                ••                 │",
        "│    │                  ••            ••                   │",
        "│    │                    ••        ••                     │",
        "│    │                      •••   ••                       │",
        "│    │                         •••                         │",
        "│5.0 │                        •• ••                        │",
        "│    │                      ••     ••                      │",
        "│    │                   •••         ••                    │",
        "│    │                 ••              ••                  │",
        "│    │               ••                  ••                │",
        "│    │             ••                      ••              │",
        "│    │           ••                          ••            │",
        "│    │         ••                              ••          │",
        "│    │       ••                                  ••        │",
        "│    │     ••                                      •••     │",
        "│    │   ••                                           ••   │",
        "│    │ ••                                               •• │",
        "│0.0 │•                                              X Axis│",
        "│    └─────────────────────────────────────────────────────│",
        "│  0.0                        50.0                    100.0│",
        "└──────────────────────────────────────────────────────────┘",
    ]);

    // Set expected background color
    for row in 0..30 {
        for col in 0..60 {
            expected[(col, row)].set_bg(Color::White);
        }
    }

    // Set expected colors of the first dataset
    let line1 = vec![
        (48, 5),
        (49, 5),
        (46, 6),
        (47, 6),
        (44, 7),
        (45, 7),
        (42, 8),
        (43, 8),
        (40, 9),
        (41, 9),
        (38, 10),
        (39, 10),
        (36, 11),
        (37, 11),
        (34, 12),
        (35, 12),
        (33, 13),
        (30, 14),
        (31, 14),
        (28, 15),
        (29, 15),
        (25, 16),
        (26, 16),
        (27, 16),
        (23, 17),
        (24, 17),
        (21, 18),
        (22, 18),
        (19, 19),
        (20, 19),
        (17, 20),
        (18, 20),
        (15, 21),
        (16, 21),
        (13, 22),
        (14, 22),
        (11, 23),
        (12, 23),
        (9, 24),
        (10, 24),
        (7, 25),
        (8, 25),
        (6, 26),
    ];
    let legend1 = vec![
        (49, 2),
        (50, 2),
        (51, 2),
        (52, 2),
        (53, 2),
        (54, 2),
        (55, 2),
        (56, 2),
        (57, 2),
    ];
    for (col, row) in line1 {
        expected[(col, row)].set_fg(Color::Blue);
    }
    for (col, row) in legend1 {
        expected[(col, row)].set_fg(Color::Blue);
    }

    // Set expected colors of the second dataset
    let line2 = vec![
        (8, 2),
        (9, 2),
        (10, 3),
        (11, 3),
        (12, 4),
        (13, 4),
        (14, 5),
        (15, 5),
        (16, 6),
        (17, 6),
        (18, 7),
        (19, 7),
        (20, 8),
        (21, 8),
        (22, 9),
        (23, 9),
        (24, 10),
        (25, 10),
        (26, 11),
        (27, 11),
        (28, 12),
        (29, 12),
        (30, 12),
        (31, 13),
        (32, 13),
        (33, 14),
        (34, 14),
        (35, 15),
        (36, 15),
        (37, 16),
        (38, 16),
        (39, 17),
        (40, 17),
        (41, 18),
        (42, 18),
        (43, 19),
        (44, 19),
        (45, 20),
        (46, 20),
        (47, 21),
        (48, 21),
        (49, 22),
        (50, 22),
        (51, 23),
        (52, 23),
        (53, 23),
        (54, 24),
        (55, 24),
        (56, 25),
        (57, 25),
    ];
    let legend2 = vec![
        (49, 3),
        (50, 3),
        (51, 3),
        (52, 3),
        (53, 3),
        (54, 3),
        (55, 3),
        (56, 3),
        (57, 3),
    ];
    for (col, row) in line2 {
        expected[(col, row)].set_fg(Color::Green);
    }
    for (col, row) in legend2 {
        expected[(col, row)].set_fg(Color::Green);
    }

    // Set expected colors of the x axis
    let x_axis_title = vec![(53, 26), (54, 26), (55, 26), (56, 26), (57, 26), (58, 26)];
    for (col, row) in x_axis_title {
        expected[(col, row)].set_fg(Color::Yellow);
    }
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_chart_top_line_styling_is_correct() {
    let backend = TestBackend::new(9, 5);
    let mut terminal = Terminal::new(backend).unwrap();

    let title_style = Style::default().fg(Color::Red).bg(Color::LightRed);
    let data_style = Style::default().fg(Color::Blue);

    terminal
        .draw(|f| {
            let data: [(f64, f64); 2] = [(0.0, 1.0), (1.0, 1.0)];
            let widget = Chart::new(vec![Dataset::default()
                .data(&data)
                .graph_type(ratatui::widgets::GraphType::Line)
                .style(data_style)])
            .y_axis(
                Axis::default()
                    .title(Span::styled("abc", title_style))
                    .bounds([0.0, 1.0])
                    .labels(create_labels(&["a", "b"])),
            )
            .x_axis(Axis::default().bounds([0.0, 1.0]));
            f.render_widget(widget, f.area());
        })
        .unwrap();

    let mut expected = Buffer::with_lines([
        "b│abc••••",
        " │       ",
        " │       ",
        " │       ",
        "a│       ",
    ]);
    expected.set_style(Rect::new(2, 0, 3, 1), title_style);
    expected.set_style(Rect::new(5, 0, 4, 1), data_style);
    terminal.backend().assert_buffer(&expected);
}
