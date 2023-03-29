use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::widgets::{BarChart, Block, Borders};
use ratatui::Terminal;

#[test]
fn widgets_barchart_not_full_below_max_value() {
    let test_case = |expected| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        let barchart = BarChart::default()
            .block(Block::default().borders(Borders::ALL))
            .data(&[("empty", 0), ("half", 50), ("almost", 99), ("full", 100)])
            .max(100)
            .bar_width(7)
            .bar_gap(0);
        terminal.render_widget_on_viewport(barchart, 0);
        terminal.flush().unwrap();
        terminal.backend().assert_buffer(&expected);
    };

    // check that bars fill up correctly up to max value
    test_case(Buffer::with_lines(vec![
        "┌────────────────────────────┐",
        "│              ▇▇▇▇▇▇▇███████│",
        "│              ██████████████│",
        "│              ██████████████│",
        "│       ▄▄▄▄▄▄▄██████████████│",
        "│       █████████████████████│",
        "│       █████████████████████│",
        "│       ██50█████99█████100██│",
        "│empty  half   almost full   │",
        "└────────────────────────────┘",
    ]));
}
