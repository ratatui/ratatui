//! Adapters that turn plain data into a [`SubTree`], with no dependency on
//! any particular widget type.
//!
//! Widget fields in `ratatui-widgets` are private, so these can't take
//! `&List` or `&Table`. They take the same data you'd already have on hand
//! to build one: item strings, a selected index. That also means they work
//! for a hand-rolled widget, or any TUI framework, not just Ratatui's built-ins.
//!
//! All of them are pure: no `Rect`, no `Buffer`, no rendering, so they're
//! testable without a terminal. `key` identifies *this instance* of the
//! widget; child ids are derived from `key` plus each item's index.

use std::hash::Hash;

use accesskit::{Node, Role};

use crate::id::node_id;
use crate::subtree::SubTree;

/// Builds a subtree for any flat, selectable collection: `container_role`
/// wraps one `item_role` node per entry.
///
/// This is the general shape behind [`list_nodes`] and [`tabs_nodes`] --
/// both are just this with the AT-SPI roles for their case already picked.
/// Reach for it directly for a collection that isn't a list or tabs
/// (a radio group, a menu, a listbox, ...): `items_nodes(options, selected,
/// Role::RadioGroup, Role::RadioButton, key)`.
pub fn items_nodes<I, S>(
    items: I,
    selected: Option<usize>,
    container_role: Role,
    item_role: Role,
    key: impl Hash,
) -> SubTree
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut nodes = Vec::new();
    let mut child_ids = Vec::new();
    let mut selected_id = None;

    for (i, item) in items.into_iter().enumerate() {
        let id = node_id((&key, i));
        let mut node = Node::new(item_role);
        node.set_label(item.into());
        let is_selected = selected == Some(i);
        node.set_selected(is_selected);
        if is_selected {
            selected_id = Some(id);
        }
        child_ids.push(id);
        nodes.push((id, node));
    }

    let root = node_id(&key);
    let mut root_node = Node::new(container_role);
    root_node.set_children(child_ids);
    nodes.push((root, root_node));

    let mut sub = SubTree::new(root, nodes);
    sub.selected = selected_id;
    sub
}

/// Builds an unlabeled `List` subtree, one `ListItem` per entry.
/// Wrap it in [`group_nodes`] to give the list itself a name.
pub fn list_nodes<I, S>(items: I, selected: Option<usize>, key: impl Hash) -> SubTree
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    items_nodes(items, selected, Role::List, Role::ListItem, key)
}

/// Builds `Table` subtree: an optional header row, then one `Row` of `Cell`s per data row.
///
/// A selected cell needs both `selected_row` and `selected_col`; a selected
/// row with `selected_col: None` marks the whole row selected instead.
pub fn table_nodes<H, R, C, S>(
    header: Option<H>,
    rows: R,
    selected_row: Option<usize>,
    selected_col: Option<usize>,
    key: impl Hash,
) -> SubTree
where
    H: IntoIterator<Item = S>,
    R: IntoIterator<Item = C>,
    C: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut nodes = Vec::new();
    let mut row_ids = Vec::new();
    let mut selected_id = None;

    if let Some(header) = header {
        let header_id = node_id((&key, "header"));
        let mut cell_ids = Vec::new();
        for (c, cell) in header.into_iter().enumerate() {
            let id = node_id((&key, "header-cell", c));
            let mut node = Node::new(Role::ColumnHeader);
            node.set_label(cell.into());
            cell_ids.push(id);
            nodes.push((id, node));
        }
        let mut header_node = Node::new(Role::Row);
        header_node.set_children(cell_ids);
        nodes.push((header_id, header_node));
        row_ids.push(header_id);
    }

    for (r, row) in rows.into_iter().enumerate() {
        let row_id = node_id((&key, "row", r));
        let mut cell_ids = Vec::new();
        let mut row_selected = false;
        for (c, cell) in row.into_iter().enumerate() {
            let id = node_id((&key, "cell", r, c));
            let mut node = Node::new(Role::Cell);
            let is_selected = selected_row == Some(r) && selected_col == Some(c);
            node.set_label(cell.into());
            node.set_selected(is_selected);
            if is_selected {
                selected_id = Some(id);
                row_selected = true;
            }
            cell_ids.push(id);
            nodes.push((id, node));
        }
        let mut row_node = Node::new(Role::Row);
        row_node.set_children(cell_ids);
        nodes.push((row_id, row_node));
        // No column picked out => the whole row is the selection.
        if !row_selected && selected_row == Some(r) && selected_col.is_none() {
            selected_id = Some(row_id);
        }
        row_ids.push(row_id);
    }

    let root = node_id(&key);
    let mut root_node = Node::new(Role::Table);
    root_node.set_children(row_ids);
    nodes.push((root, root_node));

    let mut sub = SubTree::new(root, nodes);
    sub.selected = selected_id;
    sub
}

/// Builds a `TabList` subtree, one `Tab` per title.
pub fn tabs_nodes<I, S>(titles: I, selected: Option<usize>, key: impl Hash) -> SubTree
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    items_nodes(titles, selected, Role::TabList, Role::Tab, key)
}

/// Builds a single `ProgressIndicator` node. `ratio` is clamped to `0.0..=1.0` and reported as
/// `0..=100`.
pub fn gauge_nodes(label: Option<impl Into<String>>, ratio: f64, key: impl Hash) -> SubTree {
    let id = node_id(&key);
    let mut node = Node::new(Role::ProgressIndicator);
    if let Some(label) = label {
        node.set_label(label.into());
    }
    let percent = ratio.clamp(0.0, 1.0) * 100.0;
    node.set_numeric_value(percent);
    node.set_min_numeric_value(0.0);
    node.set_max_numeric_value(100.0);
    SubTree::new(id, vec![(id, node)])
}

/// Builds a single `Paragraph` node from source text. Pass the source string,
/// not a reflowed/wrapped render of it a screen reader does its own line breaking.
pub fn text_nodes(text: impl Into<String>, key: impl Hash) -> SubTree {
    let id = node_id(&key);
    let mut node = Node::new(Role::Paragraph);
    node.set_label(text.into());
    SubTree::new(id, vec![(id, node)])
}

/// Wraps child subtrees in a single labeled `Group` node, the way to
/// attach a name to a [`list_nodes`]/[`table_nodes`]/[`tabs_nodes`] result,
/// or to bundle several widgets under one heading.
pub fn group_nodes<I>(label: impl Into<String>, children: I, key: impl Hash) -> SubTree
where
    I: IntoIterator<Item = SubTree>,
{
    let root = node_id(&key);
    let mut nodes = Vec::new();
    let mut child_ids = Vec::new();
    let mut selected_id = None;

    for child in children {
        child_ids.push(child.root);
        if child.selected.is_some() {
            selected_id = child.selected;
        }
        nodes.extend(child.into_nodes());
    }

    let mut root_node = Node::new(Role::Group);
    root_node.set_label(label.into());
    root_node.set_children(child_ids);
    nodes.push((root, root_node));

    let mut sub = SubTree::new(root, nodes);
    sub.selected = selected_id;
    sub
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_nodes_labels_and_selection() {
        let sub = list_nodes(["a", "b", "c"], Some(1), "k");
        let nodes = &sub.into_nodes();
        assert_eq!(nodes.len(), 4);
        let root = nodes.iter().find(|(id, _)| *id == sub_root(nodes)).unwrap();
        assert_eq!(root.1.role(), Role::List);
    }

    fn sub_root(nodes: &[(accesskit::NodeId, Node)]) -> accesskit::NodeId {
        nodes
            .iter()
            .find(|(_, n)| n.role() == Role::List)
            .unwrap()
            .0
    }

    #[test]
    fn list_nodes_selected_id_matches_selected_index() {
        let sub = list_nodes(["a", "b"], Some(1), "k");
        let expected = node_id((&"k", 1usize));
        assert_eq!(sub.selected, Some(expected));
    }

    #[test]
    fn list_nodes_no_selection() {
        let sub = list_nodes(["a", "b"], None, "k");
        assert_eq!(sub.selected, None);
    }

    #[test]
    fn ids_stable_across_two_calls() {
        let a = list_nodes(["a", "b"], Some(0), "same-key");
        let b = list_nodes(["a", "b"], Some(0), "same-key");
        assert_eq!(a.root, b.root);
        assert_eq!(a.selected, b.selected);
    }

    #[test]
    fn different_key_different_ids() {
        let a = list_nodes(["a"], None, "k1");
        let b = list_nodes(["a"], None, "k2");
        assert_ne!(a.root, b.root);
    }

    #[test]
    fn table_nodes_header_becomes_column_header() {
        let sub = table_nodes(
            Some(["Name", "Status"]),
            [["a", "todo"], ["b", "done"]],
            None,
            None,
            "t",
        );
        let nodes = sub.into_nodes();
        // 2 header cells + 1 header row + (2 rows * 2 cells) + 2 row nodes + 1 root.
        assert_eq!(nodes.len(), 2 + 1 + 4 + 2 + 1);
        assert!(
            nodes
                .iter()
                .any(|(_, n)| n.role() == Role::ColumnHeader && n.label() == Some("Name"))
        );
    }

    #[test]
    fn table_nodes_cell_selection() {
        let sub = table_nodes(
            None::<[&str; 0]>,
            [["a", "b"], ["c", "d"]],
            Some(1),
            Some(0),
            "t",
        );
        assert_eq!(sub.selected, Some(node_id((&"t", "cell", 1usize, 0usize))));
    }

    #[test]
    fn table_nodes_whole_row_selection() {
        let sub = table_nodes(None::<[&str; 0]>, [["a", "b"]], Some(0), None, "t");
        assert_eq!(sub.selected, Some(node_id((&"t", "row", 0usize))));
    }

    #[test]
    fn tabs_nodes_selection() {
        let sub = tabs_nodes(["one", "two"], Some(0), "tabs");
        assert_eq!(sub.selected, Some(node_id((&"tabs", 0usize))));
    }

    #[test]
    fn gauge_nodes_clamps_ratio() {
        let sub = gauge_nodes(Some("progress"), 1.5, "g");
        let (_, node) = &sub.into_nodes()[0];
        assert_eq!(node.numeric_value(), Some(100.0));
    }

    #[test]
    fn gauge_nodes_negative_ratio_clamped_to_zero() {
        let sub = gauge_nodes(Some("progress"), -0.5, "g");
        let (_, node) = &sub.into_nodes()[0];
        assert_eq!(node.numeric_value(), Some(0.0));
    }

    #[test]
    fn text_nodes_single_node() {
        let sub = text_nodes("hello", "t");
        assert_eq!(sub.into_nodes().len(), 1);
    }

    #[test]
    fn group_nodes_wraps_children_and_forwards_selection() {
        let list = list_nodes(["a", "b"], Some(0), "list-key");
        let expected_selected = list.selected;
        let group = group_nodes("My list", [list], "group-key");
        assert_eq!(group.selected, expected_selected);
        let nodes = group.into_nodes();
        assert!(nodes.iter().any(|(_, n)| n.role() == Role::Group));
    }
}
