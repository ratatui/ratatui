//! The [`Fill`] widget paints every cell in its area with a single symbol and style.
use alloc::borrow::Cow;

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::style::{Style, Styled};
use ratatui_core::widgets::Widget;

/// A widget that fills its render area with a single repeated symbol and style.
///
/// [`Fill`] is a small building block for painting solid blocks of one symbol — backgrounds,
/// separators, scrollbar tracks, custom borders, etc. — without writing the nested loop
/// yourself. It composes naturally with the [`Stylize`] trait so the typical call site is
/// a one-liner.
///
/// # Examples
///
/// ```
/// use ratatui::layout::Rect;
/// use ratatui::style::Stylize;
/// use ratatui::widgets::{Fill, Widget};
///
/// # let mut buf = ratatui::buffer::Buffer::empty(Rect::new(0, 0, 10, 5));
/// let fill = Fill::new("X").blue().bold();
/// fill.render(Rect::new(0, 0, 10, 3), &mut buf);
/// ```
///
/// This renders as:
///
/// ```plain
/// XXXXXXXXXX
/// XXXXXXXXXX
/// XXXXXXXXXX
/// ```
///
/// [`Fill`] accepts anything that converts into a [`Cow<str>`], so both string literals and
/// owned [`String`](alloc::string::String)s work:
///
/// ```
/// use ratatui::widgets::Fill;
///
/// let _ = Fill::new("•");
/// let _ = Fill::new(String::from("•"));
/// ```
///
/// Cells outside the buffer are silently clipped, mirroring the behavior of other widgets
/// such as [`Clear`](crate::clear::Clear).
///
/// [`Stylize`]: ratatui_core::style::Stylize
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Fill<'a> {
    symbol: Cow<'a, str>,
    style: Style,
}

impl<'a> Fill<'a> {
    /// Create a new [`Fill`] widget that paints `symbol` into every cell of its render area.
    ///
    /// The style defaults to [`Style::default`]; use the [`Stylize`] shorthands or
    /// [`Fill::style`] to customize it.
    ///
    /// [`Stylize`]: ratatui_core::style::Stylize
    pub fn new<S: Into<Cow<'a, str>>>(symbol: S) -> Self {
        Self {
            symbol: symbol.into(),
            style: Style::default(),
        }
    }

    /// Set the style used to paint each cell.
    ///
    /// `style` accepts any value convertible into a [`Style`] (e.g. [`Style`],
    /// [`Color`](ratatui_core::style::Color), or your own type implementing
    /// [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Set the symbol painted into each cell.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn symbol<S: Into<Cow<'a, str>>>(mut self, symbol: S) -> Self {
        self.symbol = symbol.into();
        self
    }
}

impl Widget for Fill<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &Fill<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(*buf.area());
        if area.is_empty() {
            return;
        }
        for position in area.positions() {
            buf[position].set_symbol(&self.symbol).set_style(self.style);
        }
    }
}

impl Styled for Fill<'_> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    use ratatui_core::buffer::Buffer;
    use ratatui_core::layout::Rect;
    use ratatui_core::style::{Color, Style, Stylize};
    use ratatui_core::widgets::Widget;

    use super::*;

    #[test]
    #[rustfmt::skip]
    fn fills_area_with_symbol_and_style() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 5, 3));
        Fill::new(".")
            .style(Style::new().fg(Color::Red))
            .render(Rect::new(1, 1, 3, 1), &mut buffer);

        let mut expected = Buffer::with_lines([
            "     ",
            " ... ",
            "     ",
        ]);
        for x in 1..=3 {
            expected[(x, 1)].set_style(Style::new().fg(Color::Red));
        }
        assert_eq!(buffer, expected);
    }

    #[test]
    fn clips_area_to_buffer() {
        // Render area extends past the right and bottom edges of the buffer.
        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 2));
        Fill::new("x").render(Rect::new(1, 1, 100, 100), &mut buffer);
        assert_eq!(buffer, Buffer::with_lines(["   ", " xx"]));
    }

    #[test]
    fn render_fully_out_of_bounds_is_noop() {
        let mut buffer = Buffer::with_lines(["xxxxx"; 3]);
        Fill::new(".").render(Rect::new(100, 100, 5, 5), &mut buffer);
        assert_eq!(buffer, Buffer::with_lines(["xxxxx"; 3]));
    }

    #[test]
    fn renders_with_offset_buffer_area() {
        // Buffers can have a non-zero origin; ensure intersection logic still works.
        let mut buffer = Buffer::empty(Rect::new(2, 2, 2, 2));
        Fill::new("#").render(Rect::new(0, 0, 4, 4), &mut buffer);
        let mut expected = Buffer::empty(Rect::new(2, 2, 2, 2));
        for y in 2..4 {
            for x in 2..4 {
                expected[(x, y)].set_symbol("#");
            }
        }
        assert_eq!(buffer, expected);
    }

    #[test]
    fn stylize_shorthand_works() {
        // The whole point of choosing the widget shape: composition with Stylize.
        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 1));
        Fill::new("*")
            .blue()
            .bold()
            .render(Rect::new(0, 0, 2, 1), &mut buffer);
        let mut expected = Buffer::with_lines(["**"]);
        for x in 0..2 {
            expected[(x, 0)].set_style(Style::new().fg(Color::Blue).bold());
        }
        assert_eq!(buffer, expected);
    }

    #[test]
    fn accepts_owned_string_symbol() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 1));
        Fill::new(String::from("•")).render(Rect::new(0, 0, 2, 1), &mut buffer);
        assert_eq!(buffer, Buffer::with_lines(["••"]));
    }

    #[test]
    fn symbol_setter_replaces_symbol() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 1));
        Fill::new("a")
            .symbol("b")
            .render(Rect::new(0, 0, 2, 1), &mut buffer);
        assert_eq!(buffer, Buffer::with_lines(["bb"]));
    }
}
