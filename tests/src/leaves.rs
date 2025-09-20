use super::{assert_len, create_binary_tree_for_testing, create_trees_for_testing};
use tree_iterators_rs::prelude::*;

fn get_expected_order_leaves() -> [usize; 4] {
    [3, 4, 5, 10]
}

#[test]
fn leaves_has_correct_order() {
    let expected = get_expected_order_leaves();
    for mut test_tree in create_trees_for_testing() {
        for borrowed_iter in get_borrowed_leaves_iters(&test_tree) {
            for (i, value) in borrowed_iter.enumerate() {
                assert_eq!(expected[i], *value);
            }
        }

        for borrowed_iter in get_borrowed_leaves_iters(&test_tree) {
            assert_len!(expected.len(), borrowed_iter);
        }

        for (i, value) in test_tree.dfs_preorder_iter_mut().leaves().enumerate() {
            assert_eq!(expected[i], *value);
        }

        for (i, value) in test_tree.dfs_postorder_iter_mut().leaves().enumerate() {
            assert_eq!(expected[i], *value);
        }

        for (i, value) in test_tree.bfs_iter_mut().leaves().enumerate() {
            assert_eq!(expected[i], *value);
        }

        assert_len!(expected.len(), test_tree.dfs_preorder_iter_mut().leaves());
        assert_len!(expected.len(), test_tree.dfs_postorder_iter_mut().leaves());
        assert_len!(expected.len(), test_tree.bfs_iter_mut().leaves());

        for owned_iter in get_owned_leaves_iters(test_tree.clone()) {
            for (i, value) in owned_iter.enumerate() {
                assert_eq!(expected[i], value);
            }
        }

        for owned_iter in get_owned_leaves_iters(test_tree) {
            assert_len!(expected.len(), owned_iter);
        }
    }
}

#[test]
fn root_only_leaves_has_correct_order() {
    let mut tree = Tree {
        value: 0,
        children: vec![],
    };

    for borrowed_iter in get_borrowed_leaves_iters(&tree) {
        assert_len!(1, borrowed_iter);
    }

    for borrowed_iter in get_borrowed_leaves_iters(&tree) {
        for item in borrowed_iter {
            assert_eq!(0, *item);
        }
    }

    assert_len!(1, tree.dfs_preorder_iter_mut().leaves());
    assert_len!(1, tree.dfs_postorder_iter_mut().leaves());
    assert_len!(1, tree.bfs_iter_mut().leaves());

    for item in tree.dfs_preorder_iter_mut().leaves() {
        assert_eq!(0, *item);
    }

    for item in tree.dfs_postorder_iter_mut().leaves() {
        assert_eq!(0, *item);
    }

    for item in tree.bfs_iter_mut().leaves() {
        assert_eq!(0, *item);
    }

    for owned_iter in get_owned_leaves_iters(tree.clone()) {
        assert_len!(1, owned_iter);
    }

    for owned_iter in get_owned_leaves_iters(tree.clone()) {
        for item in owned_iter {
            assert_eq!(0, item);
        }
    }
}

fn get_borrowed_leaves_iters<T>(
    test_tree: &Tree<T>,
) -> impl Iterator<Item = Box<dyn Iterator<Item = &T> + '_>> + '_ {
    [
        Box::new(test_tree.dfs_preorder_iter().leaves()) as Box<dyn Iterator<Item = &T>>,
        Box::new(test_tree.dfs_postorder_iter().leaves()),
        Box::new(test_tree.bfs_iter().leaves()),
    ]
    .into_iter()
}

fn get_owned_leaves_iters<T: Clone + 'static>(
    test_tree: Tree<T>,
) -> [Box<dyn Iterator<Item = T>>; 3] {
    [
        Box::new(test_tree.clone().dfs_preorder().leaves()) as Box<dyn Iterator<Item = T>>,
        Box::new(test_tree.clone().dfs_postorder().leaves()),
        Box::new(test_tree.clone().bfs().leaves()),
    ]
}

#[test]
fn binary_leaves_has_correct_order() {
    let expected = get_expected_order_leaves();
    let mut test_tree = create_binary_tree_for_testing();

    for borrowed_iter in get_borrowed_leaves_binary_iters(&test_tree) {
        for (i, value) in borrowed_iter.enumerate() {
            assert_eq!(expected[i], *value);
        }
    }

    for borrowed_iter in get_borrowed_leaves_binary_iters(&test_tree) {
        assert_len!(expected.len(), borrowed_iter);
    }

    for (i, value) in test_tree.dfs_preorder_iter_mut().leaves().enumerate() {
        assert_eq!(expected[i], *value);
    }

    for (i, value) in test_tree.dfs_inorder_iter_mut().leaves().enumerate() {
        assert_eq!(expected[i], *value);
    }

    for (i, value) in test_tree.dfs_postorder_iter_mut().leaves().enumerate() {
        assert_eq!(expected[i], *value);
    }

    for (i, value) in test_tree.bfs_iter_mut().leaves().enumerate() {
        assert_eq!(expected[i], *value);
    }

    assert_len!(
        expected.len(),
        test_tree.dfs_preorder_iter_mut().leaves().enumerate()
    );
    assert_len!(
        expected.len(),
        test_tree.dfs_inorder_iter_mut().leaves().enumerate()
    );
    assert_len!(
        expected.len(),
        test_tree.dfs_postorder_iter_mut().leaves().enumerate()
    );
    assert_len!(
        expected.len(),
        test_tree.bfs_iter_mut().leaves().enumerate()
    );

    for owned_iter in get_owned_leaves_binary_iters(test_tree.clone()) {
        for (i, value) in owned_iter.enumerate() {
            assert_eq!(expected[i], value);
        }
    }

    for owned_iter in get_owned_leaves_binary_iters(test_tree) {
        assert_len!(expected.len(), owned_iter);
    }
}

#[test]
fn binary_root_only_leaves_has_correct_order() {
    let mut tree = BinaryTree {
        value: 0,
        left: None,
        right: None,
    };

    for borrowed_iter in get_borrowed_leaves_binary_iters(&tree) {
        assert_len!(1, borrowed_iter);
    }

    for borrowed_iter in get_borrowed_leaves_binary_iters(&tree) {
        for item in borrowed_iter {
            assert_eq!(0, *item);
        }
    }

    assert_len!(1, tree.dfs_preorder_iter_mut().leaves());
    assert_len!(1, tree.dfs_inorder_iter_mut().leaves());
    assert_len!(1, tree.dfs_postorder_iter_mut().leaves());
    assert_len!(1, tree.bfs_iter_mut().leaves());

    for item in tree.dfs_preorder_iter_mut().leaves() {
        assert_eq!(0, *item);
    }

    for item in tree.dfs_inorder_iter_mut().leaves() {
        assert_eq!(0, *item);
    }

    for item in tree.dfs_postorder_iter_mut().leaves() {
        assert_eq!(0, *item);
    }

    for item in tree.bfs_iter_mut().leaves() {
        assert_eq!(0, *item);
    }

    for owned_iter in get_owned_leaves_binary_iters(tree.clone()) {
        assert_len!(1, owned_iter);
    }

    for owned_iter in get_owned_leaves_binary_iters(tree.clone()) {
        for item in owned_iter {
            assert_eq!(0, item);
        }
    }
}

fn get_borrowed_leaves_binary_iters<T>(
    test_tree: &BinaryTree<T>,
) -> [Box<dyn Iterator<Item = &T> + '_>; 4] {
    [
        Box::new(test_tree.dfs_preorder_iter().leaves()) as Box<dyn Iterator<Item = &T>>,
        Box::new(test_tree.dfs_inorder_iter().leaves()),
        Box::new(test_tree.dfs_postorder_iter().leaves()),
        Box::new(test_tree.bfs_iter().leaves()),
    ]
}

fn get_owned_leaves_binary_iters<T: Clone + 'static>(
    test_tree: BinaryTree<T>,
) -> [Box<dyn Iterator<Item = T>>; 4] {
    [
        Box::new(test_tree.clone().dfs_preorder().leaves()) as Box<dyn Iterator<Item = T>>,
        Box::new(test_tree.clone().dfs_inorder().leaves()),
        Box::new(test_tree.clone().dfs_postorder().leaves()),
        Box::new(test_tree.clone().bfs().leaves()),
    ]
}

#[test]
fn dfs_preorder_transformation_can_happen_mid_traversal() {
    let expected_dfs_preorder = super::dfs_preorder::get_expected_order_dfs_preorder();
    let expected_leaves = get_expected_order_leaves();
    for mut test_tree in create_trees_for_testing() {
        // interrupt traversal at all points.
        for _ in 0..expected_dfs_preorder.len() {
            let mut preorder_iter = test_tree.dfs_preorder_iter();
            let mut num_leaves_seen = 0;
            for value in preorder_iter.by_ref() {
                if *value == expected_leaves[num_leaves_seen] {
                    num_leaves_seen += 1;
                }
            }

            let preorder_iter_leaves = preorder_iter.leaves();
            for (i, value) in preorder_iter_leaves.enumerate() {
                assert_eq!(expected_leaves[i + num_leaves_seen], *value);
            }

            let mut preorder_iter_mut = test_tree.dfs_preorder_iter_mut();
            let mut num_leaves_seen = 0;
            for value in preorder_iter_mut.by_ref() {
                if *value == expected_leaves[num_leaves_seen] {
                    num_leaves_seen += 1;
                }
            }

            let preorder_iter_leaves = preorder_iter_mut.leaves();
            for (i, value) in preorder_iter_leaves.enumerate() {
                assert_eq!(expected_leaves[i + num_leaves_seen], *value);
            }

            let mut preorder = test_tree.clone().dfs_preorder();
            let mut num_leaves_seen = 0;
            for value in preorder.by_ref() {
                if value == expected_leaves[num_leaves_seen] {
                    num_leaves_seen += 1;
                }
            }

            let preorder_iter_leaves = preorder.leaves();
            for (i, value) in preorder_iter_leaves.enumerate() {
                assert_eq!(expected_leaves[i + num_leaves_seen], value);
            }
        }
    }
}

#[test]
fn dfs_postorder_transformation_can_happen_mid_traversal() {
    let expected_dfs_postorder = super::dfs_postorder::get_expected_order_dfs_postorder();
    let expected_leaves = get_expected_order_leaves();
    for mut test_tree in create_trees_for_testing() {
        // interrupt traversal at all points.
        for _ in 0..expected_dfs_postorder.len() {
            let mut postorder_iter = test_tree.dfs_postorder_iter();
            let mut num_leaves_seen = 0;
            for value in postorder_iter.by_ref() {
                // dont index outside the array!
                if num_leaves_seen == expected_leaves.len() {
                    continue;
                }
                if *value == expected_leaves[num_leaves_seen] {
                    num_leaves_seen += 1;
                }
            }

            let postorder_iter_leaves = postorder_iter.leaves();
            for (i, value) in postorder_iter_leaves.enumerate() {
                assert_eq!(expected_leaves[i + num_leaves_seen], *value);
            }

            let mut postorder_iter_mut = test_tree.dfs_postorder_iter_mut();
            let mut num_leaves_seen = 0;
            for value in postorder_iter_mut.by_ref() {
                // dont index outside the array!
                if num_leaves_seen == expected_leaves.len() {
                    continue;
                }
                if *value == expected_leaves[num_leaves_seen] {
                    num_leaves_seen += 1;
                }
            }

            let postorder_iter_leaves = postorder_iter_mut.leaves();
            for (i, value) in postorder_iter_leaves.enumerate() {
                assert_eq!(expected_leaves[i + num_leaves_seen], *value);
            }

            let mut postorder = test_tree.clone().dfs_postorder();
            let mut num_leaves_seen = 0;
            for value in postorder.by_ref() {
                // dont index outside the array!
                if num_leaves_seen == expected_leaves.len() {
                    continue;
                }
                if value == expected_leaves[num_leaves_seen] {
                    num_leaves_seen += 1;
                }
            }

            let postorder_iter_leaves = postorder.leaves();
            for (i, value) in postorder_iter_leaves.enumerate() {
                assert_eq!(expected_leaves[i + num_leaves_seen], value);
            }
        }
    }
}

#[test]
fn bfs_transformation_can_happen_mid_traversal() {
    let expected_bfs = super::bfs::get_expected_order_bfs();
    let expected_leaves = get_expected_order_leaves();
    for mut test_tree in create_trees_for_testing() {
        // interrupt traversal at all points.
        for _ in 0..expected_bfs.len() {
            let mut bfs_iter = test_tree.bfs_iter();
            let mut num_leaves_seen = 0;
            for value in bfs_iter.by_ref() {
                if *value == expected_leaves[num_leaves_seen] {
                    num_leaves_seen += 1;
                }
            }

            let bfs_iter_leaves = bfs_iter.leaves();
            for (i, value) in bfs_iter_leaves.enumerate() {
                assert_eq!(expected_leaves[i + num_leaves_seen], *value);
            }

            let mut bfs_iter_mut = test_tree.bfs_iter_mut();
            let mut num_leaves_seen = 0;
            for value in bfs_iter_mut.by_ref() {
                if *value == expected_leaves[num_leaves_seen] {
                    num_leaves_seen += 1;
                }
            }

            let bfs_iter_mut_leaves = bfs_iter_mut.leaves();
            for (i, value) in bfs_iter_mut_leaves.enumerate() {
                assert_eq!(expected_leaves[i + num_leaves_seen], *value);
            }

            let mut bfs = test_tree.clone().bfs();
            let mut num_leaves_seen = 0;
            for value in bfs.by_ref() {
                if value == expected_leaves[num_leaves_seen] {
                    num_leaves_seen += 1;
                }
            }

            let bfs_leaves = bfs.leaves();
            for (i, value) in bfs_leaves.enumerate() {
                assert_eq!(expected_leaves[i + num_leaves_seen], value);
            }
        }
    }
}

#[test]
fn dfs_preorder_binary_transformation_can_happen_mid_traversal() {
    let expected_dfs_preorder = super::dfs_preorder::get_expected_order_dfs_preorder();
    let expected_leaves = get_expected_order_leaves();
    let mut test_tree = create_binary_tree_for_testing();
    // interrupt traversal at all points.
    for _ in 0..expected_dfs_preorder.len() {
        let mut preorder_iter = test_tree.dfs_preorder_iter();
        let mut num_leaves_seen = 0;
        for value in preorder_iter.by_ref() {
            if *value == expected_leaves[num_leaves_seen] {
                num_leaves_seen += 1;
            }
        }

        let preorder_iter_leaves = preorder_iter.leaves();
        for (i, value) in preorder_iter_leaves.enumerate() {
            assert_eq!(expected_leaves[i + num_leaves_seen], *value);
        }

        let mut preorder_iter_mut = test_tree.dfs_preorder_iter_mut();
        let mut num_leaves_seen = 0;
        for value in preorder_iter_mut.by_ref() {
            if *value == expected_leaves[num_leaves_seen] {
                num_leaves_seen += 1;
            }
        }

        let preorder_iter_leaves = preorder_iter_mut.leaves();
        for (i, value) in preorder_iter_leaves.enumerate() {
            assert_eq!(expected_leaves[i + num_leaves_seen], *value);
        }

        let mut preorder = test_tree.clone().dfs_preorder();
        let mut num_leaves_seen = 0;
        for value in preorder.by_ref() {
            if value == expected_leaves[num_leaves_seen] {
                num_leaves_seen += 1;
            }
        }

        let preorder_iter_leaves = preorder.leaves();
        for (i, value) in preorder_iter_leaves.enumerate() {
            assert_eq!(expected_leaves[i + num_leaves_seen], value);
        }
    }
}

#[test]
fn dfs_inorder_binary_transformation_can_happen_mid_traversal() {
    let expected_dfs_inorder = super::dfs_inorder::get_expected_order_dfs_inorder();
    let expected_leaves = get_expected_order_leaves();
    let mut test_tree = create_binary_tree_for_testing();
    // interrupt traversal at all points.
    for _ in 0..expected_dfs_inorder.len() {
        let mut inorder_iter = test_tree.dfs_inorder_iter();
        let mut num_leaves_seen = 0;
        for value in inorder_iter.by_ref() {
            // dont index outside the array!
            if num_leaves_seen == expected_leaves.len() {
                continue;
            }
            if *value == expected_leaves[num_leaves_seen] {
                num_leaves_seen += 1;
            }
        }

        let inorder_iter_leaves = inorder_iter.leaves();
        for (i, value) in inorder_iter_leaves.enumerate() {
            assert_eq!(expected_leaves[i + num_leaves_seen], *value);
        }

        let mut inorder_iter_mut = test_tree.dfs_inorder_iter_mut();
        let mut num_leaves_seen = 0;
        for value in inorder_iter_mut.by_ref() {
            // dont index outside the array!
            if num_leaves_seen == expected_leaves.len() {
                continue;
            }
            if *value == expected_leaves[num_leaves_seen] {
                num_leaves_seen += 1;
            }
        }

        let inorder_iter_leaves = inorder_iter_mut.leaves();
        for (i, value) in inorder_iter_leaves.enumerate() {
            assert_eq!(expected_leaves[i + num_leaves_seen], *value);
        }

        let mut inorder = test_tree.clone().dfs_inorder();
        let mut num_leaves_seen = 0;
        for value in inorder.by_ref() {
            // dont index outside the array!
            if num_leaves_seen == expected_leaves.len() {
                continue;
            }
            if value == expected_leaves[num_leaves_seen] {
                num_leaves_seen += 1;
            }
        }

        let inorder_iter_leaves = inorder.leaves();
        for (i, value) in inorder_iter_leaves.enumerate() {
            assert_eq!(expected_leaves[i + num_leaves_seen], value);
        }
    }
}

#[test]
fn dfs_postorder_binary_transformation_can_happen_mid_traversal() {
    let expected_dfs_postorder = super::dfs_postorder::get_expected_order_dfs_postorder();
    let expected_leaves = get_expected_order_leaves();
    let mut test_tree = create_binary_tree_for_testing();
    // interrupt traversal at all points.
    for _ in 0..expected_dfs_postorder.len() {
        let mut postorder_iter = test_tree.dfs_postorder_iter();
        let mut num_leaves_seen = 0;
        for value in postorder_iter.by_ref() {
            // dont index outside the array!
            if num_leaves_seen == expected_leaves.len() {
                continue;
            }
            if *value == expected_leaves[num_leaves_seen] {
                num_leaves_seen += 1;
            }
        }

        let postorder_iter_leaves = postorder_iter.leaves();
        for (i, value) in postorder_iter_leaves.enumerate() {
            // dont index outside the array!
            if num_leaves_seen == expected_leaves.len() {
                continue;
            }
            assert_eq!(expected_leaves[i + num_leaves_seen], *value);
        }

        let mut postorder_iter_mut = test_tree.dfs_postorder_iter_mut();
        let mut num_leaves_seen = 0;
        for value in postorder_iter_mut.by_ref() {
            // dont index outside the array!
            if num_leaves_seen == expected_leaves.len() {
                continue;
            }
            if *value == expected_leaves[num_leaves_seen] {
                num_leaves_seen += 1;
            }
        }

        let postorder_iter_leaves = postorder_iter_mut.leaves();
        for (i, value) in postorder_iter_leaves.enumerate() {
            assert_eq!(expected_leaves[i + num_leaves_seen], *value);
        }

        let mut postorder = test_tree.clone().dfs_postorder();
        let mut num_leaves_seen = 0;
        for value in postorder.by_ref() {
            // dont index outside the array!
            if num_leaves_seen == expected_leaves.len() {
                continue;
            }
            if value == expected_leaves[num_leaves_seen] {
                num_leaves_seen += 1;
            }
        }

        let postorder_iter_leaves = postorder.leaves();
        for (i, value) in postorder_iter_leaves.enumerate() {
            assert_eq!(expected_leaves[i + num_leaves_seen], value);
        }
    }
}

#[test]
fn bfs_binary_transformation_can_happen_mid_traversal() {
    let expected_bfs = super::bfs::get_expected_order_bfs();
    let expected_leaves = get_expected_order_leaves();
    let mut test_tree = create_binary_tree_for_testing();
    // interrupt traversal at all points.
    for _ in 0..expected_bfs.len() {
        let mut bfs_iter = test_tree.bfs_iter();
        let mut num_leaves_seen = 0;
        for value in bfs_iter.by_ref() {
            if *value == expected_leaves[num_leaves_seen] {
                num_leaves_seen += 1;
            }
        }

        let bfs_iter_leaves = bfs_iter.leaves();
        for (i, value) in bfs_iter_leaves.enumerate() {
            assert_eq!(expected_leaves[i + num_leaves_seen], *value);
        }

        let mut bfs_iter_mut = test_tree.bfs_iter_mut();
        let mut num_leaves_seen = 0;
        for value in bfs_iter_mut.by_ref() {
            if *value == expected_leaves[num_leaves_seen] {
                num_leaves_seen += 1;
            }
        }

        let bfs_iter_mut_leaves = bfs_iter_mut.leaves();
        for (i, value) in bfs_iter_mut_leaves.enumerate() {
            assert_eq!(expected_leaves[i + num_leaves_seen], *value);
        }

        let mut bfs = test_tree.clone().bfs();
        let mut num_leaves_seen = 0;
        for value in bfs.by_ref() {
            if value == expected_leaves[num_leaves_seen] {
                num_leaves_seen += 1;
            }
        }

        let bfs_leaves = bfs.leaves();
        for (i, value) in bfs_leaves.enumerate() {
            assert_eq!(expected_leaves[i + num_leaves_seen], value);
        }
    }
}
