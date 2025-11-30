use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Offset, Position, Rect};
use ratatui_core::style::{Color, Modifier, Style};
use ratatui_core::symbols::shade;
use ratatui_core::widgets::Widget;

/// TODO: docs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shadow {
    filter: fn(Rect, Rect, &mut Buffer),
    style: Style,
    offset: Offset,
}

impl Shadow {
    /// TODO: docs
    pub fn new(filter: fn(Rect, Rect, &mut Buffer)) -> Self {
        Self {
            filter,
            style: Style::default(),
            offset: Offset::new(1, 1),
        }
    }

    /// TODO: docs
    pub fn overlay() -> Self {
        Self::new(|_, _, _| {})
    }

    /// TODO: docs
    pub fn block() -> Self {
        Self::new(crate::cell_filter!(shade::FULL))
    }

    /// TODO: docs
    pub fn light_shade() -> Self {
        Self::new(crate::cell_filter!(shade::LIGHT))
    }

    /// TODO: docs
    pub fn medium_shade() -> Self {
        Self::new(crate::cell_filter!(shade::MEDIUM))
    }

    /// TODO: docs
    pub fn dark_shade() -> Self {
        Self::new(crate::cell_filter!(shade::DARK))
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

        // Apply style
        for y in shadow_area.top()..shadow_area.bottom() {
            for x in shadow_area.left()..shadow_area.right() {
                if base_area.contains(Position { x, y }) {
                    continue;
                }
                buf[(x, y)].set_style(self.style);
            }
        }

        // Apply filter
        (self.filter)(shadow_area, base_area, buf);
    }
}

impl Default for Shadow {
    fn default() -> Self {
        Self::overlay()
    }
}

impl Widget for &Shadow {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // TODO: just move it here ig
        self.render(area, buf);
    }
}

/// TODO: docs
#[macro_export]
macro_rules! cell_filter {
    // base case
    ($x:ident, $y:ident, $shadow:ident, $base:ident, $buf:ident, $body:block) => {
        for $y in $shadow.top()..$shadow.bottom() {
            for $x in $shadow.left()..$shadow.right() {
                if $base.contains(Position { $x, $y }) {
                    continue;
                }
                $body
            }
        }
    };

    // function
    ($(#[$attr:meta])* $vis:vis fn $name:ident($x:ident: u16, $y:ident: u16, $buf:ident: &mut Buffer) $body:block) => {
        $(#[$attr])* $vis fn $name (shadow_area: Rect, base_area: Rect, $buf: &mut Buffer) {
            $crate::cell_filter!($x, $y, shadow_area, base_area, $buf, $body)
        }
    };

    // closure
    (|$x:ident: u16, $y:ident: u16, $buf:ident: &mut Buffer| $body:block) => {
        |shadow_area: Rect, base_area: Rect, $buf: &mut Buffer| {
            $crate::cell_filter!($x, $y, shadow_area, base_area, $buf, $body)
        }
    };
    (|$x:ident: u16, $y:ident: u16, $buf:ident: &mut Buffer| $body:expr) => {
        $crate::cell_filter!(|$x: u16, $y: u16, $buf: &mut Buffer| { $body; })
    };

    // fill
    ($s:expr) => {
        $crate::cell_filter!(|x: u16, y: u16, buf: &mut Buffer| buf[(x, y)].set_symbol($s))
    };
}

cell_filter! {
    /// TODO: docs
    pub fn dimmed(x: u16, y: u16, buf: &mut Buffer) {
        buf[(x, y)].modifier.insert(Modifier::DIM);
        if let Color::Rgb(r, g, b) = buf[(x, y)].bg {
            buf[(x, y)].bg = Color::Rgb(r / 2, g / 2, b / 2);
        } else {
            buf[(x, y)].bg = Color::Black;
        }
    }
}
