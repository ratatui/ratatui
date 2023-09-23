/// Amounts by which to move a [`Rect`](super::Rect).
///
/// Positive numbers move to the right/bottom and negative to the left/top.
///
/// See [`Rect::offset`](super::Rect::offset)
#[derive(Debug, Default, Clone, Copy)]
pub struct Offset {
    /// How much to move on the X axis
    pub x: i32,
    /// How much to move on the Y axis
    pub y: i32,
}
