use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use accesskit::NodeId;

/// Derives a stable [`NodeId`] from an app-chosen key.
///
/// Ratatui redraws immediate-mode every frame, so nothing about a widget's
/// screen position is stable across frames. Accessibility clients rely on node
/// identity being stable across updates, for focus tracking and tree diffing,
/// so derive the id from something that identifies the underlying item.
pub fn node_id(key: impl Hash) -> NodeId {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    NodeId(hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_key_same_id() {
        assert_eq!(node_id("todo-item"), node_id("todo-item"));
        assert_eq!(node_id(("todo-item", 3)), node_id(("todo-item", 3)));
    }

    #[test]
    fn different_key_different_id() {
        assert_ne!(node_id("a"), node_id("b"));
        assert_ne!(node_id(("todo-item", 1)), node_id(("todo-item", 2)));
        assert_ne!(node_id("todo-item"), node_id(("todo-item", 0)));
    }

    #[test]
    fn stable_across_calls_regardless_of_hasher_state() {
        let ids: Vec<_> = (0..5).map(|_| node_id("stable")).collect();
        assert!(ids.windows(2).all(|w| w[0] == w[1]));
    }
}
