use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use crate::ids::NodeId;

/// One visible tree row after expansion and filtering have been applied.
///
/// The example tree is static, but this type shows the boundary used by larger trees: produce a
/// flat visible list from hierarchical data, then let `VirtualList` render that flat list.
#[derive(Debug, Clone, Copy)]
pub(crate) struct VisibleNode {
    /// Stable domain id for selection and activation.
    pub(crate) id: NodeId,

    /// Indentation depth used only for rendering.
    pub(crate) depth: usize,
}

/// Stable id for a release-board item imported from another system.
///
/// The table renders filtered row positions, but commands should mutate records by stable identity.
/// This newtype makes that boundary explicit without making the UI carry external API strings
/// everywhere.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) struct ItemId(
    /// Import-order id used inside the app for stable lookup.
    pub(crate) u32,
);

/// Domain model behind the release operations console.
///
/// `ReleaseBoard` owns imported work items and exposes the view-shaped data that panes need:
/// navigation rows, filtered queue items, summary counts, and stable item lookup. That is the same
/// split a production app usually wants when it receives records from an API but renders a sorted or
/// filtered terminal view.
///
/// The board does not render anything. Its job is to keep domain identity stable while UI
/// components work with frame-local positions. The tree asks for navigation nodes, the queue asks
/// for borrowed items visible under the selected node, the status bar asks for summary counts, and
/// command handlers use the queue's selected `ItemId` before mutating data.
#[derive(Debug, Clone)]
pub(crate) struct ReleaseBoard {
    /// Work items in durable storage order.
    items: Vec<ReleaseItem>,
}

impl ReleaseBoard {
    /// Creates the sample board by converting wire records into domain records.
    ///
    /// The conversion step is intentionally visible in the example. It shows where an app would
    /// normalize status labels, component names, and external ids before the UI starts planning
    /// rectangles or routing input.
    pub(crate) fn sample() -> Self {
        let items = release_feed()
            .into_iter()
            .enumerate()
            .map(|(index, record)| ReleaseItem::from_feed(index as u32, record))
            .collect();
        Self { items }
    }

    /// Returns navigation rows with helpers for moving between visible ids and row indices.
    ///
    /// The tree renders visible rows, but selection should stay attached to stable `NodeId` values.
    /// This view keeps that conversion in one place so the component can talk in terms of movement and
    /// selection instead of repeating slice searches.
    pub(crate) const fn navigation_view() -> NavigationView {
        NavigationView::STATIC
    }

    /// Returns a queue-shaped view of work items under a selected navigation node.
    ///
    /// The queue receives borrowed domain rows instead of owning a copy. It renders these rows as
    /// visible table positions, while its persistent selection remains the stable `ItemId` stored on
    /// each `ReleaseItem`.
    pub(crate) fn queue_view(&self, node: NodeId) -> QueueView<'_> {
        QueueView::new(self, node)
    }

    /// Returns an immutable item by stable id.
    pub(crate) fn item(&self, id: ItemId) -> Option<&ReleaseItem> {
        self.items.iter().find(|item| item.id == id)
    }

    /// Returns a mutable item by stable id.
    pub(crate) fn item_mut(&mut self, id: ItemId) -> Option<&mut ReleaseItem> {
        self.items.iter_mut().find(|item| item.id == id)
    }

    /// Summarizes the board for the status bar.
    pub(crate) fn summary(&self) -> BoardSummary {
        BoardSummary {
            total: self.items.len(),
            running: self
                .items
                .iter()
                .filter(|item| item.status == Status::Running)
                .count(),
            blocked: self
                .items
                .iter()
                .filter(|item| item.status == Status::Blocked)
                .count(),
        }
    }
}

/// Navigation rows and stable-id movement helpers for the project tree.
///
/// The tree has two identities at the same time: a visible row index for virtualization and a stable
/// `NodeId` for selection. `NavigationView` keeps that bridge beside the domain data so the tree
/// component does not need to know how the navigation slice is searched or clamped.
#[derive(Debug, Clone, Copy)]
pub(crate) struct NavigationView {
    /// Visible navigation nodes in render order.
    nodes: &'static [VisibleNode],
}

impl NavigationView {
    /// Static navigation used by the self-contained example.
    const STATIC: Self = Self {
        nodes: &[
            VisibleNode {
                id: NodeId::Workspace,
                depth: 0,
            },
            VisibleNode {
                id: NodeId::Services,
                depth: 1,
            },
            VisibleNode {
                id: NodeId::Api,
                depth: 2,
            },
            VisibleNode {
                id: NodeId::Worker,
                depth: 2,
            },
            VisibleNode {
                id: NodeId::Docs,
                depth: 1,
            },
            VisibleNode {
                id: NodeId::Release,
                depth: 1,
            },
        ],
    };

    /// Returns visible navigation nodes in render order.
    pub(crate) const fn nodes(self) -> &'static [VisibleNode] {
        self.nodes
    }

    /// Returns the selected node, falling back to the workspace root.
    pub(crate) fn selected_or_root(self, selected: Option<NodeId>) -> NodeId {
        selected
            .filter(|node| self.index_of(*node).is_some())
            .unwrap_or(NodeId::Workspace)
    }

    /// Returns the visible row index for a stable navigation id.
    pub(crate) fn index_of(self, node: NodeId) -> Option<usize> {
        self.nodes.iter().position(|visible| visible.id == node)
    }

    /// Moves through visible navigation rows and returns the stable node at the destination.
    pub(crate) fn move_from(self, current: Option<NodeId>, delta: isize) -> NodeId {
        let current = current
            .and_then(|node| self.index_of(node))
            .unwrap_or_default();
        let next = crate::ui::offset_index(current, delta, self.nodes.len());
        self.nodes[next].id
    }
}

/// Filtered work-queue rows for one selected navigation node.
///
/// The queue UI still receives visible rows for rendering and keyboard movement, while command
/// handling mutates items by stable `ItemId`. This view borrows the rows visible under the current
/// tree filter without making the queue own or clone domain records.
#[derive(Debug, Clone)]
pub(crate) struct QueueView<'a> {
    /// Work items currently visible in the queue.
    items: Vec<&'a ReleaseItem>,
}

impl<'a> QueueView<'a> {
    /// Builds a filtered view from the board and selected navigation node.
    fn new(board: &'a ReleaseBoard, node: NodeId) -> Self {
        let items = board
            .items
            .iter()
            .filter(|item| item.visible_under(node))
            .collect();
        Self { items }
    }

    /// Returns borrowed release items for queue rendering.
    pub(crate) fn items(&self) -> &[&'a ReleaseItem] {
        &self.items
    }

    /// Returns the visible item at a row index.
    ///
    /// Keeping row lookup on the view keeps queue state from reaching into the borrowed row slice every
    /// time a mouse click or keyboard move needs to become a stable item id.
    pub(crate) fn item_at(&self, row: usize) -> Option<&'a ReleaseItem> {
        self.items.get(row).copied()
    }

    /// Returns the visible row for a stable item id.
    ///
    /// Selection is durable, but rendering is positional. This helper is the bridge from domain
    /// identity back to the current filtered table rows.
    pub(crate) fn position_of(&self, id: ItemId) -> Option<usize> {
        self.items.iter().position(|item| item.id == id)
    }

    /// Returns whether the view has no visible rows.
    pub(crate) const fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Detail-pane projection for a selected release item.
///
/// The details component needs line counts for viewport math and borrowed lines for rendering. This
/// view keeps that presentation-shaped projection beside the domain data so the pane does not clone
/// item fields or parse a formatted block on every render.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DetailView<'a> {
    /// Item currently selected by the queue.
    item: Option<&'a ReleaseItem>,
}

impl<'a> DetailView<'a> {
    /// Creates a details projection for an optional selected item.
    pub(crate) const fn new(item: Option<&'a ReleaseItem>) -> Self {
        Self { item }
    }

    /// Returns the total number of detail lines available for viewport layout.
    pub(crate) fn line_count(self) -> usize {
        self.item
            .map_or(1, |item| item.log.lines().count() + DETAIL_HEADER_LINES)
    }

    /// Returns the lines visible for the requested viewport slice.
    pub(crate) fn visible_lines(self, offset: usize, height: usize) -> Vec<Line<'a>> {
        self.lines().skip(offset).take(height).collect()
    }

    /// Returns all detail lines as a lazy iterator.
    ///
    /// Metadata still formats on demand, but the details pane can skip directly to the visible slice
    /// instead of allocating one full vector and then dropping most of it.
    fn lines(self) -> impl Iterator<Item = Line<'a>> {
        let title = self.item.map(|item| Line::from(item.title.as_str()));
        let metadata = self.item.into_iter().flat_map(|item| {
            [
                Line::from(vec![
                    Span::raw("component: "),
                    Span::raw(item.component.label()),
                ]),
                Line::from(vec![Span::raw("owner: "), Span::raw(item.owner.as_str())]),
                Line::from(vec![Span::raw("state: "), Span::raw(item.status.label())]),
                Line::from(format!("age: {}m", item.age)),
                Line::from(vec![
                    Span::raw("external ref: "),
                    Span::raw(item.external_ref.as_str()),
                ]),
                Line::from(""),
            ]
        });
        let log = self
            .item
            .into_iter()
            .flat_map(|item| item.log.lines().map(Line::from));
        let empty = self
            .item
            .is_none()
            .then(|| Line::from("No release item selected"));
        empty.into_iter().chain(title).chain(metadata).chain(log)
    }
}

/// Number of fixed metadata lines rendered before an item's log stream.
const DETAIL_HEADER_LINES: usize = 7;

/// Counts rendered in the top status bar.
///
/// The summary is calculated by the domain model so the status renderer can stay focused on layout
/// and styling rather than on data aggregation.
#[derive(Debug, Clone, Copy)]
pub(crate) struct BoardSummary {
    /// Total imported release items.
    pub(crate) total: usize,

    /// Items currently running.
    pub(crate) running: usize,

    /// Items blocked on a dependency or review.
    pub(crate) blocked: usize,
}

/// Release-board item shown in the virtual table.
///
/// This is intentionally plain domain data. It has no layout or interaction fields because the
/// example keeps rendering data in frame snapshots and persistent UI state in `App`.
#[derive(Debug, Clone)]
pub(crate) struct ReleaseItem {
    /// Stable id assigned when an external record is imported.
    pub(crate) id: ItemId,

    /// External reference shown in details and useful for API round trips.
    pub(crate) external_ref: String,

    /// Product area that owns the item.
    pub(crate) component: Component,

    /// Human-readable item title.
    pub(crate) title: String,

    /// Team or person responsible for the item.
    pub(crate) owner: String,

    /// Current item state, rendered with status-specific styling.
    pub(crate) status: Status,

    /// Item age in minutes.
    pub(crate) age: usize,

    /// Multi-line detail text shown in the scrollable details pane.
    pub(crate) log: String,
}

impl ReleaseItem {
    /// Converts an incoming feed record into normalized domain data.
    ///
    /// Real TUIs often receive JSON, protobuf, or database rows whose shape is not what the UI
    /// wants. This conversion keeps external naming and parsing outside the render path.
    pub(crate) fn from_feed(index: u32, record: FeedRecord) -> Self {
        Self {
            id: ItemId(index),
            external_ref: record.external_ref.to_string(),
            component: Component::from_feed(record.component),
            title: record.title,
            owner: record.owner.to_string(),
            status: Status::from_label(record.status),
            age: record.age,
            log: record.log.to_string(),
        }
    }

    /// Reports whether this item belongs under the selected navigation node.
    ///
    /// The root and group nodes are broader than leaf nodes. Encoding that here lets the tree
    /// selection drive table filtering without hard-coding filter rules in the table renderer.
    pub(crate) const fn visible_under(&self, node: NodeId) -> bool {
        match node {
            NodeId::Workspace => true,
            NodeId::Services => matches!(self.component, Component::Api | Component::Worker),
            NodeId::Api => matches!(self.component, Component::Api),
            NodeId::Worker => matches!(self.component, Component::Worker),
            NodeId::Docs => matches!(self.component, Component::Docs),
            NodeId::Release => matches!(self.component, Component::Release),
        }
    }

    /// Returns a compact id for the queue's first column.
    ///
    /// The details pane still shows the external reference in full. The queue uses a shorter code
    /// so the id column stays readable beside the item title in narrow terminals.
    pub(crate) fn short_ref(&self) -> String {
        let prefix = match self.component {
            Component::Api => "API",
            Component::Worker => "WRK",
            Component::Docs => "DOC",
            Component::Release => "OPS",
        };
        format!("{prefix}-{:02}", self.id.0)
    }

    /// Marks the item as manually started from the command strip.
    ///
    /// Naming this transition keeps command handling focused on user intent. The domain model owns
    /// the status change and the audit-log text that should travel with it.
    pub(crate) fn start_manual_run(&mut self) {
        self.status = Status::Running;
        self.log.push_str("\nmanual run requested");
    }

    /// Marks the item as done from the command strip.
    ///
    /// The status and log update are one domain transition, so callers should not need to remember
    /// to perform both field edits separately.
    pub(crate) fn mark_done_from_command(&mut self) {
        self.status = Status::Done;
        self.log.push_str("\nmarked done from command strip");
    }

    /// Applies values saved from the edit dialog.
    ///
    /// The dialog keeps status as user-entered text. The domain model owns parsing that text back
    /// into a trusted `Status` value before the item is stored.
    pub(crate) fn apply_edit(&mut self, title: String, owner: String, status: Status) {
        self.title = title;
        self.owner = owner;
        self.status = status;
        self.log.push_str("\nedited from dialog");
    }
}

/// Product area used for filtering and details.
///
/// Components are separate from tree nodes because the tree has both group nodes and leaf nodes.
/// That mirrors real navigation, where a route can represent a broad view rather than a single
/// stored value.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum Component {
    /// API gateway checks and deploy work.
    Api,

    /// Worker fleet checks and deploy work.
    Worker,

    /// Documentation and support-site work.
    Docs,

    /// Release coordination and approval work.
    Release,
}

impl Component {
    /// Converts a feed component label into a domain component.
    ///
    /// Unknown labels are treated as release operations so the board keeps imported records visible
    /// instead of dropping them during this demonstration.
    pub(crate) fn from_feed(label: &str) -> Self {
        match label {
            "api" => Self::Api,
            "worker" => Self::Worker,
            "docs" => Self::Docs,
            _ => Self::Release,
        }
    }

    /// Returns the label shown in the detail pane.
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Api => "api gateway",
            Self::Worker => "worker fleet",
            Self::Docs => "docs portal",
            Self::Release => "release ops",
        }
    }
}

/// Status values used by the work queue and summary bar.
///
/// The enum keeps status parsing, labels, and styling together so render code can ask for the
/// presentation it needs without duplicating match expressions.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum Status {
    /// Waiting to run.
    Queued,

    /// Currently running.
    Running,

    /// Completed successfully for the purposes of this demo.
    Done,

    /// Waiting on a human or external dependency.
    Blocked,
}

impl Status {
    /// Statuses shown by the edit dialog in cycling order.
    pub(crate) const ALL: [Self; 4] = [Self::Queued, Self::Running, Self::Done, Self::Blocked];

    /// Returns the lower-case label rendered in the table and details pane.
    ///
    /// The label is also accepted by `from_label`, which keeps the edit dialog simple: users type
    /// the same words that the UI displays.
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Done => "done",
            Self::Blocked => "blocked",
        }
    }

    /// Parses a status label from dialog input.
    ///
    /// Unknown values fall back to queued so the example stays forgiving. A production app would
    /// likely validate and keep the dialog open with an error message instead.
    pub(crate) fn from_label(label: &str) -> Self {
        match label.trim().to_ascii_lowercase().as_str() {
            "running" => Self::Running,
            "done" => Self::Done,
            "blocked" => Self::Blocked,
            _ => Self::Queued,
        }
    }

    /// Returns the next status in dialog cycling order.
    pub(crate) fn next(self) -> Self {
        let index = Self::ALL
            .iter()
            .position(|status| *status == self)
            .expect("status appears in ALL");
        Self::ALL[(index + 1) % Self::ALL.len()]
    }

    /// Returns the previous status in dialog cycling order.
    pub(crate) fn previous(self) -> Self {
        let index = Self::ALL
            .iter()
            .position(|status| *status == self)
            .expect("status appears in ALL");
        Self::ALL[index.checked_sub(1).unwrap_or(Self::ALL.len() - 1)]
    }

    /// Returns the color style used for status cells.
    ///
    /// The table applies this only when the cell is not selected, letting the selection background
    /// remain the strongest visual signal.
    pub(crate) const fn style(self) -> Style {
        match self {
            Self::Queued => Style::new().fg(Color::Yellow),
            Self::Running => Style::new().fg(Color::Green),
            Self::Done => Style::new().fg(Color::Blue),
            Self::Blocked => Style::new().fg(Color::Red),
        }
    }
}

/// Raw record shaped like data received from another service.
///
/// Keeping this separate from `ReleaseItem` demonstrates a common production boundary: imported
/// records can stay close to the transport format, while the UI consumes normalized domain data.
/// The conversion into `ReleaseItem` is where labels become enums and external ids become stable
/// app ids.
#[derive(Debug, Clone)]
pub(crate) struct FeedRecord {
    /// External id from the release tracking service.
    external_ref: &'static str,

    /// Component label from the feed.
    component: &'static str,

    /// Human-readable work item title.
    title: String,

    /// Owner label from the feed.
    owner: &'static str,

    /// Status label from the feed.
    status: &'static str,

    /// Age in minutes since the last state change.
    age: usize,

    /// Multi-line note stream for the details pane.
    log: &'static str,
}

/// Builds deterministic input records for the release board.
///
/// The records are static so the example remains self-contained. Their shape still resembles an
/// external feed, which keeps the example honest about how a real app would normalize received
/// data before rendering.
pub(crate) fn release_feed() -> Vec<FeedRecord> {
    let scenarios = [
        ("api", "schema compatibility sweep", "platform", "running"),
        ("api", "edge cache config promote", "traffic", "blocked"),
        ("worker", "queue drain rehearsal", "runtime", "queued"),
        ("worker", "backfill safety check", "data", "done"),
        ("docs", "operator guide publish", "docs", "queued"),
        ("release", "go/no-go review", "release", "blocked"),
    ];
    (0..48)
        .map(|index| {
            let (component, title, owner, status) = scenarios[index % scenarios.len()];
            let external_ref = match component {
                "api" => "REL-API",
                "worker" => "REL-WRK",
                "docs" => "REL-DOC",
                _ => "REL-OPS",
            };
            let wave = index / scenarios.len() + 1;
            FeedRecord {
                external_ref,
                component,
                title: format!("{title} wave {wave}"),
                owner,
                status,
                age: 4 + index * 3,
                log: if status == "blocked" {
                    "feed imported\nblocked by approval gate\nnext step: assign reviewer"
                } else if status == "running" {
                    "feed imported\nchecks started\nnext step: watch rollout metrics"
                } else {
                    "feed imported\ninputs normalized\nnext step: wait for release window"
                },
            }
        })
        .collect()
}
