#![cfg(test)]

use tree_iterators_rs::prelude::*;

mod ancestors_leaves;
mod bfs;
mod dfs_inorder;
mod dfs_postorder;
mod dfs_preorder;
mod get_at_path;
mod leaves;

#[cfg(test)]
extern crate std;
#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
pub(crate) fn get_value_to_path_map() -> HashMap<usize, Vec<usize>> {
    let mut result = HashMap::new();
    result.insert(0, vec![]);
    result.insert(1, vec![0]);
    result.insert(2, vec![1]);
    result.insert(3, vec![0, 0]);
    result.insert(4, vec![0, 1]);
    result.insert(5, vec![1, 0]);
    result.insert(6, vec![1, 1]);
    result.insert(7, vec![1, 1, 0]);
    result.insert(8, vec![1, 1, 0, 0]);
    result.insert(9, vec![1, 1, 0, 0, 0]);
    result.insert(10, vec![1, 1, 0, 0, 0, 0]);
    result
}

#[cfg(test)]
pub(crate) fn get_value_to_path_map_binary() -> HashMap<usize, Vec<usize>> {
    let mut result = HashMap::new();
    result.insert(0, vec![]);
    result.insert(1, vec![0]);
    result.insert(2, vec![1]);
    result.insert(3, vec![0, 0]);
    result.insert(4, vec![0, 1]);
    result.insert(5, vec![1, 0]);
    result.insert(6, vec![1, 1]);
    result.insert(7, vec![1, 1, 0]);
    result.insert(8, vec![1, 1, 0, 1]);
    result.insert(9, vec![1, 1, 0, 1, 0]);
    result.insert(10, vec![1, 1, 0, 1, 0, 1]);
    result
}

macro_rules! assert_len {
    ($expected: expr, $iter: expr) => {
        assert_len!($expected, $iter, "");
    };
    ($expected: expr, $iter: expr, $message: expr) => {
        let mut count = 0;
        $iter.for_each(|_| count += 1);
        assert_eq!($expected, count, "{}", $message);
    };
}
use assert_len;

fn get_expected_metadata_for_value(val: usize) -> &'static [usize] {
    match val {
        0 => &[0],
        1 => &[0, 1],
        2 => &[0, 2],
        3 => &[0, 1, 3],
        4 => &[0, 1, 4],
        5 => &[0, 2, 5],
        6 => &[0, 2, 6],
        7 => &[0, 2, 6, 7],
        8 => &[0, 2, 6, 7, 8],
        9 => &[0, 2, 6, 7, 8, 9],
        10 => &[0, 2, 6, 7, 8, 9, 10],
        _ => panic!("unexpected value"),
    }
}

fn create_trees_for_testing() -> Vec<Tree<usize>> {
    vec![create_tree_for_testing()]
}

pub(crate) fn create_tree_for_testing() -> Tree<usize> {
    Tree {
        value: 0,
        children: vec![
            Tree {
                value: 1,
                children: vec![
                    Tree {
                        value: 3,
                        children: Vec::new(),
                    },
                    Tree {
                        value: 4,
                        children: Vec::new(),
                    },
                ],
            },
            Tree {
                value: 2,
                children: vec![
                    Tree {
                        value: 5,
                        children: Vec::new(),
                    },
                    Tree {
                        value: 6,
                        children: vec![Tree {
                            value: 7,
                            children: vec![Tree {
                                value: 8,
                                children: vec![Tree {
                                    value: 9,
                                    children: vec![Tree {
                                        value: 10,
                                        children: Vec::new(),
                                    }],
                                }],
                            }],
                        }],
                    },
                ],
            },
        ],
    }
}

pub fn create_binary_tree_for_testing() -> BinaryTree<usize> {
    BinaryTree {
        value: 0,
        left: Some(Box::new(BinaryTree {
            value: 1,
            left: Some(Box::new(BinaryTree {
                value: 3,
                left: None,
                right: None,
            })),
            right: Some(Box::new(BinaryTree {
                value: 4,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(BinaryTree {
            value: 2,
            left: Some(Box::new(BinaryTree {
                value: 5,
                left: None,
                right: None,
            })),
            right: Some(Box::new(BinaryTree {
                value: 6,
                left: Some(Box::new(BinaryTree {
                    value: 7,
                    left: None,
                    right: Some(Box::new(BinaryTree {
                        value: 8,
                        left: Some(Box::new(BinaryTree {
                            value: 9,
                            left: None,
                            right: Some(Box::new(BinaryTree {
                                value: 10,
                                left: None,
                                right: None,
                            })),
                        })),
                        right: None,
                    })),
                })),
                right: None,
            })),
        })),
    }
}
