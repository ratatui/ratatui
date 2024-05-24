#![allow(clippy::unreadable_literal)]

use std::{fmt, str::FromStr};

/// ANSI Color
///
/// All colors from the [ANSI color table] are supported (though some names are not exactly the
/// same).
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
/// `From<Color> for Style` is implemented by creating a style with the foreground color set to the
/// given color. This allows you to use colors anywhere that accepts `Into<Style>`.
///
/// # Example
///
/// ```
/// use std::str::FromStr;
///
/// use ratatui::prelude::*;
///
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
///
/// [ANSI color table]: https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
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
    /// An RGB color.
    ///
    /// Note that only terminals that support 24-bit true color will display this correctly.
    /// Notably versions of Windows Terminal prior to Windows 10 and macOS Terminal.app do not
    /// support this.
    ///
    /// If the terminal does not support true color, code using the  [`TermwizBackend`] will
    /// fallback to the default text color. Crossterm and Termion do not have this capability and
    /// the display will be unpredictable (e.g. Terminal.app may display glitched blinking text).
    /// See <https://github.com/ratatui-org/ratatui/issues/475> for an example of this problem.
    ///
    /// See also: <https://en.wikipedia.org/wiki/ANSI_escape_code#24-bit>
    ///
    /// [`TermwizBackend`]: crate::backend::TermwizBackend
    Rgb(u8, u8, u8),
    /// An 8-bit 256 color.
    ///
    /// See also <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
    Indexed(u8),
}

impl Color {
    /// Convert a u32 to a Color
    ///
    /// The u32 should be in the format 0x00RRGGBB.
    pub const fn from_u32(u: u32) -> Self {
        let r = (u >> 16) as u8;
        let g = (u >> 8) as u8;
        let b = u as u8;
        Self::Rgb(r, g, b)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Color {
    /// This utilises the [`fmt::Display`] implementation for serialization.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Color {
    /// This is used to deserialize a value into Color via serde.
    ///
    /// This implementation uses the `FromStr` trait to deserialize strings, so named colours, RGB,
    /// and indexed values are able to be deserialized. In addition, values that were produced by
    /// the the older serialization implementation of Color are also able to be deserialized.
    ///
    /// Prior to v0.26.0, Ratatui would be serialized using a map for indexed and RGB values, for
    /// examples in json `{"Indexed": 10}` and `{"Rgb": [255, 0, 255]}` respectively. Now they are
    /// serialized using the string representation of the index and the RGB hex value, for example
    /// in json it would now be `"10"` and `"#FF00FF"` respectively.
    ///
    /// See the [`Color`] documentation for more information on color names.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::prelude::*;
    ///
    /// #[derive(Debug, serde::Deserialize)]
    /// struct Theme {
    ///     color: Color,
    /// }
    ///
    /// # fn get_theme() -> Result<(), serde_json::Error> {
    /// let theme: Theme = serde_json::from_str(r#"{"color": "bright-white"}"#)?;
    /// assert_eq!(theme.color, Color::White);
    ///
    /// let theme: Theme = serde_json::from_str(r##"{"color": "#00FF00"}"##)?;
    /// assert_eq!(theme.color, Color::Rgb(0, 255, 0));
    ///
    /// let theme: Theme = serde_json::from_str(r#"{"color": "42"}"#)?;
    /// assert_eq!(theme.color, Color::Indexed(42));
    ///
    /// let err = serde_json::from_str::<Theme>(r#"{"color": "invalid"}"#).unwrap_err();
    /// assert!(err.is_data());
    /// assert_eq!(
    ///     err.to_string(),
    ///     "Failed to parse Colors at line 1 column 20"
    /// );
    ///
    /// // Deserializing from the previous serialization implementation
    /// let theme: Theme = serde_json::from_str(r#"{"color": {"Rgb":[255,0,255]}}"#)?;
    /// assert_eq!(theme.color, Color::Rgb(255, 0, 255));
    ///
    /// let theme: Theme = serde_json::from_str(r#"{"color": {"Indexed":10}}"#)?;
    /// assert_eq!(theme.color, Color::Indexed(10));
    /// # Ok(())
    /// # }
    /// ```
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// Colors are currently serialized with the `Display` implementation, so
        /// RGB values are serialized via hex, for example "#FFFFFF".
        ///
        /// Previously they were serialized using serde derive, which encoded
        /// RGB values as a map, for example { "rgb": [255, 255, 255] }.
        ///
        /// The deserialization implementation utilises a `Helper` struct
        /// to be able to support both formats for backwards compatibility.
        #[derive(serde::Deserialize)]
        enum ColorWrapper {
            Rgb(u8, u8, u8),
            Indexed(u8),
        }

        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum ColorFormat {
            V2(String),
            V1(ColorWrapper),
        }

        let multi_type = ColorFormat::deserialize(deserializer)
            .map_err(|err| serde::de::Error::custom(format!("Failed to parse Colors: {err}")))?;
        match multi_type {
            ColorFormat::V2(s) => FromStr::from_str(&s).map_err(serde::de::Error::custom),
            ColorFormat::V1(color_wrapper) => match color_wrapper {
                ColorWrapper::Rgb(red, green, blue) => Ok(Self::Rgb(red, green, blue)),
                ColorWrapper::Indexed(index) => Ok(Self::Indexed(index)),
            },
        }
    }
}

/// Error type indicating a failure to parse a color string.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ParseColorError;

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
/// See the [`Color`] documentation for more information on the supported color names.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use ratatui::prelude::*;
///
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

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reset => write!(f, "Reset"),
            Self::Black => write!(f, "Black"),
            Self::Red => write!(f, "Red"),
            Self::Green => write!(f, "Green"),
            Self::Yellow => write!(f, "Yellow"),
            Self::Blue => write!(f, "Blue"),
            Self::Magenta => write!(f, "Magenta"),
            Self::Cyan => write!(f, "Cyan"),
            Self::Gray => write!(f, "Gray"),
            Self::DarkGray => write!(f, "DarkGray"),
            Self::LightRed => write!(f, "LightRed"),
            Self::LightGreen => write!(f, "LightGreen"),
            Self::LightYellow => write!(f, "LightYellow"),
            Self::LightBlue => write!(f, "LightBlue"),
            Self::LightMagenta => write!(f, "LightMagenta"),
            Self::LightCyan => write!(f, "LightCyan"),
            Self::White => write!(f, "White"),
            Self::Rgb(r, g, b) => write!(f, "#{r:02X}{g:02X}{b:02X}"),
            Self::Indexed(i) => write!(f, "{i}"),
        }
    }
}

impl Color {
    /// Converts a HSL representation to a `Color::Rgb` instance.
    ///
    /// The `from_hsl` function converts the Hue, Saturation and Lightness values to a
    /// corresponding `Color` RGB equivalent.
    ///
    /// Hue values should be in the range [0, 360].
    /// Saturation and L values should be in the range [0, 100].
    /// Values that are not in the range are clamped to be within the range.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::prelude::*;
    ///
    /// let color: Color = Color::from_hsl(360.0, 100.0, 100.0);
    /// assert_eq!(color, Color::Rgb(255, 255, 255));
    ///
    /// let color: Color = Color::from_hsl(0.0, 0.0, 0.0);
    /// assert_eq!(color, Color::Rgb(0, 0, 0));
    /// ```
    pub fn from_hsl(h: f64, s: f64, l: f64) -> Self {
        // Clamp input values to valid ranges
        let h = h.clamp(0.0, 360.0);
        let s = s.clamp(0.0, 100.0);
        let l = l.clamp(0.0, 100.0);

        // Delegate to the function for normalized HSL to RGB conversion
        normalized_hsl_to_rgb(h / 360.0, s / 100.0, l / 100.0)
    }
}

/// Converts normalized HSL (Hue, Saturation, Lightness) values to RGB (Red, Green, Blue) color
/// representation. H, S, and L values should be in the range [0, 1].
///
/// Based on <https://github.com/killercup/hsl-rs/blob/b8a30e11afd75f262e0550725333293805f4ead0/src/lib.rs>
fn normalized_hsl_to_rgb(hue: f64, saturation: f64, lightness: f64) -> Color {
    // This function can be made into `const` in the future.
    // This comment contains the relevant information for making it `const`.
    //
    // If it is `const` and made public, users can write the following:
    //
    // ```rust
    // const SLATE_50: Color = normalized_hsl_to_rgb(0.210, 0.40, 0.98);
    // ```
    //
    // For it to be const now, we need `#![feature(const_fn_floating_point_arithmetic)]`
    // Tracking issue: https://github.com/rust-lang/rust/issues/57241
    //
    // We would also need to remove the use of `.round()` in this function, i.e.:
    //
    // ```rust
    // Color::Rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    // ```

    // Initialize RGB components
    let red: f64;
    let green: f64;
    let blue: f64;

    // Check if the color is achromatic (grayscale)
    if saturation == 0.0 {
        red = lightness;
        green = lightness;
        blue = lightness;
    } else {
        // Calculate RGB components for colored cases
        let q = if lightness < 0.5 {
            lightness * (1.0 + saturation)
        } else {
            lightness + saturation - lightness * saturation
        };
        let p = 2.0 * lightness - q;
        red = hue_to_rgb(p, q, hue + 1.0 / 3.0);
        green = hue_to_rgb(p, q, hue);
        blue = hue_to_rgb(p, q, hue - 1.0 / 3.0);
    }

    // Scale RGB components to the range [0, 255] and create a Color::Rgb instance
    Color::Rgb(
        (red * 255.0).round() as u8,
        (green * 255.0).round() as u8,
        (blue * 255.0).round() as u8,
    )
}

/// Helper function to calculate RGB component for a specific hue value.
fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
    // Adjust the hue value to be within the valid range [0, 1]
    let mut t = t;
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }

    // Calculate the RGB component based on the hue value
    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    #[cfg(feature = "serde")]
    use serde::de::{Deserialize, IntoDeserializer};

    use super::*;

    #[test]
    fn test_hsl_to_rgb() {
        // Test with valid HSL values
        let color = Color::from_hsl(120.0, 50.0, 75.0);
        assert_eq!(color, Color::Rgb(159, 223, 159));

        // Test with H value at upper bound
        let color = Color::from_hsl(360.0, 50.0, 75.0);
        assert_eq!(color, Color::Rgb(223, 159, 159));

        // Test with H value exceeding the upper bound
        let color = Color::from_hsl(400.0, 50.0, 75.0);
        assert_eq!(color, Color::Rgb(223, 159, 159));

        // Test with S and L values exceeding the upper bound
        let color = Color::from_hsl(240.0, 120.0, 150.0);
        assert_eq!(color, Color::Rgb(255, 255, 255));

        // Test with H, S, and L values below the lower bound
        let color = Color::from_hsl(-20.0, -50.0, -20.0);
        assert_eq!(color, Color::Rgb(0, 0, 0));

        // Test with S and L values below the lower bound
        let color = Color::from_hsl(60.0, -20.0, -10.0);
        assert_eq!(color, Color::Rgb(0, 0, 0));
    }

    #[test]
    fn from_u32() {
        assert_eq!(Color::from_u32(0x000000), Color::Rgb(0, 0, 0));
        assert_eq!(Color::from_u32(0xFF0000), Color::Rgb(255, 0, 0));
        assert_eq!(Color::from_u32(0x00FF00), Color::Rgb(0, 255, 0));
        assert_eq!(Color::from_u32(0x0000FF), Color::Rgb(0, 0, 255));
        assert_eq!(Color::from_u32(0xFFFFFF), Color::Rgb(255, 255, 255));
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

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize() -> Result<(), serde::de::value::Error> {
        assert_eq!(
            Color::Black,
            Color::deserialize("Black".into_deserializer())?
        );
        assert_eq!(
            Color::Magenta,
            Color::deserialize("magenta".into_deserializer())?
        );
        assert_eq!(
            Color::LightGreen,
            Color::deserialize("LightGreen".into_deserializer())?
        );
        assert_eq!(
            Color::White,
            Color::deserialize("bright-white".into_deserializer())?
        );
        assert_eq!(
            Color::Indexed(42),
            Color::deserialize("42".into_deserializer())?
        );
        assert_eq!(
            Color::Rgb(0, 255, 0),
            Color::deserialize("#00ff00".into_deserializer())?
        );
        Ok(())
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize_error() {
        let color: Result<_, serde::de::value::Error> =
            Color::deserialize("invalid".into_deserializer());
        assert!(color.is_err());

        let color: Result<_, serde::de::value::Error> =
            Color::deserialize("#00000000".into_deserializer());
        assert!(color.is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_then_deserialize() -> Result<(), serde_json::Error> {
        let json_rgb = serde_json::to_string(&Color::Rgb(255, 0, 255))?;
        assert_eq!(json_rgb, r##""#FF00FF""##);
        assert_eq!(
            serde_json::from_str::<Color>(&json_rgb)?,
            Color::Rgb(255, 0, 255)
        );

        let json_white = serde_json::to_string(&Color::White)?;
        assert_eq!(json_white, r#""White""#);

        let json_indexed = serde_json::to_string(&Color::Indexed(10))?;
        assert_eq!(json_indexed, r#""10""#);
        assert_eq!(
            serde_json::from_str::<Color>(&json_indexed)?,
            Color::Indexed(10)
        );

        Ok(())
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize_with_previous_format() -> Result<(), serde_json::Error> {
        assert_eq!(Color::White, serde_json::from_str::<Color>("\"White\"")?);
        assert_eq!(
            Color::Rgb(255, 0, 255),
            serde_json::from_str::<Color>(r#"{"Rgb":[255,0,255]}"#)?
        );
        assert_eq!(
            Color::Indexed(10),
            serde_json::from_str::<Color>(r#"{"Indexed":10}"#)?
        );
        Ok(())
    }
}
