use std::collections::VecDeque;

use crate::prelude::{
    MutBorrowedTreeNode, 
    BinaryChildren, 
    MutBorrowedBinaryTreeNode
};

use crate::make_peekable_iterator::MakePeekableIterator;

use super::{
    bfs_next, 
    next
};

pub struct MutBorrowedLeavesIterator<'a, Node>
    where Node: MutBorrowedTreeNode<'a> {

    pub (crate) root: Option<&'a mut Node>,
    pub (crate) old_traversal_queue: VecDeque<Node::MutBorrowedChildren>,
    pub (crate) new_traversal_queue: VecDeque<MakePeekableIterator<Node::MutBorrowedChildren>>,
}

impl<'a, Node> MutBorrowedLeavesIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    bfs_next!(get_value_and_children_iter_mut, Node::MutBorrowedValue);
}

impl<'a, Node> Iterator for MutBorrowedLeavesIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {
    
    type Item = Node::MutBorrowedValue;
    next!();
}

pub struct MutBorrowedBinaryLeavesIterator<'a, Node, Iter>
    where Node: MutBorrowedBinaryTreeNode<'a>,
        Iter: Iterator<Item = &'a mut Node> {

    pub (crate) root: Option<&'a mut Node>,
    pub (crate) old_traversal_queue: VecDeque<Iter>,
    pub (crate) new_traversal_queue: VecDeque<MakePeekableIterator<BinaryChildren<&'a mut Node>>>,
}

impl<'a, Node, Iter> MutBorrowedBinaryLeavesIterator<'a, Node, Iter> 
    where Node: MutBorrowedBinaryTreeNode<'a>,
        Iter: Iterator<Item = &'a mut Node> {

    bfs_next!(get_value_and_children_iter_mut, Node::MutBorrowedValue);
}

impl<'a, Node, Iter> Iterator for MutBorrowedBinaryLeavesIterator<'a, Node, Iter> 
    where Node: MutBorrowedBinaryTreeNode<'a>,
        Iter: Iterator<Item = &'a mut Node> {
    
    type Item = Node::MutBorrowedValue;
    next!();
}