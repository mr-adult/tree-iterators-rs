use alloc::{
    collections::VecDeque, 
    vec::Vec
};
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
    leaves_iterators::breadth_first::owned::{
        OwnedBinaryLeavesIterator, 
        OwnedLeavesIterator
    }
};
use super::{
    bfs_next, 
    bfs_advance_iterator, 
    bfs_streaming_iterator_impl,
    TreeNodeVecDeque, get_mut,
};

pub struct OwnedBFSIterator<Node> 
    where Node: OwnedTreeNode {

    root: Option<Node>,
    traversal_queue: VecDeque<Node::OwnedChildren>
}

impl<Node> OwnedBFSIterator<Node>
    where Node: OwnedTreeNode {

    pub (crate) fn new(root: Node) -> OwnedBFSIterator<Node> {
        OwnedBFSIterator { 
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
    pub fn leaves(self) -> OwnedLeavesIterator<Node, Node::OwnedChildren> {
        OwnedLeavesIterator { 
            root: self.root, 
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
        }
    }

    /// WARNING: The slice returned by this iterator points to internal iterator
    /// state. Any changes to the slice's structure/order made using the StreamingIteratorMut 
    /// API will be carried through the remaining iterations. This can result in
    /// unexpected behaviors if you are not careful.
    /// 
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
    /// let mut bfs_iter = root.bfs().attach_ancestors();
    /// while let Some(slice) = bfs_iter.next_mut() {
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
    pub fn attach_ancestors(self) -> OwnedBFSIteratorWithAncestors<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => OwnedBFSIteratorWithAncestors::new(root)
        }
    }
}

impl<Node> Iterator for OwnedBFSIterator<Node> 
    where Node: OwnedTreeNode {

    type Item = Node::OwnedValue;
    bfs_next!(get_value_and_children);
}

pub struct OwnedBFSIteratorWithAncestors<Node> 
    where Node: OwnedTreeNode {
    
    is_root: bool,
    item_stack: Vec<Node::OwnedValue>,
    tree_cache: TreeNodeVecDeque<Node::OwnedValue>,
    traversal_stack: Vec<TreeNodeVecDeque<Node::OwnedValue>>,
    iterator_queue: VecDeque<Option<Node::OwnedChildren>>,
}

impl<'a, Node> OwnedBFSIteratorWithAncestors<Node> 
    where Node: OwnedTreeNode {

    fn new(root: Node) -> OwnedBFSIteratorWithAncestors<Node> {
        let (value, children) = root.get_value_and_children();
        let tree_cache = TreeNodeVecDeque {
            value: None,
            children: None,
        };
        let mut iterator_queue = VecDeque::new();
        let mut item_stack = Vec::new();

        item_stack.push(value);
        iterator_queue.push_back(children);

        OwnedBFSIteratorWithAncestors {
            is_root: true,
            item_stack,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
        }
    }

    bfs_advance_iterator!(get_value_and_children);
}

impl<'a, Node> StreamingIterator for OwnedBFSIteratorWithAncestors<Node> 
    where Node: OwnedTreeNode {

    type Item = [Node::OwnedValue];

    bfs_streaming_iterator_impl!(get_value_and_children);
}

impl<'a, Node> StreamingIteratorMut for OwnedBFSIteratorWithAncestors<Node> 
    where Node: OwnedTreeNode {

    get_mut!();
}

pub struct OwnedBinaryBFSIterator<Node> 
    where Node: OwnedBinaryTreeNode {

    root: Option<Node>,
    traversal_queue: VecDeque<BinaryChildren<Node>>
}

impl<Node> OwnedBinaryBFSIterator<Node>
    where Node: OwnedBinaryTreeNode {

    pub (crate) fn new(root: Node) -> OwnedBinaryBFSIterator<Node> {
        OwnedBinaryBFSIterator { 
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
    pub fn leaves(self) -> OwnedBinaryLeavesIterator<Node, BinaryChildren<Node>> {
        OwnedBinaryLeavesIterator { 
            root: self.root, 
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
        }
    }

    /// WARNING: The slice returned by this iterator points to internal iterator
    /// state. Any changes to the slice's structure/order made using the StreamingIteratorMut 
    /// API will be carried through the remaining iterations. This can result in
    /// unexpected behaviors if you are not careful.
    /// 
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
    /// let mut bfs_iter = root.bfs().attach_ancestors();
    /// while let Some(slice) = bfs_iter.next_mut() {
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
    pub fn attach_ancestors(self) -> OwnedBinaryBFSIteratorWithAncestors<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => OwnedBinaryBFSIteratorWithAncestors::new(root)
        }
    }
}

impl<Node> Iterator for OwnedBinaryBFSIterator<Node> 
    where Node: OwnedBinaryTreeNode {

    type Item = Node::OwnedValue;
    bfs_next!(get_value_and_children);
}

pub struct OwnedBinaryBFSIteratorWithAncestors<Node> 
    where Node: OwnedBinaryTreeNode {
    
    is_root: bool,
    item_stack: Vec<Node::OwnedValue>,
    tree_cache: TreeNodeVecDeque<Node::OwnedValue>,
    traversal_stack: Vec<TreeNodeVecDeque<Node::OwnedValue>>,
    iterator_queue: VecDeque<Option<BinaryChildren<Node>>>,
}

impl<'a, Node> OwnedBinaryBFSIteratorWithAncestors<Node> 
    where Node: OwnedBinaryTreeNode {

    fn new(root: Node) -> OwnedBinaryBFSIteratorWithAncestors<Node> {
        let (value, children) = root.get_value_and_children();
        let tree_cache = TreeNodeVecDeque {
            value: None,
            children: None,
        };
        let mut iterator_queue = VecDeque::new();
        let mut item_stack = Vec::new();

        item_stack.push(value);
        iterator_queue.push_back(children);

        OwnedBinaryBFSIteratorWithAncestors {
            is_root: true,
            item_stack,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
        }
    }

    bfs_advance_iterator!(get_value_and_children);
}

impl<'a, Node> StreamingIterator for OwnedBinaryBFSIteratorWithAncestors<Node> 
    where Node: OwnedBinaryTreeNode {

    type Item = [Node::OwnedValue];

    bfs_streaming_iterator_impl!(get_value_and_children);
}

impl<'a, Node> StreamingIteratorMut for OwnedBinaryBFSIteratorWithAncestors<Node> 
    where Node: OwnedBinaryTreeNode {
        
    get_mut!();
}