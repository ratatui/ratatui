//! This module provides the `TestBackend` implementation for the [`Backend`] trait.
//! It is used in the integration tests to verify the correctness of the library.

use std::{
    fmt::{Display, Write},
    io,
};

use unicode_width::UnicodeWidthStr;

use crate::{
    backend::Backend,
    buffer::{Buffer, Cell},
    layout::Rect,
};

/// A backend used for the integration tests.
///
/// # Example
///
/// ```rust
/// use ratatui::{backend::{Backend, TestBackend}, buffer::Buffer};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut backend = TestBackend::new(10, 2);
/// backend.clear()?;
/// backend.assert_buffer(&Buffer::with_lines(vec!["          "; 2]));
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct TestBackend {
    width: u16,
    buffer: Buffer,
    height: u16,
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
                view.push_str(&c.symbol);
            } else {
                overwritten.push((x, &c.symbol));
            }
            skip = std::cmp::max(skip, c.symbol.width()).saturating_sub(1);
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
    /// Creates a new TestBackend with the specified width and height.
    pub fn new(width: u16, height: u16) -> TestBackend {
        TestBackend {
            width,
            height,
            buffer: Buffer::empty(Rect::new(0, 0, width, height)),
            cursor: false,
            pos: (0, 0),
        }
    }

    /// Returns a reference to the internal buffer of the TestBackend.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Resizes the TestBackend to the specified width and height.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.buffer.resize(Rect::new(0, 0, width, height));
        self.width = width;
        self.height = height;
    }

    /// Asserts that the TestBackend's buffer is equal to the expected buffer.
    /// If the buffers are not equal, a panic occurs with a detailed error message
    /// showing the differences between the expected and actual buffers.
    pub fn assert_buffer(&self, expected: &Buffer) {
        assert_eq!(expected.area, self.buffer.area);
        let diff = expected.diff(&self.buffer);
        if diff.is_empty() {
            return;
        }

        let mut debug_info = String::from("Buffers are not equal");
        debug_info.push('\n');
        debug_info.push_str("Expected:");
        debug_info.push('\n');
        let expected_view = buffer_view(expected);
        debug_info.push_str(&expected_view);
        debug_info.push('\n');
        debug_info.push_str("Got:");
        debug_info.push('\n');
        let view = buffer_view(&self.buffer);
        debug_info.push_str(&view);
        debug_info.push('\n');

        debug_info.push_str("Diff:");
        debug_info.push('\n');
        let nice_diff = diff
            .iter()
            .enumerate()
            .map(|(i, (x, y, cell))| {
                let expected_cell = expected.get(*x, *y);
                format!("{i}: at ({x}, {y}) expected {expected_cell:?} got {cell:?}")
            })
            .collect::<Vec<String>>()
            .join("\n");
        debug_info.push_str(&nice_diff);
        panic!("{debug_info}");
    }
}

impl Display for TestBackend {
    /// Formats the TestBackend for display by calling the buffer_view function
    /// on its internal buffer.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", buffer_view(&self.buffer))
    }
}

impl Backend for TestBackend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, c) in content {
            let cell = self.buffer.get_mut(x, y);
            *cell = c.clone();
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), io::Error> {
        self.cursor = false;
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), io::Error> {
        self.cursor = true;
        Ok(())
    }

    fn get_cursor(&mut self) -> Result<(u16, u16), io::Error> {
        Ok(self.pos)
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), io::Error> {
        self.pos = (x, y);
        Ok(())
    }

    fn clear(&mut self) -> Result<(), io::Error> {
        self.buffer.reset();
        Ok(())
    }

    fn size(&self) -> Result<Rect, io::Error> {
        Ok(Rect::new(0, 0, self.width, self.height))
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}
