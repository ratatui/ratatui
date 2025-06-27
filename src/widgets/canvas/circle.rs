use std::convert;

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

/// Iterator builder for Bresenham’s (midpoint) circle outline algorithm.
///
/// Yields grid coordinates of the circumference around (cx, cy).
fn bresenham_circle(cx: i32, cy: i32, r: i32) -> Vec<(i32, i32)> {
    let mut pts = Vec::new();
    let mut x = r;
    let mut y = 0;
    let mut err = 1 - r;
    while x >= y {
        pts.push((cx + x, cy + y));
        pts.push((cx - x, cy + y));
        pts.push((cx + x, cy - y));
        pts.push((cx - x, cy - y));
        pts.push((cx + y, cy + x));
        pts.push((cx - y, cy + x));
        pts.push((cx + y, cy - x));
        pts.push((cx - y, cy - x));
        y += 1;
        if err < 0 {
            err += 2 * y + 1;
        } else {
            x -= 1;
            err += 2 * (y - x + 1);
        }
    }
    pts
}

impl Shape for Circle {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        // Map circle center and radius to grid (pixel) coordinates.
        let cx = painter
            .get_point_x(self.x)
            .unwrap_or_else(convert::identity) as i32;
        let cy = painter
            .get_point_y(self.y)
            .unwrap_or_else(convert::identity) as i32;
        let gx = painter
            .get_point_x(self.x + self.radius)
            .unwrap_or_else(convert::identity) as i32;
        let gy = painter
            .get_point_y(self.y + self.radius)
            .unwrap_or_else(convert::identity) as i32;
        let r = (gx - cx).abs().min((gy - cy).abs());

        for (px, py) in bresenham_circle(cx, cy, r) {
            if px >= 0 && py >= 0 {
                painter.paint(px as usize, py as usize, self.color);
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
            "     ⢀⠤⠤⢄ ",
            "    ⢰⠁   ⢱",
            "    ⠘⢄  ⢀⠜",
            "      ⠉⠉⠁ ",
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
            "                                    ⢀⣀⡠⠤⠤⠒⠒⠒⠒⠉⠉⠉⠉⠉⠉⠉⠉⠉⠑⠒⠒⠒⠢⠤⠤⣀⣀                                     ",
            "                               ⣀⠤⠔⠒⠉⠁                          ⠉⠑⠒⠤⢄⡀                               ",
            "                          ⢀⡠⠔⠊⠉                                     ⠈⠉⠒⠤⣀                           ",
            "                       ⣀⠤⠊⠁                                              ⠉⠢⢄⡀                       ",
            "                    ⢀⠔⠊                                                     ⠈⠒⢄                     ",
            "                  ⡠⠊⠁                                                          ⠉⠢⡀                  ",
            "               ⢀⠔⠉                                                               ⠈⠑⢄                ",
            "             ⢀⠔⠁                                                                    ⠑⢄              ",
            "            ⡔⠁                                                                        ⠑⡄            ",
            "          ⡠⠊                                                                           ⠈⠢⡀          ",
            "        ⢀⠔⠁                                                                              ⠑⢄         ",
            "       ⢠⠊                                                                                 ⠈⢢        ",
            "      ⡠⠃                                                                                    ⠣⡀      ",
            "     ⡰⠁                                                                                      ⠱⡀     ",
            "    ⡰⠁                                                                                        ⠱⡀    ",
            "   ⢠⠃                                                                                          ⢣    ",
            "  ⢀⠇                                                                                            ⢇   ",
            "  ⡜                                                                                             ⠘⡄  ",
            " ⢰⠁                                                                                              ⢱  ",
            " ⡎                                                                                               ⠈⡆ ",
            "⢠⠃                                                                                                ⢣ ",
            "⢸                                                                                                 ⢸ ",
            "⡜                                                                                                 ⠘⡄",
            "⡇                                                                                                  ⡇",
            "⡇                                                                                                  ⡇",
            "⡇                                                                                                  ⡇",
            "⡇                                                                                                  ⡇",
            "⢱                                                                                                 ⢰⠁",
            "⢸                                                                                                 ⢸ ",
            "⠈⡆                                                                                                ⡎ ",
            " ⢣                                                                                               ⢠⠃ ",
            " ⠘⡄                                                                                              ⡜  ",
            "  ⢱                                                                                             ⢰⠁  ",
            "   ⢇                                                                                           ⢀⠇   ",
            "   ⠈⡆                                                                                          ⡎    ",
            "    ⠘⡄                                                                                        ⡜     ",
            "     ⠘⡄                                                                                      ⡜      ",
            "      ⠈⢆                                                                                   ⢀⠎       ",
            "       ⠈⢢                                                                                 ⢠⠊        ",
            "         ⠑⡄                                                                              ⡔⠁         ",
            "          ⠈⠢⡀                                                                          ⡠⠊           ",
            "            ⠑⢄                                                                       ⢀⠔⠁            ",
            "              ⠑⢄                                                                   ⢀⠔⠁              ",
            "                ⠑⠤⡀                                                              ⡠⠔⠁                ",
            "                  ⠈⠢⢄                                                         ⢀⠤⠊                   ",
            "                     ⠑⠢⣀                                                   ⢀⡠⠒⠁                     ",
            "                        ⠉⠢⢄⡀                                            ⣀⠤⠊⠁                        ",
            "                           ⠈⠑⠢⠤⣀                                   ⢀⡠⠤⠒⠉                            ",
            "                                ⠉⠑⠒⠤⢄⣀⡀                      ⣀⣀⠤⠔⠒⠉⠁                                ",
            "                                      ⠈⠉⠉⠒⠒⠒⠒⠤⠤⠤⠤⠤⠤⠤⠤⠤⠔⠒⠒⠒⠊⠉⠉                                       ",
        ]);
        assert_eq!(buffer, expected);
    }
}
