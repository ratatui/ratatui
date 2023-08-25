use crate::prelude::*;

mod bar;
mod bar_group;

pub use bar::Bar;
pub use bar_group::BarGroup;

use super::{Block, Widget};

/// Display multiple bars in a single widgets
///
/// # Examples
/// The following example creates a BarChart with two groups of bars.
/// The first group is added by an array slice (&[(&str, u64)]).
/// The second group is added by a slice of Groups (&[BarGroup]).
/// ```
/// # use ratatui::{prelude::*, widgets::*};
/// BarChart::default()
///     .block(Block::default().title("BarChart").borders(Borders::ALL))
///     .bar_width(3)
///     .bar_gap(1)
///     .group_gap(3)
///     .bar_style(Style::new().yellow().on_red())
///     .value_style(Style::new().red().bold())
///     .label_style(Style::new().white())
///     .data(&[("B0", 0), ("B1", 2), ("B2", 4), ("B3", 3)])
///     .data(BarGroup::default().bars(&[Bar::default().value(10), Bar::default().value(20)]))
///     .max(4);
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BarChart<'a> {
    /// Block to wrap the widget in
    block: Option<Block<'a>>,
    /// The width of each bar
    bar_width: u16,
    /// The gap between each bar
    bar_gap: u16,
    /// The gap between each group
    group_gap: u16,
    /// Set of symbols used to display the data
    bar_set: symbols::bar::Set,
    /// Style of the bars
    bar_style: Style,
    /// Style of the values printed at the bottom of each bar
    value_style: Style,
    /// Style of the labels printed under each bar
    label_style: Style,
    /// Style for the widget
    style: Style,
    /// vector of groups containing bars
    data: Vec<BarGroup<'a>>,
    /// Value necessary for a bar to reach the maximum height (if no value is specified,
    /// the maximum value in the data is taken as reference)
    max: Option<u64>,
    /// direction of the bars
    direction: Direction,
}

impl<'a> Default for BarChart<'a> {
    fn default() -> BarChart<'a> {
        BarChart {
            block: None,
            max: None,
            data: Vec::new(),
            bar_style: Style::default(),
            bar_width: 1,
            bar_gap: 1,
            value_style: Style::default(),
            label_style: Style::default(),
            group_gap: 0,
            bar_set: symbols::bar::NINE_LEVELS,
            style: Style::default(),
            direction: Direction::Vertical,
        }
    }
}

impl<'a> BarChart<'a> {
    /// Add group of bars to the BarChart
    /// # Examples
    /// The following example creates a BarChart with two groups of bars.
    /// The first group is added by an array slice (&[(&str, u64)]).
    /// The second group is added by a BarGroup instance.
    /// ```
    /// # use ratatui::{prelude::*, widgets::*};
    ///
    /// BarChart::default()
    ///        .data(&[("B0", 0), ("B1", 2), ("B2", 4), ("B3", 3)])
    ///        .data(BarGroup::default().bars(&[Bar::default().value(10), Bar::default().value(20)]));
    /// ```
    pub fn data(mut self, data: impl Into<BarGroup<'a>>) -> BarChart<'a> {
        let group: BarGroup = data.into();
        if !group.bars.is_empty() {
            self.data.push(group);
        }
        self
    }

    pub fn block(mut self, block: Block<'a>) -> BarChart<'a> {
        self.block = Some(block);
        self
    }

    pub fn max(mut self, max: u64) -> BarChart<'a> {
        self.max = Some(max);
        self
    }

    /// Set the default style of the bar.
    /// It is also possible to set individually the style of each Bar.
    /// In this case the default style will be patched by the individual style
    pub fn bar_style(mut self, style: Style) -> BarChart<'a> {
        self.bar_style = style;
        self
    }

    pub fn bar_width(mut self, width: u16) -> BarChart<'a> {
        self.bar_width = width;
        self
    }

    pub fn bar_gap(mut self, gap: u16) -> BarChart<'a> {
        self.bar_gap = gap;
        self
    }

    pub fn bar_set(mut self, bar_set: symbols::bar::Set) -> BarChart<'a> {
        self.bar_set = bar_set;
        self
    }

    /// Set the default value style of the bar.
    /// It is also possible to set individually the value style of each Bar.
    /// In this case the default value style will be patched by the individual value style
    pub fn value_style(mut self, style: Style) -> BarChart<'a> {
        self.value_style = style;
        self
    }

    /// Set the default label style of the groups and bars.
    /// It is also possible to set individually the label style of each Bar or Group.
    /// In this case the default label style will be patched by the individual label style
    pub fn label_style(mut self, style: Style) -> BarChart<'a> {
        self.label_style = style;
        self
    }

    pub fn group_gap(mut self, gap: u16) -> BarChart<'a> {
        self.group_gap = gap;
        self
    }

    pub fn style(mut self, style: Style) -> BarChart<'a> {
        self.style = style;
        self
    }

    /// Set the direction of the bars
    pub fn direction(mut self, direction: Direction) -> BarChart<'a> {
        self.direction = direction;
        self
    }
}

impl<'a> BarChart<'a> {
    /// Check the bars, which fits inside the available space and removes
    /// the bars and the groups, which are outside of the available space.
    fn remove_invisible_groups_and_bars(&mut self, mut width: u16) {
        for group_index in 0..self.data.len() {
            let n_bars = self.data[group_index].bars.len() as u16;
            let group_width = n_bars * self.bar_width + n_bars.saturating_sub(1) * self.bar_gap;

            if width > group_width {
                width = width.saturating_sub(group_width + self.group_gap + self.bar_gap);
            } else {
                let max_bars = (width + self.bar_gap) / (self.bar_width + self.bar_gap);
                if max_bars == 0 {
                    self.data.truncate(group_index);
                } else {
                    self.data[group_index].bars.truncate(max_bars as usize);
                    self.data.truncate(group_index + 1);
                }
                break;
            }
        }
    }

    /// Get the number of lines needed for the labels.
    ///
    /// The number of lines depends on whether we need to print the bar labels and/or the group
    /// labels.
    /// - If there are no labels, return 0.
    /// - If there are only bar labels, return 1.
    /// - If there are only group labels, return 1.
    /// - If there are both bar and group labels, return 2.
    fn label_height(&self) -> u16 {
        let has_group_labels = self.data.iter().any(|e| e.label.is_some());
        let has_data_labels = self
            .data
            .iter()
            .any(|e| e.bars.iter().any(|e| e.label.is_some()));

        // convert true to 1 and false to 0 and add the two values
        u16::from(has_group_labels) + u16::from(has_data_labels)
    }

    /// renders the block if there is one and updates the area to the inner area
    fn render_block(&mut self, area: &mut Rect, buf: &mut Buffer) {
        if let Some(block) = self.block.take() {
            let inner_area = block.inner(*area);
            block.render(*area, buf);
            *area = inner_area
        }
    }

    fn render_horizontal_bars(self, buf: &mut Buffer, bars_area: Rect, max: u64) {
        // convert the bar values to ratatui::symbols::bar::Set
        let groups: Vec<Vec<u16>> = self
            .data
            .iter()
            .map(|group| {
                group
                    .bars
                    .iter()
                    .map(|bar| (bar.value * u64::from(bars_area.width) / max) as u16)
                    .collect()
            })
            .collect();

        // print all visible bars
        let mut bar_y = bars_area.top();
        for (group_data, mut group) in groups.into_iter().zip(self.data) {
            let bars = std::mem::take(&mut group.bars);

            for (bar_length, bar) in group_data.into_iter().zip(bars) {
                let bar_style = self.bar_style.patch(bar.style);

                for y in 0..self.bar_width {
                    let bar_y = bar_y + y;
                    for x in 0..bars_area.width {
                        let symbol = if x < bar_length {
                            self.bar_set.full
                        } else {
                            self.bar_set.empty
                        };
                        buf.get_mut(bars_area.left() + x, bar_y)
                            .set_symbol(symbol)
                            .set_style(bar_style);
                    }
                }

                let bar_value_area = Rect {
                    y: bar_y + (self.bar_width >> 1),
                    ..bars_area
                };
                bar.render_value_with_different_styles(
                    buf,
                    bar_value_area,
                    bar_length as usize,
                    self.value_style,
                    self.bar_style,
                );

                bar_y += self.bar_gap + self.bar_width;
            }

            // if group_gap is zero, then there is no place to print the group label
            // check also if the group label is still inside the visible area
            let label_y = bar_y - self.bar_gap;
            if self.group_gap > 0 && label_y < bars_area.bottom() {
                let label_rect = Rect {
                    y: label_y,
                    ..bars_area
                };
                group.render_label(buf, label_rect, self.label_style);
                bar_y += self.group_gap;
            }
        }
    }

    fn render_vertical_bars(&self, buf: &mut Buffer, bars_area: Rect, max: u64) {
        // convert the bar values to ratatui::symbols::bar::Set
        let mut groups: Vec<Vec<u64>> = self
            .data
            .iter()
            .map(|group| {
                group
                    .bars
                    .iter()
                    .map(|bar| bar.value * u64::from(bars_area.height) * 8 / max)
                    .collect()
            })
            .collect();

        // print all visible bars (without labels and values)
        for j in (0..bars_area.height).rev() {
            let mut bar_x = bars_area.left();
            for (group_data, group) in groups.iter_mut().zip(&self.data) {
                for (d, bar) in group_data.iter_mut().zip(&group.bars) {
                    let symbol = match d {
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

                    let bar_style = self.bar_style.patch(bar.style);

                    for x in 0..self.bar_width {
                        buf.get_mut(bar_x + x, bars_area.top() + j)
                            .set_symbol(symbol)
                            .set_style(bar_style);
                    }

                    if *d > 8 {
                        *d -= 8;
                    } else {
                        *d = 0;
                    }
                    bar_x += self.bar_gap + self.bar_width;
                }
                bar_x += self.group_gap;
            }
        }
    }

    /// get the maximum data value. the returned value is always greater equal 1
    fn maximum_data_value(&self) -> u64 {
        self.max
            .unwrap_or_else(|| {
                self.data
                    .iter()
                    .map(|group| group.max().unwrap_or_default())
                    .max()
                    .unwrap_or_default()
            })
            .max(1u64)
    }

    fn render_labels_and_values(self, area: Rect, buf: &mut Buffer, label_height: u16) {
        // print labels and values in one go
        let mut bar_x = area.left();
        let bar_y = area.bottom() - label_height - 1;
        for mut group in self.data.into_iter() {
            if group.bars.is_empty() {
                continue;
            }
            let bars = std::mem::take(&mut group.bars);
            // print group labels under the bars or the previous labels
            let label_max_width =
                bars.len() as u16 * (self.bar_width + self.bar_gap) - self.bar_gap;
            let group_area = Rect {
                x: bar_x,
                y: area.bottom() - 1,
                width: label_max_width,
                height: 1,
            };
            group.render_label(buf, group_area, self.label_style);

            // print the bar values and numbers
            for bar in bars.into_iter() {
                bar.render_label_and_value(
                    buf,
                    self.bar_width,
                    bar_x,
                    bar_y,
                    self.value_style,
                    self.label_style,
                );

                bar_x += self.bar_gap + self.bar_width;
            }
            bar_x += self.group_gap;
        }
    }
}

impl<'a> Widget for BarChart<'a> {
    fn render(mut self, mut area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);

        self.render_block(&mut area, buf);

        if self.data.is_empty() {
            return;
        }

        let label_height = self.label_height();
        if area.height <= label_height {
            return;
        }

        let max = self.maximum_data_value();

        match self.direction {
            Direction::Horizontal => {
                // remove invisible groups and bars, since we don't need to print them
                self.remove_invisible_groups_and_bars(area.height);
                self.render_horizontal_bars(buf, area, max);
            }
            Direction::Vertical => {
                // remove invisible groups and bars, since we don't need to print them
                self.remove_invisible_groups_and_bars(area.width);
                let bars_area = Rect {
                    height: area.height - label_height,
                    ..area
                };
                self.render_vertical_bars(buf, bars_area, max);
                self.render_labels_and_values(area, buf, label_height);
            }
        }
    }
}

impl<'a> Styled for BarChart<'a> {
    type Item = BarChart<'a>;
    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self {
        self.style(style)
    }
}

#[cfg(test)]
mod tests {
    use itertools::iproduct;

    use super::*;
    use crate::{
        assert_buffer_eq,
        widgets::{BorderType, Borders},
    };

    #[test]
    fn default() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 3));
        let widget = BarChart::default();
        widget.render(buffer.area, &mut buffer);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["          "; 3]));
    }

    #[test]
    fn data() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));
        let widget = BarChart::default().data(&[("foo", 1), ("bar", 2)]);
        widget.render(buffer.area, &mut buffer);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![
                "  █            ",
                "█ █            ",
                "f b            ",
            ])
        );
    }

    #[test]
    fn block() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 5));
        let block = Block::default()
            .title("Block")
            .border_type(BorderType::Double)
            .borders(Borders::ALL);
        let widget = BarChart::default()
            .data(&[("foo", 1), ("bar", 2)])
            .block(block);
        widget.render(buffer.area, &mut buffer);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![
                "╔Block════════╗",
                "║  █          ║",
                "║█ █          ║",
                "║f b          ║",
                "╚═════════════╝",
            ])
        );
    }

    #[test]
    fn max() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));
        let without_max = BarChart::default().data(&[("foo", 1), ("bar", 2), ("baz", 100)]);
        without_max.render(buffer.area, &mut buffer);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![
                "    █          ",
                "    █          ",
                "f b b          ",
            ])
        );
        let with_max = BarChart::default()
            .data(&[("foo", 1), ("bar", 2), ("baz", 100)])
            .max(2);
        with_max.render(buffer.area, &mut buffer);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![
                "  █ █          ",
                "█ █ █          ",
                "f b b          ",
            ])
        );
    }

    #[test]
    fn bar_style() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));
        let widget = BarChart::default()
            .data(&[("foo", 1), ("bar", 2)])
            .bar_style(Style::new().red());
        widget.render(buffer.area, &mut buffer);
        let mut expected = Buffer::with_lines(vec![
            "  █            ",
            "█ █            ",
            "f b            ",
        ]);
        for (x, y) in iproduct!([0, 2], [0, 1]) {
            expected.get_mut(x, y).set_fg(Color::Red);
        }
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn bar_width() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));
        let widget = BarChart::default()
            .data(&[("foo", 1), ("bar", 2)])
            .bar_width(3);
        widget.render(buffer.area, &mut buffer);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![
                "    ███        ",
                "█1█ █2█        ",
                "foo bar        ",
            ])
        );
    }

    #[test]
    fn bar_gap() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));
        let widget = BarChart::default()
            .data(&[("foo", 1), ("bar", 2)])
            .bar_gap(2);
        widget.render(buffer.area, &mut buffer);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![
                "   █           ",
                "█  █           ",
                "f  b           ",
            ])
        );
    }

    #[test]
    fn bar_set() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));
        let widget = BarChart::default()
            .data(&[("foo", 0), ("bar", 1), ("baz", 3)])
            .bar_set(symbols::bar::THREE_LEVELS);
        widget.render(buffer.area, &mut buffer);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![
                "    █          ",
                "  ▄ █          ",
                "f b b          ",
            ])
        );
    }

    #[test]
    fn bar_set_nine_levels() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 18, 3));
        let widget = BarChart::default()
            .data(&[
                ("a", 0),
                ("b", 1),
                ("c", 2),
                ("d", 3),
                ("e", 4),
                ("f", 5),
                ("g", 6),
                ("h", 7),
                ("i", 8),
            ])
            .bar_set(symbols::bar::NINE_LEVELS);
        widget.render(Rect::new(0, 1, 18, 2), &mut buffer);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![
                "                  ",
                "  ▁ ▂ ▃ ▄ ▅ ▆ ▇ █ ",
                "a b c d e f g h i ",
            ])
        );
    }

    #[test]
    fn value_style() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));
        let widget = BarChart::default()
            .data(&[("foo", 1), ("bar", 2)])
            .bar_width(3)
            .value_style(Style::new().red());
        widget.render(buffer.area, &mut buffer);
        let mut expected = Buffer::with_lines(vec![
            "    ███        ",
            "█1█ █2█        ",
            "foo bar        ",
        ]);
        expected.get_mut(1, 1).set_fg(Color::Red);
        expected.get_mut(5, 1).set_fg(Color::Red);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn label_style() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));
        let widget = BarChart::default()
            .data(&[("foo", 1), ("bar", 2)])
            .label_style(Style::new().red());
        widget.render(buffer.area, &mut buffer);
        let mut expected = Buffer::with_lines(vec![
            "  █            ",
            "█ █            ",
            "f b            ",
        ]);
        expected.get_mut(0, 2).set_fg(Color::Red);
        expected.get_mut(2, 2).set_fg(Color::Red);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn style() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 3));
        let widget = BarChart::default()
            .data(&[("foo", 1), ("bar", 2)])
            .style(Style::new().red());
        widget.render(buffer.area, &mut buffer);
        let mut expected = Buffer::with_lines(vec![
            "  █            ",
            "█ █            ",
            "f b            ",
        ]);
        for (x, y) in iproduct!(0..15, 0..3) {
            expected.get_mut(x, y).set_fg(Color::Red);
        }
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn does_not_render_less_than_two_rows() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 15, 1));
        let widget = BarChart::default().data(&[("foo", 1), ("bar", 2)]);
        widget.render(buffer.area, &mut buffer);
        assert_buffer_eq!(buffer, Buffer::empty(Rect::new(0, 0, 15, 1)));
    }

    fn create_test_barchart<'a>() -> BarChart<'a> {
        BarChart::default()
            .group_gap(2)
            .data(BarGroup::default().label("G1".into()).bars(&[
                Bar::default().value(2),
                Bar::default().value(1),
                Bar::default().value(2),
            ]))
            .data(BarGroup::default().label("G2".into()).bars(&[
                Bar::default().value(1),
                Bar::default().value(2),
                Bar::default().value(1),
            ]))
            .data(BarGroup::default().label("G3".into()).bars(&[
                Bar::default().value(1),
                Bar::default().value(2),
                Bar::default().value(1),
            ]))
    }

    #[test]
    fn test_invisible_groups_and_bars_full() {
        let chart = create_test_barchart();
        // Check that the BarChart is shown in full
        {
            let mut c = chart.clone();
            c.remove_invisible_groups_and_bars(21);
            assert_eq!(c.data.len(), 3);
            assert_eq!(c.data[2].bars.len(), 3);
        }

        let mut buffer = Buffer::empty(Rect::new(0, 0, 21, 3));
        chart.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "█   █     █       █  ",
            "█ █ █   █ █ █   █ █ █",
            "G1      G2      G3   ",
        ]);

        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_invisible_groups_and_bars_missing_last_2_bars() {
        // Last 2 bars of G3 should be out of screen. (screen width is 17)
        let chart = create_test_barchart();

        {
            let mut w = chart.clone();
            w.remove_invisible_groups_and_bars(17);
            assert_eq!(w.data.len(), 3);
            assert_eq!(w.data[2].bars.len(), 1);
        }

        let mut buffer = Buffer::empty(Rect::new(0, 0, 17, 3));
        chart.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "█   █     █      ",
            "█ █ █   █ █ █   █",
            "G1      G2      G",
        ]);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_invisible_groups_and_bars_missing_last_group() {
        // G3 should be out of screen. (screen width is 16)
        let chart = create_test_barchart();

        {
            let mut w = chart.clone();
            w.remove_invisible_groups_and_bars(16);
            assert_eq!(w.data.len(), 2);
            assert_eq!(w.data[1].bars.len(), 3);
        }

        let mut buffer = Buffer::empty(Rect::new(0, 0, 16, 3));
        chart.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "█   █     █     ",
            "█ █ █   █ █ █   ",
            "G1      G2      ",
        ]);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_invisible_groups_and_bars_show_only_1_bar() {
        let chart = create_test_barchart();

        {
            let mut w = chart.clone();
            w.remove_invisible_groups_and_bars(1);
            assert_eq!(w.data.len(), 1);
            assert_eq!(w.data[0].bars.len(), 1);
        }

        let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 3));
        chart.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec!["█", "█", "G"]);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_invisible_groups_and_bars_all_bars_outside_visible_area() {
        let chart = create_test_barchart();

        {
            let mut w = chart.clone();
            w.remove_invisible_groups_and_bars(0);
            assert_eq!(w.data.len(), 0);
        }

        let mut buffer = Buffer::empty(Rect::new(0, 0, 0, 3));
        // Check if the render method panics
        chart.render(buffer.area, &mut buffer);
    }

    #[test]
    fn test_label_height() {
        {
            let barchart = BarChart::default().data(
                BarGroup::default()
                    .label("Group Label".into())
                    .bars(&[Bar::default().value(2).label("Bar Label".into())]),
            );
            assert_eq!(barchart.label_height(), 2);
        }

        {
            let barchart = BarChart::default().data(
                BarGroup::default()
                    .label("Group Label".into())
                    .bars(&[Bar::default().value(2)]),
            );
            assert_eq!(barchart.label_height(), 1);
        }

        {
            let barchart = BarChart::default().data(
                BarGroup::default().bars(&[Bar::default().value(2).label("Bar Label".into())]),
            );
            assert_eq!(barchart.label_height(), 1);
        }

        {
            let barchart =
                BarChart::default().data(BarGroup::default().bars(&[Bar::default().value(2)]));
            assert_eq!(barchart.label_height(), 0);
        }
    }

    #[test]
    fn can_be_stylized() {
        assert_eq!(
            BarChart::default().black().on_white().bold().style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
        )
    }

    #[test]
    fn test_empty_group() {
        let chart = BarChart::default()
            .data(BarGroup::default().label("invisible".into()))
            .data(
                BarGroup::default()
                    .label("G".into())
                    .bars(&[Bar::default().value(1), Bar::default().value(2)]),
            );

        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 3));
        chart.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec!["  █", "█ █", "G  "]);
        assert_buffer_eq!(buffer, expected);
    }

    fn build_test_barchart<'a>() -> BarChart<'a> {
        BarChart::default()
            .data(BarGroup::default().label("G1".into()).bars(&[
                Bar::default().value(2),
                Bar::default().value(3),
                Bar::default().value(4),
            ]))
            .data(BarGroup::default().label("G2".into()).bars(&[
                Bar::default().value(3),
                Bar::default().value(4),
                Bar::default().value(5),
            ]))
            .group_gap(1)
            .direction(Direction::Horizontal)
            .bar_gap(0)
    }

    #[test]
    fn test_horizontal_bars() {
        let chart: BarChart<'_> = build_test_barchart();

        let mut buffer = Buffer::empty(Rect::new(0, 0, 5, 8));
        chart.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "2█   ",
            "3██  ",
            "4███ ",
            "G1   ",
            "3██  ",
            "4███ ",
            "5████",
            "G2   ",
        ]);

        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_horizontal_bars_no_space_for_group_label() {
        let chart: BarChart<'_> = build_test_barchart();

        let mut buffer = Buffer::empty(Rect::new(0, 0, 5, 7));
        chart.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "2█   ",
            "3██  ",
            "4███ ",
            "G1   ",
            "3██  ",
            "4███ ",
            "5████",
        ]);

        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_horizontal_bars_no_space_for_all_bars() {
        let chart: BarChart<'_> = build_test_barchart();

        let mut buffer = Buffer::empty(Rect::new(0, 0, 5, 5));
        chart.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec!["2█   ", "3██  ", "4███ ", "G1   ", "3██  "]);

        assert_buffer_eq!(buffer, expected);
    }

    fn test_horizontal_bars_label_width_greater_than_bar(bar_color: Option<Color>) {
        let mut bar = Bar::default()
            .value(2)
            .text_value("label".into())
            .value_style(Style::default().red());

        if let Some(color) = bar_color {
            bar = bar.style(Style::default().fg(color));
        }

        let chart: BarChart<'_> = BarChart::default()
            .data(BarGroup::default().bars(&[bar, Bar::default().value(5)]))
            .direction(Direction::Horizontal)
            .bar_style(Style::default().yellow())
            .value_style(Style::default().italic())
            .bar_gap(0);

        let mut buffer = Buffer::empty(Rect::new(0, 0, 5, 2));
        chart.render(buffer.area, &mut buffer);

        let mut expected = Buffer::with_lines(vec!["label", "5████"]);

        // first line has a yellow foreground. first cell contains italic "5"
        expected.get_mut(0, 1).modifier.insert(Modifier::ITALIC);
        for x in 0..5 {
            expected.get_mut(x, 1).set_fg(Color::Yellow);
        }

        let expected_color = if let Some(color) = bar_color {
            color
        } else {
            Color::Yellow
        };

        // second line contains the word "label". Since the bar value is 2,
        // then the first 2 characters of "label" are italic red.
        // the rest is white (using the Bar's style).
        let cell = expected.get_mut(0, 0).set_fg(Color::Red);
        cell.modifier.insert(Modifier::ITALIC);
        let cell = expected.get_mut(1, 0).set_fg(Color::Red);
        cell.modifier.insert(Modifier::ITALIC);
        expected.get_mut(2, 0).set_fg(expected_color);
        expected.get_mut(3, 0).set_fg(expected_color);
        expected.get_mut(4, 0).set_fg(expected_color);

        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_horizontal_bars_label_width_greater_than_bar_without_style() {
        test_horizontal_bars_label_width_greater_than_bar(None);
    }

    #[test]
    fn test_horizontal_bars_label_width_greater_than_bar_with_style() {
        test_horizontal_bars_label_width_greater_than_bar(Some(Color::White))
    }

    #[test]
    fn test_group_label_style() {
        let chart: BarChart<'_> = BarChart::default()
            .data(
                BarGroup::default()
                    .label(Span::from("G1").red().into())
                    .bars(&[Bar::default().value(2)]),
            )
            .group_gap(1)
            .direction(Direction::Horizontal)
            .label_style(Style::default().bold().yellow());

        let mut buffer = Buffer::empty(Rect::new(0, 0, 5, 2));
        chart.render(buffer.area, &mut buffer);

        // G1 should have the bold red style
        // bold: because of BarChart::label_style
        // red: is included with the label itself
        let mut expected = Buffer::with_lines(vec!["2████", "G1   "]);
        let cell = expected.get_mut(0, 1).set_fg(Color::Red);
        cell.modifier.insert(Modifier::BOLD);
        let cell = expected.get_mut(1, 1).set_fg(Color::Red);
        cell.modifier.insert(Modifier::BOLD);

        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_group_label_center() {
        let chart: BarChart<'_> = BarChart::default().data(
            BarGroup::default()
                .label(Line::from(Span::from("G")).alignment(Alignment::Center))
                .bars(&[Bar::default().value(2), Bar::default().value(5)]),
        );

        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 3));
        chart.render(buffer.area, &mut buffer);

        let expected = Buffer::with_lines(vec!["  █", "▆ █", " G "]);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn test_group_label_right() {
        let chart: BarChart<'_> = BarChart::default().data(
            BarGroup::default()
                .label(Line::from(Span::from("G")).alignment(Alignment::Right))
                .bars(&[Bar::default().value(2), Bar::default().value(5)]),
        );

        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 3));
        chart.render(buffer.area, &mut buffer);

        let expected = Buffer::with_lines(vec!["  █", "▆ █", "  G"]);
        assert_buffer_eq!(buffer, expected);
    }
}
