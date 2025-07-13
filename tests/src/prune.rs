    use alloc::vec::Vec;
    use alloc::{boxed::Box, vec};

    use crate::prelude::{
        tests::{create_binary_tree_for_testing, create_tree_for_testing},
        BinaryTree, OwnedTreeNode, Tree,
    };
    use crate::prelude::{
        BinaryTreeCollectionIterator, BorrowedBinaryTreeNode, BorrowedIntoIteratorOfBinaryTrees,
        BorrowedIntoIteratorOfTrees, BorrowedTreeNode, MutBorrowedBinaryTreeNode,
        MutBorrowedIntoIteratorOfBinaryTrees, MutBorrowedIntoIteratorOfTrees, MutBorrowedTreeNode,
        OwnedBinaryTreeNode, OwnedIntoIteratorOfBinaryTrees, OwnedIntoIteratorOfTrees,
        TreeCollectionIterator, TreeCollectionIteratorBase,
    };

    #[test]
    fn prune_tree() {
        let tree = create_tree_for_testing();

        let expected = Some(Tree {
            value: 0,
            children: vec![Tree {
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
            }],
        });

        assert_eq!(None, tree.clone().prune(|_| true));
        assert_eq!(None, tree.clone().prune(|item| *item == 0));
        assert_eq!(expected, tree.clone().prune(|item| *item == 2));
        assert_eq!(
            expected,
            tree.clone()
                .prune_path(|path, _| matches!(path.get(0), Some(1)))
        );

        let unevenly_pruned_expected = Some(Tree {
            value: 0,
            children: vec![Tree {
                value: 1,
                children: vec![Tree {
                    value: 4,
                    children: Vec::with_capacity(0),
                }],
            }],
        });
        let unevenly_pruned_source = tree.prune(|item| *item == 2).unwrap();

        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source.clone().prune(|item| *item == 3)
        );
        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source
                .clone()
                .prune_path(|path, _| matches!(path.get(1), Some(0)))
        );
    }

    #[test]
    fn prune_tree_iter() {
        let tree = create_tree_for_testing();

        let expected = Some(Tree {
            value: &0,
            children: vec![Tree {
                value: &1,
                children: vec![
                    Tree {
                        value: &3,
                        children: Vec::with_capacity(0),
                    },
                    Tree {
                        value: &4,
                        children: Vec::with_capacity(0),
                    },
                ],
            }],
        });

        assert_eq!(None, tree.prune_ref(|_| true));
        assert_eq!(None, tree.prune_ref(|item| **item == 0));
        assert_eq!(expected, tree.prune_ref(|item| **item == 2));
        assert_eq!(
            expected,
            tree.prune_path_ref(|path, _| matches!(path.get(0), Some(1)))
        );

        let unevenly_pruned_expected = Some(Tree {
            value: &0,
            children: vec![Tree {
                value: &1,
                children: vec![Tree {
                    value: &4,
                    children: Vec::with_capacity(0),
                }],
            }],
        });
        let unevenly_pruned_source = tree.prune_ref(|item| **item == 2).unwrap().cloned();

        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source.prune_ref(|item| **item == 3)
        );
        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source.prune_path_ref(|path, _| matches!(path.get(1), Some(0)))
        );
    }

    #[test]
    fn prune_tree_mut() {
        let mut zero = 0;
        let mut one = 1;
        let mut three = 3;
        let mut four = 4;

        let mut tree = create_tree_for_testing();

        let expected = Some(Tree {
            value: &mut zero,
            children: vec![Tree {
                value: &mut one,
                children: vec![
                    Tree {
                        value: &mut three,
                        children: Vec::with_capacity(0),
                    },
                    Tree {
                        value: &mut four,
                        children: Vec::with_capacity(0),
                    },
                ],
            }],
        });

        assert_eq!(None, tree.prune_mut(|_| true));
        assert_eq!(None, tree.prune_mut(|item| **item == 0));
        assert_eq!(expected, tree.prune_mut(|item| **item == 2));
        assert_eq!(
            expected,
            tree.prune_path_mut(|path, _| matches!(path.get(0), Some(1)))
        );

        let mut unevenly_pruned_source = tree.prune_mut(|item| **item == 2).unwrap().cloned();
        let unevenly_pruned_expected = Some(Tree {
            value: &mut zero,
            children: vec![Tree {
                value: &mut one,
                children: vec![Tree {
                    value: &mut four,
                    children: Vec::with_capacity(0),
                }],
            }],
        });

        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source.prune_mut(|item| **item == 3)
        );
        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source.prune_path_mut(|path, _| matches!(path.get(1), Some(0)))
        );
    }

    #[test]
    fn prune_binary_tree() {
        let tree = create_binary_tree_for_testing();

        let expected = Some(BinaryTree {
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
            right: None,
        });

        assert_eq!(None, tree.clone().prune(|_| true));
        assert_eq!(None, tree.clone().prune(|item| *item == 0));
        assert_eq!(expected, tree.clone().prune(|item| *item == 2));
        assert_eq!(
            expected,
            tree.clone()
                .prune_path(|path, _| matches!(path.get(0), Some(1)))
        );

        let unevenly_pruned_expected = Some(BinaryTree {
            value: 0,
            left: Some(Box::new(BinaryTree {
                value: 1,
                left: None,
                right: Some(Box::new(BinaryTree {
                    value: 4,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        });
        let unevenly_pruned_source = tree.prune(|item| *item == 2).unwrap();

        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source.clone().prune(|item| *item == 3)
        );
        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source
                .clone()
                .prune_path(|path, _| matches!(path.get(1), Some(0)))
        );
    }

    #[test]
    fn prune_binary_tree_iter() {
        let tree = create_binary_tree_for_testing();

        let expected = Some(BinaryTree {
            value: &0,
            left: Some(Box::new(BinaryTree {
                value: &1,
                left: Some(Box::new(BinaryTree {
                    value: &3,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(BinaryTree {
                    value: &4,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        });

        assert_eq!(None, tree.prune_ref(|_| true));
        assert_eq!(None, tree.prune_ref(|item| **item == 0));
        assert_eq!(expected, tree.prune_ref(|item| **item == 2));
        assert_eq!(
            expected,
            tree.prune_path_ref(|path, _| matches!(path.get(0), Some(1)))
        );

        let unevenly_pruned_expected = Some(BinaryTree {
            value: &0,
            left: Some(Box::new(BinaryTree {
                value: &1,
                left: None,
                right: Some(Box::new(BinaryTree {
                    value: &4,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        });
        let unevenly_pruned_source = tree.prune_ref(|item| **item == 2).unwrap().cloned();

        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source.prune_ref(|item| **item == 3)
        );
        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source.prune_path_ref(|path, _| matches!(path.get(1), Some(0)))
        );
    }

    #[test]
    fn prune_binary_tree_mut() {
        let mut tree = create_binary_tree_for_testing();

        let mut zero = 0;
        let mut one = 1;
        let mut three = 3;
        let mut four = 4;

        let expected = Some(BinaryTree {
            value: &mut zero,
            left: Some(Box::new(BinaryTree {
                value: &mut one,
                left: Some(Box::new(BinaryTree {
                    value: &mut three,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(BinaryTree {
                    value: &mut four,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        });

        assert_eq!(None, tree.prune_mut(|_| true));
        assert_eq!(None, tree.prune_mut(|item| **item == 0));
        assert_eq!(expected, tree.prune_mut(|item| **item == 2));
        assert_eq!(
            expected,
            tree.prune_path_mut(|path, _| matches!(path.get(0), Some(1)))
        );

        let mut unevenly_pruned_source = tree.prune_mut(|item| **item == 2).unwrap().cloned();
        let unevenly_pruned_expected = Some(BinaryTree {
            value: &mut zero,
            left: Some(Box::new(BinaryTree {
                value: &mut one,
                left: None,
                right: Some(Box::new(BinaryTree {
                    value: &mut four,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        });

        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source.prune_mut(|item| **item == 3)
        );
        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source.prune_path_mut(|path, _| matches!(path.get(1), Some(0)))
        );
    }

    #[test]
    fn prune_tree_collection() {
        let trees = vec![create_tree_for_testing(), create_tree_for_testing()];

        let expected_tree = Tree {
            value: 0,
            children: vec![Tree {
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
            }],
        };

        let expected: Vec<Tree<usize>> = vec![expected_tree.clone(), expected_tree];

        assert_eq!(
            Vec::<Tree<usize>>::with_capacity(0),
            trees.clone().prune_each(|_| true).collect::<Vec<_>>()
        );
        assert_eq!(
            Vec::<Tree<usize>>::with_capacity(0),
            trees
                .clone()
                .prune_each(|item| *item == 0)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            expected,
            trees
                .clone()
                .prune_each(|item| *item == 2)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            expected,
            trees
                .clone()
                .prune_path_each(|path, _| matches!(path.get(1), Some(1)))
                .collect::<Vec<_>>()
        );

        let unevenly_pruned_expected_tree = Tree {
            value: 0,
            children: vec![Tree {
                value: 1,
                children: vec![Tree {
                    value: 4,
                    children: Vec::with_capacity(0),
                }],
            }],
        };

        let unevenly_pruned_expected = vec![
            unevenly_pruned_expected_tree.clone(),
            unevenly_pruned_expected_tree,
        ];
        let unevenly_pruned_source = trees.prune_each(|item| *item == 2).collect::<Vec<_>>();

        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source
                .clone()
                .prune_each(|item| *item == 3)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source
                .clone()
                .prune_path_each(|path, _| matches!(path.get(2), Some(0)))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn prune_tree_collection_ref() {
        let trees = vec![create_tree_for_testing(), create_tree_for_testing()];

        let expected_tree = Tree {
            value: &0,
            children: vec![Tree {
                value: &1,
                children: vec![
                    Tree {
                        value: &3,
                        children: Vec::with_capacity(0),
                    },
                    Tree {
                        value: &4,
                        children: Vec::with_capacity(0),
                    },
                ],
            }],
        };

        let expected: Vec<Tree<&usize>> = vec![expected_tree.clone(), expected_tree];

        assert_eq!(
            Vec::<Tree<&usize>>::with_capacity(0),
            trees.prune_each_ref(|_| true).collect::<Vec<_>>()
        );
        assert_eq!(
            Vec::<Tree<&usize>>::with_capacity(0),
            trees.prune_each_ref(|item| **item == 0).collect::<Vec<_>>()
        );
        assert_eq!(
            expected,
            trees.prune_each_ref(|item| **item == 2).collect::<Vec<_>>()
        );
        assert_eq!(
            expected,
            trees
                .prune_path_each_ref(|path, _| matches!(path.get(1), Some(1)))
                .collect::<Vec<_>>()
        );

        let unevenly_pruned_expected_tree = Tree {
            value: &0,
            children: vec![Tree {
                value: &1,
                children: vec![Tree {
                    value: &4,
                    children: Vec::with_capacity(0),
                }],
            }],
        };

        let unevenly_pruned_source = trees.prune_each(|item| *item == 2).collect::<Vec<_>>();
        let unevenly_pruned_expected = vec![
            unevenly_pruned_expected_tree.clone(),
            unevenly_pruned_expected_tree,
        ];

        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source
                .prune_each_ref(|item| **item == 3)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source
                .prune_path_each_ref(|path, _| matches!(path.get(2), Some(0)))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn prune_tree_collection_mut() {
        let mut trees = vec![create_tree_for_testing(), create_tree_for_testing()];

        let mut zero = 0;
        let mut one = 1;
        let mut three = 3;
        let mut four = 4;

        let mut second_zero = 0;
        let mut second_one = 1;
        let mut second_three = 3;
        let mut second_four = 4;

        let expected_tree = Tree {
            value: &mut zero,
            children: vec![Tree {
                value: &mut one,
                children: vec![
                    Tree {
                        value: &mut three,
                        children: Vec::with_capacity(0),
                    },
                    Tree {
                        value: &mut four,
                        children: Vec::with_capacity(0),
                    },
                ],
            }],
        };
        let expected_tree2 = Tree {
            value: &mut second_zero,
            children: vec![Tree {
                value: &mut second_one,
                children: vec![
                    Tree {
                        value: &mut second_three,
                        children: Vec::with_capacity(0),
                    },
                    Tree {
                        value: &mut second_four,
                        children: Vec::with_capacity(0),
                    },
                ],
            }],
        };

        let expected: Vec<Tree<&mut usize>> = vec![expected_tree, expected_tree2];

        assert_eq!(
            Vec::<Tree<&mut usize>>::with_capacity(0),
            trees.prune_each_mut(|_| true).collect::<Vec<_>>()
        );
        assert_eq!(
            Vec::<Tree<&mut usize>>::with_capacity(0),
            trees.prune_each_mut(|item| **item == 0).collect::<Vec<_>>()
        );
        assert_eq!(
            expected,
            trees.prune_each_mut(|item| **item == 2).collect::<Vec<_>>()
        );
        assert_eq!(
            expected,
            trees
                .prune_path_each_mut(|path, _| matches!(path.get(1), Some(1)))
                .collect::<Vec<_>>()
        );

        let unevenly_pruned_expected_tree = Tree {
            value: &mut zero,
            children: vec![Tree {
                value: &mut one,
                children: vec![Tree {
                    value: &mut four,
                    children: Vec::with_capacity(0),
                }],
            }],
        };

        let unevenly_pruned_expected_tree2 = Tree {
            value: &mut second_zero,
            children: vec![Tree {
                value: &mut second_one,
                children: vec![Tree {
                    value: &mut second_four,
                    children: Vec::with_capacity(0),
                }],
            }],
        };

        let mut unevenly_pruned_source = trees
            .into_pipeline_mut()
            .prune(|item| **item == 2)
            .map_trees(|item| item.clone())
            .trees()
            .collect::<Vec<_>>();

        let unevenly_pruned_expected = vec![
            unevenly_pruned_expected_tree,
            unevenly_pruned_expected_tree2,
        ];

        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source
                .prune_each_mut(|item| **item == 3)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source
                .prune_path_each_mut(|path, _| matches!(path.get(2), Some(0)))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn prune_binary_tree_collection() {
        let trees = vec![
            create_binary_tree_for_testing(),
            create_binary_tree_for_testing(),
        ];

        let expected_tree = BinaryTree {
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
            right: None,
        };

        let expected: Vec<BinaryTree<usize>> = vec![expected_tree.clone(), expected_tree];

        assert_eq!(
            Vec::<BinaryTree<usize>>::with_capacity(0),
            trees.clone().prune_each(|_| true).collect::<Vec<_>>()
        );
        assert_eq!(
            Vec::<BinaryTree<usize>>::with_capacity(0),
            trees
                .clone()
                .prune_each(|item| *item == 0)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            expected,
            trees
                .clone()
                .prune_each(|item| *item == 2)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            expected,
            trees
                .clone()
                .prune_path_each(|path, _| matches!(path.get(1), Some(1)))
                .collect::<Vec<_>>()
        );

        let unevenly_pruned_expected_tree = BinaryTree {
            value: 0,
            left: Some(Box::new(BinaryTree {
                value: 1,
                left: None,
                right: Some(Box::new(BinaryTree {
                    value: 4,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        };

        let unevenly_pruned_expected = vec![
            unevenly_pruned_expected_tree.clone(),
            unevenly_pruned_expected_tree,
        ];
        let unevenly_pruned_source = trees.prune_each(|item| *item == 2).collect::<Vec<_>>();

        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source
                .clone()
                .prune_each(|item| *item == 3)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source
                .clone()
                .prune_path_each(|path, _| matches!(path.get(2), Some(0)))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn prune_binary_tree_collection_ref() {
        let trees = vec![
            create_binary_tree_for_testing(),
            create_binary_tree_for_testing(),
        ];

        let expected_tree = BinaryTree {
            value: &0,
            left: Some(Box::new(BinaryTree {
                value: &1,
                left: Some(Box::new(BinaryTree {
                    value: &3,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(BinaryTree {
                    value: &4,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        };

        let expected: Vec<BinaryTree<&usize>> = vec![expected_tree.clone(), expected_tree];

        assert_eq!(
            Vec::<BinaryTree<&usize>>::with_capacity(0),
            trees.prune_each_ref(|_| true).collect::<Vec<_>>()
        );
        assert_eq!(
            Vec::<BinaryTree<&usize>>::with_capacity(0),
            trees.prune_each_ref(|item| **item == 0).collect::<Vec<_>>()
        );
        assert_eq!(
            expected,
            trees.prune_each_ref(|item| **item == 2).collect::<Vec<_>>()
        );
        assert_eq!(
            expected,
            trees
                .prune_path_each_ref(|path, _| matches!(path.get(1), Some(1)))
                .collect::<Vec<_>>()
        );

        let unevenly_pruned_expected_tree = BinaryTree {
            value: &0,
            left: Some(Box::new(BinaryTree {
                value: &1,
                left: None,
                right: Some(Box::new(BinaryTree {
                    value: &4,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        };

        let unevenly_pruned_expected = vec![
            unevenly_pruned_expected_tree.clone(),
            unevenly_pruned_expected_tree,
        ];
        let unevenly_pruned_source = trees.prune_each(|item| *item == 2).collect::<Vec<_>>();

        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source
                .prune_each_ref(|item| **item == 3)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source
                .prune_path_each_ref(|path, _| matches!(path.get(2), Some(0)))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn prune_binary_tree_collection_mut() {
        let mut trees = vec![
            create_binary_tree_for_testing(),
            create_binary_tree_for_testing(),
        ];

        let mut zero = 0;
        let mut one = 1;
        let mut three = 3;
        let mut four = 4;

        let mut second_zero = 0;
        let mut second_one = 1;
        let mut second_three = 3;
        let mut second_four = 4;

        let expected_tree = BinaryTree {
            value: &mut zero,
            left: Some(Box::new(BinaryTree {
                value: &mut one,
                left: Some(Box::new(BinaryTree {
                    value: &mut three,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(BinaryTree {
                    value: &mut four,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        };

        let expected_tree2 = BinaryTree {
            value: &mut second_zero,
            left: Some(Box::new(BinaryTree {
                value: &mut second_one,
                left: Some(Box::new(BinaryTree {
                    value: &mut second_three,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(BinaryTree {
                    value: &mut second_four,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        };

        let expected: Vec<BinaryTree<&mut usize>> = vec![expected_tree, expected_tree2];

        assert_eq!(
            Vec::<BinaryTree<&mut usize>>::with_capacity(0),
            trees.prune_each_mut(|_| true).collect::<Vec<_>>()
        );
        assert_eq!(
            Vec::<BinaryTree<&mut usize>>::with_capacity(0),
            trees.prune_each_mut(|item| **item == 0).collect::<Vec<_>>()
        );
        assert_eq!(
            expected,
            trees.prune_each_mut(|item| **item == 2).collect::<Vec<_>>()
        );
        assert_eq!(
            expected,
            trees
                .prune_path_each_mut(|path, _| matches!(path.get(1), Some(1)))
                .collect::<Vec<_>>()
        );

        let unevenly_pruned_expected_tree = BinaryTree {
            value: &mut zero,
            left: Some(Box::new(BinaryTree {
                value: &mut one,
                left: None,
                right: Some(Box::new(BinaryTree {
                    value: &mut four,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        };

        let unevenly_pruned_expected_tree2 = BinaryTree {
            value: &mut second_zero,
            left: Some(Box::new(BinaryTree {
                value: &mut second_one,
                left: None,
                right: Some(Box::new(BinaryTree {
                    value: &mut second_four,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        };

        let unevenly_pruned_expected = vec![
            unevenly_pruned_expected_tree,
            unevenly_pruned_expected_tree2,
        ];
        let mut unevenly_pruned_source = trees
            .into_pipeline_mut()
            .prune(|item| **item == 2)
            .map_trees(|item| item.clone())
            .trees()
            .collect::<Vec<_>>();

        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source
                .prune_each_mut(|item| **item == 3)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            unevenly_pruned_expected,
            unevenly_pruned_source
                .prune_path_each_mut(|path, _| matches!(path.get(2), Some(0)))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn prune_depth_tree() {
        let tree = create_tree_for_testing();
        let tree_ref = tree.clone();
        let mut tree_mut = tree.clone();

        // depth 0
        assert_eq!(
            tree.clone().prune_depth(0),
            Tree {
                value: 0,
                children: Vec::with_capacity(0)
            }
        );
        assert_eq!(
            tree_ref.prune_depth_ref(0),
            Tree {
                value: &0,
                children: Vec::with_capacity(0)
            }
        );
        assert_eq!(
            tree_mut.prune_depth_mut(0),
            Tree {
                value: &mut 0,
                children: Vec::with_capacity(0),
            }
        );

        // depth 1
        assert_eq!(
            tree.clone().prune_depth(1),
            Tree {
                value: 0,
                children: vec![
                    Tree {
                        value: 1,
                        children: Vec::with_capacity(0),
                    },
                    Tree {
                        value: 2,
                        children: Vec::with_capacity(0),
                    }
                ]
            }
        );
        assert_eq!(
            tree_ref.prune_depth_ref(1),
            Tree {
                value: &0,
                children: vec![
                    Tree {
                        value: &1,
                        children: Vec::with_capacity(0),
                    },
                    Tree {
                        value: &2,
                        children: Vec::with_capacity(0),
                    }
                ]
            }
        );
        assert_eq!(
            tree_mut.prune_depth_mut(1),
            Tree {
                value: &mut 0,
                children: vec![
                    Tree {
                        value: &mut 1,
                        children: Vec::with_capacity(0),
                    },
                    Tree {
                        value: &mut 2,
                        children: Vec::with_capacity(0),
                    }
                ]
            }
        );

        // depth 2
        assert_eq!(
            tree.clone().prune_depth(2),
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
                            }
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
                                children: Vec::with_capacity(0),
                            }
                        ],
                    }
                ]
            }
        );
        assert_eq!(
            tree_ref.prune_depth_ref(2),
            Tree {
                value: &0,
                children: vec![
                    Tree {
                        value: &1,
                        children: vec![
                            Tree {
                                value: &3,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &4,
                                children: Vec::with_capacity(0),
                            }
                        ],
                    },
                    Tree {
                        value: &2,
                        children: vec![
                            Tree {
                                value: &5,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &6,
                                children: Vec::with_capacity(0),
                            }
                        ],
                    }
                ]
            }
        );
        assert_eq!(
            tree_mut.prune_depth_mut(2),
            Tree {
                value: &mut 0,
                children: vec![
                    Tree {
                        value: &mut 1,
                        children: vec![
                            Tree {
                                value: &mut 3,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &mut 4,
                                children: Vec::with_capacity(0),
                            }
                        ],
                    },
                    Tree {
                        value: &mut 2,
                        children: vec![
                            Tree {
                                value: &mut 5,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &mut 6,
                                children: Vec::with_capacity(0),
                            }
                        ],
                    }
                ]
            }
        );

        // depth 3
        assert_eq!(
            tree.clone().prune_depth(3),
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
                            }
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
                                    children: Vec::with_capacity(0)
                                }],
                            }
                        ],
                    }
                ]
            }
        );
        assert_eq!(
            tree_ref.prune_depth_ref(3),
            Tree {
                value: &0,
                children: vec![
                    Tree {
                        value: &1,
                        children: vec![
                            Tree {
                                value: &3,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &4,
                                children: Vec::with_capacity(0),
                            }
                        ],
                    },
                    Tree {
                        value: &2,
                        children: vec![
                            Tree {
                                value: &5,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &6,
                                children: vec![Tree {
                                    value: &7,
                                    children: Vec::with_capacity(0)
                                }],
                            }
                        ],
                    }
                ]
            }
        );
        assert_eq!(
            tree_mut.prune_depth_mut(3),
            Tree {
                value: &mut 0,
                children: vec![
                    Tree {
                        value: &mut 1,
                        children: vec![
                            Tree {
                                value: &mut 3,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &mut 4,
                                children: Vec::with_capacity(0),
                            }
                        ],
                    },
                    Tree {
                        value: &mut 2,
                        children: vec![
                            Tree {
                                value: &mut 5,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &mut 6,
                                children: vec![Tree {
                                    value: &mut 7,
                                    children: Vec::with_capacity(0)
                                }],
                            }
                        ],
                    }
                ]
            }
        );

        // depth 4
        assert_eq!(
            tree.clone().prune_depth(4),
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
                            }
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
                                        children: Vec::with_capacity(0),
                                    }]
                                }],
                            }
                        ],
                    }
                ]
            }
        );
        assert_eq!(
            tree_ref.prune_depth_ref(4),
            Tree {
                value: &0,
                children: vec![
                    Tree {
                        value: &1,
                        children: vec![
                            Tree {
                                value: &3,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &4,
                                children: Vec::with_capacity(0),
                            }
                        ],
                    },
                    Tree {
                        value: &2,
                        children: vec![
                            Tree {
                                value: &5,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &6,
                                children: vec![Tree {
                                    value: &7,
                                    children: vec![Tree {
                                        value: &8,
                                        children: Vec::with_capacity(0),
                                    }]
                                }],
                            }
                        ],
                    }
                ]
            }
        );
        assert_eq!(
            tree_mut.prune_depth_mut(4),
            Tree {
                value: &mut 0,
                children: vec![
                    Tree {
                        value: &mut 1,
                        children: vec![
                            Tree {
                                value: &mut 3,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &mut 4,
                                children: Vec::with_capacity(0),
                            }
                        ],
                    },
                    Tree {
                        value: &mut 2,
                        children: vec![
                            Tree {
                                value: &mut 5,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &mut 6,
                                children: vec![Tree {
                                    value: &mut 7,
                                    children: vec![Tree {
                                        value: &mut 8,
                                        children: Vec::with_capacity(0),
                                    }]
                                }],
                            }
                        ],
                    }
                ]
            }
        );

        // depth 5
        assert_eq!(
            tree.clone().prune_depth(5),
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
                            }
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
                                            children: Vec::with_capacity(0)
                                        }],
                                    }]
                                }],
                            }
                        ],
                    }
                ]
            }
        );
        assert_eq!(
            tree_ref.prune_depth_ref(5),
            Tree {
                value: &0,
                children: vec![
                    Tree {
                        value: &1,
                        children: vec![
                            Tree {
                                value: &3,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &4,
                                children: Vec::with_capacity(0),
                            }
                        ],
                    },
                    Tree {
                        value: &2,
                        children: vec![
                            Tree {
                                value: &5,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &6,
                                children: vec![Tree {
                                    value: &7,
                                    children: vec![Tree {
                                        value: &8,
                                        children: vec![Tree {
                                            value: &9,
                                            children: Vec::with_capacity(0)
                                        }],
                                    }]
                                }],
                            }
                        ],
                    }
                ]
            }
        );
        assert_eq!(
            tree_mut.prune_depth_mut(5),
            Tree {
                value: &mut 0,
                children: vec![
                    Tree {
                        value: &mut 1,
                        children: vec![
                            Tree {
                                value: &mut 3,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &mut 4,
                                children: Vec::with_capacity(0),
                            }
                        ],
                    },
                    Tree {
                        value: &mut 2,
                        children: vec![
                            Tree {
                                value: &mut 5,
                                children: Vec::with_capacity(0),
                            },
                            Tree {
                                value: &mut 6,
                                children: vec![Tree {
                                    value: &mut 7,
                                    children: vec![Tree {
                                        value: &mut 8,
                                        children: vec![Tree {
                                            value: &mut 9,
                                            children: Vec::with_capacity(0)
                                        }],
                                    }]
                                }],
                            }
                        ],
                    }
                ]
            }
        );

        for depth in 6..10 {
            assert_eq!(tree.clone().prune_depth(depth), tree);
            assert_eq!(
                tree_ref.map_ref(|item| item),
                tree_ref.prune_depth_ref(depth)
            );
            assert_eq!(
                tree_mut.clone().map_mut(|item| item),
                tree_mut.prune_depth_mut(depth)
            );
        }
    }

    #[test]
    fn prune_depth_binary_tree() {
        let tree = create_binary_tree_for_testing();
        let tree_ref = tree.clone();
        let mut tree_mut = tree.clone();

        // depth 0
        assert_eq!(
            tree.clone().prune_depth(0),
            BinaryTree {
                value: 0,
                left: None,
                right: None,
            }
        );
        assert_eq!(
            tree_ref.prune_depth_ref(0),
            BinaryTree {
                value: &0,
                left: None,
                right: None,
            }
        );
        assert_eq!(
            tree_mut.prune_depth_mut(0),
            BinaryTree {
                value: &mut 0,
                left: None,
                right: None,
            }
        );

        // depth 1
        assert_eq!(
            tree.clone().prune_depth(1),
            BinaryTree {
                value: 0,
                left: Some(Box::new(BinaryTree {
                    value: 1,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(BinaryTree {
                    value: 2,
                    left: None,
                    right: None,
                }))
            }
        );
        assert_eq!(
            tree_ref.prune_depth_ref(1),
            BinaryTree {
                value: &0,
                left: Some(Box::new(BinaryTree {
                    value: &1,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(BinaryTree {
                    value: &2,
                    left: None,
                    right: None,
                }))
            }
        );
        assert_eq!(
            tree_mut.prune_depth_mut(1),
            BinaryTree {
                value: &mut 0,
                left: Some(Box::new(BinaryTree {
                    value: &mut 1,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(BinaryTree {
                    value: &mut 2,
                    left: None,
                    right: None,
                }))
            }
        );

        // depth 2
        assert_eq!(
            tree.clone().prune_depth(2),
            BinaryTree {
                value: 0,
                left: Some(Box::new(BinaryTree {
                    value: 1,
                    left: Some(Box::new(BinaryTree {
                        value: 3,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: 4,
                        left: None,
                        right: None
                    })),
                })),
                right: Some(Box::new(BinaryTree {
                    value: 2,
                    left: Some(Box::new(BinaryTree {
                        value: 5,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: 6,
                        left: None,
                        right: None
                    })),
                }))
            }
        );
        assert_eq!(
            tree_ref.prune_depth_ref(2),
            BinaryTree {
                value: &0,
                left: Some(Box::new(BinaryTree {
                    value: &1,
                    left: Some(Box::new(BinaryTree {
                        value: &3,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &4,
                        left: None,
                        right: None
                    })),
                })),
                right: Some(Box::new(BinaryTree {
                    value: &2,
                    left: Some(Box::new(BinaryTree {
                        value: &5,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &6,
                        left: None,
                        right: None
                    })),
                }))
            }
        );
        assert_eq!(
            tree_mut.prune_depth_mut(2),
            BinaryTree {
                value: &mut 0,
                left: Some(Box::new(BinaryTree {
                    value: &mut 1,
                    left: Some(Box::new(BinaryTree {
                        value: &mut 3,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &mut 4,
                        left: None,
                        right: None
                    })),
                })),
                right: Some(Box::new(BinaryTree {
                    value: &mut 2,
                    left: Some(Box::new(BinaryTree {
                        value: &mut 5,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &mut 6,
                        left: None,
                        right: None
                    })),
                }))
            }
        );

        // depth 3
        assert_eq!(
            tree.clone().prune_depth(3),
            BinaryTree {
                value: 0,
                left: Some(Box::new(BinaryTree {
                    value: 1,
                    left: Some(Box::new(BinaryTree {
                        value: 3,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: 4,
                        left: None,
                        right: None
                    })),
                })),
                right: Some(Box::new(BinaryTree {
                    value: 2,
                    left: Some(Box::new(BinaryTree {
                        value: 5,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: 6,
                        left: Some(Box::new(BinaryTree {
                            value: 7,
                            left: None,
                            right: None
                        })),
                        right: None
                    })),
                }))
            }
        );
        assert_eq!(
            tree_ref.prune_depth_ref(3),
            BinaryTree {
                value: &0,
                left: Some(Box::new(BinaryTree {
                    value: &1,
                    left: Some(Box::new(BinaryTree {
                        value: &3,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &4,
                        left: None,
                        right: None
                    })),
                })),
                right: Some(Box::new(BinaryTree {
                    value: &2,
                    left: Some(Box::new(BinaryTree {
                        value: &5,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &6,
                        left: Some(Box::new(BinaryTree {
                            value: &7,
                            left: None,
                            right: None
                        })),
                        right: None
                    })),
                }))
            }
        );
        assert_eq!(
            tree_mut.prune_depth_mut(3),
            BinaryTree {
                value: &mut 0,
                left: Some(Box::new(BinaryTree {
                    value: &mut 1,
                    left: Some(Box::new(BinaryTree {
                        value: &mut 3,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &mut 4,
                        left: None,
                        right: None
                    })),
                })),
                right: Some(Box::new(BinaryTree {
                    value: &mut 2,
                    left: Some(Box::new(BinaryTree {
                        value: &mut 5,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &mut 6,
                        left: Some(Box::new(BinaryTree {
                            value: &mut 7,
                            left: None,
                            right: None
                        })),
                        right: None
                    })),
                }))
            }
        );

        // depth 4
        assert_eq!(
            tree.clone().prune_depth(4),
            BinaryTree {
                value: 0,
                left: Some(Box::new(BinaryTree {
                    value: 1,
                    left: Some(Box::new(BinaryTree {
                        value: 3,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: 4,
                        left: None,
                        right: None
                    })),
                })),
                right: Some(Box::new(BinaryTree {
                    value: 2,
                    left: Some(Box::new(BinaryTree {
                        value: 5,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: 6,
                        left: Some(Box::new(BinaryTree {
                            value: 7,
                            left: None,
                            right: Some(Box::new(BinaryTree {
                                value: 8,
                                left: None,
                                right: None
                            }))
                        })),
                        right: None
                    })),
                }))
            }
        );
        assert_eq!(
            tree_ref.prune_depth_ref(4),
            BinaryTree {
                value: &0,
                left: Some(Box::new(BinaryTree {
                    value: &1,
                    left: Some(Box::new(BinaryTree {
                        value: &3,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &4,
                        left: None,
                        right: None
                    })),
                })),
                right: Some(Box::new(BinaryTree {
                    value: &2,
                    left: Some(Box::new(BinaryTree {
                        value: &5,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &6,
                        left: Some(Box::new(BinaryTree {
                            value: &7,
                            left: None,
                            right: Some(Box::new(BinaryTree {
                                value: &8,
                                left: None,
                                right: None
                            }))
                        })),
                        right: None
                    })),
                }))
            }
        );
        assert_eq!(
            tree_mut.prune_depth_mut(4),
            BinaryTree {
                value: &mut 0,
                left: Some(Box::new(BinaryTree {
                    value: &mut 1,
                    left: Some(Box::new(BinaryTree {
                        value: &mut 3,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &mut 4,
                        left: None,
                        right: None
                    })),
                })),
                right: Some(Box::new(BinaryTree {
                    value: &mut 2,
                    left: Some(Box::new(BinaryTree {
                        value: &mut 5,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &mut 6,
                        left: Some(Box::new(BinaryTree {
                            value: &mut 7,
                            left: None,
                            right: Some(Box::new(BinaryTree {
                                value: &mut 8,
                                left: None,
                                right: None
                            }))
                        })),
                        right: None
                    })),
                }))
            }
        );

        // depth 5
        assert_eq!(
            tree.clone().prune_depth(5),
            BinaryTree {
                value: 0,
                left: Some(Box::new(BinaryTree {
                    value: 1,
                    left: Some(Box::new(BinaryTree {
                        value: 3,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: 4,
                        left: None,
                        right: None
                    })),
                })),
                right: Some(Box::new(BinaryTree {
                    value: 2,
                    left: Some(Box::new(BinaryTree {
                        value: 5,
                        left: None,
                        right: None
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
                                    right: None
                                })),
                                right: None
                            }))
                        })),
                        right: None
                    })),
                }))
            }
        );
        assert_eq!(
            tree_ref.prune_depth_ref(5),
            BinaryTree {
                value: &0,
                left: Some(Box::new(BinaryTree {
                    value: &1,
                    left: Some(Box::new(BinaryTree {
                        value: &3,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &4,
                        left: None,
                        right: None
                    })),
                })),
                right: Some(Box::new(BinaryTree {
                    value: &2,
                    left: Some(Box::new(BinaryTree {
                        value: &5,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &6,
                        left: Some(Box::new(BinaryTree {
                            value: &7,
                            left: None,
                            right: Some(Box::new(BinaryTree {
                                value: &8,
                                left: Some(Box::new(BinaryTree {
                                    value: &9,
                                    left: None,
                                    right: None
                                })),
                                right: None
                            }))
                        })),
                        right: None
                    })),
                }))
            }
        );
        assert_eq!(
            tree_mut.prune_depth_mut(5),
            BinaryTree {
                value: &mut 0,
                left: Some(Box::new(BinaryTree {
                    value: &mut 1,
                    left: Some(Box::new(BinaryTree {
                        value: &mut 3,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &mut 4,
                        left: None,
                        right: None
                    })),
                })),
                right: Some(Box::new(BinaryTree {
                    value: &mut 2,
                    left: Some(Box::new(BinaryTree {
                        value: &mut 5,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(BinaryTree {
                        value: &mut 6,
                        left: Some(Box::new(BinaryTree {
                            value: &mut 7,
                            left: None,
                            right: Some(Box::new(BinaryTree {
                                value: &mut 8,
                                left: Some(Box::new(BinaryTree {
                                    value: &mut 9,
                                    left: None,
                                    right: None
                                })),
                                right: None
                            }))
                        })),
                        right: None
                    })),
                }))
            }
        );

        for depth in 6..10 {
            assert_eq!(tree.clone().prune_depth(depth), tree);
            assert_eq!(
                tree_ref.prune_depth_ref(depth),
                tree_ref.map_ref(|item| item)
            );
            assert_eq!(
                tree.clone().prune_depth_mut(depth),
                tree_mut.map_mut(|item| item)
            );
        }
    }