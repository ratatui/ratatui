use crate::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
};
use std::cmp::{max, min};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// A buffer cell
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub symbol: String,
    pub fg: Color,
    pub bg: Color,
    pub modifier: Modifier,
}

impl Cell {
    pub fn set_symbol(&mut self, symbol: &str) -> &mut Cell {
        self.symbol.clear();
        self.symbol.push_str(symbol);
        self
    }

    pub fn set_char(&mut self, ch: char) -> &mut Cell {
        self.symbol.clear();
        self.symbol.push(ch);
        self
    }

    pub fn set_fg(&mut self, color: Color) -> &mut Cell {
        self.fg = color;
        self
    }

    pub fn set_bg(&mut self, color: Color) -> &mut Cell {
        self.bg = color;
        self
    }

    pub fn set_style(&mut self, style: Style) -> &mut Cell {
        if let Some(c) = style.fg {
            self.fg = c;
        }
        if let Some(c) = style.bg {
            self.bg = c;
        }
        self.modifier.insert(style.add_modifier);
        self.modifier.remove(style.sub_modifier);
        self
    }

    pub fn style(&self) -> Style {
        Style::default()
            .fg(self.fg)
            .bg(self.bg)
            .add_modifier(self.modifier)
    }

    pub fn clear(&mut self) {
        self.symbol.clear();
        self.symbol.push(' ');
        self.fg = Color::Reset;
        self.bg = Color::Reset;
        self.modifier = Modifier::empty();
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            symbol: " ".into(),
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
        }
    }
}

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
/// use ratatui::buffer::{Buffer, Cell};
/// use ratatui::layout::Rect;
/// use ratatui::style::{Color, Style, Modifier};
///
/// let mut buf = Buffer::empty(10, 10);
/// buf.get_mut(0, 2).set_symbol("x");
/// assert_eq!(buf.get(0, 2).symbol, "x");
/// buf.set_string(3, 0, "string", Style::default().fg(Color::Red).bg(Color::White));
/// assert_eq!(buf.get(5, 0), &Cell{
///     symbol: String::from("r"),
///     fg: Color::Red,
///     bg: Color::White,
///     modifier: Modifier::empty()
/// });
/// buf.get_mut(5, 0).set_char('x');
/// assert_eq!(buf.get(5, 0).symbol, "x");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buffer {
    height: u16,
    width: u16,
    pub cells: Vec<Cell>,
}

impl Buffer {
    /// Returns a Buffer with all cells set to the default one
    pub fn empty(width: u16, height: u16) -> Buffer {
        Buffer::filled(width, height, &mut Cell::default())
    }

    /// Returns a Buffer with all cells initialized with the attributes of the given Cell
    pub fn filled(width: u16, height: u16, cell: &mut Cell) -> Buffer {
        let cells = vec![cell.clone(); width as usize * height as usize];
        Buffer {
            width,
            height,
            cells,
        }
    }

    /// Returns a unstyled Buffer containing the given lines
    pub fn with_lines<S>(lines: Vec<S>) -> Buffer
    where
        S: AsRef<str>,
    {
        let width = lines
            .iter()
            .map(|i| i.as_ref().width() as u16)
            .max()
            .unwrap_or_default();
        let height = lines.len() as u16;
        let mut buffer = Buffer::empty(width, height);
        for (line_nbr, line) in lines.iter().enumerate() {
            buffer.set_string(0, line_nbr as u16, line, Style::default());
        }
        buffer
    }

    /// Returns the content of the buffer as a slice
    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Returns a reference to Cell at the given coordinates
    pub fn get(&self, x: u16, y: u16) -> &Cell {
        let i = self.index_of(x, y);
        &self.cells[i]
    }

    /// Returns a mutable reference to Cell at the given coordinates
    pub fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell {
        let i = self.index_of(x, y);
        &mut self.cells[i]
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn get_region(&self, area: Rect) -> Vec<(u16, u16, &Cell)> {
        let mut sub_buffer = Vec::with_capacity(area.size());
        Self::map_buffer_region(area, |x, y| sub_buffer.push((x, y, self.get(x, y))));
        sub_buffer
    }

    fn map_buffer_region<F: FnMut(u16, u16)>(area: Rect, mut closure: F) {
        for i_y in area.y..(area.y + area.height) {
            for i_x in area.x..(area.x + area.width) {
                closure(i_x, i_y)
            }
        }
    }
    /// Returns the index in the `Vec<Cell>` for the given global (x, y) coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ratatui::buffer::Buffer;
    /// let buffer = Buffer::empty(10, 10);
    /// assert_eq!(buffer.index_of(9, 9), 99);
    /// ```
    /// # Panics
    ///
    /// Panics when given an coordinate that is outside of this Buffer's area.
    ///
    /// ```should_panic
    /// # use ratatui::buffer::Buffer;
    /// let buffer = Buffer::empty(10, 10);
    /// buffer.index_of(10, 10); // Panics
    /// ```
    pub fn index_of(&self, x: u16, y: u16) -> usize {
        debug_assert!(
            x < self.width && y < self.height,
            "Trying to access position outside the buffer: x={}, y={}, buffer size={}*{}",
            x,
            y,
            self.width,
            self.height
        );
        (y * self.width + x) as usize
    }

    /// Returns the (global) coordinates of a cell given its index
    ///
    /// # Examples
    ///
    /// ```
    /// # use ratatui::buffer::Buffer;
    /// let buffer = Buffer::empty(10, 10);
    /// assert_eq!(buffer.pos_of(0), (0, 0));
    /// assert_eq!(buffer.pos_of(14), (4, 1));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics when given an index that is outside the Buffer's content.
    ///
    /// ```should_panic
    /// # use ratatui::buffer::Buffer;
    /// let buffer = Buffer::empty(10, 10); // 100 cells in total
    /// // Index 100 is the 101th cell, which lies outside of the area of this Buffer.
    /// buffer.pos_of(100); // Panics
    /// ```
    pub fn pos_of(&self, i: usize) -> (u16, u16) {
        debug_assert!(
            i < self.cells.len(),
            "Trying to get the coords of a cell outside the buffer: i={} len={}",
            i,
            self.cells.len()
        );
        (
            (i % self.width as usize) as u16,
            (i / self.height as usize) as u16,
        )
    }

    /// Print a string, starting at the position (x, y)
    pub fn set_string<S>(&mut self, x: u16, y: u16, string: S, style: Style)
    where
        S: AsRef<str>,
    {
        self.set_stringn(x, y, string, self.get_width(), style);
    }

    /// Print at most the first n characters of a string if enough space is available
    /// until the end of the line
    pub fn set_stringn<S>(
        &mut self,
        x: u16,
        y: u16,
        string: S,
        width: u16,
        style: Style,
    ) -> (u16, u16)
    where
        S: AsRef<str>,
    {
        let mut index = self.index_of(x, y);
        let mut x = x;
        let max_width = min(self.get_width(), width.saturating_add(x));
        for grapheme in UnicodeSegmentation::graphemes(string.as_ref(), true) {
            let width = grapheme.width() as u16;
            if width == 0 {
                continue;
            }
            // `x_offset + width > max_offset` could be integer overflow on 32-bit machines if we
            // change dimensions to usize or u32 and someone resizes the terminal to 1x2^32.
            if width > max_width.saturating_sub(x) {
                break;
            }

            self.cells[index].set_symbol(grapheme);
            self.cells[index].set_style(style);
            // Reset following cells if multi-width (they would be hidden by the grapheme),
            for i in index + 1..index + width as usize {
                self.cells[i].clear();
            }
            index += width as usize;
            x += width;
        }
        (x, y)
    }

    pub fn set_spans(&mut self, x: u16, y: u16, spans: &Spans<'_>, width: u16) -> (u16, u16) {
        let mut remaining_width = width;
        let mut x = x;
        for span in &spans.0 {
            if remaining_width == 0 {
                break;
            }
            let pos = self.set_stringn(x, y, span.content.as_ref(), remaining_width, span.style);
            let w = pos.0.saturating_sub(x);
            x = pos.0;
            remaining_width = remaining_width.saturating_sub(w);
        }
        (x, y)
    }

    pub fn set_span(&mut self, x: u16, y: u16, span: &Span<'_>, width: u16) -> (u16, u16) {
        self.set_stringn(x, y, span.content.as_ref(), width, span.style)
    }

    #[deprecated(
        since = "0.10.0",
        note = "You should use styling capabilities of `Buffer::set_style`"
    )]
    pub fn set_background(&mut self, area: Rect, color: Color) {
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                self.get_mut(x, y).set_bg(color);
            }
        }
    }

    pub fn set_style(&mut self, area: Rect, style: Style) {
        Self::map_buffer_region(area, |x, y| {
            self.get_mut(x, y).set_style(style);
        });
    }

    /// Short-hand for width * height or cells.len()
    pub fn size(&self) -> usize {
        self.cells.len()
    }

    /// Makes sure the buffer can contain the given area.
    pub fn expand_if_needed(&mut self, area: Rect) {
        self.width = max(area.x + area.width, self.width);
        self.height = max(area.y + area.height, self.height);
        self.cells.resize(self.size(), Default::default());
    }

    /// Resize the buffer so that the mapped area matches the given area and that the buffer
    /// length is equal to area.size()
    pub fn resize(&mut self, width: u16, height: u16) {
        let size = width as usize * height as usize;
        self.cells.resize(size, Default::default());
        self.width = width;
        self.height = height;
    }

    /// Reset all cells in the buffer
    pub fn clear(&mut self) {
        for c in &mut self.cells {
            c.clear();
        }
    }

    pub fn clear_region(&mut self, area: Rect) {
        Self::map_buffer_region(area, |x, y| self.get_mut(x, y).clear())
    }

    pub fn fill_region(&mut self, area: Rect, cell: &Cell) {
        Self::map_buffer_region(area, |x, y| *self.get_mut(x, y) = cell.clone())
    }

    // Merge other buffer with self.
    pub fn merge(&mut self, mut other: Buffer) {
        let a_self = Rect::new(0, 0, self.width, self.height);
        let a_other = Rect::new(0, 0, other.width, other.height);
        let a_new = a_self.union(a_other);
        self.resize(a_new.width, a_new.height);

        for i in (0..other.size()).rev() {
            self.cells[i] = other.cells.swap_remove(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translates_to_and_from_coordinates() {
        let buf = Buffer::empty(10, 10);
        // First cell is at the upper left corner.
        assert_eq!(buf.pos_of(10), (0, 1));
        assert_eq!(buf.index_of(0, 1), 10);

        // Last cell is in the lower right.
        assert_eq!(buf.pos_of(99), (9, 9));
        assert_eq!(buf.index_of(9, 9), 99);
    }

    #[test]
    #[should_panic(expected = "outside the buffer")]
    fn pos_of_panics_on_out_of_bounds() {
        let buf = Buffer::empty(10, 10);

        // There are a total of 100 cells; zero-indexed means that 100 would be the 101st cell.
        buf.pos_of(100);
    }

    #[test]
    #[should_panic(expected = "outside the buffer")]
    fn index_of_panics_on_out_of_bounds() {
        let buf = Buffer::empty(10, 10);
        // width is 10; zero-indexed means that 10 would be the 11th cell.
        buf.index_of(10, 0);
    }

    #[test]
    fn buffer_set_string_zero_width() {
        let mut buffer = Buffer::empty(5, 1);
        buffer.set_stringn(0, 0, "aaa", 0, Style::default());
        assert_eq!(buffer, Buffer::with_lines(vec!["     "]));
    }

    #[test]
    fn buffer_set_string() {
        let mut buffer = Buffer::empty(5, 1);

        // Zero-width
        buffer.set_stringn(0, 0, "aaa", 0, Style::default());
        assert_eq!(buffer, Buffer::with_lines(vec!["     "]));

        buffer.set_string(0, 0, "aaa", Style::default());
        assert_eq!(buffer, Buffer::with_lines(vec!["aaa  "]));

        // Width limit:
        buffer.set_stringn(0, 0, "bbbbbbbbbbbbbb", 4, Style::default());
        assert_eq!(buffer, Buffer::with_lines(vec!["bbbb "]));

        buffer.set_string(0, 0, "12345", Style::default());
        assert_eq!(buffer, Buffer::with_lines(vec!["12345"]));

        // Width truncation:
        buffer.set_string(0, 0, "123456", Style::default());
        assert_eq!(buffer, Buffer::with_lines(vec!["12345"]));
    }

    #[test]
    fn buffer_set_string_single_width() {
        let mut buffer = Buffer::empty(1, 1);

        // Leading grapheme with zero width
        let s = "\u{1}a";
        buffer.set_stringn(0, 0, s, 1, Style::default());
        assert_eq!(buffer, Buffer::with_lines(vec!["a"]));

        // Trailing grapheme with zero with
        let s = "a\u{1}";
        buffer.set_stringn(0, 0, s, 1, Style::default());
        assert_eq!(buffer, Buffer::with_lines(vec!["a"]));
    }

    #[test]
    fn buffer_set_string_double_width() {
        let mut buffer = Buffer::empty(5, 1);
        buffer.set_string(0, 0, "コン", Style::default());
        assert_eq!(buffer, Buffer::with_lines(vec!["コン "]));

        // Only 1 space left.
        buffer.set_string(0, 0, "コンピ", Style::default());
        assert_eq!(buffer, Buffer::with_lines(vec!["コン "]));
    }

    #[test]
    fn buffer_with_lines() {
        let buffer =
            Buffer::with_lines(vec!["┌────────┐", "│コンピュ│", "│ーa 上で│", "└────────┘"]);
        assert_eq!(buffer.get(9, 0).symbol, format!("┐"));
        assert_eq!(buffer.get(0, 3).symbol, format!("└"));
    }

    #[test]
    fn buffer_merge_overrides_self() {
        let mut one = Buffer::filled(2, 2, Cell::default().set_symbol("1"));
        let two = Buffer::filled(2, 2, Cell::default().set_symbol("2"));
        one.merge(two);
        assert_eq!(one, Buffer::with_lines(vec!["22", "22"]));
    }

    #[test]
    fn buffer_merge_can_resize() {
        let mut one = Buffer::filled(1, 1, Cell::default().set_symbol("1"));
        let two = Buffer::filled(2, 2, Cell::default().set_symbol("2"));
        one.merge(two);
        let merged = Buffer::with_lines(vec!["22", "22"]);
        assert_eq!(one, merged);
    }

    #[test]
    fn buffer_merge_difference_is_self() {
        let mut one = Buffer::filled(2, 2, Cell::default().set_symbol("1"));
        let two = Buffer::filled(1, 1, Cell::default().set_symbol("2"));
        one.merge(two);
        let merged = Buffer::with_lines(vec!["21", "11"]);
        assert_eq!(one, merged);
    }
}
