use ratatui::style::Color;

#[derive(Debug, Clone, Copy)]
pub enum Palette {
    Viridis,
    Plasma,
    Phosphor,
    Fear, // Red-dominant for high vol
    Calm, // Blue-dominant for low vol
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

        match self {
            Self::Viridis => viridis(t),
            Self::Plasma => plasma(t),
            Self::Phosphor => phosphor(t),
            Self::Fear => fear(t),
            Self::Calm => calm(t),
            Self::Inferno => inferno(t),
        }
    }
}

fn viridis(t: f64) -> Color {
    // Perceptually uniform colormap
    let r = (0.282 + t * (0.277 - 0.004 * t)).clamp(0.0, 1.0);
    let g = (0.0 + t * (0.982 - 0.020 * t)).clamp(0.0, 1.0);
    let b = (0.329 + t * (0.576 - 1.034 * t + 0.568 * t.powi(2))).clamp(0.0, 1.0);

    Color::Rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn plasma(t: f64) -> Color {
    let r = (0.050 + t * (1.0 - 0.050)).clamp(0.0, 1.0);
    let g = (0.030 + t * t * 0.970).clamp(0.0, 1.0);
    let b = (0.570 - t * 0.570).clamp(0.0, 1.0);

    Color::Rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn phosphor(t: f64) -> Color {
    // Retro green CRT phosphor glow
    let intensity = (t * 1.5).min(1.0);
    let r = (intensity * 0.15) as u8;
    let g = (intensity * 255.0) as u8;
    let b = (intensity * 0.25 * 255.0) as u8;

    Color::Rgb(r, g, b)
}

fn fear(t: f64) -> Color {
    // Red-orange for high volatility/fear
    let r = (0.5 + t * 0.5).min(1.0);
    let g = (t * 0.3).min(1.0);
    let b = 0.0;

    Color::Rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn calm(t: f64) -> Color {
    // Cool blues for low volatility
    let r = (t * 0.2).min(1.0);
    let g = (0.3 + t * 0.5).min(1.0);
    let b = (0.5 + t * 0.5).min(1.0);

    Color::Rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn inferno(t: f64) -> Color {
    let r = (t.powf(0.5)).min(1.0);
    let g = (t.powi(2)).min(1.0);
    let b = (t.powi(4)).min(1.0);

    Color::Rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}
