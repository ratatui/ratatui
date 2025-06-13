use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Debug, Display, Formatter, Result};

use super::{Line, Span, Text, ToLine, ToSpan, ToText};
use crate::buffer::Buffer;
use crate::layout::Rect;
use crate::style::{Style, Styled};
use crate::widgets::Widget;

/// A wrapper arround three different ratatuis text types
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum UnifiedText<'a> {
    Span(Span<'a>),
    Line(Line<'a>),
    Text(Text<'a>),
}

impl<'a> UnifiedText<'a> {
    pub fn raw<T: Into<Cow<'a, str>>>(s: T) -> Self {
        let s = s.into();
        if s.contains('\n') {
            Self::Text(s.into())
        } else {
            Self::Span(s.into())
        }
    }

    pub fn height(&self) -> usize {
        if let Self::Text(text) = self {
            text.height()
        } else {
            1
        }
    }

    pub fn width(&self) -> usize {
        match self {
            UnifiedText::Span(span) => span.width(),
            UnifiedText::Line(line) => line.width(),
            UnifiedText::Text(text) => text.width(),
        }
    }
}

pub trait ToUnifiedSpan {
    fn to_unified_span(&self) -> UnifiedText<'_>;
}

impl<T: ToSpan> ToUnifiedSpan for T {
    fn to_unified_span(&self) -> UnifiedText<'_> {
        UnifiedText::Span(self.to_span())
    }
}

pub trait ToUnifiedLine {
    fn to_unified_line(&self) -> UnifiedText<'_>;
}

impl<T: ToLine> ToUnifiedLine for T {
    fn to_unified_line(&self) -> UnifiedText<'_> {
        UnifiedText::Line(self.to_line())
    }
}

pub trait ToUnifiedText {
    fn to_unified_text(&self) -> UnifiedText<'_>;
}

impl<T: ToText> ToUnifiedText for T {
    fn to_unified_text(&self) -> UnifiedText<'_> {
        UnifiedText::Text(self.to_text())
    }
}

impl Default for UnifiedText<'_> {
    fn default() -> Self {
        Self::Span(Span::default())
    }
}

impl<'a> From<Span<'a>> for UnifiedText<'a> {
    fn from(value: Span<'a>) -> Self {
        Self::Span(value)
    }
}

impl<'a> From<Line<'a>> for UnifiedText<'a> {
    fn from(value: Line<'a>) -> Self {
        Self::Line(value)
    }
}

impl<'a> From<Text<'a>> for UnifiedText<'a> {
    fn from(value: Text<'a>) -> Self {
        Self::Text(value)
    }
}

impl Styled for UnifiedText<'_> {
    type Item = Self;

    fn style(&self) -> Style {
        match self {
            UnifiedText::Span(span) => span.style(),
            UnifiedText::Line(line) => line.style(),
            UnifiedText::Text(text) => text.style(),
        }
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        match self {
            UnifiedText::Span(span) => UnifiedText::Span(span.set_style(style)),
            UnifiedText::Line(line) => UnifiedText::Line(line.set_style(style)),
            UnifiedText::Text(text) => UnifiedText::Text(text.set_style(style)),
        }
    }
}

impl Widget for &UnifiedText<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self {
            UnifiedText::Span(span) => span.render(area, buf),
            UnifiedText::Line(line) => line.render(area, buf),
            UnifiedText::Text(text) => text.render(area, buf),
        }
    }
}

impl Widget for UnifiedText<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self {
            UnifiedText::Span(span) => span.render(area, buf),
            UnifiedText::Line(line) => line.render(area, buf),
            UnifiedText::Text(text) => text.render(area, buf),
        }
    }
}

impl From<String> for UnifiedText<'_> {
    fn from(s: String) -> Self {
        Self::raw(s)
    }
}

impl<'a> From<&'a str> for UnifiedText<'a> {
    fn from(s: &'a str) -> Self {
        Self::raw(s)
    }
}

impl<'a> From<Cow<'a, str>> for UnifiedText<'a> {
    fn from(s: Cow<'a, str>) -> Self {
        Self::raw(s)
    }
}

impl<'a> From<Vec<Line<'a>>> for UnifiedText<'a> {
    fn from(lines: Vec<Line<'a>>) -> Self {
        Self::Text(lines.into())
    }
}

impl<'a> From<Vec<Span<'a>>> for UnifiedText<'a> {
    fn from(spans: Vec<Span<'a>>) -> Self {
        Self::Line(spans.into())
    }
}

impl<'a, T> FromIterator<T> for UnifiedText<'a>
where
    T: Into<Line<'a>>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::Text(Text::from_iter(iter))
    }
}

impl Display for UnifiedText<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            UnifiedText::Span(span) => Display::fmt(span, f),
            UnifiedText::Line(line) => Display::fmt(line, f),
            UnifiedText::Text(text) => Display::fmt(text, f),
        }
    }
}

impl<'a> From<UnifiedText<'a>> for Text<'a> {
    fn from(value: UnifiedText<'a>) -> Self {
        match value {
            UnifiedText::Span(span) => span.into(),
            UnifiedText::Line(line) => line.into(),
            UnifiedText::Text(text) => text,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::text::{ToUnifiedSpan, ToUnifiedText, UnifiedText};
    #[test]
    fn from_multiline_str() {
        let str = "Hello\nWorld!";
        assert_eq!(UnifiedText::raw(str), str.to_unified_text());
    }

    #[test]
    fn from_str() {
        let str = "Hello World!";
        assert_eq!(UnifiedText::raw(str), str.to_unified_span());
    }
}
