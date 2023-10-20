use std::collections::VecDeque;

use crate::prelude::{
    BorrowedTreeNode, 
    BinaryChildren, 
    BorrowedBinaryTreeNode
};

use crate::make_peekable_iterator::MakePeekableIterator;

use super::{
    bfs_next, 
    next
};

pub struct BorrowedLeavesIterator<'a, Node>
    where Node: BorrowedTreeNode<'a> {

    pub (crate) root: Option<&'a Node>,
    pub (crate) old_traversal_queue: VecDeque<Node::BorrowedChildren>,
    pub (crate) new_traversal_queue: VecDeque<MakePeekableIterator<Node::BorrowedChildren>>,
}

impl<'a, Node> BorrowedLeavesIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a>, {

    bfs_next!(get_value_and_children_iter, Node::BorrowedValue);
}

impl<'a, Node> Iterator for BorrowedLeavesIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
    
    type Item = Node::BorrowedValue;
    next!();
}

pub struct BorrowedBinaryLeavesIterator<'a, Node>
    where Node: BorrowedBinaryTreeNode<'a> {

    pub (crate) root: Option<&'a Node>,
    pub (crate) old_traversal_queue: VecDeque<BinaryChildren<&'a Node>>,
    pub (crate) new_traversal_queue: VecDeque<MakePeekableIterator<BinaryChildren<&'a Node>>>,
}

impl<'a, Node> BorrowedBinaryLeavesIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {

    bfs_next!(get_value_and_children_iter, Node::BorrowedValue);
}

impl<'a, Node> Iterator for BorrowedBinaryLeavesIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {
    
    type Item = Node::BorrowedValue;
    next!();
}