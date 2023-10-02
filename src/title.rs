//! This module holds the [`Title`] element and its related configuration types.
//! A title is a piece of [`Block`](crate::widgets::Block) configuration.

use strum::{Display, EnumString};

use crate::{layout::Alignment, text::Line};

/// A [`Block`](crate::widgets::Block) title.
///
/// It can be aligned (see [`Alignment`]) and positioned (see [`Position`]).
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
/// Title::from(
///     Line::from(vec!["Q".white().underlined(), "uit".gray()])
/// );
/// ```
///
/// Complete example
/// ```
/// use ratatui::{prelude::*, widgets::{*, block::*}};
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
/// use ratatui::widgets::{*, block::*};
///
/// Block::new().title(
///     Title::from("title").position(Position::Bottom)
/// );
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

impl<'a> Title<'a> {
    /// Builder pattern method for setting the title content.
    pub fn content<T>(mut self, content: T) -> Title<'a>
    where
        T: Into<Line<'a>>,
    {
        self.content = content.into();
        self
    }

    /// Builder pattern method for setting the title alignment.
    pub fn alignment(mut self, alignment: Alignment) -> Title<'a> {
        self.alignment = Some(alignment);
        self
    }

    /// Builder pattern method for setting the title position.
    pub fn position(mut self, position: Position) -> Title<'a> {
        self.position = Some(position);
        self
    }
}

impl<'a, T> From<T> for Title<'a>
where
    T: Into<Line<'a>>,
{
    fn from(value: T) -> Self {
        Self::default().content(value.into())
    }
}

#[cfg(test)]
mod tests {
    use strum::ParseError;

    use super::*;

    #[test]
    fn position_tostring() {
        assert_eq!(Position::Top.to_string(), "Top");
        assert_eq!(Position::Bottom.to_string(), "Bottom");
    }

    #[test]
    fn position_from_str() {
        assert_eq!("Top".parse::<Position>(), Ok(Position::Top));
        assert_eq!("Bottom".parse::<Position>(), Ok(Position::Bottom));
        assert_eq!("".parse::<Position>(), Err(ParseError::VariantNotFound));
    }
}
