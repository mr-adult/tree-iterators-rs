use std::collections::VecDeque;

use crate::prelude::{
    OwnedTreeNode, 
    BinaryChildren, 
    OwnedBinaryTreeNode
};

use crate::make_peekable_iterator::MakePeekableIterator;

use super::{
    bfs_next,
    next
};

pub struct OwnedLeavesIterator<Node, Iter>
    where Node: OwnedTreeNode,
        Iter: Iterator<Item = Node> {

    pub (crate) root: Option<Node>,
    pub (crate) old_traversal_queue: VecDeque<Iter>,
    pub (crate) new_traversal_queue: VecDeque<MakePeekableIterator<Node::OwnedChildren>>,
}

impl<'a, Node, Iter> OwnedLeavesIterator<Node, Iter> 
    where Node: OwnedTreeNode,
        Iter: Iterator<Item = Node> {

    bfs_next!(get_value_and_children, Node::OwnedValue);
}

impl<'a, Node, Iter> Iterator for OwnedLeavesIterator<Node, Iter> 
    where Node: OwnedTreeNode,
        Iter: Iterator<Item = Node> {
    
    type Item = Node::OwnedValue;
    next!();
}

pub struct OwnedBinaryLeavesIterator<Node, Iter>
    where Node: OwnedBinaryTreeNode,
        Iter: Iterator<Item = Node> {

    pub (crate) root: Option<Node>,
    pub (crate) old_traversal_queue: VecDeque<Iter>,
    pub (crate) new_traversal_queue: VecDeque<MakePeekableIterator<BinaryChildren<Node>>>,
}

impl<'a, Node, Iter> OwnedBinaryLeavesIterator<Node, Iter> 
    where Node: OwnedBinaryTreeNode,
        Iter: Iterator<Item = Node> {

    bfs_next!(get_value_and_children, Node::OwnedValue);
}

impl<'a, Node, Iter> Iterator for OwnedBinaryLeavesIterator<Node, Iter> 
    where Node: OwnedBinaryTreeNode,
        Iter: Iterator<Item = Node> {
    
    type Item = Node::OwnedValue;
    next!();
}