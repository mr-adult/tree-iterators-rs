use std::collections::HashMap;

use tree_iterators_rs::prelude::{
    BorrowedBinaryTreeNode, BorrowedIntoIteratorOfBinaryTrees, BorrowedIntoIteratorOfTrees,
    BorrowedTreeNode, MutBorrowedBinaryTreeNode, MutBorrowedIntoIteratorOfBinaryTrees,
    MutBorrowedIntoIteratorOfTrees, MutBorrowedTreeNode, OwnedBinaryTreeNode,
    OwnedIntoIteratorOfBinaryTrees, OwnedIntoIteratorOfTrees, OwnedTreeNode,
};

use crate::{create_binary_tree_for_testing, create_tree_for_testing};

#[test]
fn fold_tree() {
    let mut tree = create_tree_for_testing();
    assert_eq!(
        55,
        tree.fold_ref(
            |child_accs: Vec<usize>, value| child_accs.into_iter().sum::<usize>() + *value
        )
    );
    assert_eq!(
        55,
        tree.fold_mut(
            |child_accs: Vec<usize>, value| child_accs.into_iter().sum::<usize>() + *value
        )
    );
    assert_eq!(
        55,
        tree.fold(|child_accs: Vec<usize>, value| child_accs.into_iter().sum::<usize>() + value)
    );
}

#[test]
fn fold_binary_tree() {
    let mut binary_tree = create_binary_tree_for_testing();
    assert_eq!(
        55,
        binary_tree.fold_ref(|child_accs: [Option<usize>; 2], value| child_accs
            .into_iter()
            .flat_map(|opt| opt)
            .sum::<usize>()
            + *value)
    );
    assert_eq!(
        55,
        binary_tree.fold_mut(|child_accs: [Option<usize>; 2], value| child_accs
            .into_iter()
            .flat_map(|opt| opt)
            .sum::<usize>()
            + *value)
    );
    assert_eq!(
        55,
        binary_tree.fold(|child_accs: [Option<usize>; 2], value| child_accs
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
            .fold_each_ref(|children_accs: Vec<usize>, value| children_accs
                .into_iter()
                .sum::<usize>()
                + *value)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_each_mut(|children_accs: Vec<usize>, value| children_accs
                .into_iter()
                .sum::<usize>()
                + *value)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_each(
                |children_accs: Vec<usize>, value| children_accs.into_iter().sum::<usize>() + value
            )
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
            .fold_each_ref(|child_accs: [Option<usize>; 2], value| child_accs
                .into_iter()
                .flat_map(|opt| opt)
                .sum::<usize>()
                + *value)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_each_mut(|child_accs: [Option<usize>; 2], value| child_accs
                .into_iter()
                .flat_map(|opt| opt)
                .sum::<usize>()
                + *value)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_each(|child_accs: [Option<usize>; 2], value| child_accs
                .into_iter()
                .flat_map(|opt| opt)
                .sum::<usize>()
                + value)
            .collect::<Vec<_>>()
    );
}

#[test]
fn fold_path_tree() {
    let mut tree = create_tree_for_testing();
    assert_eq!(
        55,
        tree.fold_path_ref(|child_accs: Vec<usize>, path, value| {
            assert_correct_path(*value, path);
            child_accs.into_iter().sum::<usize>() + value
        })
    );
    assert_eq!(
        55,
        tree.fold_path_mut(|child_accs: Vec<usize>, path, value| {
            assert_correct_path(*value, path);
            child_accs.into_iter().sum::<usize>() + *value
        })
    );
    assert_eq!(
        55,
        tree.fold_path(|child_accs: Vec<usize>, path, value| {
            assert_correct_path(value, path);
            child_accs.into_iter().sum::<usize>() + value
        })
    );
}

fn assert_correct_path(value: usize, path: &[usize]) {
    match value {
        0 => {
            assert_eq!(0, path.len());
        }
        1 => {
            assert_eq!(1, path.len());
            assert_eq!(0, path[0]);
        }
        2 => {
            assert_eq!(1, path.len());
            assert_eq!(1, path[0]);
        }
        3 => {
            assert_eq!(2, path.len());
            assert_eq!(0, path[0]);
            assert_eq!(0, path[1]);
        }
        4 => {
            assert_eq!(2, path.len());
            assert_eq!(0, path[0]);
            assert_eq!(1, path[1]);
        }
        5 => {
            assert_eq!(2, path.len());
            assert_eq!(1, path[0]);
            assert_eq!(0, path[1]);
        }
        6 => {
            assert_eq!(2, path.len());
            assert_eq!(1, path[0]);
            assert_eq!(1, path[1]);
        }
        7 => {
            assert_eq!(3, path.len());
            assert_eq!(1, path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(0, path[2]);
        }
        8 => {
            assert_eq!(4, path.len());
            assert_eq!(1, path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(0, path[2]);
            assert_eq!(0, path[3]);
        }
        9 => {
            assert_eq!(5, path.len());
            assert_eq!(1, path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(0, path[2]);
            assert_eq!(0, path[3]);
            assert_eq!(0, path[4]);
        }
        10 => {
            assert_eq!(6, path.len());
            assert_eq!(1, path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(0, path[2]);
            assert_eq!(0, path[3]);
            assert_eq!(0, path[4]);
            assert_eq!(0, path[5]);
        }
        _ => unreachable!(),
    }
}

#[test]
fn fold_path_binary_tree() {
    let mut binary_tree = create_binary_tree_for_testing();
    assert_eq!(
        55,
        binary_tree.fold_path_ref(|child_accs: [Option<usize>; 2], path, value| {
            assert_correct_binary_path(*value, path);
            child_accs.into_iter().flat_map(|opt| opt).sum::<usize>() + value
        })
    );
    assert_eq!(
        55,
        binary_tree.fold_path_mut(|child_accs: [Option<usize>; 2], path, value| {
            assert_correct_binary_path(*value, path);
            child_accs.into_iter().flat_map(|opt| opt).sum::<usize>() + *value
        })
    );
    assert_eq!(
        55,
        binary_tree.fold_path(|child_accs: [Option<usize>; 2], path, value| {
            assert_correct_binary_path(value, path);
            child_accs.into_iter().flat_map(|opt| opt).sum::<usize>() + value
        })
    );
}

fn assert_correct_binary_path(value: usize, path: &[usize]) {
    match value {
        0 => {
            assert_eq!(0, path.len());
        }
        1 => {
            assert_eq!(1, path.len());
            assert_eq!(0, path[0]);
        }
        2 => {
            assert_eq!(1, path.len());
            assert_eq!(1, path[0]);
        }
        3 => {
            assert_eq!(2, path.len());
            assert_eq!(0, path[0]);
            assert_eq!(0, path[1]);
        }
        4 => {
            assert_eq!(2, path.len());
            assert_eq!(0, path[0]);
            assert_eq!(1, path[1]);
        }
        5 => {
            assert_eq!(2, path.len());
            assert_eq!(1, path[0]);
            assert_eq!(0, path[1]);
        }
        6 => {
            assert_eq!(2, path.len());
            assert_eq!(1, path[0]);
            assert_eq!(1, path[1]);
        }
        7 => {
            assert_eq!(3, path.len());
            assert_eq!(1, path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(0, path[2]);
        }
        8 => {
            assert_eq!(4, path.len());
            assert_eq!(1, path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(0, path[2]);
            assert_eq!(1, path[3]);
        }
        9 => {
            assert_eq!(5, path.len());
            assert_eq!(1, path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(0, path[2]);
            assert_eq!(1, path[3]);
            assert_eq!(0, path[4]);
        }
        10 => {
            assert_eq!(6, path.len());
            assert_eq!(1, path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(0, path[2]);
            assert_eq!(1, path[3]);
            assert_eq!(0, path[4]);
            assert_eq!(1, path[5]);
        }
        _ => unreachable!(),
    }
}

#[test]
fn fold_path_trees() {
    let mut trees = vec![
        create_tree_for_testing(),
        create_tree_for_testing(),
        create_tree_for_testing(),
    ];

    let mut counters = HashMap::new();
    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_path_each_ref(|children_accs: Vec<usize>, path, value| {
                assert_correct_trees_path(*value, path, &mut counters);
                children_accs.into_iter().sum::<usize>() + *value
            })
            .collect::<Vec<_>>()
    );
    let mut counters = HashMap::new();
    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_path_each_mut(|children_accs: Vec<usize>, path, value| {
                assert_correct_trees_path(*value, path, &mut counters);
                children_accs.into_iter().sum::<usize>() + *value
            })
            .collect::<Vec<_>>()
    );
    let mut counters = HashMap::new();
    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_path_each(|children_accs: Vec<usize>, path, value| {
                assert_correct_trees_path(value, path, &mut counters);
                children_accs.into_iter().sum::<usize>() + value
            })
            .collect::<Vec<_>>()
    );
}

fn assert_correct_trees_path(value: usize, path: &[usize], counters: &mut HashMap<usize, usize>) {
    if let Some(counter) = counters.get_mut(&value) {
        *counter += 1;
    } else {
        counters.insert(value, 0);
    }

    match value {
        0 => {
            assert_eq!(1, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
        }
        1 => {
            assert_eq!(2, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(0, path[1]);
        }
        2 => {
            assert_eq!(2, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(1, path[1]);
        }
        3 => {
            assert_eq!(3, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(0, path[1]);
            assert_eq!(0, path[2]);
        }
        4 => {
            assert_eq!(3, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(0, path[1]);
            assert_eq!(1, path[2]);
        }
        5 => {
            assert_eq!(3, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(0, path[2]);
        }
        6 => {
            assert_eq!(3, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(1, path[2]);
        }
        7 => {
            assert_eq!(4, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(1, path[2]);
            assert_eq!(0, path[3]);
        }
        8 => {
            assert_eq!(5, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(1, path[2]);
            assert_eq!(0, path[3]);
            assert_eq!(0, path[4]);
        }
        9 => {
            assert_eq!(6, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(1, path[2]);
            assert_eq!(0, path[3]);
            assert_eq!(0, path[4]);
            assert_eq!(0, path[5]);
        }
        10 => {
            assert_eq!(7, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(1, path[2]);
            assert_eq!(0, path[3]);
            assert_eq!(0, path[4]);
            assert_eq!(0, path[5]);
            assert_eq!(0, path[6]);
        }
        _ => unreachable!(),
    }
}

#[test]
fn fold_path_binary_trees() {
    let mut trees = vec![
        create_binary_tree_for_testing(),
        create_binary_tree_for_testing(),
        create_binary_tree_for_testing(),
    ];

    let mut counters = HashMap::new();
    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_path_each_ref(|child_accs: [Option<usize>; 2], path, value| {
                assert_correct_binary_trees_path(*value, path, &mut counters);
                child_accs.into_iter().flat_map(|opt| opt).sum::<usize>() + *value
            })
            .collect::<Vec<_>>()
    );
    let mut counters = HashMap::new();
    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_path_each_mut(|child_accs: [Option<usize>; 2], path, value| {
                assert_correct_binary_trees_path(*value, path, &mut counters);
                child_accs.into_iter().flat_map(|opt| opt).sum::<usize>() + *value
            })
            .collect::<Vec<_>>()
    );
    let mut counters = HashMap::new();
    assert_eq!(
        vec![55, 55, 55],
        trees
            .fold_path_each(|child_accs: [Option<usize>; 2], path, value| {
                assert_correct_binary_trees_path(value, path, &mut counters);
                child_accs.into_iter().flat_map(|opt| opt).sum::<usize>() + value
            })
            .collect::<Vec<_>>()
    );
}

fn assert_correct_binary_trees_path(
    value: usize,
    path: &[usize],
    counters: &mut HashMap<usize, usize>,
) {
    if let Some(counter) = counters.get_mut(&value) {
        *counter += 1;
    } else {
        counters.insert(value, 0);
    }

    match value {
        0 => {
            assert_eq!(1, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
        }
        1 => {
            assert_eq!(2, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(0, path[1]);
        }
        2 => {
            assert_eq!(2, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(1, path[1]);
        }
        3 => {
            assert_eq!(3, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(0, path[1]);
            assert_eq!(0, path[2]);
        }
        4 => {
            assert_eq!(3, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(0, path[1]);
            assert_eq!(1, path[2]);
        }
        5 => {
            assert_eq!(3, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(0, path[2]);
        }
        6 => {
            assert_eq!(3, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(1, path[2]);
        }
        7 => {
            assert_eq!(4, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(1, path[2]);
            assert_eq!(0, path[3]);
        }
        8 => {
            assert_eq!(5, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(1, path[2]);
            assert_eq!(0, path[3]);
            assert_eq!(1, path[4]);
        }
        9 => {
            assert_eq!(6, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(1, path[2]);
            assert_eq!(0, path[3]);
            assert_eq!(1, path[4]);
            assert_eq!(0, path[5]);
        }
        10 => {
            assert_eq!(7, path.len());
            assert_eq!(counters.get(&value).cloned().unwrap(), path[0]);
            assert_eq!(1, path[1]);
            assert_eq!(1, path[2]);
            assert_eq!(0, path[3]);
            assert_eq!(1, path[4]);
            assert_eq!(0, path[5]);
            assert_eq!(1, path[6]);
        }
        _ => unreachable!(),
    }
}
