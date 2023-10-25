use alloc::vec::Vec;
use streaming_iterator::{
    StreamingIterator,
    StreamingIteratorMut
};

use crate::{
    prelude::{
        OwnedTreeNode, 
        BinaryChildren, 
        OwnedBinaryTreeNode
    }, 
    leaves_iterators::depth_first::owned::{
        OwnedLeavesIterator, 
        OwnedBinaryLeavesIterator
    }
};

use super::{dfs_preorder_next, preorder_streaming_iterator_impl, advance_dfs, get_mut};

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

    /// This method converts the current Depth First Search iterator into 
    /// an iterator that will yield only the leaves of the tree. Iteration
    /// proceeds in a Depth First Postorder Search order. This may not make
    /// intuitive sense at first, but in order for the lazy iterators of this 
    /// library to know a node is a leaf of the tree, postorder must be used.
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
    pub fn leaves(self) -> OwnedLeavesIterator<Node> {
        OwnedLeavesIterator {
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
    /// let mut dfs_iter = root.dfs_preorder().attach_ancestors();
    /// while let Some(slice) = dfs_iter.next_mut() {
    ///     *slice.get_mut(slice.len() - 1).unwrap() = 0;
    ///     for i in 1..slice.len() {
    ///         *slice.get_mut(slice.len() - i).unwrap() += 1;
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
    pub fn attach_ancestors(self) -> OwnedDFSPreorderIteratorWithAncestors<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
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

impl<'a, Node> StreamingIteratorMut for OwnedDFSPreorderIteratorWithAncestors<Node> 
    where Node: OwnedTreeNode {

    get_mut!();
}

pub struct OwnedBinaryDFSPreorderIterator<Node>
    where Node: OwnedBinaryTreeNode {

    root: Option<Node>,
    traversal_stack: Vec<BinaryChildren<Node>>,
}

impl<Node> OwnedBinaryDFSPreorderIterator<Node> 
    where Node: OwnedBinaryTreeNode {
        
    pub (crate) fn new(root: Node) -> OwnedBinaryDFSPreorderIterator<Node> {
        OwnedBinaryDFSPreorderIterator { 
            root: Some(root),
            traversal_stack: Vec::new()
        }
    }

    /// This method converts the current Depth First Search iterator into 
    /// an iterator that will yield only the leaves of the tree. Iteration
    /// proceeds in a Depth First Postorder Search order. This may not make
    /// intuitive sense at first, but in order for the lazy iterators of this 
    /// library to know a node is a leaf of the tree, postorder must be used.
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
    pub fn leaves(self) -> OwnedBinaryLeavesIterator<Node, BinaryChildren<Node>> {
        OwnedBinaryLeavesIterator {
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    /// WARNING: The slice returned by this iterator points to internal iterator
    /// state. Any changes to the slice's structure/order made using the StreamingIteratorMut 
    /// API will be carried through the remaining iterations. This can result in
    /// unexpected behaviors if you are not careful.
    /// 
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
    /// let mut dfs_iter = root.dfs_preorder().attach_ancestors();
    /// while let Some(slice) = dfs_iter.next_mut() {
    ///     *slice.get_mut(slice.len() - 1).unwrap() = 0;
    ///     for i in 1..slice.len() {
    ///         *slice.get_mut(slice.len() - i).unwrap() += 1;
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
    pub fn attach_ancestors(self) -> OwnedBinaryDFSPreorderIteratorWithAncestors<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedBinaryDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<Node> Iterator for OwnedBinaryDFSPreorderIterator<Node> 
    where Node: OwnedBinaryTreeNode {
    
    type Item = Node::OwnedValue;
    dfs_preorder_next!(get_value_and_children);
}

pub struct OwnedBinaryDFSPreorderIteratorWithAncestors<Node>
    where Node: OwnedBinaryTreeNode {

    root: Option<Node>,
    traversal_stack: Vec<BinaryChildren<Node>>,
    item_stack: Vec<Node::OwnedValue>,
}


impl<'a, Node> OwnedBinaryDFSPreorderIteratorWithAncestors<Node> 
    where Node: OwnedBinaryTreeNode {
        
    pub (crate) fn new(root: Node) -> OwnedBinaryDFSPreorderIteratorWithAncestors<Node> {
        OwnedBinaryDFSPreorderIteratorWithAncestors { 
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }
    
    advance_dfs!(get_value_and_children);
}

impl<'a, Node> StreamingIterator for OwnedBinaryDFSPreorderIteratorWithAncestors<Node> 
    where Node: OwnedBinaryTreeNode {
    
    type Item = [Node::OwnedValue];
    preorder_streaming_iterator_impl!();
}

impl<'a, Node> StreamingIteratorMut for OwnedBinaryDFSPreorderIteratorWithAncestors<Node> 
    where Node: OwnedBinaryTreeNode {
    
    get_mut!();
}