#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Overflow<'a> {
    /// Overflow handling for the end of the line.
    end: OverflowKind<'a>,
    /// Overflow handling for the start of the line.
    start: OverflowKind<'a>,
}

impl<'a> Overflow<'a> {
    pub const CLIP: Self = Self {
        end: OverflowKind::Clip,
        start: OverflowKind::Clip,
    };

    pub const ELLIPSIS: Self = Self {
        end: OverflowKind::Ellipsis,
        start: OverflowKind::Clip,
    };

    pub const fn custom(text: &'a str) -> Self {
        Self {
            end: OverflowKind::Custom(text),
            start: OverflowKind::Clip,
        }
    }

    /// Convenience method that returns the end and start [`OverflowKind`] as a tuple.
    pub fn kinds(&self) -> (OverflowKind, OverflowKind) {
        (self.end.clone(), self.start.clone())
    }
}

impl<'a> From<OverflowKind<'a>> for Overflow<'a> {
    fn from(kind: OverflowKind<'a>) -> Self {
        Overflow {
            end: kind.clone(),
            start: OverflowKind::default(),
        }
    }
}

impl<'a> From<(OverflowKind<'a>, OverflowKind<'a>)> for Overflow<'a> {
    fn from(kinds: (OverflowKind<'a>, OverflowKind<'a>)) -> Self {
        Overflow {
            end: kinds.0.clone(),
            start: kinds.1.clone(),
        }
    }
}

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub enum OverflowKind<'a> {
    #[default]
    Clip,
    Ellipsis,
    Custom(&'a str),
}

impl OverflowKind<'_> {
    pub const fn symbol(&self) -> Option<&str> {
        match self {
            OverflowKind::Clip => None,
            OverflowKind::Ellipsis => Some("…"),
            OverflowKind::Custom(s) => Some(s),
        }
    }
}
