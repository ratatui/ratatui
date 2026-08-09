//! `ratatui-wasm` — a capability-aware WebAssembly host for Ratatui widgets.
//!
//! This crate lets you load `.wasm` components that implement the
//! `ratatui:widget/widget` interface and render them onto a [`ratatui_core::buffer::Buffer`].

#![warn(missing_docs)]

use ratatui_core::buffer::{Buffer, Cell as RatatuiCell};
use ratatui_core::layout::{Alignment, Rect};
use ratatui_core::style::{Color, Modifier, Style};
use ratatui_core::text::{Line, Span};

use crate::generated::exports::ratatui::widget::widget::{
    Alignment as WitAlignment, Color as WitColor, Line as WitLine, RenderCommand, Span as WitSpan,
    Style as WitStyle,
};

pub mod commands;
pub mod event;
pub mod host;
pub mod manifest;

mod cache;

pub use host::{PluginWidget, StatefulWasmWidget, WasmWidget, WasmWidgetHost};

/// Re-exported WIT interface types shared between host and guest widgets.
pub mod wit {
    pub use super::generated::exports::ratatui::widget::widget::{
        Alignment, Cell, Color, Event, KeyCode, KeyEvent, Line, Rect, RenderCommand, RenderResult,
        ResizeEvent, RgbColor, Span, Style, WidgetError,
    };
}

#[allow(missing_docs)]
mod generated {
    wasmtime::component::bindgen!({
        path: "wit/widget.wit",
        world: "wasm-widget",
    });
}

use self::generated::exports::ratatui::widget::widget::Rect as WitRect;

/// Convert a host-side `Rect` into the WIT `rect` representation.
pub const fn rect_to_wit(area: Rect) -> WitRect {
    WitRect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: area.height,
    }
}

/// Convert a WIT color into a Ratatui [`Color`].
pub const fn convert_color(color: WitColor) -> Color {
    match color {
        WitColor::Reset => Color::Reset,
        WitColor::Black => Color::Black,
        WitColor::Red => Color::Red,
        WitColor::Green => Color::Green,
        WitColor::Yellow => Color::Yellow,
        WitColor::Blue => Color::Blue,
        WitColor::Magenta => Color::Magenta,
        WitColor::Cyan => Color::Cyan,
        WitColor::Gray => Color::Gray,
        WitColor::DarkGray => Color::DarkGray,
        WitColor::LightRed => Color::LightRed,
        WitColor::LightGreen => Color::LightGreen,
        WitColor::LightYellow => Color::LightYellow,
        WitColor::LightBlue => Color::LightBlue,
        WitColor::LightMagenta => Color::LightMagenta,
        WitColor::LightCyan => Color::LightCyan,
        WitColor::White => Color::White,
        WitColor::Rgb(rgb) => Color::Rgb(rgb.r, rgb.g, rgb.b),
    }
}

/// Convert a WIT style into a Ratatui [`Style`].
pub const fn convert_style(style: &WitStyle) -> Style {
    let mut host = Style::new();
    if let Some(fg) = &style.fg {
        host = host.fg(convert_color(*fg));
    }
    if let Some(bg) = &style.bg {
        host = host.bg(convert_color(*bg));
    }
    if style.bold {
        host = host.add_modifier(Modifier::BOLD);
    }
    if style.italic {
        host = host.add_modifier(Modifier::ITALIC);
    }
    if style.underline {
        host = host.add_modifier(Modifier::UNDERLINED);
    }
    host
}

fn convert_span(span: &WitSpan) -> Span<'static> {
    match &span.style {
        Some(style) => Span::styled(span.content.clone(), convert_style(style)),
        None => Span::raw(span.content.clone()),
    }
}

const fn convert_alignment(alignment: WitAlignment) -> Alignment {
    match alignment {
        WitAlignment::Left => Alignment::Left,
        WitAlignment::Center => Alignment::Center,
        WitAlignment::Right => Alignment::Right,
    }
}

/// Blit a list of draw commands returned by a guest into a Ratatui [`Buffer`].
pub fn blit_commands(area: Rect, commands: &[RenderCommand], buf: &mut Buffer) {
    for cmd in commands {
        match cmd {
            RenderCommand::Cell(cell) => blit_cell(area, cell, buf),
            RenderCommand::Line(line) => blit_line(area, line, buf),
        }
    }
}

fn blit_cell(
    area: Rect,
    cell: &crate::generated::exports::ratatui::widget::widget::Cell,
    buf: &mut Buffer,
) {
    let abs_x = area.x.saturating_add(cell.x);
    let abs_y = area.y.saturating_add(cell.y);
    if abs_x >= area.right() || abs_y >= area.bottom() {
        return;
    }
    let mut style = Style::new();
    if let Some(fg) = &cell.fg {
        style = style.fg(convert_color(*fg));
    }
    if let Some(bg) = &cell.bg {
        style = style.bg(convert_color(*bg));
    }
    let mut host_cell = RatatuiCell::default();
    host_cell.set_symbol(&cell.symbol);
    host_cell.set_style(style);
    buf[(abs_x, abs_y)] = host_cell;
}

fn blit_line(area: Rect, line: &WitLine, buf: &mut Buffer) {
    let y = area.y.saturating_add(line.y);
    if y >= area.bottom() {
        return;
    }

    let spans: Vec<Span<'static>> = line.spans.iter().map(convert_span).collect();
    let mut host_line = Line::from(spans);
    if let Some(alignment) = line.alignment {
        host_line = host_line.alignment(convert_alignment(alignment));
    }

    let line_width = u16::try_from(host_line.width()).unwrap_or(u16::MAX);
    let x = match line.alignment {
        Some(WitAlignment::Center) => area.x + (area.width.saturating_sub(line_width)) / 2,
        Some(WitAlignment::Right) => area.right().saturating_sub(line_width),
        _ => area.x,
    };

    buf.set_line(x, y, &host_line, area.width);
}
