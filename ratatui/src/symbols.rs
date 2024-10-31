use strum::{Display, EnumString};

pub mod border;
pub mod line;

pub mod block {
    pub const FULL: &str = "█";
    pub const SEVEN_EIGHTHS: &str = "▉";
    pub const THREE_QUARTERS: &str = "▊";
    pub const FIVE_EIGHTHS: &str = "▋";
    pub const HALF: &str = "▌";
    pub const THREE_EIGHTHS: &str = "▍";
    pub const ONE_QUARTER: &str = "▎";
    pub const ONE_EIGHTH: &str = "▏";

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub struct Set {
        pub full: &'static str,
        pub seven_eighths: &'static str,
        pub three_quarters: &'static str,
        pub five_eighths: &'static str,
        pub half: &'static str,
        pub three_eighths: &'static str,
        pub one_quarter: &'static str,
        pub one_eighth: &'static str,
        pub empty: &'static str,
    }

    impl Default for Set {
        fn default() -> Self {
            NINE_LEVELS
        }
    }

    pub const THREE_LEVELS: Set = Set {
        full: FULL,
        seven_eighths: FULL,
        three_quarters: HALF,
        five_eighths: HALF,
        half: HALF,
        three_eighths: HALF,
        one_quarter: HALF,
        one_eighth: " ",
        empty: " ",
    };

    pub const NINE_LEVELS: Set = Set {
        full: FULL,
        seven_eighths: SEVEN_EIGHTHS,
        three_quarters: THREE_QUARTERS,
        five_eighths: FIVE_EIGHTHS,
        half: HALF,
        three_eighths: THREE_EIGHTHS,
        one_quarter: ONE_QUARTER,
        one_eighth: ONE_EIGHTH,
        empty: " ",
    };
}

pub mod half_block {
    pub const UPPER: char = '▀';
    pub const LOWER: char = '▄';
    pub const FULL: char = '█';
}

pub mod bar {
    pub const FULL: &str = "█";
    pub const SEVEN_EIGHTHS: &str = "▇";
    pub const THREE_QUARTERS: &str = "▆";
    pub const FIVE_EIGHTHS: &str = "▅";
    pub const HALF: &str = "▄";
    pub const THREE_EIGHTHS: &str = "▃";
    pub const ONE_QUARTER: &str = "▂";
    pub const ONE_EIGHTH: &str = "▁";

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub struct Set {
        pub full: &'static str,
        pub seven_eighths: &'static str,
        pub three_quarters: &'static str,
        pub five_eighths: &'static str,
        pub half: &'static str,
        pub three_eighths: &'static str,
        pub one_quarter: &'static str,
        pub one_eighth: &'static str,
        pub empty: &'static str,
    }

    impl Default for Set {
        fn default() -> Self {
            NINE_LEVELS
        }
    }

    pub const THREE_LEVELS: Set = Set {
        full: FULL,
        seven_eighths: FULL,
        three_quarters: HALF,
        five_eighths: HALF,
        half: HALF,
        three_eighths: HALF,
        one_quarter: HALF,
        one_eighth: " ",
        empty: " ",
    };

    pub const NINE_LEVELS: Set = Set {
        full: FULL,
        seven_eighths: SEVEN_EIGHTHS,
        three_quarters: THREE_QUARTERS,
        five_eighths: FIVE_EIGHTHS,
        half: HALF,
        three_eighths: THREE_EIGHTHS,
        one_quarter: ONE_QUARTER,
        one_eighth: ONE_EIGHTH,
        empty: " ",
    };
}

pub const DOT: &str = "•";

pub mod braille {
    pub const BLANK: u16 = 0x2800;
    pub const DOTS: [[u16; 2]; 4] = [
        [0x0001, 0x0008],
        [0x0002, 0x0010],
        [0x0004, 0x0020],
        [0x0040, 0x0080],
    ];
}

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

pub mod scrollbar {
    use crate::symbols::{block, line};

    /// Scrollbar Set
    /// ```text
    /// <--▮------->
    /// ^  ^   ^   ^
    /// │  │   │   └ end
    /// │  │   └──── track
    /// │  └──────── thumb
    /// └─────────── begin
    /// ```
    #[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
    pub struct Set {
        pub track: &'static str,
        pub thumb: &'static str,
        pub begin: &'static str,
        pub end: &'static str,
    }

    pub const DOUBLE_VERTICAL: Set = Set {
        track: line::DOUBLE_VERTICAL,
        thumb: block::FULL,
        begin: "▲",
        end: "▼",
    };

    pub const DOUBLE_HORIZONTAL: Set = Set {
        track: line::DOUBLE_HORIZONTAL,
        thumb: block::FULL,
        begin: "◄",
        end: "►",
    };

    pub const VERTICAL: Set = Set {
        track: line::VERTICAL,
        thumb: block::FULL,
        begin: "↑",
        end: "↓",
    };

    pub const HORIZONTAL: Set = Set {
        track: line::HORIZONTAL,
        thumb: block::FULL,
        begin: "←",
        end: "→",
    };
}

pub mod shade {
    pub const EMPTY: &str = " ";
    pub const LIGHT: &str = "░";
    pub const MEDIUM: &str = "▒";
    pub const DARK: &str = "▓";
    pub const FULL: &str = "█";
}

#[cfg(test)]
mod tests {
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
