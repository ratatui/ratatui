use crate::{
    style::{Color, Modifier, Style},
    text::Span,
};

pub trait Styled {
    type Item;

    fn style(&self) -> Style;
    fn set_style(self, style: Style) -> Self::Item;
}

// Otherwise rustfmt behaves weirdly for some reason
macro_rules! calculated_docs {
    ($(#[doc = $doc:expr] $item:item)*) => { $(#[doc = $doc] $item)* };
}

macro_rules! modifier_method {
    ($method_name:ident Modifier::$modifier:ident) => {
        calculated_docs! {
            #[doc = concat!(
                "Applies the [`",
                stringify!($modifier),
                "`](crate::style::Modifier::",
                stringify!($modifier),
                ") modifier.",
            )]
            fn $method_name(self) -> T {
                self.modifier(Modifier::$modifier)
            }
        }
    };
}

macro_rules! color_method {
    ($method_name_fg:ident, $method_name_bg:ident Color::$color:ident) => {
        calculated_docs! {
            #[doc = concat!(
                "Sets the foreground color to [`",
                stringify!($color),
                "`](Color::",
                stringify!($color),
                ")."
            )]
            fn $method_name_fg(self) -> T {
                self.fg(Color::$color)
            }

            #[doc = concat!(
                "Sets the background color to [`",
                stringify!($color),
                "`](Color::",
                stringify!($color),
                ")."
            )]
            fn $method_name_bg(self) -> T {
                self.bg(Color::$color)
            }
        }
    };
}

/// The trait that enables something to be have a style.
/// # Examples
/// ```
/// use ratatui::{
///     style::{Color, Modifier, Style, Styled, Stylize},
///     text::Span,
/// };
///
/// assert_eq!(
///    "hello".red().on_blue().bold(),
///     Span::styled("hello", Style::default().fg(Color::Red).bg(Color::Blue).add_modifier(Modifier::BOLD))
/// )
pub trait Stylize<'a, T>: Sized {
    // Colors
    fn fg<S: Into<Color>>(self, color: S) -> T;
    fn bg(self, color: Color) -> T;

    color_method!(black, on_black Color::Black);
    color_method!(red, on_red Color::Red);
    color_method!(green, on_green Color::Green);
    color_method!(yellow, on_yellow Color::Yellow);
    color_method!(blue, on_blue Color::Blue);
    color_method!(magenta, on_magenta Color::Magenta);
    color_method!(cyan, on_cyan Color::Cyan);
    color_method!(gray, on_gray Color::Gray);
    color_method!(dark_gray, on_dark_gray Color::DarkGray);
    color_method!(light_red, on_light_red Color::LightRed);
    color_method!(light_green, on_light_green Color::LightGreen);
    color_method!(light_yellow, on_light_yellow Color::LightYellow);
    color_method!(light_blue, on_light_blue Color::LightBlue);
    color_method!(light_magenta, on_light_magenta Color::LightMagenta);
    color_method!(light_cyan, on_light_cyan Color::LightCyan);
    color_method!(white, on_white Color::White);

    // Styles
    fn reset(self) -> T;

    // Modifiers
    fn modifier(self, modifier: Modifier) -> T;

    modifier_method!(bold Modifier::BOLD);
    modifier_method!(dimmed Modifier::DIM);
    modifier_method!(italic Modifier::ITALIC);
    modifier_method!(underline Modifier::UNDERLINED);
    modifier_method!(slow_blink Modifier::SLOW_BLINK);
    modifier_method!(rapid_blink Modifier::RAPID_BLINK);
    modifier_method!(reversed Modifier::REVERSED);
    modifier_method!(hidden Modifier::HIDDEN);
    modifier_method!(crossed_out Modifier::CROSSED_OUT);
}

impl<'a, T, U> Stylize<'a, T> for U
where
    U: Styled<Item = T>,
{
    fn fg<S: Into<Color>>(self, color: S) -> T {
        let style = self.style().fg(color.into());
        self.set_style(style)
    }

    fn modifier(self, modifier: Modifier) -> T {
        let style = self.style().add_modifier(modifier);
        self.set_style(style)
    }

    fn bg(self, color: Color) -> T {
        let style = self.style().bg(color);
        self.set_style(style)
    }

    fn reset(self) -> T {
        self.set_style(Style::default())
    }
}

impl<'a> Styled for &'a str {
    type Item = Span<'a>;

    fn style(&self) -> Style {
        Style::default()
    }

    fn set_style(self, style: Style) -> Self::Item {
        Span::styled(self, style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset() {
        assert_eq!(
            "hello".on_cyan().light_red().bold().underline().reset(),
            Span::from("hello")
        )
    }

    #[test]
    fn fg() {
        let cyan_fg = Style::default().fg(Color::Cyan);

        assert_eq!("hello".cyan(), Span::styled("hello", cyan_fg));
    }

    #[test]
    fn bg() {
        let cyan_bg = Style::default().bg(Color::Cyan);

        assert_eq!("hello".on_cyan(), Span::styled("hello", cyan_bg));
    }

    #[test]
    fn color_modifier() {
        let cyan_bold = Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD);

        assert_eq!("hello".cyan().bold(), Span::styled("hello", cyan_bold))
    }

    #[test]
    fn fg_bg() {
        let cyan_fg_bg = Style::default().bg(Color::Cyan).fg(Color::Cyan);

        assert_eq!("hello".cyan().on_cyan(), Span::styled("hello", cyan_fg_bg))
    }

    #[test]
    fn repeated_attributes() {
        let cyan_bg = Style::default().bg(Color::Cyan);
        let cyan_fg = Style::default().fg(Color::Cyan);

        // Behavior: the last one set is the definitive one
        assert_eq!("hello".on_red().on_cyan(), Span::styled("hello", cyan_bg));
        assert_eq!("hello".red().cyan(), Span::styled("hello", cyan_fg));
    }

    #[test]
    fn all_chained() {
        let all_modifier_black = Style::default()
            .bg(Color::Black)
            .fg(Color::Black)
            .add_modifier(
                Modifier::UNDERLINED
                    | Modifier::BOLD
                    | Modifier::DIM
                    | Modifier::SLOW_BLINK
                    | Modifier::REVERSED
                    | Modifier::CROSSED_OUT,
            );
        assert_eq!(
            "hello"
                .on_black()
                .black()
                .bold()
                .underline()
                .dimmed()
                .slow_blink()
                .crossed_out()
                .reversed(),
            Span::styled("hello", all_modifier_black)
        );
    }
}
