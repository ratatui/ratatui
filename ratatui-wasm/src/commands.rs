//! Planned future extension: structured draw commands.
//!
//! Currently widgets return a flat list of cells. In the future this module may
//! support higher-level commands such as `Clear`, `SetLine`, `SetSpan`, etc.

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::style::Style;
use ratatui_core::text::Line;
use ratatui_core::widgets::Widget;

/// A high-level draw command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrawCommand {
    /// Write a single line at the given offset.
    Line {
        /// Horizontal offset inside the widget area.
        x: u16,
        /// Vertical offset inside the widget area.
        y: u16,
        /// Line content and style.
        line: Line<'static>,
    },
    /// Clear the entire widget area with the given style.
    Clear(Style),
}

impl DrawCommand {
    /// Apply this command to a buffer region.
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        match self {
            DrawCommand::Line { x, y, line } => {
                let rect = Rect::new(area.x + x, area.y + y, area.width - x, 1);
                line.render(rect, buf);
            }
            DrawCommand::Clear(style) => {
                for y in area.y..area.bottom() {
                    for x in area.x..area.right() {
                        buf[(x, y)].set_symbol(" ");
                        buf[(x, y)].set_style(*style);
                    }
                }
            }
        }
    }
}
