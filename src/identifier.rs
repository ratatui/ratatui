#![allow(clippy::module_name_repetitions)]

pub type TreeIdentifier<'a> = &'a [usize];
pub type TreeIdentifierVec = Vec<usize>;

pub fn get_without_leaf(identifier: &[usize]) -> (&[usize], Option<&usize>) {
    let length = identifier.len();
    let length_without_leaf = length.saturating_sub(1);

    let head = &identifier[0..length_without_leaf];
    let tail = identifier.get(length_without_leaf);

    (head, tail)
}

#[test]
fn get_without_leaf_empty() {
    let (head, tail) = get_without_leaf(&[]);
    assert_eq!(head.len(), 0);
    assert_eq!(tail, None);
}

#[test]
fn get_without_leaf_single() {
    let (head, tail) = get_without_leaf(&[2]);
    assert_eq!(head.len(), 0);
    assert_eq!(tail, Some(&2));
}

#[test]
fn get_without_leaf_multiple() {
    let (head, tail) = get_without_leaf(&[2, 4, 6]);
    assert_eq!(head, [2, 4]);
    assert_eq!(tail, Some(&6));
}
