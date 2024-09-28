use unicode_width::UnicodeWidthStr;

use crate::{buffer::Buffer, layout::Rect, style::Style, text::Line, widgets::Widget};
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
/// use ratatui::{
///     style::{Style, Stylize},
///     widgets::Bar,
/// };
///
/// Bar::default()
///     .label("Bar 1".into())
///     .value(10)
///     .style(Style::new().red())
///     .value_style(Style::new().red().on_white())
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
    /// optional `text_value` to be shown on the bar instead of the actual value
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
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn value(mut self, value: u64) -> Self {
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
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn label(mut self, label: Line<'a>) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the style of the bar.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This will apply to every non-styled element. It can be seen and used as a default value.
    ///
    /// [`Color`]: crate::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Set the style of the value.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// # See also
    ///
    /// [`Bar::value`] to set the value.
    ///
    /// [`Color`]: crate::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn value_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.value_style = style.into();
        self
    }

    /// Set the text value printed in the bar.
    ///
    /// If `text_value` is not set, then the [`ToString`] representation of `value` will be shown on
    /// the bar.
    ///
    /// # See also
    ///
    /// [`Bar::value`] to set the value.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn text_value(mut self, text_value: String) -> Self {
        self.text_value = Some(text_value);
        self
    }

    /// Render the value of the bar.
    ///
    /// [`text_value`](Bar::text_value) is used if set, otherwise the value is converted to string.
    /// The value is rendered using `value_style`. If the value width is greater than the
    /// bar width, then the value is split into 2 parts. the first part is rendered in the bar
    /// using `value_style`. The second part is rendered outside the bar using `bar_style`
    pub(super) fn render_value_with_different_styles(
        &self,
        buf: &mut Buffer,
        area: Rect,
        bar_length: usize,
        default_value_style: Style,
        bar_style: Style,
    ) {
        let value = self.value.to_string();
        let text = self.text_value.as_ref().unwrap_or(&value);

        if !text.is_empty() {
            let style = default_value_style.patch(self.value_style);
            // Since the value may be longer than the bar itself, we need to use 2 different styles
            // while rendering. Render the first part with the default value style
            buf.set_stringn(area.x, area.y, text, bar_length, style);
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
        &self,
        buf: &mut Buffer,
        max_width: u16,
        x: u16,
        y: u16,
        default_value_style: Style,
        ticks: u64,
    ) {
        if self.value != 0 {
            const TICKS_PER_LINE: u64 = 8;
            let value = self.value.to_string();
            let value_label = self.text_value.as_ref().unwrap_or(&value);
            let width = value_label.width() as u16;
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
        &self,
        buf: &mut Buffer,
        max_width: u16,
        x: u16,
        y: u16,
        default_label_style: Style,
    ) {
        // center the label. Necessary to do it this way as we don't want to set the style
        // of the whole area, just the label area
        let width = self
            .label
            .as_ref()
            .map_or(0, Line::width)
            .min(max_width as usize) as u16;
        let area = Rect {
            x: x + (max_width.saturating_sub(width)) / 2,
            y,
            width,
            height: 1,
        };
        buf.set_style(area, default_label_style);
        if let Some(label) = &self.label {
            label.render(area, buf);
        }
    }
}
