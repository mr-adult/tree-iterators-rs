use streaming_iterator::StreamingIterator;

use crate::prelude::BorrowedBinaryTreeNode;

use super::{
    dfs_inorder_next, 
    dfs_inorder_streaming_iterator_impl
};

pub struct BorrowedDFSInorderIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {
    
    right_stack: Vec<Option<&'a Node>>,
    item_stack: Vec<Node::BorrowedValue>,
    moved: bool,
}

impl<'a, Node> BorrowedDFSInorderIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a Node) -> BorrowedDFSInorderIterator<'a, Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        BorrowedDFSInorderIterator {
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
    pub fn attach_ancestors(mut self) -> BorrowedDFSInorderIteratorWithAncestors<'a, Node> {
        let root = self.right_stack.pop();
        match self.moved {
            true => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            false => BorrowedDFSInorderIteratorWithAncestors::new(root.unwrap().unwrap())
        }
    }
}

impl<'a, Node> Iterator for BorrowedDFSInorderIterator<'a, Node>
    where Node: BorrowedBinaryTreeNode<'a> {
    
    type Item = Node::BorrowedValue;
    
    dfs_inorder_next!(get_value_and_left_right_iter);
}

pub struct BorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {
    
    right_stack: Vec<Option<&'a Node>>,
    item_stack: Vec<Node::BorrowedValue>,
    has_gone_right_stack: Vec<bool>,
    last_iteration_was_just_a_pop: bool,
}

impl<'a, Node> BorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a Node) -> BorrowedDFSInorderIteratorWithAncestors<Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        BorrowedDFSInorderIteratorWithAncestors {
            last_iteration_was_just_a_pop: false,
            right_stack,
            item_stack: Vec::new(),
            has_gone_right_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedDFSInorderIteratorWithAncestors<'a, Node>
    where Node: BorrowedBinaryTreeNode<'a> {
    
    type Item = [Node::BorrowedValue];
    
    dfs_inorder_streaming_iterator_impl!(get_value_and_left_right_iter);
}