use tui::{
    backend::TestBackend,
    buffer::Buffer,
    style::{Color, Style},
    widgets::{List, ListState, Text},
    Terminal,
};

#[test]
fn it_should_highlight_the_selected_item() {
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
    assert_eq!(*terminal.backend().buffer(), expected);
}
