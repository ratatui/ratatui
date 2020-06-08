//! `style` contains the primitives used to control how your user interface will look.

use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

bitflags! {
    /// Modifier changes the way a piece of text is displayed.
    ///
    /// They are bitflags so they can easily be composed.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use tui::style::Modifier;
    ///
    /// let m = Modifier::BOLD | Modifier::ITALIC;
    /// ```
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Modifier: u16 {
        const BOLD              = 0b0000_0000_0001;
        const DIM               = 0b0000_0000_0010;
        const ITALIC            = 0b0000_0000_0100;
        const UNDERLINED        = 0b0000_0000_1000;
        const SLOW_BLINK        = 0b0000_0001_0000;
        const RAPID_BLINK       = 0b0000_0010_0000;
        const REVERSED          = 0b0000_0100_0000;
        const HIDDEN            = 0b0000_1000_0000;
        const CROSSED_OUT       = 0b0001_0000_0000;
    }
}

/// Style let you control the main characteristics of the displayed elements.
///
/// ## Examples
///
/// ```rust
/// # use tui::style::{Color, Modifier, Style};
/// // Using the raw struct initialization:
/// let s = Style {
///     fg: Color::Black,
///     bg: Color::Green,
///     modifier: Modifier::ITALIC | Modifier::BOLD
/// };
/// // Using the provided builder pattern:
/// let s = Style::default()
///     .fg(Color::Black)
///     .bg(Color::Green)
///     .modifier(Modifier::ITALIC | Modifier::BOLD);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Style {
    /// The foreground color.
    pub fg: Color,
    /// The background color.
    pub bg: Color,
    /// The emphasis applied to the text graphemes.
    pub modifier: Modifier,
}

impl Default for Style {
    fn default() -> Style {
        Style::new()
    }
}

impl Style {
    pub const fn new() -> Self {
        Style {
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
        }
    }

    /// Reinitializes the style properties. Both colors are put back to `Color::Reset` while
    /// all modifiers are cleared.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use tui::style::{Color, Modifier, Style};
    /// let mut s = Style::default().fg(Color::Red).bg(Color::Green).modifier(Modifier::BOLD);
    /// s.reset();
    /// assert_eq!(s.fg, Color::Reset);
    /// assert_eq!(s.bg, Color::Reset);
    /// assert_eq!(s.modifier, Modifier::empty());
    /// ```
    pub fn reset(&mut self) {
        self.fg = Color::Reset;
        self.bg = Color::Reset;
        self.modifier = Modifier::empty();
    }

    /// Changes the foreground color.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use tui::style::{Color, Style};
    /// let s = Style::default().fg(Color::Red);
    /// assert_eq!(s.fg, Color::Red);
    /// ```
    pub const fn fg(mut self, color: Color) -> Style {
        self.fg = color;
        self
    }

    /// Changes the background color.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use tui::style::{Color, Style};
    /// let s = Style::default().bg(Color::Red);
    /// assert_eq!(s.bg, Color::Red);
    /// ```
    pub const fn bg(mut self, color: Color) -> Style {
        self.bg = color;
        self
    }

    /// Changes the emphasis.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use tui::style::{Modifier, Style};
    /// let s = Style::default().modifier(Modifier::BOLD | Modifier::ITALIC);
    /// assert_eq!(s.modifier, Modifier::BOLD | Modifier::ITALIC);
    /// ```
    pub const fn modifier(mut self, modifier: Modifier) -> Style {
        self.modifier = modifier;
        self
    }
}
