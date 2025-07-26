use strum::{Display, EnumString};

/// A type alias for `HorizontalAlignment`.
///
/// Prior to Ratatui 0.30.0, [`HorizontalAlignment`] was named `Alignment`. This alias is provided
/// for backwards compatibility. Because this type is used almost everywhere in Ratatui related apps
/// and libraries, it's unlikely that this alias will be removed in the future.
pub type Alignment = HorizontalAlignment;

/// Horizontal content alignment within a layout area.
///
/// Prior to Ratatui 0.30.0, this type was named `Alignment`. In Ratatui 0.30.0, the name was
/// changed to `HorizontalAlignment` to make it more descriptive. The old name is still available as
/// an alias for backwards compatibility.
///
/// This type is used throughout Ratatui to control how content is positioned horizontally within
/// available space. It's commonly used with widgets to control text alignment, but can also be
/// used in layout calculations.
///
/// For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HorizontalAlignment {
    #[default]
    Left,
    Center,
    Right,
}

/// Vertical content alignment within a layout area.
///
/// This type is used to control how content is positioned vertically within available space.
/// It complements [`HorizontalAlignment`] to provide full 2D positioning control.
///
/// For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
