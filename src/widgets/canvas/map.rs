use strum::{Display, EnumString};

use crate::{
    style::Color,
    widgets::canvas::{
        world::{WORLD_HIGH_RESOLUTION, WORLD_LOW_RESOLUTION},
        Painter, Shape,
    },
};

#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MapResolution {
    #[default]
    Low,
    High,
}

impl MapResolution {
    fn data(self) -> &'static [(f64, f64)] {
        match self {
            MapResolution::Low => &WORLD_LOW_RESOLUTION,
            MapResolution::High => &WORLD_HIGH_RESOLUTION,
        }
    }
}

/// Shape to draw a world map with the given resolution and color
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Map {
    pub resolution: MapResolution,
    pub color: Color,
}

impl Shape for Map {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.resolution.data() {
            if let Some((x, y)) = painter.get_point(*x, *y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::ParseError;

    use super::*;

    #[test]
    fn map_resolution_to_string() {
        assert_eq!(MapResolution::Low.to_string(), "Low");
        assert_eq!(MapResolution::High.to_string(), "High");
    }

    #[test]
    fn map_resolution_from_str() {
        assert_eq!("Low".parse(), Ok(MapResolution::Low));
        assert_eq!("High".parse(), Ok(MapResolution::High));
        assert_eq!(
            "".parse::<MapResolution>(),
            Err(ParseError::VariantNotFound)
        );
    }
}
