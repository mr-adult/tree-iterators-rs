use alloc::collections::VecDeque;

use crate::prelude::{BinaryChildren, BorrowedBinaryTreeNode, BorrowedTreeNode, LeavesIterator};

use core::iter::Peekable;

use super::{bfs_next, next};

pub struct BorrowedLeavesIterator<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    pub(crate) root: Option<&'a Node>,
    pub(crate) old_traversal_queue: VecDeque<Node::BorrowedChildren>,
    pub(crate) new_traversal_queue: VecDeque<Peekable<Node::BorrowedChildren>>,
}

impl<'a, Node> LeavesIterator for BorrowedLeavesIterator<'a, Node> where Node: BorrowedTreeNode<'a> {}

impl<'a, Node> BorrowedLeavesIterator<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    bfs_next!(get_value_and_children_iter, Node::BorrowedValue);
}

impl<'a, Node> Iterator for BorrowedLeavesIterator<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    type Item = Node::BorrowedValue;
    next!();
}

pub struct BorrowedBinaryLeavesIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) root: Option<&'a Node>,
    pub(crate) old_traversal_queue: VecDeque<BinaryChildren<&'a Node>>,
    pub(crate) new_traversal_queue: VecDeque<Peekable<BinaryChildren<&'a Node>>>,
}

impl<'a, Node> BorrowedBinaryLeavesIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    bfs_next!(get_value_and_children_iter, Node::BorrowedValue);
}

impl<'a, Node> Iterator for BorrowedBinaryLeavesIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = Node::BorrowedValue;
    next!();
}

impl<'a, Node> LeavesIterator for BorrowedBinaryLeavesIterator<'a, Node> where
    Node: BorrowedBinaryTreeNode<'a>
{
}
