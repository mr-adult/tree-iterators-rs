use std::collections::VecDeque;
use crate::prelude::MutBorrowedTreeNode;
use crate::bfs_iterators::bfs_next;

pub struct MutBorrowedBFSIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    root: Option<&'a mut Node>,
    traversal_queue: VecDeque<Node::MutBorrowedChildren>
}

impl<'a, Node> MutBorrowedBFSIterator<'a, Node>
    where Node: MutBorrowedTreeNode<'a> {

    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedBFSIterator<'a, Node> {
        MutBorrowedBFSIterator { 
            root: Some(root), 
            traversal_queue: VecDeque::new() 
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedBFSIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    type Item = Node::MutBorrowedValue;
    bfs_next!(get_value_and_children_iter_mut);
}