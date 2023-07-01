use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

#[test]
fn paragraph_block_styles() {
    let backend = TestBackend::new(10, 1);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let paragraph = Paragraph::new("Text".cyan());
            f.render_widget(
                paragraph,
                Rect {
                    x: 0,
                    y: 0,
                    width: 10,
                    height: 1,
                },
            );
        })
        .unwrap();

    let mut expected = Buffer::with_lines(vec!["Text      "]);
    for x in 0..4 {
        expected.get_mut(x, 0).set_fg(Color::Cyan);
    }

    terminal.backend().assert_buffer(&expected);
}

#[test]
fn block_styles() {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let block = Block::default()
                .title("Title".light_blue())
                .on_cyan()
                .cyan()
                .borders(Borders::ALL);
            f.render_widget(
                block,
                Rect {
                    x: 0,
                    y: 0,
                    width: 8,
                    height: 8,
                },
            );
        })
        .unwrap();

    let mut expected = Buffer::with_lines(vec![
        "┌Title─┐  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "│      │  ",
        "└──────┘  ",
        "          ",
        "          ",
    ]);
    for x in 0..8 {
        for y in 0..8 {
            expected
                .get_mut(x, y)
                .set_fg(Color::Cyan)
                .set_bg(Color::Cyan);
        }
    }

    for x in 1..=5 {
        expected.get_mut(x, 0).set_fg(Color::LightBlue);
    }

    terminal.backend().assert_buffer(&expected);
}
