use crate::style::{Style, Styled};

/// A grapheme associated to a style.
/// Note that, although `StyledGrapheme` is the smallest divisible unit of text,
/// it actually is not a member of the text type hierarchy (`Text` -> `Line` -> `Span`).
/// It is a separate type used mostly for rendering purposes. A `Span` consists of components that
/// can be split into `StyledGrapheme`s, but it does not contain a collection of `StyledGrapheme`s.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct StyledGrapheme<'a> {
    pub symbol: &'a str,
    pub style: Style,
}

impl<'a> StyledGrapheme<'a> {
    pub fn new(symbol: &'a str, style: Style) -> StyledGrapheme<'a> {
        StyledGrapheme { symbol, style }
    }
}

impl<'a> Styled for StyledGrapheme<'a> {
    type Item = StyledGrapheme<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(mut self, style: Style) -> Self::Item {
        self.style = style;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn new() {
        let style = Style::new().yellow();
        let sg = StyledGrapheme::new("a", style);
        assert_eq!(sg.symbol, "a");
        assert_eq!(sg.style, style);
    }

    #[test]
    fn style() {
        let style = Style::new().yellow();
        let sg = StyledGrapheme::new("a", style);
        assert_eq!(sg.style(), style);
    }

    #[test]
    fn set_style() {
        let style = Style::new().yellow().on_red();
        let style2 = Style::new().green();
        let sg = StyledGrapheme::new("a", style).set_style(style2);
        assert_eq!(sg.style, style2);
    }

    #[test]
    fn stylize() {
        let style = Style::new().yellow().on_red();
        let sg = StyledGrapheme::new("a", style).green();
        assert_eq!(sg.style, Style::new().green().on_red());
    }
}
