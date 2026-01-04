//! 3D surface rendering module for the volatility surface visualization.
//!
//! This module implements perspective projection and rotation to render a 3D volatility surface
//! on a 2D canvas using Braille characters for high-resolution output.

use std::f64::consts::{PI, TAU};

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::symbols::Marker;
use ratatui::widgets::canvas::{Canvas, Line, Points};
use ratatui::widgets::{Block, Borders};

use super::Palette;

/// A 3D surface renderer that maintains rotation, zoom, and color palette state.
///
/// This renderer uses a concept called perspective projection to map 3D coordinates to 2D screen
/// space, allowing interactive orientation of a volatility surface through rotation and zoom.
/// The surface is rendered using Braille characters via the Canvas widget, which gives
/// substantially higher resolution than standard block characters.
pub struct Surface3D {
    rotation_x: f64,
    rotation_z: f64,
    zoom: f64,
    palette: Palette,
}

impl Surface3D {
    /// Distance from the camera to the projection plane
    const CAMERA_DISTANCE: f64 = 4.0;

    pub const fn new() -> Self {
        Self {
            rotation_x: 0.6, // Tilt forward for better view
            rotation_z: 0.3, // Slight rotation
            zoom: 1.0,
            palette: Palette::Plasma,
        }
    }

    /// Rotate the surface around the X axis (tilt up/down).
    pub fn rotate_x(&mut self, delta: f64) {
        self.rotation_x = (self.rotation_x + delta).clamp(-PI / 2.0, PI / 2.0);
    }

    /// Rotate the surface around the Z axis (spin left/right).
    pub fn rotate_z(&mut self, delta: f64) {
        self.rotation_z += delta;
        if self.rotation_z > TAU {
            self.rotation_z -= TAU;
        }
        if self.rotation_z < 0.0 {
            self.rotation_z += TAU;
        }
    }

    /// Adjust the zoom level by the given factor.
    pub fn zoom(&mut self, factor: f64) {
        self.zoom = (self.zoom * factor).clamp(0.3, 3.0);
    }

    /// Project a 3D point to 2D screen coordinates using perspective projection.
    ///
    /// This method applies rotation matrices and perspective division to transform
    /// 3D coordinates into 2D screen space while maintaining depth information.
    fn project(&self, x: f64, y: f64, z: f64) -> (f64, f64) {
        // Apply rotations
        let (sin_x, cos_x) = self.rotation_x.sin_cos();
        let (sin_z, cos_z) = self.rotation_z.sin_cos();

        // Rotate around Z axis
        let x1 = x * cos_z - y * sin_z;
        let y1 = x * sin_z + y * cos_z;
        let z1 = z;

        // Rotate around X axis
        let y2 = y1 * cos_x - z1 * sin_x;
        let z2 = y1 * sin_x + z1 * cos_x;

        // Perspective projection
        let perspective = Self::CAMERA_DISTANCE / (Self::CAMERA_DISTANCE + z2);

        (x1 * perspective * self.zoom, y2 * perspective * self.zoom)
    }
}

impl Surface3D {
    /// Render the 3D volatility surface to the given frame.
    ///
    /// The surface data is expected to be a 2D grid of volatility values, where the first
    /// dimension represents expiration times and the second dimension represents strike prices.
    pub fn render(&self, frame: &mut Frame, area: Rect, surface_data: &Vec<Vec<f64>>, _time: f64) {
        let n_exp = surface_data.len();
        let n_strike = surface_data.first().map_or(0, std::vec::Vec::len);

        if n_exp == 0 || n_strike == 0 {
            return;
        }

        // Find min/max for normalization
        let mut min_vol = f64::MAX;
        let mut max_vol = f64::MIN;
        for row in surface_data {
            for &vol in row {
                min_vol = min_vol.min(vol);
                max_vol = max_vol.max(vol);
            }
        }

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" 3D Volatility Surface - Use ↑↓←→ to rotate, Z/X to zoom ")
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .marker(Marker::Braille)
            .x_bounds([-2.0, 2.0])
            .y_bounds([-1.5, 1.5])
            .paint(|ctx| {
                // Draw grid lines along strike dimension
                for (i, row) in surface_data.iter().enumerate() {
                    let mut points = Vec::new();
                    let exp_norm = f64::from(i as u32) / f64::from((n_exp - 1) as u32);

                    for (j, &vol) in row.iter().enumerate().take(n_strike) {
                        let strike_norm = f64::from(j as u32) / f64::from((n_strike - 1) as u32);
                        let vol_norm = (vol - min_vol) / (max_vol - min_vol);

                        // Map to 3D coordinates
                        let x = (strike_norm - 0.5) * 3.0; // Strike
                        let y = (exp_norm - 0.5) * 3.0; // Expiry
                        let z = (vol_norm - 0.5) * 2.0; // IV height

                        let (px, py) = self.project(x, y, z);
                        points.push((px, py));
                    }

                    // Draw line with color based on expiry
                    let color_val = (i as f64 / n_exp as f64 * 0.7 + 0.3).min(1.0);
                    let color = self.palette.get_color(color_val);

                    for window in points.windows(2) {
                        ctx.draw(&Line {
                            x1: window[0].0,
                            y1: window[0].1,
                            x2: window[1].0,
                            y2: window[1].1,
                            color,
                        });
                    }
                }

                // Draw grid lines along expiry dimension
                for j in (0..n_strike).step_by(2) {
                    let mut points = Vec::new();
                    let strike_norm = f64::from(j as u32) / f64::from((n_strike - 1) as u32);

                    for (i, row) in surface_data.iter().enumerate() {
                        let exp_norm = f64::from(i as u32) / f64::from((n_exp - 1) as u32);
                        let vol = row[j];
                        let vol_norm = (vol - min_vol) / (max_vol - min_vol);

                        let x = (strike_norm - 0.5) * 3.0;
                        let y = (exp_norm - 0.5) * 3.0;
                        let z = (vol_norm - 0.5) * 2.0;

                        let (px, py) = self.project(x, y, z);
                        points.push((px, py));
                    }

                    let color_val = (strike_norm * 0.7 + 0.3).min(1.0);
                    let color = self.palette.get_color(color_val);

                    for window in points.windows(2) {
                        ctx.draw(&Line {
                            x1: window[0].0,
                            y1: window[0].1,
                            x2: window[1].0,
                            y2: window[1].1,
                            color,
                        });
                    }
                }

                // Add some glowing points at peaks for emphasis
                let mut peak_points = Vec::new();
                for i in (0..n_exp).step_by(2) {
                    for j in (0..n_strike).step_by(2) {
                        let exp_norm = i as f64 / (n_exp - 1) as f64;
                        let strike_norm = j as f64 / (n_strike - 1) as f64;
                        let vol = surface_data[i][j];
                        let vol_norm = (vol - min_vol) / (max_vol - min_vol);

                        if vol_norm > 0.7 {
                            // Only show high points
                            let x = (strike_norm - 0.5) * 3.0;
                            let y = (exp_norm - 0.5) * 3.0;
                            let z = (vol_norm - 0.5) * 2.0;

                            let (px, py) = self.project(x, y, z);
                            peak_points.push((px, py));
                        }
                    }
                }

                if !peak_points.is_empty() {
                    ctx.draw(&Points {
                        coords: &peak_points,
                        color: self.palette.get_color(0.9),
                    });
                }
            });

        frame.render_widget(canvas, area);
    }

    /// Cycle to the next color palette.
    pub const fn cycle_palette(&mut self) {
        self.palette = self.palette.next();
    }
}
