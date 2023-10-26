use std::error::Error;

use ratatui::{
    assert_buffer_eq,
    backend::{Backend, TestBackend},
    layout::Rect,
    prelude::Buffer,
    text::Line,
    widgets::{Paragraph, Widget},
    Terminal, TerminalOptions, Viewport,
};

#[test]
fn terminal_buffer_size_should_be_limited() {
    let backend = TestBackend::new(400, 400);
    let terminal = Terminal::new(backend).unwrap();
    let size = terminal.backend().size().unwrap();
    assert_eq!(size.width, 255);
    assert_eq!(size.height, 255);
}

#[test]
fn swap_buffer_clears_prev_buffer() {
    let backend = TestBackend::new(100, 50);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .current_buffer_mut()
        .set_string(0, 0, "Hello", ratatui::style::Style::reset());
    assert_eq!(terminal.current_buffer_mut().content()[0].symbol, "H");
    terminal.swap_buffers();
    assert_eq!(terminal.current_buffer_mut().content()[0].symbol, " ");
}

#[test]
fn terminal_draw_returns_the_completed_frame() -> Result<(), Box<dyn Error>> {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend)?;
    let frame = terminal.draw(|f| {
        let paragraph = Paragraph::new("Test");
        f.render_widget(paragraph, f.size());
    })?;
    assert_eq!(frame.buffer.get(0, 0).symbol, "T");
    assert_eq!(frame.area, Rect::new(0, 0, 10, 10));
    terminal.backend_mut().resize(8, 8);
    let frame = terminal.draw(|f| {
        let paragraph = Paragraph::new("test");
        f.render_widget(paragraph, f.size());
    })?;
    assert_eq!(frame.buffer.get(0, 0).symbol, "t");
    assert_eq!(frame.area, Rect::new(0, 0, 8, 8));
    Ok(())
}

#[test]
fn terminal_insert_before_moves_viewport() -> Result<(), Box<dyn Error>> {
    let backend = TestBackend::new(8, 5);
    let mut terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(2),
        },
    )?;

    terminal.insert_before(2, |buf| {
        let lines: Vec<Line> = (0..2).map(|i| Line::from(format!("Line {i}"))).collect();
        Paragraph::new(lines).render(buf.area, buf);
    })?;

    terminal.draw(|f| {
        let paragraph = Paragraph::new("Test");
        f.render_widget(paragraph, f.size());
    })?;

    assert_buffer_eq!(
        terminal.backend().buffer().clone(),
        Buffer::with_lines(vec![
            "Line 0  ", "Line 1  ", "Test    ", "        ", "        ",
        ])
    );

    Ok(())
}

#[test]
fn terminal_insert_before_scrolls_on_large_input() -> Result<(), Box<dyn Error>> {
    let backend = TestBackend::new(8, 5);
    let mut terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(2),
        },
    )?;

    terminal.insert_before(10, |buf| {
        let lines: Vec<Line> = (0..10).map(|i| Line::from(format!("Line {i}"))).collect();
        Paragraph::new(lines).render(buf.area, buf);
    })?;

    terminal.draw(|f| {
        let paragraph = Paragraph::new("Test");
        f.render_widget(paragraph, f.size());
    })?;

    assert_buffer_eq!(
        terminal.backend().buffer().clone(),
        Buffer::with_lines(vec![
            "Line 7  ", "Line 8  ", "Line 9  ", "Test    ", "        ",
        ])
    );

    print!("{:?}", terminal.backend().buffer());

    Ok(())
}

#[test]
fn terminal_insert_before_scrolls_on_many_inserts() -> Result<(), Box<dyn Error>> {
    let backend = TestBackend::new(8, 5);
    let mut terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(2),
        },
    )?;

    for i in 0..4 {
        terminal.insert_before(1, |buf| {
            Paragraph::new(Line::raw(format!("Line {i}"))).render(buf.area, buf);
        })?;
    }

    terminal.draw(|f| {
        let paragraph = Paragraph::new("Test");
        f.render_widget(paragraph, f.size());
    })?;

    assert_buffer_eq!(
        terminal.backend().buffer().clone(),
        Buffer::with_lines(vec![
            "Line 1  ", "Line 2  ", "Line 3  ", "Test    ", "        ",
        ])
    );

    Ok(())
}
