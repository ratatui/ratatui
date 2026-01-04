use colorgrad::{Color as GradColor, Gradient, LinearGradient};
use ratatui::style::Color;

#[derive(Debug, Clone, Copy)]
pub enum Palette {
    Viridis,
    Plasma,
    Phosphor,
    Fear,
    Calm,
    Inferno,
}

impl Palette {
    pub const fn next(self) -> Self {
        match self {
            Self::Viridis => Self::Plasma,
            Self::Plasma => Self::Phosphor,
            Self::Phosphor => Self::Fear,
            Self::Fear => Self::Calm,
            Self::Calm => Self::Inferno,
            Self::Inferno => Self::Viridis,
        }
    }

    /// Get color for a normalized value (0.0 to 1.0)
    pub fn get_color(self, t: f64) -> Color {
        let t = t.clamp(0.0, 1.0);

        let grad: Box<dyn Gradient> = match self {
            Self::Viridis => Box::new(colorgrad::preset::viridis()),
            Self::Plasma => Box::new(colorgrad::preset::plasma()),
            Self::Inferno => Box::new(colorgrad::preset::inferno()),
            Self::Phosphor => Box::new(Self::custom_phosphor()),
            Self::Fear => Box::new(Self::custom_fear()),
            Self::Calm => Box::new(Self::custom_calm()),
        };

        let rgba = grad.at(t as f32).to_rgba8();
        Color::Rgb(rgba[0], rgba[1], rgba[2])
    }

    fn custom_phosphor() -> LinearGradient {
        colorgrad::GradientBuilder::new()
            .colors(&[
                GradColor::from_rgba8(0, 40, 0, 255),
                GradColor::from_rgba8(38, 255, 63, 255),
            ])
            .build()
            .unwrap()
    }

    fn custom_fear() -> LinearGradient {
        colorgrad::GradientBuilder::new()
            .colors(&[
                GradColor::from_rgba8(127, 0, 0, 255),
                GradColor::from_rgba8(255, 76, 0, 255),
            ])
            .build()
            .unwrap()
    }

    fn custom_calm() -> LinearGradient {
        colorgrad::GradientBuilder::new()
            .colors(&[
                GradColor::from_rgba8(51, 76, 127, 255),
                GradColor::from_rgba8(102, 204, 255, 255),
            ])
            .build()
            .unwrap()
    }
}
