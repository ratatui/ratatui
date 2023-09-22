use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols,
    widgets::{Block, Widget},
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Bucket {
    low: f64,
    high: f64,
    count: usize,
}

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
    data: &'a [f64],
    /// each bucket keeps a count of the data points that fall into it
    /// buckets[0].count counts items where buckets[0].low <= x < buckets[0].high
    /// buckets[1].count counts items where buckets[1].low <= x < buckets[1].high
    /// etc.
    buckets: Vec<Bucket>,
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
    pub fn data(mut self, data: &'a [f64], n_buckets: u64) -> Histogram<'a> {
        self.data = data;

        let min = data.iter().cloned().fold(f64::NAN, f64::min);
        let max = data.iter().cloned().fold(f64::NAN, f64::max);

        let bucket_size = (max - min) / n_buckets as f64;
        self.buckets = Vec::with_capacity(n_buckets as usize);

        // initialize buckets
        self.values = Vec::with_capacity(n_buckets as usize);
        for i in 0..n_buckets {
            let start = min + bucket_size * i as f64;
            let bucket = Bucket {
                low: start,
                high: start + bucket_size,
                count: 0,
            };
            self.buckets.push(bucket);
            self.values
                .push(format!("[{:.1}, {:.1})", bucket.low, bucket.high));
        }

        // bucketize data
        for &x in self.data.iter() {
            let idx: usize = ((x - min) / bucket_size).floor() as usize;
            if idx < self.buckets.len() {
                self.buckets[idx].count += 1;
            } else {
                // TODO: decide what to do with excess
            }
        }

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
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;
    use crate::assert_buffer_eq;

    #[test]
    fn test_compute_bins() {
        let data = [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];
        let hist = Histogram::default().data(&data, 3);

        assert_eq!(hist.buckets.len(), 3);
        assert_eq!(hist.buckets[0].count, 2); // 0.0, 0.5
        assert_eq!(hist.buckets[1].count, 2); // 1.0, 1.5
        assert_eq!(hist.buckets[2].count, 2); // 2.0, 2.5
    }

    #[test]
    fn test_render_histogram() {}
}
