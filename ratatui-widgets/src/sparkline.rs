//! The [`Sparkline`] widget is used to display a sparkline over one or more lines.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::min;

use ratatui_core::buffer::{Buffer, Cell};
use ratatui_core::layout::Rect;
use ratatui_core::style::{Style, Styled};
use ratatui_core::symbols::braille::BRAILLE;
use ratatui_core::symbols::pixel::{OCTANTS, QUADRANTS, SEXTANTS};
use ratatui_core::symbols::{self, Marker};
use ratatui_core::widgets::Widget;
use strum::{Display, EnumString};

use crate::block::{Block, BlockExt};

/// Widget to render a sparkline over one or more lines.
///
/// Each bar in a `Sparkline` represents a value from the provided dataset. The height of the bar
/// is determined by the value in the dataset.
///
/// You can create a `Sparkline` using [`Sparkline::default`].
///
/// The data is set using [`Sparkline::data`]. The data can be a slice of `u64`, `Option<u64>`, or a
/// [`SparklineBar`].  For the `Option<u64>` and [`SparklineBar`] cases, a data point with a value
/// of `None` is interpreted an as the _absence_ of a value.
///
/// `Sparkline` can be styled either using [`Sparkline::style`] or preferably using the methods
/// provided by the [`Stylize`](ratatui_core::style::Stylize) trait.  The style may be set for the
/// entire widget or for individual bars by setting individual [`SparklineBar::style`].
///
/// The bars are rendered using a set of symbols. The default set is [`symbols::bar::NINE_LEVELS`].
/// You can change the set using [`Sparkline::bar_set`], or configure a custom [`Marker`] (such as
/// [`Marker::Braille`]) using [`Sparkline::marker`].
///
/// If the data provided is a slice of `u64` or `Option<u64>`, the bars will be styled with the
/// style of the sparkline. If the data is a slice of [`SparklineBar`], the bars will be
/// styled with the style of the sparkline combined with the style provided in the [`SparklineBar`]
/// if it is set, otherwise the sparkline style will be used.
///
/// Absent values and will be rendered with the style set by [`Sparkline::absent_value_style`] and
/// the symbol set by [`Sparkline::absent_value_symbol`].
///
/// # Setter methods
///
/// - [`Sparkline::block`] wraps the sparkline in a [`Block`]
/// - [`Sparkline::data`] defines the dataset, you'll almost always want to use it
/// - [`Sparkline::max`] sets the maximum value of bars
/// - [`Sparkline::direction`] sets the render direction
/// - [`Sparkline::marker`] sets the marker type (e.g. [`Marker::Braille`])
/// - [`Sparkline::bar_set`] sets the bar symbols when using [`Marker::Bar`]
///
/// # Examples
///
/// ```
/// use ratatui::style::{Color, Style, Stylize};
/// use ratatui::symbols;
/// use ratatui::widgets::{Block, RenderDirection, Sparkline};
///
/// Sparkline::default()
///     .block(Block::bordered().title("Sparkline"))
///     .data(&[0, 2, 3, 4, 1, 4, 10])
///     .max(5)
///     .direction(RenderDirection::RightToLeft)
///     .style(Style::default().red().on_white())
///     .absent_value_style(Style::default().fg(Color::Red))
///     .absent_value_symbol(symbols::shade::FULL);
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Sparkline<'a> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Widget style
    style: Style,
    /// Style of absent values
    absent_value_style: Style,
    /// The symbol to use for absent values
    absent_value_symbol: AbsentValueSymbol,
    /// A slice of the data to display
    data: Vec<SparklineBar>,
    /// The maximum value to take to compute the maximum bar height (if nothing is specified, the
    /// widget uses the max of the dataset)
    max: Option<u64>,
    /// A set of bar symbols used to represent the give data
    bar_set: symbols::bar::Set<'a>,
    /// The direction to render the sparkline, either from left to right, or from right to left
    direction: RenderDirection,
    /// The marker to use when rendering the sparkline
    marker: Marker,
}

impl Default for Sparkline<'_> {
    fn default() -> Self {
        Self {
            block: None,
            style: Style::default(),
            absent_value_style: Style::default(),
            absent_value_symbol: AbsentValueSymbol::default(),
            data: Vec::new(),
            max: None,
            bar_set: symbols::bar::Set::default(),
            direction: RenderDirection::default(),
            marker: Marker::Bar,
        }
    }
}

/// Defines the direction in which sparkline will be rendered.
///
/// See [`Sparkline::direction`].
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the style to use for absent values.
    ///
    /// Absent values are values in the dataset that are `None`.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// The foreground corresponds to the bars while the background is everything else.
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn absent_value_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.absent_value_style = style.into();
        self
    }

    /// Sets the symbol to use for absent values.
    ///
    /// Absent values are values in the dataset that are `None`.
    ///
    /// The default is [`symbols::shade::EMPTY`].
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn absent_value_symbol(mut self, symbol: impl Into<String>) -> Self {
        self.absent_value_symbol = AbsentValueSymbol(symbol.into());
        self
    }

    /// Sets the dataset for the sparkline.
    ///
    /// Each item in the dataset is a bar in the sparkline. The height of the bar is determined by
    /// the value in the dataset.
    ///
    /// The data can be a slice of `u64`, `Option<u64>`, or a [`SparklineBar`].  For the
    /// `Option<u64>` and [`SparklineBar`] cases, a data point with a value of `None` is
    /// interpreted an as the _absence_ of a value.
    ///
    /// If the data provided is a slice of `u64` or `Option<u64>`, the bars will be styled with the
    /// style of the sparkline. If the data is a slice of [`SparklineBar`], the bars will be
    /// styled with the style of the sparkline combined with the style provided in the
    /// [`SparklineBar`] if it is set, otherwise the sparkline style will be used.
    ///
    /// Absent values and will be rendered with the style set by [`Sparkline::absent_value_style`]
    /// and the symbol set by [`Sparkline::absent_value_symbol`].
    ///
    /// # Examples
    ///
    /// Create a `Sparkline` from a slice of `u64`:
    ///
    /// ```
    /// use ratatui::Frame;
    /// use ratatui::layout::Rect;
    /// use ratatui::widgets::Sparkline;
    ///
    /// # fn ui(frame: &mut Frame) {
    /// # let area = Rect::default();
    /// let sparkline = Sparkline::default().data(&[1, 2, 3]);
    /// frame.render_widget(sparkline, area);
    /// # }
    /// ```
    ///
    /// Create a `Sparkline` from a slice of `Option<u64>` such that some bars are absent:
    ///
    /// ```
    /// # use ratatui::{prelude::*, widgets::*};
    /// # fn ui(frame: &mut Frame) {
    /// # let area = Rect::default();
    /// let data = vec![Some(1), None, Some(3)];
    /// let sparkline = Sparkline::default().data(data);
    /// frame.render_widget(sparkline, area);
    /// # }
    /// ```
    ///
    /// Create a [`Sparkline`] from a Vec of [`SparklineBar`] such that some bars are styled:
    ///
    /// ```
    /// # use ratatui::{prelude::*, widgets::*};
    /// # fn ui(frame: &mut Frame) {
    /// # let area = Rect::default();
    /// let data = vec![
    ///     SparklineBar::from(1).style(Some(Style::default().fg(Color::Red))),
    ///     SparklineBar::from(2),
    ///     SparklineBar::from(3).style(Some(Style::default().fg(Color::Blue))),
    /// ];
    /// let sparkline = Sparkline::default().data(data);
    /// frame.render_widget(sparkline, area);
    /// # }
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn data<T>(mut self, data: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<SparklineBar>,
    {
        self.data = data.into_iter().map(Into::into).collect();
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
    pub const fn bar_set(mut self, bar_set: symbols::bar::Set<'a>) -> Self {
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

    /// Sets the marker type used to render the sparkline.
    ///
    /// Defaults to [`Marker::Bar`]. When [`Marker::Bar`] is used, the characters from
    /// [`Sparkline::bar_set`] are used.
    ///
    /// For 2-column markers like [`Marker::Braille`], [`Marker::Quadrant`], [`Marker::Sextant`],
    /// and [`Marker::Octant`], each terminal column displays 2 data points, doubling the
    /// horizontal resolution.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::symbols::Marker;
    /// use ratatui::widgets::Sparkline;
    ///
    /// let sparkline = Sparkline::default()
    ///     .data([1, 2, 3, 4])
    ///     .marker(Marker::Braille);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn marker(mut self, marker: Marker) -> Self {
        self.marker = marker;
        self
    }
}

/// An bar in a `Sparkline`.
///
/// The height of the bar is determined by the value and a value of `None` is interpreted as the
/// _absence_ of a value, as distinct from a value of `Some(0)`.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct SparklineBar {
    /// The value of the bar.
    ///
    /// If `None`, the bar is absent.
    value: Option<u64>,
    /// The style of the bar.
    ///
    /// If `None`, the bar will use the style of the sparkline.
    style: Option<Style>,
}

impl SparklineBar {
    /// Sets the style of the bar.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// If not set, the default style of the sparkline will be used.
    ///
    /// As well as the style of the sparkline, each [`SparklineBar`] may optionally set its own
    /// style.  If set, the style of the bar will be the style of the sparkline combined with
    /// the style of the bar.
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Option<Style>>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }
}

impl From<Option<u64>> for SparklineBar {
    fn from(value: Option<u64>) -> Self {
        Self { value, style: None }
    }
}

impl From<u64> for SparklineBar {
    fn from(value: u64) -> Self {
        Self {
            value: Some(value),
            style: None,
        }
    }
}

impl From<&u64> for SparklineBar {
    fn from(value: &u64) -> Self {
        Self {
            value: Some(*value),
            style: None,
        }
    }
}

impl From<&Option<u64>> for SparklineBar {
    fn from(value: &Option<u64>) -> Self {
        Self {
            value: *value,
            style: None,
        }
    }
}

impl Styled for Sparkline<'_> {
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
        Widget::render(&self, area, buf);
    }
}

impl Widget for &Sparkline<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.block.as_ref().render(area, buf);
        let inner = self.block.inner_if_some(area);
        self.render_sparkline(inner, buf);
    }
}

/// A newtype wrapper for the symbol to use for absent values.
#[derive(Debug, Clone, Eq, PartialEq)]
struct AbsentValueSymbol(String);

impl Default for AbsentValueSymbol {
    fn default() -> Self {
        Self(symbols::shade::EMPTY.to_string())
    }
}

impl Sparkline<'_> {
    fn render_sparkline(&self, spark_area: Rect, buf: &mut Buffer) {
        if spark_area.is_empty() {
            return;
        }
        let max_height = self
            .max
            .unwrap_or_else(|| self.data.iter().filter_map(|s| s.value).max().unwrap_or(1));

        match self.marker {
            Marker::Braille | Marker::Octant => {
                self.render_2_column(spark_area, buf, 4, max_height);
            }
            Marker::Sextant => self.render_2_column(spark_area, buf, 3, max_height),
            Marker::Quadrant => self.render_2_column(spark_area, buf, 2, max_height),
            Marker::HalfBlock => self.render_1_column(spark_area, buf, 2, max_height),
            Marker::Block | Marker::Dot | Marker::Custom(_) => {
                self.render_1_column(spark_area, buf, 1, max_height);
            }
            Marker::Bar | _ => self.render_1_column(spark_area, buf, 8, max_height),
        }
    }

    fn render_1_column(&self, spark_area: Rect, buf: &mut Buffer, sub_rows: u16, max_height: u64) {
        let max_index = min(spark_area.width as usize, self.data.len());
        for (i, item) in self.data.iter().take(max_index).enumerate() {
            let x = match self.direction {
                RenderDirection::LeftToRight => spark_area.left() + i as u16,
                RenderDirection::RightToLeft => spark_area.right() - i as u16 - 1,
            };

            if let Some(value) = item.value {
                let total_ticks =
                    Self::scale_height(value, max_height, spark_area.height, sub_rows);
                let style = self.style.patch(item.style.unwrap_or_default());

                for j in 0..spark_area.height {
                    let y = spark_area.top() + spark_area.height - 1 - j;
                    let ticks = Self::cell_ticks(total_ticks, j, sub_rows);
                    let cell = &mut buf[(x, y)];
                    self.set_1_column_cell(cell, ticks, style);
                }
            } else {
                let style = self.style.patch(self.absent_value_style);
                for j in 0..spark_area.height {
                    let y = spark_area.top() + j;
                    buf[(x, y)]
                        .set_symbol(&self.absent_value_symbol.0)
                        .set_style(style);
                }
            }
        }
    }

    fn set_1_column_cell(&self, cell: &mut Cell, ticks: u16, style: Style) {
        match self.marker {
            Marker::HalfBlock => {
                let ch = match ticks {
                    0 => ' ',
                    1 => symbols::half_block::LOWER,
                    _ => symbols::half_block::FULL,
                };
                cell.set_char(ch).set_style(style);
            }
            Marker::Block => {
                let symbol = if ticks > 0 { symbols::block::FULL } else { " " };
                cell.set_symbol(symbol).set_style(style);
            }
            Marker::Dot => {
                let symbol = if ticks > 0 { symbols::DOT } else { " " };
                cell.set_symbol(symbol).set_style(style);
            }
            Marker::Custom(c) => {
                let ch = if ticks > 0 { c } else { ' ' };
                cell.set_char(ch).set_style(style);
            }
            Marker::Bar | _ => {
                let symbol = self.symbol_for_height(ticks);
                cell.set_symbol(symbol).set_style(style);
            }
        }
    }

    fn render_2_column(&self, spark_area: Rect, buf: &mut Buffer, sub_rows: u16, max_height: u64) {
        for col in 0..spark_area.width {
            let x = spark_area.left() + col;
            // 2-column markers place 2 data points per terminal column.
            // For RightToLeft, the right sub-column gets the earlier data index to maintain
            // consistent right-to-left data progression within the cell.
            let (idx_left, idx_right) = match self.direction {
                RenderDirection::LeftToRight => {
                    let base = 2 * (col as usize);
                    (base, base + 1)
                }
                RenderDirection::RightToLeft => {
                    let dist = (spark_area.width - 1 - col) as usize;
                    (2 * dist + 1, 2 * dist)
                }
            };

            let left_item = self.data.get(idx_left);
            let right_item = self.data.get(idx_right);

            if left_item.is_none() && right_item.is_none() {
                continue;
            }

            let left_is_absent = left_item.is_some_and(|item| item.value.is_none());
            let right_is_absent = right_item.is_some_and(|item| item.value.is_none());

            if left_is_absent
                && (right_item.is_none() || right_is_absent)
                && self.absent_value_symbol.0 != symbols::shade::EMPTY
            {
                let style = self.style.patch(self.absent_value_style);
                for j in 0..spark_area.height {
                    let y = spark_area.top() + j;
                    buf[(x, y)]
                        .set_symbol(&self.absent_value_symbol.0)
                        .set_style(style);
                }
                continue;
            }

            let item_data = |item: Option<&SparklineBar>| match item {
                Some(SparklineBar {
                    value: Some(v),
                    style,
                }) => (
                    Self::scale_height(*v, max_height, spark_area.height, sub_rows),
                    *style,
                ),
                Some(SparklineBar { value: None, .. }) => (0, Some(self.absent_value_style)),
                None => (0, None),
            };

            let (t_left, style_left) = item_data(left_item);
            let (t_right, style_right) = item_data(right_item);

            let mut cell_style = self.style;
            if let Some(s) = style_left {
                cell_style = cell_style.patch(s);
            }
            if let Some(s) = style_right {
                cell_style = cell_style.patch(s);
            }

            for j in 0..spark_area.height {
                let y = spark_area.top() + spark_area.height - 1 - j;
                let ticks_left = Self::cell_ticks(t_left, j, sub_rows);
                let ticks_right = Self::cell_ticks(t_right, j, sub_rows);

                // Construct row-major bit patterns matching Unicode Braille/pixel tables
                // (bit = x + 2 * y, where y=0 is top row). Bars fill from bottom up.
                let mut pattern = 0u8;
                for r in 0..ticks_left {
                    let dy = (sub_rows - 1) - r;
                    pattern |= 1u8 << (2 * dy);
                }
                for r in 0..ticks_right {
                    let dy = (sub_rows - 1) - r;
                    pattern |= 1u8 << (1 + 2 * dy);
                }

                let ch = match self.marker {
                    Marker::Octant => OCTANTS[pattern as usize],
                    Marker::Sextant => SEXTANTS[pattern as usize],
                    Marker::Quadrant => QUADRANTS[pattern as usize],
                    Marker::Braille | _ => BRAILLE[pattern as usize],
                };

                buf[(x, y)].set_char(ch).set_style(cell_style);
            }
        }
    }

    const fn symbol_for_height(&self, height: u16) -> &str {
        match height {
            0 => self.bar_set.empty,
            1 => self.bar_set.one_eighth,
            2 => self.bar_set.one_quarter,
            3 => self.bar_set.three_eighths,
            4 => self.bar_set.half,
            5 => self.bar_set.five_eighths,
            6 => self.bar_set.three_quarters,
            7 => self.bar_set.seven_eighths,
            _ => self.bar_set.full,
        }
    }

    /// Calculates the number of sub-row ticks that fall into a specific vertical cell row
    /// (indexed 0 from the bottom of the spark area).
    #[allow(clippy::cast_lossless)]
    const fn cell_ticks(total_ticks: u64, row_from_bottom: u16, sub_rows: u16) -> u16 {
        let row_base = (row_from_bottom as u64) * (sub_rows as u64);
        let row_ceiling = row_base + (sub_rows as u64);
        if total_ticks >= row_ceiling {
            sub_rows
        } else if total_ticks <= row_base {
            0
        } else {
            (total_ticks - row_base) as u16
        }
    }

    fn scale_height(value: u64, max: u64, max_height: u16, sub_rows: u16) -> u64 {
        if max == 0 {
            return 0;
        }

        let max_ticks = u128::from(max_height) * u128::from(sub_rows);
        let ticks = u128::from(value) * max_ticks / u128::from(max);
        ticks.min(max_ticks) as u64
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use ratatui_core::buffer::Cell;
    use ratatui_core::style::{Color, Modifier, Stylize};
    use strum::ParseError;

    use super::*;

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

    #[test]
    fn it_can_be_created_from_vec_of_u64() {
        let data = vec![1_u64, 2, 3];
        let spark_data = Sparkline::default().data(data).data;
        let expected = vec![
            SparklineBar::from(1),
            SparklineBar::from(2),
            SparklineBar::from(3),
        ];
        assert_eq!(spark_data, expected);
    }

    #[test]
    fn it_can_be_created_from_vec_of_option_u64() {
        let data = vec![Some(1_u64), None, Some(3)];
        let spark_data = Sparkline::default().data(data).data;
        let expected = vec![
            SparklineBar::from(1),
            SparklineBar::from(None),
            SparklineBar::from(3),
        ];
        assert_eq!(spark_data, expected);
    }

    #[test]
    fn it_can_be_created_from_array_of_u64() {
        let data = [1_u64, 2, 3];
        let spark_data = Sparkline::default().data(data).data;
        let expected = vec![
            SparklineBar::from(1),
            SparklineBar::from(2),
            SparklineBar::from(3),
        ];
        assert_eq!(spark_data, expected);
    }

    #[test]
    fn it_can_be_created_from_array_of_option_u64() {
        let data = [Some(1_u64), None, Some(3)];
        let spark_data = Sparkline::default().data(data).data;
        let expected = vec![
            SparklineBar::from(1),
            SparklineBar::from(None),
            SparklineBar::from(3),
        ];
        assert_eq!(spark_data, expected);
    }

    #[test]
    fn it_can_be_created_from_slice_of_u64() {
        let data = vec![1_u64, 2, 3];
        let spark_data = Sparkline::default().data(&data).data;
        let expected = vec![
            SparklineBar::from(1),
            SparklineBar::from(2),
            SparklineBar::from(3),
        ];
        assert_eq!(spark_data, expected);
    }

    #[test]
    fn it_can_be_created_from_slice_of_option_u64() {
        let data = vec![Some(1_u64), None, Some(3)];
        let spark_data = Sparkline::default().data(&data).data;
        let expected = vec![
            SparklineBar::from(1),
            SparklineBar::from(None),
            SparklineBar::from(3),
        ];
        assert_eq!(spark_data, expected);
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
        let widget = Sparkline::default().data([0, 0, 0]);
        let buffer = render(widget, 6);
        assert_eq!(buffer, Buffer::with_lines(["   xxx"]));
    }

    #[test]
    fn it_does_not_panic_if_max_is_set_to_zero() {
        // see https://github.com/rust-lang/rust-clippy/issues/13191
        let widget = Sparkline::default().data([0, 1, 2]).max(0);
        let buffer = render(widget, 6);
        assert_eq!(buffer, Buffer::with_lines(["   xxx"]));
    }

    #[test]
    fn it_draws() {
        let widget = Sparkline::default().data([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let buffer = render(widget, 12);
        assert_eq!(buffer, Buffer::with_lines([" ▁▂▃▄▅▆▇█xxx"]));
    }

    #[test]
    fn it_draws_double_height() {
        let widget = Sparkline::default().data([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let area = Rect::new(0, 0, 12, 2);
        let mut buffer = Buffer::filled(area, Cell::new("x"));
        widget.render(area, &mut buffer);
        assert_eq!(buffer, Buffer::with_lines(["     ▂▄▆█xxx", " ▂▄▆█████xxx"]));
    }

    #[test]
    fn render_handles_u64_max_value() {
        let widget = Sparkline::default().data([u64::MAX]).max(u64::MAX);
        let area = Rect::new(0, 0, 1, 3);
        let mut buffer = Buffer::empty(area);

        widget.render(area, &mut buffer);

        assert_eq!(buffer, Buffer::with_lines(["█"; 3]));
    }

    #[test]
    fn render_keeps_integer_precision_for_large_values() {
        let widget = Sparkline::default().data([u64::MAX - 1]).max(u64::MAX);
        let area = Rect::new(0, 0, 1, 1);
        let mut buffer = Buffer::empty(area);

        widget.render(area, &mut buffer);

        assert_eq!(buffer, Buffer::with_lines(["▇"]));
    }

    #[test]
    fn it_renders_left_to_right() {
        let widget = Sparkline::default()
            .data([0, 1, 2, 3, 4, 5, 6, 7, 8])
            .direction(RenderDirection::LeftToRight);
        let buffer = render(widget, 12);
        assert_eq!(buffer, Buffer::with_lines([" ▁▂▃▄▅▆▇█xxx"]));
    }

    #[test]
    fn it_renders_right_to_left() {
        let widget = Sparkline::default()
            .data([0, 1, 2, 3, 4, 5, 6, 7, 8])
            .direction(RenderDirection::RightToLeft);
        let buffer = render(widget, 12);
        assert_eq!(buffer, Buffer::with_lines(["xxx█▇▆▅▄▃▂▁ "]));
    }

    #[test]
    fn it_renders_with_absent_value_style() {
        let widget = Sparkline::default()
            .absent_value_style(Style::default().fg(Color::Red))
            .absent_value_symbol(symbols::shade::FULL)
            .data([
                None,
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
                Some(8),
            ]);
        let buffer = render(widget, 12);
        let mut expected = Buffer::with_lines(["█▁▂▃▄▅▆▇█xxx"]);
        expected.set_style(Rect::new(0, 0, 1, 1), Style::default().fg(Color::Red));
        assert_eq!(buffer, expected);
    }

    #[test]
    fn it_renders_with_absent_value_style_double_height() {
        let widget = Sparkline::default()
            .absent_value_style(Style::default().fg(Color::Red))
            .absent_value_symbol(symbols::shade::FULL)
            .data([
                None,
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
                Some(8),
            ]);
        let area = Rect::new(0, 0, 12, 2);
        let mut buffer = Buffer::filled(area, Cell::new("x"));
        widget.render(area, &mut buffer);
        let mut expected = Buffer::with_lines(["█    ▂▄▆█xxx", "█▂▄▆█████xxx"]);
        expected.set_style(Rect::new(0, 0, 1, 2), Style::default().fg(Color::Red));
        assert_eq!(buffer, expected);
    }

    #[test]
    fn it_renders_with_custom_absent_value_style() {
        let widget = Sparkline::default().absent_value_symbol('*').data([
            None,
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
        ]);
        let buffer = render(widget, 12);
        let expected = Buffer::with_lines(["*▁▂▃▄▅▆▇█xxx"]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn it_renders_with_custom_bar_styles() {
        let widget = Sparkline::default().data(vec![
            SparklineBar::from(Some(0)).style(Some(Style::default().fg(Color::Red))),
            SparklineBar::from(Some(1)).style(Some(Style::default().fg(Color::Red))),
            SparklineBar::from(Some(2)).style(Some(Style::default().fg(Color::Red))),
            SparklineBar::from(Some(3)).style(Some(Style::default().fg(Color::Green))),
            SparklineBar::from(Some(4)).style(Some(Style::default().fg(Color::Green))),
            SparklineBar::from(Some(5)).style(Some(Style::default().fg(Color::Green))),
            SparklineBar::from(Some(6)).style(Some(Style::default().fg(Color::Blue))),
            SparklineBar::from(Some(7)).style(Some(Style::default().fg(Color::Blue))),
            SparklineBar::from(Some(8)).style(Some(Style::default().fg(Color::Blue))),
        ]);
        let buffer = render(widget, 12);
        let mut expected = Buffer::with_lines([" ▁▂▃▄▅▆▇█xxx"]);
        expected.set_style(Rect::new(0, 0, 3, 1), Style::default().fg(Color::Red));
        expected.set_style(Rect::new(3, 0, 3, 1), Style::default().fg(Color::Green));
        expected.set_style(Rect::new(6, 0, 3, 1), Style::default().fg(Color::Blue));
        assert_eq!(buffer, expected);
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

    #[test]
    fn render_in_minimal_buffer() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));
        let sparkline = Sparkline::default()
            .data([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
            .max(10);
        // This should not panic, even if the buffer is too small to render the sparkline.
        sparkline.render(buffer.area, &mut buffer);
        assert_eq!(buffer, Buffer::with_lines([" "]));
    }

    #[test]
    fn render_in_zero_size_buffer() {
        let mut buffer = Buffer::empty(Rect::ZERO);
        let sparkline = Sparkline::default()
            .data([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
            .max(10);
        // This should not panic, even if the buffer has zero size.
        sparkline.render(buffer.area, &mut buffer);
    }

    #[test]
    fn marker_setter() {
        let sparkline = Sparkline::default().marker(Marker::Braille);
        assert_eq!(sparkline.marker, Marker::Braille);
    }

    #[test]
    fn it_renders_braille() {
        let widget = Sparkline::default()
            .marker(Marker::Braille)
            .data([0, 1, 2, 3, 4, 3, 2, 1, 0])
            .max(4);
        let buffer = render(widget, 6);
        assert_eq!(buffer, Buffer::with_lines(["⢀⣴⣷⣄⠀x"]));
    }

    #[test]
    fn it_renders_braille_double_height() {
        let widget = Sparkline::default()
            .marker(Marker::Braille)
            .data([0, 1, 2, 3, 4, 5, 6, 7, 8])
            .max(8);
        let area = Rect::new(0, 0, 6, 2);
        let mut buffer = Buffer::filled(area, Cell::new("x"));
        widget.render(area, &mut buffer);
        assert_eq!(buffer, Buffer::with_lines(["⠀⠀⢀⣴⡇x", "⢀⣴⣿⣿⡇x"]));
    }

    #[test]
    fn it_renders_braille_right_to_left() {
        let widget = Sparkline::default()
            .marker(Marker::Braille)
            .direction(RenderDirection::RightToLeft)
            .data([0, 1, 2, 3, 4])
            .max(4);
        let buffer = render(widget, 4);
        assert_eq!(buffer, Buffer::with_lines(["x⢸⣦⡀"]));
    }

    #[test]
    fn it_renders_braille_with_absent_values() {
        let widget = Sparkline::default()
            .marker(Marker::Braille)
            .absent_value_style(Style::default().fg(Color::Red))
            .absent_value_symbol(symbols::shade::FULL)
            .data([None, None, Some(1), Some(4)])
            .max(4);
        let buffer = render(widget, 3);
        let mut expected = Buffer::with_lines(["█⣸x"]);
        expected.set_style(Rect::new(0, 0, 1, 1), Style::default().fg(Color::Red));
        assert_eq!(buffer, expected);
    }

    #[test]
    fn it_renders_braille_with_custom_bar_styles() {
        let widget = Sparkline::default()
            .marker(Marker::Braille)
            .data([
                SparklineBar::from(1).style(Some(Style::default().fg(Color::Red))),
                SparklineBar::from(4).style(Some(Style::default().fg(Color::Blue))),
            ])
            .max(4);
        let buffer = render(widget, 2);
        let mut expected = Buffer::with_lines(["⣸x"]);
        expected.set_style(Rect::new(0, 0, 1, 1), Style::default().fg(Color::Blue));
        assert_eq!(buffer, expected);
    }

    #[test]
    fn it_renders_quadrant() {
        let widget = Sparkline::default()
            .marker(Marker::Quadrant)
            .data([0, 1, 2, 2, 1, 0])
            .max(2);
        let buffer = render(widget, 4);
        assert_eq!(buffer, Buffer::with_lines(["▗█▖x"]));
    }

    #[test]
    fn it_renders_sextant() {
        let widget = Sparkline::default()
            .marker(Marker::Sextant)
            .data([0, 1, 2, 3, 3, 2, 1, 0])
            .max(3);
        let buffer = render(widget, 5);
        assert_eq!(buffer, Buffer::with_lines(["🬞🬻🬺🬏x"]));
    }

    #[test]
    fn it_renders_octant() {
        let widget = Sparkline::default()
            .marker(Marker::Octant)
            .data([0, 1, 2, 3, 4, 3, 2, 1, 0])
            .max(4);
        let buffer = render(widget, 6);
        assert_eq!(buffer, Buffer::with_lines(["𜺠𜷡𜷤𜶻 x"]));
    }

    #[test]
    fn it_renders_half_block() {
        let widget = Sparkline::default()
            .marker(Marker::HalfBlock)
            .data([0, 1, 2])
            .max(2);
        let buffer = render(widget, 4);
        assert_eq!(buffer, Buffer::with_lines([" ▄█x"]));
    }

    #[test]
    fn it_renders_block() {
        let widget = Sparkline::default()
            .marker(Marker::Block)
            .data([0, 1, 0, 1])
            .max(1);
        let buffer = render(widget, 5);
        assert_eq!(buffer, Buffer::with_lines([" █ █x"]));
    }

    #[test]
    fn it_renders_dot() {
        let widget = Sparkline::default()
            .marker(Marker::Dot)
            .data([0, 1, 0, 1])
            .max(1);
        let buffer = render(widget, 5);
        assert_eq!(buffer, Buffer::with_lines([" • •x"]));
    }

    #[test]
    fn it_renders_custom_marker() {
        let widget = Sparkline::default()
            .marker(Marker::Custom('*'))
            .data([0, 1, 0, 1])
            .max(1);
        let buffer = render(widget, 5);
        assert_eq!(buffer, Buffer::with_lines([" * *x"]));
    }
}
