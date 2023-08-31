use streaming_iterator::StreamingIterator;

use crate::prelude::OwnedTreeNode;

use super::{dfs_preorder_next, preorder_streaming_iterator_impl, advance_dfs};

pub struct OwnedDFSPreorderIterator<Node>
    where Node: OwnedTreeNode {

    root: Option<Node>,
    traversal_stack: Vec<Node::OwnedChildren>,
}

impl<Node> OwnedDFSPreorderIterator<Node> 
    where Node: OwnedTreeNode {
        
    pub (crate) fn new(root: Node) -> OwnedDFSPreorderIterator<Node> {
        OwnedDFSPreorderIterator { 
            root: Some(root),
            traversal_stack: Vec::new()
        }
    }

        /// This method retrieves an iterator that can be used to perform
    /// Depth First Preorder searches of a tree.
    /// 
    /// A Depth First Preorder search is defined as:
    /// 
    /// A tree traversal that involves depth-first searching a tree 
    /// from the top down. this 
    /// traversal type would traverse the elements and yield slices in
    /// the following order. Note for each slice, the current node is
    /// at index slice.len() - 1, the root is at index 0 and all other 
    /// ancestors are found in between.
    /// - \[0\],
    /// - \[0, 1\],
    /// - \[0, 1, 3\],
    /// - \[0, 1, 4\],
    /// - \[0, 2\],
    /// - \[0, 2, 5\],
    /// - \[0, 2, 6\],
    /// - \[0, 2, 6, 7\],
    /// - \[0, 2, 6, 7, 8\],
    /// - \[0, 2, 6, 7, 8, 9\],
    /// - \[0, 2, 6, 7, 8, 9, 10\],
    /// 
    /// In this traversal, each node will only be traversed before any
    /// of its children have been traversed.
    /// ```
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
    /// meaning that the values can no longer be directly save and used 
    /// across loop iterations. The references to the nodes themselves 
    /// are still valid across the entirety of the loop, but you must 
    /// extract them from their containing slice to reuse them. This
    /// will incure a performance penalty that this library does not
    /// assume you want.
    /// 
    /// Since this iterator is no longer a Rust Iterator, for loops will
    /// no longer work. See details on how to work around this in the 
    /// [streaming-iterator](https://crates.io/crates/streaming-iterator) crate.
    /// 
    pub fn attach_ancestors(self) -> OwnedDFSPreorderIteratorWithAncestors<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFSPostOrderIterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<Node> Iterator for OwnedDFSPreorderIterator<Node> 
    where Node: OwnedTreeNode {
    
    type Item = Node::OwnedValue;
    dfs_preorder_next!(get_value_and_children);
}

pub struct OwnedDFSPreorderIteratorWithAncestors<Node>
    where Node: OwnedTreeNode {

    root: Option<Node>,
    traversal_stack: Vec<Node::OwnedChildren>,
    item_stack: Vec<Node::OwnedValue>,
}


impl<'a, Node> OwnedDFSPreorderIteratorWithAncestors<Node> 
    where Node: OwnedTreeNode {
        
    pub (crate) fn new(root: Node) -> OwnedDFSPreorderIteratorWithAncestors<Node> {
        OwnedDFSPreorderIteratorWithAncestors { 
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }
    
    advance_dfs!(get_value_and_children);
}

impl<'a, Node> StreamingIterator for OwnedDFSPreorderIteratorWithAncestors<Node> 
    where Node: OwnedTreeNode {
    
    type Item = [Node::OwnedValue];
    preorder_streaming_iterator_impl!();
}