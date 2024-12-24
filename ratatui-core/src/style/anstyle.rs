//! This module contains conversion functions for styles from the `anstyle` crate.
use anstyle::{Ansi256Color, AnsiColor, Effects, RgbColor};
use thiserror::Error;

use super::{Color, Modifier, Style};

/// Error type for converting between `anstyle` colors and `Color`
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TryFromColorError {
    #[error("cannot convert Ratatui Color to an Ansi256Color as it is not an indexed color")]
    Ansi256,
    #[error("cannot convert Ratatui Color to AnsiColor as it is not a 4-bit color")]
    Ansi,
    #[error("cannot convert Ratatui Color to RgbColor as it is not an RGB color")]
    RgbColor,
}

impl From<Ansi256Color> for Color {
    fn from(color: Ansi256Color) -> Self {
        Self::Indexed(color.index())
    }
}

impl TryFrom<Color> for Ansi256Color {
    type Error = TryFromColorError;

    fn try_from(color: Color) -> Result<Self, Self::Error> {
        match color {
            Color::Indexed(index) => Ok(Self(index)),
            _ => Err(TryFromColorError::Ansi256),
        }
    }
}

impl From<AnsiColor> for Color {
    fn from(value: AnsiColor) -> Self {
        match value {
            AnsiColor::Black => Self::Black,
            AnsiColor::Red => Self::Red,
            AnsiColor::Green => Self::Green,
            AnsiColor::Yellow => Self::Yellow,
            AnsiColor::Blue => Self::Blue,
            AnsiColor::Magenta => Self::Magenta,
            AnsiColor::Cyan => Self::Cyan,
            AnsiColor::White => Self::Gray,
            AnsiColor::BrightBlack => Self::DarkGray,
            AnsiColor::BrightRed => Self::LightRed,
            AnsiColor::BrightGreen => Self::LightGreen,
            AnsiColor::BrightYellow => Self::LightYellow,
            AnsiColor::BrightBlue => Self::LightBlue,
            AnsiColor::BrightMagenta => Self::LightMagenta,
            AnsiColor::BrightCyan => Self::LightCyan,
            AnsiColor::BrightWhite => Self::White,
        }
    }
}

impl TryFrom<Color> for AnsiColor {
    type Error = TryFromColorError;

    fn try_from(color: Color) -> Result<Self, Self::Error> {
        match color {
            Color::Black => Ok(Self::Black),
            Color::Red => Ok(Self::Red),
            Color::Green => Ok(Self::Green),
            Color::Yellow => Ok(Self::Yellow),
            Color::Blue => Ok(Self::Blue),
            Color::Magenta => Ok(Self::Magenta),
            Color::Cyan => Ok(Self::Cyan),
            Color::Gray => Ok(Self::White),
            Color::DarkGray => Ok(Self::BrightBlack),
            Color::LightRed => Ok(Self::BrightRed),
            Color::LightGreen => Ok(Self::BrightGreen),
            Color::LightYellow => Ok(Self::BrightYellow),
            Color::LightBlue => Ok(Self::BrightBlue),
            Color::LightMagenta => Ok(Self::BrightMagenta),
            Color::LightCyan => Ok(Self::BrightCyan),
            Color::White => Ok(Self::BrightWhite),
            _ => Err(TryFromColorError::Ansi),
        }
    }
}

impl From<RgbColor> for Color {
    fn from(color: RgbColor) -> Self {
        Self::Rgb(color.r(), color.g(), color.b())
    }
}

impl TryFrom<Color> for RgbColor {
    type Error = TryFromColorError;

    fn try_from(color: Color) -> Result<Self, Self::Error> {
        match color {
            Color::Rgb(red, green, blue) => Ok(Self(red, green, blue)),
            _ => Err(TryFromColorError::RgbColor),
        }
    }
}

impl From<anstyle::Color> for Color {
    fn from(color: anstyle::Color) -> Self {
        match color {
            anstyle::Color::Ansi(ansi_color) => Self::from(ansi_color),
            anstyle::Color::Ansi256(ansi256_color) => Self::from(ansi256_color),
            anstyle::Color::Rgb(rgb_color) => Self::from(rgb_color),
        }
    }
}

impl From<Color> for anstyle::Color {
    fn from(color: Color) -> Self {
        match color {
            Color::Rgb(_, _, _) => Self::Rgb(RgbColor::try_from(color).unwrap()),
            Color::Indexed(_) => Self::Ansi256(Ansi256Color::try_from(color).unwrap()),
            _ => Self::Ansi(AnsiColor::try_from(color).unwrap()),
        }
    }
}

impl From<Effects> for Modifier {
    fn from(effect: Effects) -> Self {
        let mut modifier = Self::empty();
        if effect.contains(Effects::BOLD) {
            modifier |= Self::BOLD;
        }
        if effect.contains(Effects::DIMMED) {
            modifier |= Self::DIM;
        }
        if effect.contains(Effects::ITALIC) {
            modifier |= Self::ITALIC;
        }
        if effect.contains(Effects::UNDERLINE)
            || effect.contains(Effects::DOUBLE_UNDERLINE)
            || effect.contains(Effects::CURLY_UNDERLINE)
            || effect.contains(Effects::DOTTED_UNDERLINE)
            || effect.contains(Effects::DASHED_UNDERLINE)
        {
            modifier |= Self::UNDERLINED;
        }
        if effect.contains(Effects::BLINK) {
            modifier |= Self::SLOW_BLINK;
        }
        if effect.contains(Effects::INVERT) {
            modifier |= Self::REVERSED;
        }
        if effect.contains(Effects::HIDDEN) {
            modifier |= Self::HIDDEN;
        }
        if effect.contains(Effects::STRIKETHROUGH) {
            modifier |= Self::CROSSED_OUT;
        }
        modifier
    }
}

impl From<Modifier> for Effects {
    fn from(modifier: Modifier) -> Self {
        let mut effects = Self::new();
        if modifier.contains(Modifier::BOLD) {
            effects |= Self::BOLD;
        }
        if modifier.contains(Modifier::DIM) {
            effects |= Self::DIMMED;
        }
        if modifier.contains(Modifier::ITALIC) {
            effects |= Self::ITALIC;
        }
        if modifier.contains(Modifier::UNDERLINED) {
            effects |= Self::UNDERLINE;
        }
        if modifier.contains(Modifier::SLOW_BLINK) || modifier.contains(Modifier::RAPID_BLINK) {
            effects |= Self::BLINK;
        }
        if modifier.contains(Modifier::REVERSED) {
            effects |= Self::INVERT;
        }
        if modifier.contains(Modifier::HIDDEN) {
            effects |= Self::HIDDEN;
        }
        if modifier.contains(Modifier::CROSSED_OUT) {
            effects |= Self::STRIKETHROUGH;
        }
        effects
    }
}

impl From<anstyle::Style> for Style {
    fn from(style: anstyle::Style) -> Self {
        Self {
            fg: style.get_fg_color().map(Color::from),
            bg: style.get_bg_color().map(Color::from),
            add_modifier: style.get_effects().into(),
            ..Default::default()
        }
    }
}

impl From<Style> for anstyle::Style {
    fn from(style: Style) -> Self {
        let mut anstyle_style = Self::new();
        if let Some(fg) = style.fg {
            let fg = anstyle::Color::from(fg);
            anstyle_style = anstyle_style.fg_color(Some(fg));
        }
        if let Some(bg) = style.bg {
            let bg = anstyle::Color::from(bg);
            anstyle_style = anstyle_style.bg_color(Some(bg));
        }
        anstyle_style = anstyle_style.effects(style.add_modifier.into());
        anstyle_style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anstyle_to_color() {
        let anstyle_color = Ansi256Color(42);
        let color = Color::from(anstyle_color);
        assert_eq!(color, Color::Indexed(42));
    }

    #[test]
    fn color_to_ansi256color() {
        let color = Color::Indexed(42);
        let anstyle_color = Ansi256Color::try_from(color);
        assert_eq!(anstyle_color, Ok(Ansi256Color(42)));
    }

    #[test]
    fn color_to_ansi256color_error() {
        let color = Color::Rgb(0, 0, 0);
        let anstyle_color = Ansi256Color::try_from(color);
        assert_eq!(anstyle_color, Err(TryFromColorError::Ansi256));
    }

    #[test]
    fn ansi_color_to_color() {
        let ansi_color = AnsiColor::Red;
        let color = Color::from(ansi_color);
        assert_eq!(color, Color::Red);
    }

    #[test]
    fn color_to_ansicolor() {
        let color = Color::Red;
        let ansi_color = AnsiColor::try_from(color);
        assert_eq!(ansi_color, Ok(AnsiColor::Red));
    }

    #[test]
    fn color_to_ansicolor_error() {
        let color = Color::Rgb(0, 0, 0);
        let ansi_color = AnsiColor::try_from(color);
        assert_eq!(ansi_color, Err(TryFromColorError::Ansi));
    }

    #[test]
    fn rgb_color_to_color() {
        let rgb_color = RgbColor(255, 0, 0);
        let color = Color::from(rgb_color);
        assert_eq!(color, Color::Rgb(255, 0, 0));
    }

    #[test]
    fn color_to_rgbcolor() {
        let color = Color::Rgb(255, 0, 0);
        let rgb_color = RgbColor::try_from(color);
        assert_eq!(rgb_color, Ok(RgbColor(255, 0, 0)));
    }

    #[test]
    fn color_to_rgbcolor_error() {
        let color = Color::Indexed(42);
        let rgb_color = RgbColor::try_from(color);
        assert_eq!(rgb_color, Err(TryFromColorError::RgbColor));
    }

    #[test]
    fn effects_to_modifier() {
        let effects = Effects::BOLD | Effects::ITALIC;
        let modifier = Modifier::from(effects);
        assert!(modifier.contains(Modifier::BOLD));
        assert!(modifier.contains(Modifier::ITALIC));
    }

    #[test]
    fn modifier_to_effects() {
        let modifier = Modifier::BOLD | Modifier::ITALIC;
        let effects = Effects::from(modifier);
        assert!(effects.contains(Effects::BOLD));
        assert!(effects.contains(Effects::ITALIC));
    }

    #[test]
    fn anstyle_style_to_style() {
        let anstyle_style = anstyle::Style::new()
            .fg_color(Some(anstyle::Color::Ansi(AnsiColor::Red)))
            .bg_color(Some(anstyle::Color::Ansi(AnsiColor::Blue)))
            .effects(Effects::BOLD | Effects::ITALIC);
        let style = Style::from(anstyle_style);
        assert_eq!(style.fg, Some(Color::Red));
        assert_eq!(style.bg, Some(Color::Blue));
        assert!(style.add_modifier.contains(Modifier::BOLD));
        assert!(style.add_modifier.contains(Modifier::ITALIC));
    }

    #[test]
    fn style_to_anstyle_style() {
        let style = Style {
            fg: Some(Color::Red),
            bg: Some(Color::Blue),
            add_modifier: Modifier::BOLD | Modifier::ITALIC,
            ..Default::default()
        };
        let anstyle_style = anstyle::Style::from(style);
        assert_eq!(
            anstyle_style.get_fg_color(),
            Some(anstyle::Color::Ansi(AnsiColor::Red))
        );
        assert_eq!(
            anstyle_style.get_bg_color(),
            Some(anstyle::Color::Ansi(AnsiColor::Blue))
        );
        assert!(anstyle_style.get_effects().contains(Effects::BOLD));
        assert!(anstyle_style.get_effects().contains(Effects::ITALIC));
    }
}
