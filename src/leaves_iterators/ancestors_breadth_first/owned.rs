use alloc::{collections::VecDeque, vec::Vec};
use core::iter::Peekable;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use crate::{
    bfs_iterators::owned::{OwnedBFSIteratorWithAncestors, OwnedBinaryBFSIteratorWithAncestors},
    prelude::{BinaryChildren, OwnedBinaryTreeNode, OwnedTreeNode},
};

use super::{bfs_next, get_mut, streaming_leaves};

use crate::bfs_iterators::TreeNodeVecDeque;

pub struct OwnedBFSLeavesIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    is_root: bool,
    item_stack: Vec<Node::OwnedValue>,
    tree_cache: TreeNodeVecDeque<Node::OwnedValue>,
    traversal_stack: Vec<TreeNodeVecDeque<Node::OwnedValue>>,
    iterator_queue: VecDeque<Peekable<Node::OwnedChildren>>,
}

impl<Node> OwnedBFSLeavesIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    pub(crate) fn new(
        source: OwnedBFSIteratorWithAncestors<Node>,
    ) -> OwnedBFSLeavesIteratorWithAncestors<Node> {
        OwnedBFSLeavesIteratorWithAncestors {
            is_root: source.is_root,
            item_stack: source.item_stack,
            iterator_queue: source
                .iterator_queue
                .into_iter()
                .map(|val| val.peekable())
                .collect(),
            traversal_stack: source.traversal_stack,
            tree_cache: source.tree_cache,
        }
    }

    bfs_next!(get_value_and_children);
}

impl<Node> StreamingIterator for OwnedBFSLeavesIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    type Item = [Node::OwnedValue];

    streaming_leaves!();
}

impl<Node> StreamingIteratorMut for OwnedBFSLeavesIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    get_mut!();
}

pub struct OwnedBinaryBFSLeavesIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    is_root: bool,
    item_stack: Vec<Node::OwnedValue>,
    tree_cache: TreeNodeVecDeque<Node::OwnedValue>,
    traversal_stack: Vec<TreeNodeVecDeque<Node::OwnedValue>>,
    iterator_queue: VecDeque<Peekable<BinaryChildren<Node>>>,
}

impl<Node> OwnedBinaryBFSLeavesIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) fn new(
        source: OwnedBinaryBFSIteratorWithAncestors<Node>,
    ) -> OwnedBinaryBFSLeavesIteratorWithAncestors<Node> {
        OwnedBinaryBFSLeavesIteratorWithAncestors {
            is_root: source.is_root,
            item_stack: source.item_stack,
            iterator_queue: source
                .iterator_queue
                .into_iter()
                .map(|val| val.peekable())
                .collect(),
            traversal_stack: source.traversal_stack,
            tree_cache: source.tree_cache,
        }
    }

    bfs_next!(get_value_and_children);
}

impl<Node> StreamingIterator for OwnedBinaryBFSLeavesIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = [Node::OwnedValue];

    streaming_leaves!();
}

impl<Node> StreamingIteratorMut for OwnedBinaryBFSLeavesIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    get_mut!();
}
