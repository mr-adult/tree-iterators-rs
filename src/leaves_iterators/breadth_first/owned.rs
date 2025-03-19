use alloc::collections::VecDeque;

use crate::prelude::{BinaryChildren, OwnedBinaryTreeNode, OwnedTreeNode};

use core::iter::Peekable;

use super::{bfs_next, next};

pub struct OwnedLeavesIterator<Node, Iter>
where
    Node: OwnedTreeNode,
    Iter: Iterator<Item = Node>,
{
    pub(crate) root: Option<Node>,
    pub(crate) old_traversal_queue: VecDeque<Iter>,
    pub(crate) new_traversal_queue: VecDeque<Peekable<Node::OwnedChildren>>,
}

impl<'a, Node, Iter> OwnedLeavesIterator<Node, Iter>
where
    Node: OwnedTreeNode,
    Iter: Iterator<Item = Node>,
{
    bfs_next!(get_value_and_children, Node::OwnedValue);
}

impl<Node, Iter> Iterator for OwnedLeavesIterator<Node, Iter>
where
    Node: OwnedTreeNode,
    Iter: Iterator<Item = Node>,
{
    type Item = Node::OwnedValue;
    next!();
}

pub struct OwnedBinaryLeavesIterator<Node, Iter>
where
    Node: OwnedBinaryTreeNode,
    Iter: Iterator<Item = Node>,
{
    pub(crate) root: Option<Node>,
    pub(crate) old_traversal_queue: VecDeque<Iter>,
    pub(crate) new_traversal_queue: VecDeque<Peekable<BinaryChildren<Node>>>,
}

impl<Node, Iter> OwnedBinaryLeavesIterator<Node, Iter>
where
    Node: OwnedBinaryTreeNode,
    Iter: Iterator<Item = Node>,
{
    bfs_next!(get_value_and_children, Node::OwnedValue);
}

impl<'a, Node, Iter> Iterator for OwnedBinaryLeavesIterator<Node, Iter>
where
    Node: OwnedBinaryTreeNode,
    Iter: Iterator<Item = Node>,
{
    type Item = Node::OwnedValue;
    next!();
}
