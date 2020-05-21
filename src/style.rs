use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

bitflags! {
    pub struct Modifier: u16 {
        const BOLD              = 0b0000_0000_0001;
        const DIM               = 0b0000_0000_0010;
        const ITALIC            = 0b0000_0000_0100;
        const UNDERLINED        = 0b0000_0000_1000;
        const SLOW_BLINK        = 0b0000_0001_0000;
        const RAPID_BLINK       = 0b0000_0010_0000;
        const REVERSED          = 0b0000_0100_0000;
        const HIDDEN            = 0b0000_1000_0000;
        const CROSSED_OUT       = 0b0001_0000_0000;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub modifier: Modifier,
}

impl Default for Style {
    fn default() -> Style {
        Style::new()
    }
}

impl Style {
    pub const fn new() -> Self {
        Style {
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
        }
    }
    pub fn reset(&mut self) {
        self.fg = Color::Reset;
        self.bg = Color::Reset;
        self.modifier = Modifier::empty();
    }

    pub const fn fg(mut self, color: Color) -> Style {
        self.fg = color;
        self
    }
    pub const fn bg(mut self, color: Color) -> Style {
        self.bg = color;
        self
    }
    pub const fn modifier(mut self, modifier: Modifier) -> Style {
        self.modifier = modifier;
        self
    }
}
