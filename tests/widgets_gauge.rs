use tui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Block, Borders, Gauge, LineGauge},
    Terminal,
};

#[test]
fn widgets_gauge_renders() {
    let backend = TestBackend::new(40, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let gauge = Gauge::default()
                .block(Block::default().title("Percentage").borders(Borders::ALL))
                .gauge_style(Style::default().bg(Color::Blue).fg(Color::Red))
                .use_unicode(true)
                .percent(43);
            f.render_widget(gauge, chunks[0]);
            let gauge = Gauge::default()
                .block(Block::default().title("Ratio").borders(Borders::ALL))
                .gauge_style(Style::default().bg(Color::Blue).fg(Color::Red))
                .use_unicode(true)
                .ratio(0.511_313_934_313_1);
            f.render_widget(gauge, chunks[1]);
        })
        .unwrap();
    let mut expected = Buffer::with_lines(vec![
        "                                        ",
        "                                        ",
        "  ┌Percentage────────────────────────┐  ",
        "  │              ▋43%                │  ",
        "  └──────────────────────────────────┘  ",
        "  ┌Ratio─────────────────────────────┐  ",
        "  │               51%                │  ",
        "  └──────────────────────────────────┘  ",
        "                                        ",
        "                                        ",
    ]);

    for i in 3..17 {
        expected
            .get_mut(i, 3)
            .set_bg(Color::Red)
            .set_fg(Color::Blue);
    }
    for i in 17..37 {
        expected
            .get_mut(i, 3)
            .set_bg(Color::Blue)
            .set_fg(Color::Red);
    }

    for i in 3..20 {
        expected
            .get_mut(i, 6)
            .set_bg(Color::Red)
            .set_fg(Color::Blue);
    }
    for i in 20..37 {
        expected
            .get_mut(i, 6)
            .set_bg(Color::Blue)
            .set_fg(Color::Red);
    }

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_gauge_renders_no_unicode() {
    let backend = TestBackend::new(40, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let gauge = Gauge::default()
                .block(Block::default().title("Percentage").borders(Borders::ALL))
                .percent(43)
                .use_unicode(false);
            f.render_widget(gauge, chunks[0]);
            let gauge = Gauge::default()
                .block(Block::default().title("Ratio").borders(Borders::ALL))
                .ratio(0.211_313_934_313_1)
                .use_unicode(false);
            f.render_widget(gauge, chunks[1]);
        })
        .unwrap();
    let expected = Buffer::with_lines(vec![
        "                                        ",
        "                                        ",
        "  ┌Percentage────────────────────────┐  ",
        "  │               43%                │  ",
        "  └──────────────────────────────────┘  ",
        "  ┌Ratio─────────────────────────────┐  ",
        "  │               21%                │  ",
        "  └──────────────────────────────────┘  ",
        "                                        ",
        "                                        ",
    ]);
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_gauge_applies_styles() {
    let backend = TestBackend::new(12, 5);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let gauge = Gauge::default()
                .block(
                    Block::default()
                        .title(Span::styled("Test", Style::default().fg(Color::Red)))
                        .borders(Borders::ALL),
                )
                .gauge_style(Style::default().fg(Color::Blue).bg(Color::Red))
                .percent(43)
                .label(Span::styled(
                    "43%",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ));
            f.render_widget(gauge, f.size());
        })
        .unwrap();
    let mut expected = Buffer::with_lines(vec![
        "┌Test──────┐",
        "│          │",
        "│   43%    │",
        "│          │",
        "└──────────┘",
    ]);
    // title
    expected.set_style(Rect::new(1, 0, 4, 1), Style::default().fg(Color::Red));
    // gauge area
    expected.set_style(
        Rect::new(1, 1, 10, 3),
        Style::default().fg(Color::Blue).bg(Color::Red),
    );
    // filled area
    for y in 1..4 {
        expected.set_style(
            Rect::new(1, y, 4, 1),
            // filled style is invert of gauge_style
            Style::default().fg(Color::Red).bg(Color::Blue),
        );
    }
    // label (foreground and modifier from label style)
    expected.set_style(
        Rect::new(4, 2, 1, 1),
        Style::default()
            .fg(Color::Green)
            // "4" is in the filled area so background is gauge_style foreground
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD),
    );
    expected.set_style(
        Rect::new(5, 2, 2, 1),
        Style::default()
            .fg(Color::Green)
            // "3%" is not in the filled area so background is gauge_style background
            .bg(Color::Red)
            .add_modifier(Modifier::BOLD),
    );
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_gauge_supports_large_labels() {
    let backend = TestBackend::new(10, 1);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let gauge = Gauge::default()
                .percent(43)
                .label("43333333333333333333333333333%");
            f.render_widget(gauge, f.size());
        })
        .unwrap();
    let expected = Buffer::with_lines(vec!["4333333333"]);
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_line_gauge_renders() {
    let backend = TestBackend::new(20, 4);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let gauge = LineGauge::default()
                .gauge_style(Style::default().fg(Color::Green).bg(Color::White))
                .ratio(0.43);
            f.render_widget(
                gauge,
                Rect {
                    x: 0,
                    y: 0,
                    width: 20,
                    height: 1,
                },
            );
            let gauge = LineGauge::default()
                .block(Block::default().title("Gauge 2").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Green))
                .line_set(symbols::line::THICK)
                .ratio(0.211_313_934_313_1);
            f.render_widget(
                gauge,
                Rect {
                    x: 0,
                    y: 1,
                    width: 20,
                    height: 3,
                },
            );
        })
        .unwrap();
    let mut expected = Buffer::with_lines(vec![
        "43% ────────────────",
        "┌Gauge 2───────────┐",
        "│21% ━━━━━━━━━━━━━━│",
        "└──────────────────┘",
    ]);
    for col in 4..10 {
        expected.get_mut(col, 0).set_fg(Color::Green);
    }
    for col in 10..20 {
        expected.get_mut(col, 0).set_fg(Color::White);
    }
    for col in 5..7 {
        expected.get_mut(col, 2).set_fg(Color::Green);
    }
    terminal.backend().assert_buffer(&expected);
}
