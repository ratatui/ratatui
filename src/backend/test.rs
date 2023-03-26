use unicode_width::UnicodeWidthStr;

use crate::{
    backend::Backend,
    buffer::{Buffer, Cell},
};
use std::{fmt::Write, io};

/// A backend used for the integration tests.
#[derive(Debug)]
pub struct TestBackend {
    buffer: Buffer,
    cursor: bool,
    pos: (u16, u16),
}

/// Returns a string representation of the given buffer for debugging purpose.
fn buffer_view(buffer: &Buffer) -> String {
    let mut view = String::with_capacity(buffer.cells.len());
    for cells in buffer.cells.chunks(buffer.get_width() as usize) {
        let mut overwritten = vec![];
        let mut skip: usize = 0;
        view.push('"');
        for (x, c) in cells.iter().enumerate() {
            if skip == 0 {
                view.push_str(&c.symbol);
            } else {
                overwritten.push((x, &c.symbol))
            }
            skip = std::cmp::max(skip, c.symbol.width()).saturating_sub(1);
        }
        view.push('"');
        if !overwritten.is_empty() {
            write!(
                &mut view,
                " Hidden by multi-width symbols: {:?}",
                overwritten
            )
            .unwrap();
        }
        view.push('\n');
    }
    view
}

impl TestBackend {
    pub fn new(width: u16, height: u16) -> TestBackend {
        TestBackend {
            buffer: Buffer::empty(width, height),
            cursor: false,
            pos: (0, 0),
        }
    }

    pub fn assert_buffer(&self, expected: &Buffer) {
        assert_eq!(expected.size(), self.buffer.size());
        if self.buffer.cells == expected.cells {
            return;
        }

        let nice_diff = self
            .buffer
            .cells
            .iter()
            .enumerate()
            .filter_map(|(i, got_cell)| {
                let expected_cell = &expected.cells[i];
                match got_cell != expected_cell {
                    true => {
                        let (x, y) = expected.pos_of(i);
                        Some(format!(
                            "{}: at ({}, {}) expected {:?} got {:?}",
                            i, x, y, expected_cell.symbol, got_cell.symbol
                        ))
                    }
                    false => None,
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        panic!(
            "Buffers are not equal:\nExpected:\n{}\nGot:\n{}\nDiff:\n{}\n",
            buffer_view(expected),
            buffer_view(&self.buffer),
            nice_diff
        );
    }
}

impl Backend for TestBackend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = &'a (u16, u16, &'a Cell)>,
    {
        for (x, y, c) in content {
            *self.buffer.get_mut(*x, *y) = (*c).clone();
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
        self.buffer.clear();
        Ok(())
    }

    fn dimensions(&self) -> io::Result<(u16, u16)> {
        Ok((self.buffer.get_width(), self.buffer.get_height()))
    }
}
