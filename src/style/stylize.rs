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

    /// Returns the style of the object.
    fn style(&self) -> Style;

    /// Sets the style of the object.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item;
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
            #[must_use = concat!("`", stringify!($color), "` returns the modified style without modifying the original")]
            fn $color(self) -> T {
                self.fg(Color::[<$color:camel>])
            }

            #[doc = "Sets the background color to [`" $color "`](Color::" $color:camel ")."]
            #[must_use = concat!("`on_", stringify!($color), "` returns the modified style without modifying the original")]
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
            #[must_use = concat!("`", stringify!($modifier), "` returns the modified style without modifying the original")]
            fn [<$modifier>](self) -> T {
                self.add_modifier(Modifier::[<$modifier:upper>])
            }
        }

        paste! {
            #[doc = "Removes the [`" $modifier:upper "`](Modifier::" $modifier:upper ") modifier."]
            #[must_use = concat!("`not_", stringify!($modifier), "` returns the modified style without modifying the original")]
            fn [<not_ $modifier>](self) -> T {
                self.remove_modifier(Modifier::[<$modifier:upper>])
            }
        }
    };
}

/// An extension trait for styling objects.
///
/// For any type that implements `Stylize`, the provided methods in this trait can be used to style
/// the type further. This trait is automatically implemented for any type that implements the
/// [`Styled`] trait which e.g.: [`String`], [`&str`], [`Span`], [`Style`] and many Widget types.
///
/// This results in much more ergonomic styling of text and widgets. For example, instead of
/// writing:
///
/// ```rust,ignore
/// let text = Span::styled("Hello", Style::default().fg(Color::Red).bg(Color::Blue));
/// ```
///
/// You can write:
///
/// ```rust,ignore
/// let text = "Hello".red().on_blue();
/// ```
///
/// This trait implements a provided method for every color as both foreground and background
/// (prefixed by `on_`), and all modifiers as both an additive and subtractive modifier (prefixed
/// by `not_`). The `reset()` method is also provided to reset the style.
///
/// # Examples
/// ```
/// use ratatui::{prelude::*, widgets::*};
///
/// let span = "hello".red().on_blue().bold();
/// let line = Line::from(vec![
///     "hello".red().on_blue().bold(),
///     "world".green().on_yellow().not_bold(),
/// ]);
/// let paragraph = Paragraph::new(line).italic().underlined();
/// let block = Block::bordered().title("Title").on_white().bold();
/// ```
pub trait Stylize<'a, T>: Sized {
    #[must_use = "`bg` returns the modified style without modifying the original"]
    fn bg(self, color: Color) -> T;
    #[must_use = "`fg` returns the modified style without modifying the original"]
    fn fg<S: Into<Color>>(self, color: S) -> T;
    #[must_use = "`reset` returns the modified style without modifying the original"]
    fn reset(self) -> T;
    #[must_use = "`add_modifier` returns the modified style without modifying the original"]
    fn add_modifier(self, modifier: Modifier) -> T;
    #[must_use = "`remove_modifier` returns the modified style without modifying the original"]
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

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        Span::styled(self, style)
    }
}

impl Styled for String {
    type Item = Span<'static>;

    fn style(&self) -> Style {
        Style::default()
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        Span::styled(self, style)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn str_styled() {
        assert_eq!("hello".style(), Style::default());
        assert_eq!(
            "hello".set_style(Style::new().cyan()),
            Span::styled("hello", Style::new().cyan())
        );
        assert_eq!("hello".black(), Span::from("hello").black());
        assert_eq!("hello".red(), Span::from("hello").red());
        assert_eq!("hello".green(), Span::from("hello").green());
        assert_eq!("hello".yellow(), Span::from("hello").yellow());
        assert_eq!("hello".blue(), Span::from("hello").blue());
        assert_eq!("hello".magenta(), Span::from("hello").magenta());
        assert_eq!("hello".cyan(), Span::from("hello").cyan());
        assert_eq!("hello".gray(), Span::from("hello").gray());
        assert_eq!("hello".dark_gray(), Span::from("hello").dark_gray());
        assert_eq!("hello".light_red(), Span::from("hello").light_red());
        assert_eq!("hello".light_green(), Span::from("hello").light_green());
        assert_eq!("hello".light_yellow(), Span::from("hello").light_yellow());
        assert_eq!("hello".light_blue(), Span::from("hello").light_blue());
        assert_eq!("hello".light_magenta(), Span::from("hello").light_magenta());
        assert_eq!("hello".light_cyan(), Span::from("hello").light_cyan());
        assert_eq!("hello".white(), Span::from("hello").white());

        assert_eq!("hello".on_black(), Span::from("hello").on_black());
        assert_eq!("hello".on_red(), Span::from("hello").on_red());
        assert_eq!("hello".on_green(), Span::from("hello").on_green());
        assert_eq!("hello".on_yellow(), Span::from("hello").on_yellow());
        assert_eq!("hello".on_blue(), Span::from("hello").on_blue());
        assert_eq!("hello".on_magenta(), Span::from("hello").on_magenta());
        assert_eq!("hello".on_cyan(), Span::from("hello").on_cyan());
        assert_eq!("hello".on_gray(), Span::from("hello").on_gray());
        assert_eq!("hello".on_dark_gray(), Span::from("hello").on_dark_gray());
        assert_eq!("hello".on_light_red(), Span::from("hello").on_light_red());
        assert_eq!(
            "hello".on_light_green(),
            Span::from("hello").on_light_green()
        );
        assert_eq!(
            "hello".on_light_yellow(),
            Span::from("hello").on_light_yellow()
        );
        assert_eq!("hello".on_light_blue(), Span::from("hello").on_light_blue());
        assert_eq!(
            "hello".on_light_magenta(),
            Span::from("hello").on_light_magenta()
        );
        assert_eq!("hello".on_light_cyan(), Span::from("hello").on_light_cyan());
        assert_eq!("hello".on_white(), Span::from("hello").on_white());

        assert_eq!("hello".bold(), Span::from("hello").bold());
        assert_eq!("hello".dim(), Span::from("hello").dim());
        assert_eq!("hello".italic(), Span::from("hello").italic());
        assert_eq!("hello".underlined(), Span::from("hello").underlined());
        assert_eq!("hello".slow_blink(), Span::from("hello").slow_blink());
        assert_eq!("hello".rapid_blink(), Span::from("hello").rapid_blink());
        assert_eq!("hello".reversed(), Span::from("hello").reversed());
        assert_eq!("hello".hidden(), Span::from("hello").hidden());
        assert_eq!("hello".crossed_out(), Span::from("hello").crossed_out());

        assert_eq!("hello".not_bold(), Span::from("hello").not_bold());
        assert_eq!("hello".not_dim(), Span::from("hello").not_dim());
        assert_eq!("hello".not_italic(), Span::from("hello").not_italic());
        assert_eq!(
            "hello".not_underlined(),
            Span::from("hello").not_underlined()
        );
        assert_eq!(
            "hello".not_slow_blink(),
            Span::from("hello").not_slow_blink()
        );
        assert_eq!(
            "hello".not_rapid_blink(),
            Span::from("hello").not_rapid_blink()
        );
        assert_eq!("hello".not_reversed(), Span::from("hello").not_reversed());
        assert_eq!("hello".not_hidden(), Span::from("hello").not_hidden());
        assert_eq!(
            "hello".not_crossed_out(),
            Span::from("hello").not_crossed_out()
        );

        assert_eq!("hello".reset(), Span::from("hello").reset());
    }

    #[test]
    fn string_styled() {
        let s = String::from("hello");
        assert_eq!(s.style(), Style::default());
        assert_eq!(
            s.clone().set_style(Style::new().cyan()),
            Span::styled("hello", Style::new().cyan())
        );
        assert_eq!(s.clone().black(), Span::from("hello").black());
        assert_eq!(s.clone().on_black(), Span::from("hello").on_black());
        assert_eq!(s.clone().bold(), Span::from("hello").bold());
        assert_eq!(s.clone().not_bold(), Span::from("hello").not_bold());
        assert_eq!(s.clone().reset(), Span::from("hello").reset());
    }

    #[test]
    fn temporary_string_styled() {
        // to_string() is used to create a temporary String, which is then styled. Without the
        // `Styled` trait impl for `String`, this would fail to compile with the error: "temporary
        // value dropped while borrowed"
        let s = "hello".to_string().red();
        assert_eq!(s, Span::from("hello").red());

        // format!() is used to create a temporary String inside a closure, which suffers the same
        // issue as above without the `Styled` trait impl for `String`
        let items = [String::from("a"), String::from("b")];
        let sss = items.iter().map(|s| format!("{s}{s}").red()).collect_vec();
        assert_eq!(sss, vec![Span::from("aa").red(), Span::from("bb").red()]);
    }

    #[test]
    fn reset() {
        assert_eq!(
            "hello".on_cyan().light_red().bold().underlined().reset(),
            Span::styled("hello", Style::reset())
        );
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

        assert_eq!("hello".cyan().bold(), Span::styled("hello", cyan_bold));
    }

    #[test]
    fn fg_bg() {
        let cyan_fg_bg = Style::default().bg(Color::Cyan).fg(Color::Cyan);

        assert_eq!("hello".cyan().on_cyan(), Span::styled("hello", cyan_fg_bg));
    }

    #[test]
    fn repeated_attributes() {
        let bg = Style::default().bg(Color::Cyan);
        let fg = Style::default().fg(Color::Cyan);

        // Behavior: the last one set is the definitive one
        assert_eq!("hello".on_red().on_cyan(), Span::styled("hello", bg));
        assert_eq!("hello".red().cyan(), Span::styled("hello", fg));
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
