use tui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, List, ListState, Text},
    Terminal,
};

#[test]
fn widgets_list_should_highlight_the_selected_item() {
    let backend = TestBackend::new(10, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut state = ListState::default();
    state.select(Some(1));
    terminal
        .draw(|mut f| {
            let size = f.size();
            let items = vec![
                Text::raw("Item 1"),
                Text::raw("Item 2"),
                Text::raw("Item 3"),
            ];
            let list = List::new(items.into_iter())
                .highlight_style(Style::default().bg(Color::Yellow))
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, size, &mut state);
        })
        .unwrap();
    let mut expected = Buffer::with_lines(vec!["   Item 1 ", ">> Item 2 ", "   Item 3 "]);
    for x in 0..9 {
        expected.get_mut(x, 1).set_bg(Color::Yellow);
    }
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn widgets_list_should_truncate_items() {
    let backend = TestBackend::new(10, 2);
    let mut terminal = Terminal::new(backend).unwrap();

    struct TruncateTestCase<'a> {
        selected: Option<usize>,
        items: Vec<Text<'a>>,
        expected: Buffer,
    }

    let cases = vec![
        // An item is selected
        TruncateTestCase {
            selected: Some(0),
            items: vec![Text::raw("A very long line"), Text::raw("A very long line")],
            expected: Buffer::with_lines(vec![
                format!(">> A ve{}  ", symbols::line::VERTICAL),
                format!("   A ve{}  ", symbols::line::VERTICAL),
            ]),
        },
        // No item is selected
        TruncateTestCase {
            selected: None,
            items: vec![Text::raw("A very long line"), Text::raw("A very long line")],
            expected: Buffer::with_lines(vec![
                format!("A very {}  ", symbols::line::VERTICAL),
                format!("A very {}  ", symbols::line::VERTICAL),
            ]),
        },
    ];
    for mut case in cases {
        let mut state = ListState::default();
        state.select(case.selected);
        let items = case.items.drain(..);
        terminal
            .draw(|mut f| {
                let list = List::new(items)
                    .block(Block::default().borders(Borders::RIGHT))
                    .highlight_symbol(">> ");
                f.render_stateful_widget(list, Rect::new(0, 0, 8, 2), &mut state);
            })
            .unwrap();
        terminal.backend().assert_buffer(&case.expected);
    }
}
