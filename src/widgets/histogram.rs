use crate::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols,
    widgets::{Block, Widget},
};
use unicode_width::UnicodeWidthStr;

/// A bar chart specialized for showing histograms
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders, Histogram};
/// # use tui::style::{Style, Color, Modifier};
/// Histogram::default()
///     .block(Block::default().title("Histogram").borders(Borders::ALL))
///     .bar_width(3)
///     .bar_gap(1)
///     .bar_style(Style::default().fg(Color::Yellow).bg(Color::Red))
///     .value_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
///     .label_style(Style::default().fg(Color::White))
///     .data(&[("B0", 0), ("B1", 2), ("B2", 4), ("B3", 3)])
///     .max(4);
/// ```
#[derive(Debug, Clone)]
pub struct Histogram<'a> {
    /// Block to wrap the widget in
    block: Option<Block<'a>>,
    /// The gap between each bar
    bar_gap: u16,
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
    /// Slice of values to plot on the chart
    data: &'a [u64],
    /// each bucket keeps a count of the data points that fall into it
    /// buckets[0] counts items where 0 <= x < bucket_size
    /// buckets[1] counts items where bucket_size <= x < 2*bucket_size
    /// etc.
    buckets: Vec<u64>,
    /// Value necessary for a bar to reach the maximum height (if no value is specified,
    /// the maximum value in the data is taken as reference)
    max: Option<u64>,
    /// Values to display on the bar (computed when the data is passed to the widget)
    values: Vec<String>,
}

impl<'a> Default for Histogram<'a> {
    fn default() -> Histogram<'a> {
        Histogram {
            block: None,
            max: None,
            data: &[],
            values: Vec::new(),
            bar_style: Style::default(),
            bar_gap: 1,
            bar_set: symbols::bar::NINE_LEVELS,
            buckets: Vec::new(),
            value_style: Default::default(),
            label_style: Default::default(),
            style: Default::default(),
        }
    }
}

impl<'a> Histogram<'a> {
    pub fn data(mut self, data: &'a [u64], n_buckets: u64) -> Histogram<'a> {
        self.data = data;

        let min = *self.data.iter().min().unwrap();
        let max = *self.data.iter().max().unwrap() + 1;
        let bucket_size: u64 = ((max - min) as f64 / n_buckets as f64).ceil() as u64;
        self.buckets = vec![0; n_buckets as usize];

        // initialize buckets
        self.values = Vec::with_capacity(n_buckets as usize);
        for v in 0..n_buckets {
            self.values.push(format!("{}", v * bucket_size));
        }

        // bucketize data
        for &x in self.data.iter() {
            let idx: usize = ((x - min) / bucket_size) as usize;
            self.buckets[idx] += 1;
        }

        self.max = Some(*self.buckets.iter().max().unwrap());

        self
    }

    pub fn block(mut self, block: Block<'a>) -> Histogram<'a> {
        self.block = Some(block);
        self
    }

    pub fn max(mut self, max: u64) -> Histogram<'a> {
        self.max = Some(max);
        self
    }

    pub fn bar_style(mut self, style: Style) -> Histogram<'a> {
        self.bar_style = style;
        self
    }

    pub fn bar_gap(mut self, gap: u16) -> Histogram<'a> {
        self.bar_gap = gap;
        self
    }

    pub fn bar_set(mut self, bar_set: symbols::bar::Set) -> Histogram<'a> {
        self.bar_set = bar_set;
        self
    }

    pub fn value_style(mut self, style: Style) -> Histogram<'a> {
        self.value_style = style;
        self
    }

    pub fn label_style(mut self, style: Style) -> Histogram<'a> {
        self.label_style = style;
        self
    }

    pub fn style(mut self, style: Style) -> Histogram<'a> {
        self.style = style;
        self
    }
}

impl<'a> Widget for Histogram<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);

        let chart_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if chart_area.height < 2 {
            return;
        }

        let n_bars = self.buckets.len() as u16;
        let bar_width: u16 = (chart_area.width - (n_bars + 1) * self.bar_gap) / n_bars;

        let max = self
            .max
            .unwrap_or_else(|| self.buckets.iter().copied().max().unwrap_or_default());

        let mut data = self
            .buckets
            .iter()
            .take(n_bars as usize)
            .map(|&v| v * u64::from(chart_area.height - 1) * 8 / std::cmp::max(max, 1))
            .collect::<Vec<u64>>();
        for j in (0..chart_area.height - 1).rev() {
            for (i, d) in data.iter_mut().enumerate() {
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

                for x in 0..bar_width {
                    buf.get_mut(
                        chart_area.left() + i as u16 * (bar_width + self.bar_gap) + x,
                        chart_area.top() + j,
                    )
                    .set_symbol(symbol)
                    .set_style(self.bar_style);
                }

                if *d > 8 {
                    *d -= 8;
                } else {
                    *d = 0;
                }
            }
        }

        for (i, &value) in self.buckets.iter().enumerate() {
            let label = &self.values[i];
            if value != 0 {
                let value_label = format!("{}", &self.buckets[i]);
                let width = value_label.width() as u16;
                if width < bar_width {
                    buf.set_string(
                        chart_area.left()
                            + i as u16 * (bar_width + self.bar_gap)
                            + (bar_width - width) / 2,
                        chart_area.bottom() - 2,
                        value_label,
                        self.value_style,
                    );
                }
            }
            buf.set_stringn(
                chart_area.left() + i as u16 * (bar_width + self.bar_gap),
                chart_area.bottom() - 1,
                label,
                bar_width as usize,
                self.label_style,
            );
        }
    }
}
