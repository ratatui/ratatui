use std::cmp::min;

use strum::{Display, EnumString};

use crate::{prelude::*, style::Styled, widgets::Block};

/// Widget to render a sparkline over one or more lines.
///
/// You can create a `Sparkline` using [`Sparkline::default`].
///
/// `Sparkline` can be styled either using [`Sparkline::style`] or preferably using the methods
/// provided by the [`Stylize`](crate::style::Stylize) trait.
///
/// # Setter methods
///
/// - [`Sparkline::block`] wraps the sparkline in a [`Block`]
/// - [`Sparkline::data`] defines the dataset, you'll almost always want to use it
/// - [`Sparkline::max`] sets the maximum value of bars
/// - [`Sparkline::direction`] sets the render direction
///
/// # Examples
///
/// ```
/// use ratatui::{prelude::*, widgets::*};
///
/// Sparkline::default()
///     .block(Block::bordered().title("Sparkline"))
///     .data(&[0, 2, 3, 4, 1, 4, 10])
///     .max(5)
///     .direction(RenderDirection::RightToLeft)
///     .style(Style::default().red().on_white());
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq)]
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
    /// A set of bar symbols used to represent the give data
    bar_set: symbols::bar::Set,
    // The direction to render the sparkine, either from left to right, or from right to left
    direction: RenderDirection,
}

/// Defines the direction in which sparkline will be rendered.
///
/// See [`Sparkline::direction`].
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum RenderDirection {
    /// The first value is on the left, going to the right
    #[default]
    LeftToRight,
    /// The first value is on the right, going to the left
    RightToLeft,
}

impl<'a> Sparkline<'a> {
    /// Wraps the sparkline with the given `block`.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Sets the style of the entire widget.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// The foreground corresponds to the bars while the background is everything else.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the dataset for the sparkline.
    ///
    /// # Example
    ///
    /// ```
    /// # use ratatui::{prelude::*, widgets::*};
    /// # fn ui(frame: &mut Frame) {
    /// # let area = Rect::default();
    /// let sparkline = Sparkline::default().data(&[1, 2, 3]);
    /// frame.render_widget(sparkline, area);
    /// # }
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn data(mut self, data: &'a [u64]) -> Self {
        self.data = data;
        self
    }

    /// Sets the maximum value of bars.
    ///
    /// Every bar will be scaled accordingly. If no max is given, this will be the max in the
    /// dataset.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn max(mut self, max: u64) -> Self {
        self.max = Some(max);
        self
    }

    /// Sets the characters used to display the bars.
    ///
    /// Can be [`symbols::bar::THREE_LEVELS`], [`symbols::bar::NINE_LEVELS`] (default) or a custom
    /// [`Set`](symbols::bar::Set).
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn bar_set(mut self, bar_set: symbols::bar::Set) -> Self {
        self.bar_set = bar_set;
        self
    }

    /// Sets the direction of the sparkline.
    ///
    /// [`RenderDirection::LeftToRight`] by default.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn direction(mut self, direction: RenderDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl<'a> Styled for Sparkline<'a> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl Widget for Sparkline<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

impl WidgetRef for Sparkline<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.block.render_ref(area, buf);
        let inner = self.block.inner_if_some(area);
        self.render_sparkline(inner, buf);
    }
}

impl Sparkline<'_> {
    fn render_sparkline(&self, spark_area: Rect, buf: &mut Buffer) {
        if spark_area.is_empty() {
            return;
        }

        let max = self
            .max
            .unwrap_or_else(|| *self.data.iter().max().unwrap_or(&1));
        let max_index = min(spark_area.width as usize, self.data.len());
        let mut data = self
            .data
            .iter()
            .take(max_index)
            .map(|e| {
                if max == 0 {
                    0
                } else {
                    e * u64::from(spark_area.height) * 8 / max
                }
            })
            .collect::<Vec<u64>>();
        for j in (0..spark_area.height).rev() {
            for (i, d) in data.iter_mut().enumerate() {
                let symbol = match *d {
                    0 => self.bar_set.empty,
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
                buf[(x, spark_area.top() + j)]
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
    use strum::ParseError;

    use super::*;
    use crate::buffer::Cell;

    #[test]
    fn render_direction_to_string() {
        assert_eq!(RenderDirection::LeftToRight.to_string(), "LeftToRight");
        assert_eq!(RenderDirection::RightToLeft.to_string(), "RightToLeft");
    }

    #[test]
    fn render_direction_from_str() {
        assert_eq!(
            "LeftToRight".parse::<RenderDirection>(),
            Ok(RenderDirection::LeftToRight)
        );
        assert_eq!(
            "RightToLeft".parse::<RenderDirection>(),
            Ok(RenderDirection::RightToLeft)
        );
        assert_eq!(
            "".parse::<RenderDirection>(),
            Err(ParseError::VariantNotFound)
        );
    }

    // Helper function to render a sparkline to a buffer with a given width
    // filled with x symbols to make it easier to assert on the result
    fn render(widget: Sparkline<'_>, width: u16) -> Buffer {
        let area = Rect::new(0, 0, width, 1);
        let mut buffer = Buffer::filled(area, Cell::new("x"));
        widget.render(area, &mut buffer);
        buffer
    }

    #[test]
    fn it_does_not_panic_if_max_is_zero() {
        let widget = Sparkline::default().data(&[0, 0, 0]);
        let buffer = render(widget, 6);
        assert_eq!(buffer, Buffer::with_lines(["   xxx"]));
    }

    #[test]
    fn it_does_not_panic_if_max_is_set_to_zero() {
        // see https://github.com/rust-lang/rust-clippy/issues/13191
        #[allow(clippy::unnecessary_min_or_max)]
        let widget = Sparkline::default().data(&[0, 1, 2]).max(0);
        let buffer = render(widget, 6);
        assert_eq!(buffer, Buffer::with_lines(["   xxx"]));
    }

    #[test]
    fn it_draws() {
        let widget = Sparkline::default().data(&[0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let buffer = render(widget, 12);
        assert_eq!(buffer, Buffer::with_lines([" ▁▂▃▄▅▆▇█xxx"]));
    }

    #[test]
    fn it_renders_left_to_right() {
        let widget = Sparkline::default()
            .data(&[0, 1, 2, 3, 4, 5, 6, 7, 8])
            .direction(RenderDirection::LeftToRight);
        let buffer = render(widget, 12);
        assert_eq!(buffer, Buffer::with_lines([" ▁▂▃▄▅▆▇█xxx"]));
    }

    #[test]
    fn it_renders_right_to_left() {
        let widget = Sparkline::default()
            .data(&[0, 1, 2, 3, 4, 5, 6, 7, 8])
            .direction(RenderDirection::RightToLeft);
        let buffer = render(widget, 12);
        assert_eq!(buffer, Buffer::with_lines(["xxx█▇▆▅▄▃▂▁ "]));
    }

    #[test]
    fn can_be_stylized() {
        assert_eq!(
            Sparkline::default()
                .black()
                .on_white()
                .bold()
                .not_dim()
                .style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::DIM)
        );
    }
}
