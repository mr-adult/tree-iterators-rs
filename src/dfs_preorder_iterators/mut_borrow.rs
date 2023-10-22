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
    dfs_preorder_next, 
    preorder_streaming_iterator_impl, 
    advance_dfs
};

pub struct MutBorrowedDFSPreorderIterator<'a, Node>
    where Node: MutBorrowedTreeNode<'a> {

    root: Option<&'a mut Node>,
    traversal_stack: Vec<Node::MutBorrowedChildren>,
}

impl<'a, Node> MutBorrowedDFSPreorderIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {
        
    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedDFSPreorderIterator<'a, Node> {
        MutBorrowedDFSPreorderIterator { 
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
    pub fn leaves(self) -> MutBorrowedLeavesIterator<'a, Node, Node::MutBorrowedChildren> {
        MutBorrowedLeavesIterator { 
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
    /// use crate::tree_iterators_rs::examples::create_example_tree;
    /// use crate::tree_iterators_rs::prelude::*;
    /// use streaming_iterator::StreamingIteratorMut;
    /// 
    /// let mut root = create_example_tree();
    /// 
    /// let mut dfs_iter = root.dfs_preorder_iter_mut().attach_ancestors();
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
    pub fn attach_ancestors(self) -> MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                MutBorrowedDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedDFSPreorderIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {
    
    type Item = Node::MutBorrowedValue;
    dfs_preorder_next!(get_value_and_children_iter_mut);
}

pub struct MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node>
    where Node: MutBorrowedTreeNode<'a> {

    root: Option<&'a mut Node>,
    traversal_stack: Vec<Node::MutBorrowedChildren>,
    item_stack: Vec<Node::MutBorrowedValue>,
}


impl<'a, Node> MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {
        
    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node> {
        MutBorrowedDFSPreorderIteratorWithAncestors { 
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }
    
    advance_dfs!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIterator for MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {
    
    type Item = [Node::MutBorrowedValue];
    preorder_streaming_iterator_impl!();
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {
    
    get_mut!();
}


pub struct MutBorrowedBinaryDFSPreorderIterator<'a, Node>
    where Node: MutBorrowedBinaryTreeNode<'a> {

    root: Option<&'a mut Node>,
    traversal_stack: Vec<BinaryChildren<&'a mut Node>>,
}

impl<'a, Node> MutBorrowedBinaryDFSPreorderIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {
        
    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedBinaryDFSPreorderIterator<'a, Node> {
        MutBorrowedBinaryDFSPreorderIterator { 
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
    pub fn leaves(self) -> MutBorrowedBinaryLeavesIterator<'a, Node, BinaryChildren<&'a mut Node>> {
        MutBorrowedBinaryLeavesIterator {
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
    /// let mut dfs_iter = root.dfs_preorder_iter_mut().attach_ancestors();
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
    pub fn attach_ancestors(self) -> MutBorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                MutBorrowedBinaryDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedBinaryDFSPreorderIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    type Item = Node::MutBorrowedValue;
    dfs_preorder_next!(get_value_and_children_iter_mut);
}

pub struct MutBorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node>
    where Node: MutBorrowedBinaryTreeNode<'a> {

    root: Option<&'a mut Node>,
    traversal_stack: Vec<BinaryChildren<&'a mut Node>>,
    item_stack: Vec<Node::MutBorrowedValue>,
}


impl<'a, Node> MutBorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {
        
    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node> {
        MutBorrowedBinaryDFSPreorderIteratorWithAncestors { 
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }
    
    advance_dfs!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIterator for MutBorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    type Item = [Node::MutBorrowedValue];
    preorder_streaming_iterator_impl!();
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    get_mut!();
}
