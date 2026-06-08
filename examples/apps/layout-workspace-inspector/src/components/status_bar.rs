use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Rect, Size};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use ratatui_layout::{
    FrameSnapshot, LayoutParticipant, MeasureConstraint, MeasureContext, Regions, RenderContext,
    Row, SizeHint,
};

use crate::domain::BoardSummary;
use crate::ids::{PaneId, TargetId};
use crate::ui::margin;

/// Renderer for the release-board status bar.
///
/// The status bar is visually simple, but it demonstrates an important coordination pattern:
/// measured content can still produce a `Regions`. The component measures summary chips through
/// a small `LayoutParticipant`, renders each chip into the planned row, and returns pane regions so
/// the frame snapshot records where the summary region existed.
#[derive(Debug, Default)]
pub(crate) struct StatusBar;

impl StatusBar {
    /// Renders summary chips and returns their frame-local region data.
    ///
    /// The component does not own release data. `App` passes a `BoardSummary` each frame, which
    /// keeps domain aggregation in `ReleaseBoard` and lets this component stay focused on layout
    /// and rendering.
    pub(crate) fn render(
        frame: &mut Frame,
        area: Rect,
        summary: BoardSummary,
    ) -> FrameSnapshot<TargetId> {
        Self::render_shell(frame, area);

        let inner = area.inner(margin(1, 1));
        let mut participant = StatusChips::new(summary);
        let chips = Self::plan_chips(inner, &participant, participant.len());
        Self::render_chips(frame, &mut participant, &chips);

        FrameSnapshot::from_layout(chips.map_id(|id| TargetId::Pane(PaneId::Status(id))))
    }

    /// Draws the status bar border and title.
    fn render_shell(frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("release board"),
            area,
        );
    }

    /// Plans chip regions from the participant's measured preferred widths.
    fn plan_chips(
        area: Rect,
        participant: &impl LayoutParticipant<usize>,
        count: usize,
    ) -> Regions<usize> {
        let constraints = (0..count).map(|id| {
            let size = participant
                .measure(id, MeasureConstraint::Unbounded, MeasureContext)
                .clamped_preferred();
            Constraint::Length(size.width)
        });
        Row::new(constraints)
            .spacing(2)
            .flex(Flex::Start)
            .regions(area)
    }

    /// Renders each chip into the region chosen by the region set.
    fn render_chips(
        frame: &mut Frame,
        participant: &mut impl LayoutParticipant<usize>,
        chips: &Regions<usize>,
    ) {
        for region in chips.regions() {
            participant.render(
                region.id,
                region.area,
                frame.buffer_mut(),
                RenderContext::default(),
            );
        }
    }
}

/// Measurable and renderable status-chip content.
///
/// `StatusBar` owns layout orchestration. `StatusChips` owns the small content contract needed by
/// `LayoutParticipant`: measuring each label and rendering it with the color that communicates the
/// count category.
#[derive(Debug, Clone)]
struct StatusChips {
    /// Chip labels in visual order.
    labels: [String; 3],
}

impl StatusChips {
    /// Builds the summary labels from release-board counts.
    fn new(summary: BoardSummary) -> Self {
        Self {
            labels: [
                format!("items {}", summary.total),
                format!("running {}", summary.running),
                format!("blocked {}", summary.blocked),
            ],
        }
    }

    /// Returns the number of status chips.
    const fn len(&self) -> usize {
        self.labels.len()
    }

    /// Returns the display color for one summary chip.
    const fn color(id: usize) -> Color {
        match id {
            1 => Color::Green,
            2 => Color::Yellow,
            _ => Color::Cyan,
        }
    }
}

impl LayoutParticipant<usize> for StatusChips {
    /// Measures a chip from its label width.
    fn measure(&self, id: usize, _: MeasureConstraint, _: MeasureContext) -> SizeHint {
        SizeHint::exact(Size::new(self.labels[id].len() as u16 + 2, 1))
    }

    /// Renders one status chip into the area assigned by the status-bar row layout.
    fn render(&mut self, id: usize, area: Rect, buf: &mut Buffer, _: RenderContext) {
        Paragraph::new(self.labels[id].as_str())
            .style(Style::new().fg(Self::color(id)))
            .render(area, buf);
    }
}
