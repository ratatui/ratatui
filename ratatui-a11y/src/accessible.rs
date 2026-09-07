use accesskit::NodeId;

use crate::subtree::SubTree;

/// Opt-in extension point for a stateless widget to describe its own
/// accessibility subtree.
///
/// Implement this for your own widget type (or, if you're a widget crate,
/// behind a feature flag so `ratatui-a11y` stays optional for your users)
/// and build the returned [`SubTree`] with the adapter functions in this
/// crate (`list_nodes`, `table_nodes`, ...).
///
/// ```
/// use ratatui_a11y::{Accessible, NodeId, SubTree, list_nodes};
///
/// struct Menu {
///     items: Vec<String>,
///     selected: Option<usize>,
/// }
///
/// impl Accessible for Menu {
///     fn a11y_nodes(&self, id: NodeId) -> SubTree {
///         let _ = id; // menu roots itself; nothing to nest it under here
///         list_nodes(&self.items, self.selected, "menu")
///     }
/// }
/// ```
pub trait Accessible {
    /// Builds this widget's subtree. `id` is a caller-chosen key for this
    /// instance, fold it into any ids you derive so two instances of the
    /// same widget type don't collide.
    fn a11y_nodes(&self, id: NodeId) -> SubTree;
}

/// Same as [`Accessible`], for widgets whose accessible state lives in a
/// separate `State` type (mirrors Ratatui's `StatefulWidget` split).
pub trait StatefulAccessible {
    /// The state type this widget reads from (e.g. a selection index).
    type State;

    /// Builds this widget's subtree from `state`. See [`Accessible::a11y_nodes`].
    fn a11y_nodes(&self, state: &Self::State, id: NodeId) -> SubTree;
}
