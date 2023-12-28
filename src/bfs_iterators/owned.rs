use alloc::{collections::VecDeque, vec::Vec};
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use super::{
    bfs_advance_iterator, bfs_next, bfs_streaming_iterator_impl, get_mut, TreeNodeVecDeque,
};
use crate::{
    leaves_iterators::{
        ancestors_breadth_first::owned::{
            OwnedBFSLeavesIteratorWithAncestors, OwnedBinaryBFSLeavesIteratorWithAncestors,
        },
        breadth_first::owned::{OwnedBinaryLeavesIterator, OwnedLeavesIterator},
    },
    prelude::{
        AncestorsIteratorMut, AncestorsLeavesIteratorMut, BinaryChildren, LeavesIterator,
        OwnedBinaryTreeNode, OwnedTreeNode, TreeIteratorMut,
    },
};

pub struct OwnedBFSIterator<Node>
where
    Node: OwnedTreeNode,
{
    root: Option<Node>,
    traversal_queue: VecDeque<Node::OwnedChildren>,
}

impl<Node> OwnedBFSIterator<Node>
where
    Node: OwnedTreeNode,
{
    pub(crate) fn new(root: Node) -> OwnedBFSIterator<Node> {
        OwnedBFSIterator {
            root: Some(root),
            traversal_queue: VecDeque::new(),
        }
    }
}

impl<Node> TreeIteratorMut for OwnedBFSIterator<Node>
where
    Node: OwnedTreeNode,
{
    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        OwnedLeavesIterator {
            root: self.root,
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
        }
    }

    fn attach_ancestors(self) -> impl AncestorsIteratorMut<Item = [Node::OwnedValue]> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => OwnedBFSIteratorWithAncestors::new(root)
        }
    }
}

impl<Node> Iterator for OwnedBFSIterator<Node>
where
    Node: OwnedTreeNode,
{
    type Item = Node::OwnedValue;
    bfs_next!(get_value_and_children);
}

pub struct OwnedBFSIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    pub(crate) is_root: bool,
    pub(crate) item_stack: Vec<Node::OwnedValue>,
    pub(crate) tree_cache: TreeNodeVecDeque<Node::OwnedValue>,
    pub(crate) traversal_stack: Vec<TreeNodeVecDeque<Node::OwnedValue>>,
    pub(crate) iterator_queue: VecDeque<Option<Node::OwnedChildren>>,
}

impl<'a, Node> OwnedBFSIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
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
where
    Node: OwnedTreeNode,
{
    type Item = [Node::OwnedValue];

    bfs_streaming_iterator_impl!(get_value_and_children);
}

impl<'a, Node> StreamingIteratorMut for OwnedBFSIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    get_mut!();
}

impl<Node> AncestorsIteratorMut for OwnedBFSIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    fn leaves(self) -> impl AncestorsLeavesIteratorMut<Item = Self::Item> {
        OwnedBFSLeavesIteratorWithAncestors::new(self)
    }
}

pub struct OwnedBinaryBFSIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    root: Option<Node>,
    traversal_queue: VecDeque<BinaryChildren<Node>>,
}

impl<Node> OwnedBinaryBFSIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) fn new(root: Node) -> OwnedBinaryBFSIterator<Node> {
        OwnedBinaryBFSIterator {
            root: Some(root),
            traversal_queue: VecDeque::new(),
        }
    }
}

impl<Node> TreeIteratorMut for OwnedBinaryBFSIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        OwnedBinaryLeavesIterator {
            root: self.root,
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
        }
    }

    fn attach_ancestors(self) -> impl AncestorsIteratorMut<Item = [Node::OwnedValue]> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => OwnedBinaryBFSIteratorWithAncestors::new(root)
        }
    }
}

impl<Node> Iterator for OwnedBinaryBFSIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = Node::OwnedValue;
    bfs_next!(get_value_and_children);
}

pub struct OwnedBinaryBFSIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) is_root: bool,
    pub(crate) item_stack: Vec<Node::OwnedValue>,
    pub(crate) tree_cache: TreeNodeVecDeque<Node::OwnedValue>,
    pub(crate) traversal_stack: Vec<TreeNodeVecDeque<Node::OwnedValue>>,
    pub(crate) iterator_queue: VecDeque<Option<BinaryChildren<Node>>>,
}

impl<'a, Node> OwnedBinaryBFSIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
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

impl<Node> AncestorsIteratorMut for OwnedBinaryBFSIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    fn leaves(self) -> impl AncestorsLeavesIteratorMut<Item = Self::Item> {
        OwnedBinaryBFSLeavesIteratorWithAncestors::new(self)
    }
}

impl<'a, Node> StreamingIterator for OwnedBinaryBFSIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = [Node::OwnedValue];

    bfs_streaming_iterator_impl!(get_value_and_children);
}

impl<'a, Node> StreamingIteratorMut for OwnedBinaryBFSIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    get_mut!();
}

impl<Node> AncestorsLeavesIteratorMut for OwnedBinaryBFSIteratorWithAncestors<Node> where
    Node: OwnedBinaryTreeNode
{
}
