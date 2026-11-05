//! The [`Ring`] widget displays progress as an arc around a braille annulus.
use core::f64::consts::TAU;

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Position, Rect};
use ratatui_core::style::{Style, Styled};
use ratatui_core::text::Line;
use ratatui_core::widgets::Widget;

use crate::block::{Block, BlockExt};
#[cfg(not(feature = "std"))]
use crate::polyfills::F64Polyfills;

/// Braille dot bits by row (top to bottom) and column (left, right) inside one cell.
const DOTS: [[u32; 2]; 4] = [[0x01, 0x08], [0x02, 0x10], [0x04, 0x20], [0x40, 0x80]];
const BRAILLE_BLANK: u32 = 0x2800;
/// The ring's width as a share of its outer radius.
const THICKNESS: f64 = 0.25;
/// Braille dots sit closer together across a cell than down it; this keeps the ring round.
const DOT_ASPECT: f64 = 1.07;

/// A widget to display progress as a ring.
///
/// A `Ring` draws an annulus of braille dots that fills the [`Rect`] it is
/// [rendered](Widget::render) in. The arc from twelve o'clock, clockwise, up to the value given
/// to [`Ring::ratio`] or [`Ring::percent`] uses [`Ring::filled_style`]; the rest of the turn uses
/// [`Ring::unfilled_style`]. A cell holding dots of both takes only the filled dots and style, so
/// the arc ends cleanly.
///
/// An optional label is centred in the ring's hole. A ring needs at least 5 × 3 cells to leave
/// room for a short label.
///
/// # Example
///
/// ```
/// use ratatui::style::{Color, Style};
/// use ratatui::widgets::{Block, Ring};
///
/// Ring::default()
///     .block(Block::bordered().title("Context"))
///     .filled_style(Color::Green)
///     .unfilled_style(Color::DarkGray)
///     .label("42%")
///     .ratio(0.42);
/// ```
///
/// # See also
///
/// - [`Gauge`](crate::gauge::Gauge) for a horizontal bar
/// - [`LineGauge`](crate::gauge::LineGauge) for a thin progress bar
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Ring<'a> {
    block: Option<Block<'a>>,
    ratio: f64,
    label: Option<Line<'a>>,
    style: Style,
    filled_style: Style,
    unfilled_style: Style,
}

impl<'a> Ring<'a> {
    /// Surrounds the `Ring` with a [`Block`].
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Sets the share of the turn to fill, from 0.0 to 1.0.
    ///
    /// # Panics
    ///
    /// This method panics if `ratio` is **not** between 0 and 1 inclusively.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn ratio(mut self, ratio: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&ratio),
            "Ratio should be between 0 and 1 inclusively."
        );
        self.ratio = ratio;
        self
    }

    /// Sets the share of the turn to fill, as a percentage.
    ///
    /// # Panics
    ///
    /// This method panics if `percent` is **not** between 0 and 100 inclusively.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn percent(mut self, percent: u16) -> Self {
        assert!(
            percent <= 100,
            "Percentage should be between 0 and 100 inclusively."
        );
        self.ratio = f64::from(percent) / 100.0;
        self
    }

    /// Sets the label centred in the ring's hole.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<Line<'a>>,
    {
        self.label = Some(label.into());
        self
    }

    /// Sets the widget style: the background of the whole area, under the dots and the label.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the style of the filled arc.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn filled_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.filled_style = style.into();
        self
    }

    /// Sets the style of the rest of the turn.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn unfilled_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.unfilled_style = style.into();
        self
    }
}

impl Styled for Ring<'_> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl Widget for Ring<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &Ring<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        if area.is_empty() {
            return;
        }
        buf.set_style(area, self.style);
        self.block.as_ref().render(area, buf);
        let inner = self.block.inner_if_some(area);
        if inner.is_empty() {
            return;
        }
        self.render_ring(inner, buf);
        self.render_label(inner, buf);
    }
}

impl Ring<'_> {
    fn render_ring(&self, area: Rect, buf: &mut Buffer) {
        let centre_x = f64::from(area.width * 2 - 1) / 2.0;
        let centre_y = f64::from(area.height * 4 - 1) / 2.0;
        let outer = centre_x.min(centre_y * DOT_ASPECT) + 0.5;
        let inner = outer * (1.0 - THICKNESS);
        let (outer_sq, inner_sq) = (outer * outer, inner * inner);
        for row in 0..area.height {
            for column in 0..area.width {
                let (mut filled, mut unfilled) = (0, 0);
                for (dy, bits) in (0_u16..).zip(DOTS) {
                    for (dx, bit) in (0_u16..).zip(bits) {
                        let x = f64::from(column * 2 + dx) - centre_x;
                        let y = (f64::from(row * 4 + dy) - centre_y) * DOT_ASPECT;
                        let distance_sq = x * x + y * y;
                        if distance_sq < inner_sq || distance_sq > outer_sq {
                            continue;
                        }
                        if self.ratio > 0.0 && turn(x, y) <= self.ratio {
                            filled |= bit;
                        } else {
                            unfilled |= bit;
                        }
                    }
                }
                let (dots, style) = if filled > 0 {
                    (filled, self.filled_style)
                } else {
                    (unfilled, self.unfilled_style)
                };
                let Some(symbol) = char::from_u32(BRAILLE_BLANK + dots).filter(|_| dots > 0) else {
                    continue;
                };
                if let Some(cell) = buf.cell_mut(Position::new(area.x + column, area.y + row)) {
                    cell.set_char(symbol).set_style(style);
                }
            }
        }
    }

    fn render_label(&self, area: Rect, buf: &mut Buffer) {
        let Some(label) = &self.label else {
            return;
        };
        let width = u16::try_from(label.width()).unwrap_or(u16::MAX).min(area.width);
        let x = area.x + (area.width - width) / 2;
        let y = area.y + (area.height - 1) / 2;
        buf.set_line(x, y, label, width);
    }
}

/// The share of a clockwise turn from twelve o'clock to the point `(x, y)`, y pointing down.
fn turn(x: f64, y: f64) -> f64 {
    let angle = x.atan2(-y);
    if angle < 0.0 {
        (angle + TAU) / TAU
    } else {
        angle / TAU
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use ratatui_core::style::Color;
    use rstest::rstest;

    use super::*;

    fn render(ring: &Ring<'_>, width: u16, height: u16) -> Buffer {
        let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
        ring.render(buf.area, &mut buf);
        buf
    }

    fn cells_with(buf: &Buffer, fg: Color) -> usize {
        buf.content().iter().filter(|cell| cell.fg == fg).count()
    }

    #[test]
    fn renders_a_braille_annulus_with_a_centred_label() {
        let ring = Ring::default().ratio(0.42).label("42%");

        let buf = render(&ring, 13, 6);

        assert_eq!(
            buf,
            Buffer::with_lines([
                "  ⣠⣴⣾⠿⠸⠿⣷⣦⣄  ",
                "⢀⣾⡟⠉     ⠉⢻⣷⡀",
                "⣾⡿   42%   ⢿⣷",
                "⢿⣷         ⣾⡿",
                "⠈⢿⣧⣀     ⣀⣼⡿⠁",
                "  ⠙⠻⢿⣶⣶⣶⡿⠙⠋  ",
            ])
        );
    }

    #[test]
    fn a_quarter_fills_only_the_top_right() {
        let ring = Ring::default()
            .ratio(0.25)
            .filled_style(Color::Green)
            .unfilled_style(Color::Gray);

        let buf = render(&ring, 13, 6);

        let filled: Vec<_> = buf
            .content()
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.fg == Color::Green)
            .map(|(index, _)| buf.pos_of(index))
            .collect();
        assert_ne!(filled, Vec::new());
        assert!(filled.iter().all(|&(x, y)| x >= 6 && y <= 3), "{filled:?}");
    }

    #[rstest]
    #[case::empty(0.0, Color::Green)]
    #[case::full(1.0, Color::Gray)]
    fn the_ends_of_the_range_use_one_style(#[case] ratio: f64, #[case] absent: Color) {
        let ring = Ring::default()
            .ratio(ratio)
            .filled_style(Color::Green)
            .unfilled_style(Color::Gray);

        let buf = render(&ring, 13, 6);

        assert_eq!(cells_with(&buf, absent), 0);
    }

    #[test]
    fn percent_sets_the_ratio() {
        assert_eq!(Ring::default().percent(50), Ring::default().ratio(0.5));
    }

    #[test]
    #[should_panic = "Ratio should be between 0 and 1 inclusively"]
    fn ratio_above_one_panics() {
        let _ = Ring::default().ratio(1.1);
    }

    #[test]
    #[should_panic = "Percentage should be between 0 and 100 inclusively"]
    fn percent_above_one_hundred_panics() {
        let _ = Ring::default().percent(101);
    }

    #[test]
    fn renders_inside_its_block() {
        let ring = Ring::default().block(Block::bordered()).ratio(0.5);

        let buf = render(&ring, 15, 8);

        assert_eq!(buf[(0, 0)].symbol(), "┌");
        assert_ne!(buf[(3, 1)].symbol(), " ");
    }

    #[test]
    fn style_fills_the_background() {
        let ring = Ring::default().style(Style::new().on_blue()).ratio(0.5);

        let buf = render(&ring, 13, 6);

        assert!(buf.content().iter().all(|cell| cell.bg == Color::Blue));
    }

    #[test]
    fn render_outside_the_buffer_is_a_noop() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 4, 4));

        Ring::default()
            .ratio(0.5)
            .render(Rect::new(10, 10, 6, 6), &mut buf);

        assert_eq!(buf, Buffer::empty(Rect::new(0, 0, 4, 4)));
    }
}
