use crate::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

/// A shape to draw a group of points with the given color
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Points<'a> {
    pub coords: &'a [(f64, f64)],
    pub color: Color,
}

impl<'a> Shape for Points<'a> {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.coords {
            if let Some((x, y)) = painter.get_point(*x, *y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}
