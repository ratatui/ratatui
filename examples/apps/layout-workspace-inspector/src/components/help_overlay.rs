use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui_layout::{FrameSnapshot, Overlay, PointerTarget, PointerTargets, Region};

use crate::HELP_Z;
use crate::ids::{PaneId, TargetId};
use crate::ui::{centered, margin};

/// Stateless help overlay shown above the release-board page.
///
/// Help is modal like the edit dialog, but it has no editable state and no focusable controls. It
/// still contributes high-z layout and pointer data so clicks over the overlay do not reach page
/// controls drawn underneath.
pub(crate) struct HelpOverlay;

impl HelpOverlay {
    /// Renders help text and returns the overlay's frame-local routing data.
    pub(crate) fn render(frame: &mut Frame) -> FrameSnapshot<TargetId> {
        let area = centered(frame.area(), 58, 11);
        let inner = area.inner(margin(2, 1));
        frame.render_widget(Clear, area);
        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .title("help")
                .border_style(Style::new().fg(Color::Yellow)),
            area,
        );
        let text = [
            "Tab / Shift-Tab  move focus by region",
            "h j k l / arrows move inside focused pane",
            "Enter / Space    activate focused item",
            "e                edit selected item",
            "r                run selected item",
            "m                mark selected item done",
            "?                toggle this help",
            "Esc              close overlay or quit",
        ]
        .join("\n");
        frame.render_widget(Paragraph::new(text).wrap(Wrap { trim: false }), inner);
        Self::frame_snapshot(frame.area(), area)
    }

    /// Builds high-z layout and pointer data for the modal overlay.
    fn frame_snapshot(frame_area: Rect, area: Rect) -> FrameSnapshot<TargetId> {
        let plan = Overlay::new()
            .region(Region::new(TargetId::Pane(PaneId::Help), area).z(HELP_Z))
            .regions(frame_area);
        FrameSnapshot::from_layout(plan).mouse(PointerTargets::from_targets([PointerTarget::new(
            TargetId::Pane(PaneId::Help),
            area,
        )
        .z(HELP_Z)]))
    }
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;
    use ratatui_layout::{FrameSnapshot, Region, Regions};

    use super::HelpOverlay;
    use crate::ids::{PaneId, TargetId};

    #[test]
    fn help_overlay_routes_above_page_content() {
        let frame_area = Rect::new(0, 0, 80, 24);
        let overlay_area = Rect::new(10, 5, 40, 10);
        let page = FrameSnapshot::from_layout(Regions::from_regions(
            frame_area,
            [Region::new(TargetId::Pane(PaneId::Queue), overlay_area)],
        ));
        let frame = page.merge(HelpOverlay::frame_snapshot(frame_area, overlay_area));

        assert_eq!(
            frame.route_click((12, 6)).map(|hit| hit.id),
            Some(TargetId::Pane(PaneId::Help))
        );
    }
}
