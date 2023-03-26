use ratatui::{backend::TestBackend, layout::Rect, widgets::Paragraph, Terminal};

#[test]
fn draw_returns_the_completed_frame() {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let paragraph = Paragraph::new("Test");
    terminal.render_widget(paragraph, terminal.viewport_area());
    let frame = terminal.flush().unwrap();
    assert_eq!(frame.buffer.get(0, 0).symbol, "T");
    assert_eq!(frame.area, Rect::new(0, 0, 10, 10));
}
