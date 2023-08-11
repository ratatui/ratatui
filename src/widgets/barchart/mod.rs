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

    pub fn value_style(mut self, style: Style) -> BarChart<'a> {
        self.value_style = style;
        self
    }

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

    fn render_bars(&self, buf: &mut Buffer, bars_area: Rect, max: u64) {
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

                    let bar_style = bar.style.patch(self.bar_style);

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
        for group in self.data.into_iter() {
            // print group labels under the bars or the previous labels
            if let Some(mut label) = group.label {
                label.patch_style(self.label_style);
                let label_max_width = group.bars.len() as u16 * self.bar_width
                    + (group.bars.len() as u16 - 1) * self.bar_gap;

                buf.set_line(
                    bar_x + (label_max_width.saturating_sub(label.width() as u16) >> 1),
                    area.bottom() - 1,
                    &label,
                    label_max_width,
                );
            }

            // print the bar values and numbers
            for bar in group.bars.into_iter() {
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

        // remove invisible groups and bars, since we don't need to print them
        self.remove_invisible_groups_and_bars(area.width);

        let bars_area = Rect {
            height: area.height - label_height,
            ..area
        };
        self.render_bars(buf, bars_area, max);

        self.render_labels_and_values(area, buf, label_height);
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
            " G1      G2      G3  ",
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
            " G1      G2     G",
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
            " G1      G2     ",
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
        let expected = Buffer::with_lines(vec!["  █", "█ █", " G "]);
        assert_buffer_eq!(buffer, expected);
    }
}
