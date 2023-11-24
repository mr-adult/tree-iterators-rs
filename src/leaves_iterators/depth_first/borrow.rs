use crate::prelude::{BinaryChildren, BorrowedBinaryTreeNode, BorrowedTreeNode, LeavesIterator};
use alloc::vec::Vec;

use super::dfs_postorder_leaves_next;

pub struct BorrowedLeavesIterator<'a, Node, Iter>
where
    Node: BorrowedTreeNode<'a>,
    Iter: Iterator<Item = &'a Node>,
{
    pub(crate) root: Option<&'a Node>,
    pub(crate) item_stack: Vec<Node::BorrowedValue>,
    pub(crate) traversal_stack_bottom: Vec<Iter>,
    pub(crate) traversal_stack_top: Vec<Node::BorrowedChildren>,
}

impl<'a, Node, Iter> Iterator for BorrowedLeavesIterator<'a, Node, Iter>
where
    Node: BorrowedTreeNode<'a>,
    Iter: Iterator<Item = &'a Node>,
{
    type Item = Node::BorrowedValue;

    dfs_postorder_leaves_next!(get_value_and_children_iter);
}

impl<'a, Node, Iter> LeavesIterator for BorrowedLeavesIterator<'a, Node, Iter>
where
    Node: BorrowedTreeNode<'a>,
    Iter: Iterator<Item = &'a Node>,
{
}

pub struct BorrowedBinaryLeavesIterator<'a, Node, Iter>
where
    Node: BorrowedBinaryTreeNode<'a>,
    Iter: Iterator<Item = &'a Node>,
{
    pub(crate) root: Option<&'a Node>,
    pub(crate) item_stack: Vec<Node::BorrowedValue>,
    pub(crate) traversal_stack_bottom: Vec<Iter>,
    pub(crate) traversal_stack_top: Vec<BinaryChildren<&'a Node>>,
}

impl<'a, Node, Iter> LeavesIterator for BorrowedBinaryLeavesIterator<'a, Node, Iter>
where
    Node: BorrowedBinaryTreeNode<'a>,
    Iter: Iterator<Item = &'a Node>,
{
}

impl<'a, Node, Iter> Iterator for BorrowedBinaryLeavesIterator<'a, Node, Iter>
where
    Node: BorrowedBinaryTreeNode<'a>,
    Iter: Iterator<Item = &'a Node>,
{
    type Item = Node::BorrowedValue;

    dfs_postorder_leaves_next!(get_value_and_children_iter);
}
