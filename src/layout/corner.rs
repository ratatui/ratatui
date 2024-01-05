use strum::{Display, EnumString};

#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Corner {
    #[default]
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
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
}
