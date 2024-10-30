//! `style` contains the primitives used to control how your user interface will look.
//!
//! There are two ways to set styles:
//! - Creating and using the [`Style`] struct. (e.g. `Style::new().fg(Color::Red)`).
//! - Using style shorthands. (e.g. `"hello".red()`).
//!
//! # Using the `Style` struct
//!
//! This is the original approach to styling and likely the most common. This is useful when
//! creating style variables to reuse, however the shorthands are often more convenient and
//! readable for most use cases.
//!
//! ## Example
//!
//! ```
//! use ratatui::{
//!     style::{Color, Modifier, Style},
//!     text::Span,
//! };
//!
//! let heading_style = Style::new()
//!     .fg(Color::Black)
//!     .bg(Color::Green)
//!     .add_modifier(Modifier::ITALIC | Modifier::BOLD);
//! let span = Span::styled("hello", heading_style);
//! ```
//!
//! # Using style shorthands
//!
//! Originally Ratatui only had the ability to set styles using the `Style` struct. This is still
//! supported, but there are now shorthands for all the styles that can be set. These save you from
//! having to create a `Style` struct every time you want to set a style.
//!
//! The shorthands are implemented in the [`Stylize`] trait which is automatically implemented for
//! many types via the [`Styled`] trait. This means that you can use the shorthands on any type
//! that implements [`Styled`]. E.g.:
//! - Strings and string slices when styled return a [`Span`]
//! - [`Span`]s can be styled again, which will merge the styles.
//! - Many widget types can be styled directly rather than calling their `style()` method.
//!
//! See the [`Stylize`] and [`Styled`] traits for more information.
//!
//! ## Example
//!
//! ```
//! use ratatui::{
//!     style::{Color, Modifier, Style, Stylize},
//!     text::Span,
//!     widgets::Paragraph,
//! };
//!
//! assert_eq!(
//!     "hello".red().on_blue().bold(),
//!     Span::styled(
//!         "hello",
//!         Style::default()
//!             .fg(Color::Red)
//!             .bg(Color::Blue)
//!             .add_modifier(Modifier::BOLD)
//!     )
//! );
//!
//! assert_eq!(
//!     Paragraph::new("hello").red().on_blue().bold(),
//!     Paragraph::new("hello").style(
//!         Style::default()
//!             .fg(Color::Red)
//!             .bg(Color::Blue)
//!             .add_modifier(Modifier::BOLD)
//!     )
//! );
//! ```
//!
//! [`Span`]: crate::text::Span

use std::fmt;

use bitflags::bitflags;
pub use color::{Color, ParseColorError};
use stylize::ColorDebugKind;
pub use stylize::{Styled, Stylize};

mod color;
pub mod palette;
#[cfg(feature = "palette")]
mod palette_conversion;
mod stylize;

bitflags! {
    /// Modifier changes the way a piece of text is displayed.
    ///
    /// They are bitflags so they can easily be composed.
    ///
    /// `From<Modifier> for Style` is implemented so you can use `Modifier` anywhere that accepts
    /// `Into<Style>`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ratatui::style::Modifier;
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
        write!(f, "{}", self.0)
    }
}

/// Style lets you control the main characteristics of the displayed elements.
///
/// ```rust
/// use ratatui::style::{Color, Modifier, Style};
///
/// Style::default()
///     .fg(Color::Black)
///     .bg(Color::Green)
///     .add_modifier(Modifier::ITALIC | Modifier::BOLD);
/// ```
///
/// Styles can also be created with a [shorthand notation](crate::style#using-style-shorthands).
///
/// ```rust
/// use ratatui::style::{Style, Stylize};
///
/// Style::new().black().on_green().italic().bold();
/// ```
///
/// For more information about the style shorthands, see the [`Stylize`] trait.
///
/// We implement conversions from [`Color`] and [`Modifier`] to [`Style`] so you can use them
/// anywhere that accepts `Into<Style>`.
///
/// ```rust
/// use ratatui::{
///     style::{Color, Modifier, Style},
///     text::Line,
/// };
///
/// Line::styled("hello", Style::new().fg(Color::Red));
/// // simplifies to
/// Line::styled("hello", Color::Red);
///
/// Line::styled("hello", Style::new().add_modifier(Modifier::BOLD));
/// // simplifies to
/// Line::styled("hello", Modifier::BOLD);
/// ```
///
/// Styles represents an incremental change. If you apply the styles S1, S2, S3 to a cell of the
/// terminal buffer, the style of this cell will be the result of the merge of S1, S2 and S3, not
/// just S3.
///
/// ```rust
/// use ratatui::{
///     buffer::Buffer,
///     layout::Rect,
///     style::{Color, Modifier, Style},
/// };
///
/// let styles = [
///     Style::default()
///         .fg(Color::Blue)
///         .add_modifier(Modifier::BOLD | Modifier::ITALIC),
///     Style::default()
///         .bg(Color::Red)
///         .add_modifier(Modifier::UNDERLINED),
///     #[cfg(feature = "underline-color")]
///     Style::default().underline_color(Color::Green),
///     Style::default()
///         .fg(Color::Yellow)
///         .remove_modifier(Modifier::ITALIC),
/// ];
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));
/// for style in &styles {
///     buffer[(0, 0)].set_style(*style);
/// }
/// assert_eq!(
///     Style {
///         fg: Some(Color::Yellow),
///         bg: Some(Color::Red),
///         #[cfg(feature = "underline-color")]
///         underline_color: Some(Color::Green),
///         add_modifier: Modifier::BOLD | Modifier::UNDERLINED,
///         sub_modifier: Modifier::empty(),
///     },
///     buffer[(0, 0)].style(),
/// );
/// ```
///
/// The default implementation returns a `Style` that does not modify anything. If you wish to
/// reset all properties until that point use [`Style::reset`].
///
/// ```
/// use ratatui::{
///     buffer::Buffer,
///     layout::Rect,
///     style::{Color, Modifier, Style},
/// };
///
/// let styles = [
///     Style::default()
///         .fg(Color::Blue)
///         .add_modifier(Modifier::BOLD | Modifier::ITALIC),
///     Style::reset().fg(Color::Yellow),
/// ];
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));
/// for style in &styles {
///     buffer[(0, 0)].set_style(*style);
/// }
/// assert_eq!(
///     Style {
///         fg: Some(Color::Yellow),
///         bg: Some(Color::Reset),
///         #[cfg(feature = "underline-color")]
///         underline_color: Some(Color::Reset),
///         add_modifier: Modifier::empty(),
///         sub_modifier: Modifier::empty(),
///     },
///     buffer[(0, 0)].style(),
/// );
/// ```
#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    #[cfg(feature = "underline-color")]
    pub underline_color: Option<Color>,
    pub add_modifier: Modifier,
    pub sub_modifier: Modifier,
}

/// A custom debug implementation that prints only the fields that are not the default, and unwraps
/// the `Option`s.
impl fmt::Debug for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Style::new()")?;
        self.fmt_stylize(f)?;
        Ok(())
    }
}

impl Styled for Style {
    type Item = Self;

    fn style(&self) -> Style {
        *self
    }

    fn set_style<S: Into<Self>>(self, style: S) -> Self::Item {
        self.patch(style)
    }
}

impl Style {
    pub const fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            #[cfg(feature = "underline-color")]
            underline_color: None,
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::empty(),
        }
    }

    /// Returns a `Style` resetting all properties.
    pub const fn reset() -> Self {
        Self {
            fg: Some(Color::Reset),
            bg: Some(Color::Reset),
            #[cfg(feature = "underline-color")]
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
    /// use ratatui::style::{Color, Style};
    ///
    /// let style = Style::default().fg(Color::Blue);
    /// let diff = Style::default().fg(Color::Red);
    /// assert_eq!(style.patch(diff), Style::default().fg(Color::Red));
    /// ```
    #[must_use = "`fg` returns the modified style without modifying the original"]
    pub const fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Changes the background color.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ratatui::style::{Color, Style};
    ///
    /// let style = Style::default().bg(Color::Blue);
    /// let diff = Style::default().bg(Color::Red);
    /// assert_eq!(style.patch(diff), Style::default().bg(Color::Red));
    /// ```
    #[must_use = "`bg` returns the modified style without modifying the original"]
    pub const fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Changes the underline color. The text must be underlined with a modifier for this to work.
    ///
    /// This uses a non-standard ANSI escape sequence. It is supported by most terminal emulators,
    /// but is only implemented in the crossterm backend and enabled by the `underline-color`
    /// feature flag.
    ///
    /// See
    /// [Wikipedia](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters)
    /// code `58` and `59` for more information.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ratatui::style::{Color, Modifier, Style};
    ///
    /// let style = Style::default()
    ///     .underline_color(Color::Blue)
    ///     .add_modifier(Modifier::UNDERLINED);
    /// let diff = Style::default()
    ///     .underline_color(Color::Red)
    ///     .add_modifier(Modifier::UNDERLINED);
    /// assert_eq!(
    ///     style.patch(diff),
    ///     Style::default()
    ///         .underline_color(Color::Red)
    ///         .add_modifier(Modifier::UNDERLINED)
    /// );
    /// ```
    #[cfg(feature = "underline-color")]
    #[must_use = "`underline_color` returns the modified style without modifying the original"]
    pub const fn underline_color(mut self, color: Color) -> Self {
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
    /// use ratatui::style::{Modifier, Style};
    ///
    /// let style = Style::default().add_modifier(Modifier::BOLD);
    /// let diff = Style::default().add_modifier(Modifier::ITALIC);
    /// let patched = style.patch(diff);
    /// assert_eq!(patched.add_modifier, Modifier::BOLD | Modifier::ITALIC);
    /// assert_eq!(patched.sub_modifier, Modifier::empty());
    /// ```
    #[must_use = "`add_modifier` returns the modified style without modifying the original"]
    pub const fn add_modifier(mut self, modifier: Modifier) -> Self {
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
    /// use ratatui::style::{Modifier, Style};
    ///
    /// let style = Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC);
    /// let diff = Style::default().remove_modifier(Modifier::ITALIC);
    /// let patched = style.patch(diff);
    /// assert_eq!(patched.add_modifier, Modifier::BOLD);
    /// assert_eq!(patched.sub_modifier, Modifier::ITALIC);
    /// ```
    #[must_use = "`remove_modifier` returns the modified style without modifying the original"]
    pub const fn remove_modifier(mut self, modifier: Modifier) -> Self {
        self.add_modifier = self.add_modifier.difference(modifier);
        self.sub_modifier = self.sub_modifier.union(modifier);
        self
    }

    /// Results in a combined style that is equivalent to applying the two individual styles to
    /// a style one after the other.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// ## Examples
    /// ```
    /// use ratatui::style::{Color, Modifier, Style};
    ///
    /// let style_1 = Style::default().fg(Color::Yellow);
    /// let style_2 = Style::default().bg(Color::Red);
    /// let combined = style_1.patch(style_2);
    /// assert_eq!(
    ///     Style::default().patch(style_1).patch(style_2),
    ///     Style::default().patch(combined)
    /// );
    /// ```
    #[must_use = "`patch` returns the modified style without modifying the original"]
    pub fn patch<S: Into<Self>>(mut self, other: S) -> Self {
        let other = other.into();
        self.fg = other.fg.or(self.fg);
        self.bg = other.bg.or(self.bg);

        #[cfg(feature = "underline-color")]
        {
            self.underline_color = other.underline_color.or(self.underline_color);
        }

        self.add_modifier.remove(other.sub_modifier);
        self.add_modifier.insert(other.add_modifier);
        self.sub_modifier.remove(other.add_modifier);
        self.sub_modifier.insert(other.sub_modifier);

        self
    }

    /// Formats the style in a way that can be copy-pasted into code using the style shorthands.
    ///
    /// This is useful for debugging and for generating code snippets.
    pub(crate) fn fmt_stylize(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use fmt::Debug;
        if let Some(fg) = self.fg {
            fg.stylize_debug(ColorDebugKind::Foreground).fmt(f)?;
        }
        if let Some(bg) = self.bg {
            bg.stylize_debug(ColorDebugKind::Background).fmt(f)?;
        }
        #[cfg(feature = "underline-color")]
        if let Some(underline_color) = self.underline_color {
            underline_color
                .stylize_debug(ColorDebugKind::Underline)
                .fmt(f)?;
        }
        for modifier in self.add_modifier.iter() {
            match modifier {
                Modifier::BOLD => f.write_str(".bold()")?,
                Modifier::DIM => f.write_str(".dim()")?,
                Modifier::ITALIC => f.write_str(".italic()")?,
                Modifier::UNDERLINED => f.write_str(".underlined()")?,
                Modifier::SLOW_BLINK => f.write_str(".slow_blink()")?,
                Modifier::RAPID_BLINK => f.write_str(".rapid_blink()")?,
                Modifier::REVERSED => f.write_str(".reversed()")?,
                Modifier::HIDDEN => f.write_str(".hidden()")?,
                Modifier::CROSSED_OUT => f.write_str(".crossed_out()")?,
                _ => f.write_fmt(format_args!(".add_modifier(Modifier::{modifier:?})"))?,
            }
        }
        for modifier in self.sub_modifier.iter() {
            match modifier {
                Modifier::BOLD => f.write_str(".not_bold()")?,
                Modifier::DIM => f.write_str(".not_dim()")?,
                Modifier::ITALIC => f.write_str(".not_italic()")?,
                Modifier::UNDERLINED => f.write_str(".not_underlined()")?,
                Modifier::SLOW_BLINK => f.write_str(".not_slow_blink()")?,
                Modifier::RAPID_BLINK => f.write_str(".not_rapid_blink()")?,
                Modifier::REVERSED => f.write_str(".not_reversed()")?,
                Modifier::HIDDEN => f.write_str(".not_hidden()")?,
                Modifier::CROSSED_OUT => f.write_str(".not_crossed_out()")?,
                _ => f.write_fmt(format_args!(".remove_modifier(Modifier::{modifier:?})"))?,
            }
        }
        Ok(())
    }
}

impl From<Color> for Style {
    /// Creates a new `Style` with the given foreground color.
    ///
    /// To specify a foreground and background color, use the `from((fg, bg))` constructor.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::{Color, Style};
    ///
    /// let style = Style::from(Color::Red);
    /// ```
    fn from(color: Color) -> Self {
        Self::new().fg(color)
    }
}

impl From<(Color, Color)> for Style {
    /// Creates a new `Style` with the given foreground and background colors.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::{Color, Style};
    ///
    /// // red foreground, blue background
    /// let style = Style::from((Color::Red, Color::Blue));
    /// // default foreground, blue background
    /// let style = Style::from((Color::Reset, Color::Blue));
    /// ```
    fn from((fg, bg): (Color, Color)) -> Self {
        Self::new().fg(fg).bg(bg)
    }
}

impl From<Modifier> for Style {
    /// Creates a new `Style` with the given modifier added.
    ///
    /// To specify multiple modifiers, use the `|` operator.
    ///
    /// To specify modifiers to add and remove, use the `from((add_modifier, sub_modifier))`
    /// constructor.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::{Style, Modifier};
    ///
    /// // add bold and italic
    /// let style = Style::from(Modifier::BOLD|Modifier::ITALIC);
    fn from(modifier: Modifier) -> Self {
        Self::new().add_modifier(modifier)
    }
}

impl From<(Modifier, Modifier)> for Style {
    /// Creates a new `Style` with the given modifiers added and removed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::{Modifier, Style};
    ///
    /// // add bold and italic, remove dim
    /// let style = Style::from((Modifier::BOLD | Modifier::ITALIC, Modifier::DIM));
    /// ```
    fn from((add_modifier, sub_modifier): (Modifier, Modifier)) -> Self {
        Self::new()
            .add_modifier(add_modifier)
            .remove_modifier(sub_modifier)
    }
}

impl From<(Color, Modifier)> for Style {
    /// Creates a new `Style` with the given foreground color and modifier added.
    ///
    /// To specify multiple modifiers, use the `|` operator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::{Color, Modifier, Style};
    ///
    /// // red foreground, add bold and italic
    /// let style = Style::from((Color::Red, Modifier::BOLD | Modifier::ITALIC));
    /// ```
    fn from((fg, modifier): (Color, Modifier)) -> Self {
        Self::new().fg(fg).add_modifier(modifier)
    }
}

impl From<(Color, Color, Modifier)> for Style {
    /// Creates a new `Style` with the given foreground and background colors and modifier added.
    ///
    /// To specify multiple modifiers, use the `|` operator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::{Color, Modifier, Style};
    ///
    /// // red foreground, blue background, add bold and italic
    /// let style = Style::from((Color::Red, Color::Blue, Modifier::BOLD | Modifier::ITALIC));
    /// ```
    fn from((fg, bg, modifier): (Color, Color, Modifier)) -> Self {
        Self::new().fg(fg).bg(bg).add_modifier(modifier)
    }
}

impl From<(Color, Color, Modifier, Modifier)> for Style {
    /// Creates a new `Style` with the given foreground and background colors and modifiers added
    /// and removed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::{Color, Modifier, Style};
    ///
    /// // red foreground, blue background, add bold and italic, remove dim
    /// let style = Style::from((
    ///     Color::Red,
    ///     Color::Blue,
    ///     Modifier::BOLD | Modifier::ITALIC,
    ///     Modifier::DIM,
    /// ));
    /// ```
    fn from((fg, bg, add_modifier, sub_modifier): (Color, Color, Modifier, Modifier)) -> Self {
        Self::new()
            .fg(fg)
            .bg(bg)
            .add_modifier(add_modifier)
            .remove_modifier(sub_modifier)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(Style::new(), "Style::new()")]
    #[case(Style::new().red(), "Style::new().red()")]
    #[case(Style::new().on_blue(), "Style::new().on_blue()")]
    #[case(Style::new().bold(), "Style::new().bold()")]
    #[case(Style::new().not_italic(), "Style::new().not_italic()")]
    #[case(
        Style::new().red().on_blue().bold().italic().not_dim().not_hidden(),
        "Style::new().red().on_blue().bold().italic().not_dim().not_hidden()"
    )]
    fn debug(#[case] style: Style, #[case] expected: &'static str) {
        assert_eq!(format!("{style:?}"), expected);
    }

    #[test]
    fn combined_patch_gives_same_result_as_individual_patch() {
        let styles = [
            Style::new(),
            Style::new().fg(Color::Yellow),
            Style::new().bg(Color::Yellow),
            Style::new().add_modifier(Modifier::BOLD),
            Style::new().remove_modifier(Modifier::BOLD),
            Style::new().add_modifier(Modifier::ITALIC),
            Style::new().remove_modifier(Modifier::ITALIC),
            Style::new().add_modifier(Modifier::ITALIC | Modifier::BOLD),
            Style::new().remove_modifier(Modifier::ITALIC | Modifier::BOLD),
        ];
        for &a in &styles {
            for &b in &styles {
                for &c in &styles {
                    for &d in &styles {
                        assert_eq!(
                            Style::new().patch(a).patch(b).patch(c).patch(d),
                            Style::new().patch(a.patch(b.patch(c.patch(d))))
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn combine_individual_modifiers() {
        use crate::{buffer::Buffer, layout::Rect};

        let mods = [
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

        for m in mods {
            buffer[(0, 0)].set_style(Style::reset());
            buffer[(0, 0)].set_style(Style::new().add_modifier(m));
            let style = buffer[(0, 0)].style();
            assert!(style.add_modifier.contains(m));
            assert!(!style.sub_modifier.contains(m));
        }
    }

    #[rstest]
    #[case(Modifier::empty(), "NONE")]
    #[case(Modifier::BOLD, "BOLD")]
    #[case(Modifier::DIM, "DIM")]
    #[case(Modifier::ITALIC, "ITALIC")]
    #[case(Modifier::UNDERLINED, "UNDERLINED")]
    #[case(Modifier::SLOW_BLINK, "SLOW_BLINK")]
    #[case(Modifier::RAPID_BLINK, "RAPID_BLINK")]
    #[case(Modifier::REVERSED, "REVERSED")]
    #[case(Modifier::HIDDEN, "HIDDEN")]
    #[case(Modifier::CROSSED_OUT, "CROSSED_OUT")]
    #[case(Modifier::BOLD | Modifier::DIM, "BOLD | DIM")]
    #[case(Modifier::all(), "BOLD | DIM | ITALIC | UNDERLINED | SLOW_BLINK | RAPID_BLINK | REVERSED | HIDDEN | CROSSED_OUT")]
    fn modifier_debug(#[case] modifier: Modifier, #[case] expected: &str) {
        assert_eq!(format!("{modifier:?}"), expected);
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
        );
    }

    #[rstest]
    #[case(Style::new().black(), Color::Black)]
    #[case(Style::new().red(), Color::Red)]
    #[case(Style::new().green(), Color::Green)]
    #[case(Style::new().yellow(), Color::Yellow)]
    #[case(Style::new().blue(), Color::Blue)]
    #[case(Style::new().magenta(), Color::Magenta)]
    #[case(Style::new().cyan(), Color::Cyan)]
    #[case(Style::new().white(), Color::White)]
    #[case(Style::new().gray(), Color::Gray)]
    #[case(Style::new().dark_gray(), Color::DarkGray)]
    #[case(Style::new().light_red(), Color::LightRed)]
    #[case(Style::new().light_green(), Color::LightGreen)]
    #[case(Style::new().light_yellow(), Color::LightYellow)]
    #[case(Style::new().light_blue(), Color::LightBlue)]
    #[case(Style::new().light_magenta(), Color::LightMagenta)]
    #[case(Style::new().light_cyan(), Color::LightCyan)]
    #[case(Style::new().white(), Color::White)]
    fn fg_can_be_stylized(#[case] stylized: Style, #[case] expected: Color) {
        assert_eq!(stylized, Style::new().fg(expected));
    }

    #[rstest]
    #[case(Style::new().on_black(), Color::Black)]
    #[case(Style::new().on_red(), Color::Red)]
    #[case(Style::new().on_green(), Color::Green)]
    #[case(Style::new().on_yellow(), Color::Yellow)]
    #[case(Style::new().on_blue(), Color::Blue)]
    #[case(Style::new().on_magenta(), Color::Magenta)]
    #[case(Style::new().on_cyan(), Color::Cyan)]
    #[case(Style::new().on_white(), Color::White)]
    #[case(Style::new().on_gray(), Color::Gray)]
    #[case(Style::new().on_dark_gray(), Color::DarkGray)]
    #[case(Style::new().on_light_red(), Color::LightRed)]
    #[case(Style::new().on_light_green(), Color::LightGreen)]
    #[case(Style::new().on_light_yellow(), Color::LightYellow)]
    #[case(Style::new().on_light_blue(), Color::LightBlue)]
    #[case(Style::new().on_light_magenta(), Color::LightMagenta)]
    #[case(Style::new().on_light_cyan(), Color::LightCyan)]
    #[case(Style::new().on_white(), Color::White)]
    fn bg_can_be_stylized(#[case] stylized: Style, #[case] expected: Color) {
        assert_eq!(stylized, Style::new().bg(expected));
    }

    #[rstest]
    #[case(Style::new().bold(), Modifier::BOLD)]
    #[case(Style::new().dim(), Modifier::DIM)]
    #[case(Style::new().italic(), Modifier::ITALIC)]
    #[case(Style::new().underlined(), Modifier::UNDERLINED)]
    #[case(Style::new().slow_blink(), Modifier::SLOW_BLINK)]
    #[case(Style::new().rapid_blink(), Modifier::RAPID_BLINK)]
    #[case(Style::new().reversed(), Modifier::REVERSED)]
    #[case(Style::new().hidden(), Modifier::HIDDEN)]
    #[case(Style::new().crossed_out(), Modifier::CROSSED_OUT)]
    fn add_modifier_can_be_stylized(#[case] stylized: Style, #[case] expected: Modifier) {
        assert_eq!(stylized, Style::new().add_modifier(expected));
    }

    #[rstest]
    #[case(Style::new().not_bold(), Modifier::BOLD)]
    #[case(Style::new().not_dim(), Modifier::DIM)]
    #[case(Style::new().not_italic(), Modifier::ITALIC)]
    #[case(Style::new().not_underlined(), Modifier::UNDERLINED)]
    #[case(Style::new().not_slow_blink(), Modifier::SLOW_BLINK)]
    #[case(Style::new().not_rapid_blink(), Modifier::RAPID_BLINK)]
    #[case(Style::new().not_reversed(), Modifier::REVERSED)]
    #[case(Style::new().not_hidden(), Modifier::HIDDEN)]
    #[case(Style::new().not_crossed_out(), Modifier::CROSSED_OUT)]
    fn remove_modifier_can_be_stylized(#[case] stylized: Style, #[case] expected: Modifier) {
        assert_eq!(stylized, Style::new().remove_modifier(expected));
    }

    #[test]
    fn reset_can_be_stylized() {
        assert_eq!(Style::new().reset(), Style::reset());
    }

    #[test]
    fn from_color() {
        assert_eq!(Style::from(Color::Red), Style::new().fg(Color::Red));
    }

    #[test]
    fn from_color_color() {
        assert_eq!(
            Style::from((Color::Red, Color::Blue)),
            Style::new().fg(Color::Red).bg(Color::Blue)
        );
    }

    #[test]
    fn from_modifier() {
        assert_eq!(
            Style::from(Modifier::BOLD | Modifier::ITALIC),
            Style::new()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC)
        );
    }

    #[test]
    fn from_modifier_modifier() {
        assert_eq!(
            Style::from((Modifier::BOLD | Modifier::ITALIC, Modifier::DIM)),
            Style::new()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC)
                .remove_modifier(Modifier::DIM)
        );
    }

    #[test]
    fn from_color_modifier() {
        assert_eq!(
            Style::from((Color::Red, Modifier::BOLD | Modifier::ITALIC)),
            Style::new()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC)
        );
    }

    #[test]
    fn from_color_color_modifier() {
        assert_eq!(
            Style::from((Color::Red, Color::Blue, Modifier::BOLD | Modifier::ITALIC)),
            Style::new()
                .fg(Color::Red)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC)
        );
    }

    #[test]
    fn from_color_color_modifier_modifier() {
        assert_eq!(
            Style::from((
                Color::Red,
                Color::Blue,
                Modifier::BOLD | Modifier::ITALIC,
                Modifier::DIM
            )),
            Style::new()
                .fg(Color::Red)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC)
                .remove_modifier(Modifier::DIM)
        );
    }
}
