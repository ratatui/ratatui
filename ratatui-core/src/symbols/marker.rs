use strum::{Display, EnumString};

pub const DOT: &str = "•";

/// Marker to use when plotting data points
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Marker {
    /// One point per cell in shape of dot (`•`)
    #[default]
    Dot,
    /// One point per cell in shape of a block (`█`)
    Block,
    /// One point per cell in the shape of a bar (`▄`)
    Bar,
    /// Use the [Unicode Braille Patterns](https://en.wikipedia.org/wiki/Braille_Patterns) block to
    /// represent data points.
    ///
    /// This is a 2x4 grid of dots, where each dot can be either on or off.
    ///
    /// Note: Support for this marker is limited to terminals and fonts that support Unicode
    /// Braille Patterns. If your terminal does not support this, you will see unicode replacement
    /// characters (`�`) instead of Braille dots (`⠓`, `⣇`, `⣿`).
    Braille,
    /// Use the unicode block and half block characters (`█`, `▄`, and `▀`) to represent points in
    /// a grid that is double the resolution of the terminal. Because each terminal cell is
    /// generally about twice as tall as it is wide, this allows for a square grid of pixels.
    HalfBlock,
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use strum::ParseError;

    use super::*;

    #[test]
    fn marker_tostring() {
        assert_eq!(Marker::Dot.to_string(), "Dot");
        assert_eq!(Marker::Block.to_string(), "Block");
        assert_eq!(Marker::Bar.to_string(), "Bar");
        assert_eq!(Marker::Braille.to_string(), "Braille");
    }

    #[test]
    fn marker_from_str() {
        assert_eq!("Dot".parse::<Marker>(), Ok(Marker::Dot));
        assert_eq!("Block".parse::<Marker>(), Ok(Marker::Block));
        assert_eq!("Bar".parse::<Marker>(), Ok(Marker::Bar));
        assert_eq!("Braille".parse::<Marker>(), Ok(Marker::Braille));
        assert_eq!("".parse::<Marker>(), Err(ParseError::VariantNotFound));
    }
}
