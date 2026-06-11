use ratatui::Frame;
use ratatui::layout::{Position, Rect, Size};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::frame::{FrameSnapshot, FrameTargets};
use ratatui_layout::scroll::ScrollMetrics;
use ratatui_layout::viewport::{Viewport, ViewportLayout, ViewportState};

use crate::DETAILS_FOCUS;
use crate::domain::DetailView;
use crate::ids::{PaneId, TargetId};
use crate::ui::{margin, offset_u16, pane_style_for};

/// Stateful controller for the details pane.
///
/// The pane owns only viewport state. It receives the selected item each frame from `App` so it can
/// render detail text without knowing how queue selection is implemented.
///
/// Details is intentionally simpler than the tree and queue: it has one focus target for the whole
/// viewport instead of one target per visible line. Keyboard movement scrolls `ViewportState`, and
/// the next render clamps that desired offset against the text height. The component still returns a
/// `FrameSnapshot` so focus can land on the pane and mouse routing can treat the viewport as a real
/// region.
#[derive(Debug)]
pub(crate) struct DetailsPane {
    /// Scroll offset for the detail log viewport.
    viewport: ViewportState,
}

#[allow(
    clippy::unused_self,
    reason = "region phase helpers stay as methods so the example reads by component"
)]
impl DetailsPane {
    /// Creates a details pane scrolled to the start of the log.
    ///
    /// The selected item is not stored here. `App` supplies it each frame so details always reflect
    /// the queue's current selection.
    pub(crate) fn new() -> Self {
        Self {
            viewport: ViewportState::default(),
        }
    }

    /// Renders the details pane and returns a single focus target for scrolling.
    ///
    /// The render flow is: draw the shell, build text from the selected item, lay out the viewport,
    /// render visible lines, render scroll metrics, and return one focusable routed target for the
    /// viewport.
    pub(crate) fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        focused: Option<TargetId>,
        details: DetailView<'_>,
    ) -> FrameSnapshot<TargetId> {
        self.render_shell(frame, area, focused);

        let inner = area.inner(margin(1, 1));
        let viewport = self.layout_viewport(inner, details);
        self.render_body(frame, details, &viewport);
        self.render_metrics(frame, area, &viewport);

        self.frame_snapshot(&viewport)
    }

    /// Draws the details border and title.
    ///
    /// Like the other panes, details receives global focus identity from `App` and uses it only to
    /// choose border styling.
    fn render_shell(&self, frame: &mut Frame, area: Rect, focused: Option<TargetId>) {
        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .title("details")
                .border_style(pane_style_for(focused, PaneId::Details)),
            area,
        );
    }

    /// Calculates the visible viewport for the current detail text.
    ///
    /// `Viewport` owns the offset clamping. Input code may request a scroll offset beyond the
    /// content bounds; layout turns that request into a valid visible rectangle.
    fn layout_viewport(&mut self, area: Rect, details: DetailView<'_>) -> ViewportLayout {
        let content_size = Size::new(area.width, details.line_count().max(1) as u16);
        Viewport::new(content_size).layout(area, &mut self.viewport)
    }

    /// Renders the visible log lines inside the viewport.
    ///
    /// `DetailView` supplies only the borrowed lines needed for the visible slice. This mirrors what a
    /// custom widget would do after receiving a `ViewportLayout`.
    fn render_body(&self, frame: &mut Frame, details: DetailView<'_>, viewport: &ViewportLayout) {
        let body = details.visible_lines(
            viewport.offset.y as usize,
            viewport.viewport.height as usize,
        );
        frame.render_widget(Paragraph::new(Text::from(body)), viewport.viewport);
    }

    /// Renders the log scroll metrics in the pane footer.
    ///
    /// `ScrollMetrics` translates viewport math into user-facing status text. The same metrics
    /// could feed a scrollbar widget in a fuller app.
    fn render_metrics(&self, frame: &mut Frame, area: Rect, viewport: &ViewportLayout) {
        let metrics = ScrollMetrics::vertical(viewport);
        let footer = Rect::new(
            area.x + 1,
            area.bottom().saturating_sub(1),
            area.width - 2,
            1,
        );
        frame.render_widget(
            Paragraph::new(format!(
                "log offset {} / {}",
                metrics.offset, metrics.max_offset
            ))
            .style(Style::new().fg(Color::DarkGray)),
            footer,
        );
    }

    /// Builds the focus target collection for scrolling the whole details viewport.
    ///
    /// The whole viewport is one target because details has no per-line activation. `FrameTargets`
    /// still records the region in layout, mouse, and focus target collections so focus traversal and wheel routing
    /// can treat it like the richer tree and queue panes.
    fn frame_snapshot(&self, viewport: &ViewportLayout) -> FrameSnapshot<TargetId> {
        let id = TargetId::Pane(PaneId::Details);
        FrameTargets::new(viewport.viewport, DETAILS_FOCUS)
            .region(id, viewport.viewport)
            .clip_to(viewport.viewport)
    }

    /// Adjusts the desired log scroll offset.
    ///
    /// The method does not know the content height. It records the user's desired movement, and
    /// `layout_viewport` clamps the result during the next render.
    pub(crate) const fn scroll(&mut self, delta: isize) {
        let y = offset_u16(self.viewport.offset.y, delta);
        self.viewport.offset = Position::new(self.viewport.offset.x, y);
    }
}
