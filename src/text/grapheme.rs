use crate::style::{Style, Styled};

/// A grapheme associated to a style.
/// Note that, although `StyledGrapheme` is the smallest divisible unit of text,
/// it actually is not a member of the text type hierarchy (`Text` -> `Line` -> `Span`).
/// It is a separate type used mostly for rendering purposes. A `Span` consists of components that
/// can be split into `StyledGrapheme`s, but it does not contain a collection of `StyledGrapheme`s.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
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
