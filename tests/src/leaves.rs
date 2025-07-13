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

        for mut_borrowed_iter in get_mut_borrowed_leaves_iters(&mut test_tree) {
            for (i, value) in mut_borrowed_iter.enumerate() {
                assert_eq!(expected[i], *value);
            }
        }

        for mut_borrowed_iter in get_mut_borrowed_leaves_iters(&mut test_tree) {
            assert_len!(expected.len(), mut_borrowed_iter);
        }

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

fn get_mut_borrowed_leaves_iters<T>(
    test_tree: &mut Tree<T>,
) -> impl Iterator<Item = Box<dyn Iterator<Item = &mut T> + '_>> + '_ {
    // Rust doesn't like this, but we know that only 1 iterator will be accessed at a time
    // and no reallocations will be done as we are doing a readonly test,
    // so we are still within the "safe" rust system with only 1 active mutable reference.
    // This also makes the test much nicer to write.
    unsafe {
        [
            Box::new(
                (*(test_tree as *mut Tree<T>))
                    .dfs_preorder_iter_mut()
                    .leaves(),
            ) as Box<dyn Iterator<Item = &mut T>>,
            Box::new(
                (*(test_tree as *mut Tree<T>))
                    .dfs_postorder_iter_mut()
                    .leaves(),
            ),
            Box::new((*(test_tree as *mut Tree<T>)).bfs_iter_mut().leaves()),
        ]
        .into_iter()
    }
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

    for mut_borrowed_iter in get_mut_borrowed_leaves_binary_iters(&mut test_tree) {
        for (i, value) in mut_borrowed_iter.enumerate() {
            assert_eq!(expected[i], *value);
        }
    }

    for mut_borrowed_iter in get_mut_borrowed_leaves_binary_iters(&mut test_tree) {
        assert_len!(expected.len(), mut_borrowed_iter);
    }

    for owned_iter in get_owned_leaves_binary_iters(test_tree.clone()) {
        for (i, value) in owned_iter.enumerate() {
            assert_eq!(expected[i], value);
        }
    }

    for owned_iter in get_owned_leaves_binary_iters(test_tree) {
        assert_len!(expected.len(), owned_iter);
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

fn get_mut_borrowed_leaves_binary_iters<T>(
    test_tree: &mut BinaryTree<T>,
) -> impl Iterator<Item = Box<dyn Iterator<Item = &mut T> + '_>> {
    unsafe {
        [
            Box::new(
                (*(test_tree as *mut BinaryTree<T>))
                    .dfs_preorder_iter_mut()
                    .leaves(),
            ) as Box<dyn Iterator<Item = &mut T>>,
            Box::new(
                (*(test_tree as *mut BinaryTree<T>))
                    .dfs_inorder_iter_mut()
                    .leaves(),
            ),
            Box::new(
                (*(test_tree as *mut BinaryTree<T>))
                    .dfs_postorder_iter_mut()
                    .leaves(),
            ),
            Box::new((*(test_tree as *mut BinaryTree<T>)).bfs_iter_mut().leaves()),
        ]
        .into_iter()
    }
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
            while let Some(value) = preorder_iter.next() {
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
            while let Some(value) = preorder_iter_mut.next() {
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
            while let Some(value) = preorder.next() {
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
            while let Some(value) = postorder_iter.next() {
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
            while let Some(value) = postorder_iter_mut.next() {
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
            while let Some(value) = postorder.next() {
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
            while let Some(value) = bfs_iter.next() {
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
            while let Some(value) = bfs_iter_mut.next() {
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
            while let Some(value) = bfs.next() {
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
        while let Some(value) = preorder_iter.next() {
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
        while let Some(value) = preorder_iter_mut.next() {
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
        while let Some(value) = preorder.next() {
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
        while let Some(value) = inorder_iter.next() {
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
        while let Some(value) = inorder_iter_mut.next() {
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
        while let Some(value) = inorder.next() {
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
        while let Some(value) = postorder_iter.next() {
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
        while let Some(value) = postorder_iter_mut.next() {
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
        while let Some(value) = postorder.next() {
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
        while let Some(value) = bfs_iter.next() {
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
        while let Some(value) = bfs_iter_mut.next() {
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
        while let Some(value) = bfs.next() {
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
