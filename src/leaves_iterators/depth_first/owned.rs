use alloc::vec::Vec;
use crate::prelude::{
    OwnedTreeNode, 
    BinaryChildren, 
    OwnedBinaryTreeNode, LeavesIterator
};

use super::dfs_postorder_leaves_next;

pub struct OwnedLeavesIterator<Node>
    where Node: OwnedTreeNode {

    pub (crate) root: Option<Node>,
    pub (crate) traversal_stack_bottom: Vec<Node::OwnedChildren>,
    pub (crate) traversal_stack_top: Vec<Node::OwnedChildren>,
    pub (crate) item_stack: Vec<Node::OwnedValue>,
}

impl<'a, Node> Iterator for OwnedLeavesIterator<Node> 
    where Node: OwnedTreeNode {
    
    type Item = Node::OwnedValue;
    
    dfs_postorder_leaves_next!(get_value_and_children);
}

impl<'a, Node> LeavesIterator for OwnedLeavesIterator<Node> 
    where Node: OwnedTreeNode {}

pub struct OwnedBinaryLeavesIterator<Node, Iter>
    where Node: OwnedBinaryTreeNode,
        Iter: Iterator<Item = Node> {

    pub (crate) root: Option<Node>,
    pub (crate) traversal_stack_bottom: Vec<Iter>,
    pub (crate) traversal_stack_top: Vec<BinaryChildren<Node>>,
    pub (crate) item_stack: Vec<Node::OwnedValue>,
}

impl<Node, Iter> Iterator for OwnedBinaryLeavesIterator<Node, Iter> 
    where Node: OwnedBinaryTreeNode,
        Iter: Iterator<Item = Node> {
    
    type Item = Node::OwnedValue;
    
    dfs_postorder_leaves_next!(get_value_and_children);
}

impl<Node, Iter> LeavesIterator for OwnedBinaryLeavesIterator<Node, Iter> 
    where Node: OwnedBinaryTreeNode,
        Iter: Iterator<Item = Node> {}