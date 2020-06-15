use tui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Gauge},
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
                .percent(43);
            f.render_widget(gauge, chunks[0]);
            let gauge = Gauge::default()
                .block(Block::default().title("Ratio").borders(Borders::ALL))
                .ratio(0.211_313_934_313_1);
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
