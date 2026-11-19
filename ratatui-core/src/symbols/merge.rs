//! This module provides strategies for merging symbols in a layout.
//!
//! It defines the [`MergeStrategy`] enum, which allows for different behaviors when combining
//! symbols, such as replacing the previous symbol, merging them if an exact match exists, or using
//! a fuzzy match to find the closest representation.
//!
//! The merging strategies are useful for [collapsing borders] in layouts, where multiple symbols
//! may need to be combined to create a single, coherent border representation.
//!
//! [collapsing borders]: https://ratatui.rs/recipes/layout/collapse-borders
use core::str::FromStr;

use crate::symbols::border;

/// A strategy for merging two symbols into one.
///
/// This enum defines how two symbols should be merged together, allowing for different behaviors
/// when combining symbols, such as replacing the previous symbol, merging them if an exact match
/// exists, or using a fuzzy match to find the closest representation.
///
/// This is useful for [collapsing borders] in layouts, where multiple symbols may need to be
/// combined to create a single, coherent border representation.
///
/// Not all combinations of box drawing symbols can be represented as a single unicode character, as
/// many of them are not defined in the [Box Drawing Unicode block]. This means that some merging
/// strategies will not yield a valid unicode character. The [`MergeStrategy::Replace`] strategy
/// will be used as a fallback in such cases, replacing the previous symbol with the next one.
///
/// Specifically, the following combinations of box drawing symbols are not defined in the [Box
/// Drawing Unicode block]:
///
/// - Combining any dashed segments with any non dashed segments (e.g. `╎` with `─` or `━`).
/// - Combining any rounded segments with any other segments (e.g. `╯` with `─` or `━`).
/// - Combining any double segments with any thick segments (e.g. `═` with `┃` or `━`).
/// - Combining some double segments with some plain segments (e.g. `┐` with `╔`).
///
/// Block elements made of quadrants merge into the block element that fills the quadrants of both
/// symbols. Unicode has a character for every combination, so [`Self::Exact`] and [`Self::Fuzzy`]
/// always merge them. This collapses borders drawn with [`border::QUADRANT_INSIDE`],
/// [`border::QUADRANT_OUTSIDE`], [`border::PROPORTIONAL_WIDE`], [`border::PROPORTIONAL_TALL`] and
/// [`border::FULL`]. No character combines a box drawing line with a quadrant, so merging one with
/// the other falls back to [`Self::Replace`].
///
/// The following diagram shows blocks with [`border::QUADRANT_INSIDE`] borders merged with either
/// strategy:
///
/// ```text
/// ▗▄▄▄▖     ▗▄▄▄▖   ▗▄▄▄▄▄▄▄▖ ▗▄▄▄▖
/// ▐   ▌     ▐   ▌   ▐   █   ▌ ▐   ▌
/// ▐   ▌     ▐ ▗▄▙▄▖ ▐   █   ▌ ▐   ▌
/// ▐   ▌     ▐ ▐ ▌ ▌ ▐   █   ▌ ▐   ▌
/// ▝▀▀▀▚▄▄▄▖ ▝▀▜▀▘ ▌ ▝▀▀▀▀▀▀▀▘ ▐███▌
///     ▐   ▌   ▐   ▌           ▐   ▌
///     ▐   ▌   ▝▀▀▀▘           ▐   ▌
///     ▐   ▌                   ▐   ▌
///     ▝▀▀▀▘                   ▝▀▀▀▘
/// ```
///
/// The merging strategies include:
///
/// - [`Self::Replace`]: Replaces the previous symbol with the next one.
/// - [`Self::Exact`]: Merges symbols only if an exact composite unicode character exists, falling
///   back to [`Self::Replace`] if not.
/// - [`Self::Fuzzy`]: Merges symbols even if an exact composite unicode character doesn't exist,
///   using the closest match, and falling back to [`Self::Exact`] if necessary.
///
/// See [`Cell::merge_symbol`] for how to use this strategy in practice, and
/// [`Block::merge_borders`] for a more concrete example of merging borders in a layout.
///
/// # Examples
///
/// ```
/// # use ratatui_core::symbols::merge::MergeStrategy;
///
/// assert_eq!(MergeStrategy::Replace.merge("│", "━"), "━");
/// assert_eq!(MergeStrategy::Exact.merge("│", "─"), "┼");
/// assert_eq!(MergeStrategy::Fuzzy.merge("┘", "╔"), "╬");
/// assert_eq!(MergeStrategy::Exact.merge("▌", "▐"), "█");
/// ```
///
/// [Box Drawing Unicode block]: https://en.wikipedia.org/wiki/Box_Drawing
/// [collapsing borders]: https://ratatui.rs/recipes/layout/collapse-borders
/// [`Block::merge_borders`]:
///     https://docs.rs/ratatui/latest/ratatui/widgets/struct.Block.html#method.merge_borders
/// [`Cell::merge_symbol`]: crate::buffer::Cell::merge_symbol
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum MergeStrategy {
    /// Replaces the previous symbol with the next one.
    ///
    /// This strategy simply replaces the previous symbol with the next one, without attempting to
    /// merge them. This is useful when you want to ensure that the last rendered symbol takes
    /// precedence over the previous one, regardless of their compatibility.
    ///
    /// The following diagram illustrates how this would apply to several overlapping blocks where
    /// the thick bordered blocks are rendered last, replacing the previous symbols:
    ///
    /// ```text
    /// ┌───┐    ┌───┐  ┌───┏━━━┓┌───┐
    /// │   │    │   │  │   ┃   ┃│   │
    /// │   │    │ ┏━━━┓│   ┃   ┃│   │
    /// │   │    │ ┃ │ ┃│   ┃   ┃│   │
    /// └───┏━━━┓└─┃─┘ ┃└───┗━━━┛┏━━━┓
    ///     ┃   ┃  ┃   ┃         ┃   ┃
    ///     ┃   ┃  ┗━━━┛         ┃   ┃
    ///     ┃   ┃                ┃   ┃
    ///     ┗━━━┛                ┗━━━┛
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// # use ratatui_core::symbols::merge::MergeStrategy;
    /// let strategy = MergeStrategy::Replace;
    /// assert_eq!(strategy.merge("│", "━"), "━");
    /// ```
    #[default]
    Replace,

    /// Merges symbols only if an exact composite unicode character exists.
    ///
    /// This strategy attempts to merge two symbols into a single composite unicode character if the
    /// exact representation exists. If the required unicode symbol does not exist, it falls back to
    /// [`MergeStrategy::Replace`], replacing the previous symbol with the next one.
    ///
    /// The following diagram illustrates how this would apply to several overlapping blocks where
    /// the thick bordered blocks are rendered last, merging the previous symbols into a single
    /// composite character. All combinations of the plain and thick segments exist, so these
    /// symbols can be merged into a single character:
    ///
    /// ```text
    /// ┌───┐    ┌───┐  ┌───┲━━━┓┌───┐
    /// │   │    │   │  │   ┃   ┃│   │
    /// │   │    │ ┏━┿━┓│   ┃   ┃│   │
    /// │   │    │ ┃ │ ┃│   ┃   ┃│   │
    /// └───╆━━━┓└─╂─┘ ┃└───┺━━━┛┢━━━┪
    ///     ┃   ┃  ┃   ┃         ┃   ┃
    ///     ┃   ┃  ┗━━━┛         ┃   ┃
    ///     ┃   ┃                ┃   ┃
    ///     ┗━━━┛                ┗━━━┛
    /// ```
    ///
    /// The following diagram illustrates how this would apply to several overlapping blocks where
    /// the characters don't have a composite unicode character, so the previous symbols are
    /// replaced by the next one:
    ///
    /// ```text
    /// ┌───┐    ┌───┐  ┌───╔═══╗┌───┐
    /// │   │    │   │  │   ║   ║│   │
    /// │   │    │ ╔═╪═╗│   ║   ║│   │
    /// │   │    │ ║ │ ║│   ║   ║│   │
    /// └───╔═══╗└─╫─┘ ║└───╚═══╝╔═══╗
    ///     ║   ║  ║   ║         ║   ║
    ///     ║   ║  ╚═══╝         ║   ║
    ///     ║   ║                ║   ║
    ///     ╚═══╝                ╚═══╝
    /// ┌───┐    ┌───┐  ┌───╭───╮┌───┐
    /// │   │    │   │  │   │   ││   │
    /// │   │    │ ╭─┼─╮│   │   ││   │
    /// │   │    │ │ │ ││   │   ││   │
    /// └───╭───╮└─┼─┘ │└───╰───╯╭───╮
    ///     │   │  │   │         │   │
    ///     │   │  ╰───╯         │   │
    ///     │   │                │   │
    ///     ╰───╯                ╰───╯
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// # use ratatui_core::symbols::merge::MergeStrategy;
    /// let strategy = MergeStrategy::Exact;
    /// assert_eq!(strategy.merge("│", "━"), "┿"); // exact match exists
    /// assert_eq!(strategy.merge("┘", "╔"), "╔"); // no exact match, falls back to Replace
    /// ```
    Exact,

    /// Merges symbols even if an exact composite unicode character doesn't exist, using the closest
    /// match.
    ///
    /// If required unicode symbol exists, acts exactly like [`MergeStrategy::Exact`], if not, the
    /// following rules are applied:
    ///
    /// 1. There are no characters that combine dashed with plain / thick segments, so we replace
    ///    dashed segments with plain and thick dashed segments with thick. The following diagram
    ///    shows how this would apply to merging a block with thick dashed borders over a block with
    ///    plain dashed borders:
    ///
    /// ```text
    /// ┌╌╌╌┐    ┌╌╌╌┐  ┌╌╌╌┲╍╍╍┓┌╌╌╌┐
    /// ╎   ╎    ╎   ╎  ╎   ╏   ╏╎   ╎
    /// ╎   ╎    ╎ ┏╍┿╍┓╎   ╏   ╏╎   ╎
    /// ╎   ╎    ╎ ╏ ╎ ╏╎   ╏   ╏╎   ╎
    /// └╌╌╌╆╍╍╍┓└╌╂╌┘ ╏└╌╌╌┺╍╍╍┛┢╍╍╍┪
    ///     ╏   ╏  ╏   ╏         ╏   ╏
    ///     ╏   ╏  ┗╍╍╍┛         ╏   ╏
    ///     ╏   ╏                ╏   ╏
    ///     ┗╍╍╍┛                ┗╍╍╍┛
    /// ```
    ///
    /// 2. There are no characters that combine rounded segments with other segments, so we replace
    ///    rounded segments with plain. The following diagram shows how this would apply to merging
    ///    a block with rounded corners over a block with plain corners:
    ///
    /// ```text
    /// ┌───┐    ┌───┐  ┌───┬───╮┌───┐
    /// │   │    │   │  │   │   ││   │
    /// │   │    │ ╭─┼─╮│   │   ││   │
    /// │   │    │ │ │ ││   │   ││   │
    /// └───┼───╮└─┼─┘ │└───┴───╯├───┤
    ///     │   │  │   │         │   │
    ///     │   │  ╰───╯         │   │
    ///     │   │                │   │
    ///     ╰───╯                ╰───╯
    /// ```
    ///
    /// 3. There are no symbols that combine thick and double borders, so we replace all double
    ///    segments with thick or all thick with double. The second symbol parameter takes
    ///    precedence in choosing whether to use double or thick. The following diagram shows how
    ///    this would apply to merging a block with double borders over a block with thick borders
    ///    and then the reverse (merging a block with thick borders over a block with double
    ///    borders):
    ///
    /// ```text
    /// ┏━━━┓    ┏━━━┓  ┏━━━╦═══╗┏━━━┓
    /// ┃   ┃    ┃   ┃  ┃   ║   ║┃   ┃
    /// ┃   ┃    ┃ ╔═╬═╗┃   ║   ║┃   ┃
    /// ┃   ┃    ┃ ║ ┃ ║┃   ║   ║┃   ┃
    /// ┗━━━╬═══╗┗━╬━┛ ║┗━━━╩═══╝╠═══╣
    ///     ║   ║  ║   ║         ║   ║
    ///     ║   ║  ╚═══╝         ║   ║
    ///     ║   ║                ║   ║
    ///     ╚═══╝                ╚═══╝
    ///
    /// ╔═══╗    ╔═══╗  ╔═══┳━━━┓╔═══╗
    /// ║   ║    ║   ║  ║   ┃   ┃║   ║
    /// ║   ║    ║ ┏━╋━┓║   ┃   ┃║   ║
    /// ║   ║    ║ ┃ ║ ┃║   ┃   ┃║   ║
    /// ╚═══╋━━━┓╚═╋═╝ ┃╚═══┻━━━┛┣━━━┫
    ///     ┃   ┃  ┃   ┃         ┃   ┃
    ///     ┃   ┃  ┗━━━┛         ┃   ┃
    ///     ┃   ┃                ┃   ┃
    ///     ┗━━━┛                ┗━━━┛
    /// ```
    ///
    /// 4. Some combinations of double and plain don't exist, so if the symbol is still
    ///    unrepresentable, change all plain segments with double or all double with plain. The
    ///    second symbol parameter takes precedence in choosing whether to use double or plain. The
    ///    following diagram shows how this would apply to merging a block with double borders over
    ///    a block with plain borders and then the reverse (merging a block with plain borders over
    ///    a block with double borders):
    ///
    /// ```text
    /// ┌───┐    ┌───┐  ┌───╦═══╗┌───┐
    /// │   │    │   │  │   ║   ║│   │
    /// │   │    │ ╔═╪═╗│   ║   ║│   │
    /// │   │    │ ║ │ ║│   ║   ║│   │
    /// └───╬═══╗└─╫─┘ ║└───╩═══╝╠═══╣
    ///     ║   ║  ║   ║         ║   ║
    ///     ║   ║  ╚═══╝         ║   ║
    ///     ║   ║                ║   ║
    ///     ╚═══╝                ╚═══╝
    /// ╔═══╗    ╔═══╗  ╔═══┬───┐╔═══╗
    /// ║   ║    ║   ║  ║   │   │║   ║
    /// ║   ║    ║ ┌─╫─┐║   │   │║   ║
    /// ║   ║    ║ │ ║ │║   │   │║   ║
    /// ╚═══┼───┐╚═╪═╝ │╚═══┴───┘├───┤
    ///     │   │  │   │         │   │
    ///     │   │  └───┘         │   │
    ///     │   │                │   │
    ///     └───┘                └───┘
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use ratatui_core::symbols::merge::MergeStrategy;
    /// let strategy = MergeStrategy::Fuzzy;
    ///
    /// // exact matches are merged normally
    /// assert_eq!(strategy.merge("┌", "┐"), "┬");
    ///
    /// // dashed segments are replaced with plain
    /// assert_eq!(strategy.merge("╎", "╍"), "┿");
    ///
    /// // rounded segments are replaced with plain
    /// assert_eq!(strategy.merge("┘", "╭"), "┼");
    ///
    /// // double and thick segments are merged based on the second symbol
    /// assert_eq!(strategy.merge("┃", "═"), "╬");
    /// assert_eq!(strategy.merge("═", "┃"), "╋");
    ///
    /// // combinations of double with plain that don't exist are merged based on the second symbol
    /// assert_eq!(strategy.merge("┐", "╔"), "╦");
    /// assert_eq!(strategy.merge("╔", "┐"), "┬");
    /// ```
    Fuzzy,
}

impl MergeStrategy {
    /// Merges two symbols using this merge strategy.
    ///
    /// This method takes two string slices representing the previous and next symbols, and
    /// returns a string slice representing the merged symbol based on the merge strategy.
    ///
    /// Characters from the [Box Drawing Unicode block] and block elements made of quadrants are
    /// merged according to the rules defined in the [`MergeStrategy`]. Other symbols are never
    /// merged. If `next` is one of them, it is returned as is. If only `prev` is one of them,
    /// [`Self::Exact`] and [`Self::Fuzzy`] return `prev`, so a border drawn over text, such as a
    /// block title, keeps the text.
    ///
    /// Most code using this method will use the [`Cell::merge_symbol`] method, which uses this
    /// method internally to merge the symbols of a cell.
    ///
    /// # Example
    ///
    /// ```
    /// # use ratatui_core::symbols::merge::MergeStrategy;
    ///
    /// let strategy = MergeStrategy::Fuzzy;
    /// assert_eq!(strategy.merge("┌", "┐"), "┬"); // merges to a single character
    /// assert_eq!(strategy.merge("┘", "╭"), "┼"); // replaces rounded with plain
    /// assert_eq!(strategy.merge("╎", "╍"), "┿"); // replaces dashed with plain
    /// assert_eq!(strategy.merge("┐", "╔"), "╦"); // merges double with plain
    /// assert_eq!(strategy.merge("╔", "┐"), "┬"); // merges plain with double
    /// assert_eq!(strategy.merge("▖", "▝"), "▞"); // fills the quadrants of both symbols
    /// assert_eq!(strategy.merge("▀", "─"), "─"); // replaces a quadrant with a line
    /// assert_eq!(strategy.merge("a", "─"), "a"); // keeps text under a border
    /// ```
    ///
    /// [Box Drawing Unicode block]: https://en.wikipedia.org/wiki/Box_Drawing
    /// [`Cell::merge_symbol`]: crate::buffer::Cell::merge_symbol
    pub fn merge<'a>(self, prev: &'a str, next: &'a str) -> &'a str {
        // Replace should always just return the last symbol.
        if self == Self::Replace {
            return next;
        }

        match (MergeableSymbol::parse(prev), MergeableSymbol::parse(next)) {
            (Some(prev_symbol), Some(next_symbol)) => {
                prev_symbol.merge(next_symbol, self).unwrap_or(next)
            }
            // Non-border symbols take precedence in strategies other than Replace.
            (None, Some(_)) => prev,
            (_, None) => next,
        }
    }
}

/// A symbol that a [`MergeStrategy`] can combine with another symbol.
#[derive(Debug, Copy, Clone)]
enum MergeableSymbol {
    /// A box drawing character.
    Line(BorderSymbol),
    /// A block element made of quadrants.
    Quadrant(QuadrantSymbol),
}

impl MergeableSymbol {
    fn parse(symbol: &str) -> Option<Self> {
        QuadrantSymbol::parse(symbol)
            .map(Self::Quadrant)
            .or_else(|| BorderSymbol::from_str(symbol).ok().map(Self::Line))
    }

    /// Merges `next` into this symbol, returning `None` if no character represents the result.
    fn merge(self, next: Self, strategy: MergeStrategy) -> Option<&'static str> {
        match (self, next) {
            (Self::Line(prev), Self::Line(next)) => prev.merge(next, strategy).try_into().ok(),
            (Self::Quadrant(prev), Self::Quadrant(next)) => Some(prev.union(next).as_str()),
            // No character combines a box drawing line with a quadrant.
            (Self::Line(_), Self::Quadrant(_)) | (Self::Quadrant(_), Self::Line(_)) => None,
        }
    }
}

/// A block element that fills one or more quadrants of a cell.
///
/// Bit 0 is the top left quadrant, bit 1 the top right, bit 2 the bottom left and bit 3 the bottom
/// right. Unicode has a character for every non-empty combination of quadrants, so two quadrant
/// symbols always merge into an existing character.
#[derive(Debug, Copy, Clone)]
struct QuadrantSymbol(u8);

impl QuadrantSymbol {
    /// The block elements for the quadrant bits `0b0001` to `0b1111`, in that order.
    const SYMBOLS: [&'static str; 15] = [
        border::QUADRANT_TOP_LEFT,
        border::QUADRANT_TOP_RIGHT,
        border::QUADRANT_TOP_HALF,
        border::QUADRANT_BOTTOM_LEFT,
        border::QUADRANT_LEFT_HALF,
        border::QUADRANT_TOP_RIGHT_BOTTOM_LEFT,
        border::QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_LEFT,
        border::QUADRANT_BOTTOM_RIGHT,
        border::QUADRANT_TOP_LEFT_BOTTOM_RIGHT,
        border::QUADRANT_RIGHT_HALF,
        border::QUADRANT_TOP_LEFT_TOP_RIGHT_BOTTOM_RIGHT,
        border::QUADRANT_BOTTOM_HALF,
        border::QUADRANT_TOP_LEFT_BOTTOM_LEFT_BOTTOM_RIGHT,
        border::QUADRANT_TOP_RIGHT_BOTTOM_LEFT_BOTTOM_RIGHT,
        border::QUADRANT_BLOCK,
    ];

    fn parse(symbol: &str) -> Option<Self> {
        let index = Self::SYMBOLS.iter().position(|&s| s == symbol)?;
        Some(Self(index as u8 + 1))
    }

    /// Returns the symbol that fills the quadrants of both symbols.
    const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    const fn as_str(self) -> &'static str {
        Self::SYMBOLS[self.0 as usize - 1]
    }
}

/// Represents a composite border symbol using individual line components.
///
/// This is an internal type for now specifically used to make the merge logic easier to implement.
/// At some point in the future, we might make a similar type public.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct BorderSymbol {
    right: LineStyle,
    up: LineStyle,
    left: LineStyle,
    down: LineStyle,
}

impl BorderSymbol {
    /// Creates a new [`BorderSymbol`], based on individual line styles.
    #[must_use]
    const fn new(right: LineStyle, up: LineStyle, left: LineStyle, down: LineStyle) -> Self {
        Self {
            right,
            up,
            left,
            down,
        }
    }

    /// Finds the closest representation of the [`BorderSymbol`], that has a corresponding unicode
    /// character.
    #[must_use]
    fn fuzzy(mut self, other: Self) -> Self {
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
        if self.contains(Double) && self.contains(Thick) {
            // Decide whether to use Double or Thick, based on the last merged-in symbol.
            if other.contains(Double) {
                self = self.replace(Thick, Double);
            } else {
                self = self.replace(Double, Thick);
            }
        }

        // Some Plain + Double variants don't exist.
        if <&str>::try_from(self).is_err() {
            // Decide whether to use Double or Plain, based on the last merged-in symbol.
            if other.contains(Double) {
                self = self.replace(Plain, Double);
            } else {
                self = self.replace(Double, Plain);
            }
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
        match (self.up, self.right, self.down, self.left) {
            (up, right, Nothing, Nothing) => up == right,
            (Nothing, right, down, Nothing) => right == down,
            (Nothing, Nothing, down, left) => down == left,
            (up, Nothing, Nothing, left) => up == left,
            _ => false,
        }
    }

    /// Checks if any of the line components making the [`BorderSymbol`] matches the `style`.
    fn contains(self, style: LineStyle) -> bool {
        self.up == style || self.right == style || self.down == style || self.left == style
    }

    /// Replaces all line styles matching `from` by `to`.
    #[must_use]
    fn replace(mut self, from: LineStyle, to: LineStyle) -> Self {
        self.up = if self.up == from { to } else { self.up };
        self.right = if self.right == from { to } else { self.right };
        self.down = if self.down == from { to } else { self.down };
        self.left = if self.left == from { to } else { self.left };
        self
    }

    /// Merges two border symbols into one.
    fn merge(self, other: Self, strategy: MergeStrategy) -> Self {
        let exact_result = Self::new(
            self.right.merge(other.right),
            self.up.merge(other.up),
            self.left.merge(other.left),
            self.down.merge(other.down),
        );
        match strategy {
            MergeStrategy::Replace => other,
            MergeStrategy::Fuzzy => exact_result.fuzzy(other),
            MergeStrategy::Exact => exact_result,
        }
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
enum BorderSymbolError {
    #[error("cannot parse &str `{0}` to BorderSymbol")]
    CannotParse(alloc::string::String),
    #[error("cannot convert BorderSymbol `{0:#?}` to &str: no such symbol exists")]
    Unrepresentable(BorderSymbol),
}

/// A visual style defining the appearance of a single line making up a block border.
///
/// This is an internal type used to represent the different styles of lines that can be used in
/// border symbols.
///
/// At some point in the future, we might make this type (or a similar one) public to allow users to
/// work with line styles directly, but for now, it is used internally only to simplify the merge
/// logic of border symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

        impl FromStr for BorderSymbol {
            type Err = BorderSymbolError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use LineStyle::*;
                use alloc::string::ToString;
                match s {
                    $( $symbol => Ok(Self::new($right, $up, $left, $down)) ),* ,
                    _ => Err(BorderSymbolError::CannotParse(s.to_string())),
                }
            }
        }

        impl TryFrom<BorderSymbol> for &'static str {
            type Error = BorderSymbolError;
            fn try_from(value: BorderSymbol) -> Result<Self, Self::Error> {
                use LineStyle::*;
                match (value.right, value.up, value.left, value.down) {
                    $( ($right, $up, $left, $down) => Ok($symbol) ),* ,
                    _ => Err(BorderSymbolError::Unrepresentable(value)),
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

#[cfg(test)]
mod tests {
    use itertools::iproduct;
    use rstest::rstest;

    use super::*;

    #[test]
    fn replace_merge_strategy() {
        let strategy = MergeStrategy::Replace;
        let symbols = [
            "─", "━", "│", "┃", "┄", "┅", "┆", "┇", "┈", "┉", "┊", "┋", "┌", "┍", "┎", "┏", "┐",
            "┑", "┒", "┓", "└", "┕", "┖", "┗", "┘", "┙", "┚", "┛", "├", "┝", "┞", "┟", "┠", "┡",
            "┢", "┣", "┤", "┥", "┦", "┧", "┨", "┩", "┪", "┫", "┬", "┭", "┮", "┯", "┰", "┱", "┲",
            "┳", "┴", "┵", "┶", "┷", "┸", "┹", "┺", "┻", "┼", "┽", "┾", "┿", "╀", "╁", "╂", "╃",
            "╄", "╅", "╆", "╇", "╈", "╉", "╊", "╋", "╌", "╍", "╎", "╏", "═", "║", "╒", "╓", "╔",
            "╕", "╖", "╗", "╘", "╙", "╚", "╛", "╜", "╝", "╞", "╟", "╠", "╡", "╢", "╣", "╤", "╥",
            "╦", "╧", "╨", "╩", "╪", "╫", "╬", "╭", "╮", "╯", "╰", "╴", "╵", "╶", "╷", "╸", "╹",
            "╺", "╻", "╼", "╽", "╾", "╿", "▘", "▝", "▀", "▖", "▌", "▞", "▛", "▗", "▚", "▐", "▜",
            "▄", "▙", "▟", "█", " ", "a", "b",
        ];

        for a in symbols {
            for b in symbols {
                assert_eq!(strategy.merge(a, b), b);
            }
        }
    }

    #[test]
    fn exact_merge_strategy() {
        let strategy = MergeStrategy::Exact;
        assert_eq!(strategy.merge("┆", "─"), "─");
        assert_eq!(strategy.merge("┏", "┆"), "┆");
        assert_eq!(strategy.merge("╎", "┉"), "┉");
        assert_eq!(strategy.merge("╎", "┉"), "┉");
        assert_eq!(strategy.merge("┋", "┋"), "┋");
        assert_eq!(strategy.merge("╷", "╶"), "┌");
        assert_eq!(strategy.merge("╭", "┌"), "┌");
        assert_eq!(strategy.merge("│", "┕"), "┝");
        assert_eq!(strategy.merge("┏", "│"), "┝");
        assert_eq!(strategy.merge("│", "┏"), "┢");
        assert_eq!(strategy.merge("╽", "┕"), "┢");
        assert_eq!(strategy.merge("│", "─"), "┼");
        assert_eq!(strategy.merge("┘", "┌"), "┼");
        assert_eq!(strategy.merge("┵", "┝"), "┿");
        assert_eq!(strategy.merge("│", "━"), "┿");
        assert_eq!(strategy.merge("┵", "╞"), "╞");
        assert_eq!(strategy.merge(" ", "╠"), " ");
        assert_eq!(strategy.merge("╠", " "), " ");
        assert_eq!(strategy.merge("╎", "╧"), "╧");
        assert_eq!(strategy.merge("╛", "╒"), "╪");
        assert_eq!(strategy.merge("│", "═"), "╪");
        assert_eq!(strategy.merge("╤", "╧"), "╪");
        assert_eq!(strategy.merge("╡", "╞"), "╪");
        assert_eq!(strategy.merge("┌", "╭"), "╭");
        assert_eq!(strategy.merge("┘", "╭"), "╭");
        assert_eq!(strategy.merge("┌", "a"), "a");
        assert_eq!(strategy.merge("a", "╭"), "a");
        assert_eq!(strategy.merge("a", "b"), "b");
    }

    #[test]
    fn fuzzy_merge_strategy() {
        let strategy = MergeStrategy::Fuzzy;
        assert_eq!(strategy.merge("┄", "╴"), "─");
        assert_eq!(strategy.merge("│", "┆"), "┆");
        assert_eq!(strategy.merge(" ", "┉"), " ");
        assert_eq!(strategy.merge("┋", "┋"), "┋");
        assert_eq!(strategy.merge("╷", "╶"), "┌");
        assert_eq!(strategy.merge("╭", "┌"), "┌");
        assert_eq!(strategy.merge("│", "┕"), "┝");
        assert_eq!(strategy.merge("┏", "│"), "┝");
        assert_eq!(strategy.merge("┏", "┆"), "┝");
        assert_eq!(strategy.merge("│", "┏"), "┢");
        assert_eq!(strategy.merge("╽", "┕"), "┢");
        assert_eq!(strategy.merge("│", "─"), "┼");
        assert_eq!(strategy.merge("┆", "─"), "┼");
        assert_eq!(strategy.merge("┘", "┌"), "┼");
        assert_eq!(strategy.merge("┘", "╭"), "┼");
        assert_eq!(strategy.merge("╎", "┉"), "┿");
        assert_eq!(strategy.merge(" ", "╠"), " ");
        assert_eq!(strategy.merge("╠", " "), " ");
        assert_eq!(strategy.merge("┵", "╞"), "╪");
        assert_eq!(strategy.merge("╛", "╒"), "╪");
        assert_eq!(strategy.merge("│", "═"), "╪");
        assert_eq!(strategy.merge("╤", "╧"), "╪");
        assert_eq!(strategy.merge("╡", "╞"), "╪");
        assert_eq!(strategy.merge("╎", "╧"), "╪");
        assert_eq!(strategy.merge("┌", "╭"), "╭");
        assert_eq!(strategy.merge("┌", "a"), "a");
        assert_eq!(strategy.merge("a", "╭"), "a");
        assert_eq!(strategy.merge("a", "b"), "b");
    }

    const QUADRANT_SYMBOLS: [&str; 15] = [
        "▘", "▝", "▀", "▖", "▌", "▞", "▛", "▗", "▚", "▐", "▜", "▄", "▙", "▟", "█",
    ];

    #[rstest]
    #[case::same_quadrant("▘", "▘", "▘")]
    #[case::top_quadrants("▘", "▝", "▀")]
    #[case::bottom_quadrants("▖", "▗", "▄")]
    #[case::left_quadrants("▘", "▖", "▌")]
    #[case::right_quadrants("▝", "▗", "▐")]
    #[case::falling_diagonal("▘", "▗", "▚")]
    #[case::rising_diagonal("▝", "▖", "▞")]
    #[case::left_and_right_halves("▌", "▐", "█")]
    #[case::top_and_bottom_halves("▀", "▄", "█")]
    #[case::top_half_and_left_half("▀", "▌", "▛")]
    #[case::top_half_and_right_half("▀", "▐", "▜")]
    #[case::bottom_half_and_left_half("▄", "▌", "▙")]
    #[case::bottom_half_and_right_half("▄", "▐", "▟")]
    #[case::diagonals("▚", "▞", "█")]
    #[case::three_quadrants_and_missing_quadrant("▛", "▗", "█")]
    #[case::three_quadrants_and_contained_quadrant("▙", "▘", "▙")]
    #[case::full_block("█", "▝", "█")]
    fn merge_quadrants(
        #[values(MergeStrategy::Exact, MergeStrategy::Fuzzy)] strategy: MergeStrategy,
        #[case] prev: &str,
        #[case] next: &str,
        #[case] expected: &str,
    ) {
        assert_eq!(strategy.merge(prev, next), expected);
    }

    #[rstest]
    #[case::line_then_quadrant("─", "▀", "▀")]
    #[case::quadrant_then_line("▀", "─", "─")]
    #[case::text_then_quadrant("a", "▀", "a")]
    #[case::quadrant_then_text("▀", "a", "a")]
    #[case::space_then_quadrant(" ", "▄", " ")]
    #[case::quadrant_then_space("▄", " ", " ")]
    #[case::one_eighth_then_quadrant("▏", "▌", "▏")]
    #[case::quadrant_then_one_eighth("▌", "▏", "▏")]
    fn merge_quadrants_with_other_symbols(
        #[values(MergeStrategy::Exact, MergeStrategy::Fuzzy)] strategy: MergeStrategy,
        #[case] prev: &str,
        #[case] next: &str,
        #[case] expected: &str,
    ) {
        assert_eq!(strategy.merge(prev, next), expected);
    }

    #[rstest]
    fn merge_quadrants_keeps_both_symbols(
        #[values(MergeStrategy::Exact, MergeStrategy::Fuzzy)] strategy: MergeStrategy,
    ) {
        for (prev, next) in iproduct!(QUADRANT_SYMBOLS, QUADRANT_SYMBOLS) {
            let merged = strategy.merge(prev, next);
            assert_eq!(merged, strategy.merge(next, prev), "{prev} {next}");
            assert_eq!(strategy.merge(merged, prev), merged, "{prev} {next}");
            assert_eq!(strategy.merge(merged, next), merged, "{prev} {next}");
        }
    }
}
