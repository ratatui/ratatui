use strum::{Display, EnumString};

#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    Horizontal,
    #[default]
    Vertical,
}

#[cfg(test)]
mod tests {
    use strum::ParseError;

    use super::*;

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
}
