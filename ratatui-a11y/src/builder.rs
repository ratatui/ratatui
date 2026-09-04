use std::collections::HashSet;

use accesskit::{Node, NodeId, Role, Tree, TreeId, TreeUpdate};

use crate::id::node_id;
use crate::subtree::SubTree;

/// Builds a [`TreeUpdate`] by hand, one frame at a time.
///
/// Ratatui has no retained widget tree to derive an accessibility tree
/// from (see the crate docs), so the app declares one explicitly, alongside
/// its normal rendering code. Build bottom-up: create child nodes first so
/// you have their ids on hand when you create the parent.
///
/// ```
/// use ratatui_a11y::{Role, TreeBuilder, node_id};
///
/// let window_id = node_id("window");
/// let item_id = node_id(("item", 0));
///
/// let mut tree = TreeBuilder::new();
/// tree.node(item_id, Role::ListItem, "First item", []);
/// tree.node(window_id, Role::Window, "My app", [item_id]);
/// tree.root(window_id);
/// tree.focus(window_id);
/// let update = tree.build();
/// ```
#[derive(Debug, Default)]
pub struct TreeBuilder {
    nodes: Vec<(NodeId, Node)>,
    root: Option<NodeId>,
    focus: Option<NodeId>,
}

impl TreeBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a node with the given role, label, and children, returning it
    /// for further configuration.
    pub fn node(
        &mut self,
        id: NodeId,
        role: Role,
        label: impl Into<String>,
        children: impl IntoIterator<Item = NodeId>,
    ) -> &mut Node {
        let mut node = Node::new(role);
        node.set_label(label.into());
        let children: Vec<NodeId> = children.into_iter().collect();
        if !children.is_empty() {
            node.set_children(children);
        }
        self.nodes.push((id, node));
        &mut self.nodes.last_mut().expect("just pushed").1
    }

    /// Merges a pre-built subtree into this builder and returns its root id,
    /// ready to use as a child elsewhere.
    pub fn subtree(&mut self, sub: SubTree) -> NodeId {
        let root = sub.root;
        self.nodes.extend(sub.into_nodes());
        root
    }

    /// Sets the tree's root node. Required before [`Self::build`].
    pub const fn root(&mut self, id: NodeId) -> &mut Self {
        self.root = Some(id);
        self
    }

    /// Sets the currently focused node. Defaults to the root if unset.
    pub const fn focus(&mut self, id: NodeId) -> &mut Self {
        self.focus = Some(id);
        self
    }

    /// Builds the [`TreeUpdate`].
    ///
    /// Forgetting to call [`Self::root`] never crashes the app: it serves
    /// an empty placeholder root instead. In debug builds this also runs a
    /// couple of `debug_assert!` checks duplicate node ids, and a focus
    /// id absent from this update both of which desync AT clients if
    /// shipped, so they're worth catching early in development.
    pub fn build(self) -> TreeUpdate {
        let mut nodes = self.nodes;
        let root = self.root.unwrap_or_else(|| {
            let id = node_id("ratatui-a11y:missing-root");
            nodes.push((id, Node::new(Role::Unknown)));
            id
        });
        let focus = self.focus.unwrap_or(root);

        #[cfg(debug_assertions)]
        {
            let mut seen = HashSet::with_capacity(nodes.len());
            for (id, _) in &nodes {
                debug_assert!(seen.insert(*id), "duplicate NodeId in TreeBuilder: {id:?}");
            }
            debug_assert!(
                focus == root || nodes.iter().any(|(id, _)| *id == focus),
                "TreeBuilder::focus set to a node not present in this update: {focus:?}"
            );
        }

        TreeUpdate {
            nodes,
            tree: Some(Tree::new(root)),
            tree_id: TreeId::ROOT,
            focus,
        }
    }
}

#[cfg(test)]
mod tests {
    use accesskit::Action;

    use super::*;

    #[test]
    fn focus_defaults_to_root() {
        let root = node_id("root");
        let mut tree = TreeBuilder::new();
        tree.node(root, Role::Window, "w", []);
        tree.root(root);
        let update = tree.build();
        assert_eq!(update.focus, root);
    }

    #[test]
    fn explicit_focus_wins() {
        let root = node_id("root");
        let item = node_id("item");
        let mut tree = TreeBuilder::new();
        tree.node(item, Role::ListItem, "i", []);
        tree.node(root, Role::Window, "w", [item]);
        tree.root(root);
        tree.focus(item);
        let update = tree.build();
        assert_eq!(update.focus, item);
    }

    #[test]
    fn missing_root_does_not_panic() {
        let update = TreeBuilder::new().build();
        assert!(update.nodes.iter().any(|(id, _)| *id == update.focus));
    }

    #[test]
    fn subtree_merges_nodes_and_returns_root() {
        let mut sub_nodes = Vec::new();
        let child = node_id("child");
        sub_nodes.push((child, Node::new(Role::ListItem)));
        let sub_root = node_id("sub-root");
        sub_nodes.push((sub_root, Node::new(Role::List)));
        let sub = SubTree::new(sub_root, sub_nodes);

        let mut tree = TreeBuilder::new();
        let returned_root = tree.subtree(sub);
        assert_eq!(returned_root, sub_root);
        tree.root(sub_root);
        let update = tree.build();
        assert_eq!(update.nodes.len(), 2);
    }

    #[test]
    #[should_panic(expected = "duplicate NodeId")]
    fn debug_validator_catches_duplicate_ids() {
        let id = node_id("dup");
        let mut tree = TreeBuilder::new();
        tree.node(id, Role::Window, "a", []);
        tree.node(id, Role::Window, "b", []);
        tree.root(id);
        tree.build();
    }

    #[test]
    #[should_panic(expected = "not present in this update")]
    fn debug_validator_catches_dangling_focus() {
        let root = node_id("root");
        let ghost = node_id("ghost");
        let mut tree = TreeBuilder::new();
        tree.node(root, Role::Window, "w", []);
        tree.root(root);
        tree.focus(ghost);
        tree.build();
    }

    #[test]
    fn action_survives_build() {
        let root = node_id("root");
        let mut tree = TreeBuilder::new();
        tree.node(root, Role::Button, "b", [])
            .add_action(Action::Click);
        tree.root(root);
        let update = tree.build();
        assert!(update.nodes[0].1.supports_action(Action::Click));
    }
}
