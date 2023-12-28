use crate::prelude::{
    AncestorsLeavesIteratorMut, BinaryChildren, OwnedBinaryTreeNode, OwnedTreeNode,
};
use alloc::vec::Vec;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use super::{get_mut, streaming_leaves};

pub struct OwnedDFSLeavesPostorderIteratorWithAncestors<Node, Iter>
where
    Node: OwnedTreeNode,
    Iter: Iterator<Item = Node>,
{
    pub(crate) root: Option<Node>,
    pub(crate) item_stack: Vec<Node::OwnedValue>,
    pub(crate) old_traversal_stack: Vec<Iter>,
    pub(crate) new_traversal_stack: Vec<Node::OwnedChildren>,
}

impl<Node, Iter> StreamingIterator for OwnedDFSLeavesPostorderIteratorWithAncestors<Node, Iter>
where
    Node: OwnedTreeNode,
    Iter: Iterator<Item = Node>,
{
    type Item = [Node::OwnedValue];
    streaming_leaves!(get_value_and_children);
}

impl<Node, Iter> StreamingIteratorMut for OwnedDFSLeavesPostorderIteratorWithAncestors<Node, Iter>
where
    Node: OwnedTreeNode,
    Iter: Iterator<Item = Node>,
{
    get_mut!();
}

impl<Node, Iter> AncestorsLeavesIteratorMut
    for OwnedDFSLeavesPostorderIteratorWithAncestors<Node, Iter>
where
    Node: OwnedTreeNode,
    Iter: Iterator<Item = Node>,
{
}

pub struct OwnedBinaryDFSLeavesPostorderIteratorWithAncestors<Node, Iter>
where
    Node: OwnedBinaryTreeNode,
    Iter: Iterator<Item = Node>,
{
    pub(crate) root: Option<Node>,
    pub(crate) item_stack: Vec<Node::OwnedValue>,
    pub(crate) old_traversal_stack: Vec<Iter>,
    pub(crate) new_traversal_stack: Vec<BinaryChildren<Node>>,
}

impl<Node, Iter> StreamingIterator
    for OwnedBinaryDFSLeavesPostorderIteratorWithAncestors<Node, Iter>
where
    Node: OwnedBinaryTreeNode,
    Iter: Iterator<Item = Node>,
{
    type Item = [Node::OwnedValue];
    streaming_leaves!(get_value_and_children);
}

impl<Node, Iter> StreamingIteratorMut
    for OwnedBinaryDFSLeavesPostorderIteratorWithAncestors<Node, Iter>
where
    Node: OwnedBinaryTreeNode,
    Iter: Iterator<Item = Node>,
{
    get_mut!();
}

impl<Node, Iter> AncestorsLeavesIteratorMut
    for OwnedBinaryDFSLeavesPostorderIteratorWithAncestors<Node, Iter>
where
    Node: OwnedBinaryTreeNode,
    Iter: Iterator<Item = Node>,
{
}
