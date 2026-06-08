//! Page-level rectangle planning for the release-board screen.
//!
//! The app has a stable screen structure: status at the top, commands at the bottom, and three panes
//! in the body. Keeping that layout calculation in one helper lets `App::render` read as component
//! rendering instead of interleaving every constraint with every pane.

use ratatui::layout::{Constraint, Rect};
use ratatui_layout::{Column, Row};

use crate::ids::{BodySlot, PageSlot};

/// Rectangles assigned to the top-level page regions for one frame.
#[derive(Debug, Clone, Copy)]
pub(super) struct PageAreas {
    /// Summary bar at the top of the screen.
    pub(super) status: Rect,

    /// Project navigation pane.
    pub(super) project: Rect,

    /// Work queue pane.
    pub(super) queue: Rect,

    /// Detail viewport pane.
    pub(super) details: Rect,

    /// Footer command strip.
    pub(super) footer: Rect,
}

impl PageAreas {
    /// Plans the page using named regions instead of numeric indexes.
    pub(super) fn regions(area: Rect) -> Self {
        let page = Column::named(Self::page_slots()).regions(area);
        let body_area = page.area_for(PageSlot::Body).expect("body");
        let body = Row::named(Self::body_slots()).spacing(1).regions(body_area);
        Self {
            status: page.area_for(PageSlot::Status).expect("status"),
            project: body.area_for(BodySlot::Project).expect("project"),
            queue: body.area_for(BodySlot::Queue).expect("queue"),
            details: body.area_for(BodySlot::Details).expect("details"),
            footer: page.area_for(PageSlot::Footer).expect("footer"),
        }
    }

    /// Returns vertical constraints for page-level regions.
    const fn page_slots() -> [(PageSlot, Constraint); 3] {
        [
            (PageSlot::Status, Constraint::Length(3)),
            (PageSlot::Body, Constraint::Fill(1)),
            (PageSlot::Footer, Constraint::Length(3)),
        ]
    }

    /// Returns horizontal constraints for the body panes.
    const fn body_slots() -> [(BodySlot, Constraint); 3] {
        [
            (BodySlot::Project, Constraint::Length(24)),
            (BodySlot::Queue, Constraint::Fill(2)),
            (BodySlot::Details, Constraint::Fill(1)),
        ]
    }
}
