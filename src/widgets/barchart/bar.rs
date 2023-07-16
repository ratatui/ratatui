use crate::{buffer::Buffer, prelude::Rect, style::Style, text::Line};

/// represent a bar to be shown by the Barchart
///
/// # Examples
/// the following example creates a bar with the label "Bar 1", a value "10",
/// red background and a white value foreground
///
/// ```
/// # use ratatui::{prelude::*, widgets::*};
/// Bar::default()
///     .label("Bar 1".into())
///     .value(10)
///     .style(Style::default().fg(Color::Red))
///     .value_style(Style::default().bg(Color::Red).fg(Color::White))
///     .text_value("10°C".to_string());
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Bar<'a> {
    /// Value to display on the bar (computed when the data is passed to the widget)
    pub(super) value: u64,
    /// optional label to be printed under the bar
    pub(super) label: Option<Line<'a>>,
    /// style for the bar
    pub(super) style: Style,
    /// style of the value printed at the bottom of the bar.
    pub(super) value_style: Style,
    /// optional text_value to be shown on the bar instead of the actual value
    pub(super) text_value: Option<String>,
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

    /// set the text value printed in the bar. (By default self.value is printed)
    pub fn text_value(mut self, text_value: String) -> Bar<'a> {
        self.text_value = Some(text_value);
        self
    }

    /// Render the value of the bar. value_text is used if set, otherwise the value is converted to
    /// string. The value is rendered using value_style. If the value width is greater than the
    /// bar width, then the value is split into 2 parts. the first part is rendered in the bar
    /// using value_style. The second part is rendered outside the bar using bar_style
    pub(super) fn render_value_with_different_styles(
        self,
        buf: &mut Buffer,
        area: Rect,
        bar_length: usize,
        default_value_style: Style,
        bar_style: Style,
    ) {
        let text = if let Some(text) = self.text_value {
            text
        } else {
            self.value.to_string()
        };

        if !text.is_empty() {
            let style = default_value_style.patch(self.value_style);
            // Since the value may be longer than the bar itself, we need to use 2 different styles
            // while rendering. Render the first part with the default value style
            buf.set_stringn(area.x, area.y, &text, bar_length, style);
            // render the second part with the bar_style
            if text.len() > bar_length {
                let (first, second) = text.split_at(bar_length);

                let style = bar_style.patch(self.style);
                buf.set_stringn(
                    area.x + first.len() as u16,
                    area.y,
                    second,
                    area.width as usize - first.len(),
                    style,
                );
            }
        }
    }

    pub(super) fn render_label_and_value(
        self,
        buf: &mut Buffer,
        max_width: u16,
        x: u16,
        y: u16,
        default_value_style: Style,
        default_label_style: Style,
    ) {
        // render the value
        if self.value != 0 {
            let value_label = if let Some(text) = self.text_value {
                text
            } else {
                self.value.to_string()
            };

            let width = value_label.len() as u16;
            if width < max_width {
                buf.set_string(
                    x + (max_width.saturating_sub(value_label.len() as u16) >> 1),
                    y,
                    value_label,
                    default_value_style.patch(self.value_style),
                );
            }
        }

        // render the label
        if let Some(mut label) = self.label {
            // patch label styles
            for span in &mut label.spans {
                span.style = default_label_style.patch(span.style);
            }

            buf.set_line(
                x + (max_width.saturating_sub(label.width() as u16) >> 1),
                y + 1,
                &label,
                max_width,
            );
        }
    }
}
