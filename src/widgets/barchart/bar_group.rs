use super::Bar;
use crate::{
    prelude::{Alignment, Buffer, Rect},
    style::Style,
    text::Line,
};

/// represent a group of bars to be shown by the Barchart
///
/// # Examples
/// ```
/// # use ratatui::{prelude::*, widgets::*};
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
    pub fn label(mut self, label: Line<'a>) -> BarGroup<'a> {
        self.label = Some(label);
        self
    }

    /// Set the bars of the group to be shown
    pub fn bars(mut self, bars: &[Bar<'a>]) -> BarGroup<'a> {
        self.bars = bars.to_vec();
        self
    }

    /// return the maximum bar value of this group
    pub(super) fn max(&self) -> Option<u64> {
        self.bars.iter().max_by_key(|v| v.value).map(|v| v.value)
    }

    pub(super) fn render_label(self, buf: &mut Buffer, area: Rect, default_label_style: Style) {
        if let Some(mut label) = self.label {
            // patch label styles
            for span in &mut label.spans {
                span.style = default_label_style.patch(span.style);
            }

            let x_offset = match label.alignment {
                Some(Alignment::Center) => area.width.saturating_sub(label.width() as u16) >> 1,
                Some(Alignment::Right) => area.width.saturating_sub(label.width() as u16),
                _ => 0,
            };

            buf.set_line(area.x + x_offset, area.y, &label, area.width);
        }
    }
}

impl<'a> From<&[(&'a str, u64)]> for BarGroup<'a> {
    fn from(value: &[(&'a str, u64)]) -> BarGroup<'a> {
        BarGroup {
            label: None,
            bars: value
                .iter()
                .map(|&(text, v)| Bar::default().value(v).label(text.into()))
                .collect(),
        }
    }
}

impl<'a, const N: usize> From<&[(&'a str, u64); N]> for BarGroup<'a> {
    fn from(value: &[(&'a str, u64); N]) -> BarGroup<'a> {
        Self::from(value.as_ref())
    }
}

impl<'a> From<&Vec<(&'a str, u64)>> for BarGroup<'a> {
    fn from(value: &Vec<(&'a str, u64)>) -> BarGroup<'a> {
        let array: &[(&str, u64)] = value;
        Self::from(array)
    }
}
