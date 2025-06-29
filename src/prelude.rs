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
    /// Maps a BinaryTree<&T> to a BinaryTree<T> by cloning the contents of the BinaryTree.
    pub fn cloned(&self) -> BinaryTree<T> {
        self.map_ref(|item| (*item).clone())
    }
}

impl<T> BinaryTree<&mut T>
where
    T: Clone,
{
    /// Maps a BinaryTree<&mut T> to a BinaryTree<T> by cloning the contents of the BinaryTree.
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
    /// Maps a Tree<&T> to a Tree<T> by cloning the contents of the Tree.
    pub fn cloned(self) -> Tree<T> {
        self.map(|item| item.clone())
    }
}

impl<T> Tree<&mut T>
where
    T: Clone,
{
    /// Maps a Tree<&mut T> to a Tree<T> by cloning the contents of the Tree.
    pub fn cloned(self) -> Tree<T> {
        self.map(|item| item.clone())
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
                match self.left {
                    Some(boxed) => Some(*boxed),
                    None => None,
                },
                match self.right {
                    Some(boxed) => Some(*boxed),
                    None => None,
                },
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

#[cfg(test)]
use streaming_iterator::StreamingIterator;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[cfg(test)]
    extern crate std;
    #[cfg(test)]
    use std::collections::HashMap;
    #[cfg(test)]
    pub(crate) fn get_value_to_path_map() -> HashMap<usize, Vec<usize>> {
        let mut result = HashMap::new();
        result.insert(0, vec![]);
        result.insert(1, vec![0]);
        result.insert(2, vec![1]);
        result.insert(3, vec![0, 0]);
        result.insert(4, vec![0, 1]);
        result.insert(5, vec![1, 0]);
        result.insert(6, vec![1, 1]);
        result.insert(7, vec![1, 1, 0]);
        result.insert(8, vec![1, 1, 0, 0]);
        result.insert(9, vec![1, 1, 0, 0, 0]);
        result.insert(10, vec![1, 1, 0, 0, 0, 0]);
        result
    }

    #[cfg(test)]
    pub(crate) fn get_value_to_path_map_binary() -> HashMap<usize, Vec<usize>> {
        let mut result = HashMap::new();
        result.insert(0, vec![]);
        result.insert(1, vec![0]);
        result.insert(2, vec![1]);
        result.insert(3, vec![0, 0]);
        result.insert(4, vec![0, 1]);
        result.insert(5, vec![1, 0]);
        result.insert(6, vec![1, 1]);
        result.insert(7, vec![1, 1, 0]);
        result.insert(8, vec![1, 1, 0, 1]);
        result.insert(9, vec![1, 1, 0, 1, 0]);
        result.insert(10, vec![1, 1, 0, 1, 0, 1]);
        result
    }

    #[cfg(test)]
    mod dfs_preorder_tests {
        use super::{
            assert_len, create_binary_tree_for_testing, create_trees_for_testing,
            get_expected_metadata_for_value, get_value_to_path_map, get_value_to_path_map_binary,
        };
        use crate::prelude::*;
        use streaming_iterator::StreamingIterator;

        pub(crate) fn get_expected_order_dfs_preorder() -> [usize; 11] {
            [0, 1, 3, 4, 2, 5, 6, 7, 8, 9, 10]
        }

        #[test]
        fn dfs_preorder_has_correct_order() {
            let expected = get_expected_order_dfs_preorder();

            for mut test_tree in create_trees_for_testing() {
                for (i, value) in test_tree.dfs_preorder_iter().enumerate() {
                    assert_eq!(expected[i], *value);
                }
                assert_len!(expected.len(), test_tree.dfs_preorder_iter());

                for (i, value) in test_tree.dfs_preorder_iter_mut().enumerate() {
                    assert_eq!(expected[i], *value);
                }
                assert_len!(expected.len(), test_tree.dfs_preorder_iter_mut());

                for (i, value) in test_tree.clone().dfs_preorder().enumerate() {
                    assert_eq!(expected[i], value);
                }
                assert_len!(expected.len(), test_tree.dfs_preorder());
            }
        }

        #[test]
        fn binary_dfs_preorder_has_correct_order() {
            let expected = get_expected_order_dfs_preorder();

            let mut test_tree = create_binary_tree_for_testing();
            for (i, value) in test_tree.dfs_preorder_iter().enumerate() {
                assert_eq!(expected[i], *value);
            }
            assert_len!(expected.len(), test_tree.dfs_preorder_iter());

            for (i, value) in test_tree.dfs_preorder_iter_mut().enumerate() {
                assert_eq!(expected[i], *value);
            }
            assert_len!(expected.len(), test_tree.dfs_preorder_iter_mut());

            for (i, value) in test_tree.clone().dfs_preorder().enumerate() {
                assert_eq!(expected[i], value);
            }
            assert_len!(expected.len(), test_tree.dfs_preorder());
        }

        #[test]
        fn dfs_preorder_attach_ancestors_works() {
            let expected = get_expected_order_dfs_preorder();

            for mut test_tree in create_trees_for_testing() {
                let mut i = 0;
                let mut iter_with_metadata = test_tree.dfs_preorder_iter().attach_ancestors();
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
                let mut iter_with_metadata = test_tree.dfs_preorder_iter_mut().attach_ancestors();
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
                let mut iter_with_metadata = test_tree.dfs_preorder().attach_ancestors();
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
        fn dfs_preorder_attach_context_works() {
            let expected = get_expected_order_dfs_preorder();
            let expected_paths = get_value_to_path_map();

            for mut test_tree in create_trees_for_testing() {
                let mut i = 0;
                let mut iter_with_metadata = test_tree.dfs_preorder_iter().attach_context();
                while let Some(value) = iter_with_metadata.next() {
                    assert_eq!(expected[i], *value.ancestors()[value.ancestors().len() - 1]);
                    let expected = get_expected_metadata_for_value(
                        *value.ancestors()[value.ancestors().len() - 1],
                    );
                    for j in 0..expected.len() {
                        assert_eq!(expected[j], *value.ancestors[j]);
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
                let mut iter_with_metadata = test_tree.dfs_preorder_iter_mut().attach_context();
                while let Some(value) = iter_with_metadata.next() {
                    assert_eq!(expected[i], *value.ancestors()[value.ancestors().len() - 1]);
                    let expected = get_expected_metadata_for_value(
                        *value.ancestors()[value.ancestors().len() - 1],
                    );
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
                let mut iter_with_metadata = test_tree.dfs_preorder().attach_context();
                while let Some(value) = iter_with_metadata.next() {
                    assert_eq!(expected[i], value.ancestors()[value.ancestors().len() - 1]);
                    let expected = get_expected_metadata_for_value(
                        value.ancestors()[value.ancestors().len() - 1],
                    );
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
        fn binary_dfs_preorder_attach_ancestors_works() {
            let expected = get_expected_order_dfs_preorder();

            let mut i = 0;
            let test_tree = create_binary_tree_for_testing();
            let mut iter_with_metadata = test_tree.dfs_preorder_iter().attach_ancestors();
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
            let mut iter_with_metadata = test_tree.dfs_preorder_iter_mut().attach_ancestors();
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
                .dfs_preorder()
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

        #[test]
        fn binary_dfs_preorder_attach_context_works() {
            let expected = get_expected_order_dfs_preorder();
            let expected_paths = get_value_to_path_map_binary();

            let mut test_tree = create_binary_tree_for_testing();
            let mut i = 0;
            let mut iter_with_metadata = test_tree.dfs_preorder_iter().attach_context();
            while let Some(value) = iter_with_metadata.next() {
                assert_eq!(expected[i], *value.ancestors()[value.ancestors().len() - 1]);
                let expected = get_expected_metadata_for_value(
                    *value.ancestors()[value.ancestors().len() - 1],
                );
                for j in 0..expected.len() {
                    assert_eq!(expected[j], *value.ancestors[j]);
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
            let mut iter_with_metadata = test_tree.dfs_preorder_iter_mut().attach_context();
            while let Some(value) = iter_with_metadata.next() {
                assert_eq!(expected[i], *value.ancestors()[value.ancestors().len() - 1]);
                let expected = get_expected_metadata_for_value(
                    *value.ancestors()[value.ancestors().len() - 1],
                );
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
            let mut iter_with_metadata = test_tree.dfs_preorder().attach_context();
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

    #[cfg(test)]
    mod dfs_inorder_tests {
        use super::{assert_len, create_binary_tree_for_testing, get_expected_metadata_for_value};
        use crate::prelude::{tests::get_value_to_path_map_binary, *};
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
                let expected = get_expected_metadata_for_value(
                    *value.ancestors()[value.ancestors().len() - 1],
                );
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
                let expected = get_expected_metadata_for_value(
                    *value.ancestors()[value.ancestors().len() - 1],
                );
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
    }

    mod dfs_postorder_tests {
        use super::{
            assert_len, create_binary_tree_for_testing, create_trees_for_testing,
            get_expected_metadata_for_value, get_value_to_path_map,
        };
        use crate::prelude::{tests::get_value_to_path_map_binary, *};
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
                let expected = get_expected_metadata_for_value(
                    *value.ancestors()[value.ancestors().len() - 1],
                );
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
                let expected = get_expected_metadata_for_value(
                    *value.ancestors()[value.ancestors().len() - 1],
                );
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
                    let expected = get_expected_metadata_for_value(
                        *value.ancestors()[value.ancestors().len() - 1],
                    );
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
                    let expected = get_expected_metadata_for_value(
                        *value.ancestors()[value.ancestors().len() - 1],
                    );
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
                    let expected = get_expected_metadata_for_value(
                        value.ancestors()[value.ancestors().len() - 1],
                    );
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
    }

    mod bfs_tests {
        use super::{
            assert_len, create_binary_tree_for_testing, create_trees_for_testing,
            get_expected_metadata_for_value, get_value_to_path_map, get_value_to_path_map_binary,
        };
        use crate::prelude::*;
        use streaming_iterator::StreamingIterator;

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
                    let expected = get_expected_metadata_for_value(
                        *value.ancestors()[value.ancestors().len() - 1],
                    );
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
                let mut iter_with_metadata = test_tree.bfs_iter_mut().attach_context();
                while let Some(value) = iter_with_metadata.next() {
                    assert_eq!(expected[i], *value.ancestors()[value.ancestors().len() - 1]);
                    let expected = get_expected_metadata_for_value(
                        *value.ancestors()[value.ancestors().len() - 1],
                    );
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
                let mut iter_with_metadata = test_tree.bfs().attach_context();
                while let Some(value) = iter_with_metadata.next() {
                    assert_eq!(expected[i], value.ancestors()[value.ancestors().len() - 1]);
                    let expected = get_expected_metadata_for_value(
                        value.ancestors()[value.ancestors().len() - 1],
                    );
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
                let expected = get_expected_metadata_for_value(
                    *value.ancestors()[value.ancestors().len() - 1],
                );
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
            let mut iter_with_metadata = test_tree.bfs_iter_mut().attach_context();
            while let Some(value) = iter_with_metadata.next() {
                assert_eq!(expected[i], *value.ancestors()[value.ancestors().len() - 1]);
                let expected = get_expected_metadata_for_value(
                    *value.ancestors()[value.ancestors().len() - 1],
                );
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
                    value.path
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
    }

    #[cfg(test)]
    mod ancestors_leaves_tests {
        use alloc::{string::ToString, vec};

        use super::{assert_len, create_binary_tree_for_testing, create_trees_for_testing};
        use crate::prelude::*;

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
                        assert!(expected[i].iter().eq(value.iter().map(|val| *val)));
                        i += 1;
                    }
                }

                let mut i = 0;
                for borrowed_iter in get_borrowed_leaves_iters(&test_tree) {
                    assert_len!(
                        expected.len(),
                        borrowed_iter,
                        alloc::format!("Failure at index {}", i.to_string())
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
                    assert!(expected[i].iter().eq(value.iter().map(|val| *val)));
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
                    alloc::format!("Failure at index {}", i.to_string())
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
            let expected_dfs_preorder =
                super::dfs_preorder_tests::get_expected_order_dfs_preorder();
            let expected_leaves = get_expected_order_leaves();
            for mut test_tree in create_trees_for_testing() {
                // interrupt traversal at all points.
                for _ in 0..expected_dfs_preorder.len() {
                    let mut preorder_iter = test_tree.dfs_preorder_iter().attach_ancestors();
                    let mut num_leaves_seen = 0;
                    while let Some(value) = preorder_iter.next() {
                        if *value[value.len() - 1]
                            == expected_leaves[num_leaves_seen]
                                [expected_leaves[num_leaves_seen].len() - 1]
                        {
                            num_leaves_seen += 1;
                        }
                    }

                    let mut preorder_iter_leaves = preorder_iter.leaves();
                    let mut i = 0;
                    while let Some(value) = preorder_iter_leaves.next() {
                        assert!(expected_leaves[i].iter().eq(value.iter().map(|val| *val)));
                        i += 1;
                    }
                    drop(preorder_iter_leaves);

                    let mut preorder_iter_mut =
                        test_tree.dfs_preorder_iter_mut().attach_ancestors();
                    let mut num_leaves_seen = 0;
                    while let Some(value) = preorder_iter_mut.next() {
                        if *value[value.len() - 1]
                            == expected_leaves[num_leaves_seen]
                                [expected_leaves[num_leaves_seen].len() - 1]
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
                            == expected_leaves[num_leaves_seen]
                                [expected_leaves[num_leaves_seen].len() - 1]
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
            let expected_dfs_postorder =
                super::dfs_postorder_tests::get_expected_order_dfs_postorder();
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
                            == expected_leaves[num_leaves_seen]
                                [expected_leaves[num_leaves_seen].len() - 1]
                        {
                            num_leaves_seen += 1;
                        }
                    }

                    let mut postorder_iter_leaves = postorder_iter.leaves();
                    let mut i = 0;
                    while let Some(value) = postorder_iter_leaves.next() {
                        assert!(expected_leaves[i].iter().eq(value.iter().map(|val| *val)));
                        i += 1;
                    }
                    drop(postorder_iter_leaves);

                    let mut postorder_iter_mut =
                        test_tree.dfs_postorder_iter_mut().attach_ancestors();
                    let mut num_leaves_seen = 0;
                    while let Some(value) = postorder_iter_mut.next() {
                        // dont index outside the array!
                        if num_leaves_seen == expected_leaves.len() {
                            continue;
                        }
                        if *value[value.len() - 1]
                            == expected_leaves[num_leaves_seen]
                                [expected_leaves[num_leaves_seen].len() - 1]
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
                            == expected_leaves[num_leaves_seen]
                                [expected_leaves[num_leaves_seen].len() - 1]
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
            let expected_bfs = super::bfs_tests::get_expected_order_bfs();
            let expected_leaves = get_expected_order_leaves();
            for mut test_tree in create_trees_for_testing() {
                // interrupt traversal at all points.
                for _ in 0..expected_bfs.len() {
                    let mut bfs_iter = test_tree.bfs_iter().attach_ancestors();
                    let mut num_leaves_seen = 0;
                    while let Some(value) = bfs_iter.next() {
                        if *value[value.len() - 1]
                            == expected_leaves[num_leaves_seen]
                                [expected_leaves[num_leaves_seen].len() - 1]
                        {
                            num_leaves_seen += 1;
                        }
                    }

                    let mut bfs_iter_leaves = bfs_iter.leaves();
                    let mut i = 0;
                    while let Some(value) = bfs_iter_leaves.next() {
                        assert!(expected_leaves[i].iter().eq(value.iter().map(|val| *val)));
                        i += 1;
                    }
                    drop(bfs_iter_leaves);

                    let mut bfs_iter_mut = test_tree.bfs_iter_mut().attach_ancestors();
                    let mut num_leaves_seen = 0;
                    while let Some(value) = bfs_iter_mut.next() {
                        if *value[value.len() - 1]
                            == expected_leaves[num_leaves_seen]
                                [expected_leaves[num_leaves_seen].len() - 1]
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
                            == expected_leaves[num_leaves_seen]
                                [expected_leaves[num_leaves_seen].len() - 1]
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
            let expected_dfs_preorder =
                super::dfs_preorder_tests::get_expected_order_dfs_preorder();
            let expected_leaves = get_expected_order_leaves();
            let mut test_tree = create_binary_tree_for_testing();
            // interrupt traversal at all points.
            for _ in 0..expected_dfs_preorder.len() {
                let mut preorder_iter = test_tree.dfs_preorder_iter().attach_ancestors();
                let mut num_leaves_seen = 0;
                while let Some(value) = preorder_iter.next() {
                    if *value[value.len() - 1]
                        == expected_leaves[num_leaves_seen]
                            [expected_leaves[num_leaves_seen].len() - 1]
                    {
                        num_leaves_seen += 1;
                    }
                }

                let mut preorder_iter_leaves = preorder_iter.leaves();
                let mut i = 0;
                while let Some(value) = preorder_iter_leaves.next() {
                    assert!(expected_leaves[i].iter().eq(value.iter().map(|val| *val)));
                    i += 1;
                }
                drop(preorder_iter_leaves);

                let mut preorder_iter_mut = test_tree.dfs_preorder_iter_mut().attach_ancestors();
                let mut num_leaves_seen = 0;
                while let Some(value) = preorder_iter_mut.next() {
                    if *value[value.len() - 1]
                        == expected_leaves[num_leaves_seen]
                            [expected_leaves[num_leaves_seen].len() - 1]
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
                        == expected_leaves[num_leaves_seen]
                            [expected_leaves[num_leaves_seen].len() - 1]
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
            let expected_dfs_inorder = super::dfs_inorder_tests::get_expected_order_dfs_inorder();
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
                        == expected_leaves[num_leaves_seen]
                            [expected_leaves[num_leaves_seen].len() - 1]
                    {
                        num_leaves_seen += 1;
                    }
                }

                let mut inorder_iter_leaves = inorder_iter.leaves();
                let mut i = 0;
                while let Some(value) = inorder_iter_leaves.next() {
                    assert!(expected_leaves[i].iter().eq(value.iter().map(|val| *val)));
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
                        == expected_leaves[num_leaves_seen]
                            [expected_leaves[num_leaves_seen].len() - 1]
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
                        == expected_leaves[num_leaves_seen]
                            [expected_leaves[num_leaves_seen].len() - 1]
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
            let expected_dfs_postorder =
                super::dfs_postorder_tests::get_expected_order_dfs_postorder();
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
                        == expected_leaves[num_leaves_seen]
                            [expected_leaves[num_leaves_seen].len() - 1]
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
                    assert!(expected_leaves[i].iter().eq(value.iter().map(|val| *val)));
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
                        == expected_leaves[num_leaves_seen]
                            [expected_leaves[num_leaves_seen].len() - 1]
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
                        == expected_leaves[num_leaves_seen]
                            [expected_leaves[num_leaves_seen].len() - 1]
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
            let expected_bfs = super::bfs_tests::get_expected_order_bfs();
            let expected_leaves = get_expected_order_leaves();
            let mut test_tree = create_binary_tree_for_testing();
            // interrupt traversal at all points.
            for _ in 0..expected_bfs.len() {
                let mut bfs_iter = test_tree.bfs_iter().attach_ancestors();
                let mut num_leaves_seen = 0;
                while let Some(value) = bfs_iter.next() {
                    if *value[value.len() - 1]
                        == expected_leaves[num_leaves_seen]
                            [expected_leaves[num_leaves_seen].len() - 1]
                    {
                        num_leaves_seen += 1;
                    }
                }

                let mut bfs_iter_leaves = bfs_iter.leaves();
                let mut i = 0;
                while let Some(value) = bfs_iter_leaves.next() {
                    assert!(expected_leaves[i].iter().eq(value.iter().map(|val| *val)));
                    i += 1;
                }
                drop(bfs_iter_leaves);

                let mut bfs_iter_mut = test_tree.bfs_iter_mut().attach_ancestors();
                let mut num_leaves_seen = 0;
                while let Some(value) = bfs_iter_mut.next() {
                    if *value[value.len() - 1]
                        == expected_leaves[num_leaves_seen]
                            [expected_leaves[num_leaves_seen].len() - 1]
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
                        == expected_leaves[num_leaves_seen]
                            [expected_leaves[num_leaves_seen].len() - 1]
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

    #[cfg(test)]
    mod leaves_tests {
        use super::{assert_len, create_binary_tree_for_testing, create_trees_for_testing};
        use crate::prelude::*;

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
            let expected_dfs_preorder =
                super::dfs_preorder_tests::get_expected_order_dfs_preorder();
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
            let expected_dfs_postorder =
                super::dfs_postorder_tests::get_expected_order_dfs_postorder();
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
            let expected_bfs = super::bfs_tests::get_expected_order_bfs();
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
            let expected_dfs_preorder =
                super::dfs_preorder_tests::get_expected_order_dfs_preorder();
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
            let expected_dfs_inorder = super::dfs_inorder_tests::get_expected_order_dfs_inorder();
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
            let expected_dfs_postorder =
                super::dfs_postorder_tests::get_expected_order_dfs_postorder();
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
            let expected_bfs = super::bfs_tests::get_expected_order_bfs();
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
    }

    mod get_at_path_tests {
        use alloc::boxed::Box;

        use super::create_binary_tree_for_testing;
        use super::create_tree_for_testing;

        fn get_tree_path_value_pairs() -> Box<[(Box<[usize]>, Option<usize>)]> {
            Box::new([
                (Box::new([]), Some(0)),
                (Box::new([0]), Some(1)),
                (Box::new([1]), Some(2)),
                (Box::new([2]), None),
                (Box::new([0, 0]), Some(3)),
                (Box::new([0, 1]), Some(4)),
                (Box::new([0, 2]), None),
                (Box::new([1, 0]), Some(5)),
                (Box::new([1, 1]), Some(6)),
                (Box::new([1, 2]), None),
                (Box::new([1, 1, 0]), Some(7)),
                (Box::new([1, 1, 1]), None),
                (Box::new([1, 1, 1, 0]), None),
                (Box::new([1, 1, 0, 0]), Some(8)),
                (Box::new([1, 1, 0, 1]), None),
                (Box::new([1, 1, 0, 0, 0]), Some(9)),
                (Box::new([1, 1, 0, 0, 1]), None),
                (Box::new([1, 1, 0, 0, 0, 0]), Some(10)),
                (Box::new([1, 1, 0, 0, 0, 1]), None),
                (Box::new([1, 1, 0, 0, 0, 0, 0]), None),
            ])
        }

        #[test]
        fn tree_at_path() {
            use super::OwnedTreeNode;

            let tree = create_tree_for_testing();
            for path_value_pair in get_tree_path_value_pairs() {
                assert_eq!(
                    path_value_pair.1,
                    tree.clone()
                        .at_path(&path_value_pair.0)
                        .map(|tree| tree.value)
                )
            }
        }

        #[test]
        fn tree_at_path_ref() {
            use super::BorrowedTreeNode;

            let tree = create_tree_for_testing();
            for path_value_pair in get_tree_path_value_pairs() {
                assert_eq!(
                    path_value_pair.1,
                    tree.at_path_ref(&path_value_pair.0).map(|tree| tree.value)
                )
            }
        }

        #[test]
        fn tree_at_path_mut() {
            use super::MutBorrowedTreeNode;

            let mut tree = create_tree_for_testing();
            for path_value_pair in get_tree_path_value_pairs() {
                assert_eq!(
                    path_value_pair.1,
                    tree.at_path_mut(&path_value_pair.0).map(|tree| tree.value)
                )
            }
        }

        fn get_binary_tree_path_value_pairs() -> Box<[(Box<[usize]>, Option<usize>)]> {
            Box::new([
                (Box::new([]), Some(0)),
                (Box::new([0]), Some(1)),
                (Box::new([1]), Some(2)),
                (Box::new([2]), None),
                (Box::new([0, 0]), Some(3)),
                (Box::new([0, 1]), Some(4)),
                (Box::new([0, 2]), None),
                (Box::new([1, 0]), Some(5)),
                (Box::new([1, 1]), Some(6)),
                (Box::new([1, 2]), None),
                (Box::new([1, 1, 0]), Some(7)),
                (Box::new([1, 1, 1]), None),
                (Box::new([1, 1, 1, 0]), None),
                (Box::new([1, 1, 0, 1]), Some(8)),
                (Box::new([1, 1, 0, 0]), None),
                (Box::new([1, 1, 0, 1, 0]), Some(9)),
                (Box::new([1, 1, 0, 1, 1]), None),
                (Box::new([1, 1, 0, 1, 0, 1]), Some(10)),
                (Box::new([1, 1, 0, 1, 0, 0]), None),
                (Box::new([1, 1, 0, 1, 0, 1, 0]), None),
            ])
        }

        #[test]
        fn binary_tree_at_path() {
            use crate::prelude::OwnedBinaryTreeNode;

            let binary_tree = create_binary_tree_for_testing();
            for path_value_pair in get_binary_tree_path_value_pairs() {
                assert_eq!(
                    path_value_pair.1,
                    binary_tree
                        .clone()
                        .at_path(&path_value_pair.0)
                        .map(|tree| tree.value)
                )
            }
        }

        #[test]
        fn binary_tree_at_path_ref() {
            use crate::prelude::BorrowedBinaryTreeNode;

            let binary_tree = create_binary_tree_for_testing();
            for path_value_pair in get_binary_tree_path_value_pairs() {
                assert_eq!(
                    path_value_pair.1,
                    binary_tree
                        .at_path_ref(&path_value_pair.0)
                        .map(|tree| tree.value)
                )
            }
        }

        #[test]
        fn binary_tree_at_path_mut() {
            use crate::prelude::MutBorrowedBinaryTreeNode;

            let mut binary_tree = create_binary_tree_for_testing();
            for path_value_pair in get_binary_tree_path_value_pairs() {
                assert_eq!(
                    path_value_pair.1,
                    binary_tree
                        .at_path_mut(&path_value_pair.0)
                        .map(|tree| tree.value)
                )
            }
        }
    }

    mod prune_tests {
        use alloc::vec::Vec;
        use alloc::{boxed::Box, vec};

        use crate::prelude::{
            tests::{create_binary_tree_for_testing, create_tree_for_testing},
            BinaryTree, OwnedTreeNode, Tree,
        };
        use crate::prelude::{
            BinaryTreeCollectionIterator, BorrowedBinaryTreeNode,
            BorrowedIntoIteratorOfBinaryTrees, BorrowedIntoIteratorOfTrees, BorrowedTreeNode,
            MutBorrowedBinaryTreeNode, MutBorrowedIntoIteratorOfBinaryTrees,
            MutBorrowedIntoIteratorOfTrees, MutBorrowedTreeNode, OwnedBinaryTreeNode,
            OwnedIntoIteratorOfBinaryTrees, OwnedIntoIteratorOfTrees, TreeCollectionIterator,
            TreeCollectionIteratorBase,
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
    }

    mod map_tests {
        use alloc::boxed::Box;
        use alloc::vec::Vec;
        use alloc::{string::ToString, vec};

        use crate::prelude::{
            BinaryTree, BorrowedBinaryTreeNode, BorrowedIntoIteratorOfBinaryTrees,
            BorrowedIntoIteratorOfTrees, BorrowedTreeNode, MutBorrowedBinaryTreeNode,
            MutBorrowedIntoIteratorOfBinaryTrees, MutBorrowedIntoIteratorOfTrees,
            MutBorrowedTreeNode, OwnedBinaryTreeNode, OwnedIntoIteratorOfBinaryTrees,
            OwnedIntoIteratorOfTrees, OwnedTreeNode, Tree,
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
    }

    mod fold_tests {
        use alloc::vec;
        use alloc::vec::Vec;

        use crate::prelude::{
            tests::{create_binary_tree_for_testing, create_tree_for_testing},
            BorrowedBinaryTreeNode, BorrowedIntoIteratorOfBinaryTrees, BorrowedIntoIteratorOfTrees,
            BorrowedTreeNode, MutBorrowedBinaryTreeNode, MutBorrowedIntoIteratorOfBinaryTrees,
            MutBorrowedIntoIteratorOfTrees, MutBorrowedTreeNode, OwnedBinaryTreeNode,
            OwnedIntoIteratorOfBinaryTrees, OwnedIntoIteratorOfTrees, OwnedTreeNode,
        };

        #[test]
        fn fold_tree() {
            let mut tree = create_tree_for_testing();
            assert_eq!(
                55,
                tree.fold_ref(|child_accs, value| child_accs.into_iter().sum::<usize>() + *value)
            );
            assert_eq!(
                55,
                tree.fold_mut(|child_accs, value| child_accs.into_iter().sum::<usize>() + *value)
            );
            assert_eq!(
                55,
                tree.fold(|child_accs, value| child_accs.into_iter().sum::<usize>() + value)
            );
        }

        #[test]
        fn fold_binary_tree() {
            let mut binary_tree = create_binary_tree_for_testing();
            assert_eq!(
                55,
                binary_tree.fold_ref(|child_accs, value| child_accs
                    .into_iter()
                    .flat_map(|opt| opt)
                    .sum::<usize>()
                    + *value)
            );
            assert_eq!(
                55,
                binary_tree.fold_mut(|child_accs, value| child_accs
                    .into_iter()
                    .flat_map(|opt| opt)
                    .sum::<usize>()
                    + *value)
            );
            assert_eq!(
                55,
                binary_tree.fold(|child_accs, value| child_accs
                    .into_iter()
                    .flat_map(|opt| opt)
                    .sum::<usize>()
                    + value)
            );
        }

        #[test]
        fn fold_trees() {
            let mut trees = vec![
                create_tree_for_testing(),
                create_tree_for_testing(),
                create_tree_for_testing(),
            ];

            assert_eq!(
                vec![55, 55, 55],
                trees
                    .fold_each_ref(
                        |children_accs, value| children_accs.into_iter().sum::<usize>() + *value
                    )
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                vec![55, 55, 55],
                trees
                    .fold_each_mut(
                        |children_accs, value| children_accs.into_iter().sum::<usize>() + *value
                    )
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                vec![55, 55, 55],
                trees
                    .fold_each(
                        |children_accs, value| children_accs.into_iter().sum::<usize>() + value
                    )
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn fold_binary_trees() {
            let mut trees = vec![
                create_binary_tree_for_testing(),
                create_binary_tree_for_testing(),
                create_binary_tree_for_testing(),
            ];

            assert_eq!(
                vec![55, 55, 55],
                trees
                    .fold_each_ref(|child_accs, value| child_accs
                        .into_iter()
                        .flat_map(|opt| opt)
                        .sum::<usize>()
                        + *value)
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                vec![55, 55, 55],
                trees
                    .fold_each_mut(|child_accs, value| child_accs
                        .into_iter()
                        .flat_map(|opt| opt)
                        .sum::<usize>()
                        + *value)
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                vec![55, 55, 55],
                trees
                    .fold_each(|child_accs, value| child_accs
                        .into_iter()
                        .flat_map(|opt| opt)
                        .sum::<usize>()
                        + value)
                    .collect::<Vec<_>>()
            );
        }
    }

    macro_rules! assert_len {
        ($expected: expr, $iter: expr) => {
            assert_len!($expected, $iter, "");
        };
        ($expected: expr, $iter: expr, $message: expr) => {
            let mut count = 0;
            $iter.for_each(|_| count += 1);
            assert_eq!($expected, count, "{}", $message);
        };
    }
    use assert_len;

    fn get_expected_metadata_for_value(val: usize) -> &'static [usize] {
        match val {
            0 => &[0],
            1 => &[0, 1],
            2 => &[0, 2],
            3 => &[0, 1, 3],
            4 => &[0, 1, 4],
            5 => &[0, 2, 5],
            6 => &[0, 2, 6],
            7 => &[0, 2, 6, 7],
            8 => &[0, 2, 6, 7, 8],
            9 => &[0, 2, 6, 7, 8, 9],
            10 => &[0, 2, 6, 7, 8, 9, 10],
            _ => panic!("unexpected value"),
        }
    }

    fn create_trees_for_testing() -> Vec<Tree<usize>> {
        vec![create_tree_for_testing()]
    }

    pub(crate) fn create_tree_for_testing() -> Tree<usize> {
        Tree {
            value: 0,
            children: vec![
                Tree {
                    value: 1,
                    children: vec![
                        Tree {
                            value: 3,
                            children: Vec::new(),
                        },
                        Tree {
                            value: 4,
                            children: Vec::new(),
                        },
                    ],
                },
                Tree {
                    value: 2,
                    children: vec![
                        Tree {
                            value: 5,
                            children: Vec::new(),
                        },
                        Tree {
                            value: 6,
                            children: vec![Tree {
                                value: 7,
                                children: vec![Tree {
                                    value: 8,
                                    children: vec![Tree {
                                        value: 9,
                                        children: vec![Tree {
                                            value: 10,
                                            children: Vec::new(),
                                        }],
                                    }],
                                }],
                            }],
                        },
                    ],
                },
            ],
        }
    }

    pub fn create_binary_tree_for_testing() -> BinaryTree<usize> {
        BinaryTree {
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
            right: Some(Box::new(BinaryTree {
                value: 2,
                left: Some(Box::new(BinaryTree {
                    value: 5,
                    left: None,
                    right: None,
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
                                right: Some(Box::new(BinaryTree {
                                    value: 10,
                                    left: None,
                                    right: None,
                                })),
                            })),
                            right: None,
                        })),
                    })),
                    right: None,
                })),
            })),
        }
    }
}
