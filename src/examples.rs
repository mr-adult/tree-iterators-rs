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
    TreeNode {
        value: 0,
        children: Some(vec![
            TreeNode {
                value: 1,
                children: Some(vec![
                    TreeNode {
                        value: 3,
                        children: None,
                    },
                    TreeNode {
                        value: 4,
                        children: None,
                    },
                ]),
            },
            TreeNode {
                value: 2,
                children: Some(vec![
                    TreeNode {
                        value: 5,
                        children: None,
                    },
                    TreeNode {
                        value: 6,
                        children: Some(vec![TreeNode {
                            value: 7,
                            children: Some(vec![TreeNode {
                                value: 8,
                                children: Some(vec![TreeNode {
                                    value: 9,
                                    children: Some(vec![TreeNode {
                                        value: 10,
                                        children: None,
                                    }]),
                                }]),
                            }]),
                        }]),
                    },
                ]),
            },
        ]),
    }
}
