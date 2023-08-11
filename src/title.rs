use crate::{layout::Alignment, text::Line};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Title<'a> {
    pub content: Line<'a>,
    /// Defaults to Left if unset
    pub alignment: Option<Alignment>,

    /// Defaults to Top if unset
    pub position: Option<Position>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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
