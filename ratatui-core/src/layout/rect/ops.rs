use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use super::{Offset, Rect};
use crate::layout::Size;

impl Neg for Offset {
    type Output = Self;

    /// Negates the offset.
    ///
    /// # Panics
    ///
    /// Panics if the negated value overflows (i.e. `x` or `y` is `i32::MIN`).
    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

impl Add<Offset> for Rect {
    type Output = Self;

    /// Moves the rect by an offset without changing its size.
    ///
    /// If the offset would move the any of the rect's edges outside the bounds of `u16`, the
    /// rect's position is clamped to the nearest edge.
    fn add(self, offset: Offset) -> Self {
        let max_x = i32::from(u16::MAX - self.width);
        let max_y = i32::from(u16::MAX - self.height);
        let x = i32::from(self.x).saturating_add(offset.x).clamp(0, max_x) as u16;
        let y = i32::from(self.y).saturating_add(offset.y).clamp(0, max_y) as u16;
        Self { x, y, ..self }
    }
}

impl Add<Rect> for Offset {
    type Output = Rect;

    /// Moves the rect by an offset without changing its size.
    ///
    /// If the offset would move the any of the rect's edges outside the bounds of `u16`, the
    /// rect's position is clamped to the nearest edge.
    fn add(self, rect: Rect) -> Rect {
        rect + self
    }
}

impl Sub<Offset> for Rect {
    type Output = Self;

    /// Subtracts an offset from the rect without changing its size.
    ///
    /// If the offset would move the any of the rect's edges outside the bounds of `u16`, the
    /// rect's position is clamped to the nearest
    fn sub(self, offset: Offset) -> Self {
        // Note this cannot be simplified to `self + -offset` because `Offset::MIN` would overflow
        let max_x = i32::from(u16::MAX - self.width);
        let max_y = i32::from(u16::MAX - self.height);
        let x = i32::from(self.x).saturating_sub(offset.x).clamp(0, max_x) as u16;
        let y = i32::from(self.y).saturating_sub(offset.y).clamp(0, max_y) as u16;
        Self { x, y, ..self }
    }
}

impl AddAssign<Offset> for Rect {
    /// Moves the rect by an offset in place without changing its size.
    ///
    /// If the offset would move the any of the rect's edges outside the bounds of `u16`, the
    /// rect's position is clamped to the nearest edge.
    fn add_assign(&mut self, offset: Offset) {
        *self = *self + offset;
    }
}

impl SubAssign<Offset> for Rect {
    /// Moves the rect by an offset in place without changing its size.
    ///
    /// If the offset would move the any of the rect's edges outside the bounds of `u16`, the
    /// rect's position is clamped to the nearest edge.
    fn sub_assign(&mut self, offset: Offset) {
        *self = *self - offset;
    }
}

impl Add<Size> for Rect {
    type Output = Self;

    /// Adds a size to the rect.
    ///
    /// The width and height of the rect are increased by the size. The position is unchanged.
    ///
    /// If the size would cause the rect to overflow `u16`, the width and height are clamped so that
    /// the rect's bottom or right side is at the maximum `u16` value.
    fn add(self, size: Size) -> Self {
        let max_width = u16::MAX - self.x;
        let max_height = u16::MAX - self.y;
        Self {
            width: self.width.saturating_add(size.width).min(max_width),
            height: self.height.saturating_add(size.height).min(max_height),
            ..self
        }
    }
}

impl Add<Rect> for Size {
    type Output = Rect;

    /// Adds a size to the rect.
    ///
    /// The width and height of the rect are increased by the size. The position is unchanged.
    ///
    /// If the size would cause the rect to overflow `u16`, the width and height are clamped so that
    /// the rect's bottom or right side is at the maximum `u16` value.
    fn add(self, rect: Rect) -> Rect {
        rect + self
    }
}

impl Sub<Size> for Rect {
    type Output = Self;

    /// Subtracts a size from the rect.
    ///
    /// The width and height of the rect are decreased by the size. The position is unchanged.
    fn sub(self, size: Size) -> Self {
        Self {
            width: self.width.saturating_sub(size.width),
            height: self.height.saturating_sub(size.height),
            ..self
        }
    }
}

impl AddAssign<Size> for Rect {
    /// Adds a size to the rect in place.
    ///
    /// The width and height of the rect are increased by the size. The position is unchanged.
    ///
    /// If the size would cause the rect to overflow `u16`, the width and height are clamped so that
    /// the rect's bottom or right side is at the maximum `u16` value.
    fn add_assign(&mut self, size: Size) {
        *self = *self + size;
    }
}

impl SubAssign<Size> for Rect {
    /// Subtracts a size from the rect in place.
    ///
    /// The width and height of the rect are decreased by the size. The position is unchanged.
    fn sub_assign(&mut self, size: Size) {
        *self = *self - size;
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::zero(Rect::new(3, 4, 5, 6), Offset::ZERO, Rect::new(3, 4, 5, 6))]
    #[case::positive(Rect::new(3, 4, 5, 6), Offset::new(1, 2), Rect::new(4, 6, 5, 6))]
    #[case::negative(Rect::new(3, 4, 5, 6), Offset::new(-1, -2), Rect::new(2, 2, 5, 6))]
    #[case::saturate_negative(Rect::new(3, 4, 5, 6), Offset::MIN, Rect::new(0, 0, 5, 6))]
    #[case::saturate_positive(Rect::new(3, 4, 5, 6), Offset::MAX, Rect::new(u16::MAX- 5, u16::MAX - 6, 5, 6))]
    fn add_offset(#[case] rect: Rect, #[case] offset: Offset, #[case] expected: Rect) {
        assert_eq!(rect + offset, expected);
        assert_eq!(offset + rect, expected);
    }

    #[rstest]
    #[case::zero(Rect::new(3, 4, 5, 6), Offset::ZERO, Rect::new(3, 4, 5, 6))]
    #[case::positive(Rect::new(3, 4, 5, 6), Offset::new(1, 2), Rect::new(2, 2, 5, 6))]
    #[case::negative(Rect::new(3, 4, 5, 6), Offset::new(-1, -2), Rect::new(4, 6, 5, 6))]
    #[case::saturate_negative(Rect::new(3, 4, 5, 6), Offset::MAX, Rect::new(0, 0, 5, 6))]
    #[case::saturate_positive(Rect::new(3, 4, 5, 6), -Offset::MAX, Rect::new(u16::MAX - 5, u16::MAX - 6, 5, 6))]
    fn sub_offset(#[case] rect: Rect, #[case] offset: Offset, #[case] expected: Rect) {
        assert_eq!(rect - offset, expected);
    }

    #[rstest]
    #[case::zero(Rect::new(3, 4, 5, 6), Offset::ZERO, Rect::new(3, 4, 5, 6))]
    #[case::positive(Rect::new(3, 4, 5, 6), Offset::new(1, 2), Rect::new(4, 6, 5, 6))]
    #[case::negative(Rect::new(3, 4, 5, 6), Offset::new(-1, -2), Rect::new(2, 2, 5, 6))]
    #[case::saturate_negative(Rect::new(3, 4, 5, 6), Offset::MIN, Rect::new(0, 0, 5, 6))]
    #[case::saturate_positive(Rect::new(3, 4, 5, 6), Offset::MAX, Rect::new(u16::MAX - 5, u16::MAX - 6, 5, 6))]
    fn add_assign_offset(#[case] rect: Rect, #[case] offset: Offset, #[case] expected: Rect) {
        let mut rect = rect;
        rect += offset;
        assert_eq!(rect, expected);
    }

    #[rstest]
    #[case::zero(Rect::new(3, 4, 5, 6), Offset::ZERO, Rect::new(3, 4, 5, 6))]
    #[case::positive(Rect::new(3, 4, 5, 6), Offset::new(1, 2), Rect::new(2, 2, 5, 6))]
    #[case::negative(Rect::new(3, 4, 5, 6), Offset::new(-1, -2), Rect::new(4, 6, 5, 6))]
    #[case::saturate_negative(Rect::new(3, 4, 5, 6), Offset::MAX, Rect::new(0, 0, 5, 6))]
    #[case::saturate_positive(Rect::new(3, 4, 5, 6), -Offset::MAX, Rect::new(u16::MAX - 5, u16::MAX - 6, 5, 6))]
    fn sub_assign_offset(#[case] rect: Rect, #[case] offset: Offset, #[case] expected: Rect) {
        let mut rect = rect;
        rect -= offset;
        assert_eq!(rect, expected);
    }

    #[rstest]
    #[case::zero(Rect::new(3, 4, 5, 6), Size::ZERO, Rect::new(3, 4, 5, 6))]
    #[case::positive(Rect::new(3, 4, 5, 6), Size::new(1, 2), Rect::new(3, 4, 6, 8))]
    #[case::saturate_positive(Rect::new(3, 4, 5, 6), Size::MAX, Rect::new(3, 4, u16::MAX - 3, u16::MAX - 4))]
    fn add_size(#[case] rect: Rect, #[case] size: Size, #[case] expected: Rect) {
        assert_eq!(rect + size, expected);
        assert!(size + rect == expected);
    }

    #[rstest]
    #[case::zero(Rect::new(3, 4, 5, 6), Size::ZERO, Rect::new(3, 4, 5, 6))]
    #[case::positive(Rect::new(3, 4, 5, 6), Size::new(1, 2), Rect::new(3, 4, 4, 4))]
    #[case::saturate_negative(Rect::new(3, 4, 5, 6), Size::MAX, Rect::new(3, 4, 0, 0))]
    fn sub_size(#[case] rect: Rect, #[case] size: Size, #[case] expected: Rect) {
        assert_eq!(rect - size, expected);
    }

    #[rstest]
    #[case::zero(Rect::new(3, 4, 5, 6), Size::ZERO, Rect::new(3, 4, 5, 6))]
    #[case::positive(Rect::new(3, 4, 5, 6), Size::new(1, 2), Rect::new(3, 4, 6, 8))]
    #[case::saturate_positive(Rect::new(3, 4, 5, 6), Size::MAX, Rect::new(3, 4, u16::MAX - 3, u16::MAX - 4))]
    fn add_assign_size(#[case] rect: Rect, #[case] size: Size, #[case] expected: Rect) {
        let mut rect = rect;
        rect += size;
        assert_eq!(rect, expected);
    }

    #[rstest]
    #[case::zero(Rect::new(3, 4, 5, 6), Size::ZERO, Rect::new(3, 4, 5, 6))]
    #[case::positive(Rect::new(3, 4, 5, 6), Size::new(1, 2), Rect::new(3, 4, 4, 4))]
    #[case::saturate_negative(Rect::new(3, 4, 5, 6), Size::MAX, Rect::new(3, 4, 0, 0))]
    fn sub_assign_size(#[case] rect: Rect, #[case] size: Size, #[case] expected: Rect) {
        let mut rect = rect;
        rect -= size;
        assert_eq!(rect, expected);
    }
}
