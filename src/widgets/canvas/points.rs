use crate::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

/// A shape to draw a group of points with the given color
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Points<'a, I>
where
    I: Iterator<Item = &'a (f64, f64)>,
{
    pub coords: I,
    pub color: Color,
}

impl<'a, I> Shape for Points<'a, I>
where
    I: Iterator<Item = &'a (f64, f64)> + Clone,
{
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.coords.clone() {
            if let Some((x, y)) = painter.get_point(*x, *y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}
