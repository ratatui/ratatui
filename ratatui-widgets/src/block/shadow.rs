use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Offset, Position, Rect};
use ratatui_core::style::{Color, Style};
use ratatui_core::symbols::shade;
use ratatui_core::widgets::Widget;

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
        Self::new(ShadowType::Fill(shade::FULL))
    }

    /// TODO: docs
    pub fn light_shade() -> Self {
        Self::new(ShadowType::Fill(shade::LIGHT))
    }

    /// TODO: docs
    pub fn medium_shade() -> Self {
        Self::new(ShadowType::Fill(shade::MEDIUM))
    }

    /// TODO: docs
    pub fn dark_shade() -> Self {
        Self::new(ShadowType::Fill(shade::DARK))
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

        // Always apply style
        for y in shadow_area.top()..shadow_area.bottom() {
            for x in shadow_area.left()..shadow_area.right() {
                if base_area.contains(Position { x, y }) {
                    continue;
                }
                buf[(x, y)].set_style(self.style);
            }
        }

        match self.shadow_type {
            ShadowType::Overlay => {}
            ShadowType::Fill(symbol) => {
                for y in shadow_area.top()..shadow_area.bottom() {
                    for x in shadow_area.left()..shadow_area.right() {
                        if base_area.contains(Position { x, y }) {
                            continue;
                        }
                        buf[(x, y)].set_symbol(symbol);
                    }
                }
            }
            ShadowType::Filter(filter) => {
                filter(shadow_area, base_area, buf);
            }
        }
    }
}

impl Default for Shadow {
    fn default() -> Self {
        Self::new(ShadowType::default())
    }
}

impl Widget for &Shadow {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // TODO: just move it here ig
        self.render(area, buf);
    }
}

/// TODO: docs
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShadowType {
    /// Only apply style on shadow area, don't change existing characters.
    #[default]
    Overlay,

    /// Fills shadowed area with a unicode symbol.
    /// ```text
    /// ┌Box──┐
    /// │     │▒
    /// └─────┘▒
    ///  ▒▒▒▒▒▒▒
    /// ```
    /// TODO: docs
    Fill(&'static str),

    /// Apply a filter to the shadowed area
    Filter(fn(Rect, Rect, &mut Buffer)),
}

/// Example
/// TODO: remove? 
pub fn dimmed(shadow_area: Rect, base_area: Rect, buf: &mut Buffer) {
    for y in shadow_area.top()..shadow_area.bottom() {
        for x in shadow_area.left()..shadow_area.right() {
            if base_area.contains(Position { x, y }) {
                continue;
            }
            if let Color::Rgb(r, g, b) = buf[(x, y)].fg {
                buf[(x, y)].fg = Color::Rgb(r / 2, g / 2, b / 2);
            }
            if let Color::Rgb(r, g, b) = buf[(x, y)].bg {
                buf[(x, y)].bg = Color::Rgb(r / 2, g / 2, b / 2);
            }
        }
    }
}
