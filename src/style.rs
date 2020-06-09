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

    /// Creates a new [`Style`] by applying the given diff to its properties.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use tui::style::{Color, Modifier, Style, StyleDiff};
    /// let style = Style::default().fg(Color::Green).bg(Color::Black).modifier(Modifier::BOLD);
    ///
    /// let diff = StyleDiff::default();
    /// let patched = style.patch(diff);
    /// assert_eq!(patched, style);
    ///
    /// let diff = StyleDiff::default().fg(Color::Blue).add_modifier(Modifier::ITALIC);
    /// let patched = style.patch(diff);
    /// assert_eq!(
    ///     patched,
    ///     Style {
    ///         fg: Color::Blue,
    ///         bg: Color::Black,
    ///         modifier: Modifier::BOLD | Modifier::ITALIC,
    ///     }
    /// );
    /// ```
    pub fn patch(mut self, diff: StyleDiff) -> Style {
        if let Some(c) = diff.fg {
            self.fg = c;
        }
        if let Some(c) = diff.bg {
            self.bg = c;
        }
        if let Some(m) = diff.modifier {
            self.modifier = m;
        }
        self.modifier.insert(diff.add_modifier);
        self.modifier.remove(diff.sub_modifier);
        self
    }
}

/// StyleDiff is a set of updates that can be applied to a [`Style`] to get a
/// new one.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StyleDiff {
    fg: Option<Color>,
    bg: Option<Color>,
    modifier: Option<Modifier>,
    add_modifier: Modifier,
    sub_modifier: Modifier,
}

impl Default for StyleDiff {
    fn default() -> StyleDiff {
        StyleDiff {
            fg: None,
            bg: None,
            modifier: None,
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::empty(),
        }
    }
}

impl From<Style> for StyleDiff {
    fn from(s: Style) -> StyleDiff {
        StyleDiff {
            fg: Some(s.fg),
            bg: Some(s.bg),
            modifier: Some(s.modifier),
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::empty(),
        }
    }
}

impl StyleDiff {
    /// Changes the foreground color.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use tui::style::{Color, Style, StyleDiff};
    /// let style = Style::default().fg(Color::Blue);
    /// let diff = StyleDiff::default().fg(Color::Red);
    /// assert_eq!(style.patch(diff), Style::default().fg(Color::Red));
    /// ```
    pub fn fg(mut self, color: Color) -> StyleDiff {
        self.fg = Some(color);
        self
    }

    /// Changes the background color.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use tui::style::{Color, Style, StyleDiff};
    /// let style = Style::default().bg(Color::Blue);
    /// let diff = StyleDiff::default().bg(Color::Red);
    /// assert_eq!(style.patch(diff), Style::default().bg(Color::Red));
    /// ```
    pub fn bg(mut self, color: Color) -> StyleDiff {
        self.bg = Some(color);
        self
    }

    /// Changes the text emphasis.
    ///
    /// When applied, it replaces the `Style` modifier with the given value.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use tui::style::{Color, Modifier, Style, StyleDiff};
    /// let style = Style::default().modifier(Modifier::BOLD);
    /// let diff = StyleDiff::default().modifier(Modifier::ITALIC);
    /// assert_eq!(style.patch(diff), Style::default().modifier(Modifier::ITALIC));
    /// ```
    pub fn modifier(mut self, modifier: Modifier) -> StyleDiff {
        self.add_modifier = Modifier::empty();
        self.sub_modifier = Modifier::empty();
        self.modifier = Some(modifier);
        self
    }

    /// Changes the text emphasis.
    ///
    /// When applied, it adds the given modifiers to the `Style` modifier.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use tui::style::{Color, Modifier, Style, StyleDiff};
    /// let style = Style::default().modifier(Modifier::BOLD);
    /// let diff = StyleDiff::default().add_modifier(Modifier::ITALIC);
    /// assert_eq!(style.patch(diff), Style::default().modifier(Modifier::BOLD | Modifier::ITALIC));
    /// ```
    pub fn add_modifier(mut self, modifier: Modifier) -> StyleDiff {
        self.sub_modifier.remove(modifier);
        self.add_modifier.insert(modifier);
        self
    }

    /// Changes the text emphasis.
    ///
    /// When applied, it removes the given modifiers from the `Style` modifier.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use tui::style::{Color, Modifier, Style, StyleDiff};
    /// let style = Style::default().modifier(Modifier::BOLD | Modifier::ITALIC);
    /// let diff = StyleDiff::default().remove_modifier(Modifier::ITALIC);
    /// assert_eq!(style.patch(diff), Style::default().modifier(Modifier::BOLD));
    /// ```
    pub fn remove_modifier(mut self, modifier: Modifier) -> StyleDiff {
        self.add_modifier.remove(modifier);
        self.sub_modifier.insert(modifier);
        self
    }

    /// Results in a combined style diff that is equivalent to applying the two individual diffs to
    /// a style one after the other.
    ///
    /// ## Examples
    /// ```
    /// # use tui::style::{Color, Modifier, Style, StyleDiff};
    /// let style_1 = StyleDiff::default().fg(Color::Yellow);
    /// let style_2 = StyleDiff::default().bg(Color::Red);
    /// let combined = style_1.patch(style_2);
    /// assert_eq!(
    ///     Style::default().patch(style_1).patch(style_2),
    ///     Style::default().patch(combined));
    /// ```
    pub fn patch(mut self, other: StyleDiff) -> StyleDiff {
        self.fg = other.fg.or(self.fg);
        self.bg = other.bg.or(self.bg);
        self.modifier = other.modifier.or(self.modifier);

        // If the other is about to specify a full modifier, it would fully override whatever
        // add/sub modifiers the current style wants to apply so ignore those in that case.
        if other.modifier.is_some() {
            self.add_modifier = other.add_modifier;
            self.sub_modifier = other.sub_modifier;
        } else {
            self.add_modifier.remove(other.sub_modifier);
            self.add_modifier.insert(other.add_modifier);
            self.sub_modifier.remove(other.add_modifier);
            self.sub_modifier.insert(other.sub_modifier);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn diffs() -> Vec<StyleDiff> {
        vec![
            StyleDiff::default(),
            StyleDiff::default().fg(Color::Yellow),
            StyleDiff::default().bg(Color::Yellow),
            StyleDiff::default().modifier(Modifier::BOLD),
            StyleDiff::default().modifier(Modifier::ITALIC),
            StyleDiff::default().modifier(Modifier::ITALIC | Modifier::BOLD),
            StyleDiff::default().add_modifier(Modifier::BOLD),
            StyleDiff::default().remove_modifier(Modifier::BOLD),
            StyleDiff::default().add_modifier(Modifier::ITALIC),
            StyleDiff::default().remove_modifier(Modifier::ITALIC),
            StyleDiff::default().add_modifier(Modifier::ITALIC | Modifier::BOLD),
            StyleDiff::default().remove_modifier(Modifier::ITALIC | Modifier::BOLD),
        ]
    }

    #[test]
    fn combined_patch_gives_same_result_as_individual_patch() {
        let diffs = diffs();
        for &a in &diffs {
            for &b in &diffs {
                for &c in &diffs {
                    for &d in &diffs {
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
    fn diffs_respect_later_modifiers() {
        let diffs = diffs();
        for &a in &diffs {
            for &b in &diffs {
                let random_diff = a.patch(b);

                let set_bold = random_diff.modifier(Modifier::BOLD);
                assert_eq!(Style::default().patch(set_bold).modifier, Modifier::BOLD);

                let add_bold = random_diff.add_modifier(Modifier::BOLD);
                assert!(Style::default()
                    .patch(add_bold)
                    .modifier
                    .contains(Modifier::BOLD));

                let remove_bold = random_diff.remove_modifier(Modifier::BOLD);
                assert!(!Style::default()
                    .patch(remove_bold)
                    .modifier
                    .contains(Modifier::BOLD));
            }
        }
    }
}
