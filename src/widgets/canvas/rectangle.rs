use crate::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

/// A rectangle to draw on a [`Canvas`](super::Canvas)
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
    /// Whether the inside of the rectangle should be filled.
    pub fill: bool,
}

impl Shape for Rectangle {
    fn draw(&self, painter: &mut Painter) {
        fn sorted_pair<T: PartialOrd>(a: T, b: T) -> (T, T) {
            if a > b {
                (b, a)
            } else {
                (a, b)
            }
        }
        let (x1, x2) = sorted_pair(self.x, self.x + self.width);
        let (y1, y2) = sorted_pair(self.y, self.y + self.height);

        let x_steps = painter.step_points_x(x1..=x2);
        let y_steps = painter.step_points_y(y1..=y2);
        for (x_index, x) in x_steps.enumerate() {
            let x_is_first = x_index == 0;
            for (y_index, y) in y_steps.clone().enumerate() {
                let y_is_first = y_index == 0;

                if self.fill || x_is_first || y_is_first || x.is_last || y.is_last {
                    painter.paint(x.grid, y.grid, self.color);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_buffer_eq, prelude::*, widgets::canvas::Canvas};

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
                    fill: false,
                });
            });
        canvas.render(buffer.area, &mut buffer);
        let mut expected = Buffer::with_lines(vec![
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
        expected.set_style(buffer.area.inner(&Margin::new(1, 1)), Style::reset());
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn draw_block_filled() {
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
                    fill: true,
                });
            });
        canvas.render(buffer.area, &mut buffer);
        let mut expected = Buffer::with_lines(vec![
            "██████████",
            "██████████",
            "██████████",
            "██████████",
            "██████████",
            "██████████",
            "██████████",
            "██████████",
            "██████████",
            "██████████",
        ]);
        expected.set_style(buffer.area, Style::new().red());
        assert_buffer_eq!(buffer, expected);
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
                    fill: false,
                });
            });
        canvas.render(buffer.area, &mut buffer);
        let mut expected = Buffer::with_lines(vec![
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
        expected.set_style(buffer.area.inner(&Margin::new(1, 0)), Style::reset().red());
        expected.set_style(buffer.area.inner(&Margin::new(1, 1)), Style::reset());
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn draw_half_block_fill() {
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
                    fill: true,
                });
            });
        canvas.render(buffer.area, &mut buffer);
        let mut expected = Buffer::with_lines(vec![
            "██████████",
            "██████████",
            "██████████",
            "██████████",
            "██████████",
            "██████████",
            "██████████",
            "██████████",
            "██████████",
            "██████████",
        ]);
        expected.set_style(buffer.area, Style::new().red().on_red());
        expected.set_style(buffer.area.inner(&Margin::new(1, 0)), Style::new().red());
        assert_buffer_eq!(buffer, expected);
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
                    fill: false,
                });
                // a rectangle that will draw the inside part of the braille
                context.draw(&Rectangle {
                    x: 2.0,
                    y: 1.75,
                    width: 6.5,
                    height: 6.5,
                    color: Color::Green,
                    fill: false,
                });
            });
        canvas.render(buffer.area, &mut buffer);
        let mut expected = Buffer::with_lines(vec![
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
        expected.set_style(buffer.area.inner(&Margin::new(1, 1)), Style::new().green());
        expected.set_style(buffer.area.inner(&Margin::new(2, 2)), Style::reset());
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn draw_braille_filled() {
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
                    fill: false,
                });
                // a rectangle that will draw the inside part of the braille
                context.draw(&Rectangle {
                    x: 2.0,
                    y: 1.75,
                    width: 6.5,
                    height: 6.5,
                    color: Color::Green,
                    fill: true,
                });
            });
        canvas.render(buffer.area, &mut buffer);
        let mut expected = Buffer::with_lines(vec![
            "⡏⠉⠉⠉⠉⠉⠉⠉⠉⢹",
            "⡇⢠⣤⣤⣤⣤⣤⣤⡄⢸",
            "⡇⢸⣿⣿⣿⣿⣿⣿⡇⢸",
            "⡇⢸⣿⣿⣿⣿⣿⣿⡇⢸",
            "⡇⢸⣿⣿⣿⣿⣿⣿⡇⢸",
            "⡇⢸⣿⣿⣿⣿⣿⣿⡇⢸",
            "⡇⢸⣿⣿⣿⣿⣿⣿⡇⢸",
            "⡇⢸⣿⣿⣿⣿⣿⣿⡇⢸",
            "⡇⠈⠉⠉⠉⠉⠉⠉⠁⢸",
            "⣇⣀⣀⣀⣀⣀⣀⣀⣀⣸",
        ]);
        expected.set_style(buffer.area, Style::new().red());
        expected.set_style(buffer.area.inner(&Margin::new(1, 1)), Style::new().green());
        assert_buffer_eq!(buffer, expected);
    }
}
