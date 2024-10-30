use crate::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

/// A circle with a given center and radius and with a given color
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Circle {
    /// `x` coordinate of the circle's center
    pub x: f64,
    /// `y` coordinate of the circle's center
    pub y: f64,
    /// Radius of the circle
    pub radius: f64,
    /// Color of the circle
    pub color: Color,
}

impl Shape for Circle {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        for angle in 0..360 {
            let radians = f64::from(angle).to_radians();
            let circle_x = self.radius.mul_add(radians.cos(), self.x);
            let circle_y = self.radius.mul_add(radians.sin(), self.y);
            if let Some((x, y)) = painter.get_point(circle_x, circle_y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        buffer::Buffer,
        layout::Rect,
        style::Color,
        symbols::Marker,
        widgets::{
            canvas::{Canvas, Circle},
            Widget,
        },
    };

    #[test]
    fn test_it_draws_a_circle() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));
        let canvas = Canvas::default()
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: 5.0,
                    y: 2.0,
                    radius: 5.0,
                    color: Color::Reset,
                });
            })
            .marker(Marker::Braille)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0]);
        canvas.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines([
            "     ⢀⣠⢤⣀ ",
            "    ⢰⠋  ⠈⣇",
            "    ⠘⣆⡀ ⣠⠇",
            "      ⠉⠉⠁ ",
            "          ",
        ]);
        assert_eq!(buffer, expected);
    }
}
