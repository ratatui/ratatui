use core::ops::RangeInclusive;

use alloc::vec::Vec;

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
/// This algorithm produces a "bridge" artifact when clipping non-convex polygons that split into multiple pieces.
/// It connects the disjoint visible regions with a straight line along the clip window border,
/// creating a single polygon where multiple separate polygons would be correct.
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
        let len = self.vertices.len();
        if len == 0 {
            return;
        }

        let x_min_bound = painter.bounds().0[0];
        let x_max_bound = painter.bounds().0[1];
        let y_min_bound = painter.bounds().1[0];
        let y_max_bound = painter.bounds().1[1];

        let clipped = clip_polygon(
            self.vertices,
            x_min_bound,
            x_max_bound,
            y_min_bound,
            y_max_bound,
        );

        if clipped.is_empty() {
            return;
        }

        let len = clipped.len();

        let (x_min, x_max, y_min, y_max) = clipped.iter().fold(
            (
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
            ),
            |(x_min, x_max, y_min, y_max), &(x, y)| {
                (x_min.min(x), x_max.max(x), y_min.min(y), y_max.max(y))
            },
        );

        let Some((_, y_max_bound)) = painter.get_point(x_min, y_min) else {
            return;
        };
        let Some((_, y_min_bound)) = painter.get_point(x_max, y_max) else {
            return;
        };

        for y in y_min_bound..=y_max_bound {
            let mut intersections = if self.fill {
                Vec::new()
            } else {
                Vec::with_capacity(0)
            };

            for i in 0..len {
                let p1 = clipped[i];
                // % len to connect last and first vertices
                let p2 = clipped[(i + 1) % len];

                let Some((x1, y1)) = painter.get_point(p1.0, p1.1) else {
                    continue;
                };
                let Some((x2, y2)) = painter.get_point(p2.0, p2.1) else {
                    continue;
                };

                // skip horizontal lines (don't contribute to intersections)
                if y1 == y2 {
                    line::draw_line(painter, x1, y1, x2, y2, self.color);
                    continue;
                }

                if self.fill && ((y1 <= y && y < y2) || (y2 <= y && y < y1)) {
                    let cross = (x1 as isize
                        + (y as isize - y1 as isize) * (x2 as isize - x1 as isize)
                            / (y2 as isize - y1 as isize)) as usize;
                    intersections.push(cross);
                }

                line::draw_line(painter, x1, y1, x2, y2, self.color);
            }

            if !self.fill {
                continue;
            }

            intersections.sort_unstable();

            let ranges: Vec<RangeInclusive<usize>> = intersections
                .chunks(2)
                .map(|chunk| chunk[0]..=chunk[1])
                .collect();

            for range in ranges {
                for x in range {
                    painter.paint(x, y, self.color);
                }
            }
        }
    }
}

fn clip_polygon(
    vertices: &[(f64, f64)],
    x_min_bound: f64,
    x_max_bound: f64,
    y_min_bound: f64,
    y_max_bound: f64,
) -> Vec<(f64, f64)> {
    let clipped = vertices.to_vec();
    let clipped = clip_left(&clipped, x_min_bound);
    let clipped = clip_right(&clipped, x_max_bound);
    let clipped = clip_bottom(&clipped, y_min_bound);
    clip_top(&clipped, y_max_bound)
}

fn clip_top(clipped: &Vec<(f64, f64)>, y_max_bound: f64) -> Vec<(f64, f64)> {
    clip_edge(
        clipped,
        |(_, y)| y <= y_max_bound,
        |(x1, y1), (x2, y2)| {
            let t = (y_max_bound - y1) / (y2 - y1);
            (x1 + t * (x2 - x1), y_max_bound)
        },
    )
}

fn clip_bottom(clipped: &Vec<(f64, f64)>, y_min_bound: f64) -> Vec<(f64, f64)> {
    clip_edge(
        clipped,
        |(_, y)| y >= y_min_bound,
        |(x1, y1), (x2, y2)| {
            let t = (y_min_bound - y1) / (y2 - y1);
            (x1 + t * (x2 - x1), y_min_bound)
        },
    )
}

fn clip_right(clipped: &Vec<(f64, f64)>, x_max_bound: f64) -> Vec<(f64, f64)> {
    clip_edge(
        clipped,
        |(x, _)| x <= x_max_bound,
        |(x1, y1), (x2, y2)| {
            let t = (x_max_bound - x1) / (x2 - x1);
            (x_max_bound, (y1 + t * (y2 - y1)))
        },
    )
}

fn clip_left(clipped: &Vec<(f64, f64)>, x_min_bound: f64) -> Vec<(f64, f64)> {
    clip_edge(
        clipped,
        |(x, _)| x >= x_min_bound,
        |(x1, y1), (x2, y2)| {
            let t = (x_min_bound - x1) / (x2 - x1);
            (x_min_bound, (y1 + t * (y2 - y1)))
        },
    )
}

fn clip_edge<F, I>(vertices: &Vec<(f64, f64)>, is_inside: F, get_intersection: I) -> Vec<(f64, f64)>
where
    F: Fn((f64, f64)) -> bool,
    I: Fn((f64, f64), (f64, f64)) -> (f64, f64),
{
    let mut result = Vec::new();
    let len = vertices.len();
    for i in 0..len {
        let p1 = vertices[i];
        // % len to connect last and first vertices
        let p2 = vertices[(i + 1) % len];

        let p1_inside = is_inside(p1);
        let p2_inside = is_inside(p2);

        if p2_inside {
            if !p1_inside {
                result.push(get_intersection(p1, p2));
            }
            result.push(p2);
        } else {
            if p1_inside {
                result.push(get_intersection(p1, p2));
            }
        }
    }
    result
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
