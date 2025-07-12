use super::{
    assert_len, create_binary_tree_for_testing, create_trees_for_testing,
    get_expected_metadata_for_value, get_value_to_path_map,
};
use crate::prelude::*;
use crate::tests::get_value_to_path_map_binary;
use streaming_iterator::StreamingIterator;

pub(crate) fn get_expected_order_dfs_postorder() -> [usize; 11] {
    [3, 4, 1, 5, 10, 9, 8, 7, 6, 2, 0]
}

#[test]
fn dfs_postorder_has_correct_order() {
    let expected = get_expected_order_dfs_postorder();
    for mut test_tree in create_trees_for_testing() {
        for (i, value) in test_tree.dfs_postorder_iter().enumerate() {
            assert_eq!(expected[i], *value);
        }
        assert_len!(expected.len(), test_tree.dfs_postorder_iter());

        for (i, value) in test_tree.dfs_postorder_iter_mut().enumerate() {
            assert_eq!(expected[i], *value);
        }
        assert_len!(expected.len(), test_tree.dfs_postorder_iter_mut());

        for (i, value) in test_tree.clone().dfs_postorder().enumerate() {
            assert_eq!(expected[i], value);
        }
        assert_len!(expected.len(), test_tree.dfs_postorder());
    }
}

#[test]
fn binary_dfs_postorder_has_correct_order() {
    let expected = get_expected_order_dfs_postorder();
    let mut test_tree = create_binary_tree_for_testing();

    for (i, value) in test_tree.dfs_postorder_iter().enumerate() {
        assert_eq!(expected[i], *value);
    }
    assert_len!(expected.len(), test_tree.dfs_postorder_iter());

    for (i, value) in test_tree.dfs_postorder_iter_mut().enumerate() {
        assert_eq!(expected[i], *value);
    }
    assert_len!(expected.len(), test_tree.dfs_postorder_iter_mut());

    for (i, value) in test_tree.clone().dfs_postorder().enumerate() {
        assert_eq!(expected[i], value);
    }
    assert_len!(expected.len(), test_tree.dfs_postorder());
}

#[test]
fn dfs_postorder_attach_ancestors_works() {
    let expected = get_expected_order_dfs_postorder();

    for mut test_tree in create_trees_for_testing() {
        let mut i = 0;
        let mut iter_with_metadata = test_tree.dfs_postorder_iter().attach_ancestors();
        while let Some(value) = iter_with_metadata.next() {
            assert_eq!(expected[i], *value[value.len() - 1]);
            let expected = get_expected_metadata_for_value(*value[value.len() - 1]);
            for j in 0..expected.len() {
                assert_eq!(expected[j], *value[j]);
            }
            i += 1;
        }
        assert_eq!(expected.len(), i);
        drop(iter_with_metadata);

        let mut i = 0;
        let mut iter_with_metadata = test_tree.dfs_postorder_iter_mut().attach_ancestors();
        while let Some(value) = iter_with_metadata.next() {
            assert_eq!(expected[i], *value[value.len() - 1]);
            let expected = get_expected_metadata_for_value(*value[value.len() - 1]);
            for j in 0..expected.len() {
                assert_eq!(expected[j], *value[j]);
            }
            i += 1;
        }
        assert_eq!(expected.len(), i);
        drop(iter_with_metadata);

        let mut i = 0;
        let mut iter_with_metadata = test_tree.dfs_postorder().attach_ancestors();
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
}

#[test]
fn binary_dfs_postorder_attach_context_works() {
    let expected = get_expected_order_dfs_postorder();
    let expected_paths = get_value_to_path_map_binary();

    let mut test_tree = create_binary_tree_for_testing();
    let mut i = 0;
    let mut iter_with_metadata = test_tree.dfs_postorder_iter().attach_context();
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
    let mut iter_with_metadata = test_tree.dfs_postorder_iter_mut().attach_context();
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
    let mut iter_with_metadata = test_tree.dfs_postorder().attach_context();
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

#[test]
fn dfs_postorder_attach_context_works() {
    let expected = get_expected_order_dfs_postorder();
    let expected_paths = get_value_to_path_map();

    for mut test_tree in create_trees_for_testing() {
        let mut i = 0;
        let mut iter_with_metadata = test_tree.dfs_postorder_iter().attach_context();
        while let Some(value) = iter_with_metadata.next() {
            assert_eq!(expected[i], *value.ancestors()[value.ancestors().len() - 1]);
            let expected =
                get_expected_metadata_for_value(*value.ancestors()[value.ancestors().len() - 1]);
            for j in 0..expected.len() {
                assert_eq!(expected[j], *value.ancestors()[j]);
            }
            assert_eq!(
                *expected_paths
                    .get(*value.ancestors().last().unwrap())
                    .unwrap(),
                value.path
            );
            i += 1;
        }
        assert_eq!(expected.len(), i);
        drop(iter_with_metadata);

        let mut i = 0;
        let mut iter_with_metadata = test_tree.dfs_postorder_iter_mut().attach_context();
        while let Some(value) = iter_with_metadata.next() {
            assert_eq!(expected[i], *value.ancestors()[value.ancestors().len() - 1]);
            let expected =
                get_expected_metadata_for_value(*value.ancestors()[value.ancestors().len() - 1]);
            for j in 0..expected.len() {
                assert_eq!(expected[j], *value.ancestors()[j]);
            }
            assert_eq!(
                *expected_paths
                    .get(*value.ancestors().last().unwrap())
                    .unwrap(),
                value.path
            );
            i += 1;
        }
        assert_eq!(expected.len(), i);
        drop(iter_with_metadata);

        let mut i = 0;
        let mut iter_with_metadata = test_tree.dfs_postorder().attach_context();
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
}

#[test]
fn binary_dfs_postorder_attach_ancestors_works() {
    let expected = get_expected_order_dfs_postorder();

    let mut i = 0;
    let test_tree = create_binary_tree_for_testing();
    let mut iter_with_metadata = test_tree.dfs_postorder_iter().attach_ancestors();
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
    let mut iter_with_metadata = test_tree.dfs_postorder_iter_mut().attach_ancestors();
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
    let mut iter_with_metadata = create_binary_tree_for_testing()
        .dfs_postorder()
        .attach_ancestors();
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
