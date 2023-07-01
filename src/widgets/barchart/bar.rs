use crate::{buffer::Buffer, style::Style, text::Line};

/// represent a bar to be shown by the Barchart
///
/// # Examples
/// the following example creates a bar with the label "Bar 1", a value "10",
/// red background and a white value foreground
///
/// ```
/// # use ratatui::prelude::*;
/// Bar::default()
///     .label("Bar 1".into())
///     .value(10)
///     .style(Style::default().fg(Color::Red))
///     .value_style(Style::default().bg(Color::Red).fg(Color::White));
/// ```
#[derive(Debug, Clone, Default)]
pub struct Bar<'a> {
    /// Value to display on the bar (computed when the data is passed to the widget)
    pub(super) value: u64,
    /// optional label to be printed under the bar
    pub(super) label: Option<Line<'a>>,
    /// style for the bar
    pub(super) style: Style,
    /// style of the value printed at the bottom of the bar.
    pub(super) value_style: Style,
}

impl<'a> Bar<'a> {
    pub fn value(mut self, value: u64) -> Bar<'a> {
        self.value = value;
        self
    }

    pub fn label(mut self, label: Line<'a>) -> Bar<'a> {
        self.label = Some(label);
        self
    }

    pub fn style(mut self, style: Style) -> Bar<'a> {
        self.style = style;
        self
    }

    pub fn value_style(mut self, style: Style) -> Bar<'a> {
        self.value_style = style;
        self
    }

    /// render the bar's value
    pub(super) fn render_value(
        &self,
        buf: &mut Buffer,
        max_width: u16,
        x: u16,
        y: u16,
        default_style: Style,
    ) {
        if self.value != 0 {
            let value_label = format!("{}", self.value);
            let width = value_label.len() as u16;
            if width < max_width {
                buf.set_string(
                    x + (max_width.saturating_sub(value_label.len() as u16) >> 1),
                    y,
                    value_label,
                    self.value_style.patch(default_style),
                );
            }
        }
    }

    /// render the bar's label
    pub(super) fn render_label(
        self,
        buf: &mut Buffer,
        max_width: u16,
        x: u16,
        y: u16,
        default_style: Style,
    ) {
        if let Some(mut label) = self.label {
            label.patch_style(default_style);
            buf.set_line(
                x,
                y,
                &label,
                max_width,
            );
        }
    }
}
