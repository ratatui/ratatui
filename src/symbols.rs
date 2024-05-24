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

    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

pub mod border {
    use super::line;

    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    pub struct Set {
        pub top_left: &'static str,
        pub top_right: &'static str,
        pub bottom_left: &'static str,
        pub bottom_right: &'static str,
        pub vertical_left: &'static str,
        pub vertical_right: &'static str,
        pub horizontal_top: &'static str,
        pub horizontal_bottom: &'static str,
    }

    impl Default for Set {
        fn default() -> Self {
            PLAIN
        }
    }

    /// Border Set with a single line width
    ///
    /// ```text
    /// ┌─────┐
    /// │xxxxx│
    /// │xxxxx│
    /// └─────┘
    pub const PLAIN: Set = Set {
        top_left: line::NORMAL.top_left,
        top_right: line::NORMAL.top_right,
        bottom_left: line::NORMAL.bottom_left,
        bottom_right: line::NORMAL.bottom_right,
        vertical_left: line::NORMAL.vertical,
        vertical_right: line::NORMAL.vertical,
        horizontal_top: line::NORMAL.horizontal,
        horizontal_bottom: line::NORMAL.horizontal,
    };

    /// Border Set with a single line width and rounded corners
    ///
    /// ```text
    /// ╭─────╮
    /// │xxxxx│
    /// │xxxxx│
    /// ╰─────╯
    pub const ROUNDED: Set = Set {
        top_left: line::ROUNDED.top_left,
        top_right: line::ROUNDED.top_right,
        bottom_left: line::ROUNDED.bottom_left,
        bottom_right: line::ROUNDED.bottom_right,
        vertical_left: line::ROUNDED.vertical,
        vertical_right: line::ROUNDED.vertical,
        horizontal_top: line::ROUNDED.horizontal,
        horizontal_bottom: line::ROUNDED.horizontal,
    };

    /// Border Set with a double line width
    ///
    /// ```text
    /// ╔═════╗
    /// ║xxxxx║
    /// ║xxxxx║
    /// ╚═════╝
    pub const DOUBLE: Set = Set {
        top_left: line::DOUBLE.top_left,
        top_right: line::DOUBLE.top_right,
        bottom_left: line::DOUBLE.bottom_left,
        bottom_right: line::DOUBLE.bottom_right,
        vertical_left: line::DOUBLE.vertical,
        vertical_right: line::DOUBLE.vertical,
        horizontal_top: line::DOUBLE.horizontal,
        horizontal_bottom: line::DOUBLE.horizontal,
    };

    /// Border Set with a thick line width
    ///
    /// ```text
    /// ┏━━━━━┓
    /// ┃xxxxx┃
    /// ┃xxxxx┃
    /// ┗━━━━━┛
    pub const THICK: Set = Set {
        top_left: line::THICK.top_left,
        top_right: line::THICK.top_right,
        bottom_left: line::THICK.bottom_left,
        bottom_right: line::THICK.bottom_right,
        vertical_left: line::THICK.vertical,
        vertical_right: line::THICK.vertical,
        horizontal_top: line::THICK.horizontal,
        horizontal_bottom: line::THICK.horizontal,
    };

    pub const QUADRANT_TOP_LEFT: &str = "▘";
    pub const QUADRANT_TOP_RIGHT: &str = "▝";
    pub const QUADRANT_BOTTOM_LEFT: &str = "▖";
    pub const QUADRANT_BOTTOM_RIGHT: &str = "▗";
    pub const QUADRANT_TOP_HALF: &str = "▀";
    pub const QUADRANT_BOTTOM_HALF: &str = "▄";
    pub const QUADRANT_LEFT_HALF: &str = "▌";
    pub const QUADRANT_RIGHT_HALF: &str = "▐";
    pub const QUADRANT_TOP_LEFT_BOTTOM_LEFT_BOTTOM_RIGHT: &str = "▙";
    pub const QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT: &str = "▛";
    pub const QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_RIGHT: &str = "▜";
    pub const QUADRANT_TOP_RIGHT_BOTTOM_LEFT_BOTTOM_RIGHT: &str = "▟";
    pub const QUADRANT_TOP_LEFT_BOTTOM_RIGHT: &str = "▚";
    pub const QUADRANT_TOP_RIGHT_BOTTOM_LEFT: &str = "▞";
    pub const QUADRANT_BLOCK: &str = "█";

    /// Quadrant used for setting a border outside a block by one half cell "pixel".
    ///
    /// ```text
    /// ▛▀▀▀▀▀▜
    /// ▌xxxxx▐
    /// ▌xxxxx▐
    /// ▙▄▄▄▄▄▟
    /// ```
    pub const QUADRANT_OUTSIDE: Set = Set {
        top_left: QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT,
        top_right: QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_RIGHT,
        bottom_left: QUADRANT_TOP_LEFT_BOTTOM_LEFT_BOTTOM_RIGHT,
        bottom_right: QUADRANT_TOP_RIGHT_BOTTOM_LEFT_BOTTOM_RIGHT,
        vertical_left: QUADRANT_LEFT_HALF,
        vertical_right: QUADRANT_RIGHT_HALF,
        horizontal_top: QUADRANT_TOP_HALF,
        horizontal_bottom: QUADRANT_BOTTOM_HALF,
    };

    /// Quadrant used for setting a border inside a block by one half cell "pixel".
    ///
    /// ```text
    /// ▗▄▄▄▄▄▖
    /// ▐xxxxx▌
    /// ▐xxxxx▌
    /// ▝▀▀▀▀▀▘
    /// ```
    pub const QUADRANT_INSIDE: Set = Set {
        top_right: QUADRANT_BOTTOM_LEFT,
        top_left: QUADRANT_BOTTOM_RIGHT,
        bottom_right: QUADRANT_TOP_LEFT,
        bottom_left: QUADRANT_TOP_RIGHT,
        vertical_left: QUADRANT_RIGHT_HALF,
        vertical_right: QUADRANT_LEFT_HALF,
        horizontal_top: QUADRANT_BOTTOM_HALF,
        horizontal_bottom: QUADRANT_TOP_HALF,
    };

    pub const ONE_EIGHTH_TOP_EIGHT: &str = "▔";
    pub const ONE_EIGHTH_BOTTOM_EIGHT: &str = "▁";
    pub const ONE_EIGHTH_LEFT_EIGHT: &str = "▏";
    pub const ONE_EIGHTH_RIGHT_EIGHT: &str = "▕";

    /// Wide border set based on McGugan box technique
    ///
    /// ```text
    /// ▁▁▁▁▁▁▁
    /// ▏xxxxx▕
    /// ▏xxxxx▕
    /// ▔▔▔▔▔▔▔
    /// ```
    #[allow(clippy::doc_markdown)]
    pub const ONE_EIGHTH_WIDE: Set = Set {
        top_right: ONE_EIGHTH_BOTTOM_EIGHT,
        top_left: ONE_EIGHTH_BOTTOM_EIGHT,
        bottom_right: ONE_EIGHTH_TOP_EIGHT,
        bottom_left: ONE_EIGHTH_TOP_EIGHT,
        vertical_left: ONE_EIGHTH_LEFT_EIGHT,
        vertical_right: ONE_EIGHTH_RIGHT_EIGHT,
        horizontal_top: ONE_EIGHTH_BOTTOM_EIGHT,
        horizontal_bottom: ONE_EIGHTH_TOP_EIGHT,
    };

    /// Tall border set based on McGugan box technique
    ///
    /// ```text
    /// ▕▔▔▏
    /// ▕xx▏
    /// ▕xx▏
    /// ▕▁▁▏
    /// ```
    #[allow(clippy::doc_markdown)]
    pub const ONE_EIGHTH_TALL: Set = Set {
        top_right: ONE_EIGHTH_LEFT_EIGHT,
        top_left: ONE_EIGHTH_RIGHT_EIGHT,
        bottom_right: ONE_EIGHTH_LEFT_EIGHT,
        bottom_left: ONE_EIGHTH_RIGHT_EIGHT,
        vertical_left: ONE_EIGHTH_RIGHT_EIGHT,
        vertical_right: ONE_EIGHTH_LEFT_EIGHT,
        horizontal_top: ONE_EIGHTH_TOP_EIGHT,
        horizontal_bottom: ONE_EIGHTH_BOTTOM_EIGHT,
    };

    /// Wide proportional (visually equal width and height) border with using set of quadrants.
    ///
    /// The border is created by using half blocks for top and bottom, and full
    /// blocks for right and left sides to make horizontal and vertical borders seem equal.
    ///
    /// ```text
    /// ▄▄▄▄
    /// █xx█
    /// █xx█
    /// ▀▀▀▀
    /// ```
    pub const PROPORTIONAL_WIDE: Set = Set {
        top_right: QUADRANT_BOTTOM_HALF,
        top_left: QUADRANT_BOTTOM_HALF,
        bottom_right: QUADRANT_TOP_HALF,
        bottom_left: QUADRANT_TOP_HALF,
        vertical_left: QUADRANT_BLOCK,
        vertical_right: QUADRANT_BLOCK,
        horizontal_top: QUADRANT_BOTTOM_HALF,
        horizontal_bottom: QUADRANT_TOP_HALF,
    };

    /// Tall proportional (visually equal width and height) border with using set of quadrants.
    ///
    /// The border is created by using full blocks for all sides, except for the top and bottom,
    /// which use half blocks to make horizontal and vertical borders seem equal.
    ///
    /// ```text
    /// ▕█▀▀█
    /// ▕█xx█
    /// ▕█xx█
    /// ▕█▄▄█
    /// ```
    pub const PROPORTIONAL_TALL: Set = Set {
        top_right: QUADRANT_BLOCK,
        top_left: QUADRANT_BLOCK,
        bottom_right: QUADRANT_BLOCK,
        bottom_left: QUADRANT_BLOCK,
        vertical_left: QUADRANT_BLOCK,
        vertical_right: QUADRANT_BLOCK,
        horizontal_top: QUADRANT_TOP_HALF,
        horizontal_bottom: QUADRANT_BOTTOM_HALF,
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
