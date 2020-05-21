use tui::{backend::TestBackend, buffer::Buffer, layout::Rect, symbols, widgets::Tabs, Terminal};

#[test]
fn tabs_should_not_panic_on_narrow_areas() {
    let backend = TestBackend::new(1, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|mut f| {
            let tabs = Tabs::default().titles(&["Tab1", "Tab2"]);
            f.render_widget(
                tabs,
                Rect {
                    x: 0,
                    y: 0,
                    width: 1,
                    height: 1,
                },
            );
        })
        .unwrap();
    let expected = Buffer::with_lines(vec![" "]);
    terminal.backend().assert_buffer(&expected);
}

#[test]
fn tabs_should_truncate_the_last_item() {
    let backend = TestBackend::new(10, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|mut f| {
            let tabs = Tabs::default().titles(&["Tab1", "Tab2"]);
            f.render_widget(
                tabs,
                Rect {
                    x: 0,
                    y: 0,
                    width: 9,
                    height: 1,
                },
            );
        })
        .unwrap();
    let expected = Buffer::with_lines(vec![format!(" Tab1 {} Ta", symbols::line::VERTICAL)]);
    terminal.backend().assert_buffer(&expected);
}
