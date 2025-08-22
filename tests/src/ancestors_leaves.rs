use crate::create_trees_for_testing;
use streaming_iterator::StreamingIterator;

#[cfg(test)]
use super::assert_len;
use super::create_binary_tree_for_testing;
use tree_iterators_rs::prelude::*;

fn get_expected_order_leaves() -> [Vec<usize>; 4] {
    [
        vec![0, 1, 3],
        vec![0, 1, 4],
        vec![0, 2, 5],
        vec![0, 2, 6, 7, 8, 9, 10],
    ]
}

#[test]
fn leaves_has_correct_order() {
    let expected = get_expected_order_leaves();
    for mut test_tree in create_trees_for_testing() {
        for mut borrowed_iter in get_borrowed_leaves_iters(&test_tree) {
            let mut i = 0;
            while let Some(value) = borrowed_iter.next() {
                assert!(expected[i].iter().eq(value.iter().copied()));
                i += 1;
            }
        }

        let mut i = 0;
        for borrowed_iter in get_borrowed_leaves_iters(&test_tree) {
            assert_len!(
                expected.len(),
                borrowed_iter,
                format!("Failure at index {}", i.to_string())
            );
            i += 1;
        }

        for mut mut_borrowed_iter in get_mut_borrowed_leaves_iters(&mut test_tree) {
            let mut i = 0;
            while let Some(value) = mut_borrowed_iter.next() {
                assert!(expected[i].iter().eq(value.iter().map(|val| &**val)));
                i += 1;
            }
        }

        for mut_borrowed_iter in get_mut_borrowed_leaves_iters(&mut test_tree) {
            assert_len!(expected.len(), mut_borrowed_iter);
        }

        for mut owned_iter in get_owned_leaves_iters(test_tree.clone()) {
            let mut i = 0;
            while let Some(value) = owned_iter.next() {
                assert!(expected[i].iter().eq(value.iter()));
                i += 1;
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

    for mut borrowed_iter in get_borrowed_leaves_iters(&tree) {
        while let Some(item) = borrowed_iter.next() {
            assert_eq!(&[&0], item);
        }
    }

    for mut_borrowed_iter in get_mut_borrowed_leaves_iters(&mut tree) {
        assert_len!(1, mut_borrowed_iter);
    }

    for mut mut_borrowed_iter in get_mut_borrowed_leaves_iters(&mut tree) {
        while let Some(item) = mut_borrowed_iter.next() {
            assert_eq!(&[&mut 0], item);
        }
    }

    for owned_iter in get_owned_leaves_iters(tree.clone()) {
        assert_len!(1, owned_iter);
    }

    for mut owned_iter in get_owned_leaves_iters(tree.clone()) {
        while let Some(item) = owned_iter.next() {
            assert_eq!(&[0], item);
        }
    }
}

fn get_borrowed_leaves_iters<T>(
    test_tree: &Tree<T>,
) -> impl Iterator<Item = Box<dyn StreamingIterator<Item = [&T]> + '_>> + '_ {
    [
        Box::new(test_tree.dfs_preorder_iter().attach_ancestors().leaves())
            as Box<dyn StreamingIterator<Item = [&T]>>,
        Box::new(test_tree.dfs_postorder_iter().attach_ancestors().leaves()),
        Box::new(test_tree.bfs_iter().attach_ancestors().leaves()),
    ]
    .into_iter()
}

fn get_mut_borrowed_leaves_iters<T>(
    test_tree: &mut Tree<T>,
) -> impl Iterator<Item = Box<dyn StreamingIterator<Item = [&mut T]> + '_>> + '_ {
    // Rust doesn't like this, but we know that only 1 iterator will be accessed at a time
    // and no reallocations will be done as we are doing a readonly test,
    // so we are still within the "safe" rust system with only 1 active mutable reference.
    // This also makes the test much nicer to write.
    unsafe {
        [
            Box::new(
                (*(test_tree as *mut Tree<T>))
                    .dfs_preorder_iter_mut()
                    .attach_ancestors()
                    .leaves(),
            ) as Box<dyn StreamingIterator<Item = [&mut T]>>,
            Box::new(
                (*(test_tree as *mut Tree<T>))
                    .dfs_postorder_iter_mut()
                    .attach_ancestors()
                    .leaves(),
            ),
            Box::new(
                (*(test_tree as *mut Tree<T>))
                    .bfs_iter_mut()
                    .attach_ancestors()
                    .leaves(),
            ),
        ]
        .into_iter()
    }
}

fn get_owned_leaves_iters<T: Clone + 'static>(
    test_tree: Tree<T>,
) -> [Box<dyn StreamingIterator<Item = [T]>>; 3] {
    [
        Box::new(test_tree.clone().dfs_preorder().attach_ancestors().leaves())
            as Box<dyn StreamingIterator<Item = [T]>>,
        Box::new(
            test_tree
                .clone()
                .dfs_postorder()
                .attach_ancestors()
                .leaves(),
        ),
        Box::new(test_tree.clone().bfs().attach_ancestors().leaves()),
    ]
}

#[test]
fn binary_leaves_has_correct_order() {
    let expected = get_expected_order_leaves();
    let mut test_tree = create_binary_tree_for_testing();

    for mut borrowed_iter in get_borrowed_leaves_binary_iters(&test_tree) {
        let mut i = 0;
        while let Some(value) = borrowed_iter.next() {
            assert!(expected[i].iter().eq(value.iter().copied()));
            i += 1;
        }
    }

    let mut results = Vec::new();
    for borrowed_iter in get_borrowed_leaves_binary_iters(&test_tree) {
        let mut count = 0;
        borrowed_iter.for_each(|_| count += 1);
        results.push(count);
    }

    let mut i = 0;
    for borrowed_iter in get_borrowed_leaves_binary_iters(&test_tree) {
        assert_len!(
            expected.len(),
            borrowed_iter,
            format!("Failure at index {}", i.to_string())
        );
        i += 1;
    }

    for mut mut_borrowed_iter in get_mut_borrowed_leaves_binary_iters(&mut test_tree) {
        let mut i = 0;
        while let Some(value) = mut_borrowed_iter.next() {
            assert!(expected[i].iter().eq(value.iter().map(|val| &**val)));
            i += 1;
        }
    }

    for mut_borrowed_iter in get_mut_borrowed_leaves_binary_iters(&mut test_tree) {
        assert_len!(expected.len(), mut_borrowed_iter);
    }

    for mut owned_iter in get_owned_leaves_binary_iters(test_tree.clone()) {
        let mut i = 0;
        while let Some(value) = owned_iter.next() {
            assert!(expected[i].iter().eq(value.iter()));
            i += 1;
        }
    }

    for owned_iter in get_owned_leaves_binary_iters(test_tree) {
        assert_len!(expected.len(), owned_iter);
    }
}

#[test]
fn root_only_binary_leaves_has_correct_order() {
    let mut tree = BinaryTree {
        value: 0,
        left: None,
        right: None,
    };

    for borrowed_iter in get_borrowed_leaves_binary_iters(&tree) {
        assert_len!(1, borrowed_iter);
    }

    for mut borrowed_iter in get_borrowed_leaves_binary_iters(&tree) {
        while let Some(item) = borrowed_iter.next() {
            assert_eq!(&[&0], item);
        }
    }

    for mut_borrowed_iter in get_mut_borrowed_leaves_binary_iters(&mut tree) {
        assert_len!(1, mut_borrowed_iter);
    }

    for mut mut_borrowed_iter in get_mut_borrowed_leaves_binary_iters(&mut tree) {
        while let Some(item) = mut_borrowed_iter.next() {
            assert_eq!(&[&mut 0], item);
        }
    }

    for owned_iter in get_owned_leaves_binary_iters(tree.clone()) {
        assert_len!(1, owned_iter);
    }

    for mut owned_iter in get_owned_leaves_binary_iters(tree.clone()) {
        while let Some(item) = owned_iter.next() {
            assert_eq!(&[0], item);
        }
    }
}

fn get_borrowed_leaves_binary_iters<T>(
    test_tree: &BinaryTree<T>,
) -> [Box<dyn StreamingIterator<Item = [&T]> + '_>; 4] {
    [
        Box::new(test_tree.dfs_preorder_iter().attach_ancestors().leaves())
            as Box<dyn StreamingIterator<Item = [&T]>>,
        Box::new(test_tree.dfs_inorder_iter().attach_ancestors().leaves()),
        Box::new(test_tree.dfs_postorder_iter().attach_ancestors().leaves()),
        Box::new(test_tree.bfs_iter().attach_ancestors().leaves()),
    ]
}

fn get_mut_borrowed_leaves_binary_iters<T>(
    test_tree: &mut BinaryTree<T>,
) -> impl Iterator<Item = Box<dyn StreamingIterator<Item = [&mut T]> + '_>> {
    unsafe {
        [
            Box::new(
                (*(test_tree as *mut BinaryTree<T>))
                    .dfs_preorder_iter_mut()
                    .attach_ancestors()
                    .leaves(),
            ) as Box<dyn StreamingIterator<Item = [&mut T]>>,
            Box::new(
                (*(test_tree as *mut BinaryTree<T>))
                    .dfs_inorder_iter_mut()
                    .attach_ancestors()
                    .leaves(),
            ),
            Box::new(
                (*(test_tree as *mut BinaryTree<T>))
                    .dfs_postorder_iter_mut()
                    .attach_ancestors()
                    .leaves(),
            ),
            Box::new(
                (*(test_tree as *mut BinaryTree<T>))
                    .bfs_iter_mut()
                    .attach_ancestors()
                    .leaves(),
            ),
        ]
        .into_iter()
    }
}

fn get_owned_leaves_binary_iters<T: Clone + 'static>(
    test_tree: BinaryTree<T>,
) -> [Box<dyn StreamingIterator<Item = [T]>>; 4] {
    [
        Box::new(test_tree.clone().dfs_preorder().attach_ancestors().leaves())
            as Box<dyn StreamingIterator<Item = [T]>>,
        Box::new(test_tree.clone().dfs_inorder().attach_ancestors().leaves()),
        Box::new(
            test_tree
                .clone()
                .dfs_postorder()
                .attach_ancestors()
                .leaves(),
        ),
        Box::new(test_tree.clone().bfs().attach_ancestors().leaves()),
    ]
}

#[test]
fn dfs_preorder_transformation_can_happen_mid_traversal() {
    let expected_dfs_preorder = super::dfs_preorder::get_expected_order_dfs_preorder();
    let expected_leaves = get_expected_order_leaves();
    for mut test_tree in create_trees_for_testing() {
        // interrupt traversal at all points.
        for _ in 0..expected_dfs_preorder.len() {
            let mut preorder_iter = test_tree.dfs_preorder_iter().attach_ancestors();
            let mut num_leaves_seen = 0;
            while let Some(value) = preorder_iter.next() {
                if *value[value.len() - 1]
                    == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
                {
                    num_leaves_seen += 1;
                }
            }

            let mut preorder_iter_leaves = preorder_iter.leaves();
            let mut i = 0;
            while let Some(value) = preorder_iter_leaves.next() {
                assert!(expected_leaves[i].iter().eq(value.iter().copied()));
                i += 1;
            }
            drop(preorder_iter_leaves);

            let mut preorder_iter_mut = test_tree.dfs_preorder_iter_mut().attach_ancestors();
            let mut num_leaves_seen = 0;
            while let Some(value) = preorder_iter_mut.next() {
                if *value[value.len() - 1]
                    == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
                {
                    num_leaves_seen += 1;
                }
            }

            let mut preorder_iter_leaves = preorder_iter_mut.leaves();
            let mut i = 0;
            while let Some(value) = preorder_iter_leaves.next() {
                assert!(expected_leaves[i].iter().eq(value.iter().map(|val| &**val)));
                i += 1;
            }
            drop(preorder_iter_leaves);

            let mut preorder = test_tree.clone().dfs_preorder().attach_ancestors();
            let mut num_leaves_seen = 0;
            while let Some(value) = preorder.next() {
                if value[value.len() - 1]
                    == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
                {
                    num_leaves_seen += 1;
                }
            }

            let mut preorder_iter_leaves = preorder.leaves();
            let mut i = 0;
            while let Some(value) = preorder_iter_leaves.next() {
                assert!(expected_leaves[i].iter().eq(value.iter()));
                i += 1;
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
            let mut postorder_iter = test_tree.dfs_postorder_iter().attach_ancestors();
            let mut num_leaves_seen = 0;
            while let Some(value) = postorder_iter.next() {
                // dont index outside the array!
                if num_leaves_seen == expected_leaves.len() {
                    continue;
                }
                if *value[value.len() - 1]
                    == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
                {
                    num_leaves_seen += 1;
                }
            }

            let mut postorder_iter_leaves = postorder_iter.leaves();
            let mut i = 0;
            while let Some(value) = postorder_iter_leaves.next() {
                assert!(expected_leaves[i].iter().eq(value.iter().copied()));
                i += 1;
            }
            drop(postorder_iter_leaves);

            let mut postorder_iter_mut = test_tree.dfs_postorder_iter_mut().attach_ancestors();
            let mut num_leaves_seen = 0;
            while let Some(value) = postorder_iter_mut.next() {
                // dont index outside the array!
                if num_leaves_seen == expected_leaves.len() {
                    continue;
                }
                if *value[value.len() - 1]
                    == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
                {
                    num_leaves_seen += 1;
                }
            }

            let mut postorder_iter_leaves = postorder_iter_mut.leaves();
            let mut i = 0;
            while let Some(value) = postorder_iter_leaves.next() {
                assert!(expected_leaves[i].iter().eq(value.iter().map(|val| &**val)));
                i += 1;
            }
            drop(postorder_iter_leaves);

            let mut postorder = test_tree.clone().dfs_postorder().attach_ancestors();
            let mut num_leaves_seen = 0;
            while let Some(value) = postorder.next() {
                // dont index outside the array!
                if num_leaves_seen == expected_leaves.len() {
                    continue;
                }
                if value[value.len() - 1]
                    == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
                {
                    num_leaves_seen += 1;
                }
            }

            let mut postorder_iter_leaves = postorder.leaves();
            let mut i = 0;
            while let Some(value) = postorder_iter_leaves.next() {
                assert!(expected_leaves[i].iter().eq(value.iter()));
                i += 1;
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
            let mut bfs_iter = test_tree.bfs_iter().attach_ancestors();
            let mut num_leaves_seen = 0;
            while let Some(value) = bfs_iter.next() {
                if *value[value.len() - 1]
                    == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
                {
                    num_leaves_seen += 1;
                }
            }

            let mut bfs_iter_leaves = bfs_iter.leaves();
            let mut i = 0;
            while let Some(value) = bfs_iter_leaves.next() {
                assert!(expected_leaves[i].iter().eq(value.iter().copied()));
                i += 1;
            }
            drop(bfs_iter_leaves);

            let mut bfs_iter_mut = test_tree.bfs_iter_mut().attach_ancestors();
            let mut num_leaves_seen = 0;
            while let Some(value) = bfs_iter_mut.next() {
                if *value[value.len() - 1]
                    == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
                {
                    num_leaves_seen += 1;
                }
            }

            let mut bfs_iter_mut_leaves = bfs_iter_mut.leaves();
            let mut i = 0;
            while let Some(value) = bfs_iter_mut_leaves.next() {
                assert!(expected_leaves[i].iter().eq(value.iter().map(|val| &**val)));
                i += 1;
            }
            drop(bfs_iter_mut_leaves);

            let mut bfs = test_tree.clone().bfs().attach_ancestors();
            let mut num_leaves_seen = 0;
            while let Some(value) = bfs.next() {
                if value[value.len() - 1]
                    == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
                {
                    num_leaves_seen += 1;
                }
            }

            let mut bfs_leaves = bfs.leaves();
            let mut i = 0;
            while let Some(value) = bfs_leaves.next() {
                assert!(expected_leaves[i].iter().eq(value.iter()));
                i += 1;
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
        let mut preorder_iter = test_tree.dfs_preorder_iter().attach_ancestors();
        let mut num_leaves_seen = 0;
        while let Some(value) = preorder_iter.next() {
            if *value[value.len() - 1]
                == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
            {
                num_leaves_seen += 1;
            }
        }

        let mut preorder_iter_leaves = preorder_iter.leaves();
        let mut i = 0;
        while let Some(value) = preorder_iter_leaves.next() {
            assert!(expected_leaves[i].iter().eq(value.iter().copied()));
            i += 1;
        }
        drop(preorder_iter_leaves);

        let mut preorder_iter_mut = test_tree.dfs_preorder_iter_mut().attach_ancestors();
        let mut num_leaves_seen = 0;
        while let Some(value) = preorder_iter_mut.next() {
            if *value[value.len() - 1]
                == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
            {
                num_leaves_seen += 1;
            }
        }

        let mut preorder_iter_leaves = preorder_iter_mut.leaves();
        let mut i = 0;
        while let Some(value) = preorder_iter_leaves.next() {
            assert!(expected_leaves[i].iter().eq(value.iter().map(|val| &**val)));
            i += 1;
        }
        drop(preorder_iter_leaves);

        let mut preorder = test_tree.clone().dfs_preorder().attach_ancestors();
        let mut num_leaves_seen = 0;
        while let Some(value) = preorder.next() {
            if value[value.len() - 1]
                == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
            {
                num_leaves_seen += 1;
            }
        }

        let mut preorder_iter_leaves = preorder.leaves();
        let mut i = 0;
        while let Some(value) = preorder_iter_leaves.next() {
            assert!(expected_leaves[i].iter().eq(value.iter()));
            i += 1;
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
        let mut inorder_iter = test_tree.dfs_inorder_iter().attach_ancestors();
        let mut num_leaves_seen = 0;
        while let Some(value) = inorder_iter.next() {
            // dont index outside the array!
            if num_leaves_seen == expected_leaves.len() {
                continue;
            }
            if *value[value.len() - 1]
                == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
            {
                num_leaves_seen += 1;
            }
        }

        let mut inorder_iter_leaves = inorder_iter.leaves();
        let mut i = 0;
        while let Some(value) = inorder_iter_leaves.next() {
            assert!(expected_leaves[i].iter().eq(value.iter().copied()));
            i += 1;
        }
        drop(inorder_iter_leaves);

        let mut inorder_iter_mut = test_tree.dfs_inorder_iter_mut().attach_ancestors();
        let mut num_leaves_seen = 0;
        while let Some(value) = inorder_iter_mut.next() {
            // dont index outside the array!
            if num_leaves_seen == expected_leaves.len() {
                continue;
            }
            if *value[value.len() - 1]
                == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
            {
                num_leaves_seen += 1;
            }
        }

        let mut inorder_iter_leaves = inorder_iter_mut.leaves();
        let mut i = 0;
        while let Some(value) = inorder_iter_leaves.next() {
            assert!(expected_leaves[i].iter().eq(value.iter().map(|val| &**val)));
            i += 1;
        }
        drop(inorder_iter_leaves);

        let mut inorder = test_tree.clone().dfs_inorder().attach_ancestors();
        let mut num_leaves_seen = 0;
        while let Some(value) = inorder.next() {
            // dont index outside the array!
            if num_leaves_seen == expected_leaves.len() {
                continue;
            }
            if value[value.len() - 1]
                == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
            {
                num_leaves_seen += 1;
            }
        }

        let mut inorder_iter_leaves = inorder.leaves();
        let mut i = 0;
        while let Some(value) = inorder_iter_leaves.next() {
            assert!(expected_leaves[i].iter().eq(value.iter()));
            i += 1;
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
        let mut postorder_iter = test_tree.dfs_postorder_iter().attach_ancestors();
        let mut num_leaves_seen = 0;
        while let Some(value) = postorder_iter.next() {
            // dont index outside the array!
            if num_leaves_seen == expected_leaves.len() {
                continue;
            }
            if *value[value.len() - 1]
                == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
            {
                num_leaves_seen += 1;
            }
        }

        let mut postorder_iter_leaves = postorder_iter.leaves();
        let mut i = 0;
        while let Some(value) = postorder_iter_leaves.next() {
            // dont index outside the array!
            if num_leaves_seen == expected_leaves.len() {
                continue;
            }
            assert!(expected_leaves[i].iter().eq(value.iter().copied()));
            i += 1;
        }
        drop(postorder_iter_leaves);

        let mut postorder_iter_mut = test_tree.dfs_postorder_iter_mut().attach_ancestors();
        let mut num_leaves_seen = 0;
        while let Some(value) = postorder_iter_mut.next() {
            // dont index outside the array!
            if num_leaves_seen == expected_leaves.len() {
                continue;
            }
            if *value[value.len() - 1]
                == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
            {
                num_leaves_seen += 1;
            }
        }

        let mut postorder_iter_leaves = postorder_iter_mut.leaves();
        let mut i = 0;
        while let Some(value) = postorder_iter_leaves.next() {
            assert!(expected_leaves[i].iter().eq(value.iter().map(|val| &**val)));
            i += 1;
        }
        drop(postorder_iter_leaves);

        let mut postorder = test_tree.clone().dfs_postorder().attach_ancestors();
        let mut num_leaves_seen = 0;
        while let Some(value) = postorder.next() {
            // dont index outside the array!
            if num_leaves_seen == expected_leaves.len() {
                continue;
            }
            if value[value.len() - 1]
                == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
            {
                num_leaves_seen += 1;
            }
        }

        let mut postorder_iter_leaves = postorder.leaves();
        let mut i = 0;
        while let Some(value) = postorder_iter_leaves.next() {
            assert!(expected_leaves[i].iter().eq(value.iter()));
            i += 1;
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
        let mut bfs_iter = test_tree.bfs_iter().attach_ancestors();
        let mut num_leaves_seen = 0;
        while let Some(value) = bfs_iter.next() {
            if *value[value.len() - 1]
                == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
            {
                num_leaves_seen += 1;
            }
        }

        let mut bfs_iter_leaves = bfs_iter.leaves();
        let mut i = 0;
        while let Some(value) = bfs_iter_leaves.next() {
            assert!(expected_leaves[i].iter().eq(value.iter().copied()));
            i += 1;
        }
        drop(bfs_iter_leaves);

        let mut bfs_iter_mut = test_tree.bfs_iter_mut().attach_ancestors();
        let mut num_leaves_seen = 0;
        while let Some(value) = bfs_iter_mut.next() {
            if *value[value.len() - 1]
                == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
            {
                num_leaves_seen += 1;
            }
        }

        let mut bfs_iter_mut_leaves = bfs_iter_mut.leaves();
        let mut i = 0;
        while let Some(value) = bfs_iter_mut_leaves.next() {
            assert!(expected_leaves[i].iter().eq(value.iter().map(|val| &**val)));
            i += 1;
        }
        drop(bfs_iter_mut_leaves);

        let mut bfs = test_tree.clone().bfs().attach_ancestors();
        let mut num_leaves_seen = 0;
        while let Some(value) = bfs.next() {
            if value[value.len() - 1]
                == expected_leaves[num_leaves_seen][expected_leaves[num_leaves_seen].len() - 1]
            {
                num_leaves_seen += 1;
            }
        }

        let mut bfs_leaves = bfs.leaves();
        let mut i = 0;
        while let Some(value) = bfs_leaves.next() {
            assert!(expected_leaves[i].iter().eq(value.iter()));
            i += 1;
        }
    }
}
