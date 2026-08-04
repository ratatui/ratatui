//! `ratatui-wasm` — a capability-aware WebAssembly host for Ratatui widgets.
//!
//! This crate lets you load `.wasm` components that implement the
//! `ratatui:widget/widget` interface and render them onto a [`ratatui_core::buffer::Buffer`].

#![warn(missing_docs)]

use ratatui_core::buffer::{Buffer, Cell as RatatuiCell};
use ratatui_core::layout::Rect;
use ratatui_core::style::{Color, Style};

pub mod commands;
pub mod event;
pub mod host;

pub use event::{Event, KeyCode, KeyEvent};
pub use host::{PluginWidget, WasmWidget, WasmWidgetHost};

#[allow(missing_docs)]
mod generated {
    wasmtime::component::bindgen!({
        path: "wit/widget.wit",
        world: "wasm-widget",
    });
}

pub use generated::*;

use self::exports::ratatui::widget::widget::{Cell, Rect as WitRect};

/// Convert a host-side `Rect` into the WIT `rect` representation.
pub fn rect_to_wit(area: Rect) -> WitRect {
    WitRect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: area.height,
    }
}

/// Blit a list of WIT cells returned by a guest into a Ratatui [`Buffer`].
pub fn blit_cells(area: Rect, commands: &[Cell], buf: &mut Buffer) {
    for cmd in commands {
        let abs_x = area.x.saturating_add(cmd.x);
        let abs_y = area.y.saturating_add(cmd.y);
        if abs_x >= area.right() || abs_y >= area.bottom() {
            continue;
        }
        let mut style = Style::new();
        if let Some(fg) = parse_color(cmd.fg.as_deref()) {
            style = style.fg(fg);
        }
        if let Some(bg) = parse_color(cmd.bg.as_deref()) {
            style = style.bg(bg);
        }
        let mut cell = RatatuiCell::default();
        cell.set_symbol(&cmd.symbol);
        cell.set_style(style);
        buf[(abs_x, abs_y)] = cell;
    }
}

fn parse_color(s: Option<&str>) -> Option<Color> {
    let s = s?;
    if s.starts_with('#') && s.len() == 7 {
        let r = u8::from_str_radix(&s[1..3], 16).ok()?;
        let g = u8::from_str_radix(&s[3..5], 16).ok()?;
        let b = u8::from_str_radix(&s[5..7], 16).ok()?;
        Some(Color::Rgb(r, g, b))
    } else {
        None
    }
}
