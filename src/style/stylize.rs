use paste::paste;

use crate::{
    style::{Color, Modifier, Style},
    text::Span,
};

/// A trait for objects that have a `Style`.
///
/// This trait enables generic code to be written that can interact with any object that has a
/// `Style`. This is used by the `Stylize` trait to allow generic code to be written that can
/// interact with any object that can be styled.
pub trait Styled {
    type Item;

    fn style(&self) -> Style;
    fn set_style(self, style: Style) -> Self::Item;
}

/// Generates two methods for each color, one for setting the foreground color (`red()`, `blue()`,
/// etc) and one for setting the background color (`on_red()`, `on_blue()`, etc.). Each method sets
/// the color of the style to the corresponding color.
///
/// ```rust,ignore
/// color!(black);
///
/// // generates
///
/// #[doc = "Sets the foreground color to [`black`](Color::Black)."]
/// fn black(self) -> T {
///     self.fg(Color::Black)
/// }
///
/// #[doc = "Sets the background color to [`black`](Color::Black)."]
/// fn on_black(self) -> T {
///     self.bg(Color::Black)
/// }
/// ```
macro_rules! color {
    ( $color:ident ) => {
        paste! {
            #[doc = "Sets the foreground color to [`" $color "`](Color::" $color:camel ")."]
            fn $color(self) -> T {
                self.fg(Color::[<$color:camel>])
            }

            #[doc = "Sets the background color to [`" $color "`](Color::" $color:camel ")."]
            fn [<on_ $color>](self) -> T {
                self.bg(Color::[<$color:camel>])
            }
        }
    };
}

/// Generates a method for a modifier (`bold()`, `italic()`, etc.). Each method sets the modifier
/// of the style to the corresponding modifier.
///
/// # Examples
///
/// ```rust,ignore
/// modifier!(bold);
///
/// // generates
///
/// #[doc = "Adds the [`BOLD`](Modifier::BOLD) modifier."]
/// fn bold(self) -> T {
///     self.add_modifier(Modifier::BOLD)
/// }
///
/// #[doc = "Removes the [`BOLD`](Modifier::BOLD) modifier."]
/// fn not_bold(self) -> T {
///     self.remove_modifier(Modifier::BOLD)
/// }
/// ```
macro_rules! modifier {
    ( $modifier:ident ) => {
        paste! {
            #[doc = "Adds the [`" $modifier:upper "`](Modifier::" $modifier:upper ") modifier."]
            fn [<$modifier>](self) -> T {
                self.add_modifier(Modifier::[<$modifier:upper>])
            }
        }

        paste! {
            #[doc = "Removes the [`" $modifier:upper "`](Modifier::" $modifier:upper ") modifier."]
            fn [<not_ $modifier>](self) -> T {
                self.remove_modifier(Modifier::[<$modifier:upper>])
            }
        }
    };
}

/// The trait that enables something to be have a style.
///
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
    fn bg(self, color: Color) -> T;
    fn fg<S: Into<Color>>(self, color: S) -> T;
    fn reset(self) -> T;
    fn add_modifier(self, modifier: Modifier) -> T;
    fn remove_modifier(self, modifier: Modifier) -> T;

    color!(black);
    color!(red);
    color!(green);
    color!(yellow);
    color!(blue);
    color!(magenta);
    color!(cyan);
    color!(gray);
    color!(dark_gray);
    color!(light_red);
    color!(light_green);
    color!(light_yellow);
    color!(light_blue);
    color!(light_magenta);
    color!(light_cyan);
    color!(white);

    modifier!(bold);
    modifier!(dim);
    modifier!(italic);
    modifier!(underlined);
    modifier!(slow_blink);
    modifier!(rapid_blink);
    modifier!(reversed);
    modifier!(hidden);
    modifier!(crossed_out);
}

impl<'a, T, U> Stylize<'a, T> for U
where
    U: Styled<Item = T>,
{
    fn bg(self, color: Color) -> T {
        let style = self.style().bg(color);
        self.set_style(style)
    }

    fn fg<S: Into<Color>>(self, color: S) -> T {
        let style = self.style().fg(color.into());
        self.set_style(style)
    }

    fn add_modifier(self, modifier: Modifier) -> T {
        let style = self.style().add_modifier(modifier);
        self.set_style(style)
    }

    fn remove_modifier(self, modifier: Modifier) -> T {
        let style = self.style().remove_modifier(modifier);
        self.set_style(style)
    }

    fn reset(self) -> T {
        self.set_style(Style::reset())
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
            "hello".on_cyan().light_red().bold().underlined().reset(),
            Span::styled("hello", Style::reset())
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
                .underlined()
                .dim()
                .slow_blink()
                .crossed_out()
                .reversed(),
            Span::styled("hello", all_modifier_black)
        );
    }
}
