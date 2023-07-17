use super::{
    identifier::{TreeIdentifier, TreeIdentifierVec},
    TreeItem,
};

pub struct Flattened<'a> {
    pub identifier: Vec<usize>,
    pub item: &'a TreeItem<'a>,
}

impl<'a> Flattened<'a> {
    #[must_use]
    pub fn depth(&self) -> usize {
        self.identifier.len() - 1
    }
}

/// Get a flat list of all visible [`TreeItem`s](TreeItem)
#[must_use]
pub fn flatten<'a>(opened: &[TreeIdentifierVec], items: &'a [TreeItem<'a>]) -> Vec<Flattened<'a>> {
    internal(opened, items, &[])
}

#[must_use]
fn internal<'a>(
    opened: &[TreeIdentifierVec],
    items: &'a [TreeItem<'a>],
    current: TreeIdentifier,
) -> Vec<Flattened<'a>> {
    let mut result = Vec::new();

    for (index, item) in items.iter().enumerate() {
        let mut child_identifier = current.to_vec();
        child_identifier.push(index);

        result.push(Flattened {
            item,
            identifier: child_identifier.clone(),
        });

        if opened.contains(&child_identifier) {
            let mut child_result = internal(opened, &item.children, &child_identifier);
            result.append(&mut child_result);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prelude::*, widgets::tree::TreeItem};

    fn get_naive_string_from_text(text: &Text<'_>) -> String {
        text.lines
            .first()
            .unwrap()
            .spans
            .first()
            .unwrap()
            .content
            .to_string()
    }

    fn get_example_tree_items() -> Vec<TreeItem<'static>> {
        vec![
            TreeItem::new_leaf("a"),
            TreeItem::new(
                "b",
                vec![
                    TreeItem::new_leaf("c"),
                    TreeItem::new("d", vec![TreeItem::new_leaf("e"), TreeItem::new_leaf("f")]),
                    TreeItem::new_leaf("g"),
                ],
            ),
            TreeItem::new_leaf("h"),
        ]
    }

    #[test]
    fn get_opened_nothing_opened_is_top_level() {
        let items = get_example_tree_items();
        let result = flatten(&[], &items);
        let result_text = result
            .iter()
            .map(|o| get_naive_string_from_text(&o.item.text))
            .collect::<Vec<_>>();
        assert_eq!(result_text, ["a", "b", "h"]);
    }

    #[test]
    fn get_opened_wrong_opened_is_only_top_level() {
        let items = get_example_tree_items();
        let opened = [vec![0], vec![1, 1]];
        let result = flatten(&opened, &items);
        let result_text = result
            .iter()
            .map(|o| get_naive_string_from_text(&o.item.text))
            .collect::<Vec<_>>();
        assert_eq!(result_text, ["a", "b", "h"]);
    }

    #[test]
    fn get_opened_one_is_opened() {
        let items = get_example_tree_items();
        let opened = [vec![1]];
        let result = flatten(&opened, &items);
        let result_text = result
            .iter()
            .map(|o| get_naive_string_from_text(&o.item.text))
            .collect::<Vec<_>>();
        assert_eq!(result_text, ["a", "b", "c", "d", "g", "h"]);
    }

    #[test]
    fn get_opened_all_opened() {
        let items = get_example_tree_items();
        let opened = [vec![1], vec![1, 1]];
        let result = flatten(&opened, &items);
        let result_text = result
            .iter()
            .map(|o| get_naive_string_from_text(&o.item.text))
            .collect::<Vec<_>>();
        assert_eq!(result_text, ["a", "b", "c", "d", "e", "f", "g", "h"]);
    }
}
