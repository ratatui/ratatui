use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Widget};
use ratatui_layout::list::{ListItemContext, ListItems};
use ratatui_layout::participant::MeasureContext;

use crate::domain::VisibleNode;
use crate::ids::NodeId;

/// Adapter that lets `VirtualList` render project tree rows.
///
/// `VirtualList` owns the viewport math, but the app still owns row content and styling. This
/// adapter is the boundary between those two responsibilities.
///
/// `TreeRows` deliberately has no input handling. It only receives the selected and hovered domain
/// ids and turns each visible node into text and style. `ProjectTree::frame_snapshot` records the mouse
/// and focus targets after `VirtualList` reports the row rectangles.
pub(crate) struct TreeRows<'a> {
    /// Visible domain rows to render.
    pub(crate) visible: &'a [VisibleNode],

    /// Currently selected domain node.
    pub(crate) selected: Option<NodeId>,

    /// Node under the pointer, if the previous frame routed hover to a tree row.
    pub(crate) hovered: Option<NodeId>,
}

impl ListItems for TreeRows<'_> {
    /// Reports the number of visible rows available to the virtual list.
    ///
    /// The list uses this to decide which indices can be selected, measured, and rendered.
    fn len(&self) -> usize {
        self.visible.len()
    }

    /// Reports the height for one tree row.
    ///
    /// This example uses fixed-height rows, but the method exists so virtual lists can support rows
    /// that wrap or expand differently at different widths.
    fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
        1
    }

    /// Renders one visible tree row into the area chosen by `VirtualList`.
    ///
    /// Selection and hover are passed in from app state. The row renderer does not perform hit
    /// testing; the surrounding `render_tree` method records mouse and focus targets for each row.
    fn render_item(&mut self, index: usize, area: Rect, buf: &mut Buffer, _: ListItemContext) {
        let node = self.visible[index];
        let style = if self.selected == Some(node.id) {
            Style::new().fg(Color::Black).bg(Color::Cyan)
        } else if self.hovered == Some(node.id) {
            Style::new().fg(Color::Yellow)
        } else {
            Style::new()
        };
        let prefix = if node.depth == 0 { "▾" } else { "•" };
        Paragraph::new(format!(
            "{}{} {}",
            "  ".repeat(node.depth),
            prefix,
            node.id.label()
        ))
        .style(style)
        .render(area, buf);
    }
}
