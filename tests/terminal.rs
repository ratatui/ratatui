use ratatui::{backend::TestBackend, layout::Rect, widgets::Paragraph, Terminal};

#[test]
fn draw_returns_the_completed_frame() {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let frame = terminal
        .draw(|f| {
            let paragraph = Paragraph::new("Test");
            f.render_widget(paragraph, f.viewport_area());
        })
        .unwrap();
    assert_eq!(frame.buffer.get(0, 0).symbol, "T");
    assert_eq!(frame.area, Rect::new(0, 0, 10, 10));
}
