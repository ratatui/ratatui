use tui::layout::Alignment;
use tui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType::Line},
    Terminal,
};

fn create_labels<'a>(labels: &'a [&'a str]) -> Vec<Span<'a>> {
    labels.iter().map(|l| Span::from(*l)).collect()
}

fn axis_test_case<S>(width: u16, height: u16, x_axis: Axis, y_axis: Axis, lines: Vec<S>)
where
    S: AsRef<str>,
{
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let chart = Chart::new(vec![]).x_axis(x_axis).y_axis(y_axis);
            f.render_widget(chart, f.size());
        })
        .unwrap();
    let expected = Buffer::with_lines(lines);
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_chart_can_render_on_small_areas() {
    let test_case = |width, height| {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                let datasets = vec![Dataset::default()
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Magenta))
                    .data(&[(0.0, 0.0)])];
                let chart = Chart::new(datasets)
                    .block(Block::default().title("Plot").borders(Borders::ALL))
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
                f.render_widget(chart, f.size());
            })
            .unwrap();
    };
    test_case(0, 0);
    test_case(0, 1);
    test_case(1, 0);
    test_case(1, 1);
    test_case(2, 2);
}

#[test]
fn widgets_chart_handles_long_labels() {
    let test_case = |x_labels, y_labels, x_alignment, lines| {
        let mut x_axis = Axis::default().bounds([0.0, 1.0]);
        if let Some((left_label, right_label)) = x_labels {
            x_axis = x_axis
                .labels(vec![Span::from(left_label), Span::from(right_label)])
                .labels_alignment(x_alignment);
        }

        let mut y_axis = Axis::default().bounds([0.0, 1.0]);
        if let Some((left_label, right_label)) = y_labels {
            y_axis = y_axis.labels(vec![Span::from(left_label), Span::from(right_label)]);
        }

        axis_test_case(10, 5, x_axis, y_axis, lines);
    };

    test_case(
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
    );
    test_case(
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
    );
    test_case(
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
    );
    test_case(
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
    );
    test_case(
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
    );
    test_case(
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
    );
    test_case(
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
    );
}

#[test]
fn widgets_chart_handles_x_axis_labels_alignments() {
    let test_case = |y_alignment, lines| {
        let x_axis = Axis::default()
            .labels(vec![Span::from("AAAA"), Span::from("B"), Span::from("C")])
            .labels_alignment(y_alignment);

        let y_axis = Axis::default();

        axis_test_case(10, 5, x_axis, y_axis, lines);
    };

    test_case(
        Alignment::Left,
        vec![
            "          ",
            "          ",
            "          ",
            "   ───────",
            "AAA   B  C",
        ],
    );
    test_case(
        Alignment::Center,
        vec![
            "          ",
            "          ",
            "          ",
            "  ────────",
            "AAAA B   C",
        ],
    );
    test_case(
        Alignment::Right,
        vec![
            "          ",
            "          ",
            "          ",
            "──────────",
            "AAA  B   C",
        ],
    );
}

#[test]
fn widgets_chart_handles_y_axis_labels_alignments() {
    let test_case = |y_alignment, lines| {
        let x_axis = Axis::default().labels(create_labels(&["AAAAA", "B"]));

        let y_axis = Axis::default()
            .labels(create_labels(&["C", "D"]))
            .labels_alignment(y_alignment);

        axis_test_case(20, 5, x_axis, y_axis, lines);
    };
    test_case(
        Alignment::Left,
        vec![
            "D   │               ",
            "    │               ",
            "C   │               ",
            "    └───────────────",
            "AAAAA              B",
        ],
    );
    test_case(
        Alignment::Center,
        vec![
            "  D │               ",
            "    │               ",
            "  C │               ",
            "    └───────────────",
            "AAAAA              B",
        ],
    );
    test_case(
        Alignment::Right,
        vec![
            "   D│               ",
            "    │               ",
            "   C│               ",
            "    └───────────────",
            "AAAAA              B",
        ],
    );
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
                .block(Block::default().title("Plot").borders(Borders::ALL))
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
                .block(Block::default().title("Plot").borders(Borders::ALL))
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
                .block(
                    Block::default()
                        .title("Empty Dataset With Line")
                        .borders(Borders::ALL),
                )
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
                .block(Block::default().title("Chart Test").borders(Borders::ALL))
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
    let mut expected = Buffer::with_lines(vec![
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

    // Set expected backgound color
    for row in 0..30 {
        for col in 0..60 {
            expected.get_mut(col, row).set_bg(Color::White);
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
        expected.get_mut(col, row).set_fg(Color::Blue);
    }
    for (col, row) in legend1 {
        expected.get_mut(col, row).set_fg(Color::Blue);
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
        expected.get_mut(col, row).set_fg(Color::Green);
    }
    for (col, row) in legend2 {
        expected.get_mut(col, row).set_fg(Color::Green);
    }

    // Set expected colors of the x axis
    let x_axis_title = vec![(53, 26), (54, 26), (55, 26), (56, 26), (57, 26), (58, 26)];
    for (col, row) in x_axis_title {
        expected.get_mut(col, row).set_fg(Color::Yellow);
    }

    terminal.backend().assert_buffer(&expected);
}
