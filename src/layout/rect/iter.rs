use self::layout::Position;
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
    pub const fn new(rect: Rect) -> Self {
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
    pub const fn new(rect: Rect) -> Self {
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

/// An iterator over positions within a `Rect`.
///
/// The iterator will yield all positions within the `Rect` in a row-major order.
pub struct Positions {
    /// The `Rect` associated with the positions.
    pub rect: Rect,
    /// The current position within the `Rect`.
    pub current_position: Position,
}

impl Positions {
    /// Creates a new `Positions` iterator.
    pub const fn new(rect: Rect) -> Self {
        Self {
            rect,
            current_position: Position::new(rect.x, rect.y),
        }
    }
}

impl Iterator for Positions {
    type Item = Position;

    /// Retrieves the next position within the `Rect`.
    ///
    /// Returns `None` when there are no more positions to iterate through.
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_position.y >= self.rect.bottom() {
            return None;
        }
        let position = self.current_position;
        self.current_position.x += 1;
        if self.current_position.x >= self.rect.right() {
            self.current_position.x = self.rect.x;
            self.current_position.y += 1;
        }
        Some(position)
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

    #[test]
    fn positions() {
        let rect = Rect::new(0, 0, 2, 2);
        let mut positions = Positions::new(rect);
        assert_eq!(positions.next(), Some(Position::new(0, 0)));
        assert_eq!(positions.next(), Some(Position::new(1, 0)));
        assert_eq!(positions.next(), Some(Position::new(0, 1)));
        assert_eq!(positions.next(), Some(Position::new(1, 1)));
        assert_eq!(positions.next(), None);
    }
}
