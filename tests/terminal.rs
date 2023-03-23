use ratatui::{
    backend::TestBackend,
    buffer::Cell,
    layout::Rect,
    text::Text,
    widgets::{Block, Fill, Paragraph},
    Terminal,
};

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

#[test]
fn diffing_emty_empty() {
    let area = Rect::new(0, 0, 10, 10);
    let backend = TestBackend::new(area.width, area.height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| frame.render_widget(Fill::new(Cell::default().set_symbol(" ")), area))
        .unwrap();
    terminal
        .draw(|frame| frame.render_widget(Fill::new(Cell::default().set_symbol(" ")), area))
        .unwrap();
    assert_diff_with_prev(terminal.diff(), vec![]);
}

#[test]
fn diffing_filled_filled() {
    let area = Rect::new(0, 0, 10, 10);
    let backend = TestBackend::new(area.width, area.height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| frame.render_widget(Fill::new(Cell::default().set_symbol("a")), area))
        .unwrap();
    terminal
        .draw(|frame| frame.render_widget(Fill::new(Cell::default().set_symbol("a")), area))
        .unwrap();

    assert_diff_with_prev(terminal.diff(), vec![]);
}

#[test]
fn diffing_empty_filled() {
    let area = Rect::new(0, 0, 10, 10);
    let backend = TestBackend::new(area.width, area.height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| frame.render_widget(Fill::new(Cell::default().set_symbol("a")), area))
        .unwrap();
    assert_eq!(terminal.diff().len(), 10 * 10);
}

#[test]
fn diffing_single_width() {
    let area = Rect::new(0, 0, 10, 10);
    let backend = TestBackend::new(area.width, area.height);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| frame.render_widget(Block::default().title("TITLE"), area))
        .unwrap();
    terminal
        .draw(|frame| frame.render_widget(Block::default().title("Title"), area))
        .unwrap();

    assert_diff_with_prev(
        terminal.diff(),
        vec![
            (1, 0, &cell("I")),
            (2, 0, &cell("T")),
            (3, 0, &cell("L")),
            (4, 0, &cell("E")),
        ],
    );
}

#[rustfmt::skip]
#[test]
fn diffing_multi_width() {
    let area = Rect::new(0, 0, 10, 1);
    let backend = TestBackend::new(area.width, area.height);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|frame| frame.render_widget(Block::default().title("称号"), area)).unwrap();
    terminal.draw(|frame| frame.render_widget(Block::default().title("Title"), area)).unwrap();

    assert_diff_with_prev(
        terminal.diff(),
        vec![
            (0, 0, &cell("称")),
            // Skipped "i"
            (2, 0, &cell("号")),
            // Skipped "l"
            (4, 0, &cell(" ")),
        ]
    );
}

#[test]
fn diffing_multi_width_offset() {
    let area = Rect::new(0, 0, 10, 1);
    let backend = TestBackend::new(area.width, area.height);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| frame.render_widget(Paragraph::new(Text::raw("┌─称号─┐")), area))
        .unwrap();
    terminal
        .draw(|frame| frame.render_widget(Paragraph::new(Text::raw("┌称号──┐")), area))
        .unwrap();

    assert_diff_with_prev(
        terminal.diff(),
        vec![(1, 0, &cell("─")), (2, 0, &cell("称")), (4, 0, &cell("号"))],
    );
}

fn cell(s: &str) -> Cell {
    let mut cell = Cell::default();
    cell.set_symbol(s);
    cell
}

fn assert_diff_with_prev(diff: Vec<(u16, u16, &Cell)>, expected: Vec<(u16, u16, &Cell)>) {
    if diff != expected {
        let output: Vec<String> = diff
            .iter()
            .map(|(x, y, cell)| format!("{} at ({x},{y})", cell.symbol))
            .collect();
        panic!("{output:#?}");
    }
}
