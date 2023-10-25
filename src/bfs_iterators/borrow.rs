use alloc::{
    collections::VecDeque, 
    vec::Vec
};
use streaming_iterator::StreamingIterator;

use crate::{
    prelude::{
        BorrowedTreeNode, 
        BinaryChildren, 
        BorrowedBinaryTreeNode
    }, 
    leaves_iterators::breadth_first::borrow::{
        BorrowedLeavesIterator, 
        BorrowedBinaryLeavesIterator
    }
};

use super::{
    bfs_next, 
    bfs_advance_iterator, 
    bfs_streaming_iterator_impl,
    TreeNodeVecDeque,
};

pub struct BorrowedBFSIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    root: Option<&'a Node>,
    traversal_queue: VecDeque<Node::BorrowedChildren>
}

impl<'a, Node> BorrowedBFSIterator<'a, Node>
    where Node: BorrowedTreeNode<'a> {

    pub (crate) fn new(root: &'a Node) -> BorrowedBFSIterator<'a, Node> {
        BorrowedBFSIterator { 
            root: Some(root), 
            traversal_queue: VecDeque::new() 
        }
    }

    /// This method converts the current Breadth First Search iterator into 
    /// an iterator that will yield only the leaves of the tree. Iteration
    /// still proceeds in a Breadth First Search order.
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
    pub fn leaves(self) -> BorrowedLeavesIterator<'a, Node> {
        BorrowedLeavesIterator {
            root: self.root, 
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
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
    pub fn attach_ancestors(self) -> BorrowedBFSIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => BorrowedBFSIteratorWithAncestors::new(root)
        }
    }
}

impl<'a, Node> Iterator for BorrowedBFSIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    type Item = Node::BorrowedValue;
    bfs_next!(get_value_and_children_iter);
}

pub struct BorrowedBFSIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
    
    is_root: bool,
    item_stack: Vec<Node::BorrowedValue>,
    tree_cache: TreeNodeVecDeque<Node::BorrowedValue>,
    traversal_stack: Vec<TreeNodeVecDeque<Node::BorrowedValue>>,
    iterator_queue: VecDeque<Option<Node::BorrowedChildren>>,
}

impl<'a, Node> BorrowedBFSIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    fn new(root: &'a Node) -> BorrowedBFSIteratorWithAncestors<'a, Node> {
        let (value, children) = root.get_value_and_children_iter();
        let tree_cache = TreeNodeVecDeque {
            value: None,
            children: None,
        };
        let mut iterator_queue = VecDeque::new();
        let mut item_stack = Vec::new();

        item_stack.push(value);
        iterator_queue.push_back(children);

        BorrowedBFSIteratorWithAncestors {
            is_root: true,
            item_stack,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
        }
    }

    bfs_advance_iterator!(get_value_and_children_iter);
}

impl<'a, Node> StreamingIterator for BorrowedBFSIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    type Item = [Node::BorrowedValue];

    bfs_streaming_iterator_impl!(get_value_and_children_iter);
}

pub struct BorrowedBinaryBFSIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {

    root: Option<&'a Node>,
    traversal_queue: VecDeque<BinaryChildren<&'a Node>>
}

impl<'a, Node> BorrowedBinaryBFSIterator<'a, Node>
    where Node: BorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a Node) -> BorrowedBinaryBFSIterator<'a, Node> {
        BorrowedBinaryBFSIterator { 
            root: Some(root), 
            traversal_queue: VecDeque::new() 
        }
    }

    /// This method converts the current Breadth First Search iterator into 
    /// an iterator that will yield only the leaves of the tree. Iteration
    /// still proceeds in a Breadth First Search order.
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
    pub fn leaves(self) -> BorrowedBinaryLeavesIterator<'a, Node> {
        BorrowedBinaryLeavesIterator { 
            root: self.root, 
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
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
    pub fn attach_ancestors(self) -> BorrowedBinaryBFSIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => BorrowedBinaryBFSIteratorWithAncestors::new(root)
        }
    }
}

impl<'a, Node> Iterator for BorrowedBinaryBFSIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {

    type Item = Node::BorrowedValue;
    bfs_next!(get_value_and_children_iter);
}

pub struct BorrowedBinaryBFSIteratorWithAncestors<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {
    
    is_root: bool,
    item_stack: Vec<Node::BorrowedValue>,
    tree_cache: TreeNodeVecDeque<Node::BorrowedValue>,
    traversal_stack: Vec<TreeNodeVecDeque<Node::BorrowedValue>>,
    iterator_queue: VecDeque<Option<BinaryChildren<&'a Node>>>,
}

impl<'a, Node> BorrowedBinaryBFSIteratorWithAncestors<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {

    fn new(root: &'a Node) -> BorrowedBinaryBFSIteratorWithAncestors<'a, Node> {
        let (value, children) = root.get_value_and_children_iter();
        let tree_cache = TreeNodeVecDeque {
            value: None,
            children: None,
        };
        let mut iterator_queue = VecDeque::new();
        let mut item_stack = Vec::new();

        item_stack.push(value);
        iterator_queue.push_back(children);

        BorrowedBinaryBFSIteratorWithAncestors {
            is_root: true,
            item_stack,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
        }
    }

    bfs_advance_iterator!(get_value_and_children_iter);
}

impl<'a, Node> StreamingIterator for BorrowedBinaryBFSIteratorWithAncestors<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {

    type Item = [Node::BorrowedValue];

    bfs_streaming_iterator_impl!(get_value_and_children_iter);
}