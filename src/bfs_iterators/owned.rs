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
    prelude::{BinaryChildren, OwnedBinaryTreeNode, OwnedTreeNode},
};

pub struct OwnedBFSIterator<Node>
where
    Node: OwnedTreeNode,
{
    root: Option<Node>,
    traversal_queue: VecDeque<<Node::OwnedChildren as IntoIterator>::IntoIter>,
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

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(
        self,
    ) -> OwnedLeavesIterator<Node, <Node::OwnedChildren as IntoIterator>::IntoIter> {
        OwnedLeavesIterator {
            root: self.root,
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> OwnedBFSIteratorWithAncestors<Node> {
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
    pub(crate) iterator_queue: VecDeque<<Node::OwnedChildren as IntoIterator>::IntoIter>,
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
        iterator_queue.push_back(children.into_iter());

        OwnedBFSIteratorWithAncestors {
            is_root: true,
            item_stack,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(self) -> OwnedBFSLeavesIteratorWithAncestors<Node> {
        OwnedBFSLeavesIteratorWithAncestors::new(self)
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

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(self) -> OwnedBinaryLeavesIterator<Node, BinaryChildren<Node>> {
        OwnedBinaryLeavesIterator {
            root: self.root,
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> OwnedBinaryBFSIteratorWithAncestors<Node> {
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
    pub(crate) iterator_queue: VecDeque<BinaryChildren<Node>>,
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

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(self) -> OwnedBinaryBFSLeavesIteratorWithAncestors<Node> {
        OwnedBinaryBFSLeavesIteratorWithAncestors::new(self)
    }

    bfs_advance_iterator!(get_value_and_children);
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
