use alloc::boxed::Box;
use alloc::vec::Vec;

use core::{fmt::Debug, iter::FusedIterator};

use core::iter::FlatMap;

#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

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

/// A default implemenation of a binary tree node. This struct
/// provides a series of tree traversal utilities to allow
/// you to easily work with and modify binary trees.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BinaryTree<T> {
    /// This node's value
    pub value: T,
    /// The left child of the current node.
    pub left: Option<Box<BinaryTree<T>>>,
    /// The right child of the current node.
    pub right: Option<Box<BinaryTree<T>>>,
}

/// A default implemenation of a tree node. This struct
/// provides a series of tree traversal utilities to allow
/// you to easily work with and modify arbitrary trees.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tree<T> {
    /// This node's value
    pub value: T,
    /// The children of the current node.
    pub children: Vec<Tree<T>>,
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
    ///
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
    ///
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
    ///
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
    ///
    fn dfs_postorder(self) -> OwnedBinaryDFSPostorderIterator<Self> {
        OwnedBinaryDFSPostorderIterator::new(self)
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
    ///
    fn dfs_postorder(self) -> OwnedDFSPostorderIterator<Self> {
        OwnedDFSPostorderIterator::new(self)
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
    ///
    fn bfs_iter_mut(&'a mut self) -> MutBorrowedBinaryBFSIterator<'_, Self> {
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
    ///
    fn dfs_preorder_iter_mut(&'a mut self) -> MutBorrowedBinaryDFSPreorderIterator<'_, Self> {
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
    ///
    fn dfs_inorder_iter_mut(&'a mut self) -> MutBorrowedDFSInorderIterator<'_, Self> {
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
    ///
    fn dfs_postorder_iter_mut(&'a mut self) -> MutBorrowedBinaryDFSPostorderIterator<'_, Self> {
        MutBorrowedBinaryDFSPostorderIterator::new(self)
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
    ///
    fn bfs_iter_mut(&'a mut self) -> MutBorrowedBFSIterator<'_, Self> {
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
    ///
    fn dfs_preorder_iter_mut(&'a mut self) -> MutBorrowedDFSPreorderIterator<'_, Self> {
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
    ///
    fn dfs_postorder_iter_mut(&'a mut self) -> MutBorrowedDFSPostorderIterator<'_, Self> {
        MutBorrowedDFSPostorderIterator::new(self)
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
    ///
    fn bfs_iter(&'a self) -> BorrowedBinaryBFSIterator<'_, Self> {
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
    ///
    fn dfs_preorder_iter(&'a self) -> BorrowedBinaryDFSPreorderIterator<'_, Self> {
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
    ///
    fn dfs_inorder_iter(&'a self) -> BorrowedDFSInorderIterator<'_, Self> {
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
    ///
    fn dfs_postorder_iter(&'a self) -> BorrowedBinaryDFSPostorderIterator<'_, Self> {
        BorrowedBinaryDFSPostorderIterator::new(self)
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
    ///
    fn bfs_iter(&'a self) -> BorrowedBFSIterator<'_, Self> {
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
    ///
    fn dfs_preorder_iter(&'a self) -> BorrowedDFSPreorderIterator<'_, Self> {
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
    ///
    fn dfs_postorder_iter(&'a self) -> BorrowedDFSPostorderIterator<'_, Self> {
        BorrowedDFSPostorderIterator::new(self)
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

fn opt_to_opt<T>(opt: Option<T>) -> Option<T> {
    opt
}

#[cfg(test)]
use streaming_iterator::StreamingIterator;

#[cfg(test)]
pub(crate) mod tests {
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
        vec![create_tree_for_testing(Vec::new())]
    }

    pub(crate) fn create_tree_for_testing(empty_children_list: Vec<Tree<usize>>) -> Tree<usize> {
        Tree {
            value: 0,
            children: vec![
                Tree {
                    value: 1,
                    children: vec![
                        Tree {
                            value: 3,
                            children: empty_children_list.clone(),
                        },
                        Tree {
                            value: 4,
                            children: empty_children_list.clone(),
                        },
                    ],
                },
                Tree {
                    value: 2,
                    children: vec![
                        Tree {
                            value: 5,
                            children: empty_children_list.clone(),
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
                                            children: empty_children_list.clone(),
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
