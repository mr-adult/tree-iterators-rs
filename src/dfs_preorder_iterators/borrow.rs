use streaming_iterator::StreamingIterator;
use crate::prelude::BorrowedTreeNode;
use super::{preorder_streaming_iterator_impl, dfs_preorder_next, advance_dfs};

pub struct BorrowedDFSPreorderIterator<'a, Node>
    where Node: BorrowedTreeNode<'a> {

    root: Option<&'a Node>,
    traversal_stack: Vec<Node::BorrowedChildren>,
}

impl<'a, Node> BorrowedDFSPreorderIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
        
    pub (crate) fn new(root: &'a Node) -> BorrowedDFSPreorderIterator<'a, Node> {
        BorrowedDFSPreorderIterator { 
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
    pub fn attach_ancestors(self) -> BorrowedDFSPreorderIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFSPreOrderIterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for BorrowedDFSPreorderIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
    
    type Item = Node::BorrowedValue;
    dfs_preorder_next!(get_value_and_children_iter);
}

pub struct BorrowedDFSPreorderIteratorWithAncestors<'a, Node>
    where Node: BorrowedTreeNode<'a> {

    root: Option<&'a Node>,
    traversal_stack: Vec<Node::BorrowedChildren>,
    item_stack: Vec<Node::BorrowedValue>,
}


impl<'a, Node> BorrowedDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
        
    pub (crate) fn new(root: &'a Node) -> BorrowedDFSPreorderIteratorWithAncestors<'a, Node> {
        BorrowedDFSPreorderIteratorWithAncestors { 
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    advance_dfs!(get_value_and_children_iter);
}

impl<'a, Node> StreamingIterator for BorrowedDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
    
    type Item = [Node::BorrowedValue];
    preorder_streaming_iterator_impl!();
}
