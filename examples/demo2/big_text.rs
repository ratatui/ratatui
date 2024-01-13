//! [tui-big-text] is a rust crate that renders large pixel text as a [Ratatui] widget using the
//! glyphs from the [font8x8] crate.
//!
//! ![Hello World example](https://vhs.charm.sh/vhs-2UxNc2SJgiNqHoowbsXAMW.gif)
//!
//! # Installation
//!
//! ```shell
//! cargo add ratatui tui-big-text
//! ```
//!
//! # Usage
//!
//! Create a [`BigText`] widget using `BigTextBuilder` and pass it to [`Frame::render_widget`] to
//! render be rendered. The builder allows you to customize the [`Style`] of the widget and the
//! [`PixelSize`] of the glyphs. The [`PixelSize`] can be used to control how many character cells
//! are used to represent a single pixel of the 8x8 font.
//!
//! # Example
//!
//! ```rust
//! use anyhow::Result;
//! use ratatui::prelude::*;
//! use tui_big_text::{BigTextBuilder, PixelSize};
//!
//! fn render(frame: &mut Frame) -> Result<()> {
//!     let big_text = BigTextBuilder::default()
//!         .pixel_size(PixelSize::Full)
//!         .style(Style::new().blue())
//!         .lines(vec![
//!             "Hello".red().into(),
//!             "World".white().into(),
//!             "~~~~~".into(),
//!         ])
//!         .build()?;
//!     frame.render_widget(big_text, frame.size());
//!     Ok(())
//! }
//! ```
//!
//! [tui-big-text]: https://crates.io/crates/tui-big-text
//! [Ratatui]: https://crates.io/crates/ratatui
//! [font8x8]: https://crates.io/crates/font8x8
//! [`BigText`]: crate::BigText
//! [`PixelSize`]: crate::PixelSize
//! [`Frame::render_widget`]: ratatui::Frame::render_widget
//! [`Style`]: ratatui::style::Style

use std::cmp::min;

use derive_builder::Builder;
use font8x8::UnicodeFonts;
use ratatui::{prelude::*, text::StyledGrapheme, widgets::Widget};

#[allow(unused)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PixelSize {
    #[default]
    /// A pixel from the 8x8 font is represented by a full character cell in the terminal.
    Full,
    /// A pixel from the 8x8 font is represented by a half (upper/lower) character cell in the
    /// terminal.
    HalfHeight,
    /// A pixel from the 8x8 font is represented by a half (left/right) character cell in the
    /// terminal.
    HalfWidth,
    /// A pixel from the 8x8 font is represented by a quadrant of a character cell in the terminal.
    Quadrant,
}

/// Displays one or more lines of text using 8x8 pixel characters.
///
/// The text is rendered using the [font8x8](https://crates.io/crates/font8x8) crate.
///
/// Using the `pixel_size` method, you can also chose, how 'big' a pixel should be.
/// Currently a pixel of the 8x8 font can be represented by one full or half
/// (horizontal/vertical/both) character cell of the terminal.
///
/// # Examples
///
/// ```rust
/// use ratatui::prelude::*;
/// use tui_big_text::{BigTextBuilder, PixelSize};
///
/// BigText::builder()
///     .pixel_size(PixelSize::Full)
///     .style(Style::new().white())
///     .lines(vec![
///         "Hello".red().into(),
///         "World".blue().into(),
///         "=====".into(),
///     ])
///     .build();
/// ```
///
/// Renders:
///
/// ```plain
/// ██  ██           ███     ███
/// ██  ██            ██      ██
/// ██  ██   ████     ██      ██     ████
/// ██████  ██  ██    ██      ██    ██  ██
/// ██  ██  ██████    ██      ██    ██  ██
/// ██  ██  ██        ██      ██    ██  ██
/// ██  ██   ████    ████    ████    ████
///
/// ██   ██                  ███       ███
/// ██   ██                   ██        ██
/// ██   ██  ████   ██ ███    ██        ██
/// ██ █ ██ ██  ██   ███ ██   ██     █████
/// ███████ ██  ██   ██  ██   ██    ██  ██
/// ███ ███ ██  ██   ██       ██    ██  ██
/// ██   ██  ████   ████     ████    ███ ██
///
///  ███ ██  ███ ██  ███ ██  ███ ██  ███ ██
/// ██ ███  ██ ███  ██ ███  ██ ███  ██ ███
/// ```
#[derive(Debug, Builder, Clone, PartialEq, Eq, Hash)]
pub struct BigText<'a> {
    /// The text to display
    #[builder(setter(into))]
    lines: Vec<Line<'a>>,

    /// The style of the widget
    ///
    /// Defaults to `Style::default()`
    #[builder(default)]
    style: Style,

    /// The size of single glyphs
    ///
    /// Defaults to `BigTextSize::default()` (=> BigTextSize::Full)
    #[builder(default)]
    pixel_size: PixelSize,
}

impl Widget for BigText<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = layout(area, &self.pixel_size);
        for (line, line_layout) in self.lines.iter().zip(layout) {
            for (g, cell) in line.styled_graphemes(self.style).zip(line_layout) {
                render_symbol(g, cell, buf, &self.pixel_size);
            }
        }
    }
}

/// Returns how many cells are needed to display a full 8x8 glyphe using the given font size
fn cells_per_glyph(size: &PixelSize) -> (u16, u16) {
    match size {
        PixelSize::Full => (8, 8),
        PixelSize::HalfHeight => (8, 4),
        PixelSize::HalfWidth => (4, 8),
        PixelSize::Quadrant => (4, 4),
    }
}

/// Chunk the area into as many x*y cells as possible returned as a 2D iterator of `Rect`s
/// representing the rows of cells.
/// The size of each cell depends on given font size
fn layout(
    area: Rect,
    pixel_size: &PixelSize,
) -> impl IntoIterator<Item = impl IntoIterator<Item = Rect>> {
    let (width, height) = cells_per_glyph(pixel_size);
    (area.top()..area.bottom())
        .step_by(height as usize)
        .map(move |y| {
            (area.left()..area.right())
                .step_by(width as usize)
                .map(move |x| {
                    let width = min(area.right() - x, width);
                    let height = min(area.bottom() - y, height);
                    Rect::new(x, y, width, height)
                })
        })
}

/// Render a single grapheme into a cell by looking up the corresponding 8x8 bitmap in the
/// `BITMAPS` array and setting the corresponding cells in the buffer.
fn render_symbol(grapheme: StyledGrapheme, area: Rect, buf: &mut Buffer, pixel_size: &PixelSize) {
    buf.set_style(area, grapheme.style);
    let c = grapheme.symbol.chars().next().unwrap(); // TODO: handle multi-char graphemes
    if let Some(glyph) = font8x8::BASIC_FONTS.get(c) {
        render_glyph(glyph, area, buf, pixel_size);
    }
}

/// Get the correct unicode symbol for two vertical "pixels"
fn get_symbol_half_height(top: u8, bottom: u8) -> char {
    match top {
        0 => match bottom {
            0 => ' ',
            _ => '▄',
        },
        _ => match bottom {
            0 => '▀',
            _ => '█',
        },
    }
}

/// Get the correct unicode symbol for two horizontal "pixels"
fn get_symbol_half_width(left: u8, right: u8) -> char {
    match left {
        0 => match right {
            0 => ' ',
            _ => '▐',
        },
        _ => match right {
            0 => '▌',
            _ => '█',
        },
    }
}

/// Get the correct unicode symbol for 2x2 "pixels"
fn get_symbol_half_size(top_left: u8, top_right: u8, bottom_left: u8, bottom_right: u8) -> char {
    let top_left = if top_left > 0 { 1 } else { 0 };
    let top_right = if top_right > 0 { 1 } else { 0 };
    let bottom_left = if bottom_left > 0 { 1 } else { 0 };
    let bottom_right = if bottom_right > 0 { 1 } else { 0 };

    const QUADRANT_SYMBOLS: [char; 16] = [
        ' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█',
    ];
    QUADRANT_SYMBOLS[top_left + (top_right << 1) + (bottom_left << 2) + (bottom_right << 3)]
}

/// Render a single 8x8 glyph into a cell by setting the corresponding cells in the buffer.
fn render_glyph(glyph: [u8; 8], area: Rect, buf: &mut Buffer, pixel_size: &PixelSize) {
    let (width, height) = cells_per_glyph(pixel_size);

    let glyph_vertical_index = (0..glyph.len()).step_by(8 / height as usize);
    let glyph_horizontal_bit_selector = (0..8).step_by(8 / width as usize);

    for (row, y) in glyph_vertical_index.zip(area.top()..area.bottom()) {
        for (col, x) in glyph_horizontal_bit_selector
            .clone()
            .zip(area.left()..area.right())
        {
            let cell = buf.get_mut(x, y);
            let symbol_character = match pixel_size {
                PixelSize::Full => match glyph[row] & (1 << col) {
                    0 => ' ',
                    _ => '█',
                },
                PixelSize::HalfHeight => {
                    let top = glyph[row] & (1 << col);
                    let bottom = glyph[row + 1] & (1 << col);
                    get_symbol_half_height(top, bottom)
                }
                PixelSize::HalfWidth => {
                    let left = glyph[row] & (1 << col);
                    let right = glyph[row] & (1 << (col + 1));
                    get_symbol_half_width(left, right)
                }
                PixelSize::Quadrant => {
                    let top_left = glyph[row] & (1 << col);
                    let top_right = glyph[row] & (1 << (col + 1));
                    let bottom_left = glyph[row + 1] & (1 << col);
                    let bottom_right = glyph[row + 1] & (1 << (col + 1));
                    get_symbol_half_size(top_left, top_right, bottom_left, bottom_right)
                }
            };
            cell.set_char(symbol_character);
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::assert_buffer_eq;

    use super::*;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn build() -> Result<()> {
        let lines = vec![Line::from(vec!["Hello".red(), "World".blue()])];
        let style = Style::new().green();
        let pixel_size = PixelSize::default();
        assert_eq!(
            BigTextBuilder::default()
                .lines(lines.clone())
                .style(style)
                .build()?,
            BigText {
                lines,
                style,
                pixel_size
            }
        );
        Ok(())
    }

    #[test]
    fn render_single_line() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .lines(vec![Line::from("SingleLine")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 8));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            " ████     ██                     ███            ████      ██                    ",
            "██  ██                            ██             ██                             ",
            "███      ███    █████    ███ ██   ██     ████    ██      ███    █████    ████   ",
            " ███      ██    ██  ██  ██  ██    ██    ██  ██   ██       ██    ██  ██  ██  ██  ",
            "   ███    ██    ██  ██  ██  ██    ██    ██████   ██   █   ██    ██  ██  ██████  ",
            "██  ██    ██    ██  ██   █████    ██    ██       ██  ██   ██    ██  ██  ██      ",
            " ████    ████   ██  ██      ██   ████    ████   ███████  ████   ██  ██   ████   ",
            "                        █████                                                   ",
        ]);
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_truncated() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .lines(vec![Line::from("Truncated")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 70, 6));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "██████                                             █               ███",
            "█ ██ █                                            ██                ██",
            "  ██    ██ ███  ██  ██  █████    ████    ████    █████   ████       ██",
            "  ██     ███ ██ ██  ██  ██  ██  ██  ██      ██    ██    ██  ██   █████",
            "  ██     ██  ██ ██  ██  ██  ██  ██       █████    ██    ██████  ██  ██",
            "  ██     ██     ██  ██  ██  ██  ██  ██  ██  ██    ██ █  ██      ██  ██",
        ]);
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_multiple_lines() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .lines(vec![Line::from("Multi"), Line::from("Lines")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 16));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "██   ██          ███       █      ██    ",
            "███ ███           ██      ██            ",
            "███████ ██  ██    ██     █████   ███    ",
            "███████ ██  ██    ██      ██      ██    ",
            "██ █ ██ ██  ██    ██      ██      ██    ",
            "██   ██ ██  ██    ██      ██ █    ██    ",
            "██   ██  ███ ██  ████      ██    ████   ",
            "                                        ",
            "████      ██                            ",
            " ██                                     ",
            " ██      ███    █████    ████    █████  ",
            " ██       ██    ██  ██  ██  ██  ██      ",
            " ██   █   ██    ██  ██  ██████   ████   ",
            " ██  ██   ██    ██  ██  ██          ██  ",
            "███████  ████   ██  ██   ████   █████   ",
            "                                        ",
        ]);
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_widget_style() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .lines(vec![Line::from("Styled")])
            .style(Style::new().bold())
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 48, 8));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            " ████      █             ███               ███  ",
            "██  ██    ██              ██                ██  ",
            "███      █████  ██  ██    ██     ████       ██  ",
            " ███      ██    ██  ██    ██    ██  ██   █████  ",
            "   ███    ██    ██  ██    ██    ██████  ██  ██  ",
            "██  ██    ██ █   █████    ██    ██      ██  ██  ",
            " ████      ██       ██   ████    ████    ███ ██ ",
            "                █████                           ",
        ]);
        expected.set_style(Rect::new(0, 0, 48, 8), Style::new().bold());
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_line_style() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .lines(vec![
                Line::from("Red".red()),
                Line::from("Green".green()),
                Line::from("Blue".blue()),
            ])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 24));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "██████             ███                  ",
            " ██  ██             ██                  ",
            " ██  ██  ████       ██                  ",
            " █████  ██  ██   █████                  ",
            " ██ ██  ██████  ██  ██                  ",
            " ██  ██ ██      ██  ██                  ",
            "███  ██  ████    ███ ██                 ",
            "                                        ",
            "  ████                                  ",
            " ██  ██                                 ",
            "██      ██ ███   ████    ████   █████   ",
            "██       ███ ██ ██  ██  ██  ██  ██  ██  ",
            "██  ███  ██  ██ ██████  ██████  ██  ██  ",
            " ██  ██  ██     ██      ██      ██  ██  ",
            "  █████ ████     ████    ████   ██  ██  ",
            "                                        ",
            "██████   ███                            ",
            " ██  ██   ██                            ",
            " ██  ██   ██    ██  ██   ████           ",
            " █████    ██    ██  ██  ██  ██          ",
            " ██  ██   ██    ██  ██  ██████          ",
            " ██  ██   ██    ██  ██  ██              ",
            "██████   ████    ███ ██  ████           ",
            "                                        ",
        ]);
        expected.set_style(Rect::new(0, 0, 24, 8), Style::new().red());
        expected.set_style(Rect::new(0, 8, 40, 8), Style::new().green());
        expected.set_style(Rect::new(0, 16, 32, 8), Style::new().blue());
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_height_single_line() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![Line::from("SingleLine")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 4));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "▄█▀▀█▄    ▀▀                     ▀██            ▀██▀      ▀▀                    ",
            "▀██▄     ▀██    ██▀▀█▄  ▄█▀▀▄█▀   ██    ▄█▀▀█▄   ██      ▀██    ██▀▀█▄  ▄█▀▀█▄  ",
            "▄▄ ▀██    ██    ██  ██  ▀█▄▄██    ██    ██▀▀▀▀   ██  ▄█   ██    ██  ██  ██▀▀▀▀  ",
            " ▀▀▀▀    ▀▀▀▀   ▀▀  ▀▀  ▄▄▄▄█▀   ▀▀▀▀    ▀▀▀▀   ▀▀▀▀▀▀▀  ▀▀▀▀   ▀▀  ▀▀   ▀▀▀▀   ",
        ]);
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_height_truncated() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![Line::from("Truncated")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 70, 3));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "█▀██▀█                                            ▄█               ▀██",
            "  ██    ▀█▄█▀█▄ ██  ██  ██▀▀█▄  ▄█▀▀█▄   ▀▀▀█▄   ▀██▀▀  ▄█▀▀█▄   ▄▄▄██",
            "  ██     ██  ▀▀ ██  ██  ██  ██  ██  ▄▄  ▄█▀▀██    ██ ▄  ██▀▀▀▀  ██  ██",
        ]);
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_height_multiple_lines() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![Line::from("Multi"), Line::from("Lines")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 8));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "██▄ ▄██          ▀██      ▄█      ▀▀    ",
            "███████ ██  ██    ██     ▀██▀▀   ▀██    ",
            "██ ▀ ██ ██  ██    ██      ██ ▄    ██    ",
            "▀▀   ▀▀  ▀▀▀ ▀▀  ▀▀▀▀      ▀▀    ▀▀▀▀   ",
            "▀██▀      ▀▀                            ",
            " ██      ▀██    ██▀▀█▄  ▄█▀▀█▄  ▄█▀▀▀▀  ",
            " ██  ▄█   ██    ██  ██  ██▀▀▀▀   ▀▀▀█▄  ",
            "▀▀▀▀▀▀▀  ▀▀▀▀   ▀▀  ▀▀   ▀▀▀▀   ▀▀▀▀▀   ",
        ]);
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_height_widget_style() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![Line::from("Styled")])
            .style(Style::new().bold())
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 48, 4));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "▄█▀▀█▄    ▄█             ▀██               ▀██  ",
            "▀██▄     ▀██▀▀  ██  ██    ██    ▄█▀▀█▄   ▄▄▄██  ",
            "▄▄ ▀██    ██ ▄  ▀█▄▄██    ██    ██▀▀▀▀  ██  ██  ",
            " ▀▀▀▀      ▀▀   ▄▄▄▄█▀   ▀▀▀▀    ▀▀▀▀    ▀▀▀ ▀▀ ",
        ]);
        expected.set_style(Rect::new(0, 0, 48, 4), Style::new().bold());
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_height_line_style() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![
                Line::from("Red".red()),
                Line::from("Green".green()),
                Line::from("Blue".blue()),
            ])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 12));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "▀██▀▀█▄            ▀██                  ",
            " ██▄▄█▀ ▄█▀▀█▄   ▄▄▄██                  ",
            " ██ ▀█▄ ██▀▀▀▀  ██  ██                  ",
            "▀▀▀  ▀▀  ▀▀▀▀    ▀▀▀ ▀▀                 ",
            " ▄█▀▀█▄                                 ",
            "██      ▀█▄█▀█▄ ▄█▀▀█▄  ▄█▀▀█▄  ██▀▀█▄  ",
            "▀█▄ ▀██  ██  ▀▀ ██▀▀▀▀  ██▀▀▀▀  ██  ██  ",
            "  ▀▀▀▀▀ ▀▀▀▀     ▀▀▀▀    ▀▀▀▀   ▀▀  ▀▀  ",
            "▀██▀▀█▄  ▀██                            ",
            " ██▄▄█▀   ██    ██  ██  ▄█▀▀█▄          ",
            " ██  ██   ██    ██  ██  ██▀▀▀▀          ",
            "▀▀▀▀▀▀   ▀▀▀▀    ▀▀▀ ▀▀  ▀▀▀▀           ",
        ]);
        expected.set_style(Rect::new(0, 0, 24, 4), Style::new().red());
        expected.set_style(Rect::new(0, 4, 40, 4), Style::new().green());
        expected.set_style(Rect::new(0, 8, 32, 4), Style::new().blue());
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_width_single_line() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::HalfWidth)
            .lines(vec![Line::from("SingleLine")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 8));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "▐█▌  █          ▐█      ██   █          ",
            "█ █              █      ▐▌              ",
            "█▌  ▐█  ██▌ ▐█▐▌ █  ▐█▌ ▐▌  ▐█  ██▌ ▐█▌ ",
            "▐█   █  █ █ █ █  █  █ █ ▐▌   █  █ █ █ █ ",
            " ▐█  █  █ █ █ █  █  ███ ▐▌ ▌ █  █ █ ███ ",
            "█ █  █  █ █ ▐██  █  █   ▐▌▐▌ █  █ █ █   ",
            "▐█▌ ▐█▌ █ █   █ ▐█▌ ▐█▌ ███▌▐█▌ █ █ ▐█▌ ",
            "            ██▌                         ",
        ]);
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_width_truncated() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::HalfWidth)
            .lines(vec![Line::from("Truncated")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 35, 6));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "███                      ▐       ▐█",
            "▌█▐                      █        █",
            " █  █▐█ █ █ ██▌ ▐█▌ ▐█▌ ▐██ ▐█▌   █",
            " █  ▐█▐▌█ █ █ █ █ █   █  █  █ █ ▐██",
            " █  ▐▌▐▌█ █ █ █ █   ▐██  █  ███ █ █",
            " █  ▐▌  █ █ █ █ █ █ █ █  █▐ █   █ █",
        ]);
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_width_multiple_lines() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::HalfWidth)
            .lines(vec![Line::from("Multi"), Line::from("Lines")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 16));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "█ ▐▌    ▐█   ▐   █  ",
            "█▌█▌     █   █      ",
            "███▌█ █  █  ▐██ ▐█  ",
            "███▌█ █  █   █   █  ",
            "█▐▐▌█ █  █   █   █  ",
            "█ ▐▌█ █  █   █▐  █  ",
            "█ ▐▌▐█▐▌▐█▌  ▐▌ ▐█▌ ",
            "                    ",
            "██   █              ",
            "▐▌                  ",
            "▐▌  ▐█  ██▌ ▐█▌ ▐██ ",
            "▐▌   █  █ █ █ █ █   ",
            "▐▌ ▌ █  █ █ ███ ▐█▌ ",
            "▐▌▐▌ █  █ █ █     █ ",
            "███▌▐█▌ █ █ ▐█▌ ██▌ ",
            "                    ",
        ]);
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_width_widget_style() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::HalfWidth)
            .lines(vec![Line::from("Styled")])
            .style(Style::new().bold())
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 24, 8));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "▐█▌  ▐      ▐█       ▐█ ",
            "█ █  █       █        █ ",
            "█▌  ▐██ █ █  █  ▐█▌   █ ",
            "▐█   █  █ █  █  █ █ ▐██ ",
            " ▐█  █  █ █  █  ███ █ █ ",
            "█ █  █▐ ▐██  █  █   █ █ ",
            "▐█▌  ▐▌   █ ▐█▌ ▐█▌ ▐█▐▌",
            "        ██▌             ",
        ]);
        expected.set_style(Rect::new(0, 0, 24, 8), Style::new().bold());
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_width_line_style() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::HalfWidth)
            .lines(vec![
                Line::from("Red".red()),
                Line::from("Green".green()),
                Line::from("Blue".blue()),
            ])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 24));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "███      ▐█         ",
            "▐▌▐▌      █         ",
            "▐▌▐▌▐█▌   █         ",
            "▐██ █ █ ▐██         ",
            "▐▌█ ███ █ █         ",
            "▐▌▐▌█   █ █         ",
            "█▌▐▌▐█▌ ▐█▐▌        ",
            "                    ",
            " ██                 ",
            "▐▌▐▌                ",
            "█   █▐█ ▐█▌ ▐█▌ ██▌ ",
            "█   ▐█▐▌█ █ █ █ █ █ ",
            "█ █▌▐▌▐▌███ ███ █ █ ",
            "▐▌▐▌▐▌  █   █   █ █ ",
            " ██▌██  ▐█▌ ▐█▌ █ █ ",
            "                    ",
            "███ ▐█              ",
            "▐▌▐▌ █              ",
            "▐▌▐▌ █  █ █ ▐█▌     ",
            "▐██  █  █ █ █ █     ",
            "▐▌▐▌ █  █ █ ███     ",
            "▐▌▐▌ █  █ █ █       ",
            "███ ▐█▌ ▐█▐▌▐█▌     ",
            "                    ",
        ]);
        expected.set_style(Rect::new(0, 0, 12, 8), Style::new().red());
        expected.set_style(Rect::new(0, 8, 20, 8), Style::new().green());
        expected.set_style(Rect::new(0, 16, 16, 8), Style::new().blue());
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn check_half_size_symbols() -> Result<()> {
        assert_eq!(get_symbol_half_size(0, 0, 0, 0), ' ');
        assert_eq!(get_symbol_half_size(1, 0, 0, 0), '▘');
        assert_eq!(get_symbol_half_size(0, 1, 0, 0), '▝');
        assert_eq!(get_symbol_half_size(1, 1, 0, 0), '▀');
        assert_eq!(get_symbol_half_size(0, 0, 1, 0), '▖');
        assert_eq!(get_symbol_half_size(1, 0, 1, 0), '▌');
        assert_eq!(get_symbol_half_size(0, 1, 1, 0), '▞');
        assert_eq!(get_symbol_half_size(1, 1, 1, 0), '▛');
        assert_eq!(get_symbol_half_size(0, 0, 0, 1), '▗');
        assert_eq!(get_symbol_half_size(1, 0, 0, 1), '▚');
        assert_eq!(get_symbol_half_size(0, 1, 0, 1), '▐');
        assert_eq!(get_symbol_half_size(1, 1, 0, 1), '▜');
        assert_eq!(get_symbol_half_size(0, 0, 1, 1), '▄');
        assert_eq!(get_symbol_half_size(1, 0, 1, 1), '▙');
        assert_eq!(get_symbol_half_size(0, 1, 1, 1), '▟');
        assert_eq!(get_symbol_half_size(1, 1, 1, 1), '█');
        Ok(())
    }

    #[test]
    fn render_half_size_single_line() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::Quadrant)
            .lines(vec![Line::from("SingleLine")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 4));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "▟▀▙  ▀          ▝█      ▜▛   ▀          ",
            "▜▙  ▝█  █▀▙ ▟▀▟▘ █  ▟▀▙ ▐▌  ▝█  █▀▙ ▟▀▙ ",
            "▄▝█  █  █ █ ▜▄█  █  █▀▀ ▐▌▗▌ █  █ █ █▀▀ ",
            "▝▀▘ ▝▀▘ ▀ ▀ ▄▄▛ ▝▀▘ ▝▀▘ ▀▀▀▘▝▀▘ ▀ ▀ ▝▀▘ ",
        ]);
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_size_truncated() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::Quadrant)
            .lines(vec![Line::from("Truncated")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 35, 3));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "▛█▜                      ▟       ▝█",
            " █  ▜▟▜▖█ █ █▀▙ ▟▀▙ ▝▀▙ ▝█▀ ▟▀▙ ▗▄█",
            " █  ▐▌▝▘█ █ █ █ █ ▄ ▟▀█  █▗ █▀▀ █ █",
        ]);
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_size_multiple_lines() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::Quadrant)
            .lines(vec![Line::from("Multi"), Line::from("Lines")])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 8));
        big_text.render(buf.area, &mut buf);
        let expected = Buffer::with_lines(vec![
            "█▖▟▌    ▝█   ▟   ▀  ",
            "███▌█ █  █  ▝█▀ ▝█  ",
            "█▝▐▌█ █  █   █▗  █  ",
            "▀ ▝▘▝▀▝▘▝▀▘  ▝▘ ▝▀▘ ",
            "▜▛   ▀              ",
            "▐▌  ▝█  █▀▙ ▟▀▙ ▟▀▀ ",
            "▐▌▗▌ █  █ █ █▀▀ ▝▀▙ ",
            "▀▀▀▘▝▀▘ ▀ ▀ ▝▀▘ ▀▀▘ ",
        ]);
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_size_widget_style() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::Quadrant)
            .lines(vec![Line::from("Styled")])
            .style(Style::new().bold())
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 24, 4));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "▟▀▙  ▟      ▝█       ▝█ ",
            "▜▙  ▝█▀ █ █  █  ▟▀▙ ▗▄█ ",
            "▄▝█  █▗ ▜▄█  █  █▀▀ █ █ ",
            "▝▀▘  ▝▘ ▄▄▛ ▝▀▘ ▝▀▘ ▝▀▝▘",
        ]);
        expected.set_style(Rect::new(0, 0, 24, 4), Style::new().bold());
        assert_buffer_eq!(buf, expected);
        Ok(())
    }

    #[test]
    fn render_half_size_line_style() -> Result<()> {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::Quadrant)
            .lines(vec![
                Line::from("Red".red()),
                Line::from("Green".green()),
                Line::from("Blue".blue()),
            ])
            .build()?;
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 12));
        big_text.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
            "▜▛▜▖     ▝█         ",
            "▐▙▟▘▟▀▙ ▗▄█         ",
            "▐▌▜▖█▀▀ █ █         ",
            "▀▘▝▘▝▀▘ ▝▀▝▘        ",
            "▗▛▜▖                ",
            "█   ▜▟▜▖▟▀▙ ▟▀▙ █▀▙ ",
            "▜▖▜▌▐▌▝▘█▀▀ █▀▀ █ █ ",
            " ▀▀▘▀▀  ▝▀▘ ▝▀▘ ▀ ▀ ",
            "▜▛▜▖▝█              ",
            "▐▙▟▘ █  █ █ ▟▀▙     ",
            "▐▌▐▌ █  █ █ █▀▀     ",
            "▀▀▀ ▝▀▘ ▝▀▝▘▝▀▘     ",
        ]);
        expected.set_style(Rect::new(0, 0, 12, 4), Style::new().red());
        expected.set_style(Rect::new(0, 4, 20, 4), Style::new().green());
        expected.set_style(Rect::new(0, 8, 16, 4), Style::new().blue());
        assert_buffer_eq!(buf, expected);
        Ok(())
    }
}
