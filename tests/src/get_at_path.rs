use super::create_binary_tree_for_testing;
use super::create_tree_for_testing;

fn get_tree_path_value_pairs() -> Box<[(Box<[usize]>, Option<usize>)]> {
    Box::new([
        (Box::new([]), Some(0)),
        (Box::new([0]), Some(1)),
        (Box::new([1]), Some(2)),
        (Box::new([2]), None),
        (Box::new([0, 0]), Some(3)),
        (Box::new([0, 1]), Some(4)),
        (Box::new([0, 2]), None),
        (Box::new([1, 0]), Some(5)),
        (Box::new([1, 1]), Some(6)),
        (Box::new([1, 2]), None),
        (Box::new([1, 1, 0]), Some(7)),
        (Box::new([1, 1, 1]), None),
        (Box::new([1, 1, 1, 0]), None),
        (Box::new([1, 1, 0, 0]), Some(8)),
        (Box::new([1, 1, 0, 1]), None),
        (Box::new([1, 1, 0, 0, 0]), Some(9)),
        (Box::new([1, 1, 0, 0, 1]), None),
        (Box::new([1, 1, 0, 0, 0, 0]), Some(10)),
        (Box::new([1, 1, 0, 0, 0, 1]), None),
        (Box::new([1, 1, 0, 0, 0, 0, 0]), None),
    ])
}

#[test]
fn tree_at_path() {
    use tree_iterators_rs::prelude::OwnedTreeNode;

    let tree = create_tree_for_testing();
    for path_value_pair in get_tree_path_value_pairs() {
        assert_eq!(
            path_value_pair.1,
            tree.clone()
                .at_path(&path_value_pair.0)
                .map(|tree| tree.value)
        )
    }
}

#[test]
fn tree_at_path_ref() {
    use tree_iterators_rs::prelude::BorrowedTreeNode;

    let tree = create_tree_for_testing();
    for path_value_pair in get_tree_path_value_pairs() {
        assert_eq!(
            path_value_pair.1,
            tree.at_path_ref(&path_value_pair.0).map(|tree| tree.value)
        )
    }
}

#[test]
fn tree_at_path_mut() {
    use tree_iterators_rs::prelude::MutBorrowedTreeNode;

    let mut tree = create_tree_for_testing();
    for path_value_pair in get_tree_path_value_pairs() {
        assert_eq!(
            path_value_pair.1,
            tree.at_path_mut(&path_value_pair.0).map(|tree| tree.value)
        )
    }
}

fn get_binary_tree_path_value_pairs() -> Box<[(Box<[usize]>, Option<usize>)]> {
    Box::new([
        (Box::new([]), Some(0)),
        (Box::new([0]), Some(1)),
        (Box::new([1]), Some(2)),
        (Box::new([2]), None),
        (Box::new([0, 0]), Some(3)),
        (Box::new([0, 1]), Some(4)),
        (Box::new([0, 2]), None),
        (Box::new([1, 0]), Some(5)),
        (Box::new([1, 1]), Some(6)),
        (Box::new([1, 2]), None),
        (Box::new([1, 1, 0]), Some(7)),
        (Box::new([1, 1, 1]), None),
        (Box::new([1, 1, 1, 0]), None),
        (Box::new([1, 1, 0, 1]), Some(8)),
        (Box::new([1, 1, 0, 0]), None),
        (Box::new([1, 1, 0, 1, 0]), Some(9)),
        (Box::new([1, 1, 0, 1, 1]), None),
        (Box::new([1, 1, 0, 1, 0, 1]), Some(10)),
        (Box::new([1, 1, 0, 1, 0, 0]), None),
        (Box::new([1, 1, 0, 1, 0, 1, 0]), None),
    ])
}

#[test]
fn binary_tree_at_path() {
    use tree_iterators_rs::prelude::OwnedBinaryTreeNode;

    let binary_tree = create_binary_tree_for_testing();
    for path_value_pair in get_binary_tree_path_value_pairs() {
        assert_eq!(
            path_value_pair.1,
            binary_tree
                .clone()
                .at_path(&path_value_pair.0)
                .map(|tree| tree.value)
        )
    }
}

#[test]
fn binary_tree_at_path_ref() {
    use tree_iterators_rs::prelude::BorrowedBinaryTreeNode;

    let binary_tree = create_binary_tree_for_testing();
    for path_value_pair in get_binary_tree_path_value_pairs() {
        assert_eq!(
            path_value_pair.1,
            binary_tree
                .at_path_ref(&path_value_pair.0)
                .map(|tree| tree.value)
        )
    }
}

#[test]
fn binary_tree_at_path_mut() {
    use tree_iterators_rs::prelude::MutBorrowedBinaryTreeNode;

    let mut binary_tree = create_binary_tree_for_testing();
    for path_value_pair in get_binary_tree_path_value_pairs() {
        assert_eq!(
            path_value_pair.1,
            binary_tree
                .at_path_mut(&path_value_pair.0)
                .map(|tree| tree.value)
        )
    }
}
