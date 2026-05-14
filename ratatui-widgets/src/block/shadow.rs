use alloc::sync::Arc;
use core::hash::{Hash, Hasher};
use core::{fmt, ptr};

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Offset, Position, Rect};
use ratatui_core::style::{Color, Modifier, Style, Styled};
use ratatui_core::symbols::shade;
use ratatui_core::widgets::Widget;

/// A configurable shadow that can be rendered behind a [`Block`](crate::block::Block).
///
/// A [`Shadow`] is rendered in an offset area relative to the block. Its [`Style`] is applied
/// first, then an optional cell effect can modify the affected cells, for example by filling them
/// with a shading symbol or dimming the existing background.
///
/// Built-in presets:
///
/// - [`Shadow::overlay`] applies only style
/// - [`Shadow::block`] fills with full block symbols
/// - [`Shadow::light_shade`], [`Shadow::medium_shade`], and [`Shadow::dark_shade`] fill with shade
///   symbols
///
/// ```plain
/// ┌Popup─────┐
/// │content   │▒
/// └──────────┘▒
///   ▒▒▒▒▒▒▒▒▒▒▒
/// ```
///
/// # Custom effects
///
/// ```
/// use ratatui::buffer::Buffer;
/// use ratatui::layout::{Position, Rect};
/// use ratatui::widgets::{Block, CellEffect, Shadow};
///
/// #[derive(Debug)]
/// struct Checker;
///
/// impl CellEffect for Checker {
///     fn apply(&self, shadow_area: Rect, base_area: Rect, buf: &mut Buffer) {
///         for y in shadow_area.top()..shadow_area.bottom() {
///             for x in shadow_area.left()..shadow_area.right() {
///                 if base_area.contains(Position { x, y }) {
///                     continue;
///                 }
///                 if (x + y) % 2 == 0 {
///                     buf[(x, y)].set_symbol("░");
///                 }
///             }
///         }
///     }
/// }
///
/// let shadow = Shadow::custom(Checker);
/// let block = Block::bordered().shadow(shadow);
/// ```
#[derive(Debug, Clone, Eq)]
pub struct Shadow {
    effect: Effect,
    style: Style,
    offset: Offset,
}

/// The built-in shadow effects.
#[derive(Debug, Clone)]
enum Effect {
    /// Applies no symbol changes and only keeps the shadow style.
    Overlay,
    /// Fills the shadow area with a single symbol.
    Symbol(&'static str),
    /// Applies a user-defined shadow effect.
    Custom(Arc<dyn CellEffect>),
}

/// A cell effect that modifies the cells covered by a [`Shadow`].
///
/// See [`Shadow::custom`] for how to create a shadow from a custom effect.
pub trait CellEffect: fmt::Debug {
    /// Applies the effect to the cells in `shadow_area`.
    fn apply(&self, shadow_area: Rect, base_area: Rect, buf: &mut Buffer);
}

impl Effect {
    /// Applies the effect to the shadow area in the buffer.
    fn apply(&self, shadow_area: Rect, base_area: Rect, buf: &mut Buffer) {
        match self {
            Self::Overlay => {}
            Self::Symbol(symbol) => {
                for_each_shadow_cell(shadow_area, base_area, buf, |x, y, buf| {
                    buf[(x, y)].set_symbol(symbol);
                });
            }
            Self::Custom(filter) => filter.apply(shadow_area, base_area, buf),
        }
    }
}

impl PartialEq for Effect {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Overlay, Self::Overlay) => true,
            (Self::Symbol(lhs), Self::Symbol(rhs)) => lhs == rhs,
            (Self::Custom(lhs), Self::Custom(rhs)) => Arc::ptr_eq(lhs, rhs),
            _ => false,
        }
    }
}

impl Eq for Effect {}

impl Hash for Effect {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Overlay => "overlay".hash(state),
            Self::Symbol(symbol) => {
                "symbol".hash(state);
                symbol.hash(state);
            }
            Self::Custom(filter) => {
                "custom".hash(state);
                ptr::hash(Arc::as_ptr(filter), state);
            }
        }
    }
}

impl PartialEq for Shadow {
    fn eq(&self, other: &Self) -> bool {
        self.effect == other.effect && self.style == other.style && self.offset == other.offset
    }
}

impl Hash for Shadow {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.effect.hash(state);
        self.style.hash(state);
        self.offset.hash(state);
    }
}

impl Shadow {
    /// Creates a shadow that only applies style to the offset area.
    ///
    /// This leaves the existing cell symbols unchanged.
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui::style::Stylize;
    /// use ratatui::widgets::{Block, Shadow};
    ///
    /// let shadow = Shadow::overlay().black().on_white();
    /// let block = Block::bordered().shadow(shadow);
    /// ```
    pub fn overlay() -> Self {
        Self {
            effect: Effect::Overlay,
            style: Style::default(),
            offset: Offset::new(1, 1),
        }
    }

    /// Creates a shadow filled with full block symbols.
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui::widgets::{Block, Shadow};
    ///
    /// let block = Block::bordered().shadow(Shadow::block());
    /// ```
    pub fn block() -> Self {
        Self::symbol(shade::FULL)
    }

    /// Creates a shadow filled with light shade symbols.
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui::widgets::{Block, Shadow};
    ///
    /// let block = Block::bordered().shadow(Shadow::light_shade());
    /// ```
    pub fn light_shade() -> Self {
        Self::symbol(shade::LIGHT)
    }

    /// Creates a shadow filled with medium shade symbols.
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui::widgets::{Block, Shadow};
    ///
    /// let block = Block::bordered().shadow(Shadow::medium_shade());
    /// ```
    pub fn medium_shade() -> Self {
        Self::symbol(shade::MEDIUM)
    }

    /// Creates a shadow filled with dark shade symbols.
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui::layout::Offset;
    /// use ratatui::style::Stylize;
    /// use ratatui::widgets::{Block, Shadow};
    ///
    /// let block = Block::bordered().shadow(
    ///     Shadow::dark_shade()
    ///         .black()
    ///         .on_white()
    ///         .offset(Offset::new(2, 1)),
    /// );
    /// ```
    pub fn dark_shade() -> Self {
        Self::symbol(shade::DARK)
    }

    /// Creates a shadow filled with the given symbol.
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui::widgets::{Block, Shadow};
    ///
    /// let shadow = Shadow::symbol("░");
    /// let block = Block::bordered().shadow(shadow);
    /// ```
    pub fn symbol(symbol: &'static str) -> Self {
        Self {
            effect: Effect::Symbol(symbol),
            style: Style::default(),
            offset: Offset::new(1, 1),
        }
    }

    /// Creates a new shadow from a custom cell effect.
    ///
    /// The effect receives the shadow area, the original block area, and the target buffer. It is
    /// called after the shadow style has been applied.
    pub fn custom<F: CellEffect + 'static>(effect: F) -> Self {
        Self {
            effect: Effect::Custom(Arc::new(effect)),
            style: Style::default(),
            offset: Offset::new(1, 1),
        }
    }

    /// Creates a new shadow from a custom cell effect.
    ///
    /// Alias for [`Shadow::custom`].
    pub fn new<F: CellEffect + 'static>(effect: F) -> Self {
        Self::custom(effect)
    }

    /// Sets the style applied to the shadow area.
    #[must_use]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the shadow offset relative to the original area.
    ///
    /// Positive horizontal values move the shadow to the right and positive vertical values move it
    /// downward.
    #[must_use]
    pub const fn offset(mut self, offset: Offset) -> Self {
        self.offset = offset;
        self
    }
}

impl Default for Shadow {
    fn default() -> Self {
        Self::overlay()
    }
}

impl Styled for Shadow {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

impl Widget for &Shadow {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let shadow_area = area.offset(self.offset).intersection(buf.area);

        // Apply style
        for y in shadow_area.top()..shadow_area.bottom() {
            for x in shadow_area.left()..shadow_area.right() {
                if area.contains(Position { x, y }) {
                    continue;
                }
                buf[(x, y)].set_style(self.style);
            }
        }

        // Apply effect
        self.effect.apply(shadow_area, area, buf);
    }
}

/// A [`CellEffect`] that dims the shadow cells by setting the [`DIM`](Modifier::DIM) modifier.
///
/// If the cell background is RGB, each channel is halved. Otherwise the background is replaced
/// with [`Color::Black`].
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Dimmed;

impl CellEffect for Dimmed {
    fn apply(&self, shadow_area: Rect, base_area: Rect, buf: &mut Buffer) {
        for_each_shadow_cell(shadow_area, base_area, buf, |x, y, buf| {
            buf[(x, y)].modifier.insert(Modifier::DIM);
            if let Color::Rgb(r, g, b) = buf[(x, y)].bg {
                buf[(x, y)].bg = Color::Rgb(r / 2, g / 2, b / 2);
            } else {
                buf[(x, y)].bg = Color::Black;
            }
        });
    }
}

/// Creates a [`Dimmed`] shadow effect.
pub const fn dimmed() -> Dimmed {
    Dimmed
}

/// Helper for iterating over the shadow area while skipping cells that overlap the base area.
fn for_each_shadow_cell(
    shadow_area: Rect,
    base_area: Rect,
    buf: &mut Buffer,
    mut f: impl FnMut(u16, u16, &mut Buffer),
) {
    for y in shadow_area.top()..shadow_area.bottom() {
        for x in shadow_area.left()..shadow_area.right() {
            if base_area.contains(Position { x, y }) {
                continue;
            }
            f(x, y, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui_core::buffer::Buffer;
    use ratatui_core::layout::Rect;
    use ratatui_core::style::{Color, Style};
    use ratatui_core::widgets::Widget;
    use rstest::rstest;

    use super::*;

    fn render_shadow(shadow: &Shadow) -> Buffer {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 4));
        shadow.render(Rect::new(0, 0, 2, 2), &mut buffer);
        buffer
    }

    #[test]
    fn overlay_renders_style_without_changing_symbols() {
        let mut buffer = Buffer::with_lines(["abcd", "efgh", "ijkl", "mnop"]);
        let shadow = Shadow::overlay().style(Style::new().red().on_blue());

        (&shadow).render(Rect::new(0, 0, 2, 2), &mut buffer);

        assert_eq!(buffer[(2, 1)].symbol(), "g");
        assert_eq!(buffer[(1, 2)].symbol(), "j");
        assert_eq!(buffer[(2, 2)].symbol(), "k");
        assert_eq!(buffer[(2, 1)].fg, Color::Red);
        assert_eq!(buffer[(2, 1)].bg, Color::Blue);
        assert_eq!(buffer[(1, 1)].fg, Color::Reset);
        assert_eq!(buffer[(1, 1)].bg, Color::Reset);
    }

    #[rstest]
    #[case(Shadow::symbol("$"), "$")]
    #[case(Shadow::block(), shade::FULL)]
    fn symbol_filters_fill_only_visible_shadow_cells(
        #[case] shadow: Shadow,
        #[case] symbol: &'static str,
    ) {
        let buffer = render_shadow(&shadow);

        assert_eq!(buffer[(2, 1)].symbol(), symbol);
        assert_eq!(buffer[(1, 2)].symbol(), symbol);
        assert_eq!(buffer[(2, 2)].symbol(), symbol);
        assert_eq!(buffer[(1, 1)].symbol(), " ");
    }

    #[test]
    fn render_is_clipped_to_buffer() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 2));
        let shadow = Shadow::symbol("#");

        (&shadow).render(Rect::new(0, 0, 2, 1), &mut buffer);

        assert_eq!(buffer[(2, 1)].symbol(), "#");
    }

    #[test]
    fn custom_filter_is_applied() {
        #[derive(Debug)]
        struct PlusFilter;

        impl CellEffect for PlusFilter {
            fn apply(&self, shadow_area: Rect, base_area: Rect, buf: &mut Buffer) {
                for_each_shadow_cell(shadow_area, base_area, buf, |x, y, buf| {
                    buf[(x, y)].set_symbol("+");
                });
            }
        }

        let buffer = render_shadow(&Shadow::new(PlusFilter));

        assert_eq!(buffer[(2, 1)].symbol(), "+");
        assert_eq!(buffer[(1, 2)].symbol(), "+");
        assert_eq!(buffer[(2, 2)].symbol(), "+");
    }

    #[test]
    fn dimmed_filter_dims_background() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 4));
        buffer.set_style(buffer.area, Style::new().bg(Color::Rgb(100, 120, 140)));
        let shadow = Shadow::new(dimmed());

        (&shadow).render(Rect::new(0, 0, 2, 2), &mut buffer);

        assert!(buffer[(2, 1)].modifier.contains(Modifier::DIM));
        assert_eq!(buffer[(2, 1)].bg, Color::Rgb(50, 60, 70));
        assert_eq!(buffer[(1, 1)].bg, Color::Rgb(100, 120, 140));
        assert!(!buffer[(1, 1)].modifier.contains(Modifier::DIM));
    }
}
