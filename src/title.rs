use strum::{Display, EnumString};

use crate::{layout::Alignment, text::Line};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Title<'a> {
    pub content: Line<'a>,
    /// Defaults to Left if unset
    pub alignment: Option<Alignment>,

    /// Defaults to Top if unset
    pub position: Option<Position>,
}

#[derive(Debug, Default, Display, EnumString, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    #[default]
    Top,
    Bottom,
}

impl<'a> Title<'a> {
    pub fn content<T>(mut self, content: T) -> Title<'a>
    where
        T: Into<Line<'a>>,
    {
        self.content = content.into();
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Title<'a> {
        self.alignment = Some(alignment);
        self
    }

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
