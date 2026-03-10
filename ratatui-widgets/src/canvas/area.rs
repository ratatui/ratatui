use alloc::vec::Vec;

use core::ops::RangeInclusive;
use std::borrow::ToOwned;

use ratatui_core::style::Color;

use crate::canvas::{Painter, Shape, line};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Area<'a> {
    /// List of vertecies
    pub vertecies: &'a [(f64, f64)],
    /// Color of the points
    pub color: Color,
    pub fill: bool,
}

impl<'a> Area<'a> {
    pub const fn new(vertecies: &'a [(f64, f64)], color: Color, fill: bool) -> Self {
        Self {
            vertecies,
            color,
            fill,
        }
    }
}

impl Shape for Area<'_> {
    fn draw(&self, painter: &mut Painter) {
        let len = self.vertecies.len();
        if len == 0 {
            return;
        }

        // let's to vec it rn to satisfy the borrow checker.
        let x_min_bound = painter.bounds().0[0];
        let x_max_bound = painter.bounds().0[1];
        let y_min_bound = painter.bounds().1[0];
        let y_max_bound = painter.bounds().1[1];
        let (x_min, x_max, y_min, y_max) = self.vertecies.iter().fold(
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

        let x_min = x_min.clamp(x_min_bound, x_max_bound);
        let x_max = x_max.clamp(x_min_bound, x_max_bound);
        let y_min = y_min.clamp(y_min_bound, y_max_bound);
        let y_max = y_max.clamp(y_min_bound, y_max_bound);

        let Some((_, y_min)) = painter.get_point(x_min, y_min) else {
            return;
        };
        let Some((_, y_max)) = painter.get_point(x_max, y_max) else {
            return;
        };

        // idk what i do wrong but my y_min is bigger than y_max
        // so i just switched them
        for y in y_max..=y_min {
            let mut intersections = if self.fill {
                Vec::new()
            } else {
                Vec::with_capacity(0)
            };

            for i in 0..len {
                let p1 = self.vertecies[i];
                // % len to connect last and first vertecies
                let p2 = self.vertecies[(i + 1) % len];

                // let Some((x1, y1, x2, y2)) = line::clip_line(
                //     &[x_min_bound, x_max_bound],
                //     &[y_min_bound, y_max_bound],
                //     p1.0,
                //     p1.1,
                //     p2.0,
                //     p2.1,
                // ) else {
                //     return;
                // };

                let x1 = p1.0.clamp(x_min_bound, x_max_bound);
                let x2 = p2.0.clamp(x_min_bound, x_max_bound);
                let y1 = p1.1.clamp(y_min_bound, y_max_bound);
                let y2 = p2.1.clamp(y_min_bound, y_max_bound);

                let Some((x1, y1)) = painter.get_point(x1, y1) else {
                    return;
                };
                let Some((x2, y2)) = painter.get_point(x2, y2) else {
                    return;
                };

                line::draw_line(painter, x1, y1, x2, y2, self.color);

                if self.fill && (y1 <= y && y < y2) || (y2 <= y && y < y1) {
                    let cross = (x1 as isize
                        + (y as isize - y1 as isize) * (x2 as isize - x1 as isize)
                            / (y2 as isize - y1 as isize)) as usize;
                    intersections.push(cross);
                }
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

// #[cfg(test)]
// mod tests {
//     use ratatui_core::buffer::Buffer;
//     use ratatui_core::layout::Rect;
//     use ratatui_core::style::Style;
//     use ratatui_core::symbols::Marker;
//     use ratatui_core::widgets::Widget;
//     use rstest::rstest;

//     use super::*;
//     use crate::canvas::Canvas;

//     #[rstest]
//     #[case::off_grid1(&Area::new(-1.0, 0.0, -1.0, 10.0, 0.0, Color::Red), ["          "; 10])]
//     #[case::off_grid2(&Area::new(0.0, -1.0, 10.0, -1.0, 0.0, Color::Red), ["          "; 10])]
//     #[case::off_grid3(&Area::new(-10.0, 5.0, -1.0, 5.0, 0.0, Color::Red), ["          "; 10])]
//     #[case::off_grid4(&Area::new(5.0, 11.0, 5.0, 20.0, 0.0, Color::Red), ["          "; 10])]
//     #[case::off_grid5(&Area::new(-10.0, 0.0, 5.0, 0.0, -10.0, Color::Red), [
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "••••••    ",
//     ])]
//     #[case::off_grid6(&Area::new(0.0, 0.0, 10.0, 10.0, 0.0, Color::Red), [
//         "         •",
//         "        ••",
//         "       •••",
//         "      ••••",
//         "     •••••",
//         "    ••••••",
//         "   •••••••",
//         "  ••••••••",
//         " •••••••••",
//         "••••••••••",
//     ])]
//     #[case::off_grid7(&Area::new(0.0, 0.0, 11.0, 11.0, 0.0, Color::Red), [
//         "         •",
//         "        ••",
//         "       •••",
//         "      ••••",
//         "     •••••",
//         "    ••••••",
//         "   •••••••",
//         "  ••••••••",
//         " •••••••••",
//         "••••••••••",
//     ])]
//     #[case::off_grid8(&Area::new(-1.0, -1.0, 11.0, 11.0, 0.0, Color::Red), [
//         "         •",
//         "        ••",
//         "       •••",
//         "      ••••",
//         "     •••••",
//         "    ••••••",
//         "   •••••••",
//         "  ••••••••",
//         " •••••••••",
//         "••••••••••",
//     ])]
//     #[case::horizontal1(&Area::new(0.0, 0.0, 10.0, 0.0, 0.0, Color::Red), [
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "••••••••••",
//     ])]
//     #[case::horizontal2(&Area::new(0.0, 0.0, 10.0, 0.0, 10.0, Color::Red), [
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//     ])]
//     #[case::horizontal3(&Area::new(10.0, 10.0, 0.0, 10.0, 10.0, Color::Red), [
//         "••••••••••",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//     ])]
//     #[case::horizontal4(&Area::new(10.0, 10.0, 0.0, 10.0, 0.0, Color::Red), [
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//         "••••••••••",
//     ])]
//     #[case::vertical1(&Area::new(0.0, 0.0, 0.0, 10.0, 0.0, Color::Red), ["•         "; 10])]
//     #[case::vertical2(&Area::new(10.0, 10.0, 10.0, 0.0, 0.0, Color::Red), ["         •"; 10])]
//     // dy < dx, x1 < x2
//     #[case::diagonal1(&Area::new(0.0, 0.0, 10.0, 5.0, 0.0, Color::Red), [
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "        ••",
//         "      ••••",
//         "    ••••••",
//         "  ••••••••",
//         "••••••••••",
//     ])]
//     // dy < dx, x1 > x2
//     #[case::diagonal2(&Area::new(10.0, 0.0, 0.0, 5.0, 0.0, Color::Red), [
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "          ",
//         "••        ",
//         "••••      ",
//         "••••••    ",
//         "••••••••  ",
//         "••••••••••",
//     ])]
//     // dy > dx, y1 < y2
//     #[case::diagonal3(&Area::new(0.0, 0.0, 5.0, 10.0, 0.0, Color::Red), [
//         "     •    ",
//         "    ••    ",
//         "    ••    ",
//         "   •••    ",
//         "   •••    ",
//         "  ••••    ",
//         "  ••••    ",
//         " •••••    ",
//         " •••••    ",
//         "••••••    ",
//     ])]
//     // dy > dx, y1 > y2
//     #[case::diagonal4(&Area::new(0.0, 10.0, 5.0, 0.0, 0.0, Color::Red), [
//         "•         ",
//         "••        ",
//         "••        ",
//         "•••       ",
//         "•••       ",
//         "••••      ",
//         "••••      ",
//         "•••••     ",
//         "•••••     ",
//         "••••••    ",
//     ])]
//     #[case::split1(&Area::new(0.0, 0.0, 10.0, 10.0, 5.0, Color::Red), [
//         "         •",
//         "        ••",
//         "       •••",
//         "      ••••",
//         "     •••••",
//         "••••••••••",
//         "••••      ",
//         "•••       ",
//         "••        ",
//         "•         ",
//     ])]
//     #[case::split2(&Area::new(0.0, 0.0, 10.0, 10.0, 7.0, Color::Red), [
//         "         •",
//         "        ••",
//         "       •••",
//         "••••••••••",
//         "••••••    ",
//         "•••••     ",
//         "••••      ",
//         "•••       ",
//         "••        ",
//         "•         ",
//     ])]
//     fn tests<'expected_line, ExpectedLines>(
//         #[case] area_line: &Area,
//         #[case] expected: ExpectedLines,
//     ) where
//         ExpectedLines: IntoIterator,
//         ExpectedLines::Item: Into<ratatui_core::text::Line<'expected_line>>,
//     {
//         let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
//         let canvas = Canvas::default()
//             .marker(Marker::Dot)
//             .x_bounds([0.0, 10.0])
//             .y_bounds([0.0, 10.0])
//             .paint(|context| context.draw(area_line));
//         canvas.render(buffer.area, &mut buffer);

//         let mut expected = Buffer::with_lines(expected);
//         for cell in &mut expected.content {
//             if cell.symbol() == "•" {
//                 cell.set_style(Style::new().red());
//             }
//         }
//         assert_eq!(buffer, expected);
//     }
// }
