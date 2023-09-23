use std::fmt::Debug;
use std::iter::FlatMap;
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

use super::bfs_iterators::{
    owned::{
        OwnedBFSIterator, 
        OwnedBinaryBFSIterator
    },
    mut_borrow::{
        MutBorrowedBFSIterator, 
        MutBorrowedBinaryBFSIterator
    },
    borrow::{
        BorrowedBFSIterator, 
        BorrowedBinaryBFSIterator
    },
};
use super::dfs_preorder_iterators::{
    owned::{
        OwnedDFSPreorderIterator, 
        OwnedBinaryDFSPreorderIterator
    },
    mut_borrow::{
        MutBorrowedDFSPreorderIterator, 
        MutBorrowedBinaryDFSPreorderIterator
    },
    borrow::{
        BorrowedDFSPreorderIterator, 
        BorrowedBinaryDFSPreorderIterator
    }
};
use super::dfs_inorder_iterators::{
    owned::OwnedDFSInorderIterator,
    mut_borrow::MutBorrowedDFSInorderIterator,
    borrow::BorrowedDFSInorderIterator,
};
use super::dfs_postorder_iterators::{
    owned::{
        OwnedDFSPostorderIterator, 
        OwnedBinaryDFSPostorderIterator
    },
    mut_borrow::{
        MutBorrowedDFSPostorderIterator, 
        MutBorrowedBinaryDFSPostorderIterator
    },
    borrow::{
        BorrowedDFSPostorderIterator, 
        BorrowedBinaryDFSPostorderIterator
    }
};

/// A default implemenation of a binary tree node. This struct 
/// provides a series of tree traversal utilities to allow 
/// you to easily work with and modify binary trees.
#[derive(Clone, Debug, Default)]
pub struct BinaryTreeNode<T> {
    /// This node's value
    pub value: T,
    /// The left child of the current node.
    pub left: Option<Box<BinaryTreeNode<T>>>,
    /// The right child of the current node.
    pub right: Option<Box<BinaryTreeNode<T>>>,
}

/// A default implemenation of a tree node. This struct 
/// provides a series of tree traversal utilities to allow 
/// you to easily work with and modify arbitrary trees.
#[derive(Clone, Debug, Default)]
pub struct TreeNode<T> {
    /// This node's value
    pub value: T,
    /// The children of the current node.
    pub children: Option<Vec<TreeNode<T>>>
}

/// Helper type to define the BinaryTreeNode's
/// Children iterator type.
pub (crate) type BinaryChildren<T> = FlatMap<
    std::array::IntoIter<
        Option<T>, 
        2
    >, 
    Option<T>, 
    fn(Option<T>) -> Option<T>   
>;

/// A binary tree node where getting its children consumes its value.
pub trait OwnedBinaryTreeNode 
    where Self: Sized {

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
    fn get_value_and_children(self) -> (Self::OwnedValue, Option<BinaryChildren<Self>>) {
        let (value, children) = self.get_value_and_children_binary();
        (
            value,
            Some(
                children.into_iter()
                    .flat_map(opt_to_opt as fn(Option<Self>) -> Option<Self>)
            )
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
    /// ```ignore
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
    /// ```ignore
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
    /// ```ignore
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
    /// ```ignore
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
    where Self: Sized {
    
    /// The value of each node in the tree.
    type OwnedValue: Sized;

    /// The type of iterator that can be used to iterate over each node's children 
    /// collection.
    type OwnedChildren: Iterator<Item = Self>;

    /// This method gets the value and children from this node, consuming it 
    /// in the process. The other methods of this trait assume that the 'Children' 
    /// list does not contain any circular references. If it does, it will create
    /// an infinite loop.
    fn get_value_and_children(self) -> (Self::OwnedValue, Option<Self::OwnedChildren>);

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
    /// ```ignore
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
    /// ```ignore
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
    /// ```ignore
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
    where Self: Sized + 'a {

    /// A mutable reference to the value of each node in the tree.
    type MutBorrowedValue;

    /// This method gets the value and left and right children from this node, 
    /// borrowing it as mutable in the process. The other methods of this trait 
    /// assume that the children do not contain any circular references. If they do, 
    /// it will create an infinite loop.
    fn get_value_and_children_binary_iter_mut(&'a mut self) -> (Self::MutBorrowedValue, [Option<&'a mut Self>; 2]);

    /// This method gets the value and children from this node. The other 
    /// methods of this trait assume that the 'Children' list does not contain 
    /// any circular references. If there are, an inifite loop will result.
    fn get_value_and_children_iter_mut(&'a mut self) -> (Self::MutBorrowedValue, Option<BinaryChildren<&'a mut Self>>) {
        let (value, children) = self.get_value_and_children_binary_iter_mut();
        (
            value,
            Some(
                children.into_iter()
                    .flat_map(opt_to_opt as fn(Option<&'a mut Self>) -> Option<&'a mut Self>)
            )
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
    /// ```ignore
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
    /// ```ignore
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
    /// ```ignore
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
    fn dfs_inorder_iter_mut(&'a mut self) -> MutBorrowedDFSInorderIterator<Self> {
        MutBorrowedDFSInorderIterator::new(self)
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
    /// ```ignore
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
    fn dfs_postorder_iter_mut(&'a mut self) -> MutBorrowedBinaryDFSPostorderIterator<'a, Self> {
        MutBorrowedBinaryDFSPostorderIterator::new(self)
    }
}

/// A tree node where getting its children mutably borrows its value.
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
    /// ```ignore
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
    /// ```ignore
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
    /// ```ignore
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
    fn dfs_postorder_iter_mut(&'a mut self) -> MutBorrowedDFSPostorderIterator<'a, Self> {
        MutBorrowedDFSPostorderIterator::new(self)
    }
}

/// A binary tree node where getting its children borrows its value.
pub trait BorrowedBinaryTreeNode<'a>
    where Self: Sized + 'a {

    /// A reference to the value of each node in the tree.
    type BorrowedValue;

    /// This method gets the value and left and right children from this node, 
    /// borrowing it in the process. The other methods of this trait 
    /// assume that the children do not contain any circular references. If they do, 
    /// it will create an infinite loop.
    fn get_value_and_children_binary_iter(&'a self) -> (Self::BorrowedValue, [Option<&'a Self>; 2]);

    /// This method gets the value and children from this node, consuming it 
    /// in the process. The other methods of this trait assume that the 'Children' 
    /// list does not contain and circular references back to parent nodes.
    fn get_value_and_children_iter(&'a self) -> (Self::BorrowedValue, Option<BinaryChildren<&'a Self>>) {
        let (value, children) = self.get_value_and_children_binary_iter();
        (
            value,
            Some(
                children.into_iter()
                    .flat_map(opt_to_opt as fn(Option<&'a Self>) -> Option<&'a Self>)
            )
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
    /// ```ignore
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
    /// ```ignore
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
    /// ```ignore
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
    fn dfs_inorder_iter(&'a self) -> BorrowedDFSInorderIterator<Self> {
        BorrowedDFSInorderIterator::new(self)
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
    /// ```ignore
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
    fn dfs_postorder_iter(&'a self) -> BorrowedBinaryDFSPostorderIterator<'a, Self> {
        BorrowedBinaryDFSPostorderIterator::new(self)
    }
}

/// A tree node where getting its children borrows its value.
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
    /// ```ignore
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
    /// ```ignore
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
    /// ```ignore
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

impl<T> OwnedBinaryTreeNode for BinaryTreeNode<T> {

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
                }
            ]
        )
    }
}

impl<'a, T> MutBorrowedBinaryTreeNode<'a> for BinaryTreeNode<T> 
    where Self: 'a {
        
    type MutBorrowedValue = &'a mut T;

    fn get_value_and_children_binary_iter_mut(&'a mut self) -> (Self::MutBorrowedValue, [Option<&'a mut Self>; 2]) {
        (
            &mut self.value,
            [
                match &mut self.left {
                    Some(left) => Some(left.as_mut()),
                    None => None
                },
                match &mut self.right {
                    Some(right) => Some(right.as_mut()),
                    None => None
                }
            ]
        )
    }
}

impl<'a, T> BorrowedBinaryTreeNode<'a> for BinaryTreeNode<T> 
    where Self: 'a {

    type BorrowedValue = &'a T;

    fn get_value_and_children_binary_iter(&'a self) -> (Self::BorrowedValue, [Option<&'a Self>; 2]) {
        (
            &self.value,
            [
                match &self.left {
                    Some(left) => Some(left.as_ref()),
                    None => None
                },
                match &self.right {
                    Some(right) => Some(right.as_ref()),
                    None => None
                }
            ]
        )
    }
}

fn opt_to_opt<T>(opt: Option<T>) -> Option<T> {
    opt
}

#[cfg(test)]
pub (crate) mod tests {
    use super::*;

    #[cfg(test)]
    mod dfs_preorder_tests {
        use crate::prelude::*;
        use super::{
            assert_len,
            create_trees_for_testing,
            create_binary_tree_for_testing,
            get_expected_metadata_for_value
        };
        use streaming_iterator::StreamingIterator;

        fn get_expected_order_dfs_preorder() -> [usize; 11] {
            [0,1,3,4,2,5,6,7,8,9,10]
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

            for test_tree in create_trees_for_testing() {
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
            }

            for mut test_tree in create_trees_for_testing() {
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
            }

            for test_tree in create_trees_for_testing() {
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
            let mut iter_with_metadata = create_binary_tree_for_testing().dfs_preorder().attach_ancestors();
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
    mod dfs_inorder_tests {
        use crate::prelude::*;
        use super::{
            assert_len,
            create_binary_tree_for_testing,
            get_expected_metadata_for_value
        };
        use streaming_iterator::StreamingIterator;  

        fn get_expected_order_dfs_inorder() -> [usize; 11] {
            [3,1,4,0,5,2,7,9,10,8,6]
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
    }

    mod dfs_postorder_tests {
        use crate::prelude::*;
        use super::{
            assert_len,
            create_trees_for_testing,
            create_binary_tree_for_testing,
            get_expected_metadata_for_value
        };
        use streaming_iterator::StreamingIterator;  

        fn get_expected_order_dfs_postorder() -> [usize; 11] {
            [3,4,1,5,10,9,8,7,6,2,0]
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
    
            for test_tree in create_trees_for_testing() {
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
            }
    
            for mut test_tree in create_trees_for_testing() {
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
            }
    
            for test_tree in create_trees_for_testing() {
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
            let mut iter_with_metadata = create_binary_tree_for_testing().dfs_postorder().attach_ancestors();
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
        use crate::prelude::*;
        use super::{
            assert_len,
            create_trees_for_testing,
            create_binary_tree_for_testing,
            get_expected_metadata_for_value
        };
        use streaming_iterator::StreamingIterator;  

        fn get_expected_order_bfs() -> [usize; 11] {
            [0,1,2,3,4,5,6,7,8,9,10]
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

    macro_rules! assert_len {
        ($expected: expr, $iter: expr) => {
            let mut count = 0;
            $iter.for_each(|_| count += 1);
            assert_eq!($expected, count);
        };
    }
    pub (crate) use assert_len;

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

    pub fn create_binary_tree_for_testing() -> BinaryTreeNode<usize> {
        BinaryTreeNode { 
            value: 0, 
            left: Some(
                Box::new(
                    BinaryTreeNode {
                        value: 1,
                        left: Some(
                            Box::new(
                                BinaryTreeNode {
                                    value: 3,
                                    left: None,
                                    right: None,
                                }
                            )
                        ),
                        right: Some(
                            Box::new(
                                BinaryTreeNode {
                                    value: 4,
                                    left: None,
                                    right: None,
                                }
                            )
                        ),
                    }
                )
            ), 
            right: Some(
                Box::new(
                    BinaryTreeNode {
                        value: 2,
                        left: Some(
                            Box::new(
                                BinaryTreeNode {
                                    value: 5,
                                    left: None,
                                    right: None,
                                }
                            )
                        ),
                        right: Some(
                            Box::new(
                                BinaryTreeNode {
                                    value: 6,
                                    left: Some(
                                        Box::new(
                                            BinaryTreeNode { 
                                                value: 7, 
                                                left: None, 
                                                right: Some(
                                                    Box::new(
                                                        BinaryTreeNode { 
                                                            value: 8, 
                                                            left: Some(
                                                                Box::new(
                                                                    BinaryTreeNode {
                                                                        value: 9,
                                                                        left: None,
                                                                        right: Some(
                                                                            Box::new(
                                                                                BinaryTreeNode { 
                                                                                    value: 10, 
                                                                                    left: None, 
                                                                                    right: None 
                                                                                }
                                                                            )
                                                                        )
                                                                    }
                                                                )
                                                            ), 
                                                            right: None 
                                                        }
                                                    )
                                                ) 
                                            }
                                        )
                                    ),
                                    right: None,
                                }
                            )
                        )
                    }
                )
            ) 
        }
    }
}