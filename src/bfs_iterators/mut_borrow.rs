use std::collections::VecDeque;
use std::ops::ControlFlow;
use streaming_iterator::StreamingIterator;

use crate::prelude::{MutBorrowedTreeNode, BinaryChildren, MutBorrowedBinaryTreeNode};
use super::{
    bfs_next, 
    bfs_advance_iterator, 
    bfs_streaming_iterator_impl
};

pub struct MutBorrowedBFSIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    root: Option<&'a mut Node>,
    traversal_queue: VecDeque<Node::MutBorrowedChildren>
}

impl<'a, Node> MutBorrowedBFSIterator<'a, Node>
    where Node: MutBorrowedTreeNode<'a> {

    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedBFSIterator<'a, Node> {
        MutBorrowedBFSIterator { 
            root: Some(root), 
            traversal_queue: VecDeque::new() 
        }
    }

    /// This method retrieves a streaming iterator that can be used to perform
    /// Breadth First searches of a tree. This converts the queue-based
    /// iterator into an iterative deepening iterator.
    /// 
    /// A Breadth First Search (BFS) is defined as:
    /// 
    /// A tree traversal that involves breadth-first searching a tree 
    /// from the top down. Given a tree of the following shape, this 
    /// traversal type would yield slices in the following order:
    /// \[0\], 
    /// \[0, 1\], 
    /// \[0, 2\], 
    /// \[0, 1, 3\], 
    /// \[0, 1, 4\], 
    /// \[0, 2, 5\], 
    /// \[0, 2, 5, 6\], 
    /// \[0, 2, 5, 6, 7\], 
    /// \[0, 2, 5, 6, 7, 8\], 
    /// \[0, 2, 5, 6, 7, 8, 9\], 
    /// \[0, 2, 5, 6, 7, 8, 9, 10\], 
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
    pub fn attach_ancestors(self) -> MutBorrowedBFSIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => MutBorrowedBFSIteratorWithAncestors::new(root)
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedBFSIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    type Item = Node::MutBorrowedValue;
    bfs_next!(get_value_and_children_iter_mut);
}

pub struct MutBorrowedBFSIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {
    
    current_depth: usize,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_queue_stack: Vec<VecDeque<Option<Node::MutBorrowedValue>>>,
    iterator_queue: VecDeque<Option<Option<Node::MutBorrowedChildren>>>,
    is_in_middle_of_iterator: bool,
}

impl<'a, Node> MutBorrowedBFSIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    fn new(root: &'a mut Node) -> MutBorrowedBFSIteratorWithAncestors<'a, Node> {
        let mut traversal_queue = VecDeque::new();
        let mut traversal_queue_stack = Vec::new();
        let mut iterator_queue = VecDeque::new();

        let (value, children) = root.get_value_and_children_iter_mut();

        let current_depth = 1;
        traversal_queue.push_back(Some(value));
        traversal_queue_stack.push(traversal_queue);
        iterator_queue.push_back(Some(children));

        MutBorrowedBFSIteratorWithAncestors {
            current_depth: current_depth,
            item_stack: Vec::new(),
            traversal_queue_stack: traversal_queue_stack,
            iterator_queue: iterator_queue,
            is_in_middle_of_iterator: false,
        }
    }

    bfs_advance_iterator!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIterator for MutBorrowedBFSIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    type Item = [Node::MutBorrowedValue];

    bfs_streaming_iterator_impl!();
}

pub struct MutBorrowedBinaryBFSIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    root: Option<&'a mut Node>,
    traversal_queue: VecDeque<BinaryChildren<&'a mut Node>>
}

impl<'a, Node> MutBorrowedBinaryBFSIterator<'a, Node>
    where Node: MutBorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedBinaryBFSIterator<'a, Node> {
        MutBorrowedBinaryBFSIterator { 
            root: Some(root), 
            traversal_queue: VecDeque::new() 
        }
    }

    /// This method retrieves a streaming iterator that can be used to perform
    /// Breadth First searches of a tree. This converts the queue-based
    /// iterator into an iterative deepening iterator.
    /// 
    /// A Breadth First Search (BFS) is defined as:
    /// 
    /// A tree traversal that involves breadth-first searching a tree 
    /// from the top down. Given a tree of the following shape, this 
    /// traversal type would yield slices in the following order:
    /// \[0\], 
    /// \[0, 1\], 
    /// \[0, 2\], 
    /// \[0, 1, 3\], 
    /// \[0, 1, 4\], 
    /// \[0, 2, 5\], 
    /// \[0, 2, 5, 6\], 
    /// \[0, 2, 5, 6, 7\], 
    /// \[0, 2, 5, 6, 7, 8\], 
    /// \[0, 2, 5, 6, 7, 8, 9\], 
    /// \[0, 2, 5, 6, 7, 8, 9, 10\], 
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
    pub fn attach_ancestors(self) -> MutBorrowedBinaryBFSIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => MutBorrowedBinaryBFSIteratorWithAncestors::new(root)
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedBinaryBFSIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    type Item = Node::MutBorrowedValue;
    bfs_next!(get_value_and_children_iter_mut);
}

pub struct MutBorrowedBinaryBFSIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    current_depth: usize,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_queue_stack: Vec<VecDeque<Option<Node::MutBorrowedValue>>>,
    iterator_queue: VecDeque<Option<Option<BinaryChildren<&'a mut Node>>>>,
    is_in_middle_of_iterator: bool,
}

impl<'a, Node> MutBorrowedBinaryBFSIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    fn new(root: &'a mut Node) -> MutBorrowedBinaryBFSIteratorWithAncestors<'a, Node> {
        let mut traversal_queue = VecDeque::new();
        let mut traversal_queue_stack = Vec::new();
        let mut iterator_queue = VecDeque::new();

        let (value, children) = root.get_value_and_children_iter_mut();

        let current_depth = 1;
        traversal_queue.push_back(Some(value));
        traversal_queue_stack.push(traversal_queue);
        iterator_queue.push_back(Some(children));

        MutBorrowedBinaryBFSIteratorWithAncestors {
            current_depth: current_depth,
            item_stack: Vec::new(),
            traversal_queue_stack: traversal_queue_stack,
            iterator_queue: iterator_queue,
            is_in_middle_of_iterator: false,
        }
    }

    bfs_advance_iterator!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIterator for MutBorrowedBinaryBFSIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    type Item = [Node::MutBorrowedValue];

    bfs_streaming_iterator_impl!();
}