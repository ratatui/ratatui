use strum::{Display, EnumString};

/// A type alias for `HorizontalAlignment`.
///
/// Prior to Ratatui 0.30.0, [`HorizontalAlignment`] was named `Alignment`. This alias is provided
/// for backwards compatibility. Because this type is used almost everywhere in Ratatui related apps
/// and libraries, it's unlikely that this alias will be removed in the future.
pub type Alignment = HorizontalAlignment;

/// A type representing horizontal alignment.
///
/// Prior to Ratatui 0.30.0, this type was named `Alignment`. In Ratatui 0.30.0, the name was
/// changed to `HorizontalAlignment` to make it more descriptive. The old name is still available as
/// an alias for backwards compatibility.
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum HorizontalAlignment {
    #[default]
    Left,
    Center,
    Right,
}

/// A type representing vertical alignment.
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum VerticalAlignment {
    #[default]
    Top,
    Center,
    Bottom,
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

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

    #[test]
    fn vertical_alignment_to_string() {
        assert_eq!(VerticalAlignment::Top.to_string(), "Top");
        assert_eq!(VerticalAlignment::Center.to_string(), "Center");
        assert_eq!(VerticalAlignment::Bottom.to_string(), "Bottom");
    }

    #[test]
    fn vertical_alignment_from_str() {
        let top = "Top".parse::<VerticalAlignment>();
        assert_eq!(top, Ok(VerticalAlignment::Top));

        let center = "Center".parse::<VerticalAlignment>();
        assert_eq!(center, Ok(VerticalAlignment::Center));

        let bottom = "Bottom".parse::<VerticalAlignment>();
        assert_eq!(bottom, Ok(VerticalAlignment::Bottom));

        let invalid = "".parse::<VerticalAlignment>();
        assert_eq!(invalid, Err(ParseError::VariantNotFound));
    }
}
