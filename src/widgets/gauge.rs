use crate::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Styled},
    symbols,
    text::{Line, Span},
    widgets::{Block, Widget},
};

/// A widget to display a task progress.
///
/// # Examples:
///
/// ```
/// # use ratatui::widgets::{Widget, Gauge, Block, Borders};
/// # use ratatui::style::{Style, Color, Modifier};
/// Gauge::default()
///     .block(Block::default().borders(Borders::ALL).title("Progress"))
///     .gauge_style(Style::default().fg(Color::White).bg(Color::Black).add_modifier(Modifier::ITALIC))
///     .percent(20);
/// ```
#[derive(Debug, Clone)]
pub struct Gauge<'a> {
    block: Option<Block<'a>>,
    ratio: f64,
    label: Option<Span<'a>>,
    use_unicode: bool,
    style: Style,
    gauge_style: Style,
}

impl<'a> Default for Gauge<'a> {
    fn default() -> Gauge<'a> {
        Gauge {
            block: None,
            ratio: 0.0,
            label: None,
            use_unicode: false,
            style: Style::default(),
            gauge_style: Style::default(),
        }
    }
}

impl<'a> Gauge<'a> {
    pub fn block(mut self, block: Block<'a>) -> Gauge<'a> {
        self.block = Some(block);
        self
    }

    pub fn percent(mut self, percent: u16) -> Gauge<'a> {
        assert!(
            percent <= 100,
            "Percentage should be between 0 and 100 inclusively."
        );
        self.ratio = f64::from(percent) / 100.0;
        self
    }

    /// Sets ratio ([0.0, 1.0]) directly.
    pub fn ratio(mut self, ratio: f64) -> Gauge<'a> {
        assert!(
            (0.0..=1.0).contains(&ratio),
            "Ratio should be between 0 and 1 inclusively."
        );
        self.ratio = ratio;
        self
    }

    pub fn label<T>(mut self, label: T) -> Gauge<'a>
    where
        T: Into<Span<'a>>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn style(mut self, style: Style) -> Gauge<'a> {
        self.style = style;
        self
    }

    pub fn gauge_style(mut self, style: Style) -> Gauge<'a> {
        self.gauge_style = style;
        self
    }

    pub fn use_unicode(mut self, unicode: bool) -> Gauge<'a> {
        self.use_unicode = unicode;
        self
    }
}

impl<'a> Widget for Gauge<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let gauge_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };
        buf.set_style(gauge_area, self.gauge_style);
        if gauge_area.height < 1 {
            return;
        }

        // compute label value and its position
        // label is put at the center of the gauge_area
        let label = {
            let pct = f64::round(self.ratio * 100.0);
            self.label.unwrap_or_else(|| Span::from(format!("{pct}%")))
        };
        let clamped_label_width = gauge_area.width.min(label.width() as u16);
        let label_col = gauge_area.left() + (gauge_area.width - clamped_label_width) / 2;
        let label_row = gauge_area.top() + gauge_area.height / 2;

        // the gauge will be filled proportionally to the ratio
        let filled_width = f64::from(gauge_area.width) * self.ratio;
        let end = if self.use_unicode {
            gauge_area.left() + filled_width.floor() as u16
        } else {
            gauge_area.left() + filled_width.round() as u16
        };
        for y in gauge_area.top()..gauge_area.bottom() {
            // render the filled area (left to end)
            for x in gauge_area.left()..end {
                let cell = buf.get_mut(x, y);
                if self.use_unicode {
                    cell.set_symbol(symbols::block::FULL)
                        .set_fg(self.gauge_style.fg.unwrap_or(Color::Reset))
                        .set_bg(self.gauge_style.bg.unwrap_or(Color::Reset));
                } else {
                    // spaces are needed to apply the background styling.
                    // note that the background and foreground colors are swapped
                    // otherwise the gauge will be inverted
                    cell.set_symbol(" ")
                        .set_fg(self.gauge_style.bg.unwrap_or(Color::Reset))
                        .set_bg(self.gauge_style.fg.unwrap_or(Color::Reset));
                }
            }
            if self.use_unicode && self.ratio < 1.0 {
                buf.get_mut(end, y)
                    .set_symbol(get_unicode_block(filled_width % 1.0));
            }
        }
        // render the label
        buf.set_span(label_col, label_row, &label, clamped_label_width);
    }
}

fn get_unicode_block<'a>(frac: f64) -> &'a str {
    match (frac * 8.0).round() as u16 {
        1 => symbols::block::ONE_EIGHTH,
        2 => symbols::block::ONE_QUARTER,
        3 => symbols::block::THREE_EIGHTHS,
        4 => symbols::block::HALF,
        5 => symbols::block::FIVE_EIGHTHS,
        6 => symbols::block::THREE_QUARTERS,
        7 => symbols::block::SEVEN_EIGHTHS,
        8 => symbols::block::FULL,
        _ => " ",
    }
}

/// A compact widget to display a task progress over a single line.
///
/// # Examples:
///
/// ```
/// # use ratatui::widgets::{Widget, LineGauge, Block, Borders};
/// # use ratatui::style::{Style, Color, Modifier};
/// # use ratatui::symbols;
/// LineGauge::default()
///     .block(Block::default().borders(Borders::ALL).title("Progress"))
///     .gauge_style(Style::default().fg(Color::White).bg(Color::Black).add_modifier(Modifier::BOLD))
///     .line_set(symbols::line::THICK)
///     .ratio(0.4);
/// ```
#[derive(Debug, Default, Clone)]
pub struct LineGauge<'a> {
    block: Option<Block<'a>>,
    ratio: f64,
    label: Option<Line<'a>>,
    line_set: symbols::line::Set,
    style: Style,
    gauge_style: Style,
}

impl<'a> LineGauge<'a> {
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn ratio(mut self, ratio: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&ratio),
            "Ratio should be between 0 and 1 inclusively."
        );
        self.ratio = ratio;
        self
    }

    pub fn line_set(mut self, set: symbols::line::Set) -> Self {
        self.line_set = set;
        self
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<Line<'a>>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn gauge_style(mut self, style: Style) -> Self {
        self.gauge_style = style;
        self
    }
}

impl<'a> Widget for LineGauge<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let gauge_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if gauge_area.height < 1 {
            return;
        }

        let ratio = self.ratio;
        let label = self
            .label
            .unwrap_or_else(move || Line::from(format!("{:.0}%", ratio * 100.0)));
        let (col, row) = buf.set_line(
            gauge_area.left(),
            gauge_area.top(),
            &label,
            gauge_area.width,
        );
        let start = col + 1;
        if start >= gauge_area.right() {
            return;
        }

        let end = start
            + (f64::from(gauge_area.right().saturating_sub(start)) * self.ratio).floor() as u16;
        for col in start..end {
            buf.get_mut(col, row)
                .set_symbol(self.line_set.horizontal)
                .set_style(Style {
                    fg: self.gauge_style.fg,
                    bg: None,
                    #[cfg(feature = "crossterm")]
                    underline_color: self.gauge_style.underline_color,
                    add_modifier: self.gauge_style.add_modifier,
                    sub_modifier: self.gauge_style.sub_modifier,
                });
        }
        for col in end..gauge_area.right() {
            buf.get_mut(col, row)
                .set_symbol(self.line_set.horizontal)
                .set_style(Style {
                    fg: self.gauge_style.bg,
                    bg: None,
                    #[cfg(feature = "crossterm")]
                    underline_color: self.gauge_style.underline_color,
                    add_modifier: self.gauge_style.add_modifier,
                    sub_modifier: self.gauge_style.sub_modifier,
                });
        }
    }
}

impl<'a> Styled for Gauge<'a> {
    type Item = Gauge<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}

impl<'a> Styled for LineGauge<'a> {
    type Item = LineGauge<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{Modifier, Stylize};

    #[test]
    #[should_panic]
    fn gauge_invalid_percentage() {
        Gauge::default().percent(110);
    }

    #[test]
    #[should_panic]
    fn gauge_invalid_ratio_upper_bound() {
        Gauge::default().ratio(1.1);
    }

    #[test]
    #[should_panic]
    fn gauge_invalid_ratio_lower_bound() {
        Gauge::default().ratio(-0.5);
    }

    #[test]
    fn gauge_can_be_stylized() {
        assert_eq!(
            Gauge::default().black().on_white().bold().not_dim().style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::DIM)
        )
    }

    #[test]
    fn line_gauge_can_be_stylized() {
        assert_eq!(
            LineGauge::default()
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
        )
    }

    #[test]
    fn line_gauge_default() {
        // TODO: replace to `assert_eq!(LineGauge::default(), LineGauge::default())`
        // when `Eq` or `PartialEq` is implemented for `LineGauge`.
        assert_eq!(
            format!("{:?}", LineGauge::default()),
            format!(
                "{:?}",
                LineGauge {
                    block: None,
                    ratio: 0.0,
                    label: None,
                    style: Style::default(),
                    line_set: symbols::line::NORMAL,
                    gauge_style: Style::default(),
                }
            ),
            "LineGauge::default() should have correct default values."
        );
    }
}
