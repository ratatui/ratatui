//! A Ratatui mascot widget
//!
//! The mascot takes 32x32 cells and is rendered using half block characters.
use itertools::Itertools;
use ratatui_core::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget}; // tuples();

const RATATUI_MASCOT: &str = indoc::indoc! {"
                   hhh
                 hhhhhh
                hhhhhhh
               hhhhhhhh
              hhhhhhhhh
             hhhhhhhhhh
            hhhhhhhhhhhh
            hhhhhhhhhhhhh
            hhhhhhhhhhhhh     ██████
             hhhhhhhhhhh    ████████
                  hhhhh ███████████
                   hhh ██ee████████
                    h █████████████
                ████ █████████████
               █████████████████
               ████████████████
               ████████████████
                ███ ██████████
              ▒▒    █████████
             ▒░░▒   █████████
            ▒░░░░▒ ██████████
           ▒░░▓░░░▒ █████████
          ▒░░▓▓░░░░▒ ████████
         ▒░░░░░░░░░░▒ ██████████
        ▒░░░░░░░░░░░░▒ ██████████
       ▒░░░░░░░▓▓░░░░░▒ █████████
      ▒░░░░░░░░░▓▓░░░░░▒ ████  ███
     ▒░░░░░░░░░░░░░░░░░░▒ ██   ███
    ▒░░░░░░░░░░░░░░░░░░░░▒ █   ███
    ▒░░░░░░░░░░░░░░░░░░░░░▒   ███
     ▒░░░░░░░░░░░░░░░░░░░░░▒ ███
      ▒░░░░░░░░░░░░░░░░░░░░░▒ █"
};

const TERM: char = '░';
const TERM_BORDER: char = '▒';

/// State for the mascot's eye
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MascotEye {
    /// The default eye color
    #[default]
    Default,

    /// The red eye color
    Red,
}

/// A widget that renders the Ratatui mascot
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RatatuiMascot {
    eye_state: MascotEye,
    /// The color of the rat
    rat_color: Color,
    /// The color of the rat's eye
    rat_eye_color: Color,
    /// The color of the rat's eye when blinking
    rat_eye_blink: Color,
    /// The color of the rat's hat
    hat_color: Color,
    /// The color of the terminal
    term_color: Color,
    /// The color of the terminal border
    term_border_color: Color,
    /// The color of the terminal cursor
    term_cursor_color: Color,
}

impl Default for RatatuiMascot {
    fn default() -> Self {
        Self {
            rat_color: Color::Indexed(252),         // light_gray #d0d0d0
            hat_color: Color::Indexed(231),         // white #ffffff
            rat_eye_color: Color::Indexed(236),     // dark_charcoal #303030
            rat_eye_blink: Color::Indexed(196),     // red #ff0000
            term_color: Color::Indexed(232),        // vampire_black #080808
            term_border_color: Color::Indexed(235), // raisin_black  #262626
            term_cursor_color: Color::Indexed(248), // dark_gray #a8a8a8
            eye_state: MascotEye::Default,
        }
    }
}

impl RatatuiMascot {
    /// Create a new Ratatui mascot widget
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Tiny help to set the color of the rat's eye
    #[must_use]
    pub const fn set_eye(self, rat_eye: MascotEye) -> Self {
        Self {
            eye_state: rat_eye,
            ..self
        }
    }
    const fn color_for(&self, c: char) -> Option<Color> {
        match c {
            '█' => Some(self.rat_color),
            'h' => Some(self.hat_color),
            'e' => Some(match self.eye_state {
                MascotEye::Default => self.rat_eye_color,
                MascotEye::Red => self.rat_eye_blink,
            }),
            '░' => Some(self.term_color),
            '▓' => Some(self.term_cursor_color),
            '▒' => Some(self.term_border_color),
            _ => None,
        }
    }
}

impl Widget for RatatuiMascot {
    /// Use half block characters to render a logo based on the `RATATUI_LOGO` const.
    ///
    /// The logo is rendered in three colors, one for the rat, one for the terminal, and one for the
    /// rat's eye. The eye color alternates between two colors based on the selected row.
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (y, (line1, line2)) in RATATUI_MASCOT.lines().tuples().enumerate() {
            for (x, (ch1, ch2)) in line1.chars().zip(line2.chars()).enumerate() {
                let x = area.left() + x as u16;
                let y = area.top() + y as u16;
                let cell = &mut buf[(x, y)];
                // given two cells which make up the top and bottom of the character,
                // Foreground color should be the non-space, non-terminal
                let (fg, bg) = match (ch1, ch2) {
                    (' ', ' ') => (None, None),
                    (c, ' ') | (' ', c) => (self.color_for(c), None),
                    (TERM, TERM_BORDER) => (self.color_for(TERM_BORDER), Some(Color::default())),
                    (TERM, c) => (Some(Color::default()), self.color_for(c)), // treat the terminal background cells as empty
                    (c, TERM) => (self.color_for(c), Some(Color::default())), // rather than having a character
                    (c1, c2) => (self.color_for(c1), self.color_for(c2)),
                };
                // symbol should make the empty space or terminal bg as the empty part of the block
                let symbol = match (ch1, ch2) {
                    (' ', ' ') | (TERM, TERM) => ' ',
                    (_c, ' ' | TERM) => '▀',
                    (' ' | TERM, _c) => '▄',
                    (c, d) if c == d => '█',
                    (_c, _d) => '▀',
                };
                if let Some(fg) = fg {
                    cell.fg = fg;
                }
                if let Some(bg) = bg {
                    cell.bg = bg;
                }
                cell.set_char(symbol);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_mascot() {
        let mascot = RatatuiMascot::new();
        assert_eq!(mascot.eye_state, MascotEye::Default);
    }

    #[test]
    fn set_eye_color() {
        let mascot = RatatuiMascot::new().set_eye(MascotEye::Red);
        assert_eq!(mascot.eye_state, MascotEye::Red,);
    }

    #[test]
    fn render_mascot() {
        let mascot = RatatuiMascot::new();
        let mut buf = Buffer::empty(Rect::new(0, 0, 32, 32));
        mascot.render(buf.area, &mut buf);
        assert_eq!(buf.area.as_size(), (32, 32).into());
    }
}
