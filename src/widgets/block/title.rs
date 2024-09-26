//! This module holds the [`Title`] element and its related configuration types.
//! A title is a piece of [`Block`](crate::widgets::Block) configuration.

use strum::{Display, EnumString};

use crate::{layout::Alignment, text::Line};

/// A [`Block`](crate::widgets::Block) title.
///
/// It can be aligned (see [`Alignment`]) and positioned (see [`Position`]).
///
/// # Future Deprecation
///
/// This type is deprecated and will be removed in a future release. The reason for this is that the
/// position of the title should be stored in the block itself, not in the title. The `Line` type
/// has an alignment method that can be used to align the title. For more information see
/// <https://github.com/ratatui/ratatui/issues/738>.
///
/// Use [`Line`] instead, when the position is not defined as part of the title. When a specific
/// position is needed, use [`Block::title_top`](crate::widgets::Block::title_top) or
/// [`Block::title_bottom`](crate::widgets::Block::title_bottom) instead.
///
/// # Example
///
/// Title with no style.
/// ```
/// use ratatui::widgets::block::Title;
///
/// Title::from("Title");
/// ```
///
/// Blue title on a white background (via [`Stylize`](crate::style::Stylize) trait).
/// ```
/// use ratatui::{prelude::*, widgets::block::*};
///
/// Title::from("Title".blue().on_white());
/// ```
///
/// Title with multiple styles (see [`Line`] and [`Stylize`](crate::style::Stylize)).
/// ```
/// use ratatui::{prelude::*, widgets::block::*};
///
/// Title::from(Line::from(vec!["Q".white().underlined(), "uit".gray()]));
/// ```
///
/// Complete example
/// ```
/// use ratatui::{
///     prelude::*,
///     widgets::{
///         block::{Position, Title},
///         Block,
///     },
/// };
///
/// Title::from("Title")
///     .position(Position::Top)
///     .alignment(Alignment::Right);
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Title<'a> {
    /// Title content
    pub content: Line<'a>,
    /// Title alignment
    ///
    /// If [`None`], defaults to the alignment defined with
    /// [`Block::title_alignment`](crate::widgets::Block::title_alignment) in the associated
    /// [`Block`](crate::widgets::Block).
    pub alignment: Option<Alignment>,

    /// Title position
    ///
    /// If [`None`], defaults to the position defined with
    /// [`Block::title_position`](crate::widgets::Block::title_position) in the associated
    /// [`Block`](crate::widgets::Block).
    pub position: Option<Position>,
}

/// Defines the [title](crate::widgets::block::Title) position.
///
/// The title can be positioned on top or at the bottom of the block.
/// Defaults to [`Position::Top`].
///
/// # Example
///
/// ```
/// use ratatui::widgets::{block::*, *};
///
/// Block::new().title(Title::from("title").position(Position::Bottom));
/// ```
#[derive(Debug, Default, Display, EnumString, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    /// Position the title at the top of the block.
    ///
    /// This is the default.
    #[default]
    Top,
    /// Position the title at the bottom of the block.
    Bottom,
}

#[deprecated = "use Block::title_top() or Block::title_bottom() instead. This will be removed in a future release."]
impl<'a> Title<'a> {
    /// Set the title content.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn content<T>(mut self, content: T) -> Self
    where
        T: Into<Line<'a>>,
    {
        self.content = content.into();
        self
    }

    /// Set the title alignment.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = Some(alignment);
        self
    }

    /// Set the title position.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }
}

impl<'a, T> From<T> for Title<'a>
where
    T: Into<Line<'a>>,
{
    fn from(value: T) -> Self {
        let content = value.into();
        let alignment = content.alignment;
        Self {
            content,
            alignment,
            position: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use strum::ParseError;

    use super::*;

    #[test]
    fn position_to_string() {
        assert_eq!(Position::Top.to_string(), "Top");
        assert_eq!(Position::Bottom.to_string(), "Bottom");
    }

    #[test]
    fn position_from_str() {
        assert_eq!("Top".parse::<Position>(), Ok(Position::Top));
        assert_eq!("Bottom".parse::<Position>(), Ok(Position::Bottom));
        assert_eq!("".parse::<Position>(), Err(ParseError::VariantNotFound));
    }

    #[test]
    fn title_from_line() {
        let title = Title::from(Line::raw("Title"));
        assert_eq!(title.content, Line::from("Title"));
        assert_eq!(title.alignment, None);
        assert_eq!(title.position, None);
    }

    #[rstest]
    #[case::left(Alignment::Left)]
    #[case::center(Alignment::Center)]
    #[case::right(Alignment::Right)]
    fn title_from_line_with_alignment(#[case] alignment: Alignment) {
        let line = Line::raw("Title").alignment(alignment);
        let title = Title::from(line.clone());
        assert_eq!(title.content, line);
        assert_eq!(title.alignment, Some(alignment));
        assert_eq!(title.position, None);
    }
}
