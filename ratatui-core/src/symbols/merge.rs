use core::str::FromStr;

/// Defines the merge strategy of overlapping characters.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum MergeStrategy {
    /// Replaces the previous symbol with the next one.
    #[default]
    Replace,

    /// Merges symbols only if an exact composite unicode character exists.
    ///
    /// Example: `┐` and `┗` will be merged into `╄`
    Exact,

    /// Merges symbols even if an exact composite unicode character doesn't exist,
    /// using the closest match.
    ///
    /// Example: `╮` and `└` will be merged into `┼`
    Fuzzy,
}

impl MergeStrategy {
    pub fn merge<'a>(self, prev: &'a str, next: &'a str) -> &'a str {
        let (Ok(prev_symbol), Ok(next_symbol)) =
            (BorderSymbol::try_from(prev), BorderSymbol::try_from(next))
        else {
            return next;
        };
        if let Ok(merged) = prev_symbol.merge(next_symbol, self).try_into() {
            return merged;
        }
        next
    }
}

/// Represents a composite border symbol using individual line components.
#[derive(Debug, Copy, Clone)]
struct BorderSymbol {
    pub right: LineStyle,
    pub up: LineStyle,
    pub left: LineStyle,
    pub down: LineStyle,
}

impl BorderSymbol {
    /// Creates a new `BorderSymbol`, based on individual line styles.
    #[must_use]
    pub const fn new(right: LineStyle, up: LineStyle, left: LineStyle, down: LineStyle) -> Self {
        Self {
            right,
            up,
            left,
            down,
        }
    }

    /// Finds the closest representation of the `BorderSymbol`, that has a corresponding unicode
    /// character.
    #[must_use]
    pub fn fuzzy(mut self) -> Self {
        #[allow(clippy::enum_glob_use)]
        use LineStyle::*;

        // Dashes only include vertical and horizontal lines.
        if !self.is_straight() {
            self = self
                .replace(DoubleDash, Plain)
                .replace(TripleDash, Plain)
                .replace(QuadrupleDash, Plain)
                .replace(DoubleDashThick, Thick)
                .replace(TripleDashThick, Thick)
                .replace(QuadrupleDashThick, Thick);
        }

        // Rounded has only corner variants.
        if !self.is_corner() {
            self = self.replace(Rounded, Plain);
        }

        // There are no Double + Thick variants.
        // Additionally, some Plain + Double variants don't exist.
        // TODO: check if this is good enough after adding tests
        if TryInto::<&str>::try_into(self).is_err() {
            self = self.replace(Double, Thick);
        }
        self
    }

    /// Return true only if the symbol is a line and both parts have the same [`LineStyle`].
    fn is_straight(self) -> bool {
        use LineStyle::Nothing;
        (self.up == self.down && self.left == self.right)
            && (self.up == Nothing || self.left == Nothing)
    }

    /// Return true only if the symbol is a corner and both parts have the same [`LineStyle`].
    fn is_corner(self) -> bool {
        use LineStyle::Nothing;
        // TODO: maybe this can be simplified?
        match (self.up, self.right, self.down, self.left) {
            (up, right, Nothing, Nothing) => up == right,
            (Nothing, right, down, Nothing) => right == down,
            (Nothing, Nothing, down, left) => down == left,
            (up, Nothing, Nothing, left) => up == left,
            _ => false,
        }
    }

    // TODO: not removing this yet as fuzzy strategy implementation may change
    /// Checks if any of the line components making the `BorderSymbol` matches the `style`.
    #[allow(dead_code)]
    pub fn contains(self, style: LineStyle) -> bool {
        self.up == style || self.right == style || self.down == style || self.left == style
    }

    /// Replaces all line styles matching `from` by `to`.
    #[must_use]
    pub fn replace(mut self, from: LineStyle, to: LineStyle) -> Self {
        self.up = if self.up == from { to } else { self.up };
        self.right = if self.right == from { to } else { self.right };
        self.down = if self.down == from { to } else { self.down };
        self.left = if self.left == from { to } else { self.left };
        self
    }

    /// Merges two border symbols into one.
    pub fn merge(self, other: Self, strategy: MergeStrategy) -> Self {
        let exact_result = Self::new(
            self.right.merge(other.right),
            self.up.merge(other.up),
            self.left.merge(other.left),
            self.down.merge(other.down),
        );
        match strategy {
            MergeStrategy::Replace => other,
            MergeStrategy::Fuzzy => exact_result.fuzzy(),
            MergeStrategy::Exact => exact_result,
        }
    }
}

impl FromStr for BorderSymbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

/// A visual style defining the appearance of a single line making up a block border.
#[derive(PartialEq, Debug, Clone, Copy)]
enum LineStyle {
    /// Represents the absence of a line.
    Nothing,

    /// A single line (e.g. `─`, `│`).
    Plain,

    /// A rounded line style, only applicable in corner symbols (e.g. `╭`, `╯`).
    Rounded,

    /// A double line (e.g. `═`, `║`).
    Double,

    /// A thickened line (e.g. `━`, `┃`).
    Thick,

    /// A dashed line with a double dash pattern (e.g. `╌`, `╎`).
    DoubleDash,

    /// A thicker variant of the double dash (e.g. `╍`, `╏`)
    DoubleDashThick,

    /// A dashed line with a triple dash pattern (e.g. `┄`, `┆`).
    TripleDash,

    /// A thicker variant of the triple dash (e.g. `┅`, `┇`).
    TripleDashThick,

    /// A dashed line with four dashes (e.g. `┈`, `┊`).
    QuadrupleDash,

    /// A thicker variant of the quadruple dash (e.g. `┉`, `┋`).
    QuadrupleDashThick,
}

impl LineStyle {
    /// Merges line styles.
    #[must_use]
    pub fn merge(self, other: Self) -> Self {
        if other == Self::Nothing { self } else { other }
    }
}

// Defines a translation between `BorderSymbol` and the corresponding character.
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

        impl TryFrom<BorderSymbol> for &'static str {
            type Error = ();
            fn try_from(value: BorderSymbol) -> Result<Self, Self::Error> {
                use LineStyle::*;
                match (value.right, value.up, value.left, value.down) {
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
    "┌" => (Plain, Nothing, Nothing, Plain),
    "┍" => (Thick, Nothing, Nothing, Plain),
    "┎" => (Plain, Nothing, Nothing, Thick),
    "┏" => (Thick, Nothing, Nothing, Thick),
    "┐" => (Nothing, Nothing, Plain, Plain),
    "┑" => (Nothing, Nothing, Thick, Plain),
    "┒" => (Nothing, Nothing, Plain, Thick),
    "┓" => (Nothing, Nothing, Thick, Thick),
    "└" => (Plain, Plain, Nothing, Nothing),
    "┕" => (Thick, Plain, Nothing, Nothing),
    "┖" => (Plain, Thick, Nothing, Nothing),
    "┗" => (Thick, Thick, Nothing, Nothing),
    "┘" => (Nothing, Plain, Plain, Nothing),
    "┙" => (Nothing, Plain, Thick, Nothing),
    "┚" => (Nothing, Thick, Plain, Nothing),
    "┛" => (Nothing, Thick, Thick, Nothing),
    "├" => (Plain, Plain, Nothing, Plain),
    "┝" => (Thick, Plain, Nothing, Plain),
    "┞" => (Plain, Thick, Nothing, Plain),
    "┟" => (Plain, Plain, Nothing, Thick),
    "┠" => (Plain, Thick, Nothing, Thick),
    "┡" => (Thick, Thick, Nothing, Plain),
    "┢" => (Thick, Plain, Nothing, Thick),
    "┣" => (Thick, Thick, Nothing, Thick),
    "┤" => (Nothing, Plain, Plain, Plain),
    "┥" => (Nothing, Plain, Thick, Plain),
    "┦" => (Nothing, Thick, Plain, Plain),
    "┧" => (Nothing, Plain, Plain, Thick),
    "┨" => (Nothing, Thick, Plain, Thick),
    "┩" => (Nothing, Thick, Thick, Plain),
    "┪" => (Nothing, Plain, Thick, Thick),
    "┫" => (Nothing, Thick, Thick, Thick),
    "┬" => (Plain, Nothing, Plain, Plain),
    "┭" => (Plain, Nothing, Thick, Plain),
    "┮" => (Thick, Nothing, Plain, Plain),
    "┯" => (Thick, Nothing, Thick, Plain),
    "┰" => (Plain, Nothing, Plain, Thick),
    "┱" => (Plain, Nothing, Thick, Thick),
    "┲" => (Thick, Nothing, Plain, Thick),
    "┳" => (Thick, Nothing, Thick, Thick),
    "┴" => (Plain, Plain, Plain, Nothing),
    "┵" => (Plain, Plain, Thick, Nothing),
    "┶" => (Thick, Plain, Plain, Nothing),
    "┷" => (Thick, Plain, Thick, Nothing),
    "┸" => (Plain, Thick, Plain, Nothing),
    "┹" => (Plain, Thick, Thick, Nothing),
    "┺" => (Thick, Thick, Plain, Nothing),
    "┻" => (Thick, Thick, Thick, Nothing),
    "┼" => (Plain, Plain, Plain, Plain),
    "┽" => (Plain, Plain, Thick, Plain),
    "┾" => (Thick, Plain, Plain, Plain),
    "┿" => (Thick, Plain, Thick, Plain),
    "╀" => (Plain, Thick, Plain, Plain),
    "╁" => (Plain, Plain, Plain, Thick),
    "╂" => (Plain, Thick, Plain, Thick),
    "╃" => (Plain, Thick, Thick, Plain),
    "╄" => (Thick, Thick, Plain, Plain),
    "╅" => (Plain, Plain, Thick, Thick),
    "╆" => (Thick, Plain, Plain, Thick),
    "╇" => (Thick, Thick, Thick, Plain),
    "╈" => (Thick, Plain, Thick, Thick),
    "╉" => (Plain, Thick, Thick, Thick),
    "╊" => (Thick, Thick, Plain, Thick),
    "╋" => (Thick, Thick, Thick, Thick),
    "╌" => (DoubleDash, Nothing, DoubleDash, Nothing),
    "╍" => (DoubleDashThick, Nothing, DoubleDashThick, Nothing),
    "╎" => (Nothing, DoubleDash, Nothing, DoubleDash),
    "╏" => (Nothing, DoubleDashThick, Nothing, DoubleDashThick),
    "═" => (Double, Nothing, Double, Nothing),
    "║" => (Nothing, Double, Nothing, Double),
    "╒" => (Double, Nothing, Nothing, Plain),
    "╓" => (Plain, Nothing, Nothing, Double),
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
