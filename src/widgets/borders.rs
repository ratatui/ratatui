use std::fmt::{self, Debug};

use bitflags::bitflags;

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

/// Implement the `Debug` trait for the `Borders` bitflags. This is a manual implementation to
/// display the flags in a more readable way. The default implementation would display the
/// flags as 'Border(0x0)' for `Borders::NONE` for example.
impl Debug for Borders {
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
            if border == Borders::NONE {
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

/// Macro that constructs and returns a [`Borders`] object from TOP, BOTTOM, LEFT, RIGHT, NONE, and
/// ALL. Internally it creates an empty `Borders` object and then inserts each bit flag specified
/// into it using `Borders::insert()`.
///
/// ## Examples
///
///```
/// use ratatui::{border, prelude::*, widgets::*};
///
/// Block::default()
///     //Construct a `Borders` object and use it in place
///     .borders(border!(TOP, BOTTOM));
///
/// //`border!` can be called with any order of individual sides
/// let bottom_first = border!(BOTTOM, LEFT, TOP);
/// //with the ALL keyword which works as expected
/// let all = border!(ALL);
/// //or with nothing to return a `Borders::NONE' bitflag.
/// let none = border!(NONE);
/// ```
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! border {
( $($b:tt), +) => {{
        let mut border = Borders::empty();
        $(
           border.insert(Borders::$b);
        )*
        border
}};
() =>{
    Borders::NONE
}
}

#[cfg(test)]
mod tests {
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
}
