use accesskit::{Action, Node, NodeId};

/// A self-contained chunk of accessibility nodes, produced by one of the
/// adapter functions (`list_nodes`, `table_nodes`, ...) and merged into a
/// [`TreeBuilder`](crate::TreeBuilder) with [`TreeBuilder::subtree`].
///
/// Adapters build these from plain data (strings, indices), not from
/// Ratatui widgets, so they have no idea what your app wants to happen when
/// an AT client activates a node -- see [`Self::add_action_to_children`] to
/// opt in.
#[derive(Debug)]
pub struct SubTree {
    /// The node other parents should link to (and pass to
    /// `TreeBuilder::focus`, via [`Self::selected`]).
    pub root: NodeId,
    /// The child node currently marked selected, if any. Callers building a
    /// selectable widget should generally pass this to
    /// `TreeBuilder::focus` so the AT client's focus follows the app's
    /// selection.
    pub selected: Option<NodeId>,
    nodes: Vec<(NodeId, Node)>,
}

impl SubTree {
    pub(crate) const fn new(root: NodeId, nodes: Vec<(NodeId, Node)>) -> Self {
        Self {
            root,
            selected: None,
            nodes,
        }
    }

    pub(crate) fn into_nodes(self) -> Vec<(NodeId, Node)> {
        self.nodes
    }

    /// Direct access to every node in this subtree (root included), for
    /// tweaks the adapter didn't anticipate.
    pub fn nodes_mut(&mut self) -> &mut [(NodeId, Node)] {
        &mut self.nodes
    }

    /// Advertises `action` on every child node (root excluded) so an AT
    /// client can invoke it. Adapters never do this on their own, an
    /// action with no handler behind it is a broken control for a screen
    /// reader user, so only add one you actually handle in
    /// [`A11y::actions`](crate::A11y::actions).
    pub fn add_action_to_children(&mut self, action: Action) -> &mut Self {
        for (id, node) in &mut self.nodes {
            if *id != self.root {
                node.add_action(action);
            }
        }
        self
    }
}
