use std::collections::VecDeque;
use crate::prelude::OwnedTreeNode;
use super::bfs_next;

pub struct OwnedBFSIterator<Node> 
    where Node: OwnedTreeNode {

    root: Option<Node>,
    traversal_queue: VecDeque<Node::OwnedChildren>
}

impl<Node> OwnedBFSIterator<Node>
    where Node: OwnedTreeNode {

    pub (crate) fn new(root: Node) -> OwnedBFSIterator<Node> {
        OwnedBFSIterator { 
            root: Some(root), 
            traversal_queue: VecDeque::new() 
        }
    }
}

impl<Node> Iterator for OwnedBFSIterator<Node> 
    where Node: OwnedTreeNode {

    type Item = Node::OwnedValue;
    bfs_next!(get_value_and_children);
}