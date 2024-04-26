use ratatui::{prelude::*, widgets::widget_ext::WidgetExt};

fn main() {
    let greeting = Text::from(vec![
        Line::styled("Hello", Color::Blue),
        Line::styled("World ", Color::Green),
    ]);
    println!("{}", greeting.to_ansi_string(5, 2));
}
