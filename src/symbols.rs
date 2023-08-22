use strum::{Display, EnumString};

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

pub mod line {
    pub const VERTICAL: &str = "│";
    pub const DOUBLE_VERTICAL: &str = "║";
    pub const THICK_VERTICAL: &str = "┃";

    pub const HORIZONTAL: &str = "─";
    pub const DOUBLE_HORIZONTAL: &str = "═";
    pub const THICK_HORIZONTAL: &str = "━";

    pub const TOP_RIGHT: &str = "┐";
    pub const ROUNDED_TOP_RIGHT: &str = "╮";
    pub const DOUBLE_TOP_RIGHT: &str = "╗";
    pub const THICK_TOP_RIGHT: &str = "┓";

    pub const TOP_LEFT: &str = "┌";
    pub const ROUNDED_TOP_LEFT: &str = "╭";
    pub const DOUBLE_TOP_LEFT: &str = "╔";
    pub const THICK_TOP_LEFT: &str = "┏";

    pub const BOTTOM_RIGHT: &str = "┘";
    pub const ROUNDED_BOTTOM_RIGHT: &str = "╯";
    pub const DOUBLE_BOTTOM_RIGHT: &str = "╝";
    pub const THICK_BOTTOM_RIGHT: &str = "┛";

    pub const BOTTOM_LEFT: &str = "└";
    pub const ROUNDED_BOTTOM_LEFT: &str = "╰";
    pub const DOUBLE_BOTTOM_LEFT: &str = "╚";
    pub const THICK_BOTTOM_LEFT: &str = "┗";

    pub const VERTICAL_LEFT: &str = "┤";
    pub const DOUBLE_VERTICAL_LEFT: &str = "╣";
    pub const THICK_VERTICAL_LEFT: &str = "┫";

    pub const VERTICAL_RIGHT: &str = "├";
    pub const DOUBLE_VERTICAL_RIGHT: &str = "╠";
    pub const THICK_VERTICAL_RIGHT: &str = "┣";

    pub const HORIZONTAL_DOWN: &str = "┬";
    pub const DOUBLE_HORIZONTAL_DOWN: &str = "╦";
    pub const THICK_HORIZONTAL_DOWN: &str = "┳";

    pub const HORIZONTAL_UP: &str = "┴";
    pub const DOUBLE_HORIZONTAL_UP: &str = "╩";
    pub const THICK_HORIZONTAL_UP: &str = "┻";

    pub const CROSS: &str = "┼";
    pub const DOUBLE_CROSS: &str = "╬";
    pub const THICK_CROSS: &str = "╋";

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub struct Set {
        pub vertical: &'static str,
        pub horizontal: &'static str,
        pub top_right: &'static str,
        pub top_left: &'static str,
        pub bottom_right: &'static str,
        pub bottom_left: &'static str,
        pub vertical_left: &'static str,
        pub vertical_right: &'static str,
        pub horizontal_down: &'static str,
        pub horizontal_up: &'static str,
        pub cross: &'static str,
    }

    impl Default for Set {
        fn default() -> Self {
            NORMAL
        }
    }

    pub const NORMAL: Set = Set {
        vertical: VERTICAL,
        horizontal: HORIZONTAL,
        top_right: TOP_RIGHT,
        top_left: TOP_LEFT,
        bottom_right: BOTTOM_RIGHT,
        bottom_left: BOTTOM_LEFT,
        vertical_left: VERTICAL_LEFT,
        vertical_right: VERTICAL_RIGHT,
        horizontal_down: HORIZONTAL_DOWN,
        horizontal_up: HORIZONTAL_UP,
        cross: CROSS,
    };

    pub const ROUNDED: Set = Set {
        top_right: ROUNDED_TOP_RIGHT,
        top_left: ROUNDED_TOP_LEFT,
        bottom_right: ROUNDED_BOTTOM_RIGHT,
        bottom_left: ROUNDED_BOTTOM_LEFT,
        ..NORMAL
    };

    pub const DOUBLE: Set = Set {
        vertical: DOUBLE_VERTICAL,
        horizontal: DOUBLE_HORIZONTAL,
        top_right: DOUBLE_TOP_RIGHT,
        top_left: DOUBLE_TOP_LEFT,
        bottom_right: DOUBLE_BOTTOM_RIGHT,
        bottom_left: DOUBLE_BOTTOM_LEFT,
        vertical_left: DOUBLE_VERTICAL_LEFT,
        vertical_right: DOUBLE_VERTICAL_RIGHT,
        horizontal_down: DOUBLE_HORIZONTAL_DOWN,
        horizontal_up: DOUBLE_HORIZONTAL_UP,
        cross: DOUBLE_CROSS,
    };

    pub const THICK: Set = Set {
        vertical: THICK_VERTICAL,
        horizontal: THICK_HORIZONTAL,
        top_right: THICK_TOP_RIGHT,
        top_left: THICK_TOP_LEFT,
        bottom_right: THICK_BOTTOM_RIGHT,
        bottom_left: THICK_BOTTOM_LEFT,
        vertical_left: THICK_VERTICAL_LEFT,
        vertical_right: THICK_VERTICAL_RIGHT,
        horizontal_down: THICK_HORIZONTAL_DOWN,
        horizontal_up: THICK_HORIZONTAL_UP,
        cross: THICK_CROSS,
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
    /// One point per cell in shape of dot
    #[default]
    Dot,
    /// One point per cell in shape of a block
    Block,
    /// One point per cell in the shape of a bar
    Bar,
    /// Up to 8 points per cell
    Braille,
}

pub mod scrollbar {
    use super::{block, line};

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
