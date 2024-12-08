//! This module provides the `TestBackend` implementation for the [`Backend`] trait.
//! It is used in the integration tests to verify the correctness of the library.

use std::{
    fmt::{self, Write},
    io, iter,
};

use unicode_width::UnicodeWidthStr;

use crate::{
    backend::{Backend, ClearType, WindowSize},
    buffer::{Buffer, Cell},
    layout::{Position, Rect, Size},
};

/// A [`Backend`] implementation used for integration testing that renders to an memory buffer.
///
/// Note: that although many of the integration and unit tests in ratatui are written using this
/// backend, it is preferable to write unit tests for widgets directly against the buffer rather
/// than using this backend. This backend is intended for integration tests that test the entire
/// terminal UI.
///
/// # Example
///
/// ```rust
/// use ratatui::backend::{Backend, TestBackend};
///
/// let mut backend = TestBackend::new(10, 2);
/// backend.clear()?;
/// backend.assert_buffer_lines(["          "; 2]);
/// # std::io::Result::Ok(())
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TestBackend {
    buffer: Buffer,
    scrollback: Buffer,
    cursor: bool,
    pos: (u16, u16),
}

/// Returns a string representation of the given buffer for debugging purpose.
///
/// This function is used to visualize the buffer content in a human-readable format.
/// It iterates through the buffer content and appends each cell's symbol to the view string.
/// If a cell is hidden by a multi-width symbol, it is added to the overwritten vector and
/// displayed at the end of the line.
fn buffer_view(buffer: &Buffer) -> String {
    let mut view = String::with_capacity(buffer.content.len() + buffer.area.height as usize * 3);
    for cells in buffer.content.chunks(buffer.area.width as usize) {
        let mut overwritten = vec![];
        let mut skip: usize = 0;
        view.push('"');
        for (x, c) in cells.iter().enumerate() {
            if skip == 0 {
                view.push_str(c.symbol());
            } else {
                overwritten.push((x, c.symbol()));
            }
            skip = std::cmp::max(skip, c.symbol().width()).saturating_sub(1);
        }
        view.push('"');
        if !overwritten.is_empty() {
            write!(&mut view, " Hidden by multi-width symbols: {overwritten:?}").unwrap();
        }
        view.push('\n');
    }
    view
}

impl TestBackend {
    /// Creates a new `TestBackend` with the specified width and height.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            buffer: Buffer::empty(Rect::new(0, 0, width, height)),
            scrollback: Buffer::empty(Rect::new(0, 0, width, 0)),
            cursor: false,
            pos: (0, 0),
        }
    }

    /// Creates a new `TestBackend` with the specified lines as the initial screen state.
    ///
    /// The backend's screen size is determined from the initial lines.
    #[must_use]
    pub fn with_lines<'line, Lines>(lines: Lines) -> Self
    where
        Lines: IntoIterator,
        Lines::Item: Into<crate::text::Line<'line>>,
    {
        let buffer = Buffer::with_lines(lines);
        let scrollback = Buffer::empty(Rect {
            width: buffer.area.width,
            ..Rect::ZERO
        });
        Self {
            buffer,
            scrollback,
            cursor: false,
            pos: (0, 0),
        }
    }

    /// Returns a reference to the internal buffer of the `TestBackend`.
    pub const fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Returns a reference to the internal scrollback buffer of the `TestBackend`.
    ///
    /// The scrollback buffer represents the part of the screen that is currently hidden from view,
    /// but that could be accessed by scrolling back in the terminal's history. This would normally
    /// be done using the terminal's scrollbar or an equivalent keyboard shortcut.
    ///
    /// The scrollback buffer starts out empty. Lines are appended when they scroll off the top of
    /// the main buffer. This happens when lines are appended to the bottom of the main buffer
    /// using [`Backend::append_lines`].
    ///
    /// The scrollback buffer has a maximum height of [`u16::MAX`]. If lines are appended to the
    /// bottom of the scrollback buffer when it is at its maximum height, a corresponding number of
    /// lines will be removed from the top.
    pub const fn scrollback(&self) -> &Buffer {
        &self.scrollback
    }

    /// Resizes the `TestBackend` to the specified width and height.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.buffer.resize(Rect::new(0, 0, width, height));
        let scrollback_height = self.scrollback.area.height;
        self.scrollback
            .resize(Rect::new(0, 0, width, scrollback_height));
    }

    /// Asserts that the `TestBackend`'s buffer is equal to the expected buffer.
    ///
    /// This is a shortcut for `assert_eq!(self.buffer(), &expected)`.
    ///
    /// # Panics
    ///
    /// When they are not equal, a panic occurs with a detailed error message showing the
    /// differences between the expected and actual buffers.
    #[allow(deprecated)]
    #[track_caller]
    pub fn assert_buffer(&self, expected: &Buffer) {
        // TODO: use assert_eq!()
        crate::assert_buffer_eq!(&self.buffer, expected);
    }

    /// Asserts that the `TestBackend`'s scrollback buffer is equal to the expected buffer.
    ///
    /// This is a shortcut for `assert_eq!(self.scrollback(), &expected)`.
    ///
    /// # Panics
    ///
    /// When they are not equal, a panic occurs with a detailed error message showing the
    /// differences between the expected and actual buffers.
    #[track_caller]
    pub fn assert_scrollback(&self, expected: &Buffer) {
        assert_eq!(&self.scrollback, expected);
    }

    /// Asserts that the `TestBackend`'s scrollback buffer is empty.
    ///
    /// # Panics
    ///
    /// When the scrollback buffer is not equal, a panic occurs with a detailed error message
    /// showing the differences between the expected and actual buffers.
    pub fn assert_scrollback_empty(&self) {
        let expected = Buffer {
            area: Rect {
                width: self.scrollback.area.width,
                ..Rect::ZERO
            },
            content: vec![],
        };
        self.assert_scrollback(&expected);
    }

    /// Asserts that the `TestBackend`'s buffer is equal to the expected lines.
    ///
    /// This is a shortcut for `assert_eq!(self.buffer(), &Buffer::with_lines(expected))`.
    ///
    /// # Panics
    ///
    /// When they are not equal, a panic occurs with a detailed error message showing the
    /// differences between the expected and actual buffers.
    #[track_caller]
    pub fn assert_buffer_lines<'line, Lines>(&self, expected: Lines)
    where
        Lines: IntoIterator,
        Lines::Item: Into<crate::text::Line<'line>>,
    {
        self.assert_buffer(&Buffer::with_lines(expected));
    }

    /// Asserts that the `TestBackend`'s scrollback buffer is equal to the expected lines.
    ///
    /// This is a shortcut for `assert_eq!(self.scrollback(), &Buffer::with_lines(expected))`.
    ///
    /// # Panics
    ///
    /// When they are not equal, a panic occurs with a detailed error message showing the
    /// differences between the expected and actual buffers.
    #[track_caller]
    pub fn assert_scrollback_lines<'line, Lines>(&self, expected: Lines)
    where
        Lines: IntoIterator,
        Lines::Item: Into<crate::text::Line<'line>>,
    {
        self.assert_scrollback(&Buffer::with_lines(expected));
    }

    /// Asserts that the `TestBackend`'s cursor position is equal to the expected one.
    ///
    /// This is a shortcut for `assert_eq!(self.get_cursor_position().unwrap(), expected)`.
    ///
    /// # Panics
    ///
    /// When they are not equal, a panic occurs with a detailed error message showing the
    /// differences between the expected and actual position.
    #[track_caller]
    pub fn assert_cursor_position<P: Into<Position>>(&mut self, position: P) {
        let actual = self.get_cursor_position().unwrap();
        assert_eq!(actual, position.into());
    }
}

impl fmt::Display for TestBackend {
    /// Formats the `TestBackend` for display by calling the `buffer_view` function
    /// on its internal buffer.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", buffer_view(&self.buffer))
    }
}

impl Backend for TestBackend {
    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, c) in content {
            self.buffer[(x, y)] = c.clone();
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        self.cursor = false;
        Ok(())
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        self.cursor = true;
        Ok(())
    }

    fn get_cursor_position(&mut self) -> io::Result<Position> {
        Ok(self.pos.into())
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> io::Result<()> {
        self.pos = position.into().into();
        Ok(())
    }

    fn clear(&mut self) -> io::Result<()> {
        self.buffer.reset();
        Ok(())
    }

    fn clear_region(&mut self, clear_type: ClearType) -> io::Result<()> {
        let region = match clear_type {
            ClearType::All => return self.clear(),
            ClearType::AfterCursor => {
                let index = self.buffer.index_of(self.pos.0, self.pos.1) + 1;
                &mut self.buffer.content[index..]
            }
            ClearType::BeforeCursor => {
                let index = self.buffer.index_of(self.pos.0, self.pos.1);
                &mut self.buffer.content[..index]
            }
            ClearType::CurrentLine => {
                let line_start_index = self.buffer.index_of(0, self.pos.1);
                let line_end_index = self.buffer.index_of(self.buffer.area.width - 1, self.pos.1);
                &mut self.buffer.content[line_start_index..=line_end_index]
            }
            ClearType::UntilNewLine => {
                let index = self.buffer.index_of(self.pos.0, self.pos.1);
                let line_end_index = self.buffer.index_of(self.buffer.area.width - 1, self.pos.1);
                &mut self.buffer.content[index..=line_end_index]
            }
        };
        for cell in region {
            cell.reset();
        }
        Ok(())
    }

    /// Inserts n line breaks at the current cursor position.
    ///
    /// After the insertion, the cursor x position will be incremented by 1 (unless it's already
    /// at the end of line). This is a common behaviour of terminals in raw mode.
    ///
    /// If the number of lines to append is fewer than the number of lines in the buffer after the
    /// cursor y position then the cursor is moved down by n rows.
    ///
    /// If the number of lines to append is greater than the number of lines in the buffer after
    /// the cursor y position then that number of empty lines (at most the buffer's height in this
    /// case but this limit is instead replaced with scrolling in most backend implementations) will
    /// be added after the current position and the cursor will be moved to the last row.
    fn append_lines(&mut self, line_count: u16) -> io::Result<()> {
        let Position { x: cur_x, y: cur_y } = self.get_cursor_position()?;
        let Rect { width, height, .. } = self.buffer.area;

        // the next column ensuring that we don't go past the last column
        let new_cursor_x = cur_x.saturating_add(1).min(width.saturating_sub(1));

        let max_y = height.saturating_sub(1);
        let lines_after_cursor = max_y.saturating_sub(cur_y);

        if line_count > lines_after_cursor {
            // We need to insert blank lines at the bottom and scroll the lines from the top into
            // scrollback.
            let scroll_by: usize = (line_count - lines_after_cursor).into();
            let width: usize = self.buffer.area.width.into();
            let cells_to_scrollback = self.buffer.content.len().min(width * scroll_by);

            append_to_scrollback(
                &mut self.scrollback,
                self.buffer.content.splice(
                    0..cells_to_scrollback,
                    iter::repeat_with(Default::default).take(cells_to_scrollback),
                ),
            );
            self.buffer.content.rotate_left(cells_to_scrollback);
            append_to_scrollback(
                &mut self.scrollback,
                iter::repeat_with(Default::default).take(width * scroll_by - cells_to_scrollback),
            );
        }

        let new_cursor_y = cur_y.saturating_add(line_count).min(max_y);
        self.set_cursor_position(Position::new(new_cursor_x, new_cursor_y))?;

        Ok(())
    }

    fn size(&self) -> io::Result<Size> {
        Ok(self.buffer.area.as_size())
    }

    fn window_size(&mut self) -> io::Result<WindowSize> {
        // Some arbitrary window pixel size, probably doesn't need much testing.
        const WINDOW_PIXEL_SIZE: Size = Size {
            width: 640,
            height: 480,
        };
        Ok(WindowSize {
            columns_rows: self.buffer.area.as_size(),
            pixels: WINDOW_PIXEL_SIZE,
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    #[cfg(feature = "scrolling-regions")]
    fn scroll_region_up(&mut self, region: std::ops::Range<u16>, scroll_by: u16) -> io::Result<()> {
        let width: usize = self.buffer.area.width.into();
        let cell_region_start = width * region.start.min(self.buffer.area.height) as usize;
        let cell_region_end = width * region.end.min(self.buffer.area.height) as usize;
        let cell_region_len = cell_region_end - cell_region_start;
        let cells_to_scroll_by = width * scroll_by as usize;

        // Deal with the simple case where nothing needs to be copied into scrollback.
        if cell_region_start > 0 {
            if cells_to_scroll_by >= cell_region_len {
                // The scroll amount is large enough to clear the whole region.
                self.buffer.content[cell_region_start..cell_region_end].fill_with(Default::default);
            } else {
                // Scroll up by rotating, then filling in the bottom with empty cells.
                self.buffer.content[cell_region_start..cell_region_end]
                    .rotate_left(cells_to_scroll_by);
                self.buffer.content[cell_region_end - cells_to_scroll_by..cell_region_end]
                    .fill_with(Default::default);
            }
            return Ok(());
        }

        // The rows inserted into the scrollback will first come from the buffer, and if that is
        // insufficient, will then be blank rows.
        let cells_from_region = cell_region_len.min(cells_to_scroll_by);
        append_to_scrollback(
            &mut self.scrollback,
            self.buffer.content.splice(
                0..cells_from_region,
                iter::repeat_with(Default::default).take(cells_from_region),
            ),
        );
        if cells_to_scroll_by < cell_region_len {
            // Rotate the remaining cells to the front of the region.
            self.buffer.content[cell_region_start..cell_region_end].rotate_left(cells_from_region);
        } else {
            // Splice cleared out the region. Insert empty rows in scrollback.
            append_to_scrollback(
                &mut self.scrollback,
                iter::repeat_with(Default::default).take(cells_to_scroll_by - cell_region_len),
            );
        }
        Ok(())
    }

    #[cfg(feature = "scrolling-regions")]
    fn scroll_region_down(
        &mut self,
        region: std::ops::Range<u16>,
        scroll_by: u16,
    ) -> io::Result<()> {
        let width: usize = self.buffer.area.width.into();
        let cell_region_start = width * region.start.min(self.buffer.area.height) as usize;
        let cell_region_end = width * region.end.min(self.buffer.area.height) as usize;
        let cell_region_len = cell_region_end - cell_region_start;
        let cells_to_scroll_by = width * scroll_by as usize;

        if cells_to_scroll_by >= cell_region_len {
            // The scroll amount is large enough to clear the whole region.
            self.buffer.content[cell_region_start..cell_region_end].fill_with(Default::default);
        } else {
            // Scroll up by rotating, then filling in the top with empty cells.
            self.buffer.content[cell_region_start..cell_region_end]
                .rotate_right(cells_to_scroll_by);
            self.buffer.content[cell_region_start..cell_region_start + cells_to_scroll_by]
                .fill_with(Default::default);
        }
        Ok(())
    }
}

/// Append the provided cells to the bottom of a scrollback buffer. The number of cells must be a
/// multiple of the buffer's width. If the scrollback buffer ends up larger than 65535 lines tall,
/// then lines will be removed from the top to get it down to size.
fn append_to_scrollback(scrollback: &mut Buffer, cells: impl IntoIterator<Item = Cell>) {
    scrollback.content.extend(cells);
    let width = scrollback.area.width as usize;
    let new_height = (scrollback.content.len() / width).min(u16::MAX as usize);
    let keep_from = scrollback
        .content
        .len()
        .saturating_sub(width * u16::MAX as usize);
    scrollback.content.drain(0..keep_from);
    scrollback.area.height = new_height as u16;
}

#[cfg(test)]
mod tests {
    use itertools::Itertools as _;

    use super::*;

    #[test]
    fn new() {
        assert_eq!(
            TestBackend::new(10, 2),
            TestBackend {
                buffer: Buffer::with_lines(["          "; 2]),
                scrollback: Buffer::empty(Rect::new(0, 0, 10, 0)),
                cursor: false,
                pos: (0, 0),
            }
        );
    }
    #[test]
    fn test_buffer_view() {
        let buffer = Buffer::with_lines(["aaaa"; 2]);
        assert_eq!(buffer_view(&buffer), "\"aaaa\"\n\"aaaa\"\n");
    }

    #[test]
    fn buffer_view_with_overwrites() {
        let multi_byte_char = "👨‍👩‍👧‍👦"; // renders 2 wide
        let buffer = Buffer::with_lines([multi_byte_char]);
        assert_eq!(
            buffer_view(&buffer),
            format!(
                r#""{multi_byte_char}" Hidden by multi-width symbols: [(1, " ")]
"#,
            )
        );
    }

    #[test]
    fn buffer() {
        let backend = TestBackend::new(10, 2);
        backend.assert_buffer_lines(["          "; 2]);
    }

    #[test]
    fn resize() {
        let mut backend = TestBackend::new(10, 2);
        backend.resize(5, 5);
        backend.assert_buffer_lines(["     "; 5]);
    }

    #[test]
    fn assert_buffer() {
        let backend = TestBackend::new(10, 2);
        backend.assert_buffer_lines(["          "; 2]);
    }

    #[test]
    #[should_panic = "buffer contents not equal"]
    fn assert_buffer_panics() {
        let backend = TestBackend::new(10, 2);
        backend.assert_buffer_lines(["aaaaaaaaaa"; 2]);
    }

    #[test]
    #[should_panic = "assertion `left == right` failed"]
    fn assert_scrollback_panics() {
        let backend = TestBackend::new(10, 2);
        backend.assert_scrollback_lines(["aaaaaaaaaa"; 2]);
    }

    #[test]
    fn display() {
        let backend = TestBackend::new(10, 2);
        assert_eq!(format!("{backend}"), "\"          \"\n\"          \"\n");
    }

    #[test]
    fn draw() {
        let mut backend = TestBackend::new(10, 2);
        let cell = Cell::new("a");
        backend.draw([(0, 0, &cell)].into_iter()).unwrap();
        backend.draw([(0, 1, &cell)].into_iter()).unwrap();
        backend.assert_buffer_lines(["a         "; 2]);
    }

    #[test]
    fn hide_cursor() {
        let mut backend = TestBackend::new(10, 2);
        backend.hide_cursor().unwrap();
        assert!(!backend.cursor);
    }

    #[test]
    fn show_cursor() {
        let mut backend = TestBackend::new(10, 2);
        backend.show_cursor().unwrap();
        assert!(backend.cursor);
    }

    #[test]
    fn get_cursor_position() {
        let mut backend = TestBackend::new(10, 2);
        assert_eq!(backend.get_cursor_position().unwrap(), Position::ORIGIN);
    }

    #[test]
    fn assert_cursor_position() {
        let mut backend = TestBackend::new(10, 2);
        backend.assert_cursor_position(Position::ORIGIN);
    }

    #[test]
    fn set_cursor_position() {
        let mut backend = TestBackend::new(10, 10);
        backend
            .set_cursor_position(Position { x: 5, y: 5 })
            .unwrap();
        assert_eq!(backend.pos, (5, 5));
    }

    #[test]
    fn clear() {
        let mut backend = TestBackend::new(4, 2);
        let cell = Cell::new("a");
        backend.draw([(0, 0, &cell)].into_iter()).unwrap();
        backend.draw([(0, 1, &cell)].into_iter()).unwrap();
        backend.clear().unwrap();
        backend.assert_buffer_lines(["    ", "    "]);
    }

    #[test]
    fn clear_region_all() {
        let mut backend = TestBackend::with_lines([
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
        ]);

        backend.clear_region(ClearType::All).unwrap();
        backend.assert_buffer_lines([
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
        ]);
    }

    #[test]
    fn clear_region_after_cursor() {
        let mut backend = TestBackend::with_lines([
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
        ]);

        backend
            .set_cursor_position(Position { x: 3, y: 2 })
            .unwrap();
        backend.clear_region(ClearType::AfterCursor).unwrap();
        backend.assert_buffer_lines([
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaa      ",
            "          ",
            "          ",
        ]);
    }

    #[test]
    fn clear_region_before_cursor() {
        let mut backend = TestBackend::with_lines([
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
        ]);

        backend
            .set_cursor_position(Position { x: 5, y: 3 })
            .unwrap();
        backend.clear_region(ClearType::BeforeCursor).unwrap();
        backend.assert_buffer_lines([
            "          ",
            "          ",
            "          ",
            "     aaaaa",
            "aaaaaaaaaa",
        ]);
    }

    #[test]
    fn clear_region_current_line() {
        let mut backend = TestBackend::with_lines([
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
        ]);

        backend
            .set_cursor_position(Position { x: 3, y: 1 })
            .unwrap();
        backend.clear_region(ClearType::CurrentLine).unwrap();
        backend.assert_buffer_lines([
            "aaaaaaaaaa",
            "          ",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
        ]);
    }

    #[test]
    fn clear_region_until_new_line() {
        let mut backend = TestBackend::with_lines([
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
        ]);

        backend
            .set_cursor_position(Position { x: 3, y: 0 })
            .unwrap();
        backend.clear_region(ClearType::UntilNewLine).unwrap();
        backend.assert_buffer_lines([
            "aaa       ",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
        ]);
    }

    #[test]
    fn append_lines_not_at_last_line() {
        let mut backend = TestBackend::with_lines([
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);

        backend.set_cursor_position(Position::ORIGIN).unwrap();

        // If the cursor is not at the last line in the terminal the addition of a
        // newline simply moves the cursor down and to the right

        backend.append_lines(1).unwrap();
        backend.assert_cursor_position(Position { x: 1, y: 1 });

        backend.append_lines(1).unwrap();
        backend.assert_cursor_position(Position { x: 2, y: 2 });

        backend.append_lines(1).unwrap();
        backend.assert_cursor_position(Position { x: 3, y: 3 });

        backend.append_lines(1).unwrap();
        backend.assert_cursor_position(Position { x: 4, y: 4 });

        // As such the buffer should remain unchanged
        backend.assert_buffer_lines([
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);
        backend.assert_scrollback_empty();
    }

    #[test]
    fn append_lines_at_last_line() {
        let mut backend = TestBackend::with_lines([
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);

        // If the cursor is at the last line in the terminal the addition of a
        // newline will scroll the contents of the buffer
        backend
            .set_cursor_position(Position { x: 0, y: 4 })
            .unwrap();

        backend.append_lines(1).unwrap();

        backend.assert_buffer_lines([
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
            "          ",
        ]);
        backend.assert_scrollback_lines(["aaaaaaaaaa"]);

        // It also moves the cursor to the right, as is common of the behaviour of
        // terminals in raw-mode
        backend.assert_cursor_position(Position { x: 1, y: 4 });
    }

    #[test]
    fn append_multiple_lines_not_at_last_line() {
        let mut backend = TestBackend::with_lines([
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);

        backend.set_cursor_position(Position::ORIGIN).unwrap();

        // If the cursor is not at the last line in the terminal the addition of multiple
        // newlines simply moves the cursor n lines down and to the right by 1

        backend.append_lines(4).unwrap();
        backend.assert_cursor_position(Position { x: 1, y: 4 });

        // As such the buffer should remain unchanged
        backend.assert_buffer_lines([
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);
        backend.assert_scrollback_empty();
    }

    #[test]
    fn append_multiple_lines_past_last_line() {
        let mut backend = TestBackend::with_lines([
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);

        backend
            .set_cursor_position(Position { x: 0, y: 3 })
            .unwrap();

        backend.append_lines(3).unwrap();
        backend.assert_cursor_position(Position { x: 1, y: 4 });

        backend.assert_buffer_lines([
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
            "          ",
            "          ",
        ]);
        backend.assert_scrollback_lines(["aaaaaaaaaa", "bbbbbbbbbb"]);
    }

    #[test]
    fn append_multiple_lines_where_cursor_at_end_appends_height_lines() {
        let mut backend = TestBackend::with_lines([
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);

        backend
            .set_cursor_position(Position { x: 0, y: 4 })
            .unwrap();

        backend.append_lines(5).unwrap();
        backend.assert_cursor_position(Position { x: 1, y: 4 });

        backend.assert_buffer_lines([
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
        ]);
        backend.assert_scrollback_lines([
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);
    }

    #[test]
    fn append_multiple_lines_where_cursor_appends_height_lines() {
        let mut backend = TestBackend::with_lines([
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);

        backend.set_cursor_position(Position::ORIGIN).unwrap();

        backend.append_lines(5).unwrap();
        backend.assert_cursor_position(Position { x: 1, y: 4 });

        backend.assert_buffer_lines([
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
            "          ",
        ]);
        backend.assert_scrollback_lines(["aaaaaaaaaa"]);
    }

    #[test]
    fn append_multiple_lines_where_cursor_at_end_appends_more_than_height_lines() {
        let mut backend = TestBackend::with_lines([
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);

        backend
            .set_cursor_position(Position { x: 0, y: 4 })
            .unwrap();

        backend.append_lines(8).unwrap();
        backend.assert_cursor_position(Position { x: 1, y: 4 });

        backend.assert_buffer_lines([
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
        ]);
        backend.assert_scrollback_lines([
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
            "          ",
            "          ",
            "          ",
        ]);
    }

    #[test]
    fn append_lines_truncates_beyond_u16_max() -> io::Result<()> {
        let mut backend = TestBackend::new(10, 5);

        // Fill the scrollback with 65535 + 10 lines.
        let row_count = u16::MAX as usize + 10;
        for row in 0..=row_count {
            if row > 4 {
                backend.set_cursor_position(Position { x: 0, y: 4 })?;
                backend.append_lines(1)?;
            }
            let cells = format!("{row:>10}").chars().map(Cell::from).collect_vec();
            let content = cells
                .iter()
                .enumerate()
                .map(|(column, cell)| (column as u16, 4.min(row) as u16, cell));
            backend.draw(content)?;
        }

        // check that the buffer contains the last 5 lines appended
        backend.assert_buffer_lines([
            "     65541",
            "     65542",
            "     65543",
            "     65544",
            "     65545",
        ]);

        // TODO: ideally this should be something like:
        //     let lines = (6..=65545).map(|row| format!("{row:>10}"));
        //     backend.assert_scrollback_lines(lines);
        // but there's some truncation happening in Buffer::with_lines that needs to be fixed
        assert_eq!(
            Buffer {
                area: Rect::new(0, 0, 10, 5),
                content: backend.scrollback.content[0..10 * 5].to_vec(),
            },
            Buffer::with_lines([
                "         6",
                "         7",
                "         8",
                "         9",
                "        10",
            ]),
            "first 5 lines of scrollback should have been truncated"
        );

        assert_eq!(
            Buffer {
                area: Rect::new(0, 0, 10, 5),
                content: backend.scrollback.content[10 * 65530..10 * 65535].to_vec(),
            },
            Buffer::with_lines([
                "     65536",
                "     65537",
                "     65538",
                "     65539",
                "     65540",
            ]),
            "last 5 lines of scrollback should have been appended"
        );

        // These checks come after the content checks as otherwise we won't see the failing content
        // when these checks fail.
        // Make sure the scrollback is the right size.
        assert_eq!(backend.scrollback.area.width, 10);
        assert_eq!(backend.scrollback.area.height, 65535);
        assert_eq!(backend.scrollback.content.len(), 10 * 65535);
        Ok(())
    }

    #[test]
    fn size() {
        let backend = TestBackend::new(10, 2);
        assert_eq!(backend.size().unwrap(), Size::new(10, 2));
    }

    #[test]
    fn flush() {
        let mut backend = TestBackend::new(10, 2);
        backend.flush().unwrap();
    }

    #[cfg(feature = "scrolling-regions")]
    mod scrolling_regions {
        use rstest::rstest;

        use super::*;

        const A: &str = "aaaa";
        const B: &str = "bbbb";
        const C: &str = "cccc";
        const D: &str = "dddd";
        const E: &str = "eeee";
        const S: &str = "    ";

        #[rstest]
        #[case([A, B, C, D, E], 0..5, 0, [],                    [A, B, C, D, E])]
        #[case([A, B, C, D, E], 0..5, 2, [A, B],                [C, D, E, S, S])]
        #[case([A, B, C, D, E], 0..5, 5, [A, B, C, D, E],       [S, S, S, S, S])]
        #[case([A, B, C, D, E], 0..5, 7, [A, B, C, D, E, S, S], [S, S, S, S, S])]
        #[case([A, B, C, D, E], 0..3, 0, [],                    [A, B, C, D, E])]
        #[case([A, B, C, D, E], 0..3, 2, [A, B],                [C, S, S, D, E])]
        #[case([A, B, C, D, E], 0..3, 3, [A, B, C],             [S, S, S, D, E])]
        #[case([A, B, C, D, E], 0..3, 4, [A, B, C, S],          [S, S, S, D, E])]
        #[case([A, B, C, D, E], 1..4, 0, [],                    [A, B, C, D, E])]
        #[case([A, B, C, D, E], 1..4, 2, [],                    [A, D, S, S, E])]
        #[case([A, B, C, D, E], 1..4, 3, [],                    [A, S, S, S, E])]
        #[case([A, B, C, D, E], 1..4, 4, [],                    [A, S, S, S, E])]
        #[case([A, B, C, D, E], 0..0, 0, [],                    [A, B, C, D, E])]
        #[case([A, B, C, D, E], 0..0, 2, [S, S],                [A, B, C, D, E])]
        #[case([A, B, C, D, E], 2..2, 0, [],                    [A, B, C, D, E])]
        #[case([A, B, C, D, E], 2..2, 2, [],                    [A, B, C, D, E])]
        fn scroll_region_up<const L: usize, const M: usize, const N: usize>(
            #[case] initial_screen: [&'static str; L],
            #[case] range: std::ops::Range<u16>,
            #[case] scroll_by: u16,
            #[case] expected_scrollback: [&'static str; M],
            #[case] expected_buffer: [&'static str; N],
        ) {
            let mut backend = TestBackend::with_lines(initial_screen);
            backend.scroll_region_up(range, scroll_by).unwrap();
            if expected_scrollback.is_empty() {
                backend.assert_scrollback_empty();
            } else {
                backend.assert_scrollback_lines(expected_scrollback);
            }
            backend.assert_buffer_lines(expected_buffer);
        }

        #[rstest]
        #[case([A, B, C, D, E], 0..5, 0, [A, B, C, D, E])]
        #[case([A, B, C, D, E], 0..5, 2, [S, S, A, B, C])]
        #[case([A, B, C, D, E], 0..5, 5, [S, S, S, S, S])]
        #[case([A, B, C, D, E], 0..5, 7, [S, S, S, S, S])]
        #[case([A, B, C, D, E], 0..3, 0, [A, B, C, D, E])]
        #[case([A, B, C, D, E], 0..3, 2, [S, S, A, D, E])]
        #[case([A, B, C, D, E], 0..3, 3, [S, S, S, D, E])]
        #[case([A, B, C, D, E], 0..3, 4, [S, S, S, D, E])]
        #[case([A, B, C, D, E], 1..4, 0, [A, B, C, D, E])]
        #[case([A, B, C, D, E], 1..4, 2, [A, S, S, B, E])]
        #[case([A, B, C, D, E], 1..4, 3, [A, S, S, S, E])]
        #[case([A, B, C, D, E], 1..4, 4, [A, S, S, S, E])]
        #[case([A, B, C, D, E], 0..0, 0, [A, B, C, D, E])]
        #[case([A, B, C, D, E], 0..0, 2, [A, B, C, D, E])]
        #[case([A, B, C, D, E], 2..2, 0, [A, B, C, D, E])]
        #[case([A, B, C, D, E], 2..2, 2, [A, B, C, D, E])]
        fn scroll_region_down<const M: usize, const N: usize>(
            #[case] initial_screen: [&'static str; M],
            #[case] range: std::ops::Range<u16>,
            #[case] scroll_by: u16,
            #[case] expected_buffer: [&'static str; N],
        ) {
            let mut backend = TestBackend::with_lines(initial_screen);
            backend.scroll_region_down(range, scroll_by).unwrap();
            backend.assert_scrollback_empty();
            backend.assert_buffer_lines(expected_buffer);
        }
    }
}
