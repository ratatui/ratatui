use crate::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::Line,
    widgets::{barchart::Bar, Widget},
};

/// A group of bars to be shown by the Barchart.
///
/// # Examples
///
/// ```
/// use ratatui::widgets::{Bar, BarGroup};
///
/// BarGroup::default()
///     .label("Group 1".into())
///     .bars(&[Bar::default().value(200), Bar::default().value(150)]);
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct BarGroup<'a> {
    /// label of the group. It will be printed centered under this group of bars
    pub(super) label: Option<Line<'a>>,
    /// list of bars to be shown
    pub(super) bars: Vec<Bar<'a>>,
}

impl<'a> BarGroup<'a> {
    /// Set the group label
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn label(mut self, label: Line<'a>) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the bars of the group to be shown
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn bars(mut self, bars: &[Bar<'a>]) -> Self {
        self.bars = bars.to_vec();
        self
    }

    /// The maximum bar value of this group
    pub(super) fn max(&self) -> Option<u64> {
        self.bars.iter().max_by_key(|v| v.value).map(|v| v.value)
    }

    pub(super) fn render_label(&self, buf: &mut Buffer, area: Rect, default_label_style: Style) {
        if let Some(label) = &self.label {
            // align the label. Necessary to do it this way as we don't want to set the style
            // of the whole area, just the label area
            let width = label.width() as u16;
            let area = match label.alignment {
                Some(Alignment::Center) => Rect {
                    x: area.x + (area.width.saturating_sub(width)) / 2,
                    width,
                    ..area
                },
                Some(Alignment::Right) => Rect {
                    x: area.x + area.width.saturating_sub(width),
                    width,
                    ..area
                },
                _ => Rect { width, ..area },
            };
            buf.set_style(area, default_label_style);
            label.render(area, buf);
        }
    }
}

impl<'a> From<&[(&'a str, u64)]> for BarGroup<'a> {
    fn from(value: &[(&'a str, u64)]) -> Self {
        Self {
            label: None,
            bars: value
                .iter()
                .map(|&(text, v)| Bar::default().value(v).label(text.into()))
                .collect(),
        }
    }
}

impl<'a, const N: usize> From<&[(&'a str, u64); N]> for BarGroup<'a> {
    fn from(value: &[(&'a str, u64); N]) -> Self {
        let value: &[(&'a str, u64)] = value.as_ref();
        Self::from(value)
    }
}

impl<'a> From<&Vec<(&'a str, u64)>> for BarGroup<'a> {
    fn from(value: &Vec<(&'a str, u64)>) -> Self {
        let array: &[(&str, u64)] = value;
        Self::from(array)
    }
}
