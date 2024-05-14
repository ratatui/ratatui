use std::fmt;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{buffer::Cell, prelude::*};

/// A buffer that maps to the desired content of the terminal after the draw call
///
/// No widget in the library interacts directly with the terminal. Instead each of them is required
/// to draw their state to an intermediate buffer. It is basically a grid where each cell contains
/// a grapheme, a foreground color and a background color. This grid will then be used to output
/// the appropriate escape sequences and characters to draw the UI as the user has defined it.
///
/// # Examples:
///
/// ```
/// use ratatui::{buffer::Cell, prelude::*};
///
/// let mut buf = Buffer::empty(Rect {
///     x: 0,
///     y: 0,
///     width: 10,
///     height: 5,
/// });
/// buf.get_mut(0, 2).set_symbol("x");
/// assert_eq!(buf.get(0, 2).symbol(), "x");
///
/// buf.set_string(
///     3,
///     0,
///     "string",
///     Style::default().fg(Color::Red).bg(Color::White),
/// );
/// let cell = buf.get(5, 0);
/// assert_eq!(cell.symbol(), "r");
/// assert_eq!(cell.fg, Color::Red);
/// assert_eq!(cell.bg, Color::White);
///
/// buf.get_mut(5, 0).set_char('x');
/// assert_eq!(buf.get(5, 0).symbol(), "x");
/// ```
#[derive(Default, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Buffer {
    /// The area represented by this buffer
    pub area: Rect,
    /// The content of the buffer. The length of this Vec should always be equal to area.width *
    /// area.height
    pub content: Vec<Cell>,
}

impl Buffer {
    /// Returns a Buffer with all cells set to the default one
    #[must_use]
    pub fn empty(area: Rect) -> Self {
        Self::filled(area, &Cell::default())
    }

    /// Returns a Buffer with all cells initialized with the attributes of the given Cell
    #[must_use]
    pub fn filled(area: Rect, cell: &Cell) -> Self {
        let size = area.area() as usize;
        let content = vec![cell.clone(); size];
        Self { area, content }
    }

    /// Returns a Buffer containing the given lines
    #[must_use]
    pub fn with_lines<'a, Iter>(lines: Iter) -> Self
    where
        Iter: IntoIterator,
        Iter::Item: Into<Line<'a>>,
    {
        let lines = lines.into_iter().map(Into::into).collect::<Vec<_>>();
        let height = lines.len() as u16;
        let width = lines.iter().map(Line::width).max().unwrap_or_default() as u16;
        let mut buffer = Self::empty(Rect::new(0, 0, width, height));
        for (y, line) in lines.iter().enumerate() {
            buffer.set_line(0, y as u16, line, width);
        }
        buffer
    }

    /// Returns the content of the buffer as a slice
    pub fn content(&self) -> &[Cell] {
        &self.content
    }

    /// Returns the area covered by this buffer
    pub const fn area(&self) -> &Rect {
        &self.area
    }

    /// Returns a reference to Cell at the given coordinates
    #[track_caller]
    pub fn get(&self, x: u16, y: u16) -> &Cell {
        let i = self.index_of(x, y);
        &self.content[i]
    }

    /// Returns a mutable reference to Cell at the given coordinates
    #[track_caller]
    pub fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell {
        let i = self.index_of(x, y);
        &mut self.content[i]
    }

    /// Returns the index in the `Vec<Cell>` for the given global (x, y) coordinates.
    ///
    /// Global coordinates are offset by the Buffer's area offset (`x`/`y`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ratatui::prelude::*;
    /// let rect = Rect::new(200, 100, 10, 10);
    /// let buffer = Buffer::empty(rect);
    /// // Global coordinates to the top corner of this buffer's area
    /// assert_eq!(buffer.index_of(200, 100), 0);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics when given an coordinate that is outside of this Buffer's area.
    ///
    /// ```should_panic
    /// # use ratatui::prelude::*;
    /// let rect = Rect::new(200, 100, 10, 10);
    /// let buffer = Buffer::empty(rect);
    /// // Top coordinate is outside of the buffer in global coordinate space, as the Buffer's area
    /// // starts at (200, 100).
    /// buffer.index_of(0, 0); // Panics
    /// ```
    #[track_caller]
    pub fn index_of(&self, x: u16, y: u16) -> usize {
        debug_assert!(
            x >= self.area.left()
                && x < self.area.right()
                && y >= self.area.top()
                && y < self.area.bottom(),
            "Trying to access position outside the buffer: x={x}, y={y}, area={:?}",
            self.area
        );
        ((y - self.area.y) * self.area.width + (x - self.area.x)) as usize
    }

    /// Returns the (global) coordinates of a cell given its index
    ///
    /// Global coordinates are offset by the Buffer's area offset (`x`/`y`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ratatui::prelude::*;
    /// let rect = Rect::new(200, 100, 10, 10);
    /// let buffer = Buffer::empty(rect);
    /// assert_eq!(buffer.pos_of(0), (200, 100));
    /// assert_eq!(buffer.pos_of(14), (204, 101));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics when given an index that is outside the Buffer's content.
    ///
    /// ```should_panic
    /// # use ratatui::prelude::*;
    /// let rect = Rect::new(0, 0, 10, 10); // 100 cells in total
    /// let buffer = Buffer::empty(rect);
    /// // Index 100 is the 101th cell, which lies outside of the area of this Buffer.
    /// buffer.pos_of(100); // Panics
    /// ```
    pub fn pos_of(&self, i: usize) -> (u16, u16) {
        debug_assert!(
            i < self.content.len(),
            "Trying to get the coords of a cell outside the buffer: i={i} len={}",
            self.content.len()
        );
        (
            self.area.x + (i as u16) % self.area.width,
            self.area.y + (i as u16) / self.area.width,
        )
    }

    /// Print a string, starting at the position (x, y)
    pub fn set_string<T, S>(&mut self, x: u16, y: u16, string: T, style: S)
    where
        T: AsRef<str>,
        S: Into<Style>,
    {
        self.set_stringn(x, y, string, usize::MAX, style);
    }

    /// Print at most the first n characters of a string if enough space is available
    /// until the end of the line.
    ///
    /// Use [`Buffer::set_string`] when the maximum amount of characters can be printed.
    pub fn set_stringn<T, S>(
        &mut self,
        mut x: u16,
        y: u16,
        string: T,
        max_width: usize,
        style: S,
    ) -> (u16, u16)
    where
        T: AsRef<str>,
        S: Into<Style>,
    {
        let max_width = max_width.try_into().unwrap_or(u16::MAX);
        let mut remaining_width = self.area.right().saturating_sub(x).min(max_width);
        let graphemes = UnicodeSegmentation::graphemes(string.as_ref(), true)
            .map(|symbol| (symbol, symbol.width() as u16))
            .filter(|(_symbol, width)| *width > 0)
            .map_while(|(symbol, width)| {
                remaining_width = remaining_width.checked_sub(width)?;
                Some((symbol, width))
            });
        let style = style.into();
        for (symbol, width) in graphemes {
            self.get_mut(x, y).set_symbol(symbol).set_style(style);
            let next_symbol = x + width;
            x += 1;
            // Reset following cells if multi-width (they would be hidden by the grapheme),
            while x < next_symbol {
                self.get_mut(x, y).reset();
                x += 1;
            }
        }
        (x, y)
    }

    /// Print a line, starting at the position (x, y)
    pub fn set_line(&mut self, x: u16, y: u16, line: &Line<'_>, max_width: u16) -> (u16, u16) {
        let mut remaining_width = max_width;
        let mut x = x;
        for span in line {
            if remaining_width == 0 {
                break;
            }
            let pos = self.set_stringn(
                x,
                y,
                span.content.as_ref(),
                remaining_width as usize,
                line.style.patch(span.style),
            );
            let w = pos.0.saturating_sub(x);
            x = pos.0;
            remaining_width = remaining_width.saturating_sub(w);
        }
        (x, y)
    }

    /// Print a span, starting at the position (x, y)
    pub fn set_span(&mut self, x: u16, y: u16, span: &Span<'_>, max_width: u16) -> (u16, u16) {
        self.set_stringn(x, y, &span.content, max_width as usize, span.style)
    }

    /// Set the style of all cells in the given area.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    pub fn set_style<S: Into<Style>>(&mut self, area: Rect, style: S) {
        let style = style.into();
        let area = self.area.intersection(area);
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                self.get_mut(x, y).set_style(style);
            }
        }
    }

    /// Resize the buffer so that the mapped area matches the given area and that the buffer
    /// length is equal to area.width * area.height
    pub fn resize(&mut self, area: Rect) {
        let length = area.area() as usize;
        if self.content.len() > length {
            self.content.truncate(length);
        } else {
            self.content.resize(length, Cell::default());
        }
        self.area = area;
    }

    /// Reset all cells in the buffer
    pub fn reset(&mut self) {
        for c in &mut self.content {
            c.reset();
        }
    }

    /// Merge an other buffer into this one
    pub fn merge(&mut self, other: &Self) {
        let area = self.area.union(other.area);
        self.content.resize(area.area() as usize, Cell::default());

        // Move original content to the appropriate space
        let size = self.area.area() as usize;
        for i in (0..size).rev() {
            let (x, y) = self.pos_of(i);
            // New index in content
            let k = ((y - area.y) * area.width + x - area.x) as usize;
            if i != k {
                self.content[k] = self.content[i].clone();
                self.content[i] = Cell::default();
            }
        }

        // Push content of the other buffer into this one (may erase previous
        // data)
        let size = other.area.area() as usize;
        for i in 0..size {
            let (x, y) = other.pos_of(i);
            // New index in content
            let k = ((y - area.y) * area.width + x - area.x) as usize;
            self.content[k] = other.content[i].clone();
        }
        self.area = area;
    }

    /// Builds a minimal sequence of coordinates and Cells necessary to update the UI from
    /// self to other.
    ///
    /// We're assuming that buffers are well-formed, that is no double-width cell is followed by
    /// a non-blank cell.
    ///
    /// # Multi-width characters handling:
    ///
    /// ```text
    /// (Index:) `01`
    /// Prev:    `コ`
    /// Next:    `aa`
    /// Updates: `0: a, 1: a'
    /// ```
    ///
    /// ```text
    /// (Index:) `01`
    /// Prev:    `a `
    /// Next:    `コ`
    /// Updates: `0: コ` (double width symbol at index 0 - skip index 1)
    /// ```
    ///
    /// ```text
    /// (Index:) `012`
    /// Prev:    `aaa`
    /// Next:    `aコ`
    /// Updates: `0: a, 1: コ` (double width symbol at index 1 - skip index 2)
    /// ```
    pub fn diff<'a>(&self, other: &'a Self) -> Vec<(u16, u16, &'a Cell)> {
        let previous_buffer = &self.content;
        let next_buffer = &other.content;

        let mut updates: Vec<(u16, u16, &Cell)> = vec![];
        // Cells invalidated by drawing/replacing preceding multi-width characters:
        let mut invalidated: usize = 0;
        // Cells from the current buffer to skip due to preceding multi-width characters taking
        // their place (the skipped cells should be blank anyway), or due to per-cell-skipping:
        let mut to_skip: usize = 0;
        for (i, (current, previous)) in next_buffer.iter().zip(previous_buffer.iter()).enumerate() {
            if !current.skip && (current != previous || invalidated > 0) && to_skip == 0 {
                let (x, y) = self.pos_of(i);
                updates.push((x, y, &next_buffer[i]));
            }

            to_skip = current.symbol().width().saturating_sub(1);

            let affected_width = std::cmp::max(current.symbol().width(), previous.symbol().width());
            invalidated = std::cmp::max(affected_width, invalidated).saturating_sub(1);
        }
        updates
    }
}

impl fmt::Debug for Buffer {
    /// Writes a debug representation of the buffer to the given formatter.
    ///
    /// The format is like a pretty printed struct, with the following fields:
    /// * `area`: displayed as `Rect { x: 1, y: 2, width: 3, height: 4 }`
    /// * `content`: displayed as a list of strings representing the content of the buffer
    /// * `styles`: displayed as a list of: `{ x: 1, y: 2, fg: Color::Red, bg: Color::Blue,
    ///   modifier: Modifier::BOLD }` only showing a value when there is a change in style.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Buffer {{\n    area: {:?}", &self.area))?;

        if self.area.is_empty() {
            return f.write_str("\n}");
        }

        f.write_str(",\n    content: [\n")?;
        let mut last_style = None;
        let mut styles = vec![];
        for (y, line) in self.content.chunks(self.area.width as usize).enumerate() {
            let mut overwritten = vec![];
            let mut skip: usize = 0;
            f.write_str("        \"")?;
            for (x, c) in line.iter().enumerate() {
                if skip == 0 {
                    f.write_str(c.symbol())?;
                } else {
                    overwritten.push((x, c.symbol()));
                }
                skip = std::cmp::max(skip, c.symbol().width()).saturating_sub(1);
                #[cfg(feature = "underline-color")]
                {
                    let style = (c.fg, c.bg, c.underline_color, c.modifier);
                    if last_style != Some(style) {
                        last_style = Some(style);
                        styles.push((x, y, c.fg, c.bg, c.underline_color, c.modifier));
                    }
                }
                #[cfg(not(feature = "underline-color"))]
                {
                    let style = (c.fg, c.bg, c.modifier);
                    if last_style != Some(style) {
                        last_style = Some(style);
                        styles.push((x, y, c.fg, c.bg, c.modifier));
                    }
                }
            }
            f.write_str("\",")?;
            if !overwritten.is_empty() {
                f.write_fmt(format_args!(
                    " // hidden by multi-width symbols: {overwritten:?}"
                ))?;
            }
            f.write_str("\n")?;
        }
        f.write_str("    ],\n    styles: [\n")?;
        for s in styles {
            #[cfg(feature = "underline-color")]
            f.write_fmt(format_args!(
                "        x: {}, y: {}, fg: {:?}, bg: {:?}, underline: {:?}, modifier: {:?},\n",
                s.0, s.1, s.2, s.3, s.4, s.5
            ))?;
            #[cfg(not(feature = "underline-color"))]
            f.write_fmt(format_args!(
                "        x: {}, y: {}, fg: {:?}, bg: {:?}, modifier: {:?},\n",
                s.0, s.1, s.2, s.3, s.4
            ))?;
        }
        f.write_str("    ]\n}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use itertools::Itertools;
    use rstest::{fixture, rstest};

    use super::*;

    fn cell(s: &str) -> Cell {
        let mut cell = Cell::default();
        cell.set_symbol(s);
        cell
    }

    #[test]
    fn debug_empty_buffer() {
        let buffer = Buffer::empty(Rect::ZERO);
        let result = format!("{buffer:?}");
        println!("{result}");
        let expected = "Buffer {\n    area: Rect { x: 0, y: 0, width: 0, height: 0 }\n}";
        assert_eq!(result, expected);
    }

    #[cfg(feature = "underline-color")]
    #[test]
    fn debug_grapheme_override() {
        let buffer = Buffer::with_lines(["a🦀b"]);
        let result = format!("{buffer:?}");
        println!("{result}");
        let expected = indoc::indoc!(
            r#"
            Buffer {
                area: Rect { x: 0, y: 0, width: 4, height: 1 },
                content: [
                    "a🦀b", // hidden by multi-width symbols: [(2, " ")]
                ],
                styles: [
                    x: 0, y: 0, fg: Reset, bg: Reset, underline: Reset, modifier: NONE,
                ]
            }"#
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn debug_some_example() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 12, 2));
        buffer.set_string(0, 0, "Hello World!", Style::default());
        buffer.set_string(
            0,
            1,
            "G'day World!",
            Style::default()
                .fg(Color::Green)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
        let result = format!("{buffer:?}");
        println!("{result}");
        #[cfg(feature = "underline-color")]
        let expected = indoc::indoc!(
            r#"
            Buffer {
                area: Rect { x: 0, y: 0, width: 12, height: 2 },
                content: [
                    "Hello World!",
                    "G'day World!",
                ],
                styles: [
                    x: 0, y: 0, fg: Reset, bg: Reset, underline: Reset, modifier: NONE,
                    x: 0, y: 1, fg: Green, bg: Yellow, underline: Reset, modifier: BOLD,
                ]
            }"#
        );
        #[cfg(not(feature = "underline-color"))]
        let expected = indoc::indoc!(
            r#"
            Buffer {
                area: Rect { x: 0, y: 0, width: 12, height: 2 },
                content: [
                    "Hello World!",
                    "G'day World!",
                ],
                styles: [
                    x: 0, y: 0, fg: Reset, bg: Reset, modifier: NONE,
                    x: 0, y: 1, fg: Green, bg: Yellow, modifier: BOLD,
                ]
            }"#
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn it_translates_to_and_from_coordinates() {
        let rect = Rect::new(200, 100, 50, 80);
        let buf = Buffer::empty(rect);

        // First cell is at the upper left corner.
        assert_eq!(buf.pos_of(0), (200, 100));
        assert_eq!(buf.index_of(200, 100), 0);

        // Last cell is in the lower right.
        assert_eq!(buf.pos_of(buf.content.len() - 1), (249, 179));
        assert_eq!(buf.index_of(249, 179), buf.content.len() - 1);
    }

    #[test]
    #[should_panic(expected = "outside the buffer")]
    fn pos_of_panics_on_out_of_bounds() {
        let rect = Rect::new(0, 0, 10, 10);
        let buf = Buffer::empty(rect);

        // There are a total of 100 cells; zero-indexed means that 100 would be the 101st cell.
        buf.pos_of(100);
    }

    #[test]
    #[should_panic(expected = "outside the buffer")]
    fn index_of_panics_on_out_of_bounds() {
        let rect = Rect::new(0, 0, 10, 10);
        let buf = Buffer::empty(rect);

        // width is 10; zero-indexed means that 10 would be the 11th cell.
        buf.index_of(10, 0);
    }

    #[test]
    fn set_string() {
        let area = Rect::new(0, 0, 5, 1);
        let mut buffer = Buffer::empty(area);

        // Zero-width
        buffer.set_stringn(0, 0, "aaa", 0, Style::default());
        assert_eq!(buffer, Buffer::with_lines(["     "]));

        buffer.set_string(0, 0, "aaa", Style::default());
        assert_eq!(buffer, Buffer::with_lines(["aaa  "]));

        // Width limit:
        buffer.set_stringn(0, 0, "bbbbbbbbbbbbbb", 4, Style::default());
        assert_eq!(buffer, Buffer::with_lines(["bbbb "]));

        buffer.set_string(0, 0, "12345", Style::default());
        assert_eq!(buffer, Buffer::with_lines(["12345"]));

        // Width truncation:
        buffer.set_string(0, 0, "123456", Style::default());
        assert_eq!(buffer, Buffer::with_lines(["12345"]));

        // multi-line
        buffer = Buffer::empty(Rect::new(0, 0, 5, 2));
        buffer.set_string(0, 0, "12345", Style::default());
        buffer.set_string(0, 1, "67890", Style::default());
        assert_eq!(buffer, Buffer::with_lines(["12345", "67890"]));
    }

    #[test]
    fn set_string_multi_width_overwrite() {
        let area = Rect::new(0, 0, 5, 1);
        let mut buffer = Buffer::empty(area);

        // multi-width overwrite
        buffer.set_string(0, 0, "aaaaa", Style::default());
        buffer.set_string(0, 0, "称号", Style::default());
        assert_eq!(buffer, Buffer::with_lines(["称号a"]));
    }

    #[test]
    fn set_string_zero_width() {
        let area = Rect::new(0, 0, 1, 1);
        let mut buffer = Buffer::empty(area);

        // Leading grapheme with zero width
        let s = "\u{1}a";
        buffer.set_stringn(0, 0, s, 1, Style::default());
        assert_eq!(buffer, Buffer::with_lines(["a"]));

        // Trailing grapheme with zero with
        let s = "a\u{1}";
        buffer.set_stringn(0, 0, s, 1, Style::default());
        assert_eq!(buffer, Buffer::with_lines(["a"]));
    }

    #[test]
    fn set_string_double_width() {
        let area = Rect::new(0, 0, 5, 1);
        let mut buffer = Buffer::empty(area);
        buffer.set_string(0, 0, "コン", Style::default());
        assert_eq!(buffer, Buffer::with_lines(["コン "]));

        // Only 1 space left.
        buffer.set_string(0, 0, "コンピ", Style::default());
        assert_eq!(buffer, Buffer::with_lines(["コン "]));
    }

    #[fixture]
    fn small_one_line_buffer() -> Buffer {
        Buffer::empty(Rect::new(0, 0, 5, 1))
    }

    #[rstest]
    #[case::empty("", "     ")]
    #[case::one("1", "1    ")]
    #[case::full("12345", "12345")]
    #[case::overflow("123456", "12345")]
    fn set_line_raw(
        mut small_one_line_buffer: Buffer,
        #[case] content: &str,
        #[case] expected: &str,
    ) {
        let line = Line::raw(content);
        small_one_line_buffer.set_line(0, 0, &line, 5);

        // note: testing with empty / set_string here instead of with_lines because with_lines calls
        // set_line
        let mut expected_buffer = Buffer::empty(small_one_line_buffer.area);
        expected_buffer.set_string(0, 0, expected, Style::default());
        assert_eq!(small_one_line_buffer, expected_buffer);
    }

    #[rstest]
    #[case::empty("", "     ")]
    #[case::one("1", "1    ")]
    #[case::full("12345", "12345")]
    #[case::overflow("123456", "12345")]
    fn set_line_styled(
        mut small_one_line_buffer: Buffer,
        #[case] content: &str,
        #[case] expected: &str,
    ) {
        let color = Color::Blue;
        let line = Line::styled(content, color);
        small_one_line_buffer.set_line(0, 0, &line, 5);

        // note: manually testing the contents here as the Buffer::with_lines calls set_line
        let actual_contents = small_one_line_buffer
            .content
            .iter()
            .map(Cell::symbol)
            .join("");
        let actual_styles = small_one_line_buffer
            .content
            .iter()
            .map(|c| c.fg)
            .collect_vec();

        // set_line only sets the style for non-empty cells (unlike Line::render which sets the
        // style for all cells)
        let expected_styles = iter::repeat(color)
            .take(content.len().min(5))
            .chain(iter::repeat(Color::default()).take(5_usize.saturating_sub(content.len())))
            .collect_vec();
        assert_eq!(actual_contents, expected);
        assert_eq!(actual_styles, expected_styles);
    }

    #[test]
    fn set_style() {
        let mut buffer = Buffer::with_lines(["aaaaa", "bbbbb", "ccccc"]);
        buffer.set_style(Rect::new(0, 1, 5, 1), Style::new().red());
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "aaaaa".into(),
            "bbbbb".red(),
            "ccccc".into(),
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn set_style_does_not_panic_when_out_of_area() {
        let mut buffer = Buffer::with_lines(["aaaaa", "bbbbb", "ccccc"]);
        buffer.set_style(Rect::new(0, 1, 10, 3), Style::new().red());
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "aaaaa".into(),
            "bbbbb".red(),
            "ccccc".red(),
        ]);
        assert_eq!(buffer, expected);
    }

    #[test]
    fn with_lines() {
        #[rustfmt::skip]
        let buffer = Buffer::with_lines([
            "┌────────┐",
            "│コンピュ│",
            "│ーa 上で│",
            "└────────┘",
        ]);
        assert_eq!(buffer.area.x, 0);
        assert_eq!(buffer.area.y, 0);
        assert_eq!(buffer.area.width, 10);
        assert_eq!(buffer.area.height, 4);
    }

    #[test]
    fn diff_empty_empty() {
        let area = Rect::new(0, 0, 40, 40);
        let prev = Buffer::empty(area);
        let next = Buffer::empty(area);
        let diff = prev.diff(&next);
        assert_eq!(diff, vec![]);
    }

    #[test]
    fn diff_empty_filled() {
        let area = Rect::new(0, 0, 40, 40);
        let prev = Buffer::empty(area);
        let next = Buffer::filled(area, Cell::default().set_symbol("a"));
        let diff = prev.diff(&next);
        assert_eq!(diff.len(), 40 * 40);
    }

    #[test]
    fn diff_filled_filled() {
        let area = Rect::new(0, 0, 40, 40);
        let prev = Buffer::filled(area, Cell::default().set_symbol("a"));
        let next = Buffer::filled(area, Cell::default().set_symbol("a"));
        let diff = prev.diff(&next);
        assert_eq!(diff, vec![]);
    }

    #[test]
    fn diff_single_width() {
        let prev = Buffer::with_lines([
            "          ",
            "┌Title─┐  ",
            "│      │  ",
            "│      │  ",
            "└──────┘  ",
        ]);
        let next = Buffer::with_lines([
            "          ",
            "┌TITLE─┐  ",
            "│      │  ",
            "│      │  ",
            "└──────┘  ",
        ]);
        let diff = prev.diff(&next);
        assert_eq!(
            diff,
            vec![
                (2, 1, &cell("I")),
                (3, 1, &cell("T")),
                (4, 1, &cell("L")),
                (5, 1, &cell("E")),
            ]
        );
    }

    #[test]
    #[rustfmt::skip]
    fn diff_multi_width() {
        let prev = Buffer::with_lines([
            "┌Title─┐  ",
            "└──────┘  ",
        ]);
        let next = Buffer::with_lines([
            "┌称号──┐  ",
            "└──────┘  ",
        ]);
        let diff = prev.diff(&next);
        assert_eq!(
            diff,
            vec![
                (1, 0, &cell("称")),
                // Skipped "i"
                (3, 0, &cell("号")),
                // Skipped "l"
                (5, 0, &cell("─")),
            ]
        );
    }

    #[test]
    fn diff_multi_width_offset() {
        let prev = Buffer::with_lines(["┌称号──┐"]);
        let next = Buffer::with_lines(["┌─称号─┐"]);

        let diff = prev.diff(&next);
        assert_eq!(
            diff,
            vec![(1, 0, &cell("─")), (2, 0, &cell("称")), (4, 0, &cell("号")),]
        );
    }

    #[test]
    fn diff_skip() {
        let prev = Buffer::with_lines(["123"]);
        let mut next = Buffer::with_lines(["456"]);
        for i in 1..3 {
            next.content[i].set_skip(true);
        }

        let diff = prev.diff(&next);
        assert_eq!(diff, vec![(0, 0, &cell("4"))],);
    }

    #[test]
    fn merge() {
        let mut one = Buffer::filled(
            Rect {
                x: 0,
                y: 0,
                width: 2,
                height: 2,
            },
            Cell::default().set_symbol("1"),
        );
        let two = Buffer::filled(
            Rect {
                x: 0,
                y: 2,
                width: 2,
                height: 2,
            },
            Cell::default().set_symbol("2"),
        );
        one.merge(&two);
        assert_eq!(one, Buffer::with_lines(["11", "11", "22", "22"]));
    }

    #[test]
    fn merge2() {
        let mut one = Buffer::filled(
            Rect {
                x: 2,
                y: 2,
                width: 2,
                height: 2,
            },
            Cell::default().set_symbol("1"),
        );
        let two = Buffer::filled(
            Rect {
                x: 0,
                y: 0,
                width: 2,
                height: 2,
            },
            Cell::default().set_symbol("2"),
        );
        one.merge(&two);
        assert_eq!(one, Buffer::with_lines(["22  ", "22  ", "  11", "  11"]));
    }

    #[test]
    fn merge3() {
        let mut one = Buffer::filled(
            Rect {
                x: 3,
                y: 3,
                width: 2,
                height: 2,
            },
            Cell::default().set_symbol("1"),
        );
        let two = Buffer::filled(
            Rect {
                x: 1,
                y: 1,
                width: 3,
                height: 4,
            },
            Cell::default().set_symbol("2"),
        );
        one.merge(&two);
        let mut merged = Buffer::with_lines(["222 ", "222 ", "2221", "2221"]);
        merged.area = Rect {
            x: 1,
            y: 1,
            width: 4,
            height: 4,
        };
        assert_eq!(one, merged);
    }

    #[test]
    fn merge_skip() {
        let mut one = Buffer::filled(
            Rect {
                x: 0,
                y: 0,
                width: 2,
                height: 2,
            },
            Cell::default().set_symbol("1"),
        );
        let two = Buffer::filled(
            Rect {
                x: 0,
                y: 1,
                width: 2,
                height: 2,
            },
            Cell::default().set_symbol("2").set_skip(true),
        );
        one.merge(&two);
        let skipped: Vec<bool> = one.content().iter().map(|c| c.skip).collect();
        assert_eq!(skipped, vec![false, false, true, true, true, true]);
    }

    #[test]
    fn merge_skip2() {
        let mut one = Buffer::filled(
            Rect {
                x: 0,
                y: 0,
                width: 2,
                height: 2,
            },
            Cell::default().set_symbol("1").set_skip(true),
        );
        let two = Buffer::filled(
            Rect {
                x: 0,
                y: 1,
                width: 2,
                height: 2,
            },
            Cell::default().set_symbol("2"),
        );
        one.merge(&two);
        let skipped: Vec<bool> = one.content().iter().map(|c| c.skip).collect();
        assert_eq!(skipped, vec![true, true, false, false, false, false]);
    }

    #[test]
    fn with_lines_accepts_into_lines() {
        use crate::style::Stylize;
        let mut buf = Buffer::empty(Rect::new(0, 0, 3, 2));
        buf.set_string(0, 0, "foo", Style::new().red());
        buf.set_string(0, 1, "bar", Style::new().blue());
        assert_eq!(buf, Buffer::with_lines(["foo".red(), "bar".blue()]));
    }
}
