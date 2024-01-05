use strum::{Display, EnumString};

mod constraint;
#[allow(clippy::module_inception)]
mod layout;
mod margin;
mod rect;
mod segment_size;

pub use constraint::Constraint;
pub use layout::Layout;
pub use margin::Margin;
pub use rect::*;
pub use segment_size::SegmentSize;

/// A simple size struct
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Corner {
    #[default]
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    Horizontal,
    #[default]
    Vertical,
}

impl From<(u16, u16)> for Size {
    fn from((width, height): (u16, u16)) -> Self {
        Size { width, height }
    }
}

#[cfg(test)]
mod tests {
    use strum::ParseError;

    use super::*;

    #[test]
    fn corner_to_string() {
        assert_eq!(Corner::BottomLeft.to_string(), "BottomLeft");
        assert_eq!(Corner::BottomRight.to_string(), "BottomRight");
        assert_eq!(Corner::TopLeft.to_string(), "TopLeft");
        assert_eq!(Corner::TopRight.to_string(), "TopRight");
    }

    #[test]
    fn corner_from_str() {
        assert_eq!("BottomLeft".parse::<Corner>(), Ok(Corner::BottomLeft));
        assert_eq!("BottomRight".parse::<Corner>(), Ok(Corner::BottomRight));
        assert_eq!("TopLeft".parse::<Corner>(), Ok(Corner::TopLeft));
        assert_eq!("TopRight".parse::<Corner>(), Ok(Corner::TopRight));
        assert_eq!("".parse::<Corner>(), Err(ParseError::VariantNotFound));
    }

    #[test]
    fn direction_to_string() {
        assert_eq!(Direction::Horizontal.to_string(), "Horizontal");
        assert_eq!(Direction::Vertical.to_string(), "Vertical");
    }

    #[test]
    fn direction_from_str() {
        assert_eq!("Horizontal".parse::<Direction>(), Ok(Direction::Horizontal));
        assert_eq!("Vertical".parse::<Direction>(), Ok(Direction::Vertical));
        assert_eq!("".parse::<Direction>(), Err(ParseError::VariantNotFound));
    }

    #[test]
    fn alignment_to_string() {
        assert_eq!(Alignment::Left.to_string(), "Left");
        assert_eq!(Alignment::Center.to_string(), "Center");
        assert_eq!(Alignment::Right.to_string(), "Right");
    }

    #[test]
    fn alignment_from_str() {
        assert_eq!("Left".parse::<Alignment>(), Ok(Alignment::Left));
        assert_eq!("Center".parse::<Alignment>(), Ok(Alignment::Center));
        assert_eq!("Right".parse::<Alignment>(), Ok(Alignment::Right));
        assert_eq!("".parse::<Alignment>(), Err(ParseError::VariantNotFound));
    }
}
