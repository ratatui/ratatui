//! `style` contains the primitives used to control how your user interface will look.
//!
//! # Using the `Style` struct
//!
//! This is useful when creating style variables.
//! ## Example
//! ```
//! use ratatui::style::{Color, Modifier, Style};
//!
//! Style::default()
//!    .fg(Color::Black)
//!    .bg(Color::Green)
//!    .add_modifier(Modifier::ITALIC | Modifier::BOLD);
//! ```
//!
//! # Using style shorthands
//!
//! This is best for concise styling.
//! ## Example
//! ```
//! use ratatui::prelude::*;
//!
//! assert_eq!(
//!    "hello".red().on_blue().bold(),
//!     Span::styled("hello", Style::default().fg(Color::Red).bg(Color::Blue).add_modifier(Modifier::BOLD))
//! )
//! ```

use std::{
    fmt::{self, Debug, Display},
    str::FromStr,
};

use bitflags::bitflags;

mod stylize;
pub use stylize::{Styled, Stylize};

/// ANSI Color
///
/// All colors from the [ANSI color table](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors)
/// are supported (though some names are not exactly the same).
///
/// | Color Name     | Color                   | Foreground | Background |
/// |----------------|-------------------------|------------|------------|
/// | `black`        | [`Color::Black`]        | 30         | 40         |
/// | `red`          | [`Color::Red`]          | 31         | 41         |
/// | `green`        | [`Color::Green`]        | 32         | 42         |
/// | `yellow`       | [`Color::Yellow`]       | 33         | 43         |
/// | `blue`         | [`Color::Blue`]         | 34         | 44         |
/// | `magenta`      | [`Color::Magenta`]      | 35         | 45         |
/// | `cyan`         | [`Color::Cyan`]         | 36         | 46         |
/// | `gray`*        | [`Color::Gray`]         | 37         | 47         |
/// | `darkgray`*    | [`Color::DarkGray`]     | 90         | 100        |
/// | `lightred`     | [`Color::LightRed`]     | 91         | 101        |
/// | `lightgreen`   | [`Color::LightGreen`]   | 92         | 102        |
/// | `lightyellow`  | [`Color::LightYellow`]  | 93         | 103        |
/// | `lightblue`    | [`Color::LightBlue`]    | 94         | 104        |
/// | `lightmagenta` | [`Color::LightMagenta`] | 95         | 105        |
/// | `lightcyan`    | [`Color::LightCyan`]    | 96         | 106        |
/// | `white`*       | [`Color::White`]        | 97         | 107        |
///
/// - `gray` is sometimes called `white` - this is not supported as we use `white` for bright white
/// - `gray` is sometimes called `silver` - this is supported
/// - `darkgray` is sometimes called `light black` or `bright black` (both are supported)
/// - `white` is sometimes called `light white` or `bright white` (both are supported)
/// - we support `bright` and `light` prefixes for all colors
/// - we support `-` and `_` and ` ` as separators for all colors
/// - we support both `gray` and `grey` spellings
///
/// # Example
///
/// ```
/// use ratatui::style::Color;
/// use std::str::FromStr;
/// assert_eq!(Color::from_str("red"), Ok(Color::Red));
/// assert_eq!("red".parse(), Ok(Color::Red));
/// assert_eq!("lightred".parse(), Ok(Color::LightRed));
/// assert_eq!("light red".parse(), Ok(Color::LightRed));
/// assert_eq!("light-red".parse(), Ok(Color::LightRed));
/// assert_eq!("light_red".parse(), Ok(Color::LightRed));
/// assert_eq!("lightRed".parse(), Ok(Color::LightRed));
/// assert_eq!("bright red".parse(), Ok(Color::LightRed));
/// assert_eq!("bright-red".parse(), Ok(Color::LightRed));
/// assert_eq!("silver".parse(), Ok(Color::Gray));
/// assert_eq!("dark-grey".parse(), Ok(Color::DarkGray));
/// assert_eq!("dark gray".parse(), Ok(Color::DarkGray));
/// assert_eq!("light-black".parse(), Ok(Color::DarkGray));
/// assert_eq!("white".parse(), Ok(Color::White));
/// assert_eq!("bright white".parse(), Ok(Color::White));
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Color {
    /// Resets the foreground or background color
    #[default]
    Reset,
    /// ANSI Color: Black. Foreground: 30, Background: 40
    Black,
    /// ANSI Color: Red. Foreground: 31, Background: 41
    Red,
    /// ANSI Color: Green. Foreground: 32, Background: 42
    Green,
    /// ANSI Color: Yellow. Foreground: 33, Background: 43
    Yellow,
    /// ANSI Color: Blue. Foreground: 34, Background: 44
    Blue,
    /// ANSI Color: Magenta. Foreground: 35, Background: 45
    Magenta,
    /// ANSI Color: Cyan. Foreground: 36, Background: 46
    Cyan,
    /// ANSI Color: White. Foreground: 37, Background: 47
    ///
    /// Note that this is sometimes called `silver` or `white` but we use `white` for bright white
    Gray,
    /// ANSI Color: Bright Black. Foreground: 90, Background: 100
    ///
    /// Note that this is sometimes called `light black` or `bright black` but we use `dark gray`
    DarkGray,
    /// ANSI Color: Bright Red. Foreground: 91, Background: 101
    LightRed,
    /// ANSI Color: Bright Green. Foreground: 92, Background: 102
    LightGreen,
    /// ANSI Color: Bright Yellow. Foreground: 93, Background: 103
    LightYellow,
    /// ANSI Color: Bright Blue. Foreground: 94, Background: 104
    LightBlue,
    /// ANSI Color: Bright Magenta. Foreground: 95, Background: 105
    LightMagenta,
    /// ANSI Color: Bright Cyan. Foreground: 96, Background: 106
    LightCyan,
    /// ANSI Color: Bright White. Foreground: 97, Background: 107
    /// Sometimes called `bright white` or `light white` in some terminals
    White,
    /// An RGB color
    Rgb(u8, u8, u8),
    /// An 8-bit 256 color. See <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
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
    /// # use ratatui::style::Modifier;
    ///
    /// let m = Modifier::BOLD | Modifier::ITALIC;
    /// ```
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
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

/// Implement the `Debug` trait for `Modifier` manually.
///
/// This will avoid printing the empty modifier as 'Borders(0x0)' and instead print it as 'NONE'.
impl fmt::Debug for Modifier {
    /// Format the modifier as `NONE` if the modifier is empty or as a list of flags separated by
    /// `|` otherwise.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "NONE");
        }
        fmt::Debug::fmt(&self.0, f)
    }
}

/// Style let you control the main characteristics of the displayed elements.
///
/// ```rust
/// # use ratatui::style::{Color, Modifier, Style};
/// Style::default()
///     .fg(Color::Black)
///     .bg(Color::Green)
///     .add_modifier(Modifier::ITALIC | Modifier::BOLD);
/// ```
///
/// It represents an incremental change. If you apply the styles S1, S2, S3 to a cell of the
/// terminal buffer, the style of this cell will be the result of the merge of S1, S2 and S3, not
/// just S3.
///
/// ```rust
/// # use ratatui::style::{Color, Modifier, Style};
/// # use ratatui::buffer::Buffer;
/// # use ratatui::layout::Rect;
/// let styles = [
///     Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD | Modifier::ITALIC),
///     Style::default().bg(Color::Red).add_modifier(Modifier::UNDERLINED),
///     #[cfg(feature = "crossterm")]
///     Style::default().underline_color(Color::Green),
///     Style::default().fg(Color::Yellow).remove_modifier(Modifier::ITALIC),
/// ];
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));
/// for style in &styles {
///   buffer.get_mut(0, 0).set_style(*style);
/// }
/// assert_eq!(
///     Style {
///         fg: Some(Color::Yellow),
///         bg: Some(Color::Red),
///         #[cfg(feature = "crossterm")]
///         underline_color: Some(Color::Green),
///         add_modifier: Modifier::BOLD | Modifier::UNDERLINED,
///         sub_modifier: Modifier::empty(),
///     },
///     buffer.get(0, 0).style(),
/// );
/// ```
///
/// The default implementation returns a `Style` that does not modify anything. If you wish to
/// reset all properties until that point use [`Style::reset`].
///
/// ```
/// # use ratatui::style::{Color, Modifier, Style};
/// # use ratatui::buffer::Buffer;
/// # use ratatui::layout::Rect;
/// let styles = [
///     Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD | Modifier::ITALIC),
///     Style::reset().fg(Color::Yellow),
/// ];
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));
/// for style in &styles {
///   buffer.get_mut(0, 0).set_style(*style);
/// }
/// assert_eq!(
///     Style {
///         fg: Some(Color::Yellow),
///         bg: Some(Color::Reset),
///         #[cfg(feature = "crossterm")]
///         underline_color: Some(Color::Reset),
///         add_modifier: Modifier::empty(),
///         sub_modifier: Modifier::empty(),
///     },
///     buffer.get(0, 0).style(),
/// );
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    #[cfg(feature = "crossterm")]
    pub underline_color: Option<Color>,
    pub add_modifier: Modifier,
    pub sub_modifier: Modifier,
}

impl Default for Style {
    fn default() -> Style {
        Style::new()
    }
}

impl Styled for Style {
    type Item = Style;

    fn style(&self) -> Style {
        *self
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.patch(style)
    }
}
impl Style {
    pub const fn new() -> Style {
        Style {
            fg: None,
            bg: None,
            #[cfg(feature = "crossterm")]
            underline_color: None,
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::empty(),
        }
    }

    /// Returns a `Style` resetting all properties.
    pub const fn reset() -> Style {
        Style {
            fg: Some(Color::Reset),
            bg: Some(Color::Reset),
            #[cfg(feature = "crossterm")]
            underline_color: Some(Color::Reset),
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::all(),
        }
    }

    /// Changes the foreground color.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::style::{Color, Style};
    /// let style = Style::default().fg(Color::Blue);
    /// let diff = Style::default().fg(Color::Red);
    /// assert_eq!(style.patch(diff), Style::default().fg(Color::Red));
    /// ```
    pub const fn fg(mut self, color: Color) -> Style {
        self.fg = Some(color);
        self
    }

    /// Changes the background color.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::style::{Color, Style};
    /// let style = Style::default().bg(Color::Blue);
    /// let diff = Style::default().bg(Color::Red);
    /// assert_eq!(style.patch(diff), Style::default().bg(Color::Red));
    /// ```
    pub const fn bg(mut self, color: Color) -> Style {
        self.bg = Some(color);
        self
    }

    /// Changes the underline color. The text must be underlined with a modifier for this to work.
    ///
    /// This uses a non-standard ANSI escape sequence. It is supported by most terminal emulators,
    /// but is only implemented in the crossterm backend.
    ///
    /// See [Wikipedia](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters) code `58` and `59` for more information.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::style::{Color, Modifier, Style};
    /// let style = Style::default().underline_color(Color::Blue).add_modifier(Modifier::UNDERLINED);
    /// let diff = Style::default().underline_color(Color::Red).add_modifier(Modifier::UNDERLINED);
    /// assert_eq!(style.patch(diff), Style::default().underline_color(Color::Red).add_modifier(Modifier::UNDERLINED));
    /// ```
    #[cfg(feature = "crossterm")]
    pub const fn underline_color(mut self, color: Color) -> Style {
        self.underline_color = Some(color);
        self
    }

    /// Changes the text emphasis.
    ///
    /// When applied, it adds the given modifier to the `Style` modifiers.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::style::{Color, Modifier, Style};
    /// let style = Style::default().add_modifier(Modifier::BOLD);
    /// let diff = Style::default().add_modifier(Modifier::ITALIC);
    /// let patched = style.patch(diff);
    /// assert_eq!(patched.add_modifier, Modifier::BOLD | Modifier::ITALIC);
    /// assert_eq!(patched.sub_modifier, Modifier::empty());
    /// ```
    pub const fn add_modifier(mut self, modifier: Modifier) -> Style {
        self.sub_modifier = self.sub_modifier.difference(modifier);
        self.add_modifier = self.add_modifier.union(modifier);
        self
    }

    /// Changes the text emphasis.
    ///
    /// When applied, it removes the given modifier from the `Style` modifiers.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use ratatui::style::{Color, Modifier, Style};
    /// let style = Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC);
    /// let diff = Style::default().remove_modifier(Modifier::ITALIC);
    /// let patched = style.patch(diff);
    /// assert_eq!(patched.add_modifier, Modifier::BOLD);
    /// assert_eq!(patched.sub_modifier, Modifier::ITALIC);
    /// ```
    pub const fn remove_modifier(mut self, modifier: Modifier) -> Style {
        self.add_modifier = self.add_modifier.difference(modifier);
        self.sub_modifier = self.sub_modifier.union(modifier);
        self
    }

    /// Results in a combined style that is equivalent to applying the two individual styles to
    /// a style one after the other.
    ///
    /// ## Examples
    /// ```
    /// # use ratatui::style::{Color, Modifier, Style};
    /// let style_1 = Style::default().fg(Color::Yellow);
    /// let style_2 = Style::default().bg(Color::Red);
    /// let combined = style_1.patch(style_2);
    /// assert_eq!(
    ///     Style::default().patch(style_1).patch(style_2),
    ///     Style::default().patch(combined));
    /// ```
    pub fn patch(mut self, other: Style) -> Style {
        self.fg = other.fg.or(self.fg);
        self.bg = other.bg.or(self.bg);

        #[cfg(feature = "crossterm")]
        {
            self.underline_color = other.underline_color.or(self.underline_color);
        }

        self.add_modifier.remove(other.sub_modifier);
        self.add_modifier.insert(other.add_modifier);
        self.sub_modifier.remove(other.add_modifier);
        self.sub_modifier.insert(other.sub_modifier);

        self
    }
}

/// Error type indicating a failure to parse a color string.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ParseColorError;

impl std::fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse Colors")
    }
}

impl std::error::Error for ParseColorError {}

/// Converts a string representation to a `Color` instance.
///
/// The `from_str` function attempts to parse the given string and convert it to the corresponding
/// `Color` variant. It supports named colors, RGB values, and indexed colors. If the string cannot
/// be parsed, a `ParseColorError` is returned.
///
/// See the [`Color`](Color) documentation for more information on the supported color names.
///
/// # Examples
///
/// ```
/// # use std::str::FromStr;
/// # use ratatui::style::Color;
/// let color: Color = Color::from_str("blue").unwrap();
/// assert_eq!(color, Color::Blue);
///
/// let color: Color = Color::from_str("#FF0000").unwrap();
/// assert_eq!(color, Color::Rgb(255, 0, 0));
///
/// let color: Color = Color::from_str("10").unwrap();
/// assert_eq!(color, Color::Indexed(10));
///
/// let color: Result<Color, _> = Color::from_str("invalid_color");
/// assert!(color.is_err());
/// ```
impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            // There is a mix of different color names and formats in the wild.
            // This is an attempt to support as many as possible.
            match s
                .to_lowercase()
                .replace([' ', '-', '_'], "")
                .replace("bright", "light")
                .replace("grey", "gray")
                .replace("silver", "gray")
                .replace("lightblack", "darkgray")
                .replace("lightwhite", "white")
                .replace("lightgray", "white")
                .as_ref()
            {
                "reset" => Self::Reset,
                "black" => Self::Black,
                "red" => Self::Red,
                "green" => Self::Green,
                "yellow" => Self::Yellow,
                "blue" => Self::Blue,
                "magenta" => Self::Magenta,
                "cyan" => Self::Cyan,
                "gray" => Self::Gray,
                "darkgray" => Self::DarkGray,
                "lightred" => Self::LightRed,
                "lightgreen" => Self::LightGreen,
                "lightyellow" => Self::LightYellow,
                "lightblue" => Self::LightBlue,
                "lightmagenta" => Self::LightMagenta,
                "lightcyan" => Self::LightCyan,
                "white" => Self::White,
                _ => {
                    if let Ok(index) = s.parse::<u8>() {
                        Self::Indexed(index)
                    } else if let (Ok(r), Ok(g), Ok(b)) = {
                        if !s.starts_with('#') || s.len() != 7 {
                            return Err(ParseColorError);
                        }
                        (
                            u8::from_str_radix(&s[1..3], 16),
                            u8::from_str_radix(&s[3..5], 16),
                            u8::from_str_radix(&s[5..7], 16),
                        )
                    } {
                        Self::Rgb(r, g, b)
                    } else {
                        return Err(ParseColorError);
                    }
                }
            },
        )
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Reset => write!(f, "Reset"),
            Color::Black => write!(f, "Black"),
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Yellow => write!(f, "Yellow"),
            Color::Blue => write!(f, "Blue"),
            Color::Magenta => write!(f, "Magenta"),
            Color::Cyan => write!(f, "Cyan"),
            Color::Gray => write!(f, "Gray"),
            Color::DarkGray => write!(f, "DarkGray"),
            Color::LightRed => write!(f, "LightRed"),
            Color::LightGreen => write!(f, "LightGreen"),
            Color::LightYellow => write!(f, "LightYellow"),
            Color::LightBlue => write!(f, "LightBlue"),
            Color::LightMagenta => write!(f, "LightMagenta"),
            Color::LightCyan => write!(f, "LightCyan"),
            Color::White => write!(f, "White"),
            Color::Rgb(r, g, b) => write!(f, "#{:02X}{:02X}{:02X}", r, g, b),
            Color::Indexed(i) => write!(f, "{}", i),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    fn styles() -> Vec<Style> {
        vec![
            Style::default(),
            Style::default().fg(Color::Yellow),
            Style::default().bg(Color::Yellow),
            Style::default().add_modifier(Modifier::BOLD),
            Style::default().remove_modifier(Modifier::BOLD),
            Style::default().add_modifier(Modifier::ITALIC),
            Style::default().remove_modifier(Modifier::ITALIC),
            Style::default().add_modifier(Modifier::ITALIC | Modifier::BOLD),
            Style::default().remove_modifier(Modifier::ITALIC | Modifier::BOLD),
        ]
    }

    #[test]
    fn combined_patch_gives_same_result_as_individual_patch() {
        let styles = styles();
        for &a in &styles {
            for &b in &styles {
                for &c in &styles {
                    for &d in &styles {
                        let combined = a.patch(b.patch(c.patch(d)));

                        assert_eq!(
                            Style::default().patch(a).patch(b).patch(c).patch(d),
                            Style::default().patch(combined)
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn combine_individual_modifiers() {
        use crate::{buffer::Buffer, layout::Rect};

        let mods = vec![
            Modifier::BOLD,
            Modifier::DIM,
            Modifier::ITALIC,
            Modifier::UNDERLINED,
            Modifier::SLOW_BLINK,
            Modifier::RAPID_BLINK,
            Modifier::REVERSED,
            Modifier::HIDDEN,
            Modifier::CROSSED_OUT,
        ];

        let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));

        for m in &mods {
            buffer.get_mut(0, 0).set_style(Style::reset());
            buffer
                .get_mut(0, 0)
                .set_style(Style::default().add_modifier(*m));
            let style = buffer.get(0, 0).style();
            assert!(style.add_modifier.contains(*m));
            assert!(!style.sub_modifier.contains(*m));
        }
    }

    #[test]
    fn modifier_debug() {
        assert_eq!(format!("{:?}", Modifier::empty()), "NONE");
        assert_eq!(format!("{:?}", Modifier::BOLD), "BOLD");
        assert_eq!(format!("{:?}", Modifier::DIM), "DIM");
        assert_eq!(format!("{:?}", Modifier::ITALIC), "ITALIC");
        assert_eq!(format!("{:?}", Modifier::UNDERLINED), "UNDERLINED");
        assert_eq!(format!("{:?}", Modifier::SLOW_BLINK), "SLOW_BLINK");
        assert_eq!(format!("{:?}", Modifier::RAPID_BLINK), "RAPID_BLINK");
        assert_eq!(format!("{:?}", Modifier::REVERSED), "REVERSED");
        assert_eq!(format!("{:?}", Modifier::HIDDEN), "HIDDEN");
        assert_eq!(format!("{:?}", Modifier::CROSSED_OUT), "CROSSED_OUT");
        assert_eq!(
            format!("{:?}", Modifier::BOLD | Modifier::DIM),
            "BOLD | DIM"
        );
        assert_eq!(
            format!("{:?}", Modifier::all()),
            "BOLD | DIM | ITALIC | UNDERLINED | SLOW_BLINK | RAPID_BLINK | REVERSED | HIDDEN | CROSSED_OUT"
        );
    }

    #[test]
    fn from_rgb_color() {
        let color: Color = Color::from_str("#FF0000").unwrap();
        assert_eq!(color, Color::Rgb(255, 0, 0));
    }

    #[test]
    fn from_indexed_color() {
        let color: Color = Color::from_str("10").unwrap();
        assert_eq!(color, Color::Indexed(10));
    }

    #[test]
    fn from_ansi_color() -> Result<(), Box<dyn Error>> {
        assert_eq!(Color::from_str("reset")?, Color::Reset);
        assert_eq!(Color::from_str("black")?, Color::Black);
        assert_eq!(Color::from_str("red")?, Color::Red);
        assert_eq!(Color::from_str("green")?, Color::Green);
        assert_eq!(Color::from_str("yellow")?, Color::Yellow);
        assert_eq!(Color::from_str("blue")?, Color::Blue);
        assert_eq!(Color::from_str("magenta")?, Color::Magenta);
        assert_eq!(Color::from_str("cyan")?, Color::Cyan);
        assert_eq!(Color::from_str("gray")?, Color::Gray);
        assert_eq!(Color::from_str("darkgray")?, Color::DarkGray);
        assert_eq!(Color::from_str("lightred")?, Color::LightRed);
        assert_eq!(Color::from_str("lightgreen")?, Color::LightGreen);
        assert_eq!(Color::from_str("lightyellow")?, Color::LightYellow);
        assert_eq!(Color::from_str("lightblue")?, Color::LightBlue);
        assert_eq!(Color::from_str("lightmagenta")?, Color::LightMagenta);
        assert_eq!(Color::from_str("lightcyan")?, Color::LightCyan);
        assert_eq!(Color::from_str("white")?, Color::White);

        // aliases
        assert_eq!(Color::from_str("lightblack")?, Color::DarkGray);
        assert_eq!(Color::from_str("lightwhite")?, Color::White);
        assert_eq!(Color::from_str("lightgray")?, Color::White);

        // silver = grey = gray
        assert_eq!(Color::from_str("grey")?, Color::Gray);
        assert_eq!(Color::from_str("silver")?, Color::Gray);

        // spaces are ignored
        assert_eq!(Color::from_str("light black")?, Color::DarkGray);
        assert_eq!(Color::from_str("light white")?, Color::White);
        assert_eq!(Color::from_str("light gray")?, Color::White);

        // dashes are ignored
        assert_eq!(Color::from_str("light-black")?, Color::DarkGray);
        assert_eq!(Color::from_str("light-white")?, Color::White);
        assert_eq!(Color::from_str("light-gray")?, Color::White);

        // underscores are ignored
        assert_eq!(Color::from_str("light_black")?, Color::DarkGray);
        assert_eq!(Color::from_str("light_white")?, Color::White);
        assert_eq!(Color::from_str("light_gray")?, Color::White);

        // bright = light
        assert_eq!(Color::from_str("bright-black")?, Color::DarkGray);
        assert_eq!(Color::from_str("bright-white")?, Color::White);

        // bright = light
        assert_eq!(Color::from_str("brightblack")?, Color::DarkGray);
        assert_eq!(Color::from_str("brightwhite")?, Color::White);

        Ok(())
    }

    #[test]
    fn from_invalid_colors() {
        let bad_colors = [
            "invalid_color", // not a color string
            "abcdef0",       // 7 chars is not a color
            " bcdefa",       // doesn't start with a '#'
            "#abcdef00",     // too many chars
            "resett",        // typo
            "lightblackk",   // typo
        ];

        for bad_color in bad_colors {
            assert!(
                Color::from_str(bad_color).is_err(),
                "bad color: '{bad_color}'"
            );
        }
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Color::Black), "Black");
        assert_eq!(format!("{}", Color::Red), "Red");
        assert_eq!(format!("{}", Color::Green), "Green");
        assert_eq!(format!("{}", Color::Yellow), "Yellow");
        assert_eq!(format!("{}", Color::Blue), "Blue");
        assert_eq!(format!("{}", Color::Magenta), "Magenta");
        assert_eq!(format!("{}", Color::Cyan), "Cyan");
        assert_eq!(format!("{}", Color::Gray), "Gray");
        assert_eq!(format!("{}", Color::DarkGray), "DarkGray");
        assert_eq!(format!("{}", Color::LightRed), "LightRed");
        assert_eq!(format!("{}", Color::LightGreen), "LightGreen");
        assert_eq!(format!("{}", Color::LightYellow), "LightYellow");
        assert_eq!(format!("{}", Color::LightBlue), "LightBlue");
        assert_eq!(format!("{}", Color::LightMagenta), "LightMagenta");
        assert_eq!(format!("{}", Color::LightCyan), "LightCyan");
        assert_eq!(format!("{}", Color::White), "White");
        assert_eq!(format!("{}", Color::Indexed(10)), "10");
        assert_eq!(format!("{}", Color::Rgb(255, 0, 0)), "#FF0000");
        assert_eq!(format!("{}", Color::Reset), "Reset");
    }

    #[test]
    fn style_can_be_const() {
        const RED: Color = Color::Red;
        const BLACK: Color = Color::Black;
        const BOLD: Modifier = Modifier::BOLD;
        const ITALIC: Modifier = Modifier::ITALIC;

        const _RESET: Style = Style::reset();
        const _RED_FG: Style = Style::new().fg(RED);
        const _BLACK_BG: Style = Style::new().bg(BLACK);
        const _ADD_BOLD: Style = Style::new().add_modifier(BOLD);
        const _REMOVE_ITALIC: Style = Style::new().remove_modifier(ITALIC);
        const ALL: Style = Style::new()
            .fg(RED)
            .bg(BLACK)
            .add_modifier(BOLD)
            .remove_modifier(ITALIC);
        assert_eq!(
            ALL,
            Style::new()
                .fg(Color::Red)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::ITALIC)
        )
    }

    #[test]
    fn style_can_be_stylized() {
        // foreground colors
        assert_eq!(Style::new().black(), Style::new().fg(Color::Black));
        assert_eq!(Style::new().red(), Style::new().fg(Color::Red));
        assert_eq!(Style::new().green(), Style::new().fg(Color::Green));
        assert_eq!(Style::new().yellow(), Style::new().fg(Color::Yellow));
        assert_eq!(Style::new().blue(), Style::new().fg(Color::Blue));
        assert_eq!(Style::new().magenta(), Style::new().fg(Color::Magenta));
        assert_eq!(Style::new().cyan(), Style::new().fg(Color::Cyan));
        assert_eq!(Style::new().white(), Style::new().fg(Color::White));
        assert_eq!(Style::new().gray(), Style::new().fg(Color::Gray));
        assert_eq!(Style::new().dark_gray(), Style::new().fg(Color::DarkGray));
        assert_eq!(Style::new().light_red(), Style::new().fg(Color::LightRed));
        assert_eq!(
            Style::new().light_green(),
            Style::new().fg(Color::LightGreen)
        );
        assert_eq!(
            Style::new().light_yellow(),
            Style::new().fg(Color::LightYellow)
        );
        assert_eq!(Style::new().light_blue(), Style::new().fg(Color::LightBlue));
        assert_eq!(
            Style::new().light_magenta(),
            Style::new().fg(Color::LightMagenta)
        );
        assert_eq!(Style::new().light_cyan(), Style::new().fg(Color::LightCyan));
        assert_eq!(Style::new().white(), Style::new().fg(Color::White));

        // Background colors
        assert_eq!(Style::new().on_black(), Style::new().bg(Color::Black));
        assert_eq!(Style::new().on_red(), Style::new().bg(Color::Red));
        assert_eq!(Style::new().on_green(), Style::new().bg(Color::Green));
        assert_eq!(Style::new().on_yellow(), Style::new().bg(Color::Yellow));
        assert_eq!(Style::new().on_blue(), Style::new().bg(Color::Blue));
        assert_eq!(Style::new().on_magenta(), Style::new().bg(Color::Magenta));
        assert_eq!(Style::new().on_cyan(), Style::new().bg(Color::Cyan));
        assert_eq!(Style::new().on_white(), Style::new().bg(Color::White));
        assert_eq!(Style::new().on_gray(), Style::new().bg(Color::Gray));
        assert_eq!(
            Style::new().on_dark_gray(),
            Style::new().bg(Color::DarkGray)
        );
        assert_eq!(
            Style::new().on_light_red(),
            Style::new().bg(Color::LightRed)
        );
        assert_eq!(
            Style::new().on_light_green(),
            Style::new().bg(Color::LightGreen)
        );
        assert_eq!(
            Style::new().on_light_yellow(),
            Style::new().bg(Color::LightYellow)
        );
        assert_eq!(
            Style::new().on_light_blue(),
            Style::new().bg(Color::LightBlue)
        );
        assert_eq!(
            Style::new().on_light_magenta(),
            Style::new().bg(Color::LightMagenta)
        );
        assert_eq!(
            Style::new().on_light_cyan(),
            Style::new().bg(Color::LightCyan)
        );
        assert_eq!(Style::new().on_white(), Style::new().bg(Color::White));

        // Add Modifiers
        assert_eq!(
            Style::new().bold(),
            Style::new().add_modifier(Modifier::BOLD)
        );
        assert_eq!(Style::new().dim(), Style::new().add_modifier(Modifier::DIM));
        assert_eq!(
            Style::new().italic(),
            Style::new().add_modifier(Modifier::ITALIC)
        );
        assert_eq!(
            Style::new().underlined(),
            Style::new().add_modifier(Modifier::UNDERLINED)
        );
        assert_eq!(
            Style::new().slow_blink(),
            Style::new().add_modifier(Modifier::SLOW_BLINK)
        );
        assert_eq!(
            Style::new().rapid_blink(),
            Style::new().add_modifier(Modifier::RAPID_BLINK)
        );
        assert_eq!(
            Style::new().reversed(),
            Style::new().add_modifier(Modifier::REVERSED)
        );
        assert_eq!(
            Style::new().hidden(),
            Style::new().add_modifier(Modifier::HIDDEN)
        );
        assert_eq!(
            Style::new().crossed_out(),
            Style::new().add_modifier(Modifier::CROSSED_OUT)
        );

        // Remove Modifiers
        assert_eq!(
            Style::new().not_bold(),
            Style::new().remove_modifier(Modifier::BOLD)
        );
        assert_eq!(
            Style::new().not_dim(),
            Style::new().remove_modifier(Modifier::DIM)
        );
        assert_eq!(
            Style::new().not_italic(),
            Style::new().remove_modifier(Modifier::ITALIC)
        );
        assert_eq!(
            Style::new().not_underlined(),
            Style::new().remove_modifier(Modifier::UNDERLINED)
        );
        assert_eq!(
            Style::new().not_slow_blink(),
            Style::new().remove_modifier(Modifier::SLOW_BLINK)
        );
        assert_eq!(
            Style::new().not_rapid_blink(),
            Style::new().remove_modifier(Modifier::RAPID_BLINK)
        );
        assert_eq!(
            Style::new().not_reversed(),
            Style::new().remove_modifier(Modifier::REVERSED)
        );
        assert_eq!(
            Style::new().not_hidden(),
            Style::new().remove_modifier(Modifier::HIDDEN)
        );
        assert_eq!(
            Style::new().not_crossed_out(),
            Style::new().remove_modifier(Modifier::CROSSED_OUT)
        );

        // reset
        assert_eq!(Style::new().reset(), Style::reset());
    }
}
