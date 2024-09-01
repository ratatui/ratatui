use ratatui::{prelude::*, widgets::ansi_string_buffer::AnsiStringBuffer};

fn main() {
    let mut buf = AnsiStringBuffer::new(5, 2);
    buf.render_ref(&Line::styled("Hello", Color::Blue), Rect::new(0, 0, 5, 1));
    buf.render_ref(&Line::styled("World", Color::Green), Rect::new(0, 1, 5, 1));
    println!("{buf}");
}
