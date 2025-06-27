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
            // identify all x pixels covered on this line
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

                painter.paint(grid_start, y_step.grid, self.color);
                painter.paint(grid_end, y_step.grid, self.color);
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
        let expected = Buffer::with_lines(vec![
            "     ⢀⢠⢤⢀ ",
            "    ⢰⠋  ⠈⡇",
            "    ⠘⡆⡀ ⣠⠇",
            "      ⠁⠁⠁ ",
            "          ",
        ]);
        assert_eq!(buffer, expected);
    }

    /// Render a larger circle into a 100×50 buffer using Braille markers.
    ///
    /// This test is initially blank; fill in `expected` after inspecting the printed output.
    #[test]
    #[allow(unused_mut)]
    fn test_draws_large_circle() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 100, 50));
        let canvas = Canvas::default()
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: 0.0,
                    y: 0.0,
                    radius: 25.0,
                    color: Color::Reset,
                });
            })
            .marker(Marker::Braille)
            .x_bounds([-25.0, 25.0])
            .y_bounds([-25.0, 25.0]);
        canvas.render(buffer.area, &mut buffer);
        // Expected rendering of a circle in a 100×50 buffer using Braille markers.
        let expected = Buffer::with_lines(vec![
            "                                   ⢀ ⢠ ⠰  ⠘      ⠈      ⠘  ⠰ ⢠ ⢀                                    ",
            "                              ⡀⡄⠰ ⠃⠈                           ⠈ ⠃⠰ ⡄⡀                              ",
            "                          ⡀⡄⠆⠃⠁                                      ⠁⠃⠆⡄⡀                          ",
            "                      ⢀⡄⠆⠃⠁                                              ⠁⠃⠆⣄                       ",
            "                   ⢀⢠⠞⠈                                                     ⠈⠘⢦⢀                    ",
            "                 ⣠⠰⠋                                                           ⠈⠳⢠⡀                 ",
            "               ⣠⠞⠁                                                                ⠙⢦⡀               ",
            "             ⣠⠞⠁                                                                    ⠙⢦⡀             ",
            "           ⣠⠞⠁                                                                        ⠙⢦⡀           ",
            "         ⢀⡴⠃                                                                            ⠳⣄          ",
            "        ⣠⠞                                                                               ⠘⢦⡀        ",
            "       ⡴⠃                                                                                  ⠳⡄       ",
            "      ⡼⠁                                                                                    ⠹⡄      ",
            "     ⡼⠁                                                                                      ⠹⡄     ",
            "    ⡼⠁                                                                                        ⠹⡄    ",
            "   ⡼⠁                                                                                          ⠹⡄   ",
            "  ⢰⠃                                                                                            ⢳   ",
            " ⢀⡏                                                                                             ⠈⣇  ",
            " ⣸                                                                                               ⢸⡀ ",
            "⢀⡇                                                                                                ⣇ ",
            "⢸                                                                                                 ⢸ ",
            "⡞                                                                                                 ⠘⡆",
            "⡇                                                                                                  ⡇",
            "⡇                                                                                                  ⡇",
            "⡇                                                                                                  ⡇",
            "⡇                                                                                                  ⡇",
            "⡇                                                                                                  ⡇",
            "⡇                                                                                                  ⡇",
            "⢳                                                                                                 ⢰⠃",
            "⢸⡀                                                                                                ⣸ ",
            " ⣇                                                                                               ⢀⡇ ",
            " ⢸⡀                                                                                              ⣸  ",
            "  ⢧                                                                                             ⢠⠇  ",
            "  ⠘⡆                                                                                            ⡞   ",
            "   ⠹⡄                                                                                          ⡼⠁   ",
            "    ⠹⡄                                                                                        ⡼⠁    ",
            "     ⠹⡄                                                                                      ⡼⠁     ",
            "      ⠹⡄                                                                                    ⡼⠁      ",
            "       ⠙⣆                                                                                 ⢀⡞⠁       ",
            "        ⠈⢳⡀                                                                              ⣰⠋         ",
            "          ⠙⣆                                                                           ⢀⡞⠁          ",
            "           ⠈⠳⣄                                                                       ⢀⡴⠋            ",
            "             ⠈⠳⣄                                                                   ⢀⡴⠋              ",
            "               ⠈⠳⣄                                                               ⢀⡴⠋                ",
            "                 ⠈⠘⢦⢀                                                         ⢀⢠⠞⠈                  ",
            "                    ⠈⠳⢠⡀                                                    ⣠⠰⠋                     ",
            "                       ⠁⠃⠆⡄⡀                                            ⡀⡄⠆⠃⠁                       ",
            "                           ⠁⠃⠆⡄⡀                                    ⡀⡄⠆⠃⠁                           ",
            "                               ⠁⠘ ⠆⢠ ⢀                       ⢀ ⢠ ⠆⠘ ⠁                               ",
            "                                     ⠈ ⠘  ⠰      ⢠      ⠰  ⠘ ⠈                                      ",
        ]);
        assert_eq!(buffer, expected);
    }
}
