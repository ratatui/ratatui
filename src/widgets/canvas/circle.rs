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

impl Circle {
    /// Create a new circle with the given center, radius, and color
    pub fn new((x, y): (f64, f64), radius: f64, color: Color) -> Self {
        Self {
            x,
            y,
            radius,
            color,
        }
    }
}

impl Shape for Circle {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        if self.is_completely_outside_bounds(painter) {
            return;
        }

        let (x_bounds, y_bounds) = painter.bounds();
        let translated_x = self.x - x_bounds[0];
        let translated_y = y_bounds[1] - self.y; // invert y to match the coordinate system
        let (center_x, center_y) = self.to_grid(translated_x, translated_y, painter);

        let (radius_x, radius_y) = self.to_grid(self.radius, self.radius, painter);
        let radius = radius_x.min(radius_y);

        let (x_resolution, y_resolution) = painter.resolution();
        for (px, py) in bresenham_circle(center_x, center_y, radius) {
            let x_in_bounds = (0..x_resolution as isize).contains(&px);
            let y_in_bounds = (0..y_resolution as isize).contains(&py);
            if x_in_bounds && y_in_bounds {
                painter.paint(px as usize, py as usize, self.color);
            }
        }
    }
}

impl Circle {
    /// Check if the circle is completely outside the canvas bounds for early exit optimization
    fn is_completely_outside_bounds(&self, painter: &Painter<'_, '_>) -> bool {
        let (x_bounds, y_bounds) = painter.bounds();
        self.x + self.radius < x_bounds[0]
            || self.x - self.radius > x_bounds[1]
            || self.y + self.radius < y_bounds[0]
            || self.y - self.radius > y_bounds[1]
    }

    /// Transform values to grid coordinates
    ///
    /// Returns isize coordinates to handle negative values when coordinates are outside
    /// the canvas bounds. Using isize is safe as grid dimensions are bounded by terminal
    /// size (u16::MAX) times pixels per cell (typically 2x4 for Braille), ensuring values
    /// fit within isize range while allowing negative coordinates for proper clipping.
    fn to_grid(&self, x_value: f64, y_value: f64, painter: &Painter<'_, '_>) -> (isize, isize) {
        let (x_bounds, y_bounds) = painter.bounds();
        let x_width = x_bounds[1] - x_bounds[0];
        let y_height = y_bounds[1] - y_bounds[0];
        if x_width <= 0.0 || y_height <= 0.0 {
            return (0, 0);
        }

        let (x_resolution, y_resolution) = painter.resolution();
        // Uses (resolution - 1.0) to account for zero-based indexing.
        let grid_x = (x_value * (x_resolution - 1.0) / x_width) as isize;
        let grid_y = (y_value * (y_resolution - 1.0) / y_height) as isize;

        (grid_x, grid_y)
    }
}

/// Builds the integer grid coordinates for a circle outline via the midpoint (Bresenham) algorithm.
///
/// Uses isize parameters and returns isize coordinates to handle negative values during
/// calculation, which is essential for proper circle generation when centers are outside bounds.
/// Returns the signed (x, y) coordinates that lie on the circumference of the circle.
fn bresenham_circle(center_x: isize, center_y: isize, radius: isize) -> Vec<(isize, isize)> {
    let mut points = Vec::new();
    let mut x = radius;
    let mut y = 0;
    let mut err = 1 - radius;

    // Generate points using Bresenham's algorithm and 8-fold symmetry
    while x >= y {
        // Generate all 8 symmetric points for this (x, y) pair in counter-clockwise order
        points.extend(center_x.checked_add(x).zip(center_y.checked_add(y)));
        points.extend(center_x.checked_add(y).zip(center_y.checked_add(x)));
        points.extend(center_x.checked_sub(y).zip(center_y.checked_add(x)));
        points.extend(center_x.checked_sub(x).zip(center_y.checked_add(y)));
        points.extend(center_x.checked_sub(x).zip(center_y.checked_sub(y)));
        points.extend(center_x.checked_sub(y).zip(center_y.checked_sub(x)));
        points.extend(center_x.checked_add(y).zip(center_y.checked_sub(x)));
        points.extend(center_x.checked_add(x).zip(center_y.checked_sub(y)));

        y += 1;
        if err < 0 {
            err += 2 * y + 1;
        } else {
            x -= 1;
            err += 2 * (y - x + 1);
        }
    }
    points
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
    fn small_circle() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));
        let circle = Circle::new((0.0, 0.0), 10.0, Color::Reset);
        Canvas::default()
            .marker(Marker::Braille)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0])
            .paint(|ctx| ctx.draw(&circle))
            .render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            " ⡠⠔⠊⠉⠉⠒⠤⡀ ",
            "⡰⠁      ⠱⡀",
            "⡇        ⡇",
            "⠘⡄      ⡜ ",
            " ⠈⠑⠢⠤⠤⠒⠉  ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn medium_circle() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 10));
        let circle = Circle::new((0.0, 0.0), 10.0, Color::Reset);
        Canvas::default()
            .marker(Marker::Braille)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0])
            .paint(|ctx| ctx.draw(&circle))
            .render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "    ⢀⠤⠒⠊⠉⠉⠉⠉⠒⠢⢄     ",
            "  ⡠⠊⠁          ⠉⠢⡀  ",
            " ⡔⠁              ⠑⡄ ",
            "⡸                 ⠸⡀",
            "⡇                  ⡇",
            "⡇                  ⡇",
            "⠸⡀                ⡸ ",
            " ⠑⡄              ⡔⠁ ",
            "  ⠈⠢⢄         ⢀⠤⠊   ",
            "     ⠉⠒⠢⠤⠤⠤⠤⠒⠊⠁     ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn medium_circle_dot() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 10));
        let circle = Circle::new((0.0, 0.0), 10.0, Color::Reset);
        Canvas::default()
            .marker(Marker::Dot)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0])
            .paint(|ctx| ctx.draw(&circle))
            .render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "     •••••••••      ",
            "   ••         ••    ",
            "  •             •   ",
            " •               •  ",
            "•                 • ",
            "•                 • ",
            " •               •  ",
            "  •             •   ",
            "   ••         ••    ",
            "     •••••••••      ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn medium_circle_block() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 10));
        let circle = Circle::new((0.0, 0.0), 10.0, Color::Reset);
        Canvas::default()
            .marker(Marker::Block)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0])
            .paint(|ctx| ctx.draw(&circle))
            .render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "     █████████      ",
            "   ██         ██    ",
            "  █             █   ",
            " █               █  ",
            "█                 █ ",
            "█                 █ ",
            " █               █  ",
            "  █             █   ",
            "   ██         ██    ",
            "     █████████      ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn medium_circle_half_block() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 10));
        let circle = Circle::new((0.0, 0.0), 10.0, Color::Reset);
        Canvas::default()
            .marker(Marker::HalfBlock)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0])
            .paint(|ctx| ctx.draw(&circle))
            .render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "     ▄▄▄▄▄▄▄▄▄      ",
            "   ▄▄         ▄▄    ",
            "  ▄             ▄   ",
            " ▄               ▄  ",
            "▄                 ▄ ",
            "▄                 ▄ ",
            " ▄               ▄  ",
            "  ▄             ▄   ",
            "   ▄▄         ▄▄    ",
            "     ▄▄▄▄▄▄▄▄▄      ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn medium_circle_bar() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 10));
        let circle = Circle::new((0.0, 0.0), 10.0, Color::Reset);
        Canvas::default()
            .marker(Marker::Bar)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0])
            .paint(|ctx| ctx.draw(&circle))
            .render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "     ▐▐▐▐▐▐▐▐▐      ",
            "   ▐▐         ▐▐    ",
            "  ▐             ▐   ",
            " ▐               ▐  ",
            "▐                 ▐ ",
            "▐                 ▐ ",
            " ▐               ▐  ",
            "  ▐             ▐   ",
            "   ▐▐         ▐▐    ",
            "     ▐▐▐▐▐▐▐▐▐      ",
        ]);
        assert_eq!(buffer, expected);
    }

    /// Ensures that circles with more then 360 points are rendered correctly without omitting any
    /// points.
    #[test]
    #[allow(unused_mut)]
    fn large_circle() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 100, 50));
        let circle = Circle::new((0.0, 0.0), 10.0, Color::Reset);
        Canvas::default()
            .marker(Marker::Braille)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0])
            .paint(|ctx| ctx.draw(&circle))
            .render(buffer.area, &mut buffer);
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
    fn out_of_bounds() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 10));
        let circle = Circle::new((20.0, 20.0), 5.0, Color::Reset);
        Canvas::default()
            .marker(Marker::Braille)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0])
            .paint(|ctx| ctx.draw(&circle))
            .render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "                    ",
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
    fn partial_out_of_bounds_bottom_left() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 10));
        let circle = Circle::new((-5.0, -5.0), 10.0, Color::Reset);
        Canvas::default()
            .marker(Marker::Braille)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0])
            .paint(|ctx| ctx.draw(&circle))
            .render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "                    ",
            "                    ",
            " ⣀⡠⠤⠤⠤⠤⣀⡀           ",
            "⠉       ⠈⠑⠤⡀        ",
            "           ⠈⢆       ",
            "             ⢣      ",
            "             ⠈⡆     ",
            "              ⡇     ",
            "             ⢠⠃     ",
            "            ⢀⠎      ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn partial_out_of_bounds_top_right() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 10));
        let circle = Circle::new((5.0, 5.0), 10.0, Color::Reset);
        Canvas::default()
            .marker(Marker::Braille)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0])
            .paint(|ctx| ctx.draw(&circle))
            .render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "     ⢠⠃             ",
            "     ⡎              ",
            "     ⡇              ",
            "     ⢣              ",
            "     ⠈⢆             ",
            "       ⠣⡀           ",
            "        ⠈⠑⠤⣀⡀    ⣀⡠⠔",
            "            ⠈⠉⠉⠉⠉   ",
            "                    ",
            "                    ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn center_out_of_bounds() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 10));
        let circle = Circle::new((20.0, 20.0), 20.0, Color::Reset);
        Canvas::default()
            .marker(Marker::Braille)
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0])
            .paint(|ctx| ctx.draw(&circle))
            .render(buffer.area, &mut buffer);
        // Now correctly handles centers outside bounds - circle is completely out of view
        let expected = Buffer::with_lines(vec![
            "            ⠱⡀      ",
            "             ⠘⢄     ",
            "               ⠑⢄   ",
            "                 ⠉⠢⢄",
            "                    ",
            "                    ",
            "                    ",
            "                    ",
            "                    ",
            "                    ",
        ]);
        assert_eq!(buffer, expected);
    }
}
