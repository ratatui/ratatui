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

/// Builds the integer‐grid offsets for a circle outline via the midpoint (Bresenham) algorithm.
///
/// Returns the signed (dx, dy) coordinates around the given center that lie on the circumference.
fn bresenham_circle(center_x: isize, center_y: isize, radius: isize) -> Vec<(isize, isize)> {
    let mut pts = Vec::new();
    let mut x = radius;
    let mut y = 0;
    let mut err = 1 - radius;
    while x >= y {
        let deltas = [
            ( x,  y), (-x,  y), ( x, -y), (-x, -y),
            ( y,  x), (-y,  x), ( y, -x), (-y, -x),
        ];
        for &(dx, dy) in &deltas {
            if let Some(px) = center_x.checked_add(dx) {
                if let Some(py) = center_y.checked_add(dy) {
                    pts.push((px, py));
                }
            }
        }
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
        // Map circle center and radius from world‑coords to grid‑coords (signed).
        let center_x = painter
            .get_point_x(self.x)
            .unwrap_or_else(convert::identity) as isize;
        let center_y = painter
            .get_point_y(self.y)
            .unwrap_or_else(convert::identity) as isize;
        let grid_x = painter
            .get_point_x(self.x + self.radius)
            .unwrap_or_else(convert::identity) as isize;
        let grid_y = painter
            .get_point_y(self.y + self.radius)
            .unwrap_or_else(convert::identity) as isize;
        let radius = (grid_x - center_x).abs().min((grid_y - center_y).abs());

        // Paint the circle outline, skipping any points that overflow beyond the grid
        for (px, py) in bresenham_circle(center_x, center_y, radius) {
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

    #[test]
    fn test_circle_partial_bounds() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 10));
        let canvas = Canvas::default()
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: -5.0,
                    y: -5.0,
                    radius: 10.0,
                    color: Color::Reset,
                });
            })
            .marker(Marker::Braille)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0]);
        canvas.render(buffer.area, &mut buffer);
        // partial circle clipped on left/top
        let expected = Buffer::with_lines(vec![
            "                    ",
            "                    ",
            "⣀⠤⠔⠒⠒⠒⠒⠤⢄⡀          ",
            "         ⠈⠑⢄        ",
            "            ⠑⡄      ",
            "             ⠘⡄     ",
            "              ⢱     ",
            "              ⢸     ",
            "              ⡜     ",
            "             ⡰⠁     ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_circle_out_of_bounds() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 10));
        let canvas = Canvas::default()
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: 100.0,
                    y: 100.0,
                    radius: 5.0,
                    color: Color::Reset,
                });
            })
            .marker(Marker::Braille)
            .x_bounds([-100.0, -90.0])
            .y_bounds([-100.0, -90.0]);
        canvas.render(buffer.area, &mut buffer);
        // fully out of bounds → nothing drawn
        let expected = Buffer::with_lines(vec![
            "                   ⠈",
            "                    ",
            "                    ",
            "                    ",
            "                    ",
            "                    ",
            "                    ",
            "                    ",
            "                    ",
            "                    ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_circle_partial_bounds_bottom_right() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 10));
        let canvas = Canvas::default()
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: 5.0,
                    y: 5.0,
                    radius: 10.0,
                    color: Color::Reset,
                });
            })
            .marker(Marker::Braille)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0]);
        canvas.render(buffer.area, &mut buffer);
        // partial circle clipped on bottom & right edges
        let expected = Buffer::with_lines(vec![
            "           ⡠⠔⠊⠉⠉⠒⠤⡀ ",
            "          ⡰⠁      ⠱⡀",
            "          ⡇        ⡇",
            "          ⠘⡄      ⡜ ",
            "           ⠈⠑⠢⠤⠤⠒⠉  ",
            "                    ",
            "                    ",
            "                    ",
            "                    ",
            "                    ",
        ]);
        assert_eq!(buffer, expected);
    }


}
