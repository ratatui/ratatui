//! The [`RatatuiLogo`] widget renders the Ratatui logo.
use indoc::indoc;
use ratatui_core::{buffer::Buffer, layout::Rect, style::Color, text::Text, widgets::Widget};

/// A widget that renders the Ratatui logo
///
/// The Ratatui logo takes up two lines of text and comes in two sizes: `Tiny` and `Small`. This may
/// be used in an application's help or about screen to show that it is powered by Ratatui.
///
/// # Examples
///
/// The [Ratatui-logo] example demonstrates how to use the `RatatuiLogo` widget. This can be run by
/// cloning the Ratatui repository and then running the following command with an optional size
/// argument:
///
/// ```shell
/// cargo run --example logo [size]
/// ```
///
/// [Ratatui-logo]: https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples/logo.rs
///
/// ## Tiny (default, 2x15 characters)
///
/// ```
/// use ratatui::widgets::RatatuiLogo;
///
/// # fn draw(frame: &mut ratatui::Frame) {
/// frame.render_widget(RatatuiLogo::tiny(), frame.area());
/// # }
/// ```
///
/// Renders:
///
/// ```text
/// ▛▚▗▀▖▜▘▞▚▝▛▐ ▌▌
/// ▛▚▐▀▌▐ ▛▜ ▌▝▄▘▌
/// ```
///
/// ## Small (2x27 characters)
///
/// ```
/// use ratatui::widgets::RatatuiLogo;
///
/// # fn draw(frame: &mut ratatui::Frame) {
/// frame.render_widget(RatatuiLogo::small(), frame.area());
/// # }
/// ```
///
/// Renders:
///
/// ```text
/// █▀▀▄ ▄▀▀▄▝▜▛▘▄▀▀▄▝▜▛▘█  █ █
/// █▀▀▄ █▀▀█ ▐▌ █▀▀█ ▐▌ ▀▄▄▀ █
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RatatuiLogo {
    size: Size,
}

/// The size of the logo
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Size {
    /// A tiny logo
    ///
    /// The default size of the logo (2x15 characters)
    ///
    /// ```text
    /// ▛▚▗▀▖▜▘▞▚▝▛▐ ▌▌
    /// ▛▚▐▀▌▐ ▛▜ ▌▝▄▘▌
    /// ```
    #[default]
    Tiny,
    /// A small logo
    ///
    /// A slightly larger version of the logo (2x27 characters)
    ///
    /// ```text
    /// █▀▀▄ ▄▀▀▄▝▜▛▘▄▀▀▄▝▜▛▘█  █ █
    /// █▀▀▄ █▀▀█ ▐▌ █▀▀█ ▐▌ ▀▄▄▀ █
    /// ```
    Small,
}

impl RatatuiLogo {
    /// Create a new Ratatui logo widget
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::{RatatuiLogo, RatatuiLogoSize};
    ///
    /// let logo = RatatuiLogo::new(RatatuiLogoSize::Tiny);
    /// ```
    pub const fn new(size: Size) -> Self {
        Self { size }
    }

    /// Set the size of the logo
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::{RatatuiLogo, RatatuiLogoSize};
    ///
    /// let logo = RatatuiLogo::default().size(RatatuiLogoSize::Small);
    /// ```
    #[must_use]
    pub const fn size(self, size: Size) -> Self {
        let _ = self;
        Self { size }
    }

    /// Create a new Ratatui logo widget with a tiny size
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::RatatuiLogo;
    ///
    /// let logo = RatatuiLogo::tiny();
    /// ```
    pub const fn tiny() -> Self {
        Self::new(Size::Tiny)
    }

    /// Create a new Ratatui logo widget with a small size
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::RatatuiLogo;
    ///
    /// let logo = RatatuiLogo::small();
    /// ```
    pub const fn small() -> Self {
        Self::new(Size::Small)
    }
}

impl Widget for RatatuiLogo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let logo = self.size.as_str();
        Text::raw(logo).render(area, buf);
    }
}

impl Size {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Tiny => Self::tiny(),
            Self::Small => Self::small(),
        }
    }

    const fn tiny() -> &'static str {
        indoc! {"
            ▛▚▗▀▖▜▘▞▚▝▛▐ ▌▌
            ▛▚▐▀▌▐ ▛▜ ▌▝▄▘▌
        "}
    }

    const fn small() -> &'static str {
        indoc! {"
            █▀▀▄ ▄▀▀▄▝▜▛▘▄▀▀▄▝▜▛▘█  █ █
            █▀▀▄ █▀▀█ ▐▌ █▀▀█ ▐▌ ▀▄▄▀ █
        "}
    }
}

const RATATUI_MASCOT: [&str; 32] = [
    "               hhh              ",
    "             hhhhhh             ",
    "            hhhhhhh             ",
    "           hhhhhhhh             ",
    "          hhhhhhhhh             ",
    "         hhhhhhhhhh             ",
    "        hhhhhhhhhhhh            ",
    "        hhhhhhhhhhhhh           ",
    "        hhhhhhhhhhhhh     ██████",
    "         hhhhhhhhhhh    ████████",
    "              hhhhh ███████████ ",
    "               hhh ██ee████████ ",
    "                h █████████████ ",
    "            ████ █████████████  ",
    "           █████████████████    ",
    "           ████████████████     ",
    "           ████████████████     ",
    "            ███ ██████████      ",
    "          bb    █████████       ",
    "         bxxb   █████████       ",
    "        bxxxxb ██████████       ",
    "       bxx█xxxb █████████       ",
    "      bxx██xxxxb ████████       ",
    "     bxxxxxxxxxxb ██████████    ",
    "    bxxxxxxxxxxxxb ██████████   ",
    "   bxxxxxxx██xxxxxb █████████   ",
    "  bxxxxxxxxx██xxxxxb ████  ███  ",
    " bxxxxxxxxxxxxxxxxxxb ██   ███  ",
    "bxxxxxxxxxxxxxxxxxxxxb █   ███  ",
    "bxxxxxxxxxxxxxxxxxxxxxb   ███   ",
    " bxxxxxxxxxxxxxxxxxxxxxb ███    ",
    "  bxxxxxxxxxxxxxxxxxxxxxb █     ",
];

/// A widget that renders the Ratatui mascot
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RatatuiMascot {
    colors: MascotColors,
}

/// The colors used to render the Ratatui mascot
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The colors used to render the Ratatui mascot
pub struct MascotColors {
    /// The color of the rat
    pub rat: Color,
    /// The color of the rat's eye
    pub rat_eye: Color,
    /// The color of the rat's hat
    pub hat: Color,
    /// The color of the terminal
    pub term: Color,
    /// The color of the terminal border
    pub term_border: Color,
}

impl Default for MascotColors {
    fn default() -> Self {
        Self {
            rat: Color::Rgb(204, 204, 204), // color of the website
            hat: Color::Rgb(255, 255, 255),
            rat_eye: Color::Rgb(8, 8, 8),
            term: Color::Rgb(8, 8, 8),
            term_border: Color::Rgb(188, 188, 188),
        }
    }
}

use itertools::Itertools; // tuples();

impl RatatuiMascot {
    /// Create a new Ratatui mascot widget
    pub const fn new(colors: MascotColors) -> Self {
        Self { colors }
    }

    /// Tiny help to set the color of the rat's eye
    #[must_use]
    pub const fn set_eye_color(self, rat_eye: Color) -> Self {
        Self {
            colors: MascotColors {
                rat_eye,
                ..self.colors
            },
        }
    }

    /// Use half block characters to render a logo based on the `RATATUI_LOGO` const.
    ///
    /// The logo is rendered in three colors, one for the rat, one for the terminal, and one for the
    /// rat's eye. The eye color alternates between two colors based on the selected row.
    pub fn render_mascot(&self, area: Rect, buf: &mut Buffer) {
        let rat_color = self.colors.rat;
        let term_color = self.colors.term;
        let hat_color = self.colors.hat;
        let border_color = self.colors.term_border;
        for (y, (line1, line2)) in RATATUI_MASCOT.iter().tuples().enumerate() {
            for (x, (ch1, ch2)) in line1.chars().zip(line2.chars()).enumerate() {
                let x = area.left() + x as u16;
                let y = area.top() + y as u16;
                let cell = &mut buf[(x, y)];
                match (ch1, ch2) {
                    ('█', '█') => {
                        cell.set_char('█');
                        cell.fg = rat_color;
                        cell.bg = rat_color;
                    }
                    ('h', 'h') => {
                        cell.set_char('█');
                        cell.fg = hat_color;
                    }
                    ('█', ' ') => {
                        cell.set_char('▀');
                        cell.fg = rat_color;
                    }
                    ('h', ' ') => {
                        cell.set_char('▀');
                        cell.fg = hat_color;
                    }
                    (' ', '█') => {
                        cell.set_char('▄');
                        cell.fg = rat_color;
                    }
                    (' ', 'h') => {
                        cell.set_char('▄');
                        cell.fg = hat_color;
                    }
                    (' ', 'b') => {
                        cell.set_char('▄');
                        cell.fg = border_color;
                    }
                    ('b', ' ') => {
                        cell.set_char('▀');
                        cell.fg = border_color;
                    }
                    ('b', 'b') => {
                        cell.set_char('█');
                        cell.fg = border_color;
                    }
                    ('b', 'x') => {
                        cell.set_char('▀');
                        cell.fg = border_color;
                        cell.bg = term_color;
                    }
                    ('x', 'b') => {
                        cell.set_char('▄');
                        cell.fg = border_color;
                        cell.bg = term_color;
                    }
                    ('█', 'x') => {
                        cell.set_char('▀');
                        cell.fg = rat_color;
                        cell.bg = term_color;
                    }
                    ('x', '█') => {
                        cell.set_char('▄');
                        cell.fg = rat_color;
                        cell.bg = term_color;
                    }
                    ('x', 'x') => {
                        cell.set_char(' ');
                        cell.fg = term_color;
                        cell.bg = term_color;
                    }
                    ('█', 'e') => {
                        cell.set_char('▀');
                        cell.fg = rat_color;
                        cell.bg = self.colors.rat_eye;
                    }
                    (_, _) => {}
                };
            }
        }
    }
}

impl Widget for RatatuiMascot {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_mascot(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::tiny(Size::Tiny)]
    #[case::small(Size::Small)]
    fn new_size(#[case] size: Size) {
        let logo = RatatuiLogo::new(size);
        assert_eq!(logo.size, size);
    }

    #[test]
    fn default_logo_is_tiny() {
        let logo = RatatuiLogo::default();
        assert_eq!(logo.size, Size::Tiny);
    }

    #[test]
    fn set_logo_size_to_small() {
        let logo = RatatuiLogo::default().size(Size::Small);
        assert_eq!(logo.size, Size::Small);
    }

    #[test]
    fn tiny_logo_constant() {
        let logo = RatatuiLogo::tiny();
        assert_eq!(logo.size, Size::Tiny);
    }

    #[test]
    fn small_logo_constant() {
        let logo = RatatuiLogo::small();
        assert_eq!(logo.size, Size::Small);
    }

    #[test]
    #[rustfmt::skip]
    fn render_tiny() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 15, 2));
        RatatuiLogo::tiny().render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "▛▚▗▀▖▜▘▞▚▝▛▐ ▌▌",
                "▛▚▐▀▌▐ ▛▜ ▌▝▄▘▌",
            ])
        );
    }

    #[test]
    #[rustfmt::skip]
    fn render_small() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 27, 2));
        RatatuiLogo::small().render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "█▀▀▄ ▄▀▀▄▝▜▛▘▄▀▀▄▝▜▛▘█  █ █",
                "█▀▀▄ █▀▀█ ▐▌ █▀▀█ ▐▌ ▀▄▄▀ █",
            ])
        );
    }

    #[test]
    fn default_mascot_colors() {
        let colors = MascotColors::default();
        assert_eq!(
            colors,
            MascotColors {
                rat: Color::Rgb(204, 204, 204),
                hat: Color::Rgb(255, 255, 255),
                rat_eye: Color::Rgb(8, 8, 8),
                term: Color::Rgb(8, 8, 8),
                term_border: Color::Rgb(188, 188, 188),
            }
        );
    }

    #[test]
    fn new_mascot() {
        let colors = MascotColors::default();
        let mascot = RatatuiMascot::new(colors);
        assert_eq!(mascot.colors, colors);
    }

    #[test]
    fn set_eye_color() {
        let colors = MascotColors::default();
        let mascot = RatatuiMascot::new(colors).set_eye_color(Color::Rgb(255, 0, 0));
        assert_eq!(
            mascot.colors,
            MascotColors {
                rat: Color::Rgb(204, 204, 204),
                hat: Color::Rgb(255, 255, 255),
                rat_eye: Color::Rgb(255, 0, 0),
                term: Color::Rgb(8, 8, 8),
                term_border: Color::Rgb(188, 188, 188),
            }
        );
    }

    #[test]
    fn render_mascot() {
        let colors = MascotColors::default();
        let mascot = RatatuiMascot::new(colors);
        let mut buf = Buffer::empty(Rect::new(0, 0, 32, 32));
        mascot.render_mascot(buf.area, &mut buf);
        assert_eq!(buf.area.as_size(), (32, 32).into());
    }

    #[test]
    fn render_mascot_with_custom_colors() {
        let colors = MascotColors {
            rat: Color::Rgb(255, 0, 0),
            hat: Color::Rgb(0, 255, 0),
            rat_eye: Color::Rgb(0, 0, 255),
            term: Color::Rgb(255, 255, 0),
            term_border: Color::Rgb(0, 255, 255),
        };
        let mascot = RatatuiMascot::new(colors);
        let mut buf = Buffer::empty(Rect::new(0, 0, 32, 32));
        mascot.render(buf.area, &mut buf);
        assert_eq!(buf.area.as_size(), (32, 32).into());
    }
}
