use alloc::vec;
use alloc::vec::Vec;

use crate::prelude::{
    tests::{create_binary_tree_for_testing, create_tree_for_testing},
    BorrowedBinaryTreeNode, BorrowedIntoIteratorOfBinaryTrees, BorrowedIntoIteratorOfTrees,
    BorrowedTreeNode, MutBorrowedBinaryTreeNode, MutBorrowedIntoIteratorOfBinaryTrees,
    MutBorrowedIntoIteratorOfTrees, MutBorrowedTreeNode, OwnedBinaryTreeNode,
    OwnedIntoIteratorOfBinaryTrees, OwnedIntoIteratorOfTrees, OwnedTreeNode,
};

#[test]
fn fold_tree() {
    let mut tree = create_tree_for_testing();
    assert_eq!(
        55,
        tree.fold_ref(|child_accs, value| child_accs.into_iter().sum::<usize>() + *value)
    );
    assert_eq!(
        55,
        tree.fold_mut(|child_accs, value| child_accs.into_iter().sum::<usize>() + *value)
    );
    assert_eq!(
        55,
        tree.fold(|child_accs, value| child_accs.into_iter().sum::<usize>() + value)
    );
}

#[test]
fn fold_binary_tree() {
    let mut binary_tree = create_binary_tree_for_testing();
    assert_eq!(
        55,
        binary_tree.fold_ref(|child_accs, value| child_accs
            .into_iter()
            .flat_map(|opt| opt)
            .sum::<usize>()
            + *value)
    );
    assert_eq!(
        55,
        binary_tree.fold_mut(|child_accs, value| child_accs
            .into_iter()
            .flat_map(|opt| opt)
            .sum::<usize>()
            + *value)
    );
    assert_eq!(
        55,
        binary_tree.fold(|child_accs, value| child_accs
            .into_iter()
            .flat_map(|opt| opt)
            .sum::<usize>()
            + value)
    );
}

#[test]
fn fold_trees() {
    let mut trees = vec![
        create_tree_for_testing(),
        create_tree_for_testing(),
        create_tree_for_testing(),
    ];

    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_each_ref(|children_accs, value| children_accs.into_iter().sum::<usize>() + *value)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_each_mut(|children_accs, value| children_accs.into_iter().sum::<usize>() + *value)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_each(|children_accs, value| children_accs.into_iter().sum::<usize>() + value)
            .collect::<Vec<_>>()
    );
}

#[test]
fn fold_binary_trees() {
    let mut trees = vec![
        create_binary_tree_for_testing(),
        create_binary_tree_for_testing(),
        create_binary_tree_for_testing(),
    ];

    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_each_ref(|child_accs, value| child_accs
                .into_iter()
                .flat_map(|opt| opt)
                .sum::<usize>()
                + *value)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_each_mut(|child_accs, value| child_accs
                .into_iter()
                .flat_map(|opt| opt)
                .sum::<usize>()
                + *value)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_each(|child_accs, value| child_accs
                .into_iter()
                .flat_map(|opt| opt)
                .sum::<usize>()
                + value)
            .collect::<Vec<_>>()
    );
}
