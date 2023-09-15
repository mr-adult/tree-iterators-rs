use streaming_iterator::StreamingIterator;

use crate::prelude::MutBorrowedBinaryTreeNode;

use super::{
    dfs_inorder_next, 
    dfs_inorder_streaming_iterator_impl
};

pub struct MutBorrowedDFSInorderIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    right_stack: Vec<Option<&'a mut Node>>,
    item_stack: Vec<Node::MutBorrowedValue>,
    moved: bool
}

impl<'a, Node> MutBorrowedDFSInorderIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedDFSInorderIterator<'a, Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        MutBorrowedDFSInorderIterator {
            right_stack,
            item_stack: Vec::new(),
            moved: false,
        }
    }

    /// This method retrieves a streaming iterator that can be used to perform
    /// Depth First In Order searches of a tree.
    /// 
    /// A Depth First In Order search (referred to as DFS In Order) 
    /// is defined as:
    /// 
    /// A tree traversal that involves depth-first searching a tree 
    /// from the left to the right. Given a tree of the following shape, this 
    /// traversal type would traverse the elements and yield slices in
    /// the following order. Note for each slice, the current node is
    /// at index slice.len() - 1, the root is at index 0 and all other 
    /// ancestors are found in between.
    /// - \[0, 1, 3\], 
    /// - \[0, 1\], 
    /// - \[0, 1, 4\], 
    /// - \[0\]
    /// - \[0, 2, 5\], 
    /// - \[0, 2\], 
    /// - \[0, 2, 6, 7\], 
    /// - \[0, 2, 6, 7, 8, 9\], 
    /// - \[0, 2, 6, 7, 8, 9, 10\], 
    /// - \[0, 3, 6, 7, 8\], 
    /// - \[0, 2, 6\], 
    /// 
    /// In this traversal, each node will only be traversed after its
    /// left child and before its right child has been traversed.
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
    pub fn attach_ancestors(mut self) -> MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> {
        let root = self.right_stack.pop();
        match self.moved {
            true => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            false => MutBorrowedDFSInorderIteratorWithAncestors::new(root.unwrap().unwrap())
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedDFSInorderIterator<'a, Node>
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    type Item = Node::MutBorrowedValue;
    
    dfs_inorder_next!(get_value_and_left_right_iter_mut);
}

pub struct MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    right_stack: Vec<Option<&'a mut Node>>,
    item_stack: Vec<Node::MutBorrowedValue>,
    has_gone_right_stack: Vec<bool>,
    last_iteration_was_just_a_pop: bool,
}

impl<'a, Node> MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        MutBorrowedDFSInorderIteratorWithAncestors {
            last_iteration_was_just_a_pop: false,
            right_stack,
            item_stack: Vec::new(),
            has_gone_right_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedDFSInorderIteratorWithAncestors<'a, Node>
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    type Item = [Node::MutBorrowedValue];
    
    dfs_inorder_streaming_iterator_impl!(get_value_and_left_right_iter_mut);
}