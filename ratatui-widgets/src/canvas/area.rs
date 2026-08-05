use alloc::vec::Vec;

use line_clipping::{Point, Polygon, Window, sutherland_hodgman};
use ratatui_core::style::Color;

use crate::canvas::{Painter, Shape, line};

/// A shape that draws a polygon defined by a list of vertices.
///
/// The polygon can be convex or non-convex, and may self-intersect. When `fill` is `true`, the
/// interior of the polygon is filled using the specified color; otherwise only the outline is
/// drawn.
///
/// # Bridge artifacts
///
/// This algorithm produces a "bridge" artifact when clipping non-convex polygons that split into
/// multiple pieces. It connects the disjoint visible regions with a straight line along the clip
/// window border, creating a single polygon where multiple separate polygons would be correct.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Area<'a> {
    /// List of vertices defining the polygon
    pub vertices: &'a [(f64, f64)],
    /// Color used to draw the polygon
    pub color: Color,
    /// Whether to fill the interior of the polygon or draw only the outline
    pub fill: bool,
}

impl<'a> Area<'a> {
    /// Creates a new polygon shape.
    ///
    /// # Arguments
    ///
    /// * `vertices` - A slice of `(x, y)` coordinate pairs defining the polygon's vertices
    /// * `color` - The color to use for drawing
    /// * `fill` - If `true`, fills the interior of the polygon; if `false`, draws only the outline
    pub const fn new(vertices: &'a [(f64, f64)], color: Color, fill: bool) -> Self {
        Self {
            vertices,
            color,
            fill,
        }
    }
}

impl Shape for Area<'_> {
    fn draw(&self, painter: &mut Painter) {
        if self.vertices.is_empty() {
            return;
        }

        let x_min_bound = painter.bounds().0[0];
        let x_max_bound = painter.bounds().0[1];
        let y_min_bound = painter.bounds().1[0];
        let y_max_bound = painter.bounds().1[1];

        let vertices: Vec<Point> = self
            .vertices
            .iter()
            .map(|&(x, y)| Point::new(x, y))
            .collect();

        let clipped = sutherland_hodgman::clip_polygon(
            &Polygon::new(&vertices),
            Window::new(x_min_bound, x_max_bound, y_min_bound, y_max_bound),
        );

        if clipped.vertices.is_empty() {
            return;
        }

        // Get the polygon bounds
        let (x_min, x_max, y_min, y_max) = clipped.vertices.iter().fold(
            (
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
            ),
            |(x_min, x_max, y_min, y_max), &Point { x, y }| {
                (x_min.min(x), x_max.max(x), y_min.min(y), y_max.max(y))
            },
        );

        let Some((_, y_max_bound)) = painter.get_point(x_min, y_min) else {
            return;
        };
        let Some((_, y_min_bound)) = painter.get_point(x_max, y_max) else {
            return;
        };

        let Some(&last) = clipped.vertices.last() else {
            return;
        };
        let mut previous = last;

        // Scanline algorithm
        for y in y_min_bound..=y_max_bound {
            let mut intersections = Vec::new();

            for current in &clipped.vertices {
                // in order to avoid [mixed_read_write_in_expression](https://rust-lang.github.io/rust-clippy/master/index.html#mixed_read_write_in_expression)
                // calculate prev_x and prev_y outside of let Some block.
                let (prev_x, prev_y) = (previous.x, previous.y);
                let Some((x1, y1)) = painter.get_point(prev_x, prev_y) else {
                    previous = *current;
                    continue;
                };
                let Some((x2, y2)) = painter.get_point(current.x, current.y) else {
                    continue;
                };
                previous = *current;

                line::draw_line(painter, x1, y1, x2, y2, self.color);

                // skip horizontal lines (don't contribute to intersections)
                if y1 == y2 {
                    continue;
                }

                // Get an intersection of a scanline with a polygon edge
                // Only used when fill because otherwise we don't need to know an intersection
                // point, just draw_line
                if self.fill && ((y1 <= y && y < y2) || (y2 <= y && y < y1)) {
                    // Linearly interpolate along the edge to find its intersection with the
                    // scanline.
                    let cross = (x1 as isize
                        + (y as isize - y1 as isize) * (x2 as isize - x1 as isize)
                            / (y2 as isize - y1 as isize)) as usize;
                    intersections.push(cross);
                }
            }

            if !self.fill {
                continue;
            }

            // Fill polygon. Even-odd rule. E.g. if we have intersections like this
            // ----|---|------|----|
            //     5   8      14   18
            // We should only paint intervals 5-8 and 14-18, not 8-14.
            // This is useful when a polygon is concave.
            intersections.sort_unstable();

            for chunk in intersections.as_chunks::<2>().0 {
                for x in chunk[0]..=chunk[1] {
                    painter.paint(x, y, self.color);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui_core::buffer::Buffer;
    use ratatui_core::layout::Rect;
    use ratatui_core::style::Style;
    use ratatui_core::symbols::Marker;
    use ratatui_core::widgets::Widget;
    use rstest::rstest;

    use super::*;
    use crate::canvas::Canvas;

    #[rstest]
    #[case::empty_area(&Area::new(&[], Color::Red, true), ["          "; 10])]
    #[case::off_grid1(&Area::new(&[(-1.0, 0.0), (-1.0, 10.0), (-1.0,-1.0)], Color::Red, true), ["          "; 10])]
    #[case::off_grid2(&Area::new(&[(0.0, -1.0), (10.0, -1.0), (0.0,-10.0)], Color::Red, true), ["          "; 10])]
    #[case::off_grid3(&Area::new(&[(-10.0, 5.0), (-1.0, 5.0), (-1.0,0.0)], Color::Red, true), ["          "; 10])]
    #[case::off_grid4(&Area::new(&[(5.0, 11.0), (5.0, 20.0), (11.0,11.0)], Color::Red, true), ["          "; 10])]
    #[case::off_grid5(&Area::new(&[(-10.0, 0.0), (5.0, 0.0), (-10.0,0.0)], Color::Red, true), [
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "••••••    ",
    ])]
    #[case::off_grid6(&Area::new(&[(0.0, 0.0), (10.0, 10.0), (10.0, 0.0)], Color::Red, true), [
        "         •",
        "        ••",
        "       •••",
        "      ••••",
        "     •••••",
        "    ••••••",
        "   •••••••",
        "  ••••••••",
        " •••••••••",
        "••••••••••",
    ])]
    #[case::off_grid7(&Area::new(&[(0.0, 0.0), (11.0, 11.0), (10.0, 0.0)], Color::Red, true), [
        "         •",
        "        ••",
        "       •••",
        "      ••••",
        "     •••••",
        "    ••••••",
        "   •••••••",
        "  ••••••••",
        " •••••••••",
        "••••••••••",
    ])]
    #[case::off_grid8(&Area::new(&[(-1.0, -1.0), (11.0, 11.0), (10.0,-1.0)], Color::Red, true), [
        "         •",
        "        ••",
        "       •••",
        "      ••••",
        "     •••••",
        "    ••••••",
        "   •••••••",
        "  ••••••••",
        " •••••••••",
        "••••••••••",
    ])]
    #[case::off_grid9(&Area::new(&[(5.0, 0.0), (5.0, 5.0), (15.0, 0.0)], Color::Red, true), [
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "     ••   ",
        "     •••• ",
        "     •••••",
        "     •••••",
        "     •••••",
    ])]
    #[case::off_grid10(&Area::new(&[(-5.0, 0.0), (-5.0, 5.0), (5.0, 0.0)], Color::Red, true), [
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "••        ",
        "••••      ",
        "••••••    ",
    ])]
    #[case::off_grid11(&Area::new(&[(5.0, 0.0), (5.0, 5.0), (10.0, 5.0), (10.0, 0.0)], Color::Red, true), [
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "     •••••",
        "     •••••",
        "     •••••",
        "     •••••",
        "     •••••",
    ])]
    #[case::off_grid12(&Area::new(&[(7.0, 5.0), (11.0, 10.0), (11.0, 0.0)], Color::Red, true), [
        "          ",
        "         •",
        "        ••",
        "       •••",
        "      ••••",
        "      ••••",
        "       •••",
        "        ••",
        "         •",
        "          ",
    ])]
    #[case::rhombus_1(&Area::new(&[(0.0, 0.0), (0.0, 7.0), (10.0, 10.0), (10.0, 3.0)], Color::Red, true), [
        "        ••",
        "     •••••",
        "  ••••••••",
        "••••••••••",
        "••••••••••",
        "••••••••••",
        "••••••••••",
        "••••••••  ",
        "•••••     ",
        "••        ",
    ])]
    #[case::rhombus_2(&Area::new(&[(5.0, 0.0), (4.0, 0.0), (0.0, 4.0), (0.0, 5.0), (4.0, 10.0), (5.0, 10.0), (10.0, 6.0), (10.0, 5.0)], Color::Red, true), [
        "    ••    ",
        "   ••••   ",
        "  ••••••  ",
        " •••••••• ",
        "••••••••••",
        "••••••••••",
        " •••••••• ",
        "  ••••••  ",
        "   ••••   ",
        "    ••    ",
    ])]
    #[case::rhombus_1_not_filled(&Area::new(&[(0.0, 0.0), (0.0, 7.0), (10.0, 10.0), (10.0, 3.0)], Color::Red, false), [
        "        ••",
        "     ••• •",
        "  •••    •",
        "••       •",
        "•        •",
        "•        •",
        "•       ••",
        "•    •••  ",
        "• •••     ",
        "••        ",
    ])]
    #[case::rhombus_2_not_filled(&Area::new(&[(5.0, 0.0), (4.0, 0.0), (0.0, 5.0), (0.0, 6.0), (4.0, 10.0), (5.0, 10.0), (10.0, 6.0), (10.0, 5.0)], Color::Red, false), [
        "    ••    ",
        "   •  •   ",
        "  •    •  ",
        " •      • ",
        "•        •",
        "•        •",
        " •      • ",
        "  •    •  ",
        "   •  •   ",
        "    ••    ",
    ])]
    #[case::cross1(&Area::new(&[(0.0, 0.0), (0.0, 5.0), (10.0, 5.0), (10.0, 10.0)], Color::Red, true), [
        "         •",
        "        ••",
        "       •••",
        "      ••••",
        "     •••••",
        "••••••••••",
        "••••      ",
        "•••       ",
        "••        ",
        "•         ",
    ])]
    #[case::cross2(&Area::new(&[(0.0, 0.0), (0.0, 7.0), (10.0, 3.0), (10.0, 10.0)], Color::Red, true), [
        "         •",
        "        ••",
        "       •••",
        "••    ••••",
        "••••••••••",
        "••••••••••",
        "••••    ••",
        "•••       ",
        "••        ",
        "•         ",
    ])]
    #[case::cross1_not_filled(&Area::new(&[(0.0, 0.0), (0.0, 5.0), (10.0, 5.0), (10.0, 10.0)], Color::Red, false), [
        "         •",
        "        ••",
        "       • •",
        "      •  •",
        "     •   •",
        "••••••••••",
        "•  •      ",
        "• •       ",
        "••        ",
        "•         ",
    ])]
    #[case::cross_not_filled2(&Area::new(&[(0.0, 0.0), (0.0, 7.0), (10.0, 3.0), (10.0, 10.0)], Color::Red, false), [
        "         •",
        "        ••",
        "       • •",
        "••    •  •",
        "• ••••   •",
        "•   •••• •",
        "•  •    ••",
        "• •       ",
        "••        ",
        "•         ",
    ])]
    fn tests<'expected_line, ExpectedLines>(#[case] area: &Area, #[case] expected: ExpectedLines)
    where
        ExpectedLines: IntoIterator,
        ExpectedLines::Item: Into<ratatui_core::text::Line<'expected_line>>,
    {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
        let canvas = Canvas::default()
            .marker(Marker::Dot)
            .x_bounds([0.0, 10.0])
            .y_bounds([0.0, 10.0])
            .paint(|context| context.draw(area));
        canvas.render(buffer.area, &mut buffer);

        let mut expected = Buffer::with_lines(expected);
        for cell in &mut expected.content {
            if cell.symbol() == "•" {
                cell.set_style(Style::new().red());
            }
        }
        assert_eq!(buffer, expected);
    }
}
