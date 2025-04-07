use crate::{context_iterators::TreeContextIteratorBase, prelude::*};
use alloc::{boxed::Box, vec};

pub fn create_example_binary_tree() -> BinaryTree<usize> {
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

pub fn create_example_tree() -> Tree<usize> {
    use alloc::vec::Vec;

    Tree {
        value: 0,
        children: vec![
            Tree {
                value: 1,
                children: vec![
                    Tree {
                        value: 3,
                        children: Vec::with_capacity(0),
                    },
                    Tree {
                        value: 4,
                        children: Vec::with_capacity(0),
                    },
                ],
            },
            Tree {
                value: 2,
                children: vec![
                    Tree {
                        value: 5,
                        children: Vec::with_capacity(0),
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
                                        children: Vec::with_capacity(0),
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

#[test]
fn test_prune() {
    use crate::prelude::{BinaryTree, OwnedBinaryTreeNode};
    extern crate std;

    let tree = BinaryTree {
        value: 0,
        left: Some(Box::new(BinaryTree {
            value: 1,
            left: Some(Box::new(BinaryTree {
                value: 3,
                left: None,
                right: None,
            })),
            right: None,
        })),
        right: Some(Box::new(BinaryTree {
            value: 2,
            left: None,
            right: None,
        })),
    };

    /// Output:
    std::eprintln!(
        "{:#?}",
        tree.prune(|value| {
            std::eprintln!("{value:?}");
            *value == 0
        })
    );
}

#[test]
fn test_prune_context() {
    extern crate std;
    use crate::prelude::{BinaryTree, OwnedBinaryTreeNode};

    let tree = BinaryTree {
        value: 0,
        left: Some(Box::new(BinaryTree {
            value: 1,
            left: Some(Box::new(BinaryTree {
                value: 3,
                left: None,
                right: None,
            })),
            right: None,
        })),
        right: Some(Box::new(BinaryTree {
            value: 2,
            left: None,
            right: None,
        })),
    };

    std::println!(
        "{:#?}",
        tree.prune_context(|value| {
            std::println!("{:?}", value.ancestors());
            *value.ancestors().last().unwrap() == 1
        })
    );
}

#[test]
fn test_prune_depth() {
    extern crate std;

    let tree = create_example_binary_tree();

    let result = tree
        .into_pipeline()
        .prune_depth(0)
        .collect_tree()
        .unwrap();

    std::println!("{:#?}", result);
}

#[test]
fn test_map_context() {
    extern crate std;
    std::println!(
        "{:#?}",
        create_example_binary_tree()
            .into_pipeline()
            .map_tree_context(|context| {
                std::println!("{:#?}", context);
                *context.ancestors().last().unwrap() + 1
            })
            .collect_tree()
    );
}

#[test]
fn fold_tree_test() {
    extern crate std;
    std::println!(
        "{:#?}",
        create_example_tree().into_pipeline().fold_tree(|children, value| {
            std::println!("{}", value);
            std::println!("{:?}", children);
            value + children.into_iter().sum::<usize>()
        })
    )
}
