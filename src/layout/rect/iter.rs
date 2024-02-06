use crate::prelude::*;

/// An iterator over rows within a `Rect`.
pub struct Rows {
    /// The `Rect` associated with the rows.
    pub rect: Rect,
    /// The y coordinate of the row within the `Rect`.
    pub current_row: u16,
}

impl Rows {
    /// Creates a new `Rows` iterator.
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            current_row: rect.y,
        }
    }
}

impl Iterator for Rows {
    type Item = Rect;

    /// Retrieves the next row within the `Rect`.
    ///
    /// Returns `None` when there are no more rows to iterate through.
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row >= self.rect.bottom() {
            return None;
        }
        let row = Rect::new(self.rect.x, self.current_row, self.rect.width, 1);
        self.current_row += 1;
        Some(row)
    }
}

/// An iterator over columns within a `Rect`.
pub struct Columns {
    /// The `Rect` associated with the columns.
    pub rect: Rect,
    /// The x coordinate of the column within the `Rect`.
    pub current_column: u16,
}

impl Columns {
    /// Creates a new `Columns` iterator.
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            current_column: rect.x,
        }
    }
}

impl Iterator for Columns {
    type Item = Rect;

    /// Retrieves the next column within the `Rect`.
    ///
    /// Returns `None` when there are no more columns to iterate through.
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_column >= self.rect.right() {
            return None;
        }
        let column = Rect::new(self.current_column, self.rect.y, 1, self.rect.height);
        self.current_column += 1;
        Some(column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows() {
        let rect = Rect::new(0, 0, 2, 2);
        let mut rows = Rows::new(rect);
        assert_eq!(rows.next(), Some(Rect::new(0, 0, 2, 1)));
        assert_eq!(rows.next(), Some(Rect::new(0, 1, 2, 1)));
        assert_eq!(rows.next(), None);
    }

    #[test]
    fn columns() {
        let rect = Rect::new(0, 0, 2, 2);
        let mut columns = Columns::new(rect);
        assert_eq!(columns.next(), Some(Rect::new(0, 0, 1, 2)));
        assert_eq!(columns.next(), Some(Rect::new(1, 0, 1, 2)));
        assert_eq!(columns.next(), None);
    }
}
