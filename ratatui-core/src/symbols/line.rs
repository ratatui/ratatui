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
    Normal,
    Thick,
    Double,
    DoubleDash,
    TripleDash,
    QuadrupleDash,
}

pub struct BorderSymbol {
    pub right: Option<LineStyle>,
    pub up: Option<LineStyle>,
    pub left: Option<LineStyle>,
    pub down: Option<LineStyle>,
}

impl BorderSymbol {
    pub const fn new(
        right: Option<LineStyle>,
        up: Option<LineStyle>,
        left: Option<LineStyle>,
        down: Option<LineStyle>,
    ) -> Self {
        Self {
            right,
            up,
            left,
            down,
        }
    }
}

impl TryFrom<&str> for BorderSymbol {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use LineStyle::{Double, Normal, Thick};
        match value {
            "╷" => Ok(Self::new(None, None, None, Some(Normal))),
            "╻" => Ok(Self::new(None, None, None, Some(Thick))),
            "╴" => Ok(Self::new(None, None, Some(Normal), None)),
            "┐" => Ok(Self::new(None, None, Some(Normal), Some(Normal))),
            "┒" => Ok(Self::new(None, None, Some(Normal), Some(Thick))),
            "╖" => Ok(Self::new(None, None, Some(Normal), Some(Double))),
            "╸" => Ok(Self::new(None, None, Some(Thick), None)),
            "┑" => Ok(Self::new(None, None, Some(Thick), Some(Normal))),
            "┓" => Ok(Self::new(None, None, Some(Thick), Some(Thick))),
            "╕" => Ok(Self::new(None, None, Some(Double), Some(Normal))),
            "╗" => Ok(Self::new(None, None, Some(Double), Some(Double))),
            "╵" => Ok(Self::new(None, Some(Normal), None, None)),
            "│" => Ok(Self::new(None, Some(Normal), None, Some(Normal))),
            "╽" => Ok(Self::new(None, Some(Normal), None, Some(Thick))),
            "┘" => Ok(Self::new(None, Some(Normal), Some(Normal), None)),
            "┤" => Ok(Self::new(None, Some(Normal), Some(Normal), Some(Normal))),
            "┧" => Ok(Self::new(None, Some(Normal), Some(Normal), Some(Thick))),
            "┙" => Ok(Self::new(None, Some(Normal), Some(Thick), None)),
            "┥" => Ok(Self::new(None, Some(Normal), Some(Thick), Some(Normal))),
            "┪" => Ok(Self::new(None, Some(Normal), Some(Thick), Some(Thick))),
            "╛" => Ok(Self::new(None, Some(Normal), Some(Double), None)),
            "╡" => Ok(Self::new(None, Some(Normal), Some(Double), Some(Normal))),
            "╹" => Ok(Self::new(None, Some(Thick), None, None)),
            "╿" => Ok(Self::new(None, Some(Thick), None, Some(Normal))),
            "┃" => Ok(Self::new(None, Some(Thick), None, Some(Thick))),
            "┚" => Ok(Self::new(None, Some(Thick), Some(Normal), None)),
            "┦" => Ok(Self::new(None, Some(Thick), Some(Normal), Some(Normal))),
            "┨" => Ok(Self::new(None, Some(Thick), Some(Normal), Some(Thick))),
            "┛" => Ok(Self::new(None, Some(Thick), Some(Thick), None)),
            "┩" => Ok(Self::new(None, Some(Thick), Some(Thick), Some(Normal))),
            "┫" => Ok(Self::new(None, Some(Thick), Some(Thick), Some(Thick))),
            "║" => Ok(Self::new(None, Some(Double), None, Some(Double))),
            "╜" => Ok(Self::new(None, Some(Double), Some(Normal), None)),
            "╢" => Ok(Self::new(None, Some(Double), Some(Normal), Some(Double))),
            "╝" => Ok(Self::new(None, Some(Double), Some(Double), None)),
            "╣" => Ok(Self::new(None, Some(Double), Some(Double), Some(Double))),
            "╶" => Ok(Self::new(Some(Normal), None, None, None)),
            "┌" => Ok(Self::new(Some(Normal), None, None, Some(Normal))),
            "┎" => Ok(Self::new(Some(Normal), None, None, Some(Thick))),
            "╓" => Ok(Self::new(Some(Normal), None, None, Some(Double))),
            "─" => Ok(Self::new(Some(Normal), None, Some(Normal), None)),
            "┬" => Ok(Self::new(Some(Normal), None, Some(Normal), Some(Normal))),
            "┰" => Ok(Self::new(Some(Normal), None, Some(Normal), Some(Thick))),
            "╥" => Ok(Self::new(Some(Normal), None, Some(Normal), Some(Double))),
            "╾" => Ok(Self::new(Some(Normal), None, Some(Thick), None)),
            "┭" => Ok(Self::new(Some(Normal), None, Some(Thick), Some(Normal))),
            "┱" => Ok(Self::new(Some(Normal), None, Some(Thick), Some(Thick))),
            "└" => Ok(Self::new(Some(Normal), Some(Normal), None, None)),
            "├" => Ok(Self::new(Some(Normal), Some(Normal), None, Some(Normal))),
            "┟" => Ok(Self::new(Some(Normal), Some(Normal), None, Some(Thick))),
            "┴" => Ok(Self::new(Some(Normal), Some(Normal), Some(Normal), None)),
            "┼" => Ok(Self::new(
                Some(Normal),
                Some(Normal),
                Some(Normal),
                Some(Normal),
            )),
            "╁" => Ok(Self::new(
                Some(Normal),
                Some(Normal),
                Some(Normal),
                Some(Thick),
            )),
            "┵" => Ok(Self::new(Some(Normal), Some(Normal), Some(Thick), None)),
            "┽" => Ok(Self::new(
                Some(Normal),
                Some(Normal),
                Some(Thick),
                Some(Normal),
            )),
            "╅" => Ok(Self::new(
                Some(Normal),
                Some(Normal),
                Some(Thick),
                Some(Thick),
            )),
            "┖" => Ok(Self::new(Some(Normal), Some(Thick), None, None)),
            "┞" => Ok(Self::new(Some(Normal), Some(Thick), None, Some(Normal))),
            "┠" => Ok(Self::new(Some(Normal), Some(Thick), None, Some(Thick))),
            "┸" => Ok(Self::new(Some(Normal), Some(Thick), Some(Normal), None)),
            "╀" => Ok(Self::new(
                Some(Normal),
                Some(Thick),
                Some(Normal),
                Some(Normal),
            )),
            "╂" => Ok(Self::new(
                Some(Normal),
                Some(Thick),
                Some(Normal),
                Some(Thick),
            )),
            "┹" => Ok(Self::new(Some(Normal), Some(Thick), Some(Thick), None)),
            "╃" => Ok(Self::new(
                Some(Normal),
                Some(Thick),
                Some(Thick),
                Some(Normal),
            )),
            "╉" => Ok(Self::new(
                Some(Normal),
                Some(Thick),
                Some(Thick),
                Some(Thick),
            )),
            "╙" => Ok(Self::new(Some(Normal), Some(Double), None, None)),
            "╟" => Ok(Self::new(Some(Normal), Some(Double), None, Some(Double))),
            "╨" => Ok(Self::new(Some(Normal), Some(Double), Some(Normal), None)),
            "╫" => Ok(Self::new(
                Some(Normal),
                Some(Double),
                Some(Normal),
                Some(Double),
            )),
            "╺" => Ok(Self::new(Some(Thick), None, None, None)),
            "┍" => Ok(Self::new(Some(Thick), None, None, Some(Normal))),
            "┏" => Ok(Self::new(Some(Thick), None, None, Some(Thick))),
            "╼" => Ok(Self::new(Some(Thick), None, Some(Normal), None)),
            "┮" => Ok(Self::new(Some(Thick), None, Some(Normal), Some(Normal))),
            "┲" => Ok(Self::new(Some(Thick), None, Some(Normal), Some(Thick))),
            "━" => Ok(Self::new(Some(Thick), None, Some(Thick), None)),
            "┯" => Ok(Self::new(Some(Thick), None, Some(Thick), Some(Normal))),
            "┳" => Ok(Self::new(Some(Thick), None, Some(Thick), Some(Thick))),
            "┕" => Ok(Self::new(Some(Thick), Some(Normal), None, None)),
            "┝" => Ok(Self::new(Some(Thick), Some(Normal), None, Some(Normal))),
            "┢" => Ok(Self::new(Some(Thick), Some(Normal), None, Some(Thick))),
            "┶" => Ok(Self::new(Some(Thick), Some(Normal), Some(Normal), None)),
            "┾" => Ok(Self::new(
                Some(Thick),
                Some(Normal),
                Some(Normal),
                Some(Normal),
            )),
            "╆" => Ok(Self::new(
                Some(Thick),
                Some(Normal),
                Some(Normal),
                Some(Thick),
            )),
            "┷" => Ok(Self::new(Some(Thick), Some(Normal), Some(Thick), None)),
            "┿" => Ok(Self::new(
                Some(Thick),
                Some(Normal),
                Some(Thick),
                Some(Normal),
            )),
            "╈" => Ok(Self::new(
                Some(Thick),
                Some(Normal),
                Some(Thick),
                Some(Thick),
            )),
            "┗" => Ok(Self::new(Some(Thick), Some(Thick), None, None)),
            "┡" => Ok(Self::new(Some(Thick), Some(Thick), None, Some(Normal))),
            "┣" => Ok(Self::new(Some(Thick), Some(Thick), None, Some(Thick))),
            "┺" => Ok(Self::new(Some(Thick), Some(Thick), Some(Normal), None)),
            "╄" => Ok(Self::new(
                Some(Thick),
                Some(Thick),
                Some(Normal),
                Some(Normal),
            )),
            "╊" => Ok(Self::new(
                Some(Thick),
                Some(Thick),
                Some(Normal),
                Some(Thick),
            )),
            "┻" => Ok(Self::new(Some(Thick), Some(Thick), Some(Thick), None)),
            "╇" => Ok(Self::new(
                Some(Thick),
                Some(Thick),
                Some(Thick),
                Some(Normal),
            )),
            "╋" => Ok(Self::new(
                Some(Thick),
                Some(Thick),
                Some(Thick),
                Some(Thick),
            )),
            "╒" => Ok(Self::new(Some(Double), None, None, Some(Normal))),
            "╔" => Ok(Self::new(Some(Double), None, None, Some(Double))),
            "═" => Ok(Self::new(Some(Double), None, Some(Double), None)),
            "╤" => Ok(Self::new(Some(Double), None, Some(Double), Some(Normal))),
            "╦" => Ok(Self::new(Some(Double), None, Some(Double), Some(Double))),
            "╘" => Ok(Self::new(Some(Double), Some(Normal), None, None)),
            "╞" => Ok(Self::new(Some(Double), Some(Normal), None, Some(Normal))),
            "╧" => Ok(Self::new(Some(Double), Some(Normal), Some(Double), None)),
            "╪" => Ok(Self::new(
                Some(Double),
                Some(Normal),
                Some(Double),
                Some(Normal),
            )),
            "╚" => Ok(Self::new(Some(Double), Some(Double), None, None)),
            "╠" => Ok(Self::new(Some(Double), Some(Double), None, Some(Double))),
            "╩" => Ok(Self::new(Some(Double), Some(Double), Some(Double), None)),
            "╬" => Ok(Self::new(
                Some(Double),
                Some(Double),
                Some(Double),
                Some(Double),
            )),
            _ => Err(()),
        }
    }
}

impl TryInto<&str> for BorderSymbol {
    type Error = ();
    fn try_into(self) -> Result<&'static str, Self::Error> {
        use LineStyle::{Double, Normal, Thick};
        match (self.right, self.up, self.left, self.down) {
            // (None, None, None, None) => Ok(""),
            (None, None, None, Some(Normal)) => Ok("╷"),
            (None, None, None, Some(Thick)) => Ok("╻"),
            // (None, None, None, Some(Double)) => Ok(""),
            (None, None, Some(Normal), None) => Ok("╴"),
            (None, None, Some(Normal), Some(Normal)) => Ok("┐"),
            (None, None, Some(Normal), Some(Thick)) => Ok("┒"),
            (None, None, Some(Normal), Some(Double)) => Ok("╖"),
            (None, None, Some(Thick), None) => Ok("╸"),
            (None, None, Some(Thick), Some(Normal)) => Ok("┑"),
            (None, None, Some(Thick), Some(Thick)) => Ok("┓"),
            // (None, None, Some(Thick), Some(Double)) => Ok(""),
            // (None, None, Some(Double), None) => Ok(""),
            (None, None, Some(Double), Some(Normal)) => Ok("╕"),
            // (None, None, Some(Double), Some(Thick)) => Ok(""),
            (None, None, Some(Double), Some(Double)) => Ok("╗"),
            (None, Some(Normal), None, None) => Ok("╵"),
            (None, Some(Normal), None, Some(Normal)) => Ok("│"),
            (None, Some(Normal), None, Some(Thick)) => Ok("╽"),
            // (None, Some(Normal), None, Some(Double)) => Ok(""),
            (None, Some(Normal), Some(Normal), None) => Ok("┘"),
            (None, Some(Normal), Some(Normal), Some(Normal)) => Ok("┤"),
            (None, Some(Normal), Some(Normal), Some(Thick)) => Ok("┧"),
            // (None, Some(Normal), Some(Normal), Some(Double)) => Ok(""),
            (None, Some(Normal), Some(Thick), None) => Ok("┙"),
            (None, Some(Normal), Some(Thick), Some(Normal)) => Ok("┥"),
            (None, Some(Normal), Some(Thick), Some(Thick)) => Ok("┪"),
            // (None, Some(Normal), Some(Thick), Some(Double)) => Ok(""),
            (None, Some(Normal), Some(Double), None) => Ok("╛"),
            (None, Some(Normal), Some(Double), Some(Normal)) => Ok("╡"),
            // (None, Some(Normal), Some(Double), Some(Thick)) => Ok(""),
            // (None, Some(Normal), Some(Double), Some(Double)) => Ok(""),
            (None, Some(Thick), None, None) => Ok("╹"),
            (None, Some(Thick), None, Some(Normal)) => Ok("╿"),
            (None, Some(Thick), None, Some(Thick)) => Ok("┃"),
            // (None, Some(Thick), None, Some(Double)) => Ok("┃"),
            (None, Some(Thick), Some(Normal), None) => Ok("┚"),
            (None, Some(Thick), Some(Normal), Some(Normal)) => Ok("┦"),
            (None, Some(Thick), Some(Normal), Some(Thick)) => Ok("┨"),
            // (None, Some(Thick), Some(Normal), Some(Double)) => Ok(""),
            (None, Some(Thick), Some(Thick), None) => Ok("┛"),
            (None, Some(Thick), Some(Thick), Some(Normal)) => Ok("┩"),
            (None, Some(Thick), Some(Thick), Some(Thick)) => Ok("┫"),
            // (None, Some(Thick), Some(Thick), Some(Double)) => Ok(""),
            // (None, Some(Thick), Some(Double), None) => Ok(""),
            // (None, Some(Thick), Some(Double), Some(Normal)) => Ok(""),
            // (None, Some(Thick), Some(Double), Some(Thick)) => Ok(""),
            // (None, Some(Thick), Some(Double), Some(Double)) => Ok(""),
            // (None, Some(Double), None, None) => Ok(""),
            // (None, Some(Double), None, Some(Normal)) => Ok(""),
            // (None, Some(Double), None, Some(Thick)) => Ok(""),
            (None, Some(Double), None, Some(Double)) => Ok("║"),
            (None, Some(Double), Some(Normal), None) => Ok("╜"),
            // (None, Some(Double), Some(Normal), Some(Normal)) => Ok(""),
            // (None, Some(Double), Some(Normal), Some(Thick)) => Ok(""),
            (None, Some(Double), Some(Normal), Some(Double)) => Ok("╢"),
            // (None, Some(Double), Some(Thick), None) => Ok(""),
            // (None, Some(Double), Some(Thick), Some(Normal)) => Ok(""),
            // (None, Some(Double), Some(Thick), Some(Thick)) => Ok(""),
            // (None, Some(Double), Some(Thick), Some(Double)) => Ok(""),
            (None, Some(Double), Some(Double), None) => Ok("╝"),
            // (None, Some(Double), Some(Double), Some(Normal)) => Ok(""),
            // (None, Some(Double), Some(Double), Some(Thick)) => Ok(""),
            (None, Some(Double), Some(Double), Some(Double)) => Ok("╣"),
            (Some(Normal), None, None, None) => Ok("╶"),
            (Some(Normal), None, None, Some(Normal)) => Ok("┌"),
            (Some(Normal), None, None, Some(Thick)) => Ok("┎"),
            (Some(Normal), None, None, Some(Double)) => Ok("╓"),
            (Some(Normal), None, Some(Normal), None) => Ok("─"),
            (Some(Normal), None, Some(Normal), Some(Normal)) => Ok("┬"),
            (Some(Normal), None, Some(Normal), Some(Thick)) => Ok("┰"),
            (Some(Normal), None, Some(Normal), Some(Double)) => Ok("╥"),
            (Some(Normal), None, Some(Thick), None) => Ok("╾"),
            (Some(Normal), None, Some(Thick), Some(Normal)) => Ok("┭"),
            (Some(Normal), None, Some(Thick), Some(Thick)) => Ok("┱"),
            // (Some(Normal), None, Some(Thick), Some(Double)) => Ok(""),
            // (Some(Normal), None, Some(Double), None) => Ok(""),
            // (Some(Normal), None, Some(Double), Some(Normal)) => Ok(""),
            // (Some(Normal), None, Some(Double), Some(Thick)) => Ok(""),
            // (Some(Normal), None, Some(Double), Some(Double)) => Ok(""),
            (Some(Normal), Some(Normal), None, None) => Ok("└"),
            (Some(Normal), Some(Normal), None, Some(Normal)) => Ok("├"),
            (Some(Normal), Some(Normal), None, Some(Thick)) => Ok("┟"),
            // (Some(Normal), Some(Normal), None, Some(Double)) => Ok(""),
            (Some(Normal), Some(Normal), Some(Normal), None) => Ok("┴"),
            (Some(Normal), Some(Normal), Some(Normal), Some(Normal)) => Ok("┼"),
            (Some(Normal), Some(Normal), Some(Normal), Some(Thick)) => Ok("╁"),
            // (Some(Normal), Some(Normal), Some(Normal), Some(Double)) => Ok(""),
            (Some(Normal), Some(Normal), Some(Thick), None) => Ok("┵"),
            (Some(Normal), Some(Normal), Some(Thick), Some(Normal)) => Ok("┽"),
            (Some(Normal), Some(Normal), Some(Thick), Some(Thick)) => Ok("╅"),
            // (Some(Normal), Some(Normal), Some(Thick), Some(Double)) => Ok(""),
            // (Some(Normal), Some(Normal), Some(Double), None) => Ok(""),
            // (Some(Normal), Some(Normal), Some(Double), Some(Normal)) => Ok(""),
            // (Some(Normal), Some(Normal), Some(Double), Some(Thick)) => Ok(""),
            // (Some(Normal), Some(Normal), Some(Double), Some(Double)) => Ok(""),
            (Some(Normal), Some(Thick), None, None) => Ok("┖"),
            (Some(Normal), Some(Thick), None, Some(Normal)) => Ok("┞"),
            (Some(Normal), Some(Thick), None, Some(Thick)) => Ok("┠"),
            // (Some(Normal), Some(Thick), None, Some(Double)) => Ok(""),
            (Some(Normal), Some(Thick), Some(Normal), None) => Ok("┸"),
            (Some(Normal), Some(Thick), Some(Normal), Some(Normal)) => Ok("╀"),
            (Some(Normal), Some(Thick), Some(Normal), Some(Thick)) => Ok("╂"),
            // (Some(Normal), Some(Thick), Some(Normal), Some(Double)) => Ok(""),
            (Some(Normal), Some(Thick), Some(Thick), None) => Ok("┹"),
            (Some(Normal), Some(Thick), Some(Thick), Some(Normal)) => Ok("╃"),
            (Some(Normal), Some(Thick), Some(Thick), Some(Thick)) => Ok("╉"),
            // (Some(Normal), Some(Thick), Some(Thick), Some(Double)) => Ok(""),
            // (Some(Normal), Some(Thick), Some(Double), None) => Ok(""),
            // (Some(Normal), Some(Thick), Some(Double), Some(Normal)) => Ok(""),
            // (Some(Normal), Some(Thick), Some(Double), Some(Thick)) => Ok(""),
            // (Some(Normal), Some(Thick), Some(Double), Some(Double)) => Ok(""),
            (Some(Normal), Some(Double), None, None) => Ok("╙"),
            // (Some(Normal), Some(Double), None, Some(Normal)) => Ok("╨"),
            // (Some(Normal), Some(Double), None, Some(Thick)) => Ok(""),
            (Some(Normal), Some(Double), None, Some(Double)) => Ok("╟"),
            (Some(Normal), Some(Double), Some(Normal), None) => Ok("╨"),
            // (Some(Normal), Some(Double), Some(Normal), Some(Normal)) => Ok(""),
            // (Some(Normal), Some(Double), Some(Normal), Some(Thick)) => Ok(""),
            (Some(Normal), Some(Double), Some(Normal), Some(Double)) => Ok("╫"),
            // (Some(Normal), Some(Double), Some(Thick), None) => Ok(""),
            // (Some(Normal), Some(Double), Some(Thick), Some(Normal)) => Ok(""),
            // (Some(Normal), Some(Double), Some(Thick), Some(Thick)) => Ok(""),
            // (Some(Normal), Some(Double), Some(Thick), Some(Double)) => Ok(""),
            // (Some(Normal), Some(Double), Some(Double), None) => Ok(""),
            // (Some(Normal), Some(Double), Some(Double), Some(Normal)) => Ok(""),
            // (Some(Normal), Some(Double), Some(Double), Some(Thick)) => Ok(""),
            // (Some(Normal), Some(Double), Some(Double), Some(Double)) => Ok(""),
            (Some(Thick), None, None, None) => Ok("╺"),
            (Some(Thick), None, None, Some(Normal)) => Ok("┍"),
            (Some(Thick), None, None, Some(Thick)) => Ok("┏"),
            // (Some(Thick), None, None, Some(Double)) => Ok(""),
            (Some(Thick), None, Some(Normal), None) => Ok("╼"),
            (Some(Thick), None, Some(Normal), Some(Normal)) => Ok("┮"),
            (Some(Thick), None, Some(Normal), Some(Thick)) => Ok("┲"),
            // (Some(Thick), None, Some(Normal), Some(Double)) => Ok(""),
            (Some(Thick), None, Some(Thick), None) => Ok("━"),
            (Some(Thick), None, Some(Thick), Some(Normal)) => Ok("┯"),
            (Some(Thick), None, Some(Thick), Some(Thick)) => Ok("┳"),
            // (Some(Thick), None, Some(Thick), Some(Double)) => Ok(""),
            // (Some(Thick), None, Some(Double), None) => Ok(""),
            // (Some(Thick), None, Some(Double), Some(Normal)) => Ok(""),
            // (Some(Thick), None, Some(Double), Some(Thick)) => Ok(""),
            // (Some(Thick), None, Some(Double), Some(Double)) => Ok(""),
            (Some(Thick), Some(Normal), None, None) => Ok("┕"),
            (Some(Thick), Some(Normal), None, Some(Normal)) => Ok("┝"),
            (Some(Thick), Some(Normal), None, Some(Thick)) => Ok("┢"),
            // (Some(Thick), Some(Normal), None, Some(Double)) => Ok(""),
            (Some(Thick), Some(Normal), Some(Normal), None) => Ok("┶"),
            (Some(Thick), Some(Normal), Some(Normal), Some(Normal)) => Ok("┾"),
            (Some(Thick), Some(Normal), Some(Normal), Some(Thick)) => Ok("╆"),
            // (Some(Thick), Some(Normal), Some(Normal), Some(Double)) => Ok(""),
            (Some(Thick), Some(Normal), Some(Thick), None) => Ok("┷"),
            (Some(Thick), Some(Normal), Some(Thick), Some(Normal)) => Ok("┿"),
            (Some(Thick), Some(Normal), Some(Thick), Some(Thick)) => Ok("╈"),
            // (Some(Thick), Some(Normal), Some(Thick), Some(Double)) => Ok(""),
            // (Some(Thick), Some(Normal), Some(Double), None) => Ok(""),
            // (Some(Thick), Some(Normal), Some(Double), Some(Normal)) => Ok(""),
            // (Some(Thick), Some(Normal), Some(Double), Some(Thick)) => Ok(""),
            // (Some(Thick), Some(Normal), Some(Double), Some(Double)) => Ok(""),
            (Some(Thick), Some(Thick), None, None) => Ok("┗"),
            (Some(Thick), Some(Thick), None, Some(Normal)) => Ok("┡"),
            (Some(Thick), Some(Thick), None, Some(Thick)) => Ok("┣"),
            // (Some(Thick), Some(Thick), None, Some(Double)) => Ok(""),
            (Some(Thick), Some(Thick), Some(Normal), None) => Ok("┺"),
            (Some(Thick), Some(Thick), Some(Normal), Some(Normal)) => Ok("╄"),
            (Some(Thick), Some(Thick), Some(Normal), Some(Thick)) => Ok("╊"),
            // (Some(Thick), Some(Thick), Some(Normal), Some(Double)) => Ok(""),
            (Some(Thick), Some(Thick), Some(Thick), None) => Ok("┻"),
            (Some(Thick), Some(Thick), Some(Thick), Some(Normal)) => Ok("╇"),
            (Some(Thick), Some(Thick), Some(Thick), Some(Thick)) => Ok("╋"),
            // (Some(Thick), Some(Thick), Some(Thick), Some(Double)) => Ok(""),
            // (Some(Thick), Some(Thick), Some(Double), None) => Ok(""),
            // (Some(Thick), Some(Thick), Some(Double), Some(Normal)) => Ok(""),
            // (Some(Thick), Some(Thick), Some(Double), Some(Thick)) => Ok(""),
            // (Some(Thick), Some(Thick), Some(Double), Some(Double)) => Ok(""),
            // (Some(Thick), Some(Double), None, None) => Ok(""),
            // (Some(Thick), Some(Double), None, Some(Normal)) => Ok(""),
            // (Some(Thick), Some(Double), None, Some(Thick)) => Ok(""),
            // (Some(Thick), Some(Double), None, Some(Double)) => Ok(""),
            // (Some(Thick), Some(Double), Some(Normal), None) => Ok(""),
            // (Some(Thick), Some(Double), Some(Normal), Some(Normal)) => Ok(""),
            // (Some(Thick), Some(Double), Some(Normal), Some(Thick)) => Ok(""),
            // (Some(Thick), Some(Double), Some(Normal), Some(Double)) => Ok(""),
            // (Some(Thick), Some(Double), Some(Thick), None) => Ok(""),
            // (Some(Thick), Some(Double), Some(Thick), Some(Normal)) => Ok(""),
            // (Some(Thick), Some(Double), Some(Thick), Some(Thick)) => Ok(""),
            // (Some(Thick), Some(Double), Some(Thick), Some(Double)) => Ok(""),
            // (Some(Thick), Some(Double), Some(Double), None) => Ok(""),
            // (Some(Thick), Some(Double), Some(Double), Some(Normal)) => Ok(""),
            // (Some(Thick), Some(Double), Some(Double), Some(Thick)) => Ok(""),
            // (Some(Thick), Some(Double), Some(Double), Some(Double)) => Ok(""),
            // (Some(Double), None, None, None) => Ok(""),
            (Some(Double), None, None, Some(Normal)) => Ok("╒"),
            // (Some(Double), None, None, Some(Thick)) => Ok(""),
            (Some(Double), None, None, Some(Double)) => Ok("╔"),
            // (Some(Double), None, Some(Normal), None) => Ok(""),
            // (Some(Double), None, Some(Normal), Some(Normal)) => Ok(""),
            // (Some(Double), None, Some(Normal), Some(Thick)) => Ok(""),
            // (Some(Double), None, Some(Normal), Some(Double)) => Ok(""),
            // (Some(Double), None, Some(Thick), None) => Ok(""),
            // (Some(Double), None, Some(Thick), Some(Normal)) => Ok(""),
            // (Some(Double), None, Some(Thick), Some(Thick)) => Ok(""),
            // (Some(Double), None, Some(Thick), Some(Double)) => Ok(""),
            (Some(Double), None, Some(Double), None) => Ok("═"),
            (Some(Double), None, Some(Double), Some(Normal)) => Ok("╤"),
            // (Some(Double), None, Some(Double), Some(Thick)) => Ok(""),
            (Some(Double), None, Some(Double), Some(Double)) => Ok("╦"),
            (Some(Double), Some(Normal), None, None) => Ok("╘"),
            (Some(Double), Some(Normal), None, Some(Normal)) => Ok("╞"),
            // (Some(Double), Some(Normal), None, Some(Thick)) => Ok(""),
            // (Some(Double), Some(Normal), None, Some(Double)) => Ok(""),
            // (Some(Double), Some(Normal), Some(Normal), None) => Ok(""),
            // (Some(Double), Some(Normal), Some(Normal), Some(Normal)) => Ok(""),
            // (Some(Double), Some(Normal), Some(Normal), Some(Thick)) => Ok(""),
            // (Some(Double), Some(Normal), Some(Normal), Some(Double)) => Ok(""),
            // (Some(Double), Some(Normal), Some(Thick), None) => Ok(""),
            // (Some(Double), Some(Normal), Some(Thick), Some(Normal)) => Ok(""),
            // (Some(Double), Some(Normal), Some(Thick), Some(Thick)) => Ok(""),
            // (Some(Double), Some(Normal), Some(Thick), Some(Double)) => Ok(""),
            (Some(Double), Some(Normal), Some(Double), None) => Ok("╧"),
            (Some(Double), Some(Normal), Some(Double), Some(Normal)) => Ok("╪"),
            // (Some(Double), Some(Normal), Some(Double), Some(Thick)) => Ok(""),
            // (Some(Double), Some(Normal), Some(Double), Some(Double)) => Ok(""),
            // (Some(Double), Some(Thick), None, None) => Ok(""),
            // (Some(Double), Some(Thick), None, Some(Normal)) => Ok(""),
            // (Some(Double), Some(Thick), None, Some(Thick)) => Ok(""),
            // (Some(Double), Some(Thick), None, Some(Double)) => Ok(""),
            // (Some(Double), Some(Thick), Some(Normal), None) => Ok(""),
            // (Some(Double), Some(Thick), Some(Normal), Some(Normal)) => Ok(""),
            // (Some(Double), Some(Thick), Some(Normal), Some(Thick)) => Ok(""),
            // (Some(Double), Some(Thick), Some(Normal), Some(Double)) => Ok(""),
            // (Some(Double), Some(Thick), Some(Thick), None) => Ok(""),
            // (Some(Double), Some(Thick), Some(Thick), Some(Normal)) => Ok(""),
            // (Some(Double), Some(Thick), Some(Thick), Some(Thick)) => Ok(""),
            // (Some(Double), Some(Thick), Some(Thick), Some(Double)) => Ok(""),
            // (Some(Double), Some(Thick), Some(Double), None) => Ok(""),
            // (Some(Double), Some(Thick), Some(Double), Some(Normal)) => Ok(""),
            // (Some(Double), Some(Thick), Some(Double), Some(Thick)) => Ok(""),
            // (Some(Double), Some(Thick), Some(Double), Some(Double)) => Ok(""),
            (Some(Double), Some(Double), None, None) => Ok("╚"),
            // (Some(Double), Some(Double), None, Some(Normal)) => Ok(""),
            // (Some(Double), Some(Double), None, Some(Thick)) => Ok(""),
            (Some(Double), Some(Double), None, Some(Double)) => Ok("╠"),
            // (Some(Double), Some(Double), Some(Normal), None) => Ok(""),
            // (Some(Double), Some(Double), Some(Normal), Some(Normal)) => Ok(""),
            // (Some(Double), Some(Double), Some(Normal), Some(Thick)) => Ok(""),
            // (Some(Double), Some(Double), Some(Normal), Some(Double)) => Ok(""),
            // (Some(Double), Some(Double), Some(Thick), None) => Ok(""),
            // (Some(Double), Some(Double), Some(Thick), Some(Normal)) => Ok(""),
            // (Some(Double), Some(Double), Some(Thick), Some(Thick)) => Ok(""),
            // (Some(Double), Some(Double), Some(Thick), Some(Double)) => Ok(""),
            (Some(Double), Some(Double), Some(Double), None) => Ok("╩"),
            // (Some(Double), Some(Double), Some(Double), Some(Normal)) => Ok(""),
            // (Some(Double), Some(Double), Some(Double), Some(Thick)) => Ok(""),
            (Some(Double), Some(Double), Some(Double), Some(Double)) => Ok("╬"),
            _ => Err(()),
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
