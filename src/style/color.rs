#![allow(clippy::unreadable_literal)]

use std::{fmt, str::FromStr};

use crate::style::stylize::{ColorDebug, ColorDebugKind};

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
/// use ratatui::style::Color;
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
    /// See <https://github.com/ratatui/ratatui/issues/475> for an example of this problem.
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
    /// use std::str::FromStr;
    ///
    /// use ratatui::style::Color;
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
/// use ratatui::style::Color;
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
                    } else if let Some((r, g, b)) = parse_hex_color(s) {
                        Self::Rgb(r, g, b)
                    } else {
                        return Err(ParseColorError);
                    }
                }
            },
        )
    }
}

fn parse_hex_color(input: &str) -> Option<(u8, u8, u8)> {
    if !input.starts_with('#') || input.len() != 7 {
        return None;
    }
    let r = u8::from_str_radix(input.get(1..3)?, 16).ok()?;
    let g = u8::from_str_radix(input.get(3..5)?, 16).ok()?;
    let b = u8::from_str_radix(input.get(5..7)?, 16).ok()?;
    Some((r, g, b))
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
    pub(crate) const fn stylize_debug(self, kind: ColorDebugKind) -> ColorDebug {
        ColorDebug { kind, color: self }
    }

    /// Converts a HSL representation to a `Color::Rgb` instance.
    ///
    /// The `from_hsl` function converts the Hue, Saturation and Lightness values to a corresponding
    /// `Color` RGB equivalent.
    ///
    /// Hue values should be in the range [-180..180]. Values outside this range are normalized by
    /// wrapping.
    ///
    /// Saturation and L values should be in the range [0.0..1.0]. Values outside this range are
    /// clamped.
    ///
    /// Clamping to valid ranges happens before conversion to RGB.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::{palette::Hsl, style::Color};
    ///
    /// // Minimum Lightness is black
    /// let color: Color = Color::from_hsl(Hsl::new(0.0, 0.0, 0.0));
    /// assert_eq!(color, Color::Rgb(0, 0, 0));
    ///
    /// // Maximum Lightness is white
    /// let color: Color = Color::from_hsl(Hsl::new(0.0, 0.0, 1.0));
    /// assert_eq!(color, Color::Rgb(255, 255, 255));
    ///
    /// // Minimum Saturation is fully desaturated red = gray
    /// let color: Color = Color::from_hsl(Hsl::new(0.0, 0.0, 0.5));
    /// assert_eq!(color, Color::Rgb(128, 128, 128));
    ///
    /// // Bright red
    /// let color: Color = Color::from_hsl(Hsl::new(0.0, 1.0, 0.5));
    /// assert_eq!(color, Color::Rgb(255, 0, 0));
    ///
    /// // Bright blue
    /// let color: Color = Color::from_hsl(Hsl::new(-120.0, 1.0, 0.5));
    /// assert_eq!(color, Color::Rgb(0, 0, 255));
    /// ```
    #[cfg(feature = "palette")]
    pub fn from_hsl(hsl: palette::Hsl) -> Self {
        use palette::{Clamp, FromColor, Srgb};
        let hsl = hsl.clamp();
        let Srgb {
            red,
            green,
            blue,
            standard: _,
        }: Srgb<u8> = Srgb::from_color(hsl).into();

        Self::Rgb(red, green, blue)
    }

    /// Converts a `HSLuv` representation to a `Color::Rgb` instance.
    ///
    /// The `from_hsluv` function converts the Hue, Saturation and Lightness values to a
    /// corresponding `Color` RGB equivalent.
    ///
    /// Hue values should be in the range [-180.0..180.0]. Values outside this range are normalized
    /// by wrapping.
    ///
    /// Saturation and L values should be in the range [0.0..100.0]. Values outside this range are
    /// clamped.
    ///
    /// Clamping to valid ranges happens before conversion to RGB.
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::{palette::Hsluv, style::Color};
    ///
    /// // Minimum Lightness is black
    /// let color: Color = Color::from_hsluv(Hsluv::new(0.0, 100.0, 0.0));
    /// assert_eq!(color, Color::Rgb(0, 0, 0));
    ///
    /// // Maximum Lightness is white
    /// let color: Color = Color::from_hsluv(Hsluv::new(0.0, 0.0, 100.0));
    /// assert_eq!(color, Color::Rgb(255, 255, 255));
    ///
    /// // Minimum Saturation is fully desaturated red = gray
    /// let color = Color::from_hsluv(Hsluv::new(0.0, 0.0, 50.0));
    /// assert_eq!(color, Color::Rgb(119, 119, 119));
    ///
    /// // Bright Red
    /// let color = Color::from_hsluv(Hsluv::new(12.18, 100.0, 53.2));
    /// assert_eq!(color, Color::Rgb(255, 0, 0));
    ///
    /// // Bright Blue
    /// let color = Color::from_hsluv(Hsluv::new(-94.13, 100.0, 32.3));
    /// assert_eq!(color, Color::Rgb(0, 0, 255));
    /// ```
    #[cfg(feature = "palette")]
    pub fn from_hsluv(hsluv: palette::Hsluv) -> Self {
        use palette::{Clamp, FromColor, Srgb};
        let hsluv = hsluv.clamp();
        let Srgb {
            red,
            green,
            blue,
            standard: _,
        }: Srgb<u8> = Srgb::from_color(hsluv).into();

        Self::Rgb(red, green, blue)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    #[cfg(feature = "palette")]
    use palette::{Hsl, Hsluv};
    use rstest::rstest;
    #[cfg(feature = "serde")]
    use serde::de::{Deserialize, IntoDeserializer};

    use super::*;

    #[cfg(feature = "palette")]
    #[rstest]
    #[case::black(Hsl::new(0.0, 0.0, 0.0), Color::Rgb(0, 0, 0))]
    #[case::white(Hsl::new(0.0, 0.0, 1.0), Color::Rgb(255, 255, 255))]
    #[case::valid(Hsl::new(120.0, 0.5, 0.75), Color::Rgb(159, 223, 159))]
    #[case::min_hue(Hsl::new(-180.0, 0.5, 0.75), Color::Rgb(159, 223, 223))]
    #[case::max_hue(Hsl::new(180.0, 0.5, 0.75), Color::Rgb(159, 223, 223))]
    #[case::min_saturation(Hsl::new(0.0, 0.0, 0.5), Color::Rgb(128, 128, 128))]
    #[case::max_saturation(Hsl::new(0.0, 1.0, 0.5), Color::Rgb(255, 0, 0))]
    #[case::min_lightness(Hsl::new(0.0, 0.5, 0.0), Color::Rgb(0, 0, 0))]
    #[case::max_lightness(Hsl::new(0.0, 0.5, 1.0), Color::Rgb(255, 255, 255))]
    #[case::under_hue_wraps(Hsl::new(-240.0, 0.5, 0.75), Color::Rgb(159, 223, 159))]
    #[case::over_hue_wraps(Hsl::new(480.0, 0.5, 0.75), Color::Rgb(159, 223, 159))]
    #[case::under_saturation_clamps(Hsl::new(0.0, -0.5, 0.75), Color::Rgb(191, 191, 191))]
    #[case::over_saturation_clamps(Hsl::new(0.0, 1.2, 0.75), Color::Rgb(255, 128, 128))]
    #[case::under_lightness_clamps(Hsl::new(0.0, 0.5, -0.20), Color::Rgb(0, 0, 0))]
    #[case::over_lightness_clamps(Hsl::new(0.0, 0.5, 1.5), Color::Rgb(255, 255, 255))]
    #[case::under_saturation_lightness_clamps(Hsl::new(0.0, -0.5, -0.20), Color::Rgb(0, 0, 0))]
    #[case::over_saturation_lightness_clamps(Hsl::new(0.0, 1.2, 1.5), Color::Rgb(255, 255, 255))]
    fn test_hsl_to_rgb(#[case] hsl: palette::Hsl, #[case] expected: Color) {
        assert_eq!(Color::from_hsl(hsl), expected);
    }

    #[cfg(feature = "palette")]
    #[rstest]
    #[case::black(Hsluv::new(0.0, 0.0, 0.0), Color::Rgb(0, 0, 0))]
    #[case::white(Hsluv::new(0.0, 0.0, 100.0), Color::Rgb(255, 255, 255))]
    #[case::valid(Hsluv::new(120.0, 50.0, 75.0), Color::Rgb(147, 198, 129))]
    #[case::min_hue(Hsluv::new(-180.0, 50.0, 75.0), Color::Rgb(135,196, 188))]
    #[case::max_hue(Hsluv::new(180.0, 50.0, 75.0), Color::Rgb(135, 196, 188))]
    #[case::min_saturation(Hsluv::new(0.0, 0.0, 75.0), Color::Rgb(185, 185, 185))]
    #[case::max_saturation(Hsluv::new(0.0, 100.0, 75.0), Color::Rgb(255, 156, 177))]
    #[case::min_lightness(Hsluv::new(0.0, 50.0, 0.0), Color::Rgb(0, 0, 0))]
    #[case::max_lightness(Hsluv::new(0.0, 50.0, 100.0), Color::Rgb(255, 255, 255))]
    #[case::under_hue_wraps(Hsluv::new(-240.0, 50.0, 75.0), Color::Rgb(147, 198, 129))]
    #[case::over_hue_wraps(Hsluv::new(480.0, 50.0, 75.0), Color::Rgb(147, 198, 129))]
    #[case::under_saturation_clamps(Hsluv::new(0.0, -50.0, 75.0), Color::Rgb(185, 185, 185))]
    #[case::over_saturation_clamps(Hsluv::new(0.0, 150.0, 75.0), Color::Rgb(255, 156, 177))]
    #[case::under_lightness_clamps(Hsluv::new(0.0, 50.0, -20.0), Color::Rgb(0, 0, 0))]
    #[case::over_lightness_clamps(Hsluv::new(0.0, 50.0, 150.0), Color::Rgb(255, 255, 255))]
    #[case::under_saturation_lightness_clamps(Hsluv::new(0.0, -50.0, -20.0), Color::Rgb(0, 0, 0))]
    #[case::over_saturation_lightness_clamps(
        Hsluv::new(0.0, 150.0, 150.0),
        Color::Rgb(255, 255, 255)
    )]
    fn test_hsluv_to_rgb(#[case] hsluv: palette::Hsluv, #[case] expected: Color) {
        assert_eq!(Color::from_hsluv(hsluv), expected);
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
            "#1🦀2",         // len 7 but on char boundaries shouldnt panic
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
