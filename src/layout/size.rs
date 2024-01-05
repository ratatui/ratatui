/// A simple size struct
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl From<(u16, u16)> for Size {
    fn from((width, height): (u16, u16)) -> Self {
        Size { width, height }
    }
}
