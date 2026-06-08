mod mode;
mod mouse_action;
mod page_action;
mod page_layout;
mod target_action;

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::{DefaultTerminal, Frame};
use ratatui_layout::{FocusFallback, FocusState, FrameSnapshot, PointerState};

use crate::components::{
    CommandStrip, DetailsPane, DialogOutcome, EditDialog, HelpOverlay, ProjectTree, StatusBar,
    WorkQueue,
};
use crate::domain::{DetailView, ItemId, ReleaseBoard, ReleaseItem};
use crate::ids::{CommandId, DialogField, PaneId, TargetId};

use self::mode::AppMode;
use self::mouse_action::{MouseInput, MousePosition, mouse_input_for_event};
use self::page_action::{PageAction, page_action_for_key};
use self::page_layout::PageAreas;
use self::target_action::{FocusedRegion, TargetAction, target_action_for_id};

/// Application state for the whole showcase.
///
/// This struct deliberately separates app-owned state from frame-local values. Selections, scroll
/// offsets, and modal state live here because they must survive across frames. Geometry, focus
/// targets, and mouse targets are recomputed during each render and stored in `previous_frame` only
/// so the next input event can be routed.
///
/// `App` is the coordinator, not a rendering primitive. It decides where components live on the
/// screen, asks each component to render into its assigned `Rect`, merges the returned
/// `FrameSnapshot`s, and then uses the previous frame's focus and pointer target collections during input handling.
/// Selection is deliberately split by ownership: the tree owns the selected navigation node, the
/// queue owns the selected item id and active column, and `ReleaseBoard` owns domain data.
#[derive(Debug)]
pub(crate) struct App {
    /// UI data produced by the last render pass.
    ///
    /// Mouse routing and keyboard focus traversal use this instead of trying to recompute geometry
    /// during input handling. That keeps input code backend-agnostic and avoids duplicating layout
    /// math outside rendering.
    previous_frame: FrameSnapshot<TargetId>,

    /// Current keyboard focus.
    ///
    /// `FocusState` owns only the focused id. The visible traversal order is produced each frame
    /// by `FocusTargets`, which lets disabled or hidden controls disappear naturally.
    focus: FocusState<TargetId>,

    /// Current pointer state.
    ///
    /// The state stores hover and press identity. It does not know widget geometry; it asks the
    /// previous frame's `PointerTargets` which target is under a terminal position.
    mouse: PointerState<TargetId>,

    /// Project tree region, including virtual-list state and domain selection.
    tree: ProjectTree,

    /// Work queue region, including selected item, active column, and scroll state.
    queue: WorkQueue,

    /// Footer command strip, including last-activated command state.
    commands: CommandStrip,

    /// Details region, including log viewport state.
    details: DetailsPane,

    /// Current interaction mode.
    mode: AppMode,

    /// Domain data shown by the navigation tree, work queue, and detail pane.
    board: ReleaseBoard,
}

impl App {
    /// Creates stable app state before any frame has been rendered.
    ///
    /// The first `previous_frame` is empty because there is no UI to route against yet. After the
    /// first draw, `render` replaces it with the real geometry, focus, mouse, and cursor requests.
    pub(crate) fn new() -> Self {
        Self {
            previous_frame: FrameSnapshot::new(Rect::default()),
            focus: FocusState::default(),
            mouse: PointerState::default(),
            tree: ProjectTree::new(),
            queue: WorkQueue::new(),
            commands: CommandStrip::new(),
            details: DetailsPane::new(),
            mode: AppMode::Page,
            board: ReleaseBoard::sample(),
        }
    }

    /// Runs the draw-then-event loop after terminal setup is complete.
    ///
    /// `main` owns terminal setup because mouse capture is a terminal concern. `App::run` owns the
    /// application loop so the example still reads as: initialize terminal, render a frame, route
    /// one event through the previous frame's data, repeat.
    pub(crate) fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            match event::read()? {
                event::Event::Key(key) if key.is_press() && !self.handle_key(key) => {
                    break Ok(());
                }
                event::Event::Mouse(mouse) => self.handle_mouse(mouse),
                _ => {}
            }
        }
    }

    /// Renders one frame and stores the UI data needed by the next input event.
    ///
    /// This method is the main teaching point of the example. Each region draws ordinary Ratatui
    /// widgets into a `Rect`, then returns a `FrameSnapshot` describing the ids, focus targets, mouse
    /// targets, and cursor requests that rendering created. The app merges those child values into a
    /// single frame snapshot and keeps it for input routing.
    fn render(&mut self, frame: &mut Frame) {
        let mut frame_snapshot = FrameSnapshot::new(frame.area());
        let page = PageAreas::regions(frame.area());

        frame_snapshot = frame_snapshot.merge(self.render_status(frame, page.status));
        frame_snapshot = frame_snapshot.merge(self.render_tree(frame, page.project));
        frame_snapshot = frame_snapshot.merge(self.render_queue(frame, page.queue));
        frame_snapshot = frame_snapshot.merge(self.render_details(frame, page.details));
        frame_snapshot = frame_snapshot.merge(self.render_commands(frame, page.footer));
        if let AppMode::Editing(dialog) = &self.mode {
            frame_snapshot =
                frame_snapshot.merge(Self::render_dialog(frame, dialog, self.focus.focused()));
        }
        if matches!(self.mode, AppMode::Help) {
            frame_snapshot = frame_snapshot.merge(HelpOverlay::render(frame));
        }

        self.focus
            .ensure_visible(&frame_snapshot.focus, FocusFallback::First);
        if let Some(cursor) = frame_snapshot.cursor.final_cursor() {
            frame.set_cursor_position(cursor.position);
        }
        self.previous_frame = frame_snapshot;
    }

    /// Renders board summary counts through the status-bar component.
    fn render_status(&self, frame: &mut Frame, area: Rect) -> FrameSnapshot<TargetId> {
        StatusBar::render(frame, area, self.board.summary())
    }

    /// Renders the project tree and exposes each visible node as a routed target.
    ///
    /// The tree combines several primitives: `Container` reserves padding, `VirtualList` renders
    /// only visible rows, `SelectionState` tracks the selected domain node, `PointerTargets` makes rows
    /// clickable, and `FocusTargets` gives rows a keyboard traversal order.
    fn render_tree(&mut self, frame: &mut Frame, area: Rect) -> FrameSnapshot<TargetId> {
        let navigation = Self::navigation_view();
        self.tree
            .render(frame, area, self.focus.focused(), &self.mouse, navigation)
    }

    /// Renders the virtual work queue and exposes body cells as focusable targets.
    ///
    /// The header cells participate in mouse hit testing and layout, but only body cells are added
    /// to the focus target collection. That distinction is a common pattern for queues with clickable headers
    /// but row-oriented keyboard selection.
    fn render_queue(&mut self, frame: &mut Frame, area: Rect) -> FrameSnapshot<TargetId> {
        let view = self.board.queue_view(self.selected_node());
        self.queue.prepare_for_view(&view);
        self.queue
            .render(frame, area, self.focus.focused(), &self.mouse, &view)
    }

    /// Renders the details pane as a manually scrollable viewport.
    ///
    /// This pane has one focus target for the whole log instead of a target per visible line. That
    /// keeps keyboard behavior simple: when the pane is focused, up and down scroll the viewport.
    fn render_details(&mut self, frame: &mut Frame, area: Rect) -> FrameSnapshot<TargetId> {
        let selected = self.selected_item_id().and_then(|id| self.board.item(id));
        self.details
            .render(frame, area, self.focus.focused(), DetailView::new(selected))
    }

    /// Returns the release item selected by the queue.
    ///
    /// `WorkQueue` owns the stable selected item id. `App` resolves that id through the domain
    /// board so render and command paths do not depend on the table's current visible row
    /// numbers.
    fn selected_item(&self) -> Option<&ReleaseItem> {
        let id = self.selected_item_id()?;
        self.board.item(id)
    }

    /// Renders the command strip as a typed grid of actions.
    ///
    /// This demonstrates a compact toolbar pattern: a `Grid` gives each command a stable cell,
    /// disabled commands remain visible, and both focus and pointer target collections use the same command ids.
    fn render_commands(&self, frame: &mut Frame, area: Rect) -> FrameSnapshot<TargetId> {
        self.commands.render(
            frame,
            area,
            self.focus.focused(),
            &self.mouse,
            self.selected_item().is_some(),
        )
    }

    /// Renders the modal edit dialog and requests a terminal cursor for text fields.
    ///
    /// The dialog shows how overlays differ from normal panes. It clears and draws over the page,
    /// assigns a higher z index to its regions, contributes its own focus targets, and uses
    /// `CursorRequests` to place the terminal cursor at the active field.
    fn render_dialog(
        frame: &mut Frame,
        dialog: &EditDialog,
        focused: Option<TargetId>,
    ) -> FrameSnapshot<TargetId> {
        dialog.render(frame, focused)
    }

    /// Routes a key press through the current app mode.
    ///
    /// Returning `false` asks the outer event loop to quit. Most keys use the previous frame's
    /// focus target collection so traversal follows what was actually visible during the last draw.
    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match &self.mode {
            AppMode::Editing(_) => return self.handle_dialog_key(key),
            AppMode::Help => return self.handle_help_key(key),
            AppMode::Page => {}
        }
        let action = page_action_for_key(key);
        self.apply_page_action(action)
    }

    /// Applies page-level intent to app state.
    fn apply_page_action(&mut self, action: PageAction) -> bool {
        match action {
            PageAction::None => {}
            PageAction::Quit => return false,
            PageAction::FocusNext => self.focus_next(),
            PageAction::FocusPrevious => self.focus_previous(),
            PageAction::Move { row, column } => self.move_active(row, column),
            PageAction::ActivateFocused => self.activate_focused(),
            PageAction::Activate(target) => self.activate(target),
            PageAction::Focus(target) => self.focus.focus(Some(target)),
        }
        true
    }

    /// Handles keys while the help overlay owns input.
    fn handle_help_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('?') | KeyCode::Esc => self.mode = AppMode::Page,
            KeyCode::Char('q') => return false,
            _ => {}
        }
        true
    }

    /// Handles keys while the edit dialog is open.
    ///
    /// The dialog owns local text editing and focus movement rules. It returns a `DialogOutcome`
    /// whenever app-level state should change.
    fn handle_dialog_key(&mut self, key: KeyEvent) -> bool {
        if let AppMode::Editing(dialog) = &mut self.mode {
            let outcome = dialog.handle_key(self.focus.focused(), key);
            self.apply_dialog_outcome(outcome);
        }
        true
    }

    /// Applies the app-level side effect requested by dialog key handling.
    fn apply_dialog_outcome(&mut self, outcome: DialogOutcome) {
        match outcome {
            DialogOutcome::Continue => {}
            DialogOutcome::Focus(target) => self.focus.focus(Some(target)),
            DialogOutcome::Save => self.save_dialog(),
            DialogOutcome::Cancel => self.cancel_dialog(),
        }
    }

    /// Moves focus to the next target in the page-level focus target collection.
    ///
    /// This small helper keeps every normal traversal path using the same plan. It also reads
    /// better at call sites than repeating `self.previous_frame.focus` throughout input code.
    fn focus_next(&mut self) {
        self.focus.next(&self.previous_frame.focus);
    }

    /// Moves focus to the previous target in the page-level focus target collection.
    ///
    /// Shift-Tab can arrive as either `BackTab` or `Tab` with a shift modifier depending on the
    /// terminal, so both key shapes flow into this one helper.
    fn focus_previous(&mut self) {
        self.focus.previous(&self.previous_frame.focus);
    }

    /// Routes backend mouse events through the backend-agnostic previous frame snapshot.
    ///
    /// `ratatui-layout` deals in positions, targets, and ids. The only crossterm-specific work here
    /// is decoding the event into [`MouseInput`].
    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        self.apply_mouse_input(mouse_input_for_event(mouse));
    }

    /// Applies pointer input through the previous frame's pointer data.
    ///
    /// Hover, press, and release use `PointerState` because they need to remember pointer state across
    /// events. Wheel input routes directly to a scrollable pane and intentionally does not change row
    /// selection.
    fn apply_mouse_input(&mut self, input: MouseInput) {
        match input {
            MouseInput::Hover(position) => {
                self.mouse.hover(&self.previous_frame.mouse, position);
            }
            MouseInput::Press(position) => self.press_mouse(position),
            MouseInput::Release(position) => self.release_mouse(position),
            MouseInput::Scroll { position, delta } => self.scroll_at(position, delta),
            MouseInput::None => {}
        }
    }

    /// Records a mouse press and focuses the pressed target.
    fn press_mouse(&mut self, position: MousePosition) {
        if let Some(hit) = self
            .mouse
            .press(&self.previous_frame.mouse, position)
            .or_else(|| self.previous_frame.route_click(position))
        {
            self.focus.focus(Some(hit.id));
        }
    }

    /// Activates a target only when press and release resolve to the same routed id.
    fn release_mouse(&mut self, position: MousePosition) {
        if let Some(hit) = self.mouse.release(&self.previous_frame.mouse, position) {
            self.focus.focus(Some(hit.id));
            self.activate(hit.id);
        }
    }

    /// Routes a vertical mouse-wheel event to the pane under the pointer.
    ///
    /// Wheel input uses the previous frame's data, just like clicks. Tree and queue scrolling
    /// adjusts viewport offsets only; selection stays stable until the user clicks or uses keyboard
    /// selection.
    fn scroll_at(&mut self, position: MousePosition, delta: isize) {
        let Some(hit) = self.previous_frame.route_scroll(position) else {
            return;
        };
        match hit.id.scroll_pane() {
            Some(PaneId::Tree) => {
                self.tree.scroll(delta);
            }
            Some(PaneId::Queue) => {
                self.queue.scroll_rows(delta);
            }
            Some(PaneId::Details) => {
                self.details.scroll(delta);
            }
            _ => {}
        }
    }

    /// Moves inside the currently focused region.
    ///
    /// Focus chooses the active interaction model. A tree moves by rows, a queue moves by rows and
    /// columns, the detail pane scrolls, and the command strip uses focus traversal.
    fn move_active(&mut self, row_delta: isize, column_delta: isize) {
        match FocusedRegion::from_target(self.focus.focused()) {
            FocusedRegion::Tree => self.move_tree(row_delta),
            FocusedRegion::Queue => self.move_queue(row_delta, column_delta),
            FocusedRegion::Details => self.scroll_log(row_delta),
            FocusedRegion::Commands => self.move_command_focus(column_delta),
            FocusedRegion::Other => self.focus_next(),
        }
    }

    /// Moves focus horizontally through the footer commands.
    fn move_command_focus(&mut self, column_delta: isize) {
        if column_delta >= 0 {
            self.focus_next();
        } else {
            self.focus_previous();
        }
    }

    /// Moves project-tree selection over the currently visible domain ids.
    ///
    /// This uses `SelectionState<NodeId>` instead of storing only an index so a later expandable
    /// tree can keep selection attached to the same node when visibility changes.
    fn move_tree(&mut self, delta: isize) {
        self.tree.move_selection(delta, Self::navigation_view());
    }

    /// Moves queue selection and focuses the newly selected body cell.
    ///
    /// The queue stores durable item selection, but keyboard movement still returns the rendered
    /// cell position so app focus can stay aligned with the table target.
    fn move_queue(&mut self, row_delta: isize, column_delta: isize) {
        let view = self.board.queue_view(self.selected_node());
        let Some(position) = self.queue.move_selection(row_delta, column_delta, &view) else {
            return;
        };
        self.focus.focus(Some(TargetId::QueueCell(position)));
    }

    /// Adjusts the details viewport's desired scroll offset.
    ///
    /// The viewport will clamp this offset during layout, so input code can keep this helper small
    /// and let rendering decide the valid maximum for the current content and area size.
    const fn scroll_log(&mut self, delta: isize) {
        self.details.scroll(delta);
    }

    /// Activates whichever id currently has keyboard focus.
    ///
    /// Keyboard activation and mouse release both flow into `activate`, which keeps click and
    /// keyboard behavior aligned.
    fn activate_focused(&mut self) {
        if let Some(id) = self.focus.focused() {
            self.activate(id);
        }
    }

    /// Performs the action associated with a routed UI id.
    ///
    /// This is where frame-local ids become app behavior. Geometry and hit testing have already
    /// happened; activation only needs to interpret the id in domain terms.
    fn activate(&mut self, id: TargetId) {
        let action = target_action_for_id(id);
        self.apply_target_action(action);
    }

    /// Applies behavior represented by an activated target.
    fn apply_target_action(&mut self, action: TargetAction) {
        match action {
            TargetAction::None => {}
            TargetAction::SelectTree(node) => self.tree.select(node, Self::navigation_view()),
            TargetAction::SelectQueue(position) => {
                let view = self.board.queue_view(self.selected_node());
                self.queue.select(position, &view);
                self.focus.focus(Some(TargetId::QueueCell(position)));
            }
            TargetAction::RunCommand(command) => self.run_command_if_enabled(command),
            TargetAction::FocusDialog(field) => self.focus.focus(Some(TargetId::Dialog(field))),
            TargetAction::SaveDialog => self.save_dialog(),
            TargetAction::CancelDialog => self.cancel_dialog(),
        }
    }

    /// Runs a footer command after it has been activated.
    ///
    /// Commands operate on domain state, not on rendered cells. The queue already stores durable
    /// item identity, so command handlers can mutate the release board without reconstructing
    /// table rows.
    fn run_command(&mut self, command: CommandId) {
        self.commands.select(command);
        match command {
            CommandId::Edit => self.open_dialog(),
            CommandId::Run => {
                if let Some(item) = self.selected_item_mut() {
                    item.start_manual_run();
                }
            }
            CommandId::Mark => {
                if let Some(item) = self.selected_item_mut() {
                    item.mark_done_from_command();
                }
            }
            CommandId::Help => self.mode = AppMode::Help,
        }
    }

    /// Runs a command only when the current app state allows it.
    ///
    /// Mouse clicks, focused activation, and keyboard shortcuts all flow here through routed command
    /// ids, so disabled command policy is applied once.
    fn run_command_if_enabled(&mut self, command: CommandId) {
        if command.enabled(self.selected_item().is_some()) {
            self.run_command(command);
        }
    }

    /// Opens the edit dialog with a copy of the selected item.
    ///
    /// Copying the values into dialog state gives the user a real cancel path. It also keeps the
    /// text cursor and field focus independent from the work queue.
    fn open_dialog(&mut self) {
        if let Some(item) = self.selected_item().cloned() {
            self.mode = AppMode::Editing(EditDialog::open(&item));
            self.focus.focus(Some(TargetId::Dialog(DialogField::Title)));
        }
    }

    /// Cancels the edit dialog without applying temporary field values.
    ///
    /// Escape always flows here while the dialog is open. The focused cancel button and mouse
    /// activation use the same path so cancel behavior stays consistent.
    fn cancel_dialog(&mut self) {
        self.mode = AppMode::Page;
    }

    /// Applies dialog edits to the selected item and closes the dialog.
    ///
    /// The selected item id is looked up again on save. In a real app this is where you would
    /// likely validate input and dispatch a command rather than mutating local demo data
    /// directly.
    fn save_dialog(&mut self) {
        let AppMode::Editing(dialog) = std::mem::replace(&mut self.mode, AppMode::Page) else {
            return;
        };
        if let Some(item) = self.selected_item_mut() {
            let dialog = dialog.into_state();
            let title = dialog.title;
            let owner = dialog.owner;
            let status = dialog.status;
            item.apply_edit(title, owner, status);
        }
    }

    /// Returns the stable release item id selected by the work queue.
    ///
    /// The queue still renders frame-local cell positions, but it stores selection by durable item
    /// id so command handlers do not treat a visible row number as application identity.
    const fn selected_item_id(&self) -> Option<ItemId> {
        self.queue.selected_item_id()
    }

    /// Returns the selected navigation node used to filter the queue.
    fn selected_node(&self) -> crate::ids::NodeId {
        self.tree.selected_node(Self::navigation_view())
    }

    /// Returns a mutable reference to the selected release-board item.
    ///
    /// This mirrors `selected_item` for commands that mutate data. Centralizing the id lookup keeps
    /// the command handlers from repeating queue-cell edge cases.
    fn selected_item_mut(&mut self) -> Option<&mut ReleaseItem> {
        let id = self.selected_item_id()?;
        self.board.item_mut(id)
    }

    /// Returns the navigation rows used by the tree and queue filter.
    ///
    /// The sample navigation is static, but keeping this helper on `App` keeps tree selection, queue
    /// filtering, and activation call sites from depending on where the view currently comes from.
    const fn navigation_view() -> crate::domain::NavigationView {
        ReleaseBoard::navigation_view()
    }
}

#[cfg(test)]
mod tests {
    use super::{App, AppMode};
    use crate::app::page_action::PageAction;
    use crate::ids::{CommandId, TargetId};

    #[test]
    fn page_actions_focus_targets_without_running_commands() {
        let mut app = App::new();

        let keep_running =
            app.apply_page_action(PageAction::Focus(TargetId::Command(CommandId::Edit)));

        assert!(keep_running);
        assert_eq!(
            app.focus.focused(),
            Some(TargetId::Command(CommandId::Edit))
        );
        assert!(matches!(app.mode, AppMode::Page));
    }

    #[test]
    fn page_actions_open_help_and_quit() {
        let mut app = App::new();

        assert!(app.apply_page_action(PageAction::Activate(TargetId::Command(CommandId::Help))));
        assert!(matches!(app.mode, AppMode::Help));
        assert!(!app.apply_page_action(PageAction::Quit));
    }

    #[test]
    fn mouse_scroll_routes_without_changing_queue_selection() {
        use crate::app::mouse_action::MouseInput;
        use ratatui::layout::Rect;
        use ratatui_layout::FrameSnapshot;

        let mut app = App::new();
        let queue_area = Rect::new(0, 0, 20, 10);
        app.previous_frame = FrameSnapshot::new(queue_area)
            .scroll_region(TargetId::Pane(crate::ids::PaneId::Queue), queue_area);
        let before = app.selected_item_id();

        app.apply_mouse_input(MouseInput::Scroll {
            position: (1, 1),
            delta: 1,
        });

        assert_eq!(app.selected_item_id(), before);
        assert_eq!(app.queue.row_scroll(), 1);
    }
}
