//! A module that provides an extension trait for widgets that provides methods that are useful for
//! debugging.

use super::ansi_string_buffer::AnsiStringBuffer;
use crate::prelude::*;

/// An extension trait for widgets that provides methods that are useful for debugging.
#[stability::unstable(
    feature = "widget-ext",
    issue = "https://github.com/ratatui-org/ratatui/issues/1045"
)]
pub trait WidgetExt {
    /// Returns a string representation of the widget with ANSI escape sequences for the terminal.
    fn to_ansi_string(&self, width: u16, height: u16) -> String;
}

impl<W: WidgetRef> WidgetExt for W {
    fn to_ansi_string(&self, width: u16, height: u16) -> String {
        let mut buf = AnsiStringBuffer::new(width, height);
        buf.render_ref(self);
        buf.to_string()
    }
}

#[cfg(test)]
mod widget_ext_tests {
    use super::*;

    struct Greeting;

    impl WidgetRef for Greeting {
        fn render_ref(&self, area: Rect, buf: &mut Buffer) {
            let text = Text::from(vec![
                Line::styled("Hello", Color::Blue),
                Line::styled("World ", Color::Green),
            ]);
            text.render(area, buf);
        }
    }

    #[test]
    fn widget_ext_to_ansi_string() {
        let ansi_string = Greeting.to_ansi_string(5, 2);
        println!("{ansi_string}");
        assert_eq!(
            ansi_string,
            "\u{1b}[34;49mHello\n\u{1b}[32;49mWorld\u{1b}[0m"
        );
    }
}
