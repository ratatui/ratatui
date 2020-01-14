use crate::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols,
    widgets::{Block, Widget},
};
use std::cmp::min;

/// Widget to render a sparkline over one or more lines.
///
/// # Examples
///
/// ```
/// # use ratatui::widgets::{Block, Borders, Sparkline};
/// # use ratatui::style::{Style, Color};
/// Sparkline::default()
///     .block(Block::default().title("Sparkline").borders(Borders::ALL))
///     .data(&[0, 2, 3, 4, 1, 4, 10])
///     .max(5)
///     .style(Style::default().fg(Color::Red).bg(Color::White));
/// ```
#[derive(Debug, Clone)]
pub struct Sparkline<'a> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Widget style
    style: Style,
    /// A slice of the data to display
    data: &'a [u64],
    /// The maximum value to take to compute the maximum bar height (if nothing is specified, the
    /// widget uses the max of the dataset)
    max: Option<u64>,
    /// If true, draws a baseline of `bar::ONE_EIGHTH` spanning the bottom of the sparkline graph
    show_baseline: bool,
    /// A set of bar symbols used to represent the give data
    bar_set: symbols::bar::Set,
    // The direction to render the sparkine, either from left to right, or from right to left
    direction: RenderDirection,
}

#[derive(Debug, Clone, Copy)]
pub enum RenderDirection {
    LeftToRight,
    RightToLeft,
}

impl<'a> Default for Sparkline<'a> {
    fn default() -> Sparkline<'a> {
        Sparkline {
            block: None,
            style: Default::default(),
            data: &[],
            max: None,
            show_baseline: false,
            bar_set: symbols::bar::NINE_LEVELS,
            direction: RenderDirection::LeftToRight,
        }
    }
}

impl<'a> Sparkline<'a> {
    pub fn block(mut self, block: Block<'a>) -> Sparkline<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Sparkline<'a> {
        self.style = style;
        self
    }

    pub fn data(mut self, data: &'a [u64]) -> Sparkline<'a> {
        self.data = data;
        self
    }

    pub fn max(mut self, max: u64) -> Sparkline<'a> {
        self.max = Some(max);
        self
    }

    pub fn show_baseline(mut self, show_baseline: bool) -> Sparkline<'a> {
        self.show_baseline = show_baseline;
        self
    }

    pub fn bar_set(mut self, bar_set: symbols::bar::Set) -> Sparkline<'a> {
        self.bar_set = bar_set;
        self
    }

    pub fn direction(mut self, direction: RenderDirection) -> Sparkline<'a> {
        self.direction = direction;
        self
    }
}

impl<'a> Widget for Sparkline<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let spark_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if spark_area.height < 1 {
            return;
        }

        if self.show_baseline {
            for i in spark_area.left()..spark_area.right() {
                buf.get_mut(i, spark_area.bottom() - 1)
                    .set_symbol(self.bar_set.one_eighth)
                    .set_style(self.style);
            }
        }

        let max = match self.max {
            Some(v) => v,
            None => *self.data.iter().max().unwrap_or(&1u64),
        };
        let max_index = min(spark_area.width as usize, self.data.len());
        let mut data = self
            .data
            .iter()
            .take(max_index)
            .map(|e| {
                if max != 0 {
                    e * u64::from(spark_area.height) * 8 / max
                } else {
                    0
                }
            })
            .collect::<Vec<u64>>();
        for j in (0..spark_area.height).rev() {
            for (i, d) in data.iter_mut().enumerate() {
                let symbol = match *d {
                    0 => {
                        if self.show_baseline && j == spark_area.height - 1 {
                            self.bar_set.one_eighth
                        } else {
                            self.bar_set.empty
                        }
                    }
                    1 => self.bar_set.one_eighth,
                    2 => self.bar_set.one_quarter,
                    3 => self.bar_set.three_eighths,
                    4 => self.bar_set.half,
                    5 => self.bar_set.five_eighths,
                    6 => self.bar_set.three_quarters,
                    7 => self.bar_set.seven_eighths,
                    _ => self.bar_set.full,
                };
                let x = match self.direction {
                    RenderDirection::LeftToRight => spark_area.left() + i as u16,
                    RenderDirection::RightToLeft => spark_area.right() - i as u16 - 1,
                };
                buf.get_mut(x, spark_area.top() + j)
                    .set_symbol(symbol)
                    .set_style(self.style);

                if *d > 8 {
                    *d -= 8;
                } else {
                    *d = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_buffer_eq, buffer::Cell};

    // Helper function to render a sparkline to a buffer with a given width
    // filled with x symbols to make it easier to assert on the result
    fn render(widget: Sparkline, width: u16, height: u16) -> Buffer {
        let mut cell = Cell::default();
        cell.set_symbol("x");
        let mut buffer = Buffer::filled(Rect::new(0, 0, width, height), &cell);
        widget.render(buffer.area, &mut buffer);
        buffer
    }

    #[test]
    fn it_does_not_panic_if_max_is_zero() {
        let widget = Sparkline::default().data(&[0, 0, 0]);
        let buffer = render(widget, 6, 1);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["   xxx"]));
    }

    #[test]
    fn it_does_not_panic_if_max_is_set_to_zero() {
        let widget = Sparkline::default().data(&[0, 1, 2]).max(0);
        let buffer = render(widget, 6, 1);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["   xxx"]));
    }

    #[test]
    fn it_renders() {
        let widget = Sparkline::default().data(&[0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let buffer = render(widget, 12, 1);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![" ▁▂▃▄▅▆▇█xxx"]));
    }

    #[test]
    fn it_renders_with_max_more_than_data_max() {
        let widget = Sparkline::default()
            .data(&[0, 1, 2, 3, 4, 5, 6, 7, 8])
            .max(16);
        let buffer = render(widget, 12, 1);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["  ▁▁▂▂▃▃▄xxx"]));
    }

    #[test]
    fn it_renders_with_max_less_than_data_max() {
        let widget = Sparkline::default()
            .data(&[0, 1, 2, 3, 4, 5, 6, 7, 8])
            .max(4);
        let buffer = render(widget, 12, 1);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![" ▂▄▆█████xxx"]));
    }

    #[test]
    fn it_renders_with_multi_line() {
        let widget = Sparkline::default().data(&[0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let buffer = render(widget, 15, 3);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![
                "      ▂▅█xxxxxx",
                "   ▁▄▇███xxxxxx",
                " ▃▆██████xxxxxx",
            ])
        );
    }

    #[test]
    fn it_renders_with_multi_line_and_baseline() {
        let widget = Sparkline::default()
            .data(&[0, 1, 2, 3, 4, 5, 6, 7, 8])
            .show_baseline(true);
        let buffer = render(widget, 15, 3);
        assert_buffer_eq!(
            buffer,
            // this currently fails because the baseline logic doesn't clear
            // the parts above the line
            // "      ▂▅█xxxxxx",
            // "   ▁▄▇███xxxxxx",
            // " ▃▆██████▁▁▁▁▁▁",
            Buffer::with_lines(vec![
                "      ▂▅█      ",
                "   ▁▄▇███      ",
                " ▃▆██████▁▁▁▁▁▁",
            ])
        );
    }

    #[test]
    fn it_renders_left_to_right() {
        let widget = Sparkline::default()
            .data(&[0, 1, 2, 3, 4, 5, 6, 7, 8])
            .direction(RenderDirection::LeftToRight);
        let buffer = render(widget, 12, 1);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![" ▁▂▃▄▅▆▇█xxx"]));
    }

    #[test]
    fn it_renders_right_to_left() {
        let widget = Sparkline::default()
            .data(&[0, 1, 2, 3, 4, 5, 6, 7, 8])
            .direction(RenderDirection::RightToLeft);
        let buffer = render(widget, 12, 1);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["xxx█▇▆▅▄▃▂▁ "]));
    }

    #[test]
    fn it_renders_baseline() {
        let widget = Sparkline::default()
            .data(&[0, 1, 2, 3, 4, 5, 6, 7, 8])
            .show_baseline(true);
        let buffer = render(widget, 12, 1);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["▁▁▂▃▄▅▆▇█▁▁▁"]));
    }

    #[test]
    fn it_renders_baseline_right_to_left() {
        let widget = Sparkline::default()
            .data(&[0, 1, 2, 3, 4, 5, 6, 7, 8])
            .direction(RenderDirection::RightToLeft)
            .show_baseline(true);
        let buffer = render(widget, 12, 1);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["▁▁▁█▇▆▅▄▃▂▁▁"]));
    }
}
