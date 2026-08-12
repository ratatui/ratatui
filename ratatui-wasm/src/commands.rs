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
            Self::Line { x, y, line } => {
                let rect = Rect::new(area.x + x, area.y + y, area.width - x, 1);
                line.render(rect, buf);
            }
            Self::Clear(style) => {
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

#[cfg(test)]
mod tests {
    use ratatui_core::buffer::Buffer;
    use ratatui_core::layout::Rect;
    use ratatui_core::style::{Color, Style};
    use ratatui_core::text::Line;

    use super::*;

    #[test]
    fn line_renders_text_inside_area() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 3));
        DrawCommand::Line {
            x: 1,
            y: 1,
            line: Line::raw("hi"),
        }
        .render(Rect::new(0, 0, 10, 3), &mut buf);

        let text: String = buf.content()[10..20]
            .iter()
            .map(ratatui_core::buffer::Cell::symbol)
            .collect();
        assert!(text.trim_start().starts_with("hi"), "got {text:?}");
    }

    #[test]
    fn line_outside_area_is_clipped() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 5, 2));
        DrawCommand::Line {
            x: 0,
            y: 5,
            line: Line::raw("hello"),
        }
        .render(Rect::new(0, 0, 5, 2), &mut buf);

        let text: String = buf
            .content()
            .iter()
            .map(ratatui_core::buffer::Cell::symbol)
            .collect();
        assert!(!text.contains('h'), "out-of-bounds line should be clipped");
    }

    #[test]
    fn line_past_width_is_clipped() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 5, 2));
        DrawCommand::Line {
            x: 3,
            y: 0,
            line: Line::raw("hello"),
        }
        .render(Rect::new(0, 0, 5, 2), &mut buf);

        let text: String = buf.content()[0..5]
            .iter()
            .map(ratatui_core::buffer::Cell::symbol)
            .collect();
        assert_eq!(text.get(3..), Some("he"), "got {text:?}");
    }

    #[test]
    fn clear_paints_spaces_with_style() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 4, 2));
        let style = Style::new().fg(Color::Red).bg(Color::Blue);
        DrawCommand::Clear(style).render(Rect::new(0, 0, 4, 2), &mut buf);

        for y in 0..2 {
            for x in 0..4 {
                assert_eq!(buf[(x, y)].symbol(), " ");
                assert_eq!(buf[(x, y)].style(), style);
            }
        }
    }

    #[test]
    fn empty_line_does_not_panic() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 5, 1));
        DrawCommand::Line {
            x: 0,
            y: 0,
            line: Line::raw(""),
        }
        .render(Rect::new(0, 0, 5, 1), &mut buf);
    }
}
