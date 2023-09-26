use unicode_width::UnicodeWidthStr;

use crate::{buffer::Buffer, prelude::Rect, style::Style, text::Line};

/// A bar to be shown by the [`BarChart`](crate::widgets::BarChart) widget.
///
/// Here is an explanation of a `Bar`'s components.
/// ```plain
/// ███                          ┐
/// █2█  <- text_value or value  │ bar
/// foo  <- label                ┘
/// ```
/// Note that every element can be styled individually.
///
/// # Example
///
/// The following example creates a bar with the label "Bar 1", a value "10",
/// red background and a white value foreground.
/// ```
/// use ratatui::{prelude::*, widgets::*};
///
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
    /// Set the value of this bar.
    ///
    /// The value will be displayed inside the bar.
    ///
    /// # See also
    ///
    /// [`Bar::value_style`] to style the value.  
    /// [`Bar::text_value`] to set the displayed value.
    pub fn value(mut self, value: u64) -> Bar<'a> {
        self.value = value;
        self
    }

    /// Set the label of the bar.
    ///
    /// For [`Vertical`](crate::layout::Direction::Vertical) bars,
    /// display the label **under** the bar.  
    /// For [`Horizontal`](crate::layout::Direction::Horizontal) bars,
    /// display the label **in** the bar.  
    /// See [`BarChart::direction`](crate::widgets::BarChart::direction) to set the direction.
    pub fn label(mut self, label: Line<'a>) -> Bar<'a> {
        self.label = Some(label);
        self
    }

    /// Set the style of the bar.
    ///
    /// This will apply to every non-styled element.
    /// It can be seen and used as a default value.
    pub fn style(mut self, style: Style) -> Bar<'a> {
        self.style = style;
        self
    }

    /// Set the style of the value.
    ///
    /// # See also
    ///
    /// [`Bar::value`] to set the value.
    pub fn value_style(mut self, style: Style) -> Bar<'a> {
        self.value_style = style;
        self
    }

    /// Set the text value printed in the bar.
    ///
    /// If `text_value` is not set, then the [ToString] representation of `value` will be shown on
    /// the bar.
    ///
    /// # See also
    ///
    /// [`Bar::value`] to set the value.
    pub fn text_value(mut self, text_value: String) -> Bar<'a> {
        self.text_value = Some(text_value);
        self
    }

    /// Render the value of the bar.
    ///
    /// [`text_value`](Bar::text_value) is used if set, otherwise the value is converted to string.
    /// The value is rendered using value_style. If the value width is greater than the
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

    pub(super) fn render_value(
        self,
        buf: &mut Buffer,
        max_width: u16,
        x: u16,
        y: u16,
        default_value_style: Style,
        ticks: u64,
    ) {
        if self.value != 0 {
            let value_label = if let Some(text) = self.text_value {
                text
            } else {
                self.value.to_string()
            };

            let width = value_label.width() as u16;
            const TICKS_PER_LINE: u64 = 8;
            // if we have enough space or the ticks are greater equal than 1 cell (8)
            // then print the value
            if width < max_width || (width == max_width && ticks >= TICKS_PER_LINE) {
                buf.set_string(
                    x + (max_width.saturating_sub(value_label.len() as u16) >> 1),
                    y,
                    value_label,
                    default_value_style.patch(self.value_style),
                );
            }
        }
    }

    pub(super) fn render_label(
        &mut self,
        buf: &mut Buffer,
        max_width: u16,
        x: u16,
        y: u16,
        default_label_style: Style,
    ) {
        if let Some(label) = &mut self.label {
            // patch label styles
            for span in &mut label.spans {
                span.style = default_label_style.patch(span.style);
            }

            buf.set_line(
                x + (max_width.saturating_sub(label.width() as u16) >> 1),
                y,
                label,
                max_width,
            );
        }
    }
}
