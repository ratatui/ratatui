//! Border related types ([`Borders`], [`BorderType`]) and a macro to create borders ([`border`]).
use alloc::fmt;

use bitflags::bitflags;
use ratatui_core::symbols::border;
use strum::{Display, EnumString};

bitflags! {
    /// Bitflags that can be composed to set the visible borders essentially on the block widget.
    #[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
    pub struct Borders: u8 {
        /// Show no border (default)
        const NONE   = 0b0000;
        /// Show the top border
        const TOP    = 0b0001;
        /// Show the right border
        const RIGHT  = 0b0010;
        /// Show the bottom border
        const BOTTOM = 0b0100;
        /// Show the left border
        const LEFT   = 0b1000;
        /// Show all borders
        const ALL = Self::TOP.bits() | Self::RIGHT.bits() | Self::BOTTOM.bits() | Self::LEFT.bits();
    }
}

/// The type of border of a [`Block`](crate::block::Block).
///
/// See the [`borders`](crate::block::Block::borders) method of `Block` to configure its borders.
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum BorderType {
    /// A plain, simple border.
    ///
    /// This is the default
    ///
    /// # Example
    ///
    /// ```plain
    /// ┌───────┐
    /// │       │
    /// └───────┘
    /// ```
    #[default]
    Plain,
    /// A plain border with rounded corners.
    ///
    /// # Example
    ///
    /// ```plain
    /// ╭───────╮
    /// │       │
    /// ╰───────╯
    /// ```
    Rounded,
    /// A doubled border.
    ///
    /// Note this uses one character that draws two lines.
    ///
    /// # Example
    ///
    /// ```plain
    /// ╔═══════╗
    /// ║       ║
    /// ╚═══════╝
    /// ```
    Double,
    /// A thick border.
    ///
    /// # Example
    ///
    /// ```plain
    /// ┏━━━━━━━┓
    /// ┃       ┃
    /// ┗━━━━━━━┛
    /// ```
    Thick,
    /// A light double-dashed border.
    ///
    /// ```plain
    /// ┌╌╌╌╌╌╌╌┐
    /// ╎       ╎
    /// └╌╌╌╌╌╌╌┘
    /// ```
    LightDoubleDashed,
    /// A heavy double-dashed border.
    ///
    /// ```plain
    /// ┏╍╍╍╍╍╍╍┓
    /// ╏       ╏
    /// ┗╍╍╍╍╍╍╍┛
    /// ```
    HeavyDoubleDashed,
    /// A light triple-dashed border.
    ///
    /// ```plain
    /// ┌┄┄┄┄┄┄┄┐
    /// ┆       ┆
    /// └┄┄┄┄┄┄┄┘
    /// ```
    LightTripleDashed,
    /// A heavy triple-dashed border.
    ///
    /// ```plain
    /// ┏┅┅┅┅┅┅┅┓
    /// ┇       ┇
    /// ┗┅┅┅┅┅┅┅┛
    /// ```
    HeavyTripleDashed,
    /// A light quadruple-dashed border.
    ///
    /// ```plain
    /// ┌┈┈┈┈┈┈┈┐
    /// ┊       ┊
    /// └┈┈┈┈┈┈┈┘
    /// ```
    LightQuadrupleDashed,
    /// A heavy quadruple-dashed border.
    ///
    /// ```plain
    /// ┏┉┉┉┉┉┉┉┓
    /// ┋       ┋
    /// ┗┉┉┉┉┉┉┉┛
    /// ```
    HeavyQuadrupleDashed,
    /// A border with a single line on the inside of a half block.
    ///
    /// # Example
    ///
    /// ```plain
    /// ▗▄▄▄▄▄▄▄▖
    /// ▐       ▌
    /// ▐       ▌
    /// ▝▀▀▀▀▀▀▀▘
    QuadrantInside,

    /// A border with a single line on the outside of a half block.
    ///
    /// # Example
    ///
    /// ```plain
    /// ▛▀▀▀▀▀▀▀▜
    /// ▌       ▐
    /// ▌       ▐
    /// ▙▄▄▄▄▄▄▄▟
    QuadrantOutside,
}

impl BorderType {
    /// Convert this `BorderType` into the corresponding [`Set`](border::Set) of border symbols.
    pub const fn border_symbols(border_type: Self) -> border::Set {
        match border_type {
            Self::Plain => border::PLAIN,
            Self::Rounded => border::ROUNDED,
            Self::Double => border::DOUBLE,
            Self::Thick => border::THICK,
            Self::LightDoubleDashed => border::LIGHT_DOUBLE_DASHED,
            Self::HeavyDoubleDashed => border::HEAVY_DOUBLE_DASHED,
            Self::LightTripleDashed => border::LIGHT_TRIPLE_DASHED,
            Self::HeavyTripleDashed => border::HEAVY_TRIPLE_DASHED,
            Self::LightQuadrupleDashed => border::LIGHT_QUADRUPLE_DASHED,
            Self::HeavyQuadrupleDashed => border::HEAVY_QUADRUPLE_DASHED,
            Self::QuadrantInside => border::QUADRANT_INSIDE,
            Self::QuadrantOutside => border::QUADRANT_OUTSIDE,
        }
    }

    /// Convert this `BorderType` into the corresponding [`Set`](border::Set) of border symbols.
    pub const fn to_border_set(self) -> border::Set {
        Self::border_symbols(self)
    }
}

/// Implement the `Debug` trait for the `Borders` bitflags. This is a manual implementation to
/// display the flags in a more readable way. The default implementation would display the
/// flags as 'Border(0x0)' for `Borders::NONE` for example.
impl fmt::Debug for Borders {
    /// Display the Borders bitflags as a list of names. For example, `Borders::NONE` will be
    /// displayed as `NONE` and `Borders::ALL` will be displayed as `ALL`. If multiple flags are
    /// set, they will be displayed separated by a pipe character.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "NONE");
        }
        if self.is_all() {
            return write!(f, "ALL");
        }
        let mut first = true;
        for (name, border) in self.iter_names() {
            if border == Self::NONE {
                continue;
            }
            if first {
                write!(f, "{name}")?;
                first = false;
            } else {
                write!(f, " | {name}")?;
            }
        }
        Ok(())
    }
}

/// Macro that constructs and returns a combination of the [`Borders`] object from TOP, BOTTOM, LEFT
/// and RIGHT.
///
/// When used with NONE you should consider omitting this completely. For ALL you should consider
/// [`Block::bordered()`](crate::block::Block::bordered) instead.
///
/// ## Examples
///
/// ```
/// use ratatui::border;
/// use ratatui::widgets::{Block, Borders};
///
/// Block::new()
///     .title("Construct Borders and use them in place")
///     .borders(border!(TOP, BOTTOM));
/// ```
///
/// `border!` can be called with any number of individual sides:
///
/// ```
/// use ratatui::border;
/// use ratatui::widgets::Borders;
/// let right_open = border!(TOP, LEFT, BOTTOM);
/// assert_eq!(right_open, Borders::TOP | Borders::LEFT | Borders::BOTTOM);
/// ```
///
/// Single borders work but using `Borders::` directly would be simpler.
///
/// ```
/// use ratatui::border;
/// use ratatui::widgets::Borders;
///
/// assert_eq!(border!(TOP), Borders::TOP);
/// assert_eq!(border!(ALL), Borders::ALL);
/// assert_eq!(border!(), Borders::NONE);
/// ```
#[macro_export]
macro_rules! border {
    () => {
        Borders::NONE
    };
    ($b:ident) => {
        Borders::$b
    };
    ($first:ident,$($other:ident),*) => {
        Borders::$first
        $(
            .union(Borders::$other)
        )*
    };
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn test_borders_debug() {
        assert_eq!(format!("{:?}", Borders::empty()), "NONE");
        assert_eq!(format!("{:?}", Borders::NONE), "NONE");
        assert_eq!(format!("{:?}", Borders::TOP), "TOP");
        assert_eq!(format!("{:?}", Borders::BOTTOM), "BOTTOM");
        assert_eq!(format!("{:?}", Borders::LEFT), "LEFT");
        assert_eq!(format!("{:?}", Borders::RIGHT), "RIGHT");
        assert_eq!(format!("{:?}", Borders::ALL), "ALL");
        assert_eq!(format!("{:?}", Borders::all()), "ALL");

        assert_eq!(
            format!("{:?}", Borders::TOP | Borders::BOTTOM),
            "TOP | BOTTOM"
        );
    }

    #[test]
    fn can_be_const() {
        const NOTHING: Borders = border!();
        const JUST_TOP: Borders = border!(TOP);
        const TOP_BOTTOM: Borders = border!(TOP, BOTTOM);
        const RIGHT_OPEN: Borders = border!(TOP, LEFT, BOTTOM);

        assert_eq!(NOTHING, Borders::NONE);
        assert_eq!(JUST_TOP, Borders::TOP);
        assert_eq!(TOP_BOTTOM, Borders::TOP | Borders::BOTTOM);
        assert_eq!(RIGHT_OPEN, Borders::TOP | Borders::LEFT | Borders::BOTTOM);
    }

    #[test]
    fn border_empty() {
        let empty = Borders::NONE;
        assert_eq!(empty, border!());
    }

    #[test]
    fn border_all() {
        let all = Borders::ALL;
        assert_eq!(all, border!(ALL));
        assert_eq!(all, border!(TOP, BOTTOM, LEFT, RIGHT));
    }

    #[test]
    fn border_left_right() {
        let left_right = Borders::from_bits(Borders::LEFT.bits() | Borders::RIGHT.bits());
        assert_eq!(left_right, Some(border!(RIGHT, LEFT)));
    }
}
