use crate::prelude::{BinaryChildren, BorrowedBinaryTreeNode, BorrowedTreeNode};
use alloc::vec::Vec;
use streaming_iterator::StreamingIterator;

use super::streaming_leaves;

pub struct BorrowedDFSLeavesPostorderIteratorWithAncestors<'a, Node, Iter>
where
    Node: BorrowedTreeNode<'a>,
    Iter: Iterator<Item = &'a Node>,
{
    pub(crate) root: Option<&'a Node>,
    pub(crate) item_stack: Vec<Node::BorrowedValue>,
    pub(crate) old_traversal_stack: Vec<Iter>,
    pub(crate) new_traversal_stack: Vec<<Node::BorrowedChildren as IntoIterator>::IntoIter>,
}

impl<'a, Node, Iter> StreamingIterator
    for BorrowedDFSLeavesPostorderIteratorWithAncestors<'a, Node, Iter>
where
    Node: BorrowedTreeNode<'a>,
    Iter: Iterator<Item = &'a Node>,
{
    type Item = [Node::BorrowedValue];
    streaming_leaves!(get_value_and_children_iter);
}

pub struct BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors<'a, Node, Iter>
where
    Node: BorrowedBinaryTreeNode<'a>,
    Iter: Iterator<Item = &'a Node>,
{
    pub(crate) root: Option<&'a Node>,
    pub(crate) item_stack: Vec<Node::BorrowedValue>,
    pub(crate) old_traversal_stack: Vec<Iter>,
    pub(crate) new_traversal_stack: Vec<BinaryChildren<&'a Node>>,
}

impl<'a, Node, Iter> StreamingIterator
    for BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors<'a, Node, Iter>
where
    Node: BorrowedBinaryTreeNode<'a>,
    Iter: Iterator<Item = &'a Node>,
{
    type Item = [Node::BorrowedValue];
    streaming_leaves!(get_value_and_children_iter);
}
