use tree_iterators_rs::prelude::{
    BinaryTree, BorrowedBinaryTreeNode, BorrowedIntoIteratorOfBinaryTrees,
    BorrowedIntoIteratorOfTrees, BorrowedTreeNode, MutBorrowedBinaryTreeNode,
    MutBorrowedIntoIteratorOfBinaryTrees, MutBorrowedIntoIteratorOfTrees, MutBorrowedTreeNode,
    OwnedBinaryTreeNode, OwnedIntoIteratorOfBinaryTrees, OwnedIntoIteratorOfTrees, OwnedTreeNode,
    Tree,
};

#[test]
fn map_tree() {
    let mut original = Tree {
        value: "0_0".to_string(),
        children: vec![
            Tree {
                value: "1_1".to_string(),
                children: vec![
                    Tree {
                        value: "3_3".to_string(),
                        children: vec![Tree {
                            value: "5_5".to_string(),
                            children: Vec::with_capacity(0),
                        }],
                    },
                    Tree {
                        value: "4_4".to_string(),
                        children: Vec::with_capacity(0),
                    },
                ],
            },
            Tree {
                value: "2_2".to_string(),
                children: Vec::with_capacity(0),
            },
        ],
    };

    let expected = Tree {
        value: 0,
        children: vec![
            Tree {
                value: 1,
                children: vec![
                    Tree {
                        value: 3,
                        children: vec![Tree {
                            value: 5,
                            children: Vec::with_capacity(0),
                        }],
                    },
                    Tree {
                        value: 4,
                        children: Vec::with_capacity(0),
                    },
                ],
            },
            Tree {
                value: 2,
                children: Vec::with_capacity(0),
            },
        ],
    };

    assert_eq!(
        expected,
        original.map_ref(|value| value.split('_').next().unwrap().parse::<i32>().unwrap())
    );
    assert_eq!(
        expected,
        original.map_mut(|value| value.split('_').next().unwrap().parse::<i32>().unwrap())
    );
    assert_eq!(
        expected,
        original.map(|value| value.split('_').next().unwrap().parse::<i32>().unwrap())
    );
}

#[test]
fn map_binary_tree() {
    let mut original = BinaryTree {
        value: "0_0".to_string(),
        left: Some(Box::new(BinaryTree {
            value: "1_1".to_string(),
            left: Some(Box::new(BinaryTree {
                value: "3_3".to_string(),
                left: None,
                right: Some(Box::new(BinaryTree {
                    value: "5_5".to_string(),
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(BinaryTree {
                value: "4_4".to_string(),
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(BinaryTree {
            value: "2_2".to_string(),
            left: None,
            right: None,
        })),
    };

    let expected = BinaryTree {
        value: 0,
        left: Some(Box::new(BinaryTree {
            value: 1,
            left: Some(Box::new(BinaryTree {
                value: 3,
                left: None,
                right: Some(Box::new(BinaryTree {
                    value: 5,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(BinaryTree {
                value: 4,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(BinaryTree {
            value: 2,
            left: None,
            right: None,
        })),
    };

    assert_eq!(
        expected,
        original.map_ref(|value| value.split('_').next().unwrap().parse::<i32>().unwrap())
    );
    assert_eq!(
        expected,
        original.map_mut(|value| value.split('_').next().unwrap().parse::<i32>().unwrap())
    );
    assert_eq!(
        expected,
        original.map(|value| value.split('_').next().unwrap().parse::<i32>().unwrap())
    );
}

#[test]
fn map_tree_collection() {
    let original = Tree {
        value: "0_0".to_string(),
        children: vec![
            Tree {
                value: "1_1".to_string(),
                children: vec![
                    Tree {
                        value: "3_3".to_string(),
                        children: vec![Tree {
                            value: "5_5".to_string(),
                            children: Vec::with_capacity(0),
                        }],
                    },
                    Tree {
                        value: "4_4".to_string(),
                        children: Vec::with_capacity(0),
                    },
                ],
            },
            Tree {
                value: "2_2".to_string(),
                children: Vec::with_capacity(0),
            },
        ],
    };

    let mut original_collection = vec![original.clone(), original];

    let expected_result = Tree {
        value: 0,
        children: vec![
            Tree {
                value: 1,
                children: vec![
                    Tree {
                        value: 3,
                        children: vec![Tree {
                            value: 5,
                            children: Vec::with_capacity(0),
                        }],
                    },
                    Tree {
                        value: 4,
                        children: Vec::with_capacity(0),
                    },
                ],
            },
            Tree {
                value: 2,
                children: Vec::with_capacity(0),
            },
        ],
    };

    assert_eq!(
        vec![expected_result.clone(), expected_result.clone()],
        original_collection
            .map_each_ref(|value| value.split('_').next().unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![expected_result.clone(), expected_result.clone()],
        original_collection
            .map_each_mut(|value| value.split('_').next().unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![expected_result.clone(), expected_result],
        original_collection
            .map_each(|value| value.split('_').next().unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    );
}

#[test]
fn map_binary_tree_collection() {
    let original = BinaryTree {
        value: "0_0".to_string(),
        left: Some(Box::new(BinaryTree {
            value: "1_1".to_string(),
            left: Some(Box::new(BinaryTree {
                value: "3_3".to_string(),
                left: None,
                right: Some(Box::new(BinaryTree {
                    value: "5_5".to_string(),
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(BinaryTree {
                value: "4_4".to_string(),
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(BinaryTree {
            value: "2_2".to_string(),
            left: None,
            right: None,
        })),
    };

    let mut original_collection = vec![original.clone(), original];

    let expected_result = BinaryTree {
        value: 0,
        left: Some(Box::new(BinaryTree {
            value: 1,
            left: Some(Box::new(BinaryTree {
                value: 3,
                left: None,
                right: Some(Box::new(BinaryTree {
                    value: 5,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(BinaryTree {
                value: 4,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(BinaryTree {
            value: 2,
            left: None,
            right: None,
        })),
    };

    assert_eq!(
        vec![expected_result.clone(), expected_result.clone()],
        original_collection
            .map_each_ref(|value| value.split('_').next().unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![expected_result.clone(), expected_result.clone()],
        original_collection
            .map_each_mut(|value| value.split('_').next().unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vec![expected_result.clone(), expected_result],
        original_collection
            .map_each(|value| value.split('_').next().unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    );
}
