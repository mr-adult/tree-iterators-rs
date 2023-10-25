use alloc::vec::Vec;
use streaming_iterator::{
    StreamingIterator,
    StreamingIteratorMut
};
use crate::{
    prelude::{
        MutBorrowedTreeNode, 
        BinaryChildren, 
        MutBorrowedBinaryTreeNode
    }, 
    leaves_iterators::depth_first::mut_borrow::{
        MutBorrowedLeavesIterator, 
        MutBorrowedBinaryLeavesIterator
    }
};

use super::{
    get_mut,
    dfs_postorder_next, 
    postorder_streaming_iterator_impl
};

pub struct MutBorrowedDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    root: Option<&'a mut Node>,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_stack: Vec<Node::MutBorrowedChildren>
}

impl<'a, Node> MutBorrowedDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedDFSPostorderIterator<'a, Node> {
        MutBorrowedDFSPostorderIterator { 
            root: Some(root),
            item_stack: Vec::new(), 
            traversal_stack: Vec::new() 
        }
    }

    /// This method converts the current Depth First Search iterator into 
    /// an iterator that will yield only the leaves of the tree. Iteration
    /// proceeds in a Depth First Postorder Search order.
    /// 
    /// A leaf is defined as:
    /// 
    /// Any tree node that has no children. Given a tree of the following shape, 
    /// this iterator would yield values in the following order:
    /// 3, 4, 5, 10
    /// 
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
    pub fn leaves(self) -> MutBorrowedLeavesIterator<'a, Node, Node::MutBorrowedChildren> {
        MutBorrowedLeavesIterator { 
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new()
        }
    }

    /// WARNING: The slice returned by this iterator points to internal iterator
    /// state. Any changes to the slice's structure/order made using the StreamingIteratorMut 
    /// API will be carried through the remaining iterations. This can result in
    /// unexpected behaviors if you are not careful.
    /// 
    /// This method retrieves a streaming iterator that can be used to perform
    /// Depth First Postorder searches of a tree.
    /// 
    /// A Depth First Postorder search (referred to as DFS Postorder) 
    /// is defined as:
    /// 
    /// A tree traversal that involves depth-first searching a tree 
    /// from the bottom up. Given a tree of the following shape, this 
    /// traversal type would traverse the elements and yield slices in
    /// the following order. Note for each slice, the current node is
    /// at index slice.len() - 1, the root is at index 0 and all other 
    /// ancestors are found in between.
    /// - \[0, 1, 3\], 
    /// - \[0, 1, 4\], 
    /// - \[0, 1\], 
    /// - \[0, 2, 5\], 
    /// - \[0, 2, 6, 7, 8, 9, 10\], 
    /// - \[0, 2, 6, 7, 8, 9\], 
    /// - \[0, 3, 6, 7, 8\], 
    /// - \[0, 2, 6, 7\], 
    /// - \[0, 2, 6\], 
    /// - \[0, 2\], 
    /// - \[0\]
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
    /// More technical details:
    /// 
    /// This method attaches the ancestors of the node to the iterator.
    /// This operation transforms the iterator into a StreamingIterator,
    /// meaning that the values can no longer be directly saved and used 
    /// across loop iterations. The references to the nodes themselves 
    /// are still valid across the entirety of the loop, but you must 
    /// extract them from their containing slice to reuse them. This
    /// will incur a performance penalty that this library does not
    /// assume you want.
    /// 
    /// Since this iterator is no longer a Rust Iterator, for loops will
    /// no longer work. See details on how to work around this in the 
    /// [streaming-iterator](https://crates.io/crates/streaming-iterator) crate.
    /// 
    /// The mutable version of this method exists because it is inherently useful 
    /// to modify the nodes of the tree based on additional metadata provided by 
    /// this stack of nodes style iterator. Modifying the values within the 
    /// slices returned by this iterator is a safe operation and will never cause 
    /// problems with the iterator.
    /// 
    /// As an example, given the following tree, we could modify each value
    /// as we go to be the count of descendant nodes.
    /// 
    /// We would start with the tree:
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
    /// The code to modify the tree would look like this:
    /// ```rust
    /// use crate::tree_iterators_rs::examples::create_example_tree;
    /// use crate::tree_iterators_rs::prelude::*;
    /// use streaming_iterator::StreamingIteratorMut;
    /// 
    /// let mut root = create_example_tree();
    /// 
    /// let mut dfs_iter = root.dfs_postorder_iter_mut().attach_ancestors();
    /// while let Some(slice) = dfs_iter.next_mut() {
    ///     *slice[slice.len() - 1] = 0;
    ///     for i in 1..slice.len() {
    ///         *slice[slice.len() - i] += 1;
    ///     }
    /// }
    /// ```
    /// 
    /// After modifying each node during the traversal we could end
    /// with this tree:
    /// ```ignore
    ///        6
    ///       / \
    ///      1   5
    ///     / \ / \
    ///    0  0 0  4
    ///           /
    ///          3
    ///           \
    ///            2
    ///           /
    ///          1
    ///           \
    ///           0
    /// ```
    /// 
    pub fn attach_ancestors(self) -> MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                MutBorrowedDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    type Item = Node::MutBorrowedValue;
    dfs_postorder_next!(get_value_and_children_iter_mut);
}

pub struct MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node>
    where Node: MutBorrowedTreeNode<'a> {

    root: Option<&'a mut Node>,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_stack: Vec<Node::MutBorrowedChildren>,
}

impl<'a, Node> MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    fn new(root: &'a mut Node) -> MutBorrowedDFSPostorderIteratorWithAncestors<'_, Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    type Item = [Node::MutBorrowedValue];
    postorder_streaming_iterator_impl!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    get_mut!();
}

pub struct MutBorrowedBinaryDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    root: Option<&'a mut Node>,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_stack: Vec<BinaryChildren<&'a mut Node>>
}

impl<'a, Node> MutBorrowedBinaryDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedBinaryDFSPostorderIterator<'a, Node> {
        MutBorrowedBinaryDFSPostorderIterator { 
            root: Some(root),
            item_stack: Vec::new(), 
            traversal_stack: Vec::new() 
        }
    }

    /// This method converts the current Depth First Search iterator into 
    /// an iterator that will yield only the leaves of the tree. Iteration
    /// proceeds in a Depth First Postorder Search order.
    /// 
    /// A leaf is defined as:
    /// 
    /// Any tree node that has no children. Given a tree of the following shape, 
    /// this iterator would yield values in the following order:
    /// 3, 4, 5, 10
    /// 
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
    pub fn leaves(self) -> MutBorrowedBinaryLeavesIterator<'a, Node, BinaryChildren<&'a mut Node>> {
        MutBorrowedBinaryLeavesIterator { 
            root: self.root, 
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new()
        }
    }

    /// WARNING: The slice returned by this iterator points to internal iterator
    /// state. Any changes to the slice's structure/order made using the StreamingIteratorMut 
    /// API will be carried through the remaining iterations. This can result in
    /// unexpected behaviors if you are not careful.
    /// 
    /// This method retrieves a streaming iterator that can be used to perform
    /// Depth First Postorder searches of a tree.
    /// 
    /// A Depth First Postorder search (referred to as DFS Postorder) 
    /// is defined as:
    /// 
    /// A tree traversal that involves depth-first searching a tree 
    /// from the bottom up. Given a tree of the following shape, this 
    /// traversal type would traverse the elements and yield slices in
    /// the following order. Note for each slice, the current node is
    /// at index slice.len() - 1, the root is at index 0 and all other 
    /// ancestors are found in between.
    /// - \[0, 1, 3\], 
    /// - \[0, 1, 4\], 
    /// - \[0, 1\], 
    /// - \[0, 2, 5\], 
    /// - \[0, 2, 6, 7, 8, 9, 10\], 
    /// - \[0, 2, 6, 7, 8, 9\], 
    /// - \[0, 3, 6, 7, 8\], 
    /// - \[0, 2, 6, 7\], 
    /// - \[0, 2, 6\], 
    /// - \[0, 2\], 
    /// - \[0\]
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
    /// More technical details:
    /// 
    /// This method attaches the ancestors of the node to the iterator.
    /// This operation transforms the iterator into a StreamingIterator,
    /// meaning that the values can no longer be directly saved and used 
    /// across loop iterations. The references to the nodes themselves 
    /// are still valid across the entirety of the loop, but you must 
    /// extract them from their containing slice to reuse them. This
    /// will incur a performance penalty that this library does not
    /// assume you want.
    /// 
    /// Since this iterator is no longer a Rust Iterator, for loops will
    /// no longer work. See details on how to work around this in the 
    /// [streaming-iterator](https://crates.io/crates/streaming-iterator) crate.
    /// 
    /// The mutable version of this method exists because it is inherently useful 
    /// to modify the nodes of the tree based on additional metadata provided by 
    /// this stack of nodes style iterator. Modifying the values within the 
    /// slices returned by this iterator is a safe operation and will never cause 
    /// problems with the iterator.
    /// 
    /// As an example, given the following tree, we could modify each value
    /// as we go to be the count of descendant nodes.
    /// 
    /// We would start with the tree:
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
    /// The code to modify the tree would look like this:
    /// ```rust
    /// use crate::tree_iterators_rs::examples::create_example_binary_tree;
    /// use crate::tree_iterators_rs::prelude::*;
    /// use streaming_iterator::StreamingIteratorMut;
    /// 
    /// let mut root = create_example_binary_tree();
    /// 
    /// let mut dfs_iter = root.dfs_postorder_iter_mut().attach_ancestors();
    /// while let Some(slice) = dfs_iter.next_mut() {
    ///     *slice[slice.len() - 1] = 0;
    ///     for i in 1..slice.len() {
    ///         *slice[slice.len() - i] += 1;
    ///     }
    /// }
    /// ```
    /// 
    /// After modifying each node during the traversal we could end
    /// with this tree:
    /// ```ignore
    ///        6
    ///       / \
    ///      1   5
    ///     / \ / \
    ///    0  0 0  4
    ///           /
    ///          3
    ///           \
    ///            2
    ///           /
    ///          1
    ///           \
    ///           0
    /// ```
    /// 
    pub fn attach_ancestors(self) -> MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                MutBorrowedBinaryDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedBinaryDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    type Item = Node::MutBorrowedValue;
    dfs_postorder_next!(get_value_and_children_iter_mut);
}

pub struct MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node>
    where Node: MutBorrowedBinaryTreeNode<'a> {

    root: Option<&'a mut Node>,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_stack: Vec<BinaryChildren<&'a mut Node>>,
}

impl<'a, Node> MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    fn new(root: &'a mut Node) -> MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'_, Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    type Item = [Node::MutBorrowedValue];
    postorder_streaming_iterator_impl!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    get_mut!();
}