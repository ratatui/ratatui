use crate::{
    style::Color,
    widgets::canvas::{Line, Painter, Shape},
};

/// Shape to draw a rectangle from a `Rect` with the given color
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
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
        assert_buffer_eq,
        prelude::*,
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
}
