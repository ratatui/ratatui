use indoc::indoc;

use crate::{buffer::Buffer, layout::Rect, text::Text, widgets::Widget};

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
/// cargo run --example ratatui-logo [size]
/// ```
///
/// [Ratatui-logo]: https://github.com/ratatui/ratatui/blob/main/examples/ratatui-logo.rs
///
/// ## Tiny
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
/// ## Small
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
    /// ```text
    /// ▛▚▗▀▖▜▘▞▚▝▛▐ ▌▌
    /// ▛▚▐▀▌▐ ▛▜ ▌▝▄▘▌
    /// ```
    #[default]
    Tiny,
    /// A small logo
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
    /// use ratatui::widgets::RatatuiLogo;
    ///
    /// let logo = RatatuiLogo::new(Size::Tiny);
    /// ```
    pub const fn new(size: Size) -> Self {
        Self { size }
    }

    /// Set the size of the logo
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::RatatuiLogo;
    ///
    /// let logo = RatatuiLogo::default().size(Size::Small);
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
