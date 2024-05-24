use strum::{Display, EnumString};

#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}

#[cfg(test)]
mod tests {
    use strum::ParseError;

    use super::*;

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
