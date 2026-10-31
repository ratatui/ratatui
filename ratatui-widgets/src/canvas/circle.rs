use ratatui_core::style::Color;

use crate::canvas::{Painter, Shape};
#[cfg(not(feature = "std"))]
use crate::polyfills::F64Polyfills;

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
    pub const fn new(x: f64, y: f64, radius: f64, color: Color) -> Self {
        Self {
            x,
            y,
            radius,
            color,
        }
    }
}

impl Shape for Circle {
    /// Paints every grid dot that the circle passes through.
    ///
    /// Dot `(i, j)` covers grid coordinates `i - 0.5..=i + 0.5` and `j - 0.5..=j + 0.5`, matching
    /// the rounding in [`Painter::get_point`]. The circle is an ellipse in grid coordinates when
    /// the canvas has a different number of dots per unit on each axis.
    ///
    /// Rather than sampling points around the circle, which leaves gaps once the circle has more
    /// dots around it than samples and costs trigonometry per point, this walks along the circle
    /// from dot to dot. Each step moves to the next row or the next column depending on whether
    /// the corner between them is inside the circle, which takes integer arithmetic only. This
    /// matters on targets without hardware `f64` support. Only the visible part of the circle is
    /// walked, so the work follows the number of painted dots rather than the size of the circle.
    ///
    /// The center is snapped to the nearest dot or halfway point between dots, moving it by at
    /// most a quarter of a dot on each axis. This makes the painted shape symmetric, so one
    /// quadrant is computed and mirrored, and keeps the shape steady as the circle moves by less
    /// than a dot. Without snapping, the same circle comes out slightly lopsided in a different
    /// way at each sub-dot position. An anti-aliased renderer would want the exact center instead,
    /// as it can show sub-dot positions.
    ///
    /// Circles with a radius of more than about 300 million dots are not drawn, as the integer
    /// arithmetic would overflow.
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        let (&[left, right], &[bottom, top]) = painter.bounds();
        if right <= left || top <= bottom {
            return;
        }
        // Everything below is in half dots, so dots are at even coordinates and, with the center
        // snapped, offsets from the center to rows and columns of dots are whole numbers.
        let (width, height) = painter.resolution;
        let (max_x, max_y) = (2.0 * (width - 1.0), 2.0 * (height - 1.0));
        let (scale_x, scale_y) = (max_x / (right - left), max_y / (top - bottom));
        let center_x = ((self.x - left) * scale_x).round();
        let center_y = ((top - self.y) * scale_y).round();
        let radius_x = self.radius.abs() * scale_x;
        let radius_y = self.radius.abs() * scale_y;
        if ![center_x, center_y, radius_x, radius_y]
            .iter()
            .all(|value| value.is_finite())
        {
            return;
        }
        // the dot offsets that the circle reaches stop one half dot past its radius
        let (min_x, min_y) = (
            offset_to_visible(center_x, max_x),
            offset_to_visible(center_y, max_y),
        );
        if min_x > radius_x + 1.0 || min_y > radius_y + 1.0 {
            return;
        }
        let Some(bits) = fraction_bits(radius_x, radius_y) else {
            return;
        };
        let mut dots = MirroredDots {
            painter,
            color: self.color,
            center: (center_x as i64, center_y as i64),
            max: (max_x as i64, max_y as i64),
            min_offset: (min_x as i64, min_y as i64),
            max_offset: (
                f64::max(center_x, max_x - center_x) as i64,
                f64::max(center_y, max_y - center_y) as i64,
            ),
        };
        if radius_x == 0.0 || radius_y == 0.0 {
            dots.paint_line(radius_x, radius_y);
        } else {
            dots.walk(radius_x, radius_y, bits);
        }
    }
}

/// Paints dots at offsets from a center in all four quadrants, in half dots.
struct MirroredDots<'a, 'b, 'c> {
    painter: &'a mut Painter<'b, 'c>,
    color: Color,
    center: (i64, i64),
    /// The largest coordinates on the canvas.
    max: (i64, i64),
    /// The smallest offsets at which some mirrored copy of a dot is on the canvas.
    min_offset: (i64, i64),
    /// The largest offsets at which some mirrored copy of a dot is on the canvas.
    max_offset: (i64, i64),
}

impl MirroredDots<'_, '_, '_> {
    /// Paints the dots at offset `(dx, dy)` from the center in each quadrant that is on the canvas.
    fn paint(&mut self, dx: i64, dy: i64) {
        let (center_x, center_y) = self.center;
        for (x, y) in [
            (center_x + dx, center_y + dy),
            (center_x - dx, center_y + dy),
            (center_x + dx, center_y - dy),
            (center_x - dx, center_y - dy),
        ] {
            if (0..=self.max.0).contains(&x) && (0..=self.max.1).contains(&y) {
                self.painter
                    .paint((x / 2) as usize, (y / 2) as usize, self.color);
            }
        }
    }

    /// Paints the dots that a circle with no width or no height covers: a line of dots or a
    /// single dot.
    fn paint_line(&mut self, radius_x: f64, radius_y: f64) {
        let end_x = self.max_offset.0.min((radius_x + 1.0).floor() as i64);
        let end_y = self.max_offset.1.min((radius_y + 1.0).floor() as i64);
        let mut dy = first_offset(self.center.1, self.min_offset.1);
        while dy <= end_y {
            let mut dx = first_offset(self.center.0, self.min_offset.0);
            while dx <= end_x {
                self.paint(dx, dy);
                dx += 2;
            }
            dy += 2;
        }
    }

    /// Walks the visible part of one quadrant of the ellipse, away from the horizontal axis.
    ///
    /// The ellipse equation `b²x² + a²y² - a²b²`, which is negative inside the ellipse, is tested
    /// at the corner of the current dot shared with the dots in the next row and column. If the
    /// corner is inside, the ellipse leaves the dot through the next row, and if it is outside,
    /// through the next column. If it is exactly on the ellipse, all four dots around the corner
    /// touch it.
    fn walk(&mut self, a: f64, b: f64, bits: u32) {
        let (min_x, min_y) = self.min_offset;
        let (max_x, max_y) = self.max_offset;
        // Offsets and corners are whole numbers, so only the squared radii are rounded, to `bits`
        // fractional bits, and the equation is scaled by `2^(2 * bits)` to match. `fraction_bits`
        // keeps the values well inside the range of `i128`.
        let scale = (1_u64 << bits) as f64;
        let a2 = (a * a * scale).round() as i128;
        let b2 = (b * b * scale).round() as i128;
        if a2 == 0 || b2 == 0 {
            // too thin for the rounded equation to describe
            self.paint_line(a, b);
            return;
        }
        let one = 1_i128 << bits;
        let equation = |x: i64, y: i64| {
            let (x, y) = (i128::from(x), i128::from(y));
            one * (b2 * x * x + a2 * y * y) - a2 * b2
        };
        // The column on the vertical axis is at offset 0 when the center is on a dot and 1 when it
        // is halfway between dots. The ellipse stays in it from there to the top.
        let axis = self.center.0 & 1;
        let top = b as i64;

        // The visible offsets form a rectangle and the quadrant only moves away from the axis, so
        // its visible part is one piece. Start where it enters: the first visible row, in the
        // furthest column whose inner edge the ellipse reaches in that row.
        let mut dy = first_offset(self.center.1, min_y);
        let near = (dy - 1).max(0);
        if equation(0, near) > 0 {
            return;
        }
        let mut dx = last_offset_where(axis, a as i64 + 1, |dx| {
            equation((dx - 1).max(0), near) <= 0
        });
        if dx > max_x {
            // It enters through the edge of the visible columns instead, in the first row whose
            // top edge reaches the ellipse at the outer edge of that column.
            dx = max_x;
            let first_row = dy;
            dy = 2 + last_offset_where(first_row - 2, b as i64 + 1, |dy| {
                dy < first_row || equation(dx + 1, dy + 1) < 0
            });
        }

        // As in the midpoint circle algorithm, the equation at the corner is updated rather than
        // recomputed: moving the corner a dot (2 half dots) up changes `y²` by `4y + 4`, and a dot
        // in changes `x²` by `4 - 4x`. Those changes themselves grow by 8 each step, so every step
        // is additions only.
        let (x, y) = (i128::from(dx - 1), i128::from(dy + 1));
        let mut corner = equation(dx - 1, dy + 1);
        let mut next_row = one * a2 * (4 * y + 4);
        let mut next_column = one * b2 * (4 - 4 * x);
        let (row_change, column_change) = (8 * one * a2, 8 * one * b2);

        while dy <= max_y && dx >= min_x {
            self.paint(dx, dy);
            if dx == axis {
                // the dot reaches the top of the ellipse
                if dy + 1 > top {
                    break;
                }
                dy += 2;
                continue;
            }
            if corner == 0 {
                self.paint(dx - 2, dy);
                self.paint(dx, dy + 2);
            }
            let (row, column) = (corner <= 0, corner >= 0);
            if row {
                dy += 2;
                corner += next_row;
                next_row += row_change;
            }
            if column {
                dx -= 2;
                corner += next_column;
                next_column += column_change;
            }
        }
    }
}

/// Returns the largest offset in `low..=high` with the same parity as `low` for which `reaches`
/// holds, given that it holds at `low` and, once it stops holding, does not hold again.
fn last_offset_where(low: i64, high: i64, reaches: impl Fn(i64) -> bool) -> i64 {
    let (mut low, mut high) = (low, high.max(low));
    high -= (high - low) & 1;
    while low < high {
        let middle = low + (high - low) / 4 * 2 + 2;
        if reaches(middle) {
            low = middle;
        } else {
            high = middle - 2;
        }
    }
    low
}

/// Returns how many fractional bits the walk can keep in the squared radii, given in half dots,
/// or `None` if the radii are too large.
///
/// The walk's values are at most about `16 * (2^bits * r²)²`, where `r` is the larger radius but
/// at least 1, so this keeps `2^bits * r²` below `2^59` to stay within `i128`. 32 bits is plenty
/// for the radius of a small circle.
fn fraction_bits(radius_x: f64, radius_y: f64) -> Option<u32> {
    let radius = radius_x.max(radius_y).max(1.0);
    // the exponent of r², so r² < 2^(exponent + 1)
    let exponent = ((radius * radius).to_bits() >> 52) as i32 - 1023;
    u32::try_from(58 - exponent).ok().map(|bits| bits.min(32))
}

/// Returns the smallest offset of at least `min` from `center` to a dot, in half dots.
const fn first_offset(center: i64, min: i64) -> i64 {
    // dots are at even coordinates, so offsets from the center have the same parity as it
    min + ((min - center) & 1)
}

/// Returns the distance from `center` to the nearest point in `0.0..=max`.
fn offset_to_visible(center: f64, max: f64) -> f64 {
    f64::max(-center, center - max).max(0.0)
}

#[cfg(test)]
mod tests {
    use ratatui_core::buffer::Buffer;
    use ratatui_core::layout::Rect;
    use ratatui_core::style::Color;
    use ratatui_core::symbols::Marker;
    use ratatui_core::widgets::Widget;
    use rstest::rstest;

    use crate::canvas::{Canvas, Circle};

    /// Renders the circle with one dot per cell over `0.0..=size - 1` on both axes, so one unit
    /// is one dot.
    fn render_dots(circle: &Circle, size: u16) -> Buffer {
        let mut buffer = Buffer::empty(Rect::new(0, 0, size, size));
        let max = f64::from(size - 1);
        Canvas::default()
            .paint(|ctx| ctx.draw(circle))
            .marker(Marker::Dot)
            .x_bounds([0.0, max])
            .y_bounds([0.0, max])
            .render(buffer.area, &mut buffer);
        buffer
    }

    /// Renders the circle in braille on a canvas of `width` x `height` cells.
    fn render_braille(
        circle: &Circle,
        (width, height): (u16, u16),
        x_bounds: [f64; 2],
        y_bounds: [f64; 2],
    ) -> Buffer {
        let mut buffer = Buffer::empty(Rect::new(0, 0, width, height));
        Canvas::default()
            .paint(|ctx| ctx.draw(circle))
            .marker(Marker::Braille)
            .x_bounds(x_bounds)
            .y_bounds(y_bounds)
            .render(buffer.area, &mut buffer);
        buffer
    }

    /// 365 dots around, more than the 360 points that sampling used.
    #[test]
    fn draws_a_large_circle() {
        let circle = Circle::new(59.5, 59.5, 58.0, Color::Reset);
        let buffer = render_braille(&circle, (60, 30), [0.0, 119.0], [0.0, 119.0]);
        let expected = Buffer::with_lines([
            "                      ⣀⣀⣠⠤⠤⠤⠤⠴⠦⠤⠤⠤⠤⣄⣀⣀                      ",
            "                 ⣀⡤⠖⠚⠉⠁              ⠈⠉⠓⠲⢤⣀                 ",
            "             ⢀⡤⠖⠋⠁                        ⠈⠙⠲⢤⡀             ",
            "           ⣠⠖⠋                                ⠙⠲⣄           ",
            "        ⢀⡴⠞⠁                                    ⠈⠳⢦⡀        ",
            "       ⣠⠞⠁                                        ⠈⠳⣄       ",
            "     ⢀⡼⠁                                            ⠈⢧⡀     ",
            "    ⢠⠏                                                ⠹⡄    ",
            "   ⢠⠏                                                  ⠹⡄   ",
            "  ⢠⠏                                                    ⠹⡄  ",
            "  ⡞                                                      ⢳  ",
            " ⢸⠁                                                      ⠈⡇ ",
            " ⡞                                                        ⢳ ",
            " ⡇                                                        ⢸ ",
            "⢀⡇                                                        ⢸⡀",
            "⠈⡇                                                        ⢸⠁",
            " ⡇                                                        ⢸ ",
            " ⢧                                                        ⡼ ",
            " ⢸⡀                                                      ⢀⡇ ",
            "  ⢧                                                      ⡼  ",
            "  ⠘⣆                                                    ⣰⠃  ",
            "   ⠘⣆                                                  ⣰⠃   ",
            "    ⠘⣆                                                ⣰⠃    ",
            "     ⠈⢳⡀                                            ⢀⡞⠁     ",
            "       ⠙⢦⡀                                        ⢀⡴⠋       ",
            "        ⠈⠳⢦⡀                                    ⢀⡴⠞⠁        ",
            "           ⠙⠦⣄                                ⣠⠴⠋           ",
            "             ⠈⠓⠦⣄⡀                        ⢀⣠⠴⠚⠁             ",
            "                 ⠉⠓⠦⢤⣀⡀              ⢀⣀⡤⠴⠚⠉                 ",
            "                      ⠉⠉⠙⠒⠒⠒⠒⠲⠖⠒⠒⠒⠒⠋⠉⠉                      ",
        ]);
        assert_eq!(buffer, expected);
    }

    /// The same bounds on both axes of a wide canvas give more dots per unit across than down.
    #[test]
    fn draws_a_large_ellipse() {
        let circle = Circle::new(0.0, 0.0, 9.0, Color::Reset);
        let buffer = render_braille(&circle, (60, 15), [-10.0, 10.0], [-10.0, 10.0]);
        let expected = Buffer::with_lines([
            "                        ⢀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⡀                        ",
            "               ⣀⣠⠤⠴⠒⠚⠉⠉⠉⠉          ⠉⠉⠉⠉⠓⠒⠦⠤⣄⣀               ",
            "          ⣀⡤⠖⠚⠉⠁                            ⠈⠉⠓⠲⢤⣀          ",
            "       ⣠⠴⠋⠁                                      ⠈⠙⠦⣄       ",
            "     ⣠⠞⠁                                            ⠈⠳⣄     ",
            "   ⢀⡞⠁                                                ⠈⢳⡀   ",
            "   ⡼                                                    ⢧   ",
            "   ⡇                                                    ⢸   ",
            "   ⢳                                                    ⡞   ",
            "   ⠈⢧⡀                                                ⢀⡼⠁   ",
            "     ⠙⢦⡀                                            ⢀⡴⠋     ",
            "       ⠙⠲⣄⡀                                      ⢀⣠⠖⠋       ",
            "          ⠉⠓⠦⢤⣀⡀                            ⢀⣀⡤⠴⠚⠉          ",
            "               ⠉⠙⠒⠲⠤⢤⣀⣀⣀⣀          ⣀⣀⣀⣀⡤⠤⠖⠒⠋⠉               ",
            "                        ⠈⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠁                        ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn draws_the_top_of_a_large_circle() {
        let circle = Circle::new(39.5, -188.0, 200.0, Color::Reset);
        let buffer = render_braille(&circle, (40, 5), [0.0, 79.0], [0.0, 19.0]);
        let expected = Buffer::with_lines([
            "                                        ",
            "            ⢀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⡀            ",
            "⣀⡤⠤⠤⠖⠒⠒⠚⠉⠉⠉⠉⠉              ⠉⠉⠉⠉⠉⠓⠒⠒⠲⠤⠤⢤⣀",
            "                                        ",
            "                                        ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn draws_the_side_of_a_large_circle() {
        let circle = Circle::new(130.0, 9.5, 100.0, Color::Reset);
        let buffer = render_braille(&circle, (40, 5), [0.0, 79.0], [0.0, 19.0]);
        let expected = Buffer::with_lines([
            "               ⡏                        ",
            "               ⡇                        ",
            "               ⡇                        ",
            "               ⡇                        ",
            "               ⣇                        ",
        ]);
        assert_eq!(buffer, expected);
    }

    fn is_painted(buffer: &Buffer, x: u16, y: u16) -> bool {
        buffer[(x, y)].symbol() != " "
    }

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
            "      ⣀⣀⣀ ",
            "     ⡞⠁ ⠈⢳",
            "     ⢧⡀ ⢀⡼",
            "      ⠉⠉⠉ ",
            "          ",
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn draws_a_small_circle() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
        let canvas = Canvas::default()
            .paint(|ctx| {
                ctx.draw(&Circle::new(0.0, 0.0, 0.5, Color::Reset));
            })
            .marker(Marker::Braille)
            .x_bounds([-1.0, 1.0])
            .y_bounds([-1.0, 1.0]);
        canvas.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines([" ⡤⢤ ", " ⠓⠚ "]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn snaps_center_so_the_shape_is_symmetric() {
        // snaps to (5.0, 5.0), the middle of the 11 x 11 grid
        let buffer = render_dots(&Circle::new(5.2, 4.9, 3.7, Color::Reset), 11);
        for y in 0..11 {
            for x in 0..11 {
                let painted = is_painted(&buffer, x, y);
                assert_eq!(painted, is_painted(&buffer, 10 - x, y), "({x}, {y})");
                assert_eq!(painted, is_painted(&buffer, x, 10 - y), "({x}, {y})");
            }
        }
    }

    /// Sampling 360 points leaves gaps in a circle this big, which a fill from the center would
    /// escape through.
    #[test]
    fn large_circle_has_no_gaps() {
        let size = 301;
        let buffer = render_dots(&Circle::new(150.0, 150.0, 140.0, Color::Reset), size);
        let mut seen = std::vec![false; usize::from(size) * usize::from(size)];
        let mut stack = std::vec![(150_u16, 150_u16)];
        while let Some((x, y)) = stack.pop() {
            let index = usize::from(y) * usize::from(size) + usize::from(x);
            if seen[index] || is_painted(&buffer, x, y) {
                continue;
            }
            seen[index] = true;
            assert!(
                x > 0 && y > 0 && x < size - 1 && y < size - 1,
                "escaped the circle at ({x}, {y})"
            );
            stack.extend([(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]);
        }
    }

    #[test]
    fn clips_a_huge_circle_to_the_canvas() {
        // only the flat top of this circle crosses the canvas, along the middle row
        let buffer = render_dots(&Circle::new(5.0, -1000.0, 1005.0, Color::Reset), 11);
        let mut expected = std::vec!["           "; 11];
        expected[5] = "•••••••••••";
        assert_eq!(buffer, Buffer::with_lines(expected));
    }

    /// Compares the painted dots with a direct check of every dot: the circle passes through a dot
    /// when the dot's square has points both inside and outside the ellipse.
    #[test]
    fn paints_exactly_the_dots_the_circle_passes_through() {
        let size = 16;
        let max = f64::from(size - 1);
        let radii = (1..=80)
            .map(|r| f64::from(r) / 8.0)
            .chain((1..=40).map(|r| f64::from(r) * 0.37))
            .chain([40.0, 100.5]);
        // grid coordinates of centers on and off the canvas, already on dots or halfway between
        let centers = [-20.0, -3.0, 0.0, 2.5, 7.0, 7.5, 15.0, 18.5, 40.0];
        let mut wrong = alloc::vec::Vec::new();
        for dots_per_unit_y in [1.0, 2.0, 0.5] {
            let top = max / dots_per_unit_y;
            for radius in radii.clone() {
                for center_x in centers {
                    for center_y in centers {
                        let circle = Circle::new(
                            center_x,
                            top - center_y / dots_per_unit_y,
                            radius,
                            Color::Reset,
                        );
                        let mut buffer = Buffer::empty(Rect::new(0, 0, size, size));
                        Canvas::default()
                            .paint(|ctx| ctx.draw(&circle))
                            .marker(Marker::Dot)
                            .x_bounds([0.0, max])
                            .y_bounds([0.0, top])
                            .render(buffer.area, &mut buffer);
                        let (a, b) = (radius, radius * dots_per_unit_y);
                        let outside =
                            |x: f64, y: f64| b * b * x * x + a * a * y * y - a * a * b * b;
                        for y in 0..size {
                            for x in 0..size {
                                let (x_near, x_far) = distances(f64::from(x) - center_x);
                                let (y_near, y_far) = distances(f64::from(y) - center_y);
                                let passes =
                                    outside(x_near, y_near) <= 0.0 && outside(x_far, y_far) >= 0.0;
                                if passes != is_painted(&buffer, x, y) {
                                    wrong.push((dots_per_unit_y, radius, center_x, center_y, x, y));
                                }
                            }
                        }
                    }
                }
            }
        }
        assert!(
            wrong.is_empty(),
            "{} wrong dots, e.g. {:?}",
            wrong.len(),
            &wrong[..wrong.len().min(5)]
        );
    }

    /// Returns the nearest and furthest distances from the center to a dot `offset` away from it.
    fn distances(offset: f64) -> (f64, f64) {
        let (low, high) = (offset - 0.5, offset + 0.5);
        let near = if low <= 0.0 && high >= 0.0 {
            0.0
        } else {
            low.abs().min(high.abs())
        };
        (near, low.abs().max(high.abs()))
    }

    #[rstest]
    #[case::outside(Circle::new(50.0, 50.0, 5.0, Color::Reset), 0)]
    #[case::enclosing(Circle::new(5.0, 5.0, 100.0, Color::Reset), 0)]
    #[case::zero_radius(Circle::new(5.0, 5.0, 0.0, Color::Reset), 1)]
    #[case::tiny_radius(Circle::new(5.0, 5.0, 1e-12, Color::Reset), 1)]
    #[case::too_large(Circle::new(5.0, -1e12, 1e12 + 5.0, Color::Reset), 0)]
    #[case::nan_radius(Circle::new(5.0, 5.0, f64::NAN, Color::Reset), 0)]
    #[case::nan_center(Circle::new(f64::NAN, 5.0, 1.0, Color::Reset), 0)]
    fn paints_edge_cases(#[case] circle: Circle, #[case] painted: usize) {
        let buffer = render_dots(&circle, 11);
        let count = buffer
            .content
            .iter()
            .filter(|cell| cell.symbol() != " ")
            .count();
        assert_eq!(count, painted);
    }
}
