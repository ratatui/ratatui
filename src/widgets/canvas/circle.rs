use std::{convert, mem};

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
    /// Whether the area part of the circle should be filled
    pub fill: bool,
}

impl Shape for Circle {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        fn swap_sort<T: PartialOrd>(a: &mut T, b: &mut T) {
            if a > b {
                mem::swap(a, b);
            }
        }

        // draw circle line by line
        for y_step in painter.step_points_y((self.y - self.radius)..=(self.y + self.radius)) {
            // identify the range of x pixels at this horizontal y overlapping with the circle line
            // dx_start, dx_end are the absolute distance of the range from the central x line.
            let [dx_start, dx_end] = [y_step.canvas.start, y_step.canvas.end].map(|canvas_y| {
                // dx_start..dx_end is the range of dx values such that dx^2 + dy^2 = r^2
                // for all dy contained in y_step.canvas
                let r2 = self.radius.powi(2);
                let dy2 = (canvas_y - self.y).powi(2);
                if r2 > dy2 {
                    (r2 - dy2).sqrt()
                } else {
                    0.0 // possibly float precision error, dx should be 0 since this implies dy is
                        // out of the circle
                }
            });

            if self.fill {
                let dx = dx_start.max(dx_end);
                let grid_start = painter
                    .get_point_x(self.x - dx)
                    .unwrap_or_else(convert::identity);
                let grid_end = painter
                    .get_point_x(self.x + dx)
                    .unwrap_or_else(convert::identity);

                for grid_x in grid_start..=grid_end {
                    painter.paint(grid_x, y_step.grid, self.color);
                }
            } else {
                for sign in [-1., 1.] {
                    let canvas_start = self.x + dx_start * sign;
                    let mut grid_start = painter
                        .get_point_x(canvas_start)
                        .unwrap_or_else(convert::identity);

                    let canvas_end = self.x + dx_end * sign;
                    let mut grid_end = painter
                        .get_point_x(canvas_end)
                        .unwrap_or_else(convert::identity);

                    swap_sort(&mut grid_start, &mut grid_end);

                    for grid_x in grid_start..=grid_end {
                        painter.paint(grid_x, y_step.grid, self.color);
                    }
                }
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
    fn test_it_draws_a_circle_line() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));
        let canvas = Canvas::default()
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: 5.0,
                    y: 2.0,
                    radius: 5.0,
                    color: Color::Reset,
                    fill: false,
                });
            })
            .marker(Marker::Braille)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0]);
        canvas.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "     ⢀⣠⢤⣀ ",
            "    ⢰⠋  ⠈⡇",
            "    ⠘⣆⡀ ⣠⠇",
            "      ⠉⠉⠁ ",
            "          ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_it_draws_a_circle_filled() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));
        let canvas = Canvas::default()
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: 5.0,
                    y: 2.0,
                    radius: 5.0,
                    color: Color::Reset,
                    fill: true,
                });
            })
            .marker(Marker::Braille)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0]);
        canvas.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "     ⢀⣠⣤⣀ ",
            "    ⢰⣿⣿⣿⣿⡇",
            "    ⠘⣿⣿⣿⣿⠇",
            "      ⠉⠉⠁ ",
            "          ",
        ]);
        assert_eq!(buffer, expected);
    }
}
