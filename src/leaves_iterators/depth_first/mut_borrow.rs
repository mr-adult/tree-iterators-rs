use crate::prelude::{BinaryChildren, MutBorrowedBinaryTreeNode, MutBorrowedTreeNode};
use alloc::vec::Vec;

use super::dfs_postorder_leaves_next;

pub struct MutBorrowedLeavesIterator<'a, Node, Iter>
where
    Node: MutBorrowedTreeNode<'a>,
    Iter: Iterator<Item = &'a mut Node>,
{
    pub(crate) root: Option<&'a mut Node>,
    pub(crate) traversal_stack_bottom: Vec<Iter>,
    pub(crate) traversal_stack_top: Vec<<Node::MutBorrowedChildren as IntoIterator>::IntoIter>,
    pub(crate) item_stack: Vec<Node::MutBorrowedValue>,
}

impl<'a, Node, Iter> Iterator for MutBorrowedLeavesIterator<'a, Node, Iter>
where
    Node: MutBorrowedTreeNode<'a>,
    Iter: Iterator<Item = &'a mut Node>,
{
    type Item = Node::MutBorrowedValue;

    dfs_postorder_leaves_next!(get_value_and_children_iter_mut);
}

pub struct MutBorrowedBinaryLeavesIterator<'a, Node, Iter>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
    Iter: Iterator<Item = &'a mut Node>,
{
    pub(crate) root: Option<&'a mut Node>,
    pub(crate) traversal_stack_bottom: Vec<Iter>,
    pub(crate) traversal_stack_top: Vec<BinaryChildren<&'a mut Node>>,
    pub(crate) item_stack: Vec<Node::MutBorrowedValue>,
}

impl<'a, Node, Iter> Iterator for MutBorrowedBinaryLeavesIterator<'a, Node, Iter>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
    Iter: Iterator<Item = &'a mut Node>,
{
    type Item = Node::MutBorrowedValue;

    dfs_postorder_leaves_next!(get_value_and_children_iter_mut);
}
