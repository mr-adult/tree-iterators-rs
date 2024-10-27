use crate::prelude::*;
use alloc::{boxed::Box, vec};

pub fn create_example_binary_tree() -> BinaryTreeNode<usize> {
    BinaryTreeNode {
        value: 0,
        left: Some(Box::new(BinaryTreeNode {
            value: 1,
            left: Some(Box::new(BinaryTreeNode {
                value: 3,
                left: None,
                right: None,
            })),
            right: Some(Box::new(BinaryTreeNode {
                value: 4,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(BinaryTreeNode {
            value: 2,
            left: Some(Box::new(BinaryTreeNode {
                value: 5,
                left: None,
                right: None,
            })),
            right: Some(Box::new(BinaryTreeNode {
                value: 6,
                left: Some(Box::new(BinaryTreeNode {
                    value: 7,
                    left: None,
                    right: Some(Box::new(BinaryTreeNode {
                        value: 8,
                        left: Some(Box::new(BinaryTreeNode {
                            value: 9,
                            left: None,
                            right: Some(Box::new(BinaryTreeNode {
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

pub fn create_example_tree() -> TreeNode<usize> {
    use alloc::vec::Vec;

    TreeNode {
        value: 0,
        children: vec![
            TreeNode {
                value: 1,
                children: vec![
                    TreeNode {
                        value: 3,
                        children: Vec::with_capacity(0),
                    },
                    TreeNode {
                        value: 4,
                        children: Vec::with_capacity(0),
                    },
                ],
            },
            TreeNode {
                value: 2,
                children: vec![
                    TreeNode {
                        value: 5,
                        children: Vec::with_capacity(0),
                    },
                    TreeNode {
                        value: 6,
                        children: vec![TreeNode {
                            value: 7,
                            children: vec![TreeNode {
                                value: 8,
                                children: vec![TreeNode {
                                    value: 9,
                                    children: vec![TreeNode {
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
