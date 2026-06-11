use super::tree_rows::TreeRows;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders};
use ratatui_layout::container::{Container, ContainerLayout, Padding};
use ratatui_layout::frame::{FrameSnapshot, FrameTargets};
use ratatui_layout::list::{ListHeightCache, ListLayout, VirtualList, VirtualListState};
use ratatui_layout::pointer::PointerState;
use ratatui_layout::selection::{SelectionMode, SelectionState};

use crate::TREE_FOCUS;
use crate::domain::{NavigationView, VisibleNode};
use crate::ids::{NodeId, PaneId, TargetId};
use crate::ui::pane_style_for;

/// Stateful controller for the project tree pane.
///
/// The tree owns its virtual-list state, measurement cache, and stable domain selection. `App`
/// still decides where the pane is placed and how routed tree ids affect the rest of the app.
///
/// Rendering happens in three phases. First, `layout` reserves an inner content rectangle inside the
/// pane border. Second, `render_rows` gives the visible navigation nodes to `VirtualList`, which
/// handles vertical clipping and row placement. Third, `frame_snapshot` turns the rows that were
/// actually visible into layout regions, mouse targets, and focus targets. Clicking a row routes a
/// `TargetId::TreeNode`, and keyboard movement changes the tree's `SelectionState<NodeId>`.
#[derive(Debug)]
pub(crate) struct ProjectTree {
    /// Scroll and selected-row state owned by the virtual list.
    state: VirtualListState,

    /// Cached row heights for variable-height list support.
    cache: ListHeightCache,

    /// Stable domain selection for the tree.
    selection: SelectionState<NodeId>,
}

#[allow(
    clippy::unused_self,
    reason = "region phase helpers stay as methods so the example reads by component"
)]
impl ProjectTree {
    /// Creates a project tree with the release-train node selected.
    pub(crate) fn new() -> Self {
        let mut selection = SelectionState::new(SelectionMode::Single);
        selection.select(NodeId::Workspace);
        Self {
            state: VirtualListState::default(),
            cache: ListHeightCache::new(),
            selection,
        }
    }

    /// Renders the tree pane and returns the routed data produced by its visible rows.
    ///
    /// This is the component's top-level render method. It draws the pane shell, synchronizes the
    /// virtual-list selected row from stable `NodeId` selection, renders only visible rows, and then
    /// returns a frame snapshot that lets the next mouse click or focus traversal find those rows.
    pub(crate) fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        focused: Option<TargetId>,
        mouse: &PointerState<TargetId>,
        navigation: NavigationView,
    ) -> FrameSnapshot<TargetId> {
        let container = self.layout(area);
        self.render_shell(frame, container.outer, focused);

        self.prepare_state_for_layout(navigation);
        let visible = navigation.nodes();
        let layout = self.render_rows(frame, container.inner, visible, mouse);

        self.frame_snapshot(container, visible, &layout)
    }

    /// Calculates the padded content area for the tree.
    ///
    /// The outer rectangle belongs to the border and title. The inner rectangle belongs to the
    /// virtual list and also becomes the clipping boundary for row targets.
    fn layout(&self, area: Rect) -> ContainerLayout<()> {
        Container::<()>::new()
            .padding(Padding::new(1, 1, 1, 1))
            .layout(area)
    }

    /// Draws the tree border and title.
    ///
    /// The border color is derived from the current focused target's pane. Focus may be on a child
    /// tree row, so `pane_style_for` maps routed ids back to their owning pane.
    fn render_shell(&self, frame: &mut Frame, area: Rect, focused: Option<TargetId>) {
        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .title("project")
                .border_style(pane_style_for(focused, PaneId::Tree)),
            area,
        );
    }

    /// Renders visible tree rows through `VirtualList`.
    ///
    /// `TreeRows` receives the stable selection and hover id so row rendering can style selected
    /// and hovered rows without performing hit testing itself. Hit testing is recorded later from
    /// the `ListLayout` returned by the virtual list.
    fn render_rows(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        visible: &[VisibleNode],
        mouse: &PointerState<TargetId>,
    ) -> ListLayout {
        let mut rows = TreeRows {
            visible,
            selected: self.selection.primary(),
            hovered: self.hovered(mouse),
        };
        VirtualList::new().render_cached(
            area,
            frame.buffer_mut(),
            &mut self.state,
            &mut rows,
            &mut self.cache,
        )
    }

    /// Converts visible row metadata into layout, mouse, and focus target collections.
    ///
    /// The list layout tells us which domain rows were visible and where they landed. This method
    /// converts those transient row rectangles into `TargetId::TreeNode` regions, mouse targets for
    /// clicks, and focus targets for keyboard traversal. The plan is clipped to the inner content
    /// area so hidden rows cannot be routed.
    fn frame_snapshot(
        &self,
        container: ContainerLayout<()>,
        visible: &[VisibleNode],
        layout: &ListLayout,
    ) -> FrameSnapshot<TargetId> {
        let regions = layout.visible_items.iter().map(|item| {
            let node = visible[item.index].id;
            ratatui_layout::Region::new(node, item.area)
        });
        FrameTargets::new(container.outer, TREE_FOCUS)
            .mouse_region(TargetId::Pane(PaneId::Tree), container.inner)
            .build_focusable(regions, TargetId::TreeNode)
            .clip_to(container.inner)
    }

    /// Converts stable domain selection into the virtual list's visible row index.
    ///
    /// The tree stores selection as `NodeId` because row positions can change when a real tree is
    /// expanded, collapsed, or filtered. `VirtualListState` still needs a visible row index, so this
    /// method bridges durable domain selection into the list's frame-local row model.
    fn prepare_state_for_layout(&mut self, navigation: NavigationView) {
        let selected = navigation
            .index_of(navigation.selected_or_root(self.selection.primary()))
            .unwrap_or_default();
        if self.state.scrolls_selected_into_view() {
            self.state.select(Some(selected));
        } else {
            self.state.select_without_scrolling(Some(selected));
        }
    }

    /// Returns the hovered domain node, if the mouse is over a tree row.
    fn hovered(&self, mouse: &PointerState<TargetId>) -> Option<NodeId> {
        mouse.hovered().and_then(|id| match id {
            TargetId::TreeNode(node) => Some(node),
            _ => None,
        })
    }

    /// Selects a tree node by stable domain id.
    ///
    /// Mouse release and keyboard activation both call this after routing has already identified
    /// which `TargetId::TreeNode` was activated.
    pub(crate) fn select(&mut self, node: NodeId, navigation: NavigationView) {
        self.selection.select(node);
        let selected = navigation.index_of(node).unwrap_or_default();
        self.state.select(Some(selected));
    }

    /// Moves selection through the currently visible tree rows.
    ///
    /// Arrow-key movement uses the same visible navigation order that rendering used. The result is
    /// stored back as `NodeId`, then mirrored into `VirtualListState` so the next render keeps the
    /// selected row visible.
    pub(crate) fn move_selection(&mut self, delta: isize, navigation: NavigationView) {
        let next = navigation.move_from(self.selection.primary(), delta);
        self.selection.select(next);
        self.state
            .select(Some(navigation.index_of(next).unwrap_or_default()));
    }

    /// Scrolls the tree viewport without changing the selected domain node.
    ///
    /// `VirtualList` will clamp the requested position during the next render. Selection remains
    /// stable so the queue filter does not change just because the user used the wheel.
    pub(crate) const fn scroll(&mut self, delta: isize) {
        self.state.scroll_viewport_by(delta);
    }

    /// Returns the selected domain node used to filter the work queue.
    ///
    /// This is the main way the tree affects another component. `App` asks for the selected node,
    /// then asks `ReleaseBoard` for queue items visible under that node.
    pub(crate) fn selected_node(&self, navigation: NavigationView) -> NodeId {
        navigation.selected_or_root(self.selection.primary())
    }
}
