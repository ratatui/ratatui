//! The [`Checkbox`] widget displays a checkbox with a label that can be in a checked or unchecked state.

use alloc::borrow::Cow;

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::style::{Style, Styled};
use ratatui_core::symbols;
use ratatui_core::text::{Line, Span};
use ratatui_core::widgets::Widget;

use crate::block::{Block, BlockExt};

/// A widget that displays a checkbox with a label.
///
/// A `Checkbox` can be in a checked or unchecked state. The checkbox is rendered with a symbol
/// (default `☐` for unchecked and `☑` for checked) followed by a label.
///
/// The widget can be styled using [`Checkbox::style`] which affects both the checkbox symbol and
/// the label. You can also style just the checkbox symbol using [`Checkbox::checkbox_style`] or
/// the label using [`Checkbox::label_style`].
///
/// You can create a `Checkbox` using [`Checkbox::new`] or [`Checkbox::default`].
///
/// # Examples
///
/// ```
/// use ratatui::style::{Color, Style, Stylize};
/// use ratatui::widgets::Checkbox;
///
/// Checkbox::new("Enable feature", true)
///     .style(Style::default().fg(Color::White))
///     .checkbox_style(Style::default().fg(Color::Green))
///     .label_style(Style::default().fg(Color::Gray));
/// ```
///
/// With a block:
/// ```
/// use ratatui::widgets::{Block, Checkbox};
///
/// Checkbox::new("Accept terms", false)
///     .block(Block::bordered().title("Settings"));
/// ```
///
/// # Styling
///
/// The widget can be styled in multiple ways:
/// - [`Checkbox::style`]: Sets the base style for the entire widget
/// - [`Checkbox::checkbox_style`]: Sets the style specifically for the checkbox symbol
/// - [`Checkbox::label_style`]: Sets the style specifically for the label text
///
/// Styles are applied in order: base style, then specific styles override it.
#[expect(clippy::struct_field_names)] // checkbox_style needs to be differentiated from style
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Checkbox<'a> {
    /// The label text displayed next to the checkbox
    label: Line<'a>,
    /// Whether the checkbox is checked
    checked: bool,
    /// Optional block to wrap the checkbox
    block: Option<Block<'a>>,
    /// Base style for the entire widget
    style: Style,
    /// Style specifically for the checkbox symbol
    checkbox_style: Style,
    /// Style specifically for the label text
    label_style: Style,
    /// Symbol to use when checked
    checked_symbol: Cow<'a, str>,
    /// Symbol to use when unchecked
    unchecked_symbol: Cow<'a, str>,
}

impl Default for Checkbox<'_> {
    /// Returns a default `Checkbox` widget.
    ///
    /// The default widget has:
    /// - Empty label
    /// - Unchecked state
    /// - No block
    /// - Default style for all elements
    /// - Unicode checkbox symbols (☐ and ☑)
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::Checkbox;
    ///
    /// let checkbox = Checkbox::default();
    /// ```
    fn default() -> Self {
        Self {
            label: Line::default(),
            checked: false,
            block: None,
            style: Style::default(),
            checkbox_style: Style::default(),
            label_style: Style::default(),
            checked_symbol: Cow::Borrowed(symbols::checkbox::CHECKED),
            unchecked_symbol: Cow::Borrowed(symbols::checkbox::UNCHECKED),
        }
    }
}

impl<'a> Checkbox<'a> {
    /// Creates a new `Checkbox` with the given label and checked state.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Enable feature", true);
    /// ```
    ///
    /// With styled label:
    /// ```
    /// use ratatui::style::Stylize;
    /// use ratatui::widgets::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Enable feature".blue(), false);
    /// ```
    pub fn new<T>(label: T, checked: bool) -> Self
    where
        T: Into<Line<'a>>,
    {
        Self {
            label: label.into(),
            checked,
            ..Default::default()
        }
    }

    /// Sets the label of the checkbox.
    ///
    /// The label can be any type that converts into a [`Line`], such as a string or a styled span.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::Checkbox;
    ///
    /// let checkbox = Checkbox::default().label("My checkbox");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<Line<'a>>,
    {
        self.label = label.into();
        self
    }

    /// Sets the checked state of the checkbox.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::Checkbox;
    ///
    /// let checkbox = Checkbox::default().checked(true);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Wraps the checkbox with the given block.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::{Block, Checkbox};
    ///
    /// let checkbox = Checkbox::new("Option", false).block(Block::bordered().title("Settings"));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Sets the base style of the widget.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be applied to both the checkbox symbol and the label unless overridden by
    /// more specific styles.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::{Color, Style};
    /// use ratatui::widgets::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Option", false).style(Style::default().fg(Color::White));
    /// ```
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the style of the checkbox symbol.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be combined with the base style set by [`Checkbox::style`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::{Color, Style};
    /// use ratatui::widgets::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Option", true).checkbox_style(Style::default().fg(Color::Green));
    /// ```
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn checkbox_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.checkbox_style = style.into();
        self
    }

    /// Sets the style of the label text.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This style will be combined with the base style set by [`Checkbox::style`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::{Color, Style};
    /// use ratatui::widgets::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Option", false).label_style(Style::default().fg(Color::Gray));
    /// ```
    ///
    /// [`Color`]: ratatui_core::style::Color
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn label_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.label_style = style.into();
        self
    }

    /// Sets the symbol to use when the checkbox is checked.
    ///
    /// The default is `☑` (U+2611).
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Option", true).checked_symbol("[X]");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn checked_symbol<T>(mut self, symbol: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.checked_symbol = symbol.into();
        self
    }

    /// Sets the symbol to use when the checkbox is unchecked.
    ///
    /// The default is `☐` (U+2610).
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::Checkbox;
    ///
    /// let checkbox = Checkbox::new("Option", false).unchecked_symbol("[ ]");
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn unchecked_symbol<T>(mut self, symbol: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.unchecked_symbol = symbol.into();
        self
    }
}

impl Styled for Checkbox<'_> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(mut self, style: S) -> Self::Item {
        self.style = style.into();
        self
    }
}

impl Widget for Checkbox<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &Checkbox<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        self.block.as_ref().render(area, buf);
        let inner = self.block.inner_if_some(area);
        self.render_checkbox(inner, buf);
    }
}

impl Checkbox<'_> {
    fn render_checkbox(&self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() {
            return;
        }

        // Determine which symbol to use based on checked state
        let symbol = if self.checked {
            &self.checked_symbol
        } else {
            &self.unchecked_symbol
        };

        // Calculate the combined styles
        let checkbox_style = self.style.patch(self.checkbox_style);
        let label_style = self.style.patch(self.label_style);

        // Render the checkbox symbol
        let checkbox_span = Span::styled(symbol.as_ref(), checkbox_style);

        // Render label with appropriate styling
        let styled_label = self.label.clone().patch_style(label_style);

        // Combine checkbox symbol and label with a space
        let mut spans = alloc::vec::Vec::new();
        spans.push(checkbox_span);
        spans.push(Span::raw(" "));
        spans.extend(styled_label.spans);

        let line = Line::from(spans);

        // Render the line
        buf.set_line(area.x, area.y, &line, area.width);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui_core::style::{Color, Modifier, Stylize};

    #[test]
    fn checkbox_new() {
        let checkbox = Checkbox::new("Test", true);
        assert_eq!(checkbox.label, Line::from("Test"));
        assert!(checkbox.checked);
    }

    #[test]
    fn checkbox_default() {
        let checkbox = Checkbox::default();
        assert_eq!(checkbox.label, Line::default());
        assert!(!checkbox.checked);
    }

    #[test]
    fn checkbox_label() {
        let checkbox = Checkbox::default().label("New label");
        assert_eq!(checkbox.label, Line::from("New label"));
    }

    #[test]
    fn checkbox_checked() {
        let checkbox = Checkbox::default().checked(true);
        assert!(checkbox.checked);
    }

    #[test]
    fn checkbox_style() {
        let style = Style::default().fg(Color::Red);
        let checkbox = Checkbox::default().style(style);
        assert_eq!(checkbox.style, style);
    }

    #[test]
    fn checkbox_checkbox_style() {
        let style = Style::default().fg(Color::Green);
        let checkbox = Checkbox::default().checkbox_style(style);
        assert_eq!(checkbox.checkbox_style, style);
    }

    #[test]
    fn checkbox_label_style() {
        let style = Style::default().fg(Color::Blue);
        let checkbox = Checkbox::default().label_style(style);
        assert_eq!(checkbox.label_style, style);
    }

    #[test]
    fn checkbox_checked_symbol() {
        let checkbox = Checkbox::default().checked_symbol("[X]");
        assert_eq!(checkbox.checked_symbol, "[X]");
    }

    #[test]
    fn checkbox_unchecked_symbol() {
        let checkbox = Checkbox::default().unchecked_symbol("[ ]");
        assert_eq!(checkbox.unchecked_symbol, "[ ]");
    }

    #[test]
    fn checkbox_styled_trait() {
        let checkbox = Checkbox::default().red();
        assert_eq!(checkbox.style, Style::default().fg(Color::Red));
    }

    #[test]
    fn checkbox_render_unchecked() {
        let checkbox = Checkbox::new("Test", false);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        checkbox.render(buffer.area, &mut buffer);

        // The buffer should contain the unchecked symbol followed by space and label
        assert!(buffer.content()[0].symbol().starts_with('☐'));
    }

    #[test]
    fn checkbox_render_checked() {
        let checkbox = Checkbox::new("Test", true);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        checkbox.render(buffer.area, &mut buffer);

        // The buffer should contain the checked symbol followed by space and label
        assert!(buffer.content()[0].symbol().starts_with('☑'));
    }

    #[test]
    fn checkbox_render_empty_area() {
        let checkbox = Checkbox::new("Test", true);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 0, 0));

        // Should not panic
        checkbox.render(buffer.area, &mut buffer);
    }

    #[test]
    fn checkbox_render_with_block() {
        let checkbox = Checkbox::new("Test", true).block(Block::bordered());
        let mut buffer = Buffer::empty(Rect::new(0, 0, 12, 3));

        // Should not panic
        checkbox.render(buffer.area, &mut buffer);
    }

    #[test]
    fn checkbox_render_with_custom_symbols() {
        let checkbox = Checkbox::new("Test", true)
            .checked_symbol("[X]")
            .unchecked_symbol("[ ]");

        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        checkbox.render(buffer.area, &mut buffer);

        assert!(buffer.content()[0].symbol().starts_with('['));
    }

    #[test]
    fn checkbox_with_styled_label() {
        let checkbox = Checkbox::new("Test".blue(), true);
        assert_eq!(checkbox.label.spans[0].style.fg, Some(Color::Blue));
    }

    #[test]
    fn checkbox_complex_styling() {
        let checkbox = Checkbox::new("Feature", true)
            .style(Style::default().fg(Color::White))
            .checkbox_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
            .label_style(Style::default().fg(Color::Gray));

        assert_eq!(checkbox.style.fg, Some(Color::White));
        assert_eq!(checkbox.checkbox_style.fg, Some(Color::Green));
        assert_eq!(checkbox.label_style.fg, Some(Color::Gray));
    }
}
