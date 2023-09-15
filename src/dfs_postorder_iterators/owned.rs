use streaming_iterator::StreamingIterator;
use crate::prelude::{OwnedTreeNode, BinaryChildren, OwnedBinaryTreeNode};

use super::{dfs_postorder_next, postorder_streaming_iterator_impl};

pub struct OwnedDFSPostorderIterator<Node> 
    where Node: OwnedTreeNode {

    root: Option<Node>,
    item_stack: Vec<Node::OwnedValue>,
    traversal_stack: Vec<Node::OwnedChildren>
}

impl<Node> OwnedDFSPostorderIterator<Node> 
    where Node: OwnedTreeNode {

    pub (crate) fn new(root: Node) -> OwnedDFSPostorderIterator<Node> {
        OwnedDFSPostorderIterator { 
            root: Some(root),
            item_stack: Vec::new(), 
            traversal_stack: Vec::new() 
        }
    }

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
    pub fn attach_ancestors(self) -> OwnedDFSPostorderIteratorWithAncestors<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<Node> Iterator for OwnedDFSPostorderIterator<Node> 
    where Node: OwnedTreeNode {

    type Item = Node::OwnedValue;
    dfs_postorder_next!(get_value_and_children);
}

pub struct OwnedDFSPostorderIteratorWithAncestors<Node>
    where Node: OwnedTreeNode {

    root: Option<Node>,
    item_stack: Vec<Node::OwnedValue>,
    traversal_stack: Vec<Node::OwnedChildren>,
}

impl<'a, Node> OwnedDFSPostorderIteratorWithAncestors<Node> 
    where Node: OwnedTreeNode {

    fn new(root: Node) -> OwnedDFSPostorderIteratorWithAncestors<Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }
}

type TreeValueStack<T> = [T];

impl<'a, Node> StreamingIterator for OwnedBinaryDFSPostorderIteratorWithAncestors<Node> 
    where Node: OwnedBinaryTreeNode {

    type Item = TreeValueStack<Node::OwnedValue>;
    postorder_streaming_iterator_impl!(get_value_and_children);
}

pub struct OwnedBinaryDFSPostorderIterator<Node> 
    where Node: OwnedBinaryTreeNode {

    root: Option<Node>,
    item_stack: Vec<Node::OwnedValue>,
    traversal_stack: Vec<BinaryChildren<Node>>
}

impl<Node> OwnedBinaryDFSPostorderIterator<Node> 
    where Node: OwnedBinaryTreeNode {

    pub (crate) fn new(root: Node) -> OwnedBinaryDFSPostorderIterator<Node> {
        OwnedBinaryDFSPostorderIterator { 
            root: Some(root),
            item_stack: Vec::new(), 
            traversal_stack: Vec::new() 
        }
    }

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
    pub fn attach_ancestors(self) -> OwnedBinaryDFSPostorderIteratorWithAncestors<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedBinaryDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<Node> Iterator for OwnedBinaryDFSPostorderIterator<Node> 
    where Node: OwnedBinaryTreeNode {

    type Item = Node::OwnedValue;
    dfs_postorder_next!(get_value_and_children);
}

pub struct OwnedBinaryDFSPostorderIteratorWithAncestors<Node>
    where Node: OwnedBinaryTreeNode {

    root: Option<Node>,
    item_stack: Vec<Node::OwnedValue>,
    traversal_stack: Vec<BinaryChildren<Node>>,
}

impl<'a, Node> OwnedBinaryDFSPostorderIteratorWithAncestors<Node> 
    where Node: OwnedBinaryTreeNode {

    fn new(root: Node) -> OwnedBinaryDFSPostorderIteratorWithAncestors<Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for OwnedDFSPostorderIteratorWithAncestors<Node> 
    where Node: OwnedTreeNode {

    type Item = TreeValueStack<Node::OwnedValue>;
    postorder_streaming_iterator_impl!(get_value_and_children);
}