use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use super::bfs_iterators::{
    owned::OwnedBFSIterator,
    mut_borrow::MutBorrowedBFSIterator,
    borrow::BorrowedBFSIterator,
};
use super::dfs_preorder_iterators::{
    owned::OwnedDFSPreorderIterator,
    mut_borrow::MutBorrowedDFSPreorderIterator,
    borrow::BorrowedDFSPreorderIterator,
};
use super::dfs_postorder_iterators::{
    owned::OwnedDFSPostorderIterator,
    mut_borrow::MutBorrowedDFSPostorderIterator,
    borrow::BorrowedDFSPostorderIterator,
};

/// A default implemenation of a tree node. This struct 
/// provides a series of tree traversal utilities to allow 
/// you to easily work with and modify trees.
#[derive(Clone, Debug)]
pub struct TreeNode<T> {
    /// This node's value
    pub value: T,
    /// The children of the current node.
    pub children: Option<Vec<TreeNode<T>>>
}

pub trait OwnedTreeNode 
    where Self: Sized {
    
    /// The value of each node in the tree.
    type OwnedValue: Sized;
    /// The type of iterator that can be used to iterate over each node's children 
    /// collection.
    type OwnedChildren: Iterator<Item = Self>;

    /// This method gets the value and children from this node, consuming it 
    /// in the process. The other methods of this trait assume that the 'Children' 
    /// list does not contain any circular references. If they do, it will create
    /// an infinite loop.
    fn get_value_and_children(self) -> (Self::OwnedValue, Option<Self::OwnedChildren>);

    /// This method retrieves an iterator that can be used to perform
    /// Breadth First (Queue - specifically VecDeque-based) searches of a tree. If performance is 
    /// not a serious concern, a Breadth First (iterative deepening) search
    /// (referred to as BFS in this library) should be preferred to make
    /// debugging easier.
    /// 
    /// A Breadth First Search (BFS) is defined as:
    /// 
    /// A tree traversal that involves breadth-first searching a tree 
    /// from the top down. Given a tree of the following shape, this 
    /// traversal type would traverse the elements in the order 
    /// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10.
    /// 
    /// In this traversal, we scan each level of the tree from left to
    /// right before going down to the next level.
    /// -        0
    /// -       / \
    /// -      1   2
    /// -     / \ / \
    /// -    3  4 5  6
    /// -           /
    /// -          7
    /// -           \
    /// -            8
    /// -           /
    /// -          9
    /// -           \
    /// -           10
    /// 
    fn bfs(self) -> OwnedBFSIterator<Self> {
        OwnedBFSIterator::new(self)
    }

   /// This method retrieves an iterator that can be used to perform
   /// Depth First Preorder searches of a tree.
   /// 
   /// A Depth First Preorder search is defined as:
   /// 
   /// A tree traversal that involves depth-first searching a tree 
   /// from the top down. Given a tree of the following shape, this 
   /// traversal type would traverse the elements in the order
   /// 0, 1, 3, 4, 2, 5, 6, 7, 8, 9, 10.
   /// 
   /// In this traversal, each node will only be traversed before any
   /// of its children have been traversed.
   /// -        0
   /// -       / \
   /// -      1   2
   /// -     / \ / \
   /// -    3  4 5  6
   /// -           /
   /// -          7
   /// -           \
   /// -            8
   /// -           /
   /// -          9
   /// -           \
   /// -           10
   /// 
    fn dfs_preorder(self) -> OwnedDFSPreorderIterator<Self> {
        OwnedDFSPreorderIterator::new(self)
    }

    /// This method retrieves an iterable that can be used to perform
    /// Depth First Postorder searches of a tree.
    /// 
    /// A Depth First Postorder search (referred to as DFS Postorder) 
    /// is defined as:
    /// 
    /// A tree traversal that involves depth-first searching a tree 
    /// from the bottom up. Given a tree of the following shape, this 
    /// traversal type would traverse the elements in the order 
    /// 3, 4, 1, 5, 10, 9, 8, 7, 6, 2, 0.
    /// 
    /// In this traversal, each node will only be traversed after all
    /// of its children have been traversed.
    /// -        0
    /// -       / \
    /// -      1   2
    /// -     / \ / \
    /// -    3  4 5  6
    /// -           /
    /// -          7
    /// -           \
    /// -            8
    /// -           /
    /// -          9
    /// -           \
    /// -           10
    /// 
    /// This traversal type guarantees that getChildren() will only be 
    /// called once per node of the tree.
    ///
    fn dfs_postorder(self) -> OwnedDFSPostorderIterator<Self> {
        OwnedDFSPostorderIterator::new(self)
    }
}

pub trait MutBorrowedTreeNode<'a> 
    where Self: Sized + 'a {
    
    /// A mutable reference to the value of each node in the tree.
    type MutBorrowedValue: Sized;
    /// The type of iterator that can be used to iterate over each node's children 
    /// collection.
    type MutBorrowedChildren: Iterator<Item = &'a mut Self>;

    /// This method gets the value and children from this node. The other 
    /// methods of this trait assume that the 'Children' list does not contain 
    /// any circular references. If there are, an inifite loop will result.
    fn get_value_and_children_iter_mut(&'a mut self) -> (Self::MutBorrowedValue, Option<Self::MutBorrowedChildren>);

    /// This method retrieves an iterator that can be used to perform
    /// Breadth First (Queue - specifically VecDeque-based) searches of a tree. If performance is 
    /// not a serious concern, a Breadth First (iterative deepening) search
    /// (referred to as BFS in this library) should be preferred to make
    /// debugging easier.
    /// 
    /// A Breadth First Search (BFS) is defined as:
    /// 
    /// A tree traversal that involves breadth-first searching a tree 
    /// from the top down. Given a tree of the following shape, this 
    /// traversal type would traverse the elements in the order 
    /// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10.
    /// 
    /// In this traversal, we scan each level of the tree from left to
    /// right before going down to the next level.
    /// -        0
    /// -       / \
    /// -      1   2
    /// -     / \ / \
    /// -    3  4 5  6
    /// -           /
    /// -          7
    /// -           \
    /// -            8
    /// -           /
    /// -          9
    /// -           \
    /// -           10
    /// 
    fn bfs_iter_mut(&'a mut self) -> MutBorrowedBFSIterator<Self> {
        MutBorrowedBFSIterator::new(self)
    }

   /// This method retrieves an iterator that can be used to perform
   /// Depth First Preorder searches of a tree.
   /// 
   /// A Depth First Preorder search is defined as:
   /// 
   /// A tree traversal that involves depth-first searching a tree 
   /// from the top down. Given a tree of the following shape, this 
   /// traversal type would traverse the elements in the order
   /// 0, 1, 3, 4, 2, 5, 6, 7, 8, 9, 10.
   /// 
   /// In this traversal, each node will only be traversed before any
   /// of its children have been traversed.
   /// -        0
   /// -       / \
   /// -      1   2
   /// -     / \ / \
   /// -    3  4 5  6
   /// -           /
   /// -          7
   /// -           \
   /// -            8
   /// -           /
   /// -          9
   /// -           \
   /// -           10
   /// 
    fn dfs_preorder_iter_mut(&'a mut self) -> MutBorrowedDFSPreorderIterator<'a, Self> {
        MutBorrowedDFSPreorderIterator::new(self)
    }

    /// This method retrieves an iterable that can be used to perform
    /// Depth First Postorder searches of a tree.
    /// 
    /// A Depth First Postorder search (referred to as DFS Postorder) 
    /// is defined as:
    /// 
    /// A tree traversal that involves depth-first searching a tree 
    /// from the bottom up. Given a tree of the following shape, this 
    /// traversal type would traverse the elements in the order 
    /// 3, 4, 1, 5, 10, 9, 8, 7, 6, 2, 0.
    /// 
    /// In this traversal, each node will only be traversed after all
    /// of its children have been traversed.
    /// -        0
    /// -       / \
    /// -      1   2
    /// -     / \ / \
    /// -    3  4 5  6
    /// -           /
    /// -          7
    /// -           \
    /// -            8
    /// -           /
    /// -          9
    /// -           \
    /// -           10
    /// 
    /// This traversal type guarantees that getChildren() will only be 
    /// called once per node of the tree.
    ///
    fn dfs_postorder_iter_mut(&'a mut self) -> MutBorrowedDFSPostorderIterator<'a, Self> {
        MutBorrowedDFSPostorderIterator::new(self)
    }
}

pub trait BorrowedTreeNode<'a> 
    where Self: Sized + 'a {
    
    /// A reference to the value of each node in the tree.
    type BorrowedValue: Sized;
    /// The type of iterator that can be used to iterate over each node's children 
    /// collection.
    type BorrowedChildren: Iterator<Item = &'a Self>;

    /// This method gets the value and children from this node, consuming it 
    /// in the process. The other methods of this trait assume that the 'Children' 
    /// list does not contain and circular references back to parent nodes.
    fn get_value_and_children_iter(&'a self) -> (Self::BorrowedValue, Option<Self::BorrowedChildren>);

    /// This method retrieves an iterator that can be used to perform
    /// Breadth First (Queue - specifically VecDeque-based) searches of a tree. If performance is 
    /// not a serious concern, a Breadth First (iterative deepening) search
    /// (referred to as BFS in this library) should be preferred to make
    /// debugging easier.
    /// 
    /// A Breadth First Search (BFS) is defined as:
    /// 
    /// A tree traversal that involves breadth-first searching a tree 
    /// from the top down. Given a tree of the following shape, this 
    /// traversal type would traverse the elements in the order 
    /// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10.
    /// 
    /// In this traversal, we scan each level of the tree from left to
    /// right before going down to the next level.
    /// -        0
    /// -       / \
    /// -      1   2
    /// -     / \ / \
    /// -    3  4 5  6
    /// -           /
    /// -          7
    /// -           \
    /// -            8
    /// -           /
    /// -          9
    /// -           \
    /// -           10
    /// 
    fn bfs_iter(&'a self) -> BorrowedBFSIterator<Self> {
        BorrowedBFSIterator::new(self)
    }

   /// This method retrieves an iterator that can be used to perform
   /// Depth First Preorder searches of a tree.
   /// 
   /// A Depth First Preorder search is defined as:
   /// 
   /// A tree traversal that involves depth-first searching a tree 
   /// from the top down. Given a tree of the following shape, this 
   /// traversal type would traverse the elements in the order
   /// 0, 1, 3, 4, 2, 5, 6, 7, 8, 9, 10.
   /// 
   /// In this traversal, each node will only be traversed before any
   /// of its children have been traversed.
   /// -        0
   /// -       / \
   /// -      1   2
   /// -     / \ / \
   /// -    3  4 5  6
   /// -           /
   /// -          7
   /// -           \
   /// -            8
   /// -           /
   /// -          9
   /// -           \
   /// -           10
   /// 
    fn dfs_preorder_iter(&'a self) -> BorrowedDFSPreorderIterator<'a, Self> {
        BorrowedDFSPreorderIterator::new(self)
    }

    /// This method retrieves an iterable that can be used to perform
    /// Depth First Postorder searches of a tree.
    /// 
    /// A Depth First Postorder search (referred to as DFS Postorder) 
    /// is defined as:
    /// 
    /// A tree traversal that involves depth-first searching a tree 
    /// from the bottom up. Given a tree of the following shape, this 
    /// traversal type would traverse the elements in the order 
    /// 3, 4, 1, 5, 10, 9, 8, 7, 6, 2, 0.
    /// 
    /// In this traversal, each node will only be traversed after all
    /// of its children have been traversed.
    /// -        0
    /// -       / \
    /// -      1   2
    /// -     / \ / \
    /// -    3  4 5  6
    /// -           /
    /// -          7
    /// -           \
    /// -            8
    /// -           /
    /// -          9
    /// -           \
    /// -           10
    /// 
    /// This traversal type guarantees that getChildren() will only be 
    /// called once per node of the tree.
    ///
    fn dfs_postorder_iter(&'a self) -> BorrowedDFSPostorderIterator<'a, Self> {
        BorrowedDFSPostorderIterator::new(self)
    }
}

impl<T> OwnedTreeNode for TreeNode<T> {
    type OwnedValue = T;
    type OwnedChildren = IntoIter<Self>;

    /// This method gets the value and children from this node. The other 
    /// methods of this trait assume that the 'Children' list does not contain 
    /// any circular references. If there are, an inifite loop will result.
    fn get_value_and_children(self) -> (Self::OwnedValue, Option<Self::OwnedChildren>) {
        (
            self.value, 
            match self.children {
                None => None,
                Some(children) => Some(children.into_iter())
            }
        )
    }
}

impl<'a, T> MutBorrowedTreeNode<'a> for TreeNode<T> 
    where T: 'a {

    type MutBorrowedValue = &'a mut T;
    type MutBorrowedChildren = IterMut<'a, TreeNode<T>>;

    /// This method gets the value and children from this node. The other 
    /// methods of this trait assume that the 'Children' list does not contain 
    /// any circular references. If there are, an inifite loop will result.
    fn get_value_and_children_iter_mut(&'a mut self) -> (Self::MutBorrowedValue, Option<Self::MutBorrowedChildren>) {
        (
            &mut self.value,
            match &mut self.children {
                None => None,
                Some(children) => Some(children.iter_mut())
            }
        )
    }
}

impl<'a, T> BorrowedTreeNode<'a> for TreeNode<T> 
    where T: 'a {

    type BorrowedValue = &'a T;
    type BorrowedChildren = Iter<'a, TreeNode<T>>;

    /// This method gets the value and children from this node. The other 
    /// methods of this trait assume that the 'Children' list does not contain 
    /// any circular references. If there are, an inifite loop will result.
    fn get_value_and_children_iter(&'a self) -> (Self::BorrowedValue, Option<Self::BorrowedChildren>) {
        let children_iter = match &self.children {
            Some(vec) => Some(vec.iter()),
            None => None
        };
        (&self.value, children_iter)
    }
}

#[cfg(test)]
pub (crate) mod tests {
    use super::*;

    #[test]
    fn dfs_preorder_has_correct_order() {
        let expected = vec![0,1,3,4,2,5,6,7,8,9,10];
        for test_tree in create_trees_for_testing() {
            for (i, value) in test_tree.dfs_preorder().enumerate() {
                assert_eq!(expected[i], value);
            }
        }

        let expected = vec![0,1,3,4,2,5,6,7,8,9,10];
        for mut test_tree in create_trees_for_testing() {
            for (i, value) in test_tree.dfs_preorder_iter_mut().enumerate() {
                assert_eq!(expected[i], *value);
            }
        }

        let expected = vec![0,1,3,4,2,5,6,7,8,9,10];
        for test_tree in create_trees_for_testing() {
            for (i, value) in test_tree.dfs_preorder_iter().enumerate() {
                assert_eq!(expected[i], *value);
            }
        }
    }

    #[test]
    fn bfs_has_correct_order() {
        let expected = (0..=10).collect::<Vec<usize>>();
        for test_tree in create_trees_for_testing() {
            for (i, value) in test_tree.bfs().enumerate() {
                assert_eq!(expected[i], value);
            }
        }

        let expected = (0..=10).collect::<Vec<usize>>();
        for mut test_tree in create_trees_for_testing() {
            for (i, value) in test_tree.bfs_iter_mut().enumerate() {
                assert_eq!(expected[i], *value);
            }
        }

        let expected = (0..=10).collect::<Vec<usize>>();
        for test_tree in create_trees_for_testing() {
            for (i, value) in test_tree.bfs_iter().enumerate() {
                assert_eq!(expected[i], *value);
            }
        }
    }

    #[test]
    fn dfs_postorder_has_correct_order() {
        let expected = get_expected_order_dfs_postorder();
        for test_tree in create_trees_for_testing() {
            for (i, value) in test_tree.dfs_postorder().enumerate() {
                assert_eq!(expected[i], value);
            }
        }

        for mut test_tree in create_trees_for_testing() {
            for (i, value) in test_tree.dfs_postorder_iter_mut().enumerate() {
                assert_eq!(expected[i], *value);
            }
        }

        for test_tree in create_trees_for_testing() {
            for (i, value) in test_tree.dfs_postorder_iter().enumerate() {
                assert_eq!(expected[i], *value);
            }
        }
    }

    #[test]
    fn dfs_postorder_with_metadata_works() {
        let expected = get_expected_order_dfs_postorder();

        for mut test_tree in create_trees_for_testing() {
            let mut i = 0;
            let mut iter_with_metadata = test_tree.dfs_postorder_iter_mut().attach_ancestors();
            while let Some(value) = iter_with_metadata.next() {
                assert_eq!(expected[i], *value[value.len() - 1]);
                let (expected_depth, expected_parent, expected_ancestors) = match *value[value.len() - 1] {
                    0 => (0, None, vec![]),
                    1 => (1, Some(0), vec![0]),
                    2 => (1, Some(0), vec![0]),
                    3 => (2, Some(1), vec![0, 1]),
                    4 => (2, Some(1), vec![0, 1]),
                    5 => (2, Some(2), vec![0, 2]),
                    6 => (2, Some(2), vec![0, 2]),
                    7 => (3, Some(6), vec![0, 2, 6]),
                    8 => (4, Some(7), vec![0, 2, 6, 7]),
                    9 => (5, Some(8), vec![0, 2, 6, 7, 8]),
                    10 => (6, Some(9), vec![0, 2, 6, 7, 8, 9]),
                    _ => panic!("unexpected value"),
                };

                assert_eq!(expected_depth, value.len() - 1);

                if value.len() > 1 {
                    assert_eq!(expected_parent.expect("parent to exist"), *value[value.len() - 2]);
                } else {
                    assert_eq!(None, expected_parent);
                }

                for (j, ancestor) in value.iter().enumerate() {
                    if j == value.len() - 1 { continue; }
                    assert_eq!(expected_ancestors[j], **ancestor);
                }

                i += 1;
            }
        }
    }

    fn get_expected_order_dfs_postorder() -> Vec<usize> {
        vec![3,4,1,5,10,9,8,7,6,2,0]
    }

    fn create_trees_for_testing() -> Vec<TreeNode<usize>> {
        vec![
            create_tree_for_testing(None),
            create_tree_for_testing(Some(Vec::new()))
        ]
    }

    pub (crate) fn create_tree_for_testing(empty_children_list: Option<Vec<TreeNode<usize>>>) -> TreeNode<usize> {
        TreeNode {
            value: 0,
            children: Some(vec![
                TreeNode {
                    value: 1,
                    children: Some(vec![
                        TreeNode {
                            value: 3,
                            children: empty_children_list.clone()
                        },
                        TreeNode {
                            value: 4,
                            children: empty_children_list.clone()
                        }
                    ])
                },
                TreeNode {
                    value: 2,
                    children: Some(vec![
                        TreeNode {
                            value: 5,
                            children: empty_children_list.clone()
                        },
                        TreeNode {
                            value: 6,
                            children: Some(vec![
                                TreeNode {
                                    value: 7,
                                    children: Some(vec![
                                        TreeNode {
                                            value: 8,
                                            children: Some(vec![
                                                TreeNode {
                                                    value: 9,
                                                    children: Some(vec![
                                                        TreeNode {
                                                            value: 10,
                                                            children: empty_children_list.clone()
                                                        }
                                                    ])
                                                }
                                            ])
                                        }
                                    ])
                                }
                            ])
                        }
                    ])
                }
            ])
        }
    }
}