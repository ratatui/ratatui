use line_clipping::{LineSegment, Point, Window, cohen_sutherland};
use ratatui_core::style::Color;

use crate::canvas::{Painter, Shape};

/// A line from `(x1, y1)` to `(x2, y2)` with the given color
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Line {
    /// `x` of the starting point
    pub x1: f64,
    /// `y` of the starting point
    pub y1: f64,
    /// `x` of the ending point
    pub x2: f64,
    /// `y` of the ending point
    pub y2: f64,
    /// Color of the line
    pub color: Color,
}

impl Line {
    /// Create a new line from `(x1, y1)` to `(x2, y2)` with the given color
    pub const fn new(x1: f64, y1: f64, x2: f64, y2: f64, color: Color) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            color,
        }
    }
}

impl Shape for Line {
    #[expect(clippy::similar_names)]
    fn draw(&self, painter: &mut Painter) {
        let (x_bounds, y_bounds) = painter.bounds();
        let Some((world_x1, world_y1, world_x2, world_y2)) =
            clip_line(x_bounds, y_bounds, self.x1, self.y1, self.x2, self.y2)
        else {
            return;
        };
        let Some((x1, y1)) = painter.get_point(world_x1, world_y1) else {
            return;
        };
        let Some((x2, y2)) = painter.get_point(world_x2, world_y2) else {
            return;
        };

        draw_line(painter, x1, y1, x2, y2, self.color);
    }
}

fn clip_line(
    &[xmin, xmax]: &[f64; 2],
    &[ymin, ymax]: &[f64; 2],
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
) -> Option<(f64, f64, f64, f64)> {
    if let Some(LineSegment {
        p1: Point { x: x1, y: y1 },
        p2: Point { x: x2, y: y2 },
    }) = cohen_sutherland::clip_line(
        LineSegment::new(Point::new(x1, y1), Point::new(x2, y2)),
        Window::new(xmin, xmax, ymin, ymax),
    ) {
        Some((x1, y1, x2, y2))
    } else {
        None
    }
}

pub(super) fn draw_line(
    painter: &mut Painter,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: Color,
) {
    for_each_line_point(x1, y1, x2, y2, |x, y| painter.paint(x, y, color));
}

/// Iterate over each point on a line using Bresenham's algorithm.
///
/// Using a callback allows us to reuse the algorithm without duplicating code:
/// - `Line` just paints each point
/// - `FilledLine` paints a vertical column from each point to `fill_to_y`
pub(super) fn for_each_line_point<F>(x1: usize, y1: usize, x2: usize, y2: usize, mut f: F)
where
    F: FnMut(usize, usize),
{
    let dx = x2.abs_diff(x1);
    let dy = y2.abs_diff(y1);

    if dx == 0 {
        for y in y1.min(y2)..=y1.max(y2) {
            f(x1, y);
        }
    } else if dy == 0 {
        for x in x1.min(x2)..=x1.max(x2) {
            f(x, y1);
        }
    } else if dy < dx {
        if x1 > x2 {
            for_each_line_point_low(x2, y2, x1, y1, f);
        } else {
            for_each_line_point_low(x1, y1, x2, y2, f);
        }
    } else if y1 > y2 {
        for_each_line_point_high(x2, y2, x1, y1, f);
    } else {
        for_each_line_point_high(x1, y1, x2, y2, f);
    }
}

fn for_each_line_point_low<F>(x1: usize, y1: usize, x2: usize, y2: usize, mut f: F)
where
    F: FnMut(usize, usize),
{
    let dx = (x2 - x1) as isize;
    let dy = (y2 as isize - y1 as isize).abs();
    let mut d = 2 * dy - dx;
    let mut y = y1;
    for x in x1..=x2 {
        f(x, y);
        if d > 0 {
            y = if y1 > y2 {
                y.saturating_sub(1)
            } else {
                y.saturating_add(1)
            };
            d -= 2 * dx;
        }
        d += 2 * dy;
    }
}

fn for_each_line_point_high<F>(x1: usize, y1: usize, x2: usize, y2: usize, mut f: F)
where
    F: FnMut(usize, usize),
{
    let dx = (x2 as isize - x1 as isize).abs();
    let dy = (y2 - y1) as isize;
    let mut d = 2 * dx - dy;
    let mut x = x1;
    for y in y1..=y2 {
        f(x, y);
        if d > 0 {
            x = if x1 > x2 {
                x.saturating_sub(1)
            } else {
                x.saturating_add(1)
            };
            d -= 2 * dy;
        }
        d += 2 * dx;
    }
}

/// A filled line from `(x1, y1)` to `(x2, y2)` that fills the area under/over the line
/// to `fill_to_y` with the given color.
///
/// This is useful for creating area charts or filling the space under a line graph.
#[derive(Debug, Clone, PartialEq)]
pub struct FilledLine {
    /// `x` of the starting point
    pub x1: f64,
    /// `y` of the starting point
    pub y1: f64,
    /// `x` of the ending point
    pub x2: f64,
    /// `y` of the ending point
    pub y2: f64,
    /// Y-coordinate to fill to (fills area between the line and this Y value)
    pub fill_to_y: f64,
    /// Color of the line and filled area
    pub color: Color,
}

impl FilledLine {
    /// Create a new filled line from `(x1, y1)` to `(x2, y2)` that fills to `fill_to_y`
    pub const fn new(x1: f64, y1: f64, x2: f64, y2: f64, fill_to_y: f64, color: Color) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            fill_to_y,
            color,
        }
    }
}

impl Shape for FilledLine {
    #[expect(clippy::similar_names)]
    fn draw(&self, painter: &mut Painter) {
        let (x_bounds, y_bounds) = painter.bounds();
        let Some((world_x1, world_y1, world_x2, world_y2)) =
            clip_line(x_bounds, y_bounds, self.x1, self.y1, self.x2, self.y2)
        else {
            return;
        };
        let Some((x1, y1)) = painter.get_point(world_x1, world_y1) else {
            return;
        };
        let Some((x2, y2)) = painter.get_point(world_x2, world_y2) else {
            return;
        };

        let y_fill = self.fill_to_y.clamp(y_bounds[0], y_bounds[1]);
        let Some((_, y_fill)) = painter.get_point(world_x1, y_fill) else {
            return;
        };

        for_each_line_point(x1, y1, x2, y2, |x, y| {
            let start = y.min(y_fill);
            let end = y.max(y_fill);
            for y in start..=end {
                painter.paint(x, y, self.color);
            }
        });
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
    #[case::off_grid1(&Line::new(-1.0, 0.0, -1.0, 10.0, Color::Red), ["          "; 10])]
    #[case::off_grid2(&Line::new(0.0, -1.0, 10.0, -1.0, Color::Red), ["          "; 10])]
    #[case::off_grid3(&Line::new(-10.0, 5.0, -1.0, 5.0, Color::Red), ["          "; 10])]
    #[case::off_grid4(&Line::new(5.0, 11.0, 5.0, 20.0, Color::Red), ["          "; 10])]
    #[case::off_grid5(&Line::new(-10.0, 0.0, 5.0, 0.0, Color::Red), [
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
    #[case::off_grid6(&Line::new(-1.0, -1.0, 10.0, 10.0, Color::Red), [
        "         •",
        "        • ",
        "       •  ",
        "      •   ",
        "     •    ",
        "    •     ",
        "   •      ",
        "  •       ",
        " •        ",
        "•         ",
    ])]
    #[case::off_grid7(&Line::new(0.0, 0.0, 11.0, 11.0, Color::Red), [
        "         •",
        "        • ",
        "       •  ",
        "      •   ",
        "     •    ",
        "    •     ",
        "   •      ",
        "  •       ",
        " •        ",
        "•         ",
    ])]
    #[case::off_grid8(&Line::new(-1.0, -1.0, 11.0, 11.0, Color::Red), [
        "         •",
        "        • ",
        "       •  ",
        "      •   ",
        "     •    ",
        "    •     ",
        "   •      ",
        "  •       ",
        " •        ",
        "•         ",
    ])]
    #[case::horizontal1(&Line::new(0.0, 0.0, 10.0, 0.0, Color::Red), [
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "••••••••••",
    ])]
    #[case::horizontal2(&Line::new(10.0, 10.0, 0.0, 10.0, Color::Red), [
        "••••••••••",
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
    ])]
    #[case::vertical1(&Line::new(0.0, 0.0, 0.0, 10.0, Color::Red), ["•         "; 10])]
    #[case::vertical2(&Line::new(10.0, 10.0, 10.0, 0.0, Color::Red), ["         •"; 10])]
    // dy < dx, x1 < x2
    #[case::diagonal1(&Line::new(0.0, 0.0, 10.0, 5.0, Color::Red), [
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "        ••",
        "      ••  ",
        "    ••    ",
        "  ••      ",
        "••        ",
    ])]
    // dy < dx, x1 > x2
    #[case::diagonal2(&Line::new(10.0, 0.0, 0.0, 5.0, Color::Red), [
        "          ",
        "          ",
        "          ",
        "          ",
        "          ",
        "••        ",
        "  ••      ",
        "    ••    ",
        "      ••  ",
        "        ••",
    ])]
    // dy > dx, y1 < y2
    #[case::diagonal3(&Line::new(0.0, 0.0, 5.0, 10.0, Color::Red), [
        "     •    ",
        "    •     ",
        "    •     ",
        "   •      ",
        "   •      ",
        "  •       ",
        "  •       ",
        " •        ",
        " •        ",
        "•         ",
    ])]
    // dy > dx, y1 > y2
    #[case::diagonal4(&Line::new(0.0, 10.0, 5.0, 0.0, Color::Red), [
        "•         ",
        " •        ",
        " •        ",
        "  •       ",
        "  •       ",
        "   •      ",
        "   •      ",
        "    •     ",
        "    •     ",
        "     •    ",
    ])]
    fn tests<'expected_line, ExpectedLines>(#[case] line: &Line, #[case] expected: ExpectedLines)
    where
        ExpectedLines: IntoIterator,
        ExpectedLines::Item: Into<ratatui_core::text::Line<'expected_line>>,
    {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
        let canvas = Canvas::default()
            .marker(Marker::Dot)
            .x_bounds([0.0, 10.0])
            .y_bounds([0.0, 10.0])
            .paint(|context| context.draw(line));
        canvas.render(buffer.area, &mut buffer);

        let mut expected = Buffer::with_lines(expected);
        for cell in &mut expected.content {
            if cell.symbol() == "•" {
                cell.set_style(Style::new().red());
            }
        }
        assert_eq!(buffer, expected);
    }

    #[rstest]
    #[case::filled_off_grid1(&FilledLine::new(-1.0, 0.0, -1.0, 10.0, 0.0, Color::Red), ["          "; 10])]
    #[case::filled_off_grid2(&FilledLine::new(0.0, -1.0, 10.0, -1.0, 0.0, Color::Red), ["          "; 10])]
    #[case::filled_off_grid3(&FilledLine::new(-10.0, 5.0, -1.0, 5.0, 0.0, Color::Red), ["          "; 10])]
    #[case::filled_off_grid4(&FilledLine::new(5.0, 11.0, 5.0, 20.0, 0.0, Color::Red), ["          "; 10])]
    #[case::filled_off_grid5(&FilledLine::new(-10.0, 0.0, 5.0, 0.0, -10.0, Color::Red), [
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
    #[case::filled_off_grid6(&FilledLine::new(0.0, 0.0, 10.0, 10.0, 0.0, Color::Red), [
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
    #[case::filled_off_grid7(&FilledLine::new(0.0, 0.0, 11.0, 11.0, 0.0, Color::Red), [
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
    #[case::filled_off_grid8(&FilledLine::new(-1.0, -1.0, 11.0, 11.0, 0.0, Color::Red), [
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
    #[case::filled_horizontal1(&FilledLine::new(0.0, 0.0, 10.0, 0.0, 0.0, Color::Red), [
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "••••••••••",
        ])]
    #[case::filled_horizontal2(&FilledLine::new(0.0, 0.0, 10.0, 0.0, 10.0, Color::Red), [
            "••••••••••",
            "••••••••••",
            "••••••••••",
            "••••••••••",
            "••••••••••",
            "••••••••••",
            "••••••••••",
            "••••••••••",
            "••••••••••",
            "••••••••••",
        ])]
    #[case::filled_horizontal3(&FilledLine::new(10.0, 10.0, 0.0, 10.0, 10.0, Color::Red), [
            "••••••••••",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
        ])]
    #[case::filled_horizontal4(&FilledLine::new(10.0, 10.0, 0.0, 10.0, 0.0, Color::Red), [
            "••••••••••",
            "••••••••••",
            "••••••••••",
            "••••••••••",
            "••••••••••",
            "••••••••••",
            "••••••••••",
            "••••••••••",
            "••••••••••",
            "••••••••••",
        ])]
    #[case::filled_vertical1(&FilledLine::new(0.0, 0.0, 0.0, 10.0, 0.0, Color::Red), ["•         "; 10])]
    #[case::filled_vertical2(&FilledLine::new(10.0, 10.0, 10.0, 0.0, 0.0, Color::Red), ["         •"; 10])]
    // dy < dx, x1 < x2
    #[case::filled_diagonal1(&FilledLine::new(0.0, 0.0, 10.0, 5.0, 0.0, Color::Red), [
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "        ••",
            "      ••••",
            "    ••••••",
            "  ••••••••",
            "••••••••••",
        ])]
    // dy < dx, x1 > x2
    #[case::filled_diagonal2(&FilledLine::new(10.0, 0.0, 0.0, 5.0, 0.0, Color::Red), [
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "••        ",
            "••••      ",
            "••••••    ",
            "••••••••  ",
            "••••••••••",
        ])]
    // dy > dx, y1 < y2
    #[case::filled_diagonal3(&FilledLine::new(0.0, 0.0, 5.0, 10.0, 0.0, Color::Red), [
            "     •    ",
            "    ••    ",
            "    ••    ",
            "   •••    ",
            "   •••    ",
            "  ••••    ",
            "  ••••    ",
            " •••••    ",
            " •••••    ",
            "••••••    ",
        ])]
    // dy > dx, y1 > y2
    #[case::filled_diagonal4(&FilledLine::new(0.0, 10.0, 5.0, 0.0, 0.0, Color::Red), [
            "•         ",
            "••        ",
            "••        ",
            "•••       ",
            "•••       ",
            "••••      ",
            "••••      ",
            "•••••     ",
            "•••••     ",
            "••••••    ",
        ])]
    #[case::filled_split1(&FilledLine::new(0.0, 0.0, 10.0, 10.0, 5.0, Color::Red), [
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
    #[case::filled_split2(&FilledLine::new(0.0, 0.0, 10.0, 10.0, 7.0, Color::Red), [
            "         •",
            "        ••",
            "       •••",
            "••••••••••",
            "••••••    ",
            "•••••     ",
            "••••      ",
            "•••       ",
            "••        ",
            "•         ",
        ])]
    fn tests_filled<'expected_line, ExpectedLines>(
        #[case] line: &FilledLine,
        #[case] expected: ExpectedLines,
    ) where
        ExpectedLines: IntoIterator,
        ExpectedLines::Item: Into<ratatui_core::text::Line<'expected_line>>,
    {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
        let canvas = Canvas::default()
            .marker(Marker::Dot)
            .x_bounds([0.0, 10.0])
            .y_bounds([0.0, 10.0])
            .paint(|context| context.draw(line));
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
