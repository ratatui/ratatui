use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Offset, Position, Rect};
use ratatui_core::style::Style;

/// TODO: docs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shadow {
    shadow_type: ShadowType,
    style: Style,
    offset: Offset,
}

impl Shadow {
    /// TODO: docs
    pub fn new(shadow_type: ShadowType) -> Self {
        Self {
            shadow_type,
            style: Style::default(),
            offset: Offset::new(1, 1),
        }
    }

    /// TODO: docs
    pub fn block() -> Self {
        Self::new(ShadowType::Block)
    }

    /// TODO: docs
    pub fn light_shade() -> Self {
        Self::new(ShadowType::LightShade)
    }

    /// TODO: docs
    pub fn medium_shade() -> Self {
        Self::new(ShadowType::MediumShade)
    }

    /// TODO: docs
    pub fn dark_shade() -> Self {
        Self::new(ShadowType::DarkShade)
    }

    /// TODO: docs
    #[must_use]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// TODO: docs
    #[must_use]
    pub const fn offset(mut self, offset: Offset) -> Self {
        self.offset = offset;
        self
    }

    /// TODO: docs
    pub fn render(&self, base_area: Rect, buf: &mut Buffer) {
        let shadow_area = base_area.offset(self.offset).intersection(buf.area);
        let symbol = self.shadow_type.get_char();

        for y in shadow_area.top()..shadow_area.bottom() {
            for x in shadow_area.left()..shadow_area.right() {
                if base_area.contains(Position { x, y }) {
                    continue;
                }
                if symbol != ' ' {
                    buf[(x, y)].set_char(symbol);
                }
                buf[(x, y)].set_style(self.style);
            }
        }
    }
}

impl Default for Shadow {
    fn default() -> Self {
        Self::new(ShadowType::default())
    }
}

/// TODO: docs
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShadowType {
    /// Only apply style on shadow area, don't change existing characters.
    #[default]
    Overlay,

    /// ```text
    /// ┌Box──┐
    /// │     │█
    /// └─────┘█
    ///  ███████
    /// ```
    Block,

    /// ```text
    /// ┌Box──┐
    /// │     │░
    /// └─────┘░
    ///  ░░░░░░░
    /// ```
    LightShade,

    /// ```text
    /// ┌Box──┐
    /// │     │▒
    /// └─────┘▒
    ///  ▒▒▒▒▒▒▒
    /// ```
    MediumShade,

    /// ```text
    /// ┌Box──┐
    /// │     │▓
    /// └─────┘▓
    ///  ▓▓▓▓▓▓▓
    /// ```
    DarkShade,

    /// TODO: docs
    Custom(char),
}

impl ShadowType {
    /// TODO: docs
    pub const fn get_char(self) -> char {
        match self {
            Self::Overlay => ' ',
            Self::Block => '█',
            Self::LightShade => '░',
            Self::MediumShade => '▒',
            Self::DarkShade => '▓',
            Self::Custom(s) => s,
        }
    }
}
