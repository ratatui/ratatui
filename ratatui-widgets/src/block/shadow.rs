use ratatui_core::layout::Offset;
use ratatui_core::style::Style;

/// TODO: docs
#[derive(Debug, Clone, Copy, PartialEq)]
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
}

impl Default for Shadow {
    fn default() -> Self {
        Self::new(ShadowType::default())
    }
}

/// TODO: docs
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum ShadowType {
    /// ```text
    /// ┌Box──┐
    /// │     │█
    /// └─────┘█
    ///  ███████
    /// ```
    #[default]
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
}
