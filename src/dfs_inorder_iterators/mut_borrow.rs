use streaming_iterator::{
    StreamingIterator,
    StreamingIteratorMut
};

use crate::{
    prelude::MutBorrowedBinaryTreeNode, 
    leaves_iterators::depth_first::mut_borrow::MutBorrowedBinaryLeavesIterator,
};

use super::{
    get_mut,
    dfs_inorder_next, 
    dfs_inorder_streaming_iterator_impl,
    TraversalStatus
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
    pub fn leaves(self) -> MutBorrowedBinaryLeavesIterator<'a, Node, core::option::IntoIter<&'a mut Node>> {
        let mut traversal_stack_bottom = Vec::with_capacity(self.right_stack.capacity());
        for opt in self.right_stack {
            traversal_stack_bottom.push(opt.into_iter());
        }

        MutBorrowedBinaryLeavesIterator {
            root: None,
            traversal_stack_bottom: traversal_stack_bottom,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    /// WARNING: The slice returned by this iterator points to internal iterator
    /// state. Any changes to the slice's structure/order made using the StreamingIteratorMut 
    /// API will be carried through the remaining iterations. This can result in
    /// unexpected behaviors if you are not careful.
    /// 
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
    /// let mut dfs_iter = root.dfs_inorder_iter_mut().attach_ancestors();
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
    
    dfs_inorder_next!(get_value_and_children_binary_iter_mut);
}

pub struct MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    right_stack: Vec<Option<&'a mut Node>>,
    item_stack: Vec<Node::MutBorrowedValue>,
    status_stack: Vec<TraversalStatus>,
}

impl<'a, Node> MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        MutBorrowedDFSInorderIteratorWithAncestors {
            right_stack,
            item_stack: Vec::new(),
            status_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedDFSInorderIteratorWithAncestors<'a, Node>
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    type Item = [Node::MutBorrowedValue];
    
    dfs_inorder_streaming_iterator_impl!(get_value_and_children_binary_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    get_mut!();
}