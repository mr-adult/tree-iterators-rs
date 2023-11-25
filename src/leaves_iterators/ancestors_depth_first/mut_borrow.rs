use crate::prelude::{BinaryChildren, MutBorrowedBinaryTreeNode, MutBorrowedTreeNode};
use alloc::vec::Vec;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use super::{get_mut, streaming_leaves};

pub struct MutBorrowedDFSLeavesPostorderIteratorWithAncestors<'a, Node, Iter>
where
    Node: MutBorrowedTreeNode<'a>,
    Iter: Iterator<Item = &'a mut Node>,
{
    pub(crate) root: Option<&'a mut Node>,
    pub(crate) item_stack: Vec<Node::MutBorrowedValue>,
    pub(crate) old_traversal_stack: Vec<Iter>,
    pub(crate) new_traversal_stack: Vec<Node::MutBorrowedChildren>,
}

impl<'a, Node, Iter> StreamingIterator
    for MutBorrowedDFSLeavesPostorderIteratorWithAncestors<'a, Node, Iter>
where
    Node: MutBorrowedTreeNode<'a>,
    Iter: Iterator<Item = &'a mut Node>,
{
    type Item = [Node::MutBorrowedValue];
    streaming_leaves!(get_value_and_children_iter_mut);
}

impl<'a, Node, Iter> StreamingIteratorMut
    for MutBorrowedDFSLeavesPostorderIteratorWithAncestors<'a, Node, Iter>
where
    Node: MutBorrowedTreeNode<'a>,
    Iter: Iterator<Item = &'a mut Node>,
{
    get_mut!();
}

pub struct MutBorrowedBinaryDFSLeavesPostorderIteratorWithAncestors<'a, Node, Iter>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
    Iter: Iterator<Item = &'a mut Node>,
{
    pub(crate) root: Option<&'a mut Node>,
    pub(crate) item_stack: Vec<Node::MutBorrowedValue>,
    pub(crate) old_traversal_stack: Vec<Iter>,
    pub(crate) new_traversal_stack: Vec<BinaryChildren<&'a mut Node>>,
}

impl<'a, Node, Iter> StreamingIterator
    for MutBorrowedBinaryDFSLeavesPostorderIteratorWithAncestors<'a, Node, Iter>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
    Iter: Iterator<Item = &'a mut Node>,
{
    type Item = [Node::MutBorrowedValue];
    streaming_leaves!(get_value_and_children_iter_mut);
}

impl<'a, Node, Iter> StreamingIteratorMut
    for MutBorrowedBinaryDFSLeavesPostorderIteratorWithAncestors<'a, Node, Iter>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
    Iter: Iterator<Item = &'a mut Node>,
{
    get_mut!();
}
