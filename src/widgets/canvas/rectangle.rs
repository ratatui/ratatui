use crate::{
    style::Color,
    widgets::canvas::{Line, Painter, Shape},
};

/// A rectangle to draw on a [`Canvas`](crate::widgets::canvas::Canvas)
///
/// Sizes used here are **not** in terminal cell. This is much more similar to the
/// mathematic coordinate system.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rectangle {
    /// The `x` position of the rectangle.
    ///
    /// The rectangle is positioned from its bottom left corner.
    pub x: f64,
    /// The `y` position of the rectangle.
    ///
    /// The rectangle is positioned from its bottom left corner.
    pub y: f64,
    /// The width of the rectangle.
    pub width: f64,
    /// The height of the rectangle.
    pub height: f64,
    /// The color of the rectangle.
    pub color: Color,
}

impl Shape for Rectangle {
    fn draw(&self, painter: &mut Painter) {
        let lines: [Line; 4] = [
            Line {
                x1: self.x,
                y1: self.y,
                x2: self.x,
                y2: self.y + self.height,
                color: self.color,
            },
            Line {
                x1: self.x,
                y1: self.y + self.height,
                x2: self.x + self.width,
                y2: self.y + self.height,
                color: self.color,
            },
            Line {
                x1: self.x + self.width,
                y1: self.y,
                x2: self.x + self.width,
                y2: self.y + self.height,
                color: self.color,
            },
            Line {
                x1: self.x,
                y1: self.y,
                x2: self.x + self.width,
                y2: self.y,
                color: self.color,
            },
        ];
        for line in &lines {
            line.draw(painter);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        buffer::Buffer,
        layout::{Margin, Rect},
        style::{Style, Stylize},
        symbols::Marker,
        widgets::{canvas::Canvas, Widget},
    };

    #[test]
    fn draw_block_lines() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
        let canvas = Canvas::default()
            .marker(Marker::Block)
            .x_bounds([0.0, 10.0])
            .y_bounds([0.0, 10.0])
            .paint(|context| {
                context.draw(&Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 10.0,
                    height: 10.0,
                    color: Color::Red,
                });
            });
        canvas.render(buffer.area, &mut buffer);
        let mut expected = Buffer::with_lines([
            "██████████",
            "█        █",
            "█        █",
            "█        █",
            "█        █",
            "█        █",
            "█        █",
            "█        █",
            "█        █",
            "██████████",
        ]);
        expected.set_style(buffer.area, Style::new().red());
        expected.set_style(buffer.area.inner(Margin::new(1, 1)), Style::reset());
        assert_eq!(buffer, expected);
    }

    #[test]
    fn draw_half_block_lines() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
        let canvas = Canvas::default()
            .marker(Marker::HalfBlock)
            .x_bounds([0.0, 10.0])
            .y_bounds([0.0, 10.0])
            .paint(|context| {
                context.draw(&Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 10.0,
                    height: 10.0,
                    color: Color::Red,
                });
            });
        canvas.render(buffer.area, &mut buffer);
        let mut expected = Buffer::with_lines([
            "█▀▀▀▀▀▀▀▀█",
            "█        █",
            "█        █",
            "█        █",
            "█        █",
            "█        █",
            "█        █",
            "█        █",
            "█        █",
            "█▄▄▄▄▄▄▄▄█",
        ]);
        expected.set_style(buffer.area, Style::new().red().on_red());
        expected.set_style(buffer.area.inner(Margin::new(1, 0)), Style::reset().red());
        expected.set_style(buffer.area.inner(Margin::new(1, 1)), Style::reset());
        assert_eq!(buffer, expected);
    }

    #[test]
    fn draw_braille_lines() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
        let canvas = Canvas::default()
            .marker(Marker::Braille)
            .x_bounds([0.0, 10.0])
            .y_bounds([0.0, 10.0])
            .paint(|context| {
                // a rectangle that will draw the outside part of the braille
                context.draw(&Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 10.0,
                    height: 10.0,
                    color: Color::Red,
                });
                // a rectangle that will draw the inside part of the braille
                context.draw(&Rectangle {
                    x: 2.0,
                    y: 1.75,
                    width: 6.5,
                    height: 6.5,
                    color: Color::Green,
                });
            });
        canvas.render(buffer.area, &mut buffer);
        let mut expected = Buffer::with_lines([
            "⡏⠉⠉⠉⠉⠉⠉⠉⠉⢹",
            "⡇⢠⠤⠤⠤⠤⠤⠤⡄⢸",
            "⡇⢸      ⡇⢸",
            "⡇⢸      ⡇⢸",
            "⡇⢸      ⡇⢸",
            "⡇⢸      ⡇⢸",
            "⡇⢸      ⡇⢸",
            "⡇⢸      ⡇⢸",
            "⡇⠈⠉⠉⠉⠉⠉⠉⠁⢸",
            "⣇⣀⣀⣀⣀⣀⣀⣀⣀⣸",
        ]);
        expected.set_style(buffer.area, Style::new().red());
        expected.set_style(buffer.area.inner(Margin::new(1, 1)), Style::new().green());
        expected.set_style(buffer.area.inner(Margin::new(2, 2)), Style::reset());
        assert_eq!(buffer, expected);
    }
}
