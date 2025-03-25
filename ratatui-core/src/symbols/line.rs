pub const VERTICAL: &str = "│";
pub const DOUBLE_VERTICAL: &str = "║";
pub const THICK_VERTICAL: &str = "┃";
pub const LIGHT_DOUBLE_DASH_VERTICAL: &str = "╎";
pub const HEAVY_DOUBLE_DASH_VERTICAL: &str = "╏";
pub const LIGHT_TRIPLE_DASH_VERTICAL: &str = "┆";
pub const HEAVY_TRIPLE_DASH_VERTICAL: &str = "┇";
pub const LIGHT_QUADRUPLE_DASH_VERTICAL: &str = "┊";
pub const HEAVY_QUADRUPLE_DASH_VERTICAL: &str = "┋";

pub const HORIZONTAL: &str = "─";
pub const DOUBLE_HORIZONTAL: &str = "═";
pub const THICK_HORIZONTAL: &str = "━";
pub const LIGHT_DOUBLE_DASH_HORIZONTAL: &str = "╌";
pub const HEAVY_DOUBLE_DASH_HORIZONTAL: &str = "╍";
pub const LIGHT_TRIPLE_DASH_HORIZONTAL: &str = "┄";
pub const HEAVY_TRIPLE_DASH_HORIZONTAL: &str = "┅";
pub const LIGHT_QUADRUPLE_DASH_HORIZONTAL: &str = "┈";
pub const HEAVY_QUADRUPLE_DASH_HORIZONTAL: &str = "┉";

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

pub enum LineStyle {
    Nothing,
    Plain,
    Rounded,
    Double,
    Thick,
    DoubleDash,
    TripleDash,
    TripleDashThick,
    QuadrupleDash,
    QuadrupleDashThick,
}

pub struct BorderSymbol {
    pub right: LineStyle,
    pub up: LineStyle,
    pub left: LineStyle,
    pub down: LineStyle,
}

macro_rules! define_symbols {
    (
        $( $symbol:expr => ($right:ident, $up:ident, $left:ident, $down:ident) ),* $(,)?
    ) => {

        impl TryFrom<&str> for BorderSymbol {
            type Error = ();
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                use LineStyle::*;
                match value {
                    $( $symbol => Ok(Self::new($right, $up, $left, $down)) ),* ,
                    _ => Err(()),
                }
            }
        }

        impl TryInto<&'static str> for BorderSymbol {
            type Error = ();
            fn try_into(self) -> Result<&'static str, Self::Error> {
                use LineStyle::*;
                match (self.right, self.up, self.left, self.down) {
                    $( ($right, $up, $left, $down) => Ok($symbol) ),* ,
                    _ => Err(()),
                }
            }
        }
    };
}

define_symbols!(
    "─" => (Plain, Nothing, Plain, Nothing),
    "━" => (Thick, Nothing, Thick, Nothing),
    "│" => (Nothing, Plain, Nothing, Plain),
    "┃" => (Nothing, Thick, Nothing, Thick),

    "┄" => (TripleDash, Nothing, TripleDash, Nothing),
    "┅" => (TripleDashThick, Nothing, TripleDashThick, Nothing),
    "┆" => (Nothing, TripleDash, Nothing, TripleDash),
    "┇" => (Nothing, TripleDashThick, Nothing, TripleDashThick),
    "┈" => (QuadrupleDash, Nothing, QuadrupleDash, Nothing),
    "┉" => (QuadrupleDashThick, Nothing, QuadrupleDashThick, Nothing),
    "┊" => (Nothing, QuadrupleDash, Nothing, QuadrupleDash),
    "┋" => (Nothing, QuadrupleDashThick, Nothing, QuadrupleDashThick),

    "┘" => (Nothing, Plain, Plain, Nothing),
    "┚" => (Nothing, Thick, Plain, Nothing),
    "┙" => (Nothing, Plain, Thick, Nothing),
    "┛" => (Nothing, Thick, Thick, Nothing),

    "┐" => (Nothing, Nothing, Plain, Plain),
    "┒" => (Nothing, Nothing, Plain, Thick),
    "┑" => (Nothing, Nothing, Thick, Plain),
    "┓" => (Nothing, Nothing, Thick, Thick),

    "┍" => (Thick, Nothing, Nothing, Plain),
    "┌" => (Plain, Nothing, Nothing, Plain),
    "┎" => (Plain, Nothing, Nothing, Thick),
    "┏" => (Thick, Nothing, Nothing, Thick),

    "└" => (Plain, Plain, Nothing, Nothing),
    "┕" => (Thick, Plain, Nothing, Nothing),
    "┖" => (Plain, Thick, Nothing, Nothing),
    "┗" => (Thick, Thick, Nothing, Nothing),

    "┸" => (Plain, Thick, Plain, Nothing),
    "┴" => (Plain, Plain, Plain, Nothing),
    "┹" => (Plain, Thick, Thick, Nothing),
    "┺" => (Thick, Thick, Plain, Nothing),
    "┵" => (Plain, Plain, Thick, Nothing),
    "┶" => (Thick, Plain, Plain, Nothing),
    "┷" => (Thick, Plain, Thick, Nothing),

    "┮" => (Thick, Nothing, Plain, Plain),
    "┬" => (Plain, Nothing, Plain, Plain),
    "┰" => (Plain, Nothing, Plain, Thick),
    "┭" => (Plain, Nothing, Thick, Plain),
    "┱" => (Plain, Nothing, Thick, Thick),
    "┲" => (Thick, Nothing, Plain, Thick),
    "┯" => (Thick, Nothing, Thick, Plain),
    "┳" => (Thick, Nothing, Thick, Thick),

    "┩" => (Nothing, Thick, Thick, Plain),
    "┦" => (Nothing, Thick, Plain, Plain),
    "┥" => (Nothing, Plain, Thick, Plain),
    "┪" => (Nothing, Plain, Thick, Thick),
    "┨" => (Nothing, Thick, Plain, Thick),
    "┤" => (Nothing, Plain, Plain, Plain),
    "┧" => (Nothing, Plain, Plain, Thick),
    "┫" => (Nothing, Thick, Thick, Thick),

    "┞" => (Plain, Thick, Nothing, Plain),
    "┠" => (Plain, Thick, Nothing, Thick),
    "┝" => (Thick, Plain, Nothing, Plain),
    "┢" => (Thick, Plain, Nothing, Thick),
    "├" => (Plain, Plain, Nothing, Plain),
    "┟" => (Plain, Plain, Nothing, Thick),
    "┡" => (Thick, Thick, Nothing, Plain),
    "┣" => (Thick, Thick, Nothing, Thick),

    "┽" => (Plain, Plain, Thick, Plain),
    "┼" => ( Plain, Plain, Plain, Plain),
    "╁" => ( Plain, Plain, Plain, Thick),
    "╅" => (Plain, Plain, Thick, Thick),
    "╃" => (Plain, Thick, Thick, Plain),
    "╀" => (Plain, Thick, Plain, Plain),
    "╂" => (Plain, Thick, Plain, Thick),
    "╉" => (Plain, Thick, Thick, Thick),
    "┾" => (Thick, Plain, Plain, Plain),
    "╆" => (Thick, Plain, Plain, Thick),
    "┿" => (Thick, Plain, Thick, Plain),
    "╈" => (Thick, Plain, Thick, Thick),
    "╄" => (Thick, Thick, Plain, Plain),
    "╊" => (Thick, Thick, Plain, Thick),
    "┻" => (Thick, Thick, Thick, Nothing),
    "╇" => (Thick, Thick, Thick, Plain),
    "╋" => (Thick, Thick, Thick, Thick),

    "═" => (Double, Nothing, Double, Nothing),
    "║" => (Nothing, Double, Nothing, Double),

    "╓" => (Plain, Nothing, Nothing, Double),
    "╒" => (Double, Nothing, Nothing, Plain),
    "╔" => (Double, Nothing, Nothing, Double),

    "╕" => (Nothing, Nothing, Double, Plain),
    "╖" => (Nothing, Nothing, Plain, Double),
    "╗" => (Nothing, Nothing, Double, Double),

    "╘" => (Double, Plain, Nothing, Nothing),
    "╙" => (Plain, Double, Nothing, Nothing),
    "╚" => (Double, Double, Nothing, Nothing),

    "╛" => (Nothing, Plain, Double, Nothing),
    "╜" => (Nothing, Double, Plain, Nothing),
    "╝" => (Nothing, Double, Double, Nothing),

    "╞" => (Double, Plain, Nothing, Plain),
    "╟" => (Plain, Double, Nothing, Double),
    "╠" => (Double, Double, Nothing, Double),

    "╡" => (Nothing, Plain, Double, Plain),
    "╢" => (Nothing, Double, Plain, Double),
    "╣" => (Nothing, Double, Double, Double),

    "╤" => (Double, Nothing, Double, Plain),
    "╥" => (Plain, Nothing, Plain, Double),
    "╦" => (Double, Nothing, Double, Double),

    "╧" => (Double, Plain, Double, Nothing),
    "╨" => (Plain, Double, Plain, Nothing),
    "╩" => (Double, Double, Double, Nothing),

    "╪" => (Double, Plain, Double, Plain),
    "╫" => (Plain, Double, Plain, Double),
    "╬" => (Double, Double, Double, Double),

    "╭" => (Rounded, Nothing, Nothing, Rounded),
    "╮" => (Nothing, Nothing, Rounded, Rounded),
    "╯" => (Nothing, Rounded, Rounded, Nothing),
    "╰" => (Rounded, Rounded, Nothing, Nothing),

    "╴" => (Nothing, Nothing, Plain, Nothing),
    "╵" => (Nothing, Plain, Nothing, Nothing),
    "╶" => (Plain, Nothing, Nothing, Nothing),
    "╷" => (Nothing, Nothing, Nothing, Plain),
    "╸" => (Nothing, Nothing, Thick, Nothing),
    "╹" => (Nothing, Thick, Nothing, Nothing),
    "╺" => (Thick, Nothing, Nothing, Nothing),
    "╻" => (Nothing, Nothing, Nothing, Thick),
    "╼" => (Thick, Nothing, Plain, Nothing),
    "╽" => (Nothing, Plain, Nothing, Thick),
    "╾" => (Plain, Nothing, Thick, Nothing),
    "╿" => (Nothing, Thick, Nothing, Plain),
);

impl BorderSymbol {
    pub const fn new(right: LineStyle, up: LineStyle, left: LineStyle, down: LineStyle) -> Self {
        Self {
            right,
            up,
            left,
            down,
        }
    }
}

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

pub const LIGHT_DOUBLE_DASHED: Set = Set {
    vertical: LIGHT_DOUBLE_DASH_VERTICAL,
    horizontal: LIGHT_DOUBLE_DASH_HORIZONTAL,
    ..NORMAL
};

pub const HEAVY_DOUBLE_DASHED: Set = Set {
    vertical: HEAVY_DOUBLE_DASH_VERTICAL,
    horizontal: HEAVY_DOUBLE_DASH_HORIZONTAL,
    ..THICK
};

pub const LIGHT_TRIPLE_DASHED: Set = Set {
    vertical: LIGHT_TRIPLE_DASH_VERTICAL,
    horizontal: LIGHT_TRIPLE_DASH_HORIZONTAL,
    ..NORMAL
};

pub const HEAVY_TRIPLE_DASHED: Set = Set {
    vertical: HEAVY_TRIPLE_DASH_VERTICAL,
    horizontal: HEAVY_TRIPLE_DASH_HORIZONTAL,
    ..THICK
};

pub const LIGHT_QUADRUPLE_DASHED: Set = Set {
    vertical: LIGHT_QUADRUPLE_DASH_VERTICAL,
    horizontal: LIGHT_QUADRUPLE_DASH_HORIZONTAL,
    ..NORMAL
};

pub const HEAVY_QUADRUPLE_DASHED: Set = Set {
    vertical: HEAVY_QUADRUPLE_DASH_VERTICAL,
    horizontal: HEAVY_QUADRUPLE_DASH_HORIZONTAL,
    ..THICK
};

#[cfg(test)]
mod tests {
    use indoc::{formatdoc, indoc};

    use super::*;

    #[test]
    fn default() {
        assert_eq!(Set::default(), NORMAL);
    }

    /// A helper function to render a set of symbols.
    fn render(set: Set) -> String {
        formatdoc!(
            "{}{}{}{}
             {}{}{}{}
             {}{}{}{}
             {}{}{}{}",
            set.top_left,
            set.horizontal,
            set.horizontal_down,
            set.top_right,
            set.vertical,
            " ",
            set.vertical,
            set.vertical,
            set.vertical_right,
            set.horizontal,
            set.cross,
            set.vertical_left,
            set.bottom_left,
            set.horizontal,
            set.horizontal_up,
            set.bottom_right
        )
    }

    #[test]
    fn normal() {
        assert_eq!(
            render(NORMAL),
            indoc!(
                "┌─┬┐
                 │ ││
                 ├─┼┤
                 └─┴┘"
            )
        );
    }

    #[test]
    fn rounded() {
        assert_eq!(
            render(ROUNDED),
            indoc!(
                "╭─┬╮
                 │ ││
                 ├─┼┤
                 ╰─┴╯"
            )
        );
    }

    #[test]
    fn double() {
        assert_eq!(
            render(DOUBLE),
            indoc!(
                "╔═╦╗
                 ║ ║║
                 ╠═╬╣
                 ╚═╩╝"
            )
        );
    }

    #[test]
    fn thick() {
        assert_eq!(
            render(THICK),
            indoc!(
                "┏━┳┓
                 ┃ ┃┃
                 ┣━╋┫
                 ┗━┻┛"
            )
        );
    }
}
