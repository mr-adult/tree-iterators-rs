use super::{assert_len, create_binary_tree_for_testing, get_expected_metadata_for_value};
use crate::prelude::*;
use crate::tests::get_value_to_path_map_binary;
use streaming_iterator::StreamingIterator;

pub(crate) fn get_expected_order_dfs_inorder() -> [usize; 11] {
    [3, 1, 4, 0, 5, 2, 7, 9, 10, 8, 6]
}

#[test]
fn dfs_inorder_has_correct_order() {
    let expected = get_expected_order_dfs_inorder();

    let mut test_tree = create_binary_tree_for_testing();
    for (i, value) in test_tree.dfs_inorder_iter().enumerate() {
        assert_eq!(expected[i], *value);
    }
    assert_len!(expected.len(), test_tree.dfs_inorder_iter());

    for (i, value) in test_tree.dfs_inorder_iter_mut().enumerate() {
        assert_eq!(expected[i], *value);
    }
    assert_len!(expected.len(), test_tree.dfs_inorder_iter_mut());

    for (i, value) in test_tree.clone().dfs_inorder().enumerate() {
        assert_eq!(expected[i], value);
    }
    assert_len!(expected.len(), test_tree.dfs_inorder());
}

#[test]
fn dfs_inorder_attach_ancestors_works() {
    let expected = get_expected_order_dfs_inorder();

    let mut i = 0;
    let test_tree = create_binary_tree_for_testing();
    let mut iter_with_metadata = test_tree.dfs_inorder_iter().attach_ancestors();
    while let Some(value) = iter_with_metadata.next() {
        assert_eq!(expected[i], *value[value.len() - 1]);
        let expected = get_expected_metadata_for_value(*value[value.len() - 1]);
        for j in 0..expected.len() {
            assert_eq!(expected[j], *value[j]);
        }
        i += 1;
    }
    assert_eq!(expected.len(), i);

    let mut i = 0;
    let mut test_tree = create_binary_tree_for_testing();
    let mut iter_mut_with_metadata = test_tree.dfs_inorder_iter_mut().attach_ancestors();
    while let Some(value) = iter_mut_with_metadata.next() {
        assert_eq!(expected[i], *value[value.len() - 1]);
        let expected = get_expected_metadata_for_value(*value[value.len() - 1]);
        for j in 0..expected.len() {
            assert_eq!(expected[j], *value[j]);
        }
        i += 1;
    }
    assert_eq!(expected.len(), i);

    let mut i = 0;
    let test_tree = create_binary_tree_for_testing();
    let mut iter_with_metadata = test_tree.dfs_inorder().attach_ancestors();
    while let Some(value) = iter_with_metadata.next() {
        assert_eq!(expected[i], value[value.len() - 1]);
        let expected = get_expected_metadata_for_value(value[value.len() - 1]);
        for j in 0..expected.len() {
            assert_eq!(expected[j], value[j]);
        }
        i += 1;
    }
    assert_eq!(expected.len(), i);
}

#[test]
fn binary_dfs_inorder_attach_context_works() {
    let expected = get_expected_order_dfs_inorder();
    let expected_paths = get_value_to_path_map_binary();

    let mut test_tree = create_binary_tree_for_testing();
    let mut i = 0;
    let mut iter_with_metadata = test_tree.dfs_inorder_iter().attach_context();
    while let Some(value) = iter_with_metadata.next() {
        assert_eq!(expected[i], *value.ancestors()[value.ancestors().len() - 1]);
        let expected =
            get_expected_metadata_for_value(*value.ancestors()[value.ancestors().len() - 1]);
        for j in 0..expected.len() {
            assert_eq!(expected[j], *value.ancestors()[j]);
        }
        assert_eq!(
            *expected_paths
                .get(value.ancestors().last().unwrap())
                .unwrap(),
            value.path
        );
        i += 1;
    }
    assert_eq!(expected.len(), i);
    drop(iter_with_metadata);

    let mut i = 0;
    let mut iter_with_metadata = test_tree.dfs_inorder_iter_mut().attach_context();
    while let Some(value) = iter_with_metadata.next() {
        assert_eq!(expected[i], *value.ancestors()[value.ancestors().len() - 1]);
        let expected =
            get_expected_metadata_for_value(*value.ancestors()[value.ancestors().len() - 1]);
        for j in 0..expected.len() {
            assert_eq!(expected[j], *value.ancestors()[j]);
        }
        assert_eq!(
            *expected_paths
                .get(value.ancestors().last().unwrap())
                .unwrap(),
            value.path
        );
        i += 1;
    }
    assert_eq!(expected.len(), i);
    drop(iter_with_metadata);

    let mut i = 0;
    let mut iter_with_metadata = test_tree.dfs_inorder().attach_context();
    while let Some(value) = iter_with_metadata.next() {
        assert_eq!(expected[i], value.ancestors()[value.ancestors().len() - 1]);
        let expected =
            get_expected_metadata_for_value(value.ancestors()[value.ancestors().len() - 1]);
        for j in 0..expected.len() {
            assert_eq!(expected[j], value.ancestors()[j]);
        }
        assert_eq!(
            *expected_paths
                .get(value.ancestors().last().unwrap())
                .unwrap(),
            value.path
        );
        i += 1;
    }
    assert_eq!(expected.len(), i);
}
