//! 3D surface rendering module for the volatility surface visualization.
//!
//! This module implements perspective projection and rotation to render a 3D volatility surface
//! on a 2D canvas using Braille characters for high-resolution output.

use std::f64::consts::{PI, TAU};

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::symbols::Marker;
use ratatui::widgets::canvas::{Canvas, Context, Line, Points};
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
    pub fn render(&self, frame: &mut Frame, area: Rect, surface_data: &[Vec<f64>]) {
        let n_exp = surface_data.len();
        let n_strike = surface_data.first().map_or(0, std::vec::Vec::len);

        if n_exp == 0 || n_strike == 0 {
            return;
        }

        let (min_vol, max_vol) = Self::find_volatility_range(surface_data);

        let canvas = Canvas::default()
            .block(Self::create_border())
            .marker(Marker::Braille)
            .x_bounds([-2.0, 2.0]) // Canvas viewport slightly larger than surface bounds
            .y_bounds([-1.5, 1.5]) // Narrower vertically to match terminal aspect ratio
            .paint(|ctx| {
                self.draw_strike_grid_lines(ctx, surface_data, n_exp, n_strike, min_vol, max_vol);
                self.draw_expiry_grid_lines(ctx, surface_data, n_exp, n_strike, min_vol, max_vol);
                self.draw_peak_highlights(ctx, surface_data, n_exp, n_strike, min_vol, max_vol);
            });

        frame.render_widget(canvas, area);
    }

    /// Find the minimum and maximum volatility values in the surface data.
    fn find_volatility_range(surface_data: &[Vec<f64>]) -> (f64, f64) {
        let mut min_vol = f64::MAX;
        let mut max_vol = f64::MIN;

        for row in surface_data {
            for &vol in row {
                min_vol = min_vol.min(vol);
                max_vol = max_vol.max(vol);
            }
        }

        (min_vol, max_vol)
    }

    /// Create the border block with title.
    fn create_border() -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .title(" 3D Volatility Surface - Use ↑↓←→ to rotate, Z/X to zoom ")
            .border_style(Style::default().fg(Color::DarkGray))
    }

    /// Draw grid lines along the strike dimension (horizontal lines).
    fn draw_strike_grid_lines(
        &self,
        ctx: &mut Context,
        surface_data: &[Vec<f64>],
        n_exp: usize,
        n_strike: usize,
        min_vol: f64,
        max_vol: f64,
    ) {
        for (i, row) in surface_data.iter().enumerate() {
            let points = self.project_row_to_points(row, i, n_exp, n_strike, min_vol, max_vol);
            // Map to [0.3, 1.0] range to avoid too-dark colors at the edges
            let color = self
                .palette
                .get_color((i as f64 / n_exp as f64 * 0.7 + 0.3).min(1.0));
            Self::draw_line_strip(ctx, &points, color);
        }
    }

    /// Draw grid lines along the expiry dimension (vertical lines).
    fn draw_expiry_grid_lines(
        &self,
        ctx: &mut Context,
        surface_data: &[Vec<f64>],
        n_exp: usize,
        n_strike: usize,
        min_vol: f64,
        max_vol: f64,
    ) {
        // Draw every other line to reduce visual clutter, got better results with this
        for j in (0..n_strike).step_by(2) {
            let points =
                self.project_column_to_points(surface_data, j, n_exp, n_strike, min_vol, max_vol);
            let strike_norm = f64::from(j as u32) / f64::from((n_strike - 1) as u32);
            let color = self.palette.get_color((strike_norm * 0.7 + 0.3).min(1.0));
            Self::draw_line_strip(ctx, &points, color);
        }
    }

    /// Draw highlighted points at volatility peaks.
    fn draw_peak_highlights(
        &self,
        ctx: &mut Context,
        surface_data: &[Vec<f64>],
        n_exp: usize,
        n_strike: usize,
        min_vol: f64,
        max_vol: f64,
    ) {
        // Sample every other point to avoid overcrowding
        let peak_points: Vec<(f64, f64)> = (0..n_exp)
            .step_by(2)
            .flat_map(|i| {
                (0..n_strike).step_by(2).filter_map(move |j| {
                    let vol = surface_data[i][j];
                    let vol_norm = (vol - min_vol) / (max_vol - min_vol);

                    // Only highlight top 30% of volatility values
                    if vol_norm > 0.7 {
                        Some(self.project_normalized_point(
                            f64::from(j as u32) / f64::from((n_strike - 1) as u32),
                            f64::from(i as u32) / f64::from((n_exp - 1) as u32),
                            vol_norm,
                        ))
                    } else {
                        None
                    }
                })
            })
            .collect();

        if !peak_points.is_empty() {
            ctx.draw(&Points {
                coords: &peak_points,
                color: self.palette.get_color(0.9),
            });
        }
    }

    /// Project a row of data to 2D screen points.
    fn project_row_to_points(
        &self,
        row: &[f64],
        row_idx: usize,
        n_exp: usize,
        n_strike: usize,
        min_vol: f64,
        max_vol: f64,
    ) -> Vec<(f64, f64)> {
        let exp_norm = f64::from(row_idx as u32) / f64::from((n_exp - 1) as u32);

        row.iter()
            .enumerate()
            .take(n_strike)
            .map(|(j, &vol)| {
                let strike_norm = f64::from(j as u32) / f64::from((n_strike - 1) as u32);
                let vol_norm = (vol - min_vol) / (max_vol - min_vol);
                self.project_normalized_point(strike_norm, exp_norm, vol_norm)
            })
            .collect()
    }

    /// Project a column of data to 2D screen points.
    fn project_column_to_points(
        &self,
        surface_data: &[Vec<f64>],
        col_idx: usize,
        n_exp: usize,
        n_strike: usize,
        min_vol: f64,
        max_vol: f64,
    ) -> Vec<(f64, f64)> {
        let strike_norm = f64::from(col_idx as u32) / f64::from((n_strike - 1) as u32);

        surface_data
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let exp_norm = f64::from(i as u32) / f64::from((n_exp - 1) as u32);
                let vol = row[col_idx];
                let vol_norm = (vol - min_vol) / (max_vol - min_vol);
                self.project_normalized_point(strike_norm, exp_norm, vol_norm)
            })
            .collect()
    }

    /// Project normalized coordinates (0.0-1.0) to 2D screen space.
    fn project_normalized_point(
        &self,
        strike_norm: f64,
        exp_norm: f64,
        vol_norm: f64,
    ) -> (f64, f64) {
        // Center the surface at origin by subtracting 0.5, then scale to world space
        let x = (strike_norm - 0.5) * 3.0; // Strike: ±1.5 units
        let y = (exp_norm - 0.5) * 3.0; // Expiry: ±1.5 units
        let z = (vol_norm - 0.5) * 2.0; // Height: ±1.0 units (less tall to avoid distortion)
        self.project(x, y, z)
    }

    /// Draw a series of connected line segments.
    fn draw_line_strip(ctx: &mut Context, points: &[(f64, f64)], color: Color) {
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

    /// Cycle to the next color palette.
    pub const fn cycle_palette(&mut self) {
        self.palette = self.palette.next();
    }
}
