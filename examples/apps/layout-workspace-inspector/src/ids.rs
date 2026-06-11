use ratatui_layout::table::CellPosition;

/// Stable target identities for frame-local routing.
///
/// The generic `Id` used by frame snapshots is the app's routing vocabulary. This enum is the normal
/// case for an app with several regions because it prevents collisions between targets that come
/// from different parts of the screen: tree nodes, table cells, commands, dialog fields, and whole
/// panes.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum TargetId {
    /// A whole pane or non-item region.
    Pane(PaneId),

    /// A project-tree row identified by domain node id.
    TreeNode(NodeId),

    /// A table cell identified by row/header and column position.
    QueueCell(CellPosition),

    /// A footer command.
    Command(CommandId),

    /// A field or action inside the edit dialog.
    Dialog(DialogField),
}

impl TargetId {
    /// Returns the pane that owns this target.
    ///
    /// This lets shared styling code ask "is this pane focused?" even when focus is currently on a
    /// child item such as a tree row or table cell.
    pub(crate) const fn pane(self) -> PaneId {
        match self {
            Self::Pane(pane) => pane,
            Self::TreeNode(_) => PaneId::Tree,
            Self::QueueCell(_) => PaneId::Queue,
            Self::Command(_) => PaneId::Commands,
            Self::Dialog(_) => PaneId::Dialog,
        }
    }

    /// Returns the pane that should receive mouse-wheel scroll for this target.
    ///
    /// Scroll routing is coarser than click routing. A queue cell and blank queue pane space both scroll
    /// the queue viewport; a tree row and blank tree pane space both scroll the tree.
    pub(crate) const fn scroll_pane(self) -> Option<PaneId> {
        match self {
            Self::TreeNode(_) | Self::Pane(PaneId::Tree) => Some(PaneId::Tree),
            Self::QueueCell(_) | Self::Pane(PaneId::Queue) => Some(PaneId::Queue),
            Self::Pane(PaneId::Details) => Some(PaneId::Details),
            _ => None,
        }
    }
}

/// Named vertical regions in the top-level page layout.
///
/// These ids travel with the page row constraints in `Column::named`, which keeps the top-level
/// render method from depending on numeric region positions.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum PageSlot {
    /// Summary bar at the top of the screen.
    Status,

    /// Main content row that holds the project tree, work queue, and details pane.
    Body,

    /// Footer command strip.
    Footer,
}

/// Named horizontal regions inside the page body.
///
/// These ids make the nested row layout read as page structure: project navigation on the left,
/// work queue in the middle, details on the right.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum BodySlot {
    /// Project tree pane.
    Project,

    /// Work queue pane.
    Queue,

    /// Details viewport pane.
    Details,
}

/// Coarse screen regions used for pane highlighting and overlay routing.
///
/// `PaneId` is intentionally less detailed than `TargetId`. It answers questions about page regions,
/// such as which border should be highlighted, without caring which exact row or field is focused.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum PaneId {
    /// One of the summary chips in the status bar.
    Status(usize),

    /// The project tree pane.
    Tree,

    /// The work queue pane.
    Queue,

    /// The detail log pane.
    Details,

    /// The footer command strip.
    Commands,

    /// The edit dialog overlay.
    Dialog,

    /// The help overlay.
    Help,
}

/// Domain ids for nodes in the project tree.
///
/// The tree stores selection by `NodeId`, not by row index. That is the important example pattern:
/// rows are transient render data, while domain ids can survive expansion, filtering, or sorting.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum NodeId {
    /// Root release-train node.
    Workspace,

    /// Services group.
    Services,

    /// API service node.
    Api,

    /// Worker service node.
    Worker,

    /// Documentation project node.
    Docs,

    /// Release project node.
    Release,
}

impl NodeId {
    /// Returns the label rendered for a tree node.
    ///
    /// The labels match the release-board filters. Keeping them on the enum gives navigation,
    /// filtering, and rendering one shared vocabulary.
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Workspace => "release train",
            Self::Services => "runtime services",
            Self::Api => "api gateway",
            Self::Worker => "worker fleet",
            Self::Docs => "docs portal",
            Self::Release => "release ops",
        }
    }
}

/// Footer commands that can be focused, clicked, or triggered by key bindings.
///
/// Commands use their own enum so command routing is independent from labels and grid positions.
/// That makes it straightforward to disable, reorder, or restyle commands later.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum CommandId {
    /// Open the edit dialog for the selected item.
    Edit,

    /// Mark the selected item as running.
    Run,

    /// Mark the selected item as done.
    Mark,

    /// Toggle the help overlay.
    Help,
}

impl CommandId {
    /// Commands rendered in the footer grid, in left-to-right display order.
    pub(crate) const ALL: [Self; 4] = [Self::Edit, Self::Run, Self::Mark, Self::Help];

    /// Reports whether this command can run with the current selection state.
    ///
    /// Item commands need a selected queue item. Help is a global command, so it remains available
    /// even before the queue has selected a row.
    pub(crate) const fn enabled(self, selected_item_exists: bool) -> bool {
        match self {
            Self::Edit | Self::Run | Self::Mark => selected_item_exists,
            Self::Help => true,
        }
    }

    /// Returns the compact footer label for a command.
    ///
    /// Labels include their shortcut because this example has no separate menu system. The command
    /// id remains the source of behavior; the label is only presentation.
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Edit => "e edit",
            Self::Run => "r run",
            Self::Mark => "m mark done",
            Self::Help => "? help",
        }
    }

    /// Returns the footer cell width for this command.
    ///
    /// The command id owns this small presentation value so the command strip can derive its grid from
    /// `CommandId::ALL` instead of keeping a parallel column-count constant.
    pub(crate) const fn width(self) -> u16 {
        match self {
            Self::Edit | Self::Run | Self::Mark | Self::Help => 14,
        }
    }
}

/// Focusable rows in the edit dialog.
///
/// Text fields, a status picker, and buttons share one enum because they share one focus order.
/// `EditDialog` narrows this enum to text-only controls when behavior applies only to editable
/// fields.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum DialogField {
    /// Editable item title.
    Title,

    /// Editable item owner.
    Owner,

    /// Editable item status label.
    Status,

    /// Save button.
    Save,

    /// Cancel button.
    Cancel,
}

impl DialogField {
    /// Returns the label rendered beside a dialog field.
    ///
    /// Button labels also come through this helper so the enum remains the single presentation
    /// vocabulary for the dialog.
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Owner => "owner",
            Self::Status => "status",
            Self::Save => "save",
            Self::Cancel => "cancel",
        }
    }

    /// Returns the terminal width occupied by the field label and separator.
    ///
    /// Cursor placement uses this to put the terminal cursor after the label and current field
    /// value, without duplicating label formatting in the dialog renderer.
    pub(crate) const fn label_width(self) -> u16 {
        self.label().len() as u16 + 2
    }
}
