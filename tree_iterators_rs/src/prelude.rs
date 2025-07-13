use alloc::boxed::Box;
use alloc::vec::Vec;

use core::slice::{Iter, IterMut};
use core::{fmt::Debug, iter::FusedIterator};

use core::iter::FlatMap;

#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

use crate::bfs_iterators::borrow::{
    BorrowedBFSCollectionIterator, BorrowedBinaryBFSCollectionIterator,
};
use crate::bfs_iterators::mut_borrow::{
    MutBorrowedBFSCollectionIterator, MutBorrowedBinaryBFSCollectionIterator,
};
use crate::bfs_iterators::owned::{OwnedBFSCollectionIterator, OwnedBinaryBFSCollectionIterator};
use crate::dfs_inorder_iterators::borrow::BorrowedDFSInorderCollectionIterator;
use crate::dfs_inorder_iterators::mut_borrow::MutBorrowedDFSInorderCollectionIterator;
use crate::dfs_inorder_iterators::owned::OwnedDFSInorderCollectionIterator;
use crate::dfs_postorder_iterators::borrow::{
    BorrowedBinaryDFSPostorderCollectionIterator, BorrowedDFSPostorderCollectionIterator,
};
use crate::dfs_postorder_iterators::mut_borrow::{
    MutBorrowedBinaryDFSPostorderCollectionIterator, MutBorrowedDFSPostorderCollectionIterator,
};
use crate::dfs_postorder_iterators::owned::{
    OwnedBinaryDFSPostorderCollectionIterator, OwnedDFSPostorderCollectionIterator,
};
use crate::dfs_preorder_iterators::borrow::{
    BorrowedBinaryDFSPreorderCollectionIterator,
    BorrowedBinaryDFSPreorderCollectionIteratorWithPathTracking,
    BorrowedBinaryDFSPreorderIteratorWithPathTracking, BorrowedDFSPreorderCollectionIterator,
    BorrowedDFSPreorderCollectionIteratorWithPathTracking,
    BorrowedDFSPreorderIteratorWithPathTracking,
};
use crate::dfs_preorder_iterators::mut_borrow::{
    MutBorrowedBinaryDFSPreorderCollectionIterator,
    MutBorrowedBinaryDFSPreorderCollectionIteratorWithPathTracking,
    MutBorrowedBinaryDFSPreorderIteratorWithPathTracking, MutBorrowedDFSPreorderCollectionIterator,
    MutBorrowedDFSPreorderCollectionIteratorWithPathTracking,
    MutBorrowedDFSPreorderIteratorWithPathTracking,
};
use crate::dfs_preorder_iterators::owned::{
    OwnedBinaryDFSPreorderCollectionIterator,
    OwnedBinaryDFSPreorderCollectionIteratorWithPathTracking,
    OwnedBinaryDFSPreorderIteratorWithPathTracking, OwnedDFSPreorderCollectionIterator,
    OwnedDFSPreorderCollectionIteratorWithPathTracking, OwnedDFSPreorderIteratorWithPathTracking,
};

pub use crate::tree_collection_iterators::BinaryTrees;
pub use crate::tree_collection_iterators::Trees;

use super::bfs_iterators::{
    borrow::{BorrowedBFSIterator, BorrowedBinaryBFSIterator},
    mut_borrow::{MutBorrowedBFSIterator, MutBorrowedBinaryBFSIterator},
    owned::{OwnedBFSIterator, OwnedBinaryBFSIterator},
};

use super::dfs_preorder_iterators::{
    borrow::{BorrowedBinaryDFSPreorderIterator, BorrowedDFSPreorderIterator},
    mut_borrow::{MutBorrowedBinaryDFSPreorderIterator, MutBorrowedDFSPreorderIterator},
    owned::{OwnedBinaryDFSPreorderIterator, OwnedDFSPreorderIterator},
};

use super::dfs_inorder_iterators::{
    borrow::BorrowedDFSInorderIterator, mut_borrow::MutBorrowedDFSInorderIterator,
    owned::OwnedDFSInorderIterator,
};

use super::dfs_postorder_iterators::{
    borrow::{BorrowedBinaryDFSPostorderIterator, BorrowedDFSPostorderIterator},
    mut_borrow::{MutBorrowedBinaryDFSPostorderIterator, MutBorrowedDFSPostorderIterator},
    owned::{OwnedBinaryDFSPostorderIterator, OwnedDFSPostorderIterator},
};

pub use super::tree_context::TreeContext;
pub use super::tree_iterators::{
    BinaryPrune, BinaryPrunePath, BinaryTreeIterator, Map, Prune, PruneDepth, PrunePath,
    TreeIterator, TreeIteratorBase,
};

pub use super::tree_collection_iterators::{
    BinaryCollectionPrune, BinaryCollectionPrunePath, BinaryFold, BinaryTreeCollectionIterator,
    CollectionMap, CollectionPrune, CollectionPruneDepth, CollectionPrunePath, Fold,
    TreeCollectionIterator, TreeCollectionIteratorBase,
};

/// A default implemenation of a binary tree node. This struct
/// provides a series of tree traversal utilities to allow
/// you to easily work with and modify binary trees.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BinaryTree<T> {
    /// This node's value
    pub value: T,
    /// The left child of the current node.
    pub left: Option<Box<BinaryTree<T>>>,
    /// The right child of the current node.
    pub right: Option<Box<BinaryTree<T>>>,
}

impl<T> BinaryTree<&T>
where
    T: Clone,
{
    /// Maps a [`BinaryTree<&T>`] to a [`BinaryTree<T>`] by cloning the contents of the [`BinaryTree`].
    pub fn cloned(&self) -> BinaryTree<T> {
        self.map_ref(|item| (*item).clone())
    }
}

impl<T> BinaryTree<&mut T>
where
    T: Clone,
{
    /// Maps a [`BinaryTree<&mut T>`] to a [`BinaryTree<T>`] by cloning the contents of the [`BinaryTree`].
    pub fn cloned(&self) -> BinaryTree<T> {
        self.map_ref(|item| (*item).clone())
    }
}

/// A default implemenation of a tree node. This struct
/// provides a series of tree traversal utilities to allow
/// you to easily work with and modify arbitrary trees.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tree<T> {
    /// This node's value
    pub value: T,
    /// The children of the current node.
    pub children: Vec<Tree<T>>,
}

impl<T> Tree<&T>
where
    T: Clone,
{
    /// Maps a [`Tree<&T>`] to a [`Tree<T>`] by cloning the contents of the [`Tree`].
    pub fn cloned(&self) -> Tree<T> {
        self.map_ref(|item| (*item).clone())
    }
}

impl<T> Tree<&mut T>
where
    T: Clone,
{
    /// Maps a [`Tree<&mut T>`] to a [`Tree<T>`] by cloning the contents of the [`Tree`].
    pub fn cloned(&self) -> Tree<T> {
        self.map_ref(|item| (*item).clone())
    }
}

/// Helper type to define the BinaryTreeNode's
/// Children iterator type.
pub(crate) type BinaryChildren<T> =
    FlatMap<core::array::IntoIter<Option<T>, 2>, Option<T>, fn(Option<T>) -> Option<T>>;

/// A binary tree node where getting its children consumes its value.
pub trait OwnedBinaryTreeNode
where
    Self: Sized,
{
    /// The value of each node in the tree.
    type OwnedValue;

    /// This method gets the value and left and right children from this node,
    /// consuming it in the process. The other methods of this trait assume that
    /// the children do not contain any circular references. If they do,
    /// it will create an infinite loop.
    fn get_value_and_children_binary(self) -> (Self::OwnedValue, [Option<Self>; 2]);

    /// This method gets the value and children from this node, consuming it
    /// in the process. The other methods of this trait assume that the 'Children'
    /// list does not contain any circular references. If it does, it will create
    /// an infinite loop.
    fn get_value_and_children(self) -> (Self::OwnedValue, BinaryChildren<Self>) {
        let (value, children) = self.get_value_and_children_binary();
        (
            value,
            children
                .into_iter()
                .flat_map(opt_to_opt as fn(Option<Self>) -> Option<Self>),
        )
    }

    #[doc = include_str!("../doc_files/at_path.md")]
    #[doc = include_str!("../doc_files/at_path_binary_example.md")]
    fn at_path(self, path: &[usize]) -> Option<Self> {
        let mut current = self;
        for path_segment in path {
            current = current
                .get_value_and_children_binary()
                .1
                .into_iter()
                .nth(*path_segment)??;
        }
        Some(current)
    }

    /// This method retrieves an iterator that can be used to perform
    /// Breadth First (Queue - specifically VecDeque-based) searches of a tree.
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    #[must_use]
    fn bfs(self) -> OwnedBinaryBFSIterator<Self> {
        OwnedBinaryBFSIterator::new(self)
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    #[must_use]
    fn dfs_preorder(self) -> OwnedBinaryDFSPreorderIterator<Self> {
        OwnedBinaryDFSPreorderIterator::new(self)
    }

    /// This method retrieves an iterator that can be used to perform
    /// Depth First In Order searches of a tree.
    ///
    /// A Depth First In Order search is defined as:
    ///
    /// A tree traversal that involves depth-first searching a tree
    /// from the left to the right. Given a tree of the following shape,
    /// this traversal type would traverse
    /// the elements in the order
    /// 3, 1, 4, 0, 5, 2, 7, 9, 10, 8, 6.
    ///
    /// In this traversal, each node will be traversed after its left
    /// child and before its right child.
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    #[must_use]
    fn dfs_inorder(self) -> OwnedDFSInorderIterator<Self> {
        OwnedDFSInorderIterator::new(self)
    }

    /// This method retrieves an iterator that can be used to perform
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    ///
    /// This traversal type guarantees that getChildren() will only be
    /// called once per node of the tree.
    #[must_use]
    fn dfs_postorder(self) -> OwnedBinaryDFSPostorderIterator<Self> {
        OwnedBinaryDFSPostorderIterator::new(self)
    }

    /// This method converts the current BinaryTreeNode into a BinaryTreeIterator.
    ///
    /// BinaryTreeIterators have 2 purposes:
    /// 1. they serve as the internal piping of tree_iterators_rs
    /// 2. they can efficiently chain the prune, map, and fold operations on a tree.
    ///
    /// If you are only applying a single prune, map, or fold operation, just call the
    /// associated method.
    /// - [`prune`](crate::prelude::OwnedBinaryTreeNode::prune)
    /// - [`map`](crate::prelude::OwnedBinaryTreeNode::map)
    /// - [`fold`](crate::prelude::OwnedBinaryTreeNode::fold)
    ///
    /// If you are chaining many operations together, use into_pipeline. This will
    /// be much more efficient in memory since it only maintains a single ancestor stack
    /// of the tree at a time.
    ///
    /// ### Example Usage:
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_binary_tree,
    ///     prelude::{BinaryTree, OwnedBinaryTreeNode, TreeIteratorBase, BinaryTreeIterator}
    /// };
    ///
    /// let tree = create_example_binary_tree();
    /// let result = tree.into_pipeline()
    ///     .prune_depth(2)
    ///     .map_tree(|value| value + 200)
    ///     .collect_tree()
    ///     .expect("all non-prune methods to collect into a Some()");
    ///
    /// assert_eq!(
    ///     BinaryTree {
    ///         value: 200,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 201,
    ///             left: Some(Box::new(BinaryTree {
    ///                 value: 203,
    ///                 left: None,
    ///                 right: None,
    ///             })),
    ///             right: Some(Box::new(BinaryTree {
    ///                 value: 204,
    ///                 left: None,
    ///                 right: None,
    ///             })),
    ///         })),
    ///         right: Some(Box::new(BinaryTree {
    ///             value: 202,
    ///             left: Some(Box::new(BinaryTree {
    ///                 value: 205,
    ///                 left: None,
    ///                 right: None,
    ///             })),
    ///             right: Some(Box::new(BinaryTree {
    ///                 value: 206,
    ///                 left: None,
    ///                 right: None,
    ///             })),
    ///         })),
    ///     },
    ///     result
    /// );
    /// ```
    #[must_use]
    fn into_pipeline(self) -> impl BinaryTreeIterator<Self::OwnedValue, [Option<Self>; 2]> {
        OwnedBinaryDFSPreorderIteratorWithPathTracking::new(self, Vec::new())
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the given closure to determine if each subtree in this tree should be pruned.
    ///
    /// Given an element the closure must return true or false. Any nodes in the tree for
    /// which this evaluates to true will be pruned out of the resulting tree. If the root node is pruned,
    /// `prune` will return [`None`].
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder`](crate::prelude::OwnedBinaryTreeNode::dfs_preorder) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, OwnedBinaryTreeNode};
    ///
    /// let tree = BinaryTree {
    ///     value: 0,
    ///     left: Some(Box::new(BinaryTree {
    ///         value: 1,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 3,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: None,
    ///     })),
    ///     right: Some(Box::new(BinaryTree {
    ///         value: 2,
    ///         left: None,
    ///         right: None,
    ///     })),
    /// };
    ///
    /// let result = tree.prune(|value| {
    ///     /// The output for this code would be the following. A couple notes about
    ///     /// this output:
    ///     /// 1. the node with a value of '1' has been removed
    ///     /// 2. the closure is never called on the node with a value of '3' since
    ///     ///    it is already determined to be pruned once '1' has been evaluated.
    ///     /// ```
    ///     /// 0
    ///     /// 1
    ///     /// 2
    ///     /// ```
    ///     println!("{value:?}");
    ///     *value == 1
    /// });
    ///
    /// assert_eq!(
    ///     Some(BinaryTree {
    ///         value: 0,
    ///         left: None,
    ///         right: Some(
    ///             Box::new(BinaryTree {
    ///                 value: 2,
    ///                 left: None,
    ///                 right: None,
    ///             }),
    ///         ),
    ///     }),
    ///     result
    /// );
    ///
    /// ```
    fn prune<F>(self, f: F) -> Option<BinaryTree<Self::OwnedValue>>
    where
        F: FnMut(&Self::OwnedValue) -> bool,
    {
        self.into_pipeline().prune(f).collect_tree()
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the depth of each subtree to determine if the subtree should be pruned.
    /// Any node with a depth that is strictly greater than the max_depth parameter
    /// will be pruned from the tree.
    ///
    /// Depth is zero-based, so the root node is considered to be at depth zero.
    ///
    /// Ex. given a tree like the following, the depths would be as labeled.
    ///
    /// ```text
    ///        0       <- depth: 0
    ///       / \
    ///      1   2     <- depth: 1
    ///     / \ / \
    ///    3  4 5  6   <- depth: 2
    ///           /
    ///          7     <- depth: 3
    ///           \
    ///            8   <- depth: 4
    ///           /
    ///          9     <- depth: 5
    ///           \
    ///           10   <- depth: 6
    /// ```
    ///
    /// ### Basic usage:
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, OwnedBinaryTreeNode};
    ///
    /// let tree = BinaryTree {
    ///     value: 0,
    ///     left: Some(Box::new(BinaryTree {
    ///         value: 1,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 3,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: None,
    ///     })),
    ///     right: Some(Box::new(BinaryTree {
    ///         value: 2,
    ///         left: None,
    ///         right: None,
    ///     }))
    /// };
    ///
    /// assert_eq!(
    ///     BinaryTree {
    ///         value: 0,
    ///         left: None,
    ///         right: None,
    ///     },
    ///     tree.prune_depth(0)
    /// );
    /// ```
    fn prune_depth(self, max_depth: usize) -> BinaryTree<Self::OwnedValue> {
        self.into_pipeline()
            .prune_depth(max_depth)
            .collect_tree()
            .expect("this should never prune the root of the tree")
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the given closure to determine if each subtree in this tree should be pruned.
    ///
    /// Given an element and its context in the tree, the closure must return true or false.
    /// Any nodes in the tree for which this evaluates to true will be pruned out of the resulting
    /// tree. If the root node is pruned, `prune` will return [`None`].
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder`](crate::prelude::OwnedTreeNode::dfs_preorder) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, OwnedBinaryTreeNode};
    ///
    /// let tree = BinaryTree {
    ///     value: 0,
    ///     left: Some(Box::new(BinaryTree {
    ///         value: 1,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 3,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: None,
    ///     })),
    ///     right: Some(Box::new(BinaryTree {
    ///         value: 2,
    ///         left: None,
    ///         right: None,
    ///     }))
    /// };
    ///
    /// let result = tree.prune_path(|path, value| {
    ///     /// The output for this code would be the following. A couple notes about
    ///     /// this output:
    ///     /// 1. the node with a value of '1' has been removed
    ///     /// 2. the closure is never called on the node with a value of '3' since
    ///     /// it is already determined to be pruned once '1' has been evaluated.
    ///     /// ```
    ///     /// [0]; 0
    ///     /// [0, 0]; 1
    ///     /// [0, 1]; 2
    ///     /// ```
    ///     println!("{:?}; {:?}", path, value);
    ///     *value == 1
    /// });
    ///
    /// assert_eq!(
    ///     Some(BinaryTree {
    ///         value: 0,
    ///         left: None,
    ///         right: Some(Box::new(BinaryTree {
    ///             value: 2,
    ///             left: None,
    ///             right: None,
    ///         }))
    ///     }),
    ///     result
    /// );
    /// ```
    fn prune_path<F>(self, f: F) -> Option<BinaryTree<Self::OwnedValue>>
    where
        F: FnMut(&[usize], &Self::OwnedValue) -> bool,
    {
        self.into_pipeline().prune_path(f).collect_tree()
    }

    /// map is a tree-based analog to [map](core::iter::Iterator::map).
    ///
    /// Takes a closure and applies that closure to each node's value in the tree.
    ///
    /// map() transforms one tree into another, by means of its argument: something that
    /// implements FnMut. It produces a new tree which calls this closure on each node of
    /// the original tree.
    ///
    /// If you are good at thinking in types, you can think of map() like this: If you
    /// have a tree that has elements of some type A, and you want a tree of some other
    /// type B, you can use map(), passing a closure that takes an A and returns a B.
    ///
    /// ### Example Usage
    ///
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_binary_tree,
    ///     prelude::{BinaryTree, OwnedBinaryTreeNode}
    /// };
    ///
    /// let tree = BinaryTree {
    ///     value: "0-0",
    ///     left: Some(Box::new(BinaryTree {
    ///         value: "1-1",
    ///         left: None,
    ///         right: None,
    ///     })),
    ///     right: Some(Box::new(BinaryTree {
    ///         value: "2-2",
    ///         left: None,
    ///         right: None,
    ///     })),
    /// };
    ///
    /// let result = tree.map(|value: &'static str| {
    ///     value.split("-").nth(1).unwrap().to_string()
    /// });
    ///
    /// assert_eq!(
    ///     BinaryTree {
    ///         value: "0".to_string(),
    ///         left: Some(Box::new(BinaryTree {
    ///             value: "1".to_string(),
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: Some(Box::new(BinaryTree {
    ///             value: "2".to_string(),
    ///             left: None,
    ///             right: None,
    ///         })),
    ///     },
    ///     result);
    /// ```
    fn map<Output, F>(self, f: F) -> BinaryTree<Output>
    where
        F: FnMut(Self::OwnedValue) -> Output,
    {
        self.into_pipeline().map_tree(f).collect_tree().unwrap()
    }

    /// fold is a tree-based analog to [fold](core::iter::Iterator::fold).
    ///
    /// Folds every element into an accumulation by applying an operation, returning the
    /// final result.
    ///
    /// fold() takes one argument: a closure with two arguments: the result of accumulating
    /// all children of the current tree node, and an element. The closure returns the value
    /// that the accumulator should have for the parent node's accumulation.
    ///
    /// After applying this closure to every node of the tree, fold() returns the accumulation.
    ///
    /// This operation is sometimes called ‘reduce’ or ‘inject’.
    ///
    /// Folding is useful whenever you have a tree of something, and want to produce a single
    /// value from it.
    ///
    /// ### Example Usage
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_binary_tree,
    ///     prelude::OwnedBinaryTreeNode
    /// };
    ///
    /// let tree = create_example_binary_tree();
    /// let accumulation = tree.fold(|child_accumulations: [Option<usize>; 2], value| {
    ///     child_accumulations
    ///         .into_iter()
    ///         .map(|opt| opt.unwrap_or_default())
    ///         .sum::<usize>()
    ///     + value
    /// });
    ///
    /// assert_eq!(55, accumulation);
    /// ```
    fn fold<Output, F>(self, f: F) -> Output
    where
        F: FnMut([Option<Output>; 2], Self::OwnedValue) -> Output,
    {
        self.into_pipeline()
            .fold_tree(f)
            .expect("there to always be at least the root to fold")
    }
}

/// A tree node where getting its children consumes its value.
pub trait OwnedTreeNode
where
    Self: Sized,
{
    /// The value of each node in the tree.
    type OwnedValue: Sized;

    /// The type of iterator that can be used to iterate over each node's children
    /// collection.
    type OwnedChildren: IntoIterator<Item = Self>;

    /// This method gets the value and children from this node, consuming it
    /// in the process. The other methods of this trait assume that the 'Children'
    /// list does not contain any circular references. If it does, it will create
    /// an infinite loop.
    fn get_value_and_children(self) -> (Self::OwnedValue, Self::OwnedChildren);

    #[doc = include_str!("../doc_files/at_path.md")]
    #[doc = include_str!("../doc_files/at_path_tree_example.md")]
    fn at_path(self, path: &[usize]) -> Option<Self> {
        let mut current = self;
        for path_segment in path {
            current = current
                .get_value_and_children()
                .1
                .into_iter()
                .nth(*path_segment)?;
        }
        Some(current)
    }

    /// This method retrieves an iterator that can be used to perform
    /// Breadth First (Queue - specifically VecDeque-based) searches of a tree.
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    #[must_use]
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    #[must_use]
    fn dfs_preorder(self) -> OwnedDFSPreorderIterator<Self> {
        OwnedDFSPreorderIterator::new(self)
    }

    /// This method retrieves an iterator that can be used to perform
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    ///
    /// This traversal type guarantees that getChildren() will only be
    /// called once per node of the tree.
    #[must_use]
    fn dfs_postorder(self) -> OwnedDFSPostorderIterator<Self> {
        OwnedDFSPostorderIterator::new(self)
    }

    /// This method converts the current TreeNode into a TreeIterator.
    ///
    /// TreeIterators have 2 purposes:
    /// 1. they serve as the internal piping of tree_iterators_rs
    /// 2. they can efficiently chain the prune, map, and fold operations on a tree.
    ///
    /// If you are only applying a single prune, map, or fold operation, just call the
    /// associated method.
    /// - [`prune`](crate::prelude::OwnedTreeNode::prune)
    /// - [`map`](crate::prelude::OwnedTreeNode::map)
    /// - [`fold`](crate::prelude::OwnedTreeNode::fold)
    ///
    /// If you are chaining many operations together, use into_pipeline. This will
    /// be much more efficient in memory since it only maintains a single ancestor stack
    /// of the tree at a time.
    ///
    /// ### Example Usage:
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_tree,
    ///     prelude::{Tree, OwnedTreeNode, TreeIteratorBase, TreeIterator}
    /// };
    ///
    /// let tree = create_example_tree();
    /// let result = tree.into_pipeline()
    ///     .prune_depth(2)
    ///     .map_tree(|value| value + 200)
    ///     .collect_tree()
    ///     .expect("all non-prune methods to collect into a Some()");
    ///
    /// assert_eq!(
    ///     Tree {
    ///        value: 200,
    ///        children: vec![
    ///            Tree {
    ///                value: 201,
    ///                children: vec![
    ///                    Tree {
    ///                        value: 203,
    ///                        children: vec![],
    ///                    },
    ///                    Tree {
    ///                        value: 204,
    ///                        children: vec![],
    ///                    },
    ///                ],
    ///            },
    ///            Tree {
    ///                value: 202,
    ///                children: vec![
    ///                    Tree {
    ///                        value: 205,
    ///                        children: vec![],
    ///                    },
    ///                    Tree {
    ///                        value: 206,
    ///                        children: vec![],
    ///                    },
    ///                ],
    ///            },
    ///        ],
    ///     },
    ///     result);
    /// ```
    #[must_use]
    fn into_pipeline(self) -> impl TreeIterator<Self::OwnedValue, Self::OwnedChildren> {
        OwnedDFSPreorderIteratorWithPathTracking::new(self, Vec::new())
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the given closure to determine if each subtree in this tree should be pruned.
    ///
    /// Given an element the closure must return true or false. Any nodes in the tree for
    /// which this evaluates to true will be pruned out of the resulting tree. If the root
    /// node is pruned, this will return [`None`].
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder`](crate::prelude::OwnedTreeNode::dfs_preorder) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{OwnedTreeNode, Tree};
    ///
    /// let tree = Tree {
    ///     value: 0,
    ///     children: vec![
    ///         Tree {
    ///             value: 1,
    ///             children: vec![Tree {
    ///                 value: 3,
    ///                 children: Vec::new(),
    ///             }],
    ///         },
    ///         Tree {
    ///             value: 2,
    ///             children: Vec::new()
    ///         },
    ///     ],
    /// };
    ///
    /// assert_eq!(
    ///     Some(
    ///         Tree {
    ///             value: 0,
    ///             children: vec![
    ///                 Tree {
    ///                     value: 2,
    ///                     children: Vec::new(),
    ///                 }
    ///             ],
    ///         },
    ///     ),
    ///     tree.prune(|value| {
    ///         /// The output for this code would be the following. A couple notes about
    ///         /// this output:
    ///         /// 1. the node with a value of '1' has been removed
    ///         /// 2. the closure is never called on the node with a value of '3' since
    ///         ///    it is already determined to be pruned once '1' has been evaluated.
    ///         /// ```
    ///         /// 0
    ///         /// 1
    ///         /// 2
    ///         /// ```
    ///         println!("{value:?}");
    ///         *value == 1
    ///     })
    /// );
    /// ```
    fn prune<F>(self, f: F) -> Option<Tree<Self::OwnedValue>>
    where
        F: FnMut(&Self::OwnedValue) -> bool,
    {
        self.into_pipeline().prune(f).collect_tree()
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the given closure to determine if each subtree in this tree should be pruned.
    ///
    /// Given the path of an element and the element's value, the closure must return true or
    /// false. Any nodes in the tree for which this evaluates to true will be pruned out of
    /// the resulting tree. If the root node is pruned, this will return [`None`].
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder`](crate::prelude::OwnedTreeNode::dfs_preorder) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{OwnedTreeNode, Tree};
    ///
    /// let tree = Tree {
    ///     value: 0,
    ///     children: vec![
    ///         Tree {
    ///             value: 1,
    ///             children: vec![Tree {
    ///                 value: 3,
    ///                 children: Vec::new(),
    ///             }],
    ///         },
    ///         Tree {
    ///             value: 2,
    ///             children: Vec::new()
    ///         },
    ///     ],
    /// };
    ///
    /// assert_eq!(
    ///     Some(
    ///         Tree {
    ///             value: 0,
    ///             children: vec![
    ///                 Tree {
    ///                     value: 2,
    ///                     children: Vec::new(),
    ///                 }
    ///             ],
    ///         },
    ///     ),
    ///     tree.prune_path(|path, value| {
    ///         /// The output for this code would be the following. A couple notes about
    ///         /// this output:
    ///         /// 1. the node with a value of '1' has been removed
    ///         /// 2. the closure is never called on the node with a value of '3' since
    ///         ///    it is already determined to be pruned once '1' has been evaluated.
    ///         /// ```
    ///         /// 0
    ///         /// 1
    ///         /// 2
    ///         /// ```
    ///         println!("{value:?}");
    ///         matches!(path.get(0), Some(0))
    ///     })
    /// );
    /// ```
    fn prune_path<F>(self, f: F) -> Option<Tree<Self::OwnedValue>>
    where
        F: FnMut(&[usize], &Self::OwnedValue) -> bool,
    {
        self.into_pipeline().prune_path(f).collect_tree()
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the depth of each subtree to determine if the subtree should be pruned.
    /// Any node with a depth that is strictly greater than the max_depth parameter
    /// will be pruned from the tree.
    ///
    /// Depth is zero-based, so the root node is considered to be at depth zero.
    ///
    /// Ex. given a tree like the following, the depths would be as labeled.
    ///
    /// ```text
    ///        0       <- depth: 0
    ///       / \
    ///      1   2     <- depth: 1
    ///     / \ / \
    ///    3  4 5  6   <- depth: 2
    ///           /
    ///          7     <- depth: 3
    ///           \
    ///            8   <- depth: 4
    ///           /
    ///          9     <- depth: 5
    ///           \
    ///           10   <- depth: 6
    /// ```
    ///
    /// ### Basic usage:
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, OwnedTreeNode};
    ///
    /// let tree = Tree {
    ///     value: 0,
    ///     children: vec![
    ///         Tree {
    ///             value: 1,
    ///             children: vec![Tree {
    ///                 value: 3,
    ///                 children: vec![],
    ///             }],
    ///         },
    ///         Tree {
    ///             value: 2,
    ///             children: vec![],
    ///         },
    ///     ],
    /// };
    ///
    /// assert_eq!(
    ///     Tree {
    ///         value: 0,
    ///         children: vec![],
    ///     },
    ///     tree.prune_depth(0)
    /// );
    /// ```
    fn prune_depth(self, max_depth: usize) -> Tree<Self::OwnedValue> {
        self.into_pipeline()
            .prune_depth(max_depth)
            .collect_tree()
            .unwrap()
    }

    /// map is a tree-based analog to [map](core::iter::Iterator::map).
    ///
    /// Takes a closure and applies that closure to each node's value in the tree.
    ///
    /// map() transforms one tree into another, by means of its argument: something that
    /// implements FnMut. It produces a new tree which calls this closure on each node of
    /// the original tree.
    ///
    /// If you are good at thinking in types, you can think of map() like this: If you
    /// have a tree that has elements of some type A, and you want a tree of some other
    /// type B, you can use map(), passing a closure that takes an A and returns a B.
    ///
    /// ### Example Usage
    ///
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_binary_tree,
    ///     prelude::{Tree, OwnedTreeNode}
    /// };
    ///
    /// let tree = Tree {
    ///     value: "0-0",
    ///     children: vec![
    ///         Tree {
    ///             value: "1-1",
    ///             children: vec![],
    ///         },
    ///         Tree {
    ///             value: "2-2",
    ///             children: vec![],
    ///         }
    ///     ],
    /// };
    ///
    /// let result = tree.map(|value: &'static str| {
    ///     value.split("-").nth(1).unwrap().to_string()
    /// });
    ///
    /// assert_eq!(
    ///     Tree {
    ///         value: "0".to_string(),
    ///         children: vec![
    ///             Tree {
    ///                 value: "1".to_string(),
    ///                 children: vec![],
    ///             },
    ///             Tree {
    ///                 value: "2".to_string(),
    ///                 children: vec![],
    ///             },
    ///         ],
    ///     },
    ///     result);
    /// ```
    fn map<Output, F>(self, f: F) -> Tree<Output>
    where
        F: FnMut(Self::OwnedValue) -> Output,
    {
        self.into_pipeline().map_tree(f).collect_tree().unwrap()
    }

    /// fold is a tree-based analog to [fold](core::iter::Iterator::fold).
    ///
    /// Folds every element into an accumulation by applying an operation, returning the
    /// final result.
    ///
    /// fold() takes one argument: a closure with two arguments: the result of accumulating
    /// all children of the current tree node, and an element. The closure returns the value
    /// that the accumulator should have for the parent node's accumulation.
    ///
    /// After applying this closure to every node of the tree, fold() returns the accumulation.
    ///
    /// This operation is sometimes called ‘reduce’ or ‘inject’.
    ///
    /// Folding is useful whenever you have a tree of something, and want to produce a single
    /// value from it.
    ///
    /// ### Example Usage
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_tree,
    ///     prelude::OwnedTreeNode
    /// };
    ///
    /// let tree = create_example_tree();
    /// let accumulation = tree.fold(|child_accumulations: Vec<usize>, value| {
    ///     child_accumulations
    ///         .into_iter()
    ///         .sum::<usize>()
    ///     + value
    /// });
    ///
    /// assert_eq!(55, accumulation);
    /// ```
    fn fold<Output, F>(self, f: F) -> Output
    where
        F: FnMut(Vec<Output>, Self::OwnedValue) -> Output,
    {
        self.into_pipeline().fold_tree(f).unwrap()
    }
}

/// A binary tree node where getting its children mutably borrows its value.
pub trait MutBorrowedBinaryTreeNode<'a>
where
    Self: Sized + 'a,
{
    /// A mutable reference to the value of each node in the tree.
    type MutBorrowedValue;

    /// This method gets the value and left and right children from this node,
    /// borrowing it as mutable in the process. The other methods of this trait
    /// assume that the children do not contain any circular references. If they do,
    /// it will create an infinite loop.
    fn get_value_and_children_binary_iter_mut(
        &'a mut self,
    ) -> (Self::MutBorrowedValue, [Option<&'a mut Self>; 2]);

    /// This method gets the value and children from this node. The other
    /// methods of this trait assume that the 'Children' list does not contain
    /// any circular references. If there are, an infinite loop will result.
    fn get_value_and_children_iter_mut(
        &'a mut self,
    ) -> (Self::MutBorrowedValue, BinaryChildren<&'a mut Self>) {
        let (value, children) = self.get_value_and_children_binary_iter_mut();
        (
            value,
            children
                .into_iter()
                .flat_map(opt_to_opt as fn(Option<&'a mut Self>) -> Option<&'a mut Self>),
        )
    }

    #[doc = include_str!("../doc_files/at_path.md")]
    #[doc = include_str!("../doc_files/at_path_binary_example.md")]
    fn at_path_mut(&'a mut self, path: &[usize]) -> Option<&'a mut Self> {
        let mut current = self;
        for path_segment in path {
            current = current
                .get_value_and_children_binary_iter_mut()
                .1
                .into_iter()
                .nth(*path_segment)??;
        }
        Some(current)
    }

    /// This method retrieves an iterator that can be used to perform
    /// Breadth First (Queue - specifically VecDeque-based) searches of a tree.
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    #[must_use]
    fn bfs_iter_mut(&'a mut self) -> MutBorrowedBinaryBFSIterator<'a, Self> {
        MutBorrowedBinaryBFSIterator::new(self)
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    #[must_use]
    fn dfs_preorder_iter_mut(&'a mut self) -> MutBorrowedBinaryDFSPreorderIterator<'a, Self> {
        MutBorrowedBinaryDFSPreorderIterator::new(self)
    }

    /// This method retrieves an iterator that can be used to perform
    /// Depth First In Order searches of a tree.
    ///
    /// A Depth First In Order search is defined as:
    ///
    /// A tree traversal that involves depth-first searching a tree
    /// from the left to the right. Given a tree of the following shape,
    /// this traversal type would traverse
    /// the elements in the order
    /// 3, 1, 4, 0, 5, 2, 7, 9, 10, 8, 6.
    ///
    /// In this traversal, each node will be traversed after its left
    /// child and before its right child.
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    #[must_use]
    fn dfs_inorder_iter_mut(&'a mut self) -> MutBorrowedDFSInorderIterator<'a, Self> {
        MutBorrowedDFSInorderIterator::new(self)
    }

    /// This method retrieves an iterator that can be used to perform
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    ///
    /// This traversal type guarantees that getChildren() will only be
    /// called once per node of the tree.
    #[must_use]
    fn dfs_postorder_iter_mut(&'a mut self) -> MutBorrowedBinaryDFSPostorderIterator<'a, Self> {
        MutBorrowedBinaryDFSPostorderIterator::new(self)
    }

    /// This method converts the current BinaryTreeNode into a BinaryTreeIterator.
    ///
    /// BinaryTreeIterators have 2 purposes:
    /// 1. they serve as the internal piping of tree_iterators_rs
    /// 2. they can efficiently chain the prune, map, and fold operations on a tree.
    ///
    /// If you are only applying a single prune, map, or fold operation, just call the
    /// associated method.
    /// - [`prune_mut`](crate::prelude::MutBorrowedBinaryTreeNode::prune_mut)
    /// - [`map_mut`](crate::prelude::MutBorrowedBinaryTreeNode::map_mut)
    /// - [`fold_mut`](crate::prelude::MutBorrowedBinaryTreeNode::fold_mut)
    ///
    /// If you are chaining many operations together, use into_pipeline. This will
    /// be much more efficient in memory since it only maintains a single ancestor stack
    /// of the tree at a time.
    ///
    /// ### Example Usage:
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_binary_tree,
    ///     prelude::{BinaryTree, MutBorrowedBinaryTreeNode, TreeIteratorBase, BinaryTreeIterator}
    /// };
    ///
    /// let mut tree = create_example_binary_tree();
    /// let result = tree.into_pipeline_mut()
    ///     .prune_depth(2)
    ///     .map_tree(|value| *value + 200)
    ///     .collect_tree()
    ///     .expect("all non-prune methods to collect into a Some()");
    ///
    /// assert_eq!(
    ///     BinaryTree {
    ///         value: 200,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 201,
    ///             left: Some(Box::new(BinaryTree {
    ///                 value: 203,
    ///                 left: None,
    ///                 right: None,
    ///             })),
    ///             right: Some(Box::new(BinaryTree {
    ///                 value: 204,
    ///                 left: None,
    ///                 right: None,
    ///             })),
    ///         })),
    ///         right: Some(Box::new(BinaryTree {
    ///             value: 202,
    ///             left: Some(Box::new(BinaryTree {
    ///                 value: 205,
    ///                 left: None,
    ///                 right: None,
    ///             })),
    ///             right: Some(Box::new(BinaryTree {
    ///                 value: 206,
    ///                 left: None,
    ///                 right: None,
    ///             })),
    ///         })),
    ///     },
    ///     result
    /// );
    /// ```
    #[must_use]
    fn into_pipeline_mut(
        &'a mut self,
    ) -> impl BinaryTreeIterator<Self::MutBorrowedValue, [Option<&'a mut Self>; 2]> {
        MutBorrowedBinaryDFSPreorderIteratorWithPathTracking::new(self, Vec::new())
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the given closure to determine if each subtree in this tree should be pruned.
    ///
    /// Given an element the closure must return true or false. Any nodes in the tree for
    /// which this evaluates to true will be pruned out of the resulting tree. If the root node is pruned,
    /// `prune` will return [`None`].
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder`](crate::prelude::OwnedBinaryTreeNode::dfs_preorder) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, MutBorrowedBinaryTreeNode};
    ///
    /// let mut tree = BinaryTree {
    ///     value: 0,
    ///     left: Some(Box::new(BinaryTree {
    ///         value: 1,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 3,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: None,
    ///     })),
    ///     right: Some(Box::new(BinaryTree {
    ///         value: 2,
    ///         left: None,
    ///         right: None,
    ///     })),
    /// };
    ///
    /// let result = tree.prune_mut(|value| {
    ///     /// The output for this code would be the following. A couple notes about
    ///     /// this output:
    ///     /// 1. the node with a value of '1' has been removed
    ///     /// 2. the closure is never called on the node with a value of '3' since
    ///     ///    it is already determined to be pruned once '1' has been evaluated.
    ///     /// ```
    ///     /// 0
    ///     /// 1
    ///     /// 2
    ///     /// ```
    ///     println!("{value:?}");
    ///     **value == 1
    /// });
    ///
    /// assert_eq!(
    ///     Some(BinaryTree {
    ///         value: &mut 0,
    ///         left: None,
    ///         right: Some(
    ///             Box::new(BinaryTree {
    ///                 value: &mut 2,
    ///                 left: None,
    ///                 right: None,
    ///             }),
    ///         ),
    ///     }),
    ///     result
    /// );
    ///
    /// ```
    fn prune_mut<F>(&'a mut self, f: F) -> Option<BinaryTree<Self::MutBorrowedValue>>
    where
        F: FnMut(&Self::MutBorrowedValue) -> bool,
    {
        self.into_pipeline_mut().prune(f).collect_tree()
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the given closure to determine if each subtree in this tree should be pruned.
    ///
    /// Given an element and its context in the tree, the closure must return true or false.
    /// Any nodes in the tree for which this evaluates to true will be pruned out of the resulting
    /// tree. If the root node is pruned, `prune` will return [`None`].
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder`](crate::prelude::MutBorrowedTreeNode::dfs_preorder_iter_mut) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, MutBorrowedBinaryTreeNode};
    ///
    /// let mut tree = BinaryTree {
    ///     value: 0,
    ///     left: Some(Box::new(BinaryTree {
    ///         value: 1,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 3,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: None,
    ///     })),
    ///     right: Some(Box::new(BinaryTree {
    ///         value: 2,
    ///         left: None,
    ///         right: None,
    ///     }))
    /// };
    ///
    /// let result = tree.prune_path_mut(|path, value| {
    ///     /// The output for this code would be the following. A couple notes about
    ///     /// this output:
    ///     /// 1. the node with a value of '1' has been removed
    ///     /// 2. the closure is never called on the node with a value of '3' since
    ///     /// it is already determined to be pruned once '1' has been evaluated.
    ///     /// ```
    ///     /// [0]; 0
    ///     /// [0, 0]; 1
    ///     /// [0, 1]; 2
    ///     /// ```
    ///     println!("{:?}; {:?}", path, value);
    ///     **value == 1
    /// });
    ///
    /// assert_eq!(
    ///     Some(BinaryTree {
    ///         value: &mut 0,
    ///         left: None,
    ///         right: Some(Box::new(BinaryTree {
    ///             value: &mut 2,
    ///             left: None,
    ///             right: None,
    ///         }))
    ///     }),
    ///     result
    /// );
    /// ```
    fn prune_path_mut<F>(&'a mut self, f: F) -> Option<BinaryTree<Self::MutBorrowedValue>>
    where
        F: FnMut(&[usize], &Self::MutBorrowedValue) -> bool,
    {
        self.into_pipeline_mut().prune_path(f).collect_tree()
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the depth of each subtree to determine if the subtree should be pruned.
    /// Any node with a depth that is strictly greater than the max_depth parameter
    /// will be pruned from the tree.
    ///
    /// Depth is zero-based, so the root node is considered to be at depth zero.
    ///
    /// Ex. given a tree like the following, the depths would be as labeled.
    ///
    /// ```text
    ///        0       <- depth: 0
    ///       / \
    ///      1   2     <- depth: 1
    ///     / \ / \
    ///    3  4 5  6   <- depth: 2
    ///           /
    ///          7     <- depth: 3
    ///           \
    ///            8   <- depth: 4
    ///           /
    ///          9     <- depth: 5
    ///           \
    ///           10   <- depth: 6
    /// ```
    ///
    /// ### Basic usage:
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, MutBorrowedBinaryTreeNode};
    ///
    /// let mut tree = BinaryTree {
    ///     value: 0,
    ///     left: Some(Box::new(BinaryTree {
    ///         value: 1,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 3,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: None,
    ///     })),
    ///     right: Some(Box::new(BinaryTree {
    ///         value: 2,
    ///         left: None,
    ///         right: None,
    ///     }))
    /// };
    ///
    /// assert_eq!(
    ///     BinaryTree {
    ///         value: &mut 0,
    ///         left: None,
    ///         right: None,
    ///     },
    ///     tree.prune_depth_mut(0)
    /// );
    /// ```
    fn prune_depth_mut(&'a mut self, max_depth: usize) -> BinaryTree<Self::MutBorrowedValue> {
        self.into_pipeline_mut()
            .prune_depth(max_depth)
            .collect_tree()
            .unwrap()
    }

    /// map is a tree-based analog to [map](core::iter::Iterator::map).
    ///
    /// Takes a closure and applies that closure to each node's value in the tree.
    ///
    /// map() transforms one tree into another, by means of its argument: something that
    /// implements FnMut. It produces a new tree which calls this closure on each node of
    /// the original tree.
    ///
    /// If you are good at thinking in types, you can think of map() like this: If you
    /// have a tree that has elements of some type A, and you want a tree of some other
    /// type B, you can use map(), passing a closure that takes an A and returns a B.
    ///
    /// ### Example Usage
    ///
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_binary_tree,
    ///     prelude::{BinaryTree, MutBorrowedBinaryTreeNode}
    /// };
    ///
    /// let mut tree = BinaryTree {
    ///     value: "0-0",
    ///     left: Some(Box::new(BinaryTree {
    ///         value: "1-1",
    ///         left: None,
    ///         right: None,
    ///     })),
    ///     right: Some(Box::new(BinaryTree {
    ///         value: "2-2",
    ///         left: None,
    ///         right: None,
    ///     })),
    /// };
    ///
    /// let result = tree.map_mut(|value: &mut &'static str| {
    ///     value.split("-").nth(1).unwrap().to_string()
    /// });
    ///
    /// assert_eq!(
    ///     BinaryTree {
    ///         value: "0".to_string(),
    ///         left: Some(Box::new(BinaryTree {
    ///             value: "1".to_string(),
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: Some(Box::new(BinaryTree {
    ///             value: "2".to_string(),
    ///             left: None,
    ///             right: None,
    ///         })),
    ///     },
    ///     result);
    /// ```
    fn map_mut<Output, F>(&'a mut self, f: F) -> BinaryTree<Output>
    where
        F: FnMut(Self::MutBorrowedValue) -> Output,
    {
        self.into_pipeline_mut().map_tree(f).collect_tree().unwrap()
    }

    /// fold is a tree-based analog to [fold](core::iter::Iterator::fold).
    ///
    /// Folds every element into an accumulation by applying an operation, returning the
    /// final result.
    ///
    /// fold() takes one argument: a closure with two arguments: the result of accumulating
    /// all children of the current tree node, and an element. The closure returns the value
    /// that the accumulator should have for the parent node's accumulation.
    ///
    /// After applying this closure to every node of the tree, fold() returns the accumulation.
    ///
    /// This operation is sometimes called ‘reduce’ or ‘inject’.
    ///
    /// Folding is useful whenever you have a tree of something, and want to produce a single
    /// value from it.
    ///
    /// ### Example Usage
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_binary_tree,
    ///     prelude::MutBorrowedBinaryTreeNode
    /// };
    ///
    /// let mut tree = create_example_binary_tree();
    /// let accumulation = tree.fold_mut(|child_accumulations: [Option<usize>; 2], value| {
    ///     child_accumulations
    ///         .into_iter()
    ///         .map(|opt| opt.unwrap_or_default())
    ///         .sum::<usize>()
    ///     + *value
    /// });
    ///
    /// assert_eq!(55, accumulation);
    /// ```
    fn fold_mut<Output, F>(&'a mut self, f: F) -> Output
    where
        F: FnMut([Option<Output>; 2], Self::MutBorrowedValue) -> Output,
    {
        self.into_pipeline_mut().fold_tree(f).unwrap()
    }
}

/// A tree node where getting its children mutably borrows its value.
pub trait MutBorrowedTreeNode<'a>
where
    Self: Sized + 'a,
{
    /// A mutable reference to the value of each node in the tree.
    type MutBorrowedValue: Sized;

    /// The type of iterator that can be used to iterate over each node's children
    /// collection.
    type MutBorrowedChildren: IntoIterator<Item = &'a mut Self, IntoIter: FusedIterator>;

    /// This method gets the value and children from this node. The other
    /// methods of this trait assume that the 'Children' list does not contain
    /// any circular references. If there are, an infinite loop will result.
    fn get_value_and_children_iter_mut(
        &'a mut self,
    ) -> (Self::MutBorrowedValue, Self::MutBorrowedChildren);

    #[doc = include_str!("../doc_files/at_path.md")]
    #[doc = include_str!("../doc_files/at_path_tree_example.md")]
    fn at_path_mut(&'a mut self, path: &[usize]) -> Option<&'a mut Self> {
        let mut current = self;
        for path_segment in path {
            current = current
                .get_value_and_children_iter_mut()
                .1
                .into_iter()
                .nth(*path_segment)?;
        }
        Some(current)
    }

    /// This method retrieves an iterator that can be used to perform
    /// Breadth First (VecDeque-based) searches of a tree.
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    #[must_use]
    fn bfs_iter_mut(&'a mut self) -> MutBorrowedBFSIterator<'a, Self> {
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    #[must_use]
    fn dfs_preorder_iter_mut(&'a mut self) -> MutBorrowedDFSPreorderIterator<'a, Self> {
        MutBorrowedDFSPreorderIterator::new(self)
    }

    /// This method retrieves an iterator that can be used to perform
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    ///
    /// This traversal type guarantees that getChildren() will only be
    /// called once per node of the tree.
    #[must_use]
    fn dfs_postorder_iter_mut(&'a mut self) -> MutBorrowedDFSPostorderIterator<'a, Self> {
        MutBorrowedDFSPostorderIterator::new(self)
    }

    /// This method converts the current TreeNode into a TreeIterator.
    ///
    /// TreeIterators have 2 purposes:
    /// 1. they serve as the internal piping of tree_iterators_rs
    /// 2. they can efficiently chain the prune, map, and fold operations on a tree.
    ///
    /// If you are only applying a single prune, map, or fold operation, just call the
    /// associated method.
    /// - [`prune_mut`](crate::prelude::MutBorrowedTreeNode::prune_mut)
    /// - [`map_mut`](crate::prelude::MutBorrowedTreeNode::map_mut)
    /// - [`fold_mut`](crate::prelude::MutBorrowedTreeNode::fold_mut)
    ///
    /// If you are chaining many operations together, use into_pipeline. This will
    /// be much more efficient in memory since it only maintains a single ancestor stack
    /// of the tree at a time.
    ///
    /// ### Example Usage:
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_tree,
    ///     prelude::{Tree, MutBorrowedTreeNode, TreeIteratorBase, TreeIterator}
    /// };
    ///
    /// let mut tree = create_example_tree();
    /// let result = tree.into_pipeline_mut()
    ///     .prune_depth(2)
    ///     .map_tree(|value| *value + 200)
    ///     .collect_tree()
    ///     .expect("all non-prune methods to collect into a Some()");
    ///
    /// assert_eq!(
    ///     Tree {
    ///        value: 200,
    ///        children: vec![
    ///            Tree {
    ///                value: 201,
    ///                children: vec![
    ///                    Tree {
    ///                        value: 203,
    ///                        children: vec![],
    ///                    },
    ///                    Tree {
    ///                        value: 204,
    ///                        children: vec![],
    ///                    },
    ///                ],
    ///            },
    ///            Tree {
    ///                value: 202,
    ///                children: vec![
    ///                    Tree {
    ///                        value: 205,
    ///                        children: vec![],
    ///                    },
    ///                    Tree {
    ///                        value: 206,
    ///                        children: vec![],
    ///                    },
    ///                ],
    ///            },
    ///        ],
    ///     },
    ///     result);
    /// ```
    #[must_use]
    fn into_pipeline_mut(
        &'a mut self,
    ) -> impl TreeIterator<Self::MutBorrowedValue, Self::MutBorrowedChildren> {
        MutBorrowedDFSPreorderIteratorWithPathTracking::new(self, Vec::new())
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the given closure to determine if each subtree in this tree should be pruned.
    ///
    /// Given an element the closure must return true or false. Any nodes in the tree for
    /// which this evaluates to true will be pruned out of the resulting tree. If the root
    /// node is pruned, this will return [`None`].
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder`](crate::prelude::OwnedTreeNode::dfs_preorder) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{MutBorrowedTreeNode, Tree};
    ///
    /// let mut tree = Tree {
    ///     value: 0,
    ///     children: vec![
    ///         Tree {
    ///             value: 1,
    ///             children: vec![Tree {
    ///                 value: 3,
    ///                 children: Vec::new(),
    ///             }],
    ///         },
    ///         Tree {
    ///             value: 2,
    ///             children: Vec::new()
    ///         },
    ///     ],
    /// };
    ///
    /// assert_eq!(
    ///     Some(
    ///         Tree {
    ///             value: &mut 0,
    ///             children: vec![
    ///                 Tree {
    ///                     value: &mut 2,
    ///                     children: Vec::new(),
    ///                 }
    ///             ],
    ///         },
    ///     ),
    ///     tree.prune_mut(|value| {
    ///         /// The output for this code would be the following. A couple notes about
    ///         /// this output:
    ///         /// 1. the node with a value of '1' has been removed
    ///         /// 2. the closure is never called on the node with a value of '3' since
    ///         ///    it is already determined to be pruned once '1' has been evaluated.
    ///         /// ```
    ///         /// 0
    ///         /// 1
    ///         /// 2
    ///         /// ```
    ///         println!("{value:?}");
    ///         **value == 1
    ///     })
    /// );
    /// ```
    fn prune_mut<F>(&'a mut self, f: F) -> Option<Tree<Self::MutBorrowedValue>>
    where
        F: FnMut(&Self::MutBorrowedValue) -> bool,
    {
        self.into_pipeline_mut().prune(f).collect_tree()
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the given closure to determine if each subtree in this tree should be pruned.
    ///
    /// Given the path of an element and the element's value, the closure must return true or
    /// false. Any nodes in the tree for which this evaluates to true will be pruned out of
    /// the resulting tree. If the root node is pruned, this will return [`None`].
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder`](crate::prelude::OwnedTreeNode::dfs_preorder) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{MutBorrowedTreeNode, Tree};
    ///
    /// let mut tree = Tree {
    ///     value: 0,
    ///     children: vec![
    ///         Tree {
    ///             value: 1,
    ///             children: vec![Tree {
    ///                 value: 3,
    ///                 children: Vec::new(),
    ///             }],
    ///         },
    ///         Tree {
    ///             value: 2,
    ///             children: Vec::new()
    ///         },
    ///     ],
    /// };
    ///
    /// assert_eq!(
    ///     Some(
    ///         Tree {
    ///             value: &mut 0,
    ///             children: vec![
    ///                 Tree {
    ///                     value: &mut 2,
    ///                     children: Vec::new(),
    ///                 }
    ///             ],
    ///         },
    ///     ),
    ///     tree.prune_path_mut(|path, value| {
    ///         /// The output for this code would be the following. A couple notes about
    ///         /// this output:
    ///         /// 1. the node with a value of '1' has been removed
    ///         /// 2. the closure is never called on the node with a value of '3' since
    ///         ///    it is already determined to be pruned once '1' has been evaluated.
    ///         /// ```
    ///         /// 0
    ///         /// 1
    ///         /// 2
    ///         /// ```
    ///         println!("{value:?}");
    ///         matches!(path.get(0), Some(0))
    ///     })
    /// );
    /// ```
    fn prune_path_mut<F>(&'a mut self, f: F) -> Option<Tree<Self::MutBorrowedValue>>
    where
        F: FnMut(&[usize], &Self::MutBorrowedValue) -> bool,
    {
        self.into_pipeline_mut().prune_path(f).collect_tree()
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the depth of each subtree to determine if the subtree should be pruned.
    /// Any node with a depth that is strictly greater than the max_depth parameter
    /// will be pruned from the tree.
    ///
    /// Depth is zero-based, so the root node is considered to be at depth zero.
    ///
    /// Ex. given a tree like the following, the depths would be as labeled.
    ///
    /// ```text
    ///        0       <- depth: 0
    ///       / \
    ///      1   2     <- depth: 1
    ///     / \ / \
    ///    3  4 5  6   <- depth: 2
    ///           /
    ///          7     <- depth: 3
    ///           \
    ///            8   <- depth: 4
    ///           /
    ///          9     <- depth: 5
    ///           \
    ///           10   <- depth: 6
    /// ```
    ///
    /// ### Basic usage:
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, MutBorrowedTreeNode};
    ///
    /// let mut tree = Tree {
    ///     value: 0,
    ///     children: vec![
    ///         Tree {
    ///             value: 1,
    ///             children: vec![Tree {
    ///                 value: 3,
    ///                 children: vec![],
    ///             }],
    ///         },
    ///         Tree {
    ///             value: 2,
    ///             children: vec![],
    ///         },
    ///     ],
    /// };
    ///
    /// assert_eq!(
    ///     Tree {
    ///         value: &mut 0,
    ///         children: vec![],
    ///     },
    ///     tree.prune_depth_mut(0)
    /// );
    /// ```
    fn prune_depth_mut(&'a mut self, max_depth: usize) -> Tree<Self::MutBorrowedValue> {
        self.into_pipeline_mut()
            .prune_depth(max_depth)
            .collect_tree()
            .unwrap()
    }

    /// map is a tree-based analog to [map](core::iter::Iterator::map).
    ///
    /// Takes a closure and applies that closure to each node's value in the tree.
    ///
    /// map() transforms one tree into another, by means of its argument: something that
    /// implements FnMut. It produces a new tree which calls this closure on each node of
    /// the original tree.
    ///
    /// If you are good at thinking in types, you can think of map() like this: If you
    /// have a tree that has elements of some type A, and you want a tree of some other
    /// type B, you can use map(), passing a closure that takes an A and returns a B.
    ///
    /// ### Example Usage
    ///
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_binary_tree,
    ///     prelude::{Tree, MutBorrowedTreeNode}
    /// };
    ///
    /// let mut tree = Tree {
    ///     value: "0-0",
    ///     children: vec![
    ///         Tree {
    ///             value: "1-1",
    ///             children: vec![],
    ///         },
    ///         Tree {
    ///             value: "2-2",
    ///             children: vec![],
    ///         }
    ///     ],
    /// };
    ///
    /// let result = tree.map_mut(|value: &mut &'static str| {
    ///     value.split("-").nth(1).unwrap().to_string()
    /// });
    ///
    /// assert_eq!(
    ///     Tree {
    ///         value: "0".to_string(),
    ///         children: vec![
    ///             Tree {
    ///                 value: "1".to_string(),
    ///                 children: vec![],
    ///             },
    ///             Tree {
    ///                 value: "2".to_string(),
    ///                 children: vec![],
    ///             },
    ///         ],
    ///     },
    ///     result);
    /// ```
    fn map_mut<Output, F>(&'a mut self, f: F) -> Tree<Output>
    where
        F: FnMut(Self::MutBorrowedValue) -> Output,
    {
        self.into_pipeline_mut().map_tree(f).collect_tree().unwrap()
    }

    /// fold is a tree-based analog to [fold](core::iter::Iterator::fold).
    ///
    /// Folds every element into an accumulation by applying an operation, returning the
    /// final result.
    ///
    /// fold() takes one argument: a closure with two arguments: the result of accumulating
    /// all children of the current tree node, and an element. The closure returns the value
    /// that the accumulator should have for the parent node's accumulation.
    ///
    /// After applying this closure to every node of the tree, fold() returns the accumulation.
    ///
    /// This operation is sometimes called ‘reduce’ or ‘inject’.
    ///
    /// Folding is useful whenever you have a tree of something, and want to produce a single
    /// value from it.
    ///
    /// ### Example Usage
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_tree,
    ///     prelude::MutBorrowedTreeNode
    /// };
    ///
    /// let mut tree = create_example_tree();
    /// let accumulation = tree.fold_mut(|child_accumulations: Vec<usize>, value| {
    ///     child_accumulations
    ///         .into_iter()
    ///         .sum::<usize>()
    ///     + *value
    /// });
    ///
    /// assert_eq!(55, accumulation);
    /// ```
    fn fold_mut<Output, F>(&'a mut self, f: F) -> Output
    where
        F: FnMut(Vec<Output>, Self::MutBorrowedValue) -> Output,
    {
        self.into_pipeline_mut().fold_tree(f).unwrap()
    }
}

/// A binary tree node where getting its children borrows its value.
pub trait BorrowedBinaryTreeNode<'a>
where
    Self: Sized + 'a,
{
    /// A reference to the value of each node in the tree.
    type BorrowedValue;

    /// This method gets the value and left and right children from this node,
    /// borrowing it in the process. The other methods of this trait
    /// assume that the children do not contain any circular references. If they do,
    /// it will create an infinite loop.
    fn get_value_and_children_binary_iter(&'a self)
        -> (Self::BorrowedValue, [Option<&'a Self>; 2]);

    /// This method gets the value and children from this node, consuming it
    /// in the process. The other methods of this trait assume that the 'Children'
    /// list does not contain and circular references back to parent nodes.
    fn get_value_and_children_iter(&'a self) -> (Self::BorrowedValue, BinaryChildren<&'a Self>) {
        let (value, children) = self.get_value_and_children_binary_iter();
        (
            value,
            children
                .into_iter()
                .flat_map(opt_to_opt as fn(Option<&'a Self>) -> Option<&'a Self>),
        )
    }

    #[doc = include_str!("../doc_files/at_path.md")]
    #[doc = include_str!("../doc_files/at_path_binary_example.md")]
    fn at_path_ref(&'a self, path: &[usize]) -> Option<&'a Self> {
        let mut current = self;
        for path_segment in path {
            current = current
                .get_value_and_children_binary_iter()
                .1
                .into_iter()
                .nth(*path_segment)??;
        }
        Some(current)
    }

    /// This method retrieves an iterator that can be used to perform
    /// Breadth First (Queue - specifically VecDeque-based) searches of a tree.
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    #[must_use]
    fn bfs_iter(&'a self) -> BorrowedBinaryBFSIterator<'a, Self> {
        BorrowedBinaryBFSIterator::new(self)
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    #[must_use]
    fn dfs_preorder_iter(&'a self) -> BorrowedBinaryDFSPreorderIterator<'a, Self> {
        BorrowedBinaryDFSPreorderIterator::new(self)
    }

    /// This method retrieves an iterator that can be used to perform
    /// Depth First In Order searches of a tree.
    ///
    /// A Depth First In Order search is defined as:
    ///
    /// A tree traversal that involves depth-first searching a tree
    /// from the left to the right. Given a tree of the following shape,
    /// this traversal type would traverse
    /// the elements in the order
    /// 3, 1, 4, 0, 5, 2, 7, 9, 10, 8, 6.
    ///
    /// In this traversal, each node will be traversed after its left
    /// child and before its right child.
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    #[must_use]
    fn dfs_inorder_iter(&'a self) -> BorrowedDFSInorderIterator<'a, Self> {
        BorrowedDFSInorderIterator::new(self)
    }

    /// This method retrieves an iterator that can be used to perform
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    ///
    /// This traversal type guarantees that getChildren() will only be
    /// called once per node of the tree.
    #[must_use]
    fn dfs_postorder_iter(&'a self) -> BorrowedBinaryDFSPostorderIterator<'a, Self> {
        BorrowedBinaryDFSPostorderIterator::new(self)
    }

    /// This method converts the current BinaryTreeNode into a BinaryTreeIterator.
    ///
    /// BinaryTreeIterators have 2 purposes:
    /// 1. they serve as the internal piping of tree_iterators_rs
    /// 2. they can efficiently chain the prune, map, and fold operations on a tree.
    ///
    /// If you are only applying a single prune, map, or fold operation, just call the
    /// associated method.
    /// - [`prune_ref`](crate::prelude::BorrowedBinaryTreeNode::prune_ref)
    /// - [`map_mut`](crate::prelude::BorrowedBinaryTreeNode::map_ref)
    /// - [`fold_mut`](crate::prelude::BorrowedBinaryTreeNode::fold_ref)
    ///
    /// If you are chaining many operations together, use into_pipeline. This will
    /// be much more efficient in memory since it only maintains a single ancestor stack
    /// of the tree at a time.
    ///
    /// ### Example Usage:
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_binary_tree,
    ///     prelude::{BinaryTree, BorrowedBinaryTreeNode, TreeIteratorBase, BinaryTreeIterator}
    /// };
    ///
    /// let tree = create_example_binary_tree();
    /// let result = tree.into_pipeline_ref()
    ///     .prune_depth(2)
    ///     .map_tree(|value| *value + 200)
    ///     .collect_tree()
    ///     .expect("all non-prune methods to collect into a Some()");
    ///
    /// assert_eq!(
    ///     BinaryTree {
    ///         value: 200,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 201,
    ///             left: Some(Box::new(BinaryTree {
    ///                 value: 203,
    ///                 left: None,
    ///                 right: None,
    ///             })),
    ///             right: Some(Box::new(BinaryTree {
    ///                 value: 204,
    ///                 left: None,
    ///                 right: None,
    ///             })),
    ///         })),
    ///         right: Some(Box::new(BinaryTree {
    ///             value: 202,
    ///             left: Some(Box::new(BinaryTree {
    ///                 value: 205,
    ///                 left: None,
    ///                 right: None,
    ///             })),
    ///             right: Some(Box::new(BinaryTree {
    ///                 value: 206,
    ///                 left: None,
    ///                 right: None,
    ///             })),
    ///         })),
    ///     },
    ///     result
    /// );
    /// ```
    #[must_use]
    fn into_pipeline_ref(
        &'a self,
    ) -> impl BinaryTreeIterator<Self::BorrowedValue, [Option<&'a Self>; 2]> {
        BorrowedBinaryDFSPreorderIteratorWithPathTracking::new(self, Vec::new())
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the given closure to determine if each subtree in this tree should be pruned.
    ///
    /// Given an element the closure must return true or false. Any nodes in the tree for
    /// which this evaluates to true will be pruned out of the resulting tree. If the root node is pruned,
    /// `prune` will return [`None`].
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder`](crate::prelude::OwnedBinaryTreeNode::dfs_preorder) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, BorrowedBinaryTreeNode};
    ///
    /// let tree = BinaryTree {
    ///     value: 0,
    ///     left: Some(Box::new(BinaryTree {
    ///         value: 1,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 3,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: None,
    ///     })),
    ///     right: Some(Box::new(BinaryTree {
    ///         value: 2,
    ///         left: None,
    ///         right: None,
    ///     })),
    /// };
    ///
    /// let result = tree.prune_ref(|value| {
    ///     /// The output for this code would be the following. A couple notes about
    ///     /// this output:
    ///     /// 1. the node with a value of '1' has been removed
    ///     /// 2. the closure is never called on the node with a value of '3' since
    ///     ///    it is already determined to be pruned once '1' has been evaluated.
    ///     /// ```
    ///     /// 0
    ///     /// 1
    ///     /// 2
    ///     /// ```
    ///     println!("{value:?}");
    ///     **value == 1
    /// });
    ///
    /// assert_eq!(
    ///     Some(BinaryTree {
    ///         value: &0,
    ///         left: None,
    ///         right: Some(
    ///             Box::new(BinaryTree {
    ///                 value: &2,
    ///                 left: None,
    ///                 right: None,
    ///             }),
    ///         ),
    ///     }),
    ///     result
    /// );
    ///
    /// ```
    fn prune_ref<F>(&'a self, f: F) -> Option<BinaryTree<Self::BorrowedValue>>
    where
        F: FnMut(&Self::BorrowedValue) -> bool,
    {
        self.into_pipeline_ref().prune(f).collect_tree()
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the given closure to determine if each subtree in this tree should be pruned.
    ///
    /// Given an element and its context in the tree, the closure must return true or false.
    /// Any nodes in the tree for which this evaluates to true will be pruned out of the resulting
    /// tree. If the root node is pruned, `prune` will return [`None`].
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder`](crate::prelude::MutBorrowedTreeNode::dfs_preorder_iter_mut) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, BorrowedBinaryTreeNode};
    ///
    /// let tree = BinaryTree {
    ///     value: 0,
    ///     left: Some(Box::new(BinaryTree {
    ///         value: 1,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 3,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: None,
    ///     })),
    ///     right: Some(Box::new(BinaryTree {
    ///         value: 2,
    ///         left: None,
    ///         right: None,
    ///     }))
    /// };
    ///
    /// let result = tree.prune_path_ref(|path, value| {
    ///     /// The output for this code would be the following. A couple notes about
    ///     /// this output:
    ///     /// 1. the node with a value of '1' has been removed
    ///     /// 2. the closure is never called on the node with a value of '3' since
    ///     /// it is already determined to be pruned once '1' has been evaluated.
    ///     /// ```
    ///     /// [0]; 0
    ///     /// [0, 0]; 1
    ///     /// [0, 1]; 2
    ///     /// ```
    ///     println!("{:?}; {:?}", path, value);
    ///     **value == 1
    /// });
    ///
    /// assert_eq!(
    ///     Some(BinaryTree {
    ///         value: &0,
    ///         left: None,
    ///         right: Some(Box::new(BinaryTree {
    ///             value: &2,
    ///             left: None,
    ///             right: None,
    ///         }))
    ///     }),
    ///     result
    /// );
    /// ```
    fn prune_path_ref<F>(&'a self, f: F) -> Option<BinaryTree<Self::BorrowedValue>>
    where
        F: FnMut(&[usize], &Self::BorrowedValue) -> bool,
    {
        self.into_pipeline_ref().prune_path(f).collect_tree()
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the depth of each subtree to determine if the subtree should be pruned.
    /// Any node with a depth that is strictly greater than the max_depth parameter
    /// will be pruned from the tree.
    ///
    /// Depth is zero-based, so the root node is considered to be at depth zero.
    ///
    /// Ex. given a tree like the following, the depths would be as labeled.
    ///
    /// ```text
    ///        0       <- depth: 0
    ///       / \
    ///      1   2     <- depth: 1
    ///     / \ / \
    ///    3  4 5  6   <- depth: 2
    ///           /
    ///          7     <- depth: 3
    ///           \
    ///            8   <- depth: 4
    ///           /
    ///          9     <- depth: 5
    ///           \
    ///           10   <- depth: 6
    /// ```
    ///
    /// ### Basic usage:
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, BorrowedBinaryTreeNode};
    ///
    /// let tree = BinaryTree {
    ///     value: 0,
    ///     left: Some(Box::new(BinaryTree {
    ///         value: 1,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 3,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: None,
    ///     })),
    ///     right: Some(Box::new(BinaryTree {
    ///         value: 2,
    ///         left: None,
    ///         right: None,
    ///     }))
    /// };
    ///
    /// assert_eq!(
    ///     BinaryTree {
    ///         value: &0,
    ///         left: None,
    ///         right: None,
    ///     },
    ///     tree.prune_depth_ref(0)
    /// );
    /// ```
    fn prune_depth_ref(&'a self, max_depth: usize) -> BinaryTree<Self::BorrowedValue> {
        self.into_pipeline_ref()
            .prune_depth(max_depth)
            .collect_tree()
            .unwrap()
    }

    /// map is a tree-based analog to [map](core::iter::Iterator::map).
    ///
    /// Takes a closure and applies that closure to each node's value in the tree.
    ///
    /// map() transforms one tree into another, by means of its argument: something that
    /// implements FnMut. It produces a new tree which calls this closure on each node of
    /// the original tree.
    ///
    /// If you are good at thinking in types, you can think of map() like this: If you
    /// have a tree that has elements of some type A, and you want a tree of some other
    /// type B, you can use map(), passing a closure that takes an A and returns a B.
    ///
    /// ### Example Usage
    ///
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_binary_tree,
    ///     prelude::{BinaryTree, BorrowedBinaryTreeNode}
    /// };
    ///
    /// let tree = BinaryTree {
    ///     value: "0-0",
    ///     left: Some(Box::new(BinaryTree {
    ///         value: "1-1",
    ///         left: None,
    ///         right: None,
    ///     })),
    ///     right: Some(Box::new(BinaryTree {
    ///         value: "2-2",
    ///         left: None,
    ///         right: None,
    ///     })),
    /// };
    ///
    /// let result = tree.map_ref(|value: &&'static str| {
    ///     value.split("-").nth(1).unwrap().to_string()
    /// });
    ///
    /// assert_eq!(
    ///     BinaryTree {
    ///         value: "0".to_string(),
    ///         left: Some(Box::new(BinaryTree {
    ///             value: "1".to_string(),
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: Some(Box::new(BinaryTree {
    ///             value: "2".to_string(),
    ///             left: None,
    ///             right: None,
    ///         })),
    ///     },
    ///     result);
    /// ```
    fn map_ref<Output, F>(&'a self, f: F) -> BinaryTree<Output>
    where
        F: FnMut(Self::BorrowedValue) -> Output,
    {
        self.into_pipeline_ref().map_tree(f).collect_tree().unwrap()
    }

    /// fold is a tree-based analog to [fold](core::iter::Iterator::fold).
    ///
    /// Folds every element into an accumulation by applying an operation, returning the
    /// final result.
    ///
    /// fold() takes one argument: a closure with two arguments: the result of accumulating
    /// all children of the current tree node, and an element. The closure returns the value
    /// that the accumulator should have for the parent node's accumulation.
    ///
    /// After applying this closure to every node of the tree, fold() returns the accumulation.
    ///
    /// This operation is sometimes called ‘reduce’ or ‘inject’.
    ///
    /// Folding is useful whenever you have a tree of something, and want to produce a single
    /// value from it.
    ///
    /// ### Example Usage
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_binary_tree,
    ///     prelude::BorrowedBinaryTreeNode
    /// };
    ///
    /// let tree = create_example_binary_tree();
    /// let accumulation = tree.fold_ref(|child_accumulations: [Option<usize>; 2], value| {
    ///     child_accumulations
    ///         .into_iter()
    ///         .map(|opt| opt.unwrap_or_default())
    ///         .sum::<usize>()
    ///     + *value
    /// });
    ///
    /// assert_eq!(55, accumulation);
    /// ```
    fn fold_ref<Output, F>(&'a self, f: F) -> Output
    where
        F: FnMut([Option<Output>; 2], Self::BorrowedValue) -> Output,
    {
        self.into_pipeline_ref().fold_tree(f).unwrap()
    }
}

/// A tree node where getting its children borrows its value.
pub trait BorrowedTreeNode<'a>
where
    Self: Sized + 'a,
{
    /// A reference to the value of each node in the tree.
    type BorrowedValue: Sized;
    /// The type of iterator that can be used to iterate over each node's children
    /// collection.
    type BorrowedChildren: IntoIterator<Item = &'a Self, IntoIter: FusedIterator>;

    /// This method gets the value and children from this node, consuming it
    /// in the process. The other methods of this trait assume that the 'Children'
    /// list does not contain and circular references back to parent nodes.
    fn get_value_and_children_iter(&'a self) -> (Self::BorrowedValue, Self::BorrowedChildren);

    #[doc = include_str!("../doc_files/at_path.md")]
    #[doc = include_str!("../doc_files/at_path_tree_example.md")]
    fn at_path_ref(&'a self, path: &[usize]) -> Option<&'a Self> {
        let mut current = self;
        for path_segment in path {
            current = current
                .get_value_and_children_iter()
                .1
                .into_iter()
                .nth(*path_segment)?;
        }
        Some(current)
    }

    /// This method retrieves an iterator that can be used to perform
    /// Breadth First (Queue - specifically VecDeque-based) searches of a tree.
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    #[must_use]
    fn bfs_iter(&'a self) -> BorrowedBFSIterator<'a, Self> {
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    #[must_use]
    fn dfs_preorder_iter(&'a self) -> BorrowedDFSPreorderIterator<'a, Self> {
        BorrowedDFSPreorderIterator::new(self)
    }

    /// This method retrieves an iterator that can be used to perform
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
    /// ```text
    ///        0
    ///       / \
    ///      1   2
    ///     / \ / \
    ///    3  4 5  6
    ///           /
    ///          7
    ///           \
    ///            8
    ///           /
    ///          9
    ///           \
    ///           10
    /// ```
    ///
    /// This traversal type guarantees that getChildren() will only be
    /// called once per node of the tree.
    #[must_use]
    fn dfs_postorder_iter(&'a self) -> BorrowedDFSPostorderIterator<'a, Self> {
        BorrowedDFSPostorderIterator::new(self)
    }

    /// This method converts the current TreeNode into a TreeIterator.
    ///
    /// TreeIterators have 2 purposes:
    /// 1. they serve as the internal piping of tree_iterators_rs
    /// 2. they can efficiently chain the prune, map, and fold operations on a tree.
    ///
    /// If you are only applying a single prune, map, or fold operation, just call the
    /// associated method.
    /// - [`prune_ref`](crate::prelude::BorrowedTreeNode::prune_ref)
    /// - [`map_ref`](crate::prelude::BorrowedTreeNode::map_ref)
    /// - [`fold_ref`](crate::prelude::BorrowedTreeNode::fold_ref)
    ///
    /// If you are chaining many operations together, use into_pipeline. This will
    /// be much more efficient in memory since it only maintains a single ancestor stack
    /// of the tree at a time.
    ///
    /// ### Example Usage:
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_tree,
    ///     prelude::{Tree, BorrowedTreeNode, TreeIteratorBase, TreeIterator}
    /// };
    ///
    /// let tree = create_example_tree();
    /// let result = tree.into_pipeline_ref()
    ///     .prune_depth(2)
    ///     .map_tree(|value| *value + 200)
    ///     .collect_tree()
    ///     .expect("all non-prune methods to collect into a Some()");
    ///
    /// assert_eq!(
    ///     Tree {
    ///        value: 200,
    ///        children: vec![
    ///            Tree {
    ///                value: 201,
    ///                children: vec![
    ///                    Tree {
    ///                        value: 203,
    ///                        children: vec![],
    ///                    },
    ///                    Tree {
    ///                        value: 204,
    ///                        children: vec![],
    ///                    },
    ///                ],
    ///            },
    ///            Tree {
    ///                value: 202,
    ///                children: vec![
    ///                    Tree {
    ///                        value: 205,
    ///                        children: vec![],
    ///                    },
    ///                    Tree {
    ///                        value: 206,
    ///                        children: vec![],
    ///                    },
    ///                ],
    ///            },
    ///        ],
    ///     },
    ///     result);
    /// ```
    #[must_use]
    fn into_pipeline_ref(
        &'a self,
    ) -> impl TreeIterator<Self::BorrowedValue, Self::BorrowedChildren> {
        BorrowedDFSPreorderIteratorWithPathTracking::new(self, Vec::new())
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the given closure to determine if each subtree in this tree should be pruned.
    ///
    /// Given an element the closure must return true or false. Any nodes in the tree for
    /// which this evaluates to true will be pruned out of the resulting tree. If the root
    /// node is pruned, this will return [`None`].
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder`](crate::prelude::OwnedTreeNode::dfs_preorder) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{BorrowedTreeNode, Tree};
    ///
    /// let tree = Tree {
    ///     value: 0,
    ///     children: vec![
    ///         Tree {
    ///             value: 1,
    ///             children: vec![Tree {
    ///                 value: 3,
    ///                 children: Vec::new(),
    ///             }],
    ///         },
    ///         Tree {
    ///             value: 2,
    ///             children: Vec::new()
    ///         },
    ///     ],
    /// };
    ///
    /// assert_eq!(
    ///     Some(
    ///         Tree {
    ///             value: &0,
    ///             children: vec![
    ///                 Tree {
    ///                     value: &2,
    ///                     children: Vec::new(),
    ///                 }
    ///             ],
    ///         },
    ///     ),
    ///     tree.prune_ref(|value| {
    ///         /// The output for this code would be the following. A couple notes about
    ///         /// this output:
    ///         /// 1. the node with a value of '1' has been removed
    ///         /// 2. the closure is never called on the node with a value of '3' since
    ///         ///    it is already determined to be pruned once '1' has been evaluated.
    ///         /// ```
    ///         /// 0
    ///         /// 1
    ///         /// 2
    ///         /// ```
    ///         println!("{value:?}");
    ///         **value == 1
    ///     })
    /// );
    /// ```
    fn prune_ref<F>(&'a self, f: F) -> Option<Tree<Self::BorrowedValue>>
    where
        F: FnMut(&Self::BorrowedValue) -> bool,
    {
        self.into_pipeline_ref().prune(f).collect_tree()
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the given closure to determine if each subtree in this tree should be pruned.
    ///
    /// Given the path of an element and the element's value, the closure must return true or
    /// false. Any nodes in the tree for which this evaluates to true will be pruned out of
    /// the resulting tree. If the root node is pruned, this will return [`None`].
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder`](crate::prelude::OwnedTreeNode::dfs_preorder) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{BorrowedTreeNode, Tree};
    ///
    /// let tree = Tree {
    ///     value: 0,
    ///     children: vec![
    ///         Tree {
    ///             value: 1,
    ///             children: vec![Tree {
    ///                 value: 3,
    ///                 children: Vec::new(),
    ///             }],
    ///         },
    ///         Tree {
    ///             value: 2,
    ///             children: Vec::new()
    ///         },
    ///     ],
    /// };
    ///
    /// assert_eq!(
    ///     Some(
    ///         Tree {
    ///             value: &0,
    ///             children: vec![
    ///                 Tree {
    ///                     value: &2,
    ///                     children: Vec::new(),
    ///                 }
    ///             ],
    ///         },
    ///     ),
    ///     tree.prune_path_ref(|path, value| {
    ///         /// The output for this code would be the following. A couple notes about
    ///         /// this output:
    ///         /// 1. the node with a value of '1' has been removed
    ///         /// 2. the closure is never called on the node with a value of '3' since
    ///         ///    it is already determined to be pruned once '1' has been evaluated.
    ///         /// ```
    ///         /// 0
    ///         /// 1
    ///         /// 2
    ///         /// ```
    ///         println!("{value:?}");
    ///         matches!(path.get(0), Some(0))
    ///     })
    /// );
    /// ```
    fn prune_path_ref<F>(&'a self, f: F) -> Option<Tree<Self::BorrowedValue>>
    where
        F: FnMut(&[usize], &Self::BorrowedValue) -> bool,
    {
        self.into_pipeline_ref().prune_path(f).collect_tree()
    }

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    ///
    /// Uses the depth of each subtree to determine if the subtree should be pruned.
    /// Any node with a depth that is strictly greater than the max_depth parameter
    /// will be pruned from the tree.
    ///
    /// Depth is zero-based, so the root node is considered to be at depth zero.
    ///
    /// Ex. given a tree like the following, the depths would be as labeled.
    ///
    /// ```text
    ///        0       <- depth: 0
    ///       / \
    ///      1   2     <- depth: 1
    ///     / \ / \
    ///    3  4 5  6   <- depth: 2
    ///           /
    ///          7     <- depth: 3
    ///           \
    ///            8   <- depth: 4
    ///           /
    ///          9     <- depth: 5
    ///           \
    ///           10   <- depth: 6
    /// ```
    ///
    /// ### Basic usage:
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, BorrowedTreeNode};
    ///
    /// let mut tree = Tree {
    ///     value: 0,
    ///     children: vec![
    ///         Tree {
    ///             value: 1,
    ///             children: vec![Tree {
    ///                 value: 3,
    ///                 children: vec![],
    ///             }],
    ///         },
    ///         Tree {
    ///             value: 2,
    ///             children: vec![],
    ///         },
    ///     ],
    /// };
    ///
    /// assert_eq!(
    ///     Tree {
    ///         value: &0,
    ///         children: vec![],
    ///     },
    ///     tree.prune_depth_ref(0)
    /// );
    /// ```
    fn prune_depth_ref(&'a self, max_depth: usize) -> Tree<Self::BorrowedValue> {
        self.into_pipeline_ref()
            .prune_depth(max_depth)
            .collect_tree()
            .unwrap()
    }

    /// map is a tree-based analog to [map](core::iter::Iterator::map).
    ///
    /// Takes a closure and applies that closure to each node's value in the tree.
    ///
    /// map() transforms one tree into another, by means of its argument: something that
    /// implements FnMut. It produces a new tree which calls this closure on each node of
    /// the original tree.
    ///
    /// If you are good at thinking in types, you can think of map() like this: If you
    /// have a tree that has elements of some type A, and you want a tree of some other
    /// type B, you can use map(), passing a closure that takes an A and returns a B.
    ///
    /// ### Example Usage
    ///
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_binary_tree,
    ///     prelude::{Tree, BorrowedTreeNode}
    /// };
    ///
    /// let mut tree = Tree {
    ///     value: "0-0",
    ///     children: vec![
    ///         Tree {
    ///             value: "1-1",
    ///             children: vec![],
    ///         },
    ///         Tree {
    ///             value: "2-2",
    ///             children: vec![],
    ///         }
    ///     ],
    /// };
    ///
    /// let result = tree.map_ref(|value: &&'static str| {
    ///     value.split("-").nth(1).unwrap().to_string()
    /// });
    ///
    /// assert_eq!(
    ///     Tree {
    ///         value: "0".to_string(),
    ///         children: vec![
    ///             Tree {
    ///                 value: "1".to_string(),
    ///                 children: vec![],
    ///             },
    ///             Tree {
    ///                 value: "2".to_string(),
    ///                 children: vec![],
    ///             },
    ///         ],
    ///     },
    ///     result);
    /// ```
    fn map_ref<Output, F>(&'a self, f: F) -> Tree<Output>
    where
        F: FnMut(Self::BorrowedValue) -> Output,
    {
        self.into_pipeline_ref().map_tree(f).collect_tree().unwrap()
    }

    /// fold is a tree-based analog to [fold](core::iter::Iterator::fold).
    ///
    /// Folds every element into an accumulation by applying an operation, returning the
    /// final result.
    ///
    /// fold() takes one argument: a closure with two arguments: the result of accumulating
    /// all children of the current tree node, and an element. The closure returns the value
    /// that the accumulator should have for the parent node's accumulation.
    ///
    /// After applying this closure to every node of the tree, fold() returns the accumulation.
    ///
    /// This operation is sometimes called ‘reduce’ or ‘inject’.
    ///
    /// Folding is useful whenever you have a tree of something, and want to produce a single
    /// value from it.
    ///
    /// ### Example Usage
    /// ```rust
    /// use tree_iterators_rs::{
    ///     examples::create_example_tree,
    ///     prelude::BorrowedTreeNode
    /// };
    ///
    /// let mut tree = create_example_tree();
    /// let accumulation = tree.fold_ref(|child_accumulations: Vec<usize>, value| {
    ///     child_accumulations
    ///         .into_iter()
    ///         .sum::<usize>()
    ///     + *value
    /// });
    ///
    /// assert_eq!(55, accumulation);
    /// ```
    fn fold_ref<Output, F>(&'a self, f: F) -> Output
    where
        F: FnMut(Vec<Output>, Self::BorrowedValue) -> Output,
    {
        self.into_pipeline_ref().fold_tree(f).unwrap()
    }
}

impl<T> OwnedTreeNode for Tree<T> {
    type OwnedValue = T;
    type OwnedChildren = Vec<Self>;

    /// This method gets the value and children from this node. The other
    /// methods of this trait assume that the 'Children' list does not contain
    /// any circular references. If there are, an infinite loop will result.
    fn get_value_and_children(self) -> (Self::OwnedValue, Self::OwnedChildren) {
        (self.value, self.children)
    }
}

impl<'a, T> MutBorrowedTreeNode<'a> for Tree<T>
where
    T: 'a,
{
    type MutBorrowedValue = &'a mut T;
    type MutBorrowedChildren = &'a mut Vec<Self>;

    /// This method gets the value and children from this node. The other
    /// methods of this trait assume that the 'Children' list does not contain
    /// any circular references. If there are, an infinite loop will result.
    fn get_value_and_children_iter_mut(
        &'a mut self,
    ) -> (Self::MutBorrowedValue, Self::MutBorrowedChildren) {
        (&mut self.value, &mut self.children)
    }
}

impl<'a, T> BorrowedTreeNode<'a> for Tree<T>
where
    T: 'a,
{
    type BorrowedValue = &'a T;
    type BorrowedChildren = &'a Vec<Self>;

    /// This method gets the value and children from this node. The other
    /// methods of this trait assume that the 'Children' list does not contain
    /// any circular references. If there are, an infinite loop will result.
    fn get_value_and_children_iter(&'a self) -> (Self::BorrowedValue, Self::BorrowedChildren) {
        (&self.value, &self.children)
    }
}

impl<T> OwnedBinaryTreeNode for BinaryTree<T> {
    type OwnedValue = T;

    fn get_value_and_children_binary(self) -> (Self::OwnedValue, [Option<Self>; 2]) {
        (
            self.value,
            [
                self.left.map(|boxed| *boxed),
                self.right.map(|boxed| *boxed),
            ],
        )
    }
}

impl<'a, T> MutBorrowedBinaryTreeNode<'a> for BinaryTree<T>
where
    Self: 'a,
{
    type MutBorrowedValue = &'a mut T;

    fn get_value_and_children_binary_iter_mut(
        &'a mut self,
    ) -> (Self::MutBorrowedValue, [Option<&'a mut Self>; 2]) {
        (
            &mut self.value,
            [
                match &mut self.left {
                    Some(left) => Some(left.as_mut()),
                    None => None,
                },
                match &mut self.right {
                    Some(right) => Some(right.as_mut()),
                    None => None,
                },
            ],
        )
    }
}

impl<'a, T> BorrowedBinaryTreeNode<'a> for BinaryTree<T>
where
    Self: 'a,
{
    type BorrowedValue = &'a T;

    fn get_value_and_children_binary_iter(
        &'a self,
    ) -> (Self::BorrowedValue, [Option<&'a Self>; 2]) {
        (
            &self.value,
            [
                match &self.left {
                    Some(left) => Some(left.as_ref()),
                    None => None,
                },
                match &self.right {
                    Some(right) => Some(right.as_ref()),
                    None => None,
                },
            ],
        )
    }
}

pub trait OwnedIntoIteratorOfTrees<T>: IntoIterator<Item = T> + Sized
where
    T: OwnedTreeNode,
{
    #[doc = include_str!("../doc_files/at_path.md")]
    #[doc = include_str!("../doc_files/at_path_tree_collection_example.md")]
    fn at_path(self, path: &[usize]) -> Option<T> {
        let first_path_segment = path.first()?;
        let tree = self.into_iter().nth(*first_path_segment)?;
        tree.at_path(&path[1..])
    }

    #[must_use]
    fn into_pipeline(self) -> impl TreeCollectionIterator<T::OwnedValue, T::OwnedChildren> {
        OwnedDFSPreorderCollectionIteratorWithPathTracking::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a breadth first search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.bfs());
    /// ```
    #[must_use]
    fn bfs_each(self) -> OwnedBFSCollectionIterator<Self> {
        OwnedBFSCollectionIterator::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a depth first preorder search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.dfs_preorder());
    /// ```
    #[must_use]
    fn dfs_preorder_each(self) -> OwnedDFSPreorderCollectionIterator<Self> {
        OwnedDFSPreorderCollectionIterator::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a depth first postorder search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.dfs_postorder());
    /// ```
    #[must_use]
    fn dfs_postorder_each(self) -> OwnedDFSPostorderCollectionIterator<Self> {
        OwnedDFSPostorderCollectionIterator::new(self)
    }

    /// Applies the prune operation to every tree within this [`OwnedIntoIteratorOfTrees`],
    /// removing any trees where the root node was pruned.
    ///
    /// For more details, see [`prune`](OwnedTreeNode::prune)
    ///
    /// ### Example Usage
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, OwnedIntoIteratorOfTrees};
    ///
    /// let trees = vec![
    ///     Tree {
    ///         value: 0,
    ///         children: vec![
    ///             Tree {
    ///                 value: 1,
    ///                 children: Vec::new()
    ///             },
    ///             Tree {
    ///                 value: 2,
    ///                 children: Vec::new()
    ///             }
    ///         ]
    ///     },
    ///     Tree {
    ///         value: 0,
    ///         children: vec![
    ///             Tree {
    ///                 value: 1,
    ///                 children: Vec::new()
    ///             },
    ///             Tree {
    ///                 value: 2,
    ///                 children: Vec::new()
    ///             }
    ///         ]
    ///     }
    /// ];
    ///
    /// assert_eq!(
    ///     Vec::<Tree<usize>>::new(),
    ///     trees.clone().prune_each(|_| true).collect::<Vec<_>>()
    /// );
    /// assert_eq!(
    ///     vec![
    ///         Tree {
    ///             value: 0,
    ///             children: Vec::new()
    ///         },
    ///         Tree {
    ///             value: 0,
    ///             children: Vec::new()
    ///         }
    ///     ],
    ///     trees.prune_each(|value| *value != 0).collect::<Vec<_>>()
    /// );
    /// ```
    #[must_use]
    fn prune_each<F>(
        self,
        f: F,
    ) -> Trees<
        T::OwnedValue,
        T::OwnedChildren,
        CollectionPrune<
            T::OwnedValue,
            T::OwnedChildren,
            impl TreeCollectionIterator<T::OwnedValue, T::OwnedChildren>,
            F,
        >,
    >
    where
        F: FnMut(&T::OwnedValue) -> bool,
    {
        self.into_pipeline().prune(f).trees()
    }

    /// Applies the prune operation to every tree within this [`OwnedIntoIteratorOfTrees`].
    ///
    /// For more details, see [`prune_path`](OwnedTreeNode::prune_path)
    #[must_use]
    fn prune_path_each<F>(
        self,
        f: F,
    ) -> Trees<
        T::OwnedValue,
        T::OwnedChildren,
        CollectionPrunePath<
            T::OwnedValue,
            T::OwnedChildren,
            impl TreeCollectionIterator<T::OwnedValue, T::OwnedChildren>,
            F,
        >,
    >
    where
        F: FnMut(&[usize], &T::OwnedValue) -> bool,
    {
        self.into_pipeline().prune_path(f).trees()
    }

    /// Applies the prune_depth operation to every tree within this [`OwnedIntoIteratorOfTrees`],
    /// removing any trees where the root node was pruned.
    ///
    /// For more details, see [`prune_depth`](OwnedTreeNode::prune_depth)
    #[must_use]
    fn prune_depth_each<F>(
        self,
        depth_limit: usize,
    ) -> Trees<
        T::OwnedValue,
        T::OwnedChildren,
        CollectionPruneDepth<
            T::OwnedValue,
            T::OwnedChildren,
            impl TreeCollectionIterator<T::OwnedValue, T::OwnedChildren>,
        >,
    > {
        self.into_pipeline().prune_depth(depth_limit).trees()
    }

    /// Applies the map operation to every tree within this [`OwnedIntoIteratorOfTrees`].
    ///
    /// For more details, see [`map`](OwnedTreeNode::map)
    #[must_use]
    fn map_each<Output, F>(
        self,
        f: F,
    ) -> Trees<
        Output,
        (),
        CollectionMap<
            T::OwnedValue,
            T::OwnedChildren,
            impl TreeCollectionIterator<T::OwnedValue, T::OwnedChildren>,
            F,
            Output,
        >,
    >
    where
        F: FnMut(T::OwnedValue) -> Output,
    {
        self.into_pipeline().map_trees(f).trees()
    }

    /// Applies the fold operation to every tree within this [`OwnedIntoIteratorOfTrees`].
    ///
    /// For more details, see [`fold`](OwnedTreeNode::fold)
    #[must_use]
    fn fold_each<Output, F>(
        self,
        f: F,
    ) -> Fold<
        T::OwnedValue,
        T::OwnedChildren,
        impl TreeCollectionIterator<T::OwnedValue, T::OwnedChildren>,
        F,
        Output,
    >
    where
        F: FnMut(Vec<Output>, T::OwnedValue) -> Output,
    {
        self.into_pipeline().fold_trees(f)
    }
}

pub trait OwnedIntoIteratorOfBinaryTrees<T>: IntoIterator<Item = T> + Sized
where
    T: OwnedBinaryTreeNode,
{
    #[doc = include_str!("../doc_files/at_path.md")]
    #[doc = include_str!("../doc_files/at_path_binary_collection_example.md")]
    fn at_path(self, path: &[usize]) -> Option<T> {
        let first_path_segment = path.first()?;
        let tree = self.into_iter().nth(*first_path_segment)?;
        tree.at_path(&path[1..])
    }

    #[must_use]
    fn into_pipeline(
        self,
    ) -> impl BinaryTreeCollectionIterator<
        <Self::Item as OwnedBinaryTreeNode>::OwnedValue,
        [Option<Self::Item>; 2],
    > {
        OwnedBinaryDFSPreorderCollectionIteratorWithPathTracking::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a breadth first search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.bfs());
    /// ```
    #[must_use]
    fn bfs_each(self) -> OwnedBinaryBFSCollectionIterator<Self> {
        OwnedBinaryBFSCollectionIterator::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a depth first preorder search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.dfs_preorder());
    /// ```
    #[must_use]
    fn dfs_preorder_each(self) -> OwnedBinaryDFSPreorderCollectionIterator<Self> {
        OwnedBinaryDFSPreorderCollectionIterator::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a depth first inorder search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.dfs_inorder());
    /// ```
    #[must_use]
    fn dfs_inorder_each(self) -> OwnedDFSInorderCollectionIterator<Self> {
        OwnedDFSInorderCollectionIterator::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a depth first postorder search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.dfs_postorder());
    /// ```
    #[must_use]
    fn dfs_postorder_each(self) -> OwnedBinaryDFSPostorderCollectionIterator<Self> {
        OwnedBinaryDFSPostorderCollectionIterator::new(self)
    }

    /// Applies the prune operation to every tree within this [`OwnedIntoIteratorOfBinaryTrees`],
    /// removing any trees where the root node was pruned.
    ///
    /// For more details, see [`prune`](OwnedBinaryTreeNode::prune)
    ///
    /// ### Example Usage
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, OwnedIntoIteratorOfBinaryTrees};
    ///
    /// let trees = vec![
    ///     BinaryTree {
    ///         value: 0,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 1,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: Some(Box::new(BinaryTree {
    ///             value: 2,
    ///             left: None,
    ///             right: None,
    ///         }))
    ///     },
    ///     BinaryTree {
    ///         value: 0,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 1,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: Some(Box::new(BinaryTree {
    ///             value: 2,
    ///             left: None,
    ///             right: None,
    ///         }))
    ///     }
    /// ];
    ///
    /// assert_eq!(
    ///     Vec::<BinaryTree<usize>>::new(),
    ///     trees.clone().prune_each(|_| true).collect::<Vec<_>>()
    /// );
    /// assert_eq!(
    ///     vec![
    ///         BinaryTree {
    ///             value: 0,
    ///             left: None,
    ///             right: None,
    ///         },
    ///         BinaryTree {
    ///             value: 0,
    ///             left: None,
    ///             right: None,
    ///         }
    ///     ],
    ///     trees.prune_each(|value| *value != 0).collect::<Vec<_>>()
    /// );
    /// ```
    #[must_use]
    fn prune_each<F>(
        self,
        f: F,
    ) -> BinaryTrees<
        T::OwnedValue,
        [Option<T>; 2],
        BinaryCollectionPrune<
            T::OwnedValue,
            [Option<T>; 2],
            impl BinaryTreeCollectionIterator<T::OwnedValue, [Option<T>; 2]>,
            F,
        >,
    >
    where
        F: FnMut(&T::OwnedValue) -> bool,
    {
        self.into_pipeline().prune(f).trees()
    }

    /// Applies the prune operation to every tree within this [`OwnedIntoIteratorOfBinaryTrees`].
    ///
    /// For more details, see [`prune_path`](OwnedBinaryTreeNode::prune_path)
    #[must_use]
    fn prune_path_each<F>(
        self,
        f: F,
    ) -> BinaryTrees<
        T::OwnedValue,
        [Option<T>; 2],
        BinaryCollectionPrunePath<
            T::OwnedValue,
            [Option<T>; 2],
            impl BinaryTreeCollectionIterator<T::OwnedValue, [Option<T>; 2]>,
            F,
        >,
    >
    where
        F: FnMut(&[usize], &T::OwnedValue) -> bool,
    {
        self.into_pipeline().prune_path(f).trees()
    }

    /// Applies the prune_depth operation to every tree within this [`OwnedIntoIteratorOfBinaryTrees`],
    /// removing any trees where the root node was pruned.
    ///
    /// For more details, see [`prune_depth`](OwnedBinaryTreeNode::prune_depth)
    #[must_use]
    fn prune_depth_each(
        self,
        depth_limit: usize,
    ) -> BinaryTrees<
        T::OwnedValue,
        [Option<T>; 2],
        CollectionPruneDepth<
            T::OwnedValue,
            [Option<T>; 2],
            impl BinaryTreeCollectionIterator<T::OwnedValue, [Option<T>; 2]>,
        >,
    > {
        self.into_pipeline().prune_depth(depth_limit).trees()
    }

    /// Applies the map operation to every tree within this [`OwnedIntoIteratorOfBinaryTrees`].
    ///
    /// For more details, see [`map`](OwnedBinaryTreeNode::map)
    #[must_use]
    fn map_each<Output, F>(
        self,
        f: F,
    ) -> BinaryTrees<
        Output,
        (),
        CollectionMap<
            T::OwnedValue,
            [Option<T>; 2],
            impl BinaryTreeCollectionIterator<T::OwnedValue, [Option<T>; 2]>,
            F,
            Output,
        >,
    >
    where
        F: FnMut(T::OwnedValue) -> Output,
    {
        self.into_pipeline().map_trees(f).trees()
    }

    /// Applies the fold operation to every tree within this [`OwnedIntoIteratorOfBinaryTrees`].
    ///
    /// For more details, see [`fold`](OwnedBinaryTreeNode::fold)
    #[must_use]
    fn fold_each<Output, F>(
        self,
        f: F,
    ) -> BinaryFold<
        T::OwnedValue,
        [Option<T>; 2],
        impl BinaryTreeCollectionIterator<T::OwnedValue, [Option<T>; 2]>,
        F,
        Output,
    >
    where
        F: FnMut([Option<Output>; 2], T::OwnedValue) -> Output,
    {
        self.into_pipeline().fold_trees(f)
    }
}

pub trait MutBorrowedIntoIteratorOfTrees<'a, T>: IntoIterator<Item = &'a mut T> + Sized
where
    T: MutBorrowedTreeNode<'a>,
{
    #[doc = include_str!("../doc_files/at_path.md")]
    #[doc = include_str!("../doc_files/at_path_tree_collection_example.md")]
    fn at_path_mut(self, path: &[usize]) -> Option<&'a mut T> {
        let first_path_segment = path.first()?;
        let tree = self.into_iter().nth(*first_path_segment)?;
        tree.at_path_mut(&path[1..])
    }

    #[must_use]
    fn into_pipeline_mut(
        self,
    ) -> impl TreeCollectionIterator<T::MutBorrowedValue, T::MutBorrowedChildren> {
        MutBorrowedDFSPreorderCollectionIteratorWithPathTracking::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a breadth first search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.bfs_iter_mut());
    /// ```
    #[must_use]
    fn bfs_each_iter_mut(self) -> MutBorrowedBFSCollectionIterator<'a, Self, T> {
        MutBorrowedBFSCollectionIterator::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a depth first preorder search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.dfs_preorder_iter_mut());
    /// ```
    #[must_use]
    fn dfs_preorder_each_iter_mut(self) -> MutBorrowedDFSPreorderCollectionIterator<'a, Self, T> {
        MutBorrowedDFSPreorderCollectionIterator::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a depth first postorder search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.dfs_postorder_iter_mut());
    /// ```
    #[must_use]
    fn dfs_postorder_each_iter_mut(self) -> MutBorrowedDFSPostorderCollectionIterator<'a, Self, T> {
        MutBorrowedDFSPostorderCollectionIterator::new(self)
    }

    /// Applies the prune operation to every tree within this [`MutBorrowedIntoIteratorOfTrees`],
    /// removing any trees where the root node was pruned.
    ///
    /// For more details, see [`prune_mut`](MutBorrowedTreeNode::prune_mut)
    ///
    /// ### Example Usage
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, MutBorrowedIntoIteratorOfTrees};
    ///
    /// let mut trees = vec![
    ///     Tree {
    ///         value: 0,
    ///         children: vec![
    ///             Tree {
    ///                 value: 1,
    ///                 children: Vec::new()
    ///             },
    ///             Tree {
    ///                 value: 2,
    ///                 children: Vec::new()
    ///             }
    ///         ]
    ///     },
    ///     Tree {
    ///         value: 0,
    ///         children: vec![
    ///             Tree {
    ///                 value: 1,
    ///                 children: Vec::new()
    ///             },
    ///             Tree {
    ///                 value: 2,
    ///                 children: Vec::new()
    ///             }
    ///         ]
    ///     }
    /// ];
    ///
    /// assert_eq!(
    ///     Vec::<Tree<&mut usize>>::new(),
    ///     trees.prune_each_mut(|_| true).collect::<Vec<_>>()
    /// );
    /// assert_eq!(
    ///     vec![
    ///         Tree {
    ///             value: &mut 0,
    ///             children: Vec::new()
    ///         },
    ///         Tree {
    ///             value: &mut 0,
    ///             children: Vec::new()
    ///         }
    ///     ],
    ///     trees.prune_each_mut(|value| **value != 0).collect::<Vec<_>>()
    /// );
    /// ```
    #[must_use]
    fn prune_each_mut<F>(
        self,
        f: F,
    ) -> Trees<
        T::MutBorrowedValue,
        T::MutBorrowedChildren,
        CollectionPrune<
            T::MutBorrowedValue,
            T::MutBorrowedChildren,
            impl TreeCollectionIterator<T::MutBorrowedValue, T::MutBorrowedChildren>,
            F,
        >,
    >
    where
        F: FnMut(&T::MutBorrowedValue) -> bool,
    {
        self.into_pipeline_mut().prune(f).trees()
    }

    /// Applies the prune operation to every tree within this [`MutBorrowedIntoIteratorOfTrees`].
    ///
    /// For more details, see [`prune_path_mut`](MutBorrowedTreeNode::prune_path_mut)
    #[must_use]
    fn prune_path_each_mut<F>(
        self,
        f: F,
    ) -> Trees<
        T::MutBorrowedValue,
        T::MutBorrowedChildren,
        CollectionPrunePath<
            T::MutBorrowedValue,
            T::MutBorrowedChildren,
            impl TreeCollectionIterator<T::MutBorrowedValue, T::MutBorrowedChildren>,
            F,
        >,
    >
    where
        F: FnMut(&[usize], &T::MutBorrowedValue) -> bool,
    {
        self.into_pipeline_mut().prune_path(f).trees()
    }

    /// Applies the prune_depth operation to every tree within this [`MutBorrowedIntoIteratorOfTrees`],
    /// removing any trees where the root node was pruned.
    ///
    /// For more details, see [`prune_depth_mut`](MutBorrowedTreeNode::prune_depth_mut)
    #[must_use]
    fn prune_depth_each_mut(
        self,
        depth_limit: usize,
    ) -> Trees<
        T::MutBorrowedValue,
        T::MutBorrowedChildren,
        CollectionPruneDepth<
            T::MutBorrowedValue,
            T::MutBorrowedChildren,
            impl TreeCollectionIterator<T::MutBorrowedValue, T::MutBorrowedChildren>,
        >,
    > {
        self.into_pipeline_mut().prune_depth(depth_limit).trees()
    }

    /// Applies the map operation to every tree within this [`MutBorrowedIntoIteratorOfTrees`].
    ///
    /// For more details, see [`map_mut`](MutBorrowedTreeNode::map_mut)
    #[must_use]
    fn map_each_mut<Output, F>(
        self,
        f: F,
    ) -> Trees<
        Output,
        (),
        CollectionMap<
            T::MutBorrowedValue,
            T::MutBorrowedChildren,
            impl TreeCollectionIterator<T::MutBorrowedValue, T::MutBorrowedChildren>,
            F,
            Output,
        >,
    >
    where
        F: FnMut(T::MutBorrowedValue) -> Output,
    {
        self.into_pipeline_mut().map_trees(f).trees()
    }

    /// Applies the fold operation to every tree within this [`MutBorrowedIntoIteratorOfTrees`].
    ///
    /// For more details, see [`fold_mut`](MutBorrowedTreeNode::fold_mut)
    #[must_use]
    fn fold_each_mut<Output, F>(
        self,
        f: F,
    ) -> Fold<
        T::MutBorrowedValue,
        T::MutBorrowedChildren,
        impl TreeCollectionIterator<T::MutBorrowedValue, T::MutBorrowedChildren>,
        F,
        Output,
    >
    where
        F: FnMut(Vec<Output>, T::MutBorrowedValue) -> Output,
    {
        self.into_pipeline_mut().fold_trees(f)
    }
}

pub trait MutBorrowedIntoIteratorOfBinaryTrees<'a, T>:
    IntoIterator<Item = &'a mut T> + Sized
where
    T: MutBorrowedBinaryTreeNode<'a>,
{
    #[doc = include_str!("../doc_files/at_path.md")]
    #[doc = include_str!("../doc_files/at_path_binary_collection_example.md")]
    fn at_path_mut(self, path: &[usize]) -> Option<&'a mut T> {
        let first_path_segment = path.first()?;
        let tree = self.into_iter().nth(*first_path_segment)?;
        tree.at_path_mut(&path[1..])
    }

    #[must_use]
    fn into_pipeline_mut(
        self,
    ) -> impl BinaryTreeCollectionIterator<T::MutBorrowedValue, [Option<&'a mut T>; 2]> {
        MutBorrowedBinaryDFSPreorderCollectionIteratorWithPathTracking::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a breadth first search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.bfs_iter_mut());
    /// ```
    #[must_use]
    fn bfs_each_iter_mut(self) -> MutBorrowedBinaryBFSCollectionIterator<'a, Self, T> {
        MutBorrowedBinaryBFSCollectionIterator::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a depth first preorder search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.dfs_preorder_iter_mut());
    /// ```
    #[must_use]
    fn dfs_preorder_each_iter_mut(
        self,
    ) -> MutBorrowedBinaryDFSPreorderCollectionIterator<'a, Self, T> {
        MutBorrowedBinaryDFSPreorderCollectionIterator::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a depth first inorder search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.dfs_inorder_iter_mut());
    /// ```
    #[must_use]
    fn dfs_inorder_each_iter_mut(self) -> MutBorrowedDFSInorderCollectionIterator<'a, Self, T> {
        MutBorrowedDFSInorderCollectionIterator::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a depth first postorder search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.dfs_postorder_iter_mut());
    /// ```
    #[must_use]
    fn dfs_postorder_each_iter_mut(
        self,
    ) -> MutBorrowedBinaryDFSPostorderCollectionIterator<'a, Self, T> {
        MutBorrowedBinaryDFSPostorderCollectionIterator::new(self)
    }

    /// Applies the prune operation to every tree within this [`MutBorrowedIntoIteratorOfBinaryTrees`],
    /// removing any trees where the root node was pruned.
    ///
    /// For more details, see [`prune_mut`](MutBorrowedBinaryTreeNode::prune_mut)
    ///
    /// ### Example Usage
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, MutBorrowedIntoIteratorOfBinaryTrees};
    ///
    /// let mut trees = vec![
    ///     BinaryTree {
    ///         value: 0,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 1,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: Some(Box::new(BinaryTree {
    ///             value: 2,
    ///             left: None,
    ///             right: None,
    ///         }))
    ///     },
    ///     BinaryTree {
    ///         value: 0,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 1,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: Some(Box::new(BinaryTree {
    ///             value: 2,
    ///             left: None,
    ///             right: None,
    ///         }))
    ///     }
    /// ];
    ///
    /// assert_eq!(
    ///     Vec::<BinaryTree<&mut usize>>::new(),
    ///     trees.prune_each_mut(|_| true).collect::<Vec<_>>()
    /// );
    /// assert_eq!(
    ///     vec![
    ///         BinaryTree {
    ///             value: &mut 0,
    ///             left: None,
    ///             right: None,
    ///         },
    ///         BinaryTree {
    ///             value: &mut 0,
    ///             left: None,
    ///             right: None,
    ///         }
    ///     ],
    ///     trees.prune_each_mut(|value| **value != 0).collect::<Vec<_>>()
    /// );
    /// ```
    #[must_use]
    fn prune_each_mut<F>(
        self,
        f: F,
    ) -> BinaryTrees<
        T::MutBorrowedValue,
        [Option<&'a mut T>; 2],
        BinaryCollectionPrune<
            T::MutBorrowedValue,
            [Option<&'a mut T>; 2],
            impl BinaryTreeCollectionIterator<T::MutBorrowedValue, [Option<&'a mut T>; 2]>,
            F,
        >,
    >
    where
        F: FnMut(&T::MutBorrowedValue) -> bool,
    {
        self.into_pipeline_mut().prune(f).trees()
    }

    /// Applies the prune operation to every tree within this [`MutBorrowedIntoIteratorOfBinaryTrees`].
    ///
    /// For more details, see [`prune_path_mut`](MutBorrowedBinaryTreeNode::prune_path_mut)
    #[must_use]
    fn prune_path_each_mut<F>(
        self,
        f: F,
    ) -> BinaryTrees<
        T::MutBorrowedValue,
        [Option<&'a mut T>; 2],
        BinaryCollectionPrunePath<
            T::MutBorrowedValue,
            [Option<&'a mut T>; 2],
            impl BinaryTreeCollectionIterator<T::MutBorrowedValue, [Option<&'a mut T>; 2]>,
            F,
        >,
    >
    where
        F: FnMut(&[usize], &T::MutBorrowedValue) -> bool,
    {
        self.into_pipeline_mut().prune_path(f).trees()
    }

    /// Applies the prune_depth operation to every tree within this [`MutBorrowedIntoIteratorOfBinaryTrees`],
    /// removing any trees where the root node was pruned.
    ///
    /// For more details, see [`prune_depth_mut`](MutBorrowedBinaryTreeNode::prune_depth_mut)
    #[must_use]
    fn prune_depth_each_mut(
        self,
        depth_limit: usize,
    ) -> BinaryTrees<
        T::MutBorrowedValue,
        [Option<&'a mut T>; 2],
        CollectionPruneDepth<
            T::MutBorrowedValue,
            [Option<&'a mut T>; 2],
            impl BinaryTreeCollectionIterator<T::MutBorrowedValue, [Option<&'a mut T>; 2]>,
        >,
    > {
        self.into_pipeline_mut().prune_depth(depth_limit).trees()
    }

    /// Applies the map operation to every tree within this [`MutBorrowedIntoIteratorOfBinaryTrees`].
    ///
    /// For more details, see [`map_mut`](MutBorrowedBinaryTreeNode::map_mut)
    #[must_use]
    fn map_each_mut<Output, F>(
        self,
        f: F,
    ) -> BinaryTrees<
        Output,
        (),
        CollectionMap<
            T::MutBorrowedValue,
            [Option<&'a mut T>; 2],
            impl BinaryTreeCollectionIterator<T::MutBorrowedValue, [Option<&'a mut T>; 2]>,
            F,
            Output,
        >,
    >
    where
        F: FnMut(T::MutBorrowedValue) -> Output,
    {
        self.into_pipeline_mut().map_trees(f).trees()
    }

    /// Applies the fold operation to every tree within this [`MutBorrowedIntoIteratorOfBinaryTrees`].
    ///
    /// For more details, see [`fold_mut`](MutBorrowedBinaryTreeNode::fold_mut)
    #[must_use]
    fn fold_each_mut<Output, F>(
        self,
        f: F,
    ) -> BinaryFold<
        T::MutBorrowedValue,
        [Option<&'a mut T>; 2],
        impl BinaryTreeCollectionIterator<T::MutBorrowedValue, [Option<&'a mut T>; 2]>,
        F,
        Output,
    >
    where
        F: FnMut([Option<Output>; 2], T::MutBorrowedValue) -> Output,
    {
        self.into_pipeline_mut().fold_trees(f)
    }
}

pub trait BorrowedIntoIteratorOfTrees<'a, T>: IntoIterator<Item = &'a T> + Sized
where
    T: BorrowedTreeNode<'a>,
{
    #[doc = include_str!("../doc_files/at_path.md")]
    #[doc = include_str!("../doc_files/at_path_tree_collection_example.md")]
    fn at_path_ref(self, path: &[usize]) -> Option<&'a T> {
        let first_path_segment = path.first()?;
        let tree = self.into_iter().nth(*first_path_segment)?;
        tree.at_path_ref(&path[1..])
    }

    #[must_use]
    fn into_pipeline_ref(
        self,
    ) -> impl TreeCollectionIterator<T::BorrowedValue, T::BorrowedChildren> {
        BorrowedDFSPreorderCollectionIteratorWithPathTracking::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a breadth first search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.bfs_iter());
    /// ```
    #[must_use]
    fn bfs_each_iter(self) -> BorrowedBFSCollectionIterator<'a, Self, T> {
        BorrowedBFSCollectionIterator::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a depth first preorder search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.dfs_preorder_iter());
    /// ```
    #[must_use]
    fn dfs_preorder_each_iter(self) -> BorrowedDFSPreorderCollectionIterator<'a, Self, T> {
        BorrowedDFSPreorderCollectionIterator::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a depth first postorder search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.dfs_postorder_iter());
    /// ```
    #[must_use]
    fn dfs_postorder_each_iter(self) -> BorrowedDFSPostorderCollectionIterator<'a, Self, T> {
        BorrowedDFSPostorderCollectionIterator::new(self)
    }

    /// Applies the prune operation to every tree within this [`BorrowedIntoIteratorOfTrees`],
    /// removing any trees where the root node was pruned.
    ///
    /// For more details, see [`prune_ref`](BorrowedTreeNode::prune_ref)
    ///
    /// ### Example Usage
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, BorrowedIntoIteratorOfTrees};
    ///
    /// let mut trees = vec![
    ///     Tree {
    ///         value: 0,
    ///         children: vec![
    ///             Tree {
    ///                 value: 1,
    ///                 children: Vec::new()
    ///             },
    ///             Tree {
    ///                 value: 2,
    ///                 children: Vec::new()
    ///             }
    ///         ]
    ///     },
    ///     Tree {
    ///         value: 0,
    ///         children: vec![
    ///             Tree {
    ///                 value: 1,
    ///                 children: Vec::new()
    ///             },
    ///             Tree {
    ///                 value: 2,
    ///                 children: Vec::new()
    ///             }
    ///         ]
    ///     }
    /// ];
    ///
    /// assert_eq!(
    ///     Vec::<Tree<&usize>>::new(),
    ///     trees.prune_each_ref(|_| true).collect::<Vec<_>>()
    /// );
    /// assert_eq!(
    ///     vec![
    ///         Tree {
    ///             value: &0,
    ///             children: Vec::new()
    ///         },
    ///         Tree {
    ///             value: &0,
    ///             children: Vec::new()
    ///         }
    ///     ],
    ///     trees.prune_each_ref(|value| **value != 0).collect::<Vec<_>>()
    /// );
    /// ```
    #[must_use]
    fn prune_each_ref<F>(
        self,
        f: F,
    ) -> Trees<
        T::BorrowedValue,
        T::BorrowedChildren,
        CollectionPrune<
            T::BorrowedValue,
            T::BorrowedChildren,
            impl TreeCollectionIterator<T::BorrowedValue, T::BorrowedChildren>,
            F,
        >,
    >
    where
        F: FnMut(&T::BorrowedValue) -> bool,
    {
        self.into_pipeline_ref().prune(f).trees()
    }

    /// Applies the prune operation to every tree within this [`BorrowedIntoIteratorOfTrees`].
    ///
    /// For more details, see [`prune_path_ref`](BorrowedTreeNode::prune_path_ref)
    #[must_use]
    fn prune_path_each_ref<F>(
        self,
        f: F,
    ) -> Trees<
        T::BorrowedValue,
        T::BorrowedChildren,
        CollectionPrunePath<
            T::BorrowedValue,
            T::BorrowedChildren,
            impl TreeCollectionIterator<T::BorrowedValue, T::BorrowedChildren>,
            F,
        >,
    >
    where
        F: FnMut(&[usize], &T::BorrowedValue) -> bool,
    {
        self.into_pipeline_ref().prune_path(f).trees()
    }

    /// Applies the prune_depth operation to every tree within this [`BorrowedIntoIteratorOfTrees`],
    /// removing any trees where the root node was pruned.
    ///
    /// For more details, see [`prune_depth_ref`](BorrowedTreeNode::prune_depth_ref)
    #[must_use]
    fn prune_depth_each_ref(
        self,
        depth_limit: usize,
    ) -> Trees<
        T::BorrowedValue,
        T::BorrowedChildren,
        CollectionPruneDepth<
            T::BorrowedValue,
            T::BorrowedChildren,
            impl TreeCollectionIterator<T::BorrowedValue, T::BorrowedChildren>,
        >,
    > {
        self.into_pipeline_ref().prune_depth(depth_limit).trees()
    }

    /// Applies the map operation to every tree within this [`BorrowedIntoIteratorOfTrees`].
    ///
    /// For more details, see [`map_ref`](BorrowedTreeNode::map_ref)
    #[must_use]
    fn map_each_ref<Output, F>(
        self,
        f: F,
    ) -> Trees<
        Output,
        (),
        CollectionMap<
            T::BorrowedValue,
            T::BorrowedChildren,
            impl TreeCollectionIterator<T::BorrowedValue, T::BorrowedChildren>,
            F,
            Output,
        >,
    >
    where
        F: FnMut(T::BorrowedValue) -> Output,
    {
        self.into_pipeline_ref().map_trees(f).trees()
    }

    /// Applies the fold operation to every tree within this [`BorrowedIntoIteratorOfTrees`].
    ///
    /// For more details, see [`fold_ref`](BorrowedTreeNode::fold_ref)
    #[must_use]
    fn fold_each_ref<Output, F>(
        self,
        f: F,
    ) -> Fold<
        T::BorrowedValue,
        T::BorrowedChildren,
        impl TreeCollectionIterator<T::BorrowedValue, T::BorrowedChildren>,
        F,
        Output,
    >
    where
        F: FnMut(Vec<Output>, T::BorrowedValue) -> Output,
    {
        self.into_pipeline_ref().fold_trees(f)
    }
}

pub trait BorrowedIntoIteratorOfBinaryTrees<'a, T>: IntoIterator<Item = &'a T> + Sized
where
    T: BorrowedBinaryTreeNode<'a>,
{
    #[doc = include_str!("../doc_files/at_path.md")]
    #[doc = include_str!("../doc_files/at_path_binary_collection_example.md")]
    fn at_path_ref(self, path: &[usize]) -> Option<&'a T> {
        let first_path_segment = path.first()?;
        let tree = self.into_iter().nth(*first_path_segment)?;
        tree.at_path_ref(&path[1..])
    }

    #[must_use]
    fn into_pipeline_ref(
        self,
    ) -> impl BinaryTreeCollectionIterator<T::BorrowedValue, [Option<&'a T>; 2]> {
        BorrowedBinaryDFSPreorderCollectionIteratorWithPathTracking::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a breadth first search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.bfs_iter());
    /// ```
    #[must_use]
    fn bfs_each_iter(self) -> BorrowedBinaryBFSCollectionIterator<'a, Self, T> {
        BorrowedBinaryBFSCollectionIterator::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a depth first preorder search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.dfs_preorder_iter());
    /// ```
    #[must_use]
    fn dfs_preorder_each_iter(self) -> BorrowedBinaryDFSPreorderCollectionIterator<'a, Self, T> {
        BorrowedBinaryDFSPreorderCollectionIterator::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a depth first inorder search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.dfs_inorder_iter());
    /// ```
    #[must_use]
    fn dfs_inorder_each_iter(self) -> BorrowedDFSInorderCollectionIterator<'a, Self, T> {
        BorrowedDFSInorderCollectionIterator::new(self)
    }

    /// Iterates over each tree in the IntoIterator, then over each node in
    /// each tree in a depth first postorder search.
    ///
    /// This is equivalent to the following:
    ///
    /// ```ignore
    /// self.into_iter().flat_map(|tree| tree.dfs_postorder_iter());
    /// ```
    #[must_use]
    fn dfs_postorder_each_iter(self) -> BorrowedBinaryDFSPostorderCollectionIterator<'a, Self, T> {
        BorrowedBinaryDFSPostorderCollectionIterator::new(self)
    }

    /// Applies the prune operation to every tree within this [`BorrowedIntoIteratorOfBinaryTrees`],
    /// removing any trees where the root node was pruned.
    ///
    /// For more details, see [`prune_ref`](BorrowedBinaryTreeNode::prune_ref)
    ///
    /// ### Example Usage
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, BorrowedIntoIteratorOfBinaryTrees};
    ///
    /// let trees = vec![
    ///     BinaryTree {
    ///         value: 0,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 1,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: Some(Box::new(BinaryTree {
    ///             value: 2,
    ///             left: None,
    ///             right: None,
    ///         }))
    ///     },
    ///     BinaryTree {
    ///         value: 0,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 1,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: Some(Box::new(BinaryTree {
    ///             value: 2,
    ///             left: None,
    ///             right: None,
    ///         }))
    ///     }
    /// ];
    ///
    /// assert_eq!(
    ///     Vec::<BinaryTree<&usize>>::new(),
    ///     trees.prune_each_ref(|_| true).collect::<Vec<_>>()
    /// );
    /// assert_eq!(
    ///     vec![
    ///         BinaryTree {
    ///             value: &0,
    ///             left: None,
    ///             right: None,
    ///         },
    ///         BinaryTree {
    ///             value: &0,
    ///             left: None,
    ///             right: None,
    ///         }
    ///     ],
    ///     trees.prune_each_ref(|value| **value != 0).collect::<Vec<_>>()
    /// );
    /// ```
    #[must_use]
    fn prune_each_ref<F>(
        self,
        f: F,
    ) -> BinaryTrees<
        T::BorrowedValue,
        [Option<&'a T>; 2],
        BinaryCollectionPrune<
            T::BorrowedValue,
            [Option<&'a T>; 2],
            impl BinaryTreeCollectionIterator<T::BorrowedValue, [Option<&'a T>; 2]>,
            F,
        >,
    >
    where
        F: FnMut(&T::BorrowedValue) -> bool,
    {
        self.into_pipeline_ref().prune(f).trees()
    }

    /// Applies the prune operation to every tree within this [`BorrowedIntoIteratorOfBinaryTrees`].
    ///
    /// For more details, see [`prune_path_ref`](BorrowedBinaryTreeNode::prune_path_ref)
    #[must_use]
    fn prune_path_each_ref<F>(
        self,
        f: F,
    ) -> BinaryTrees<
        T::BorrowedValue,
        [Option<&'a T>; 2],
        BinaryCollectionPrunePath<
            T::BorrowedValue,
            [Option<&'a T>; 2],
            impl BinaryTreeCollectionIterator<T::BorrowedValue, [Option<&'a T>; 2]>,
            F,
        >,
    >
    where
        F: FnMut(&[usize], &T::BorrowedValue) -> bool,
    {
        self.into_pipeline_ref().prune_path(f).trees()
    }

    /// Applies the prune_depth operation to every tree within this [`BorrowedIntoIteratorOfBinaryTrees`],
    /// removing any trees where the root node was pruned.
    ///
    /// For more details, see [`prune_depth_ref`](BorrowedBinaryTreeNode::prune_depth_ref)
    #[must_use]
    fn prune_depth_each_ref(
        self,
        depth_limit: usize,
    ) -> BinaryTrees<
        T::BorrowedValue,
        [Option<&'a T>; 2],
        CollectionPruneDepth<
            T::BorrowedValue,
            [Option<&'a T>; 2],
            impl BinaryTreeCollectionIterator<T::BorrowedValue, [Option<&'a T>; 2]>,
        >,
    > {
        self.into_pipeline_ref().prune_depth(depth_limit).trees()
    }

    /// Applies the map operation to every tree within this [`BorrowedIntoIteratorOfBinaryTrees`].
    ///
    /// For more details, see [`map_ref`](BorrowedBinaryTreeNode::map_ref)
    #[must_use]
    fn map_each_ref<Output, F>(
        self,
        f: F,
    ) -> BinaryTrees<
        Output,
        (),
        CollectionMap<
            T::BorrowedValue,
            [Option<&'a T>; 2],
            impl BinaryTreeCollectionIterator<T::BorrowedValue, [Option<&'a T>; 2]>,
            F,
            Output,
        >,
    >
    where
        F: FnMut(T::BorrowedValue) -> Output,
    {
        self.into_pipeline_ref().map_trees(f).trees()
    }

    /// Applies the fold operation to every tree within this [`BorrowedIntoIteratorOfBinaryTrees`].
    ///
    /// For more details, see [`fold_ref`](BorrowedBinaryTreeNode::fold_ref)
    #[must_use]
    fn fold_each_ref<Output, F>(
        self,
        f: F,
    ) -> BinaryFold<
        T::BorrowedValue,
        [Option<&'a T>; 2],
        impl BinaryTreeCollectionIterator<T::BorrowedValue, [Option<&'a T>; 2]>,
        F,
        Output,
    >
    where
        F: FnMut([Option<Output>; 2], T::BorrowedValue) -> Output,
    {
        self.into_pipeline_ref().fold_trees(f)
    }
}

impl<T> OwnedIntoIteratorOfTrees<T> for Vec<T> where T: OwnedTreeNode {}
impl<T> OwnedIntoIteratorOfBinaryTrees<T> for Vec<T> where T: OwnedBinaryTreeNode {}
impl<const LEN: usize, T> OwnedIntoIteratorOfTrees<T> for [T; LEN] where T: OwnedTreeNode {}
impl<const LEN: usize, T> OwnedIntoIteratorOfBinaryTrees<T> for [T; LEN] where T: OwnedBinaryTreeNode
{}

impl<'a, T> MutBorrowedIntoIteratorOfTrees<'a, T> for &'a mut Vec<T> where T: MutBorrowedTreeNode<'a>
{}
impl<'a, const LEN: usize, T> MutBorrowedIntoIteratorOfTrees<'a, T> for &'a mut [T; LEN] where
    T: MutBorrowedTreeNode<'a>
{
}
impl<'a, T> MutBorrowedIntoIteratorOfTrees<'a, T> for IterMut<'a, T> where T: MutBorrowedTreeNode<'a>
{}
impl<'a, T> MutBorrowedIntoIteratorOfBinaryTrees<'a, T> for &'a mut Vec<T> where
    T: MutBorrowedBinaryTreeNode<'a>
{
}
impl<'a, const LEN: usize, T> MutBorrowedIntoIteratorOfBinaryTrees<'a, T> for &'a mut [T; LEN] where
    T: MutBorrowedBinaryTreeNode<'a>
{
}
impl<'a, T> MutBorrowedIntoIteratorOfBinaryTrees<'a, T> for IterMut<'a, T> where
    T: MutBorrowedBinaryTreeNode<'a>
{
}

impl<'a, T> BorrowedIntoIteratorOfTrees<'a, T> for &'a Vec<T> where T: BorrowedTreeNode<'a> {}
impl<'a, T> BorrowedIntoIteratorOfTrees<'a, T> for &'a [T] where T: BorrowedTreeNode<'a> {}
impl<'a, T> BorrowedIntoIteratorOfTrees<'a, T> for Iter<'a, T> where T: BorrowedTreeNode<'a> {}
impl<'a, T> BorrowedIntoIteratorOfBinaryTrees<'a, T> for &'a Vec<T> where
    T: BorrowedBinaryTreeNode<'a>
{
}
impl<'a, const LEN: usize, T> BorrowedIntoIteratorOfBinaryTrees<'a, T> for &'a [T; LEN] where
    T: BorrowedBinaryTreeNode<'a>
{
}
impl<'a, T> BorrowedIntoIteratorOfBinaryTrees<'a, T> for Iter<'a, T> where
    T: BorrowedBinaryTreeNode<'a>
{
}

fn opt_to_opt<T>(opt: Option<T>) -> Option<T> {
    opt
}
