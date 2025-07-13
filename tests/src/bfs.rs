use super::{
    assert_len, create_binary_tree_for_testing, create_trees_for_testing,
    get_expected_metadata_for_value, get_value_to_path_map, get_value_to_path_map_binary,
};
use streaming_iterator::StreamingIterator;
use tree_iterators_rs::prelude::*;

pub(crate) fn get_expected_order_bfs() -> [usize; 11] {
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
}

#[test]
fn bfs_has_correct_order() {
    let expected = get_expected_order_bfs();
    for mut test_tree in create_trees_for_testing() {
        for (i, value) in test_tree.bfs_iter().enumerate() {
            assert_eq!(expected[i], *value);
        }
        assert_len!(expected.len(), test_tree.bfs_iter());

        for (i, value) in test_tree.bfs_iter_mut().enumerate() {
            assert_eq!(expected[i], *value);
        }
        assert_len!(expected.len(), test_tree.bfs_iter_mut());

        for (i, value) in test_tree.clone().bfs().enumerate() {
            assert_eq!(expected[i], value);
        }
        assert_len!(expected.len(), test_tree.bfs());
    }
}

#[test]
fn bfs_attach_ancestors_works() {
    let expected = get_expected_order_bfs();

    for mut test_tree in create_trees_for_testing() {
        let mut i = 0;
        let mut iter_with_metadata = test_tree.bfs_iter().attach_ancestors();
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
        let mut iter_with_metadata = test_tree.bfs_iter_mut().attach_ancestors();
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
        let mut iter_with_metadata = test_tree.bfs().attach_ancestors();
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
fn bfs_attach_context_works() {
    let expected = get_expected_order_bfs();
    let expected_paths = get_value_to_path_map();

    for mut test_tree in create_trees_for_testing() {
        let mut i = 0;
        let mut iter_with_metadata = test_tree.bfs_iter().attach_context();
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
                value.path()
            );
            i += 1;
        }
        assert_eq!(expected.len(), i);
        drop(iter_with_metadata);

        let mut i = 0;
        let mut iter_with_metadata = test_tree.bfs_iter_mut().attach_context();
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
                value.path()
            );
            i += 1;
        }
        assert_eq!(expected.len(), i);
        drop(iter_with_metadata);

        let mut i = 0;
        let mut iter_with_metadata = test_tree.bfs().attach_context();
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
                value.path()
            );
            i += 1;
        }
        assert_eq!(expected.len(), i);
    }
}

#[test]
fn binary_bfs_has_correct_order() {
    let expected = get_expected_order_bfs();
    let mut test_tree = create_binary_tree_for_testing();

    for (i, value) in test_tree.bfs_iter().enumerate() {
        assert_eq!(expected[i], *value);
    }
    assert_len!(expected.len(), test_tree.bfs_iter());

    for (i, value) in test_tree.bfs_iter_mut().enumerate() {
        assert_eq!(expected[i], *value);
    }
    assert_len!(expected.len(), test_tree.bfs_iter_mut());

    for (i, value) in test_tree.clone().bfs().enumerate() {
        assert_eq!(expected[i], value);
    }
    assert_len!(expected.len(), test_tree.bfs());
}

#[test]
fn binary_bfs_attach_context_works() {
    let expected = get_expected_order_bfs();
    let expected_paths = get_value_to_path_map_binary();

    let mut test_tree = create_binary_tree_for_testing();
    let mut i = 0;
    let mut iter_with_metadata = test_tree.bfs_iter().attach_context();
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
            value.path()
        );
        i += 1;
    }
    assert_eq!(expected.len(), i);
    drop(iter_with_metadata);

    let mut i = 0;
    let mut iter_with_metadata = test_tree.bfs_iter_mut().attach_context();
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
            value.path()
        );
        i += 1;
    }
    assert_eq!(expected.len(), i);
    drop(iter_with_metadata);

    let mut i = 0;
    let mut iter_with_metadata = test_tree.bfs().attach_context();
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
            value.path()
        );
        i += 1;
    }
    assert_eq!(expected.len(), i);
}

#[test]
fn binary_bfs_attach_ancestors_works() {
    let expected = get_expected_order_bfs();

    let mut i = 0;
    let test_tree = create_binary_tree_for_testing();
    let mut iter_with_metadata = test_tree.bfs_iter().attach_ancestors();
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
    let mut iter_with_metadata = test_tree.bfs_iter_mut().attach_ancestors();
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
    let mut iter_with_metadata = create_binary_tree_for_testing().bfs().attach_ancestors();
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
