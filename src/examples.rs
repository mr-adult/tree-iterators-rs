use crate::prelude::{Tree, BinaryTree};
use alloc::boxed::Box;

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
    use alloc::vec;
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
