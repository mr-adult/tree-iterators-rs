use crate::prelude::{
    BorrowedTreeNode, 
    BinaryChildren, 
    BorrowedBinaryTreeNode
};

use super::dfs_postorder_leaves_next;

pub struct BorrowedLeavesIterator<'a, Node, Iter>
    where Node: BorrowedTreeNode<'a>,
        Iter: Iterator<Item = &'a Node> {

    pub (crate) root: Option<&'a Node>,
    pub (crate) item_stack: Vec<Node::BorrowedValue>,
    pub (crate) traversal_stack_bottom: Vec<Iter>,
    pub (crate) traversal_stack_top: Vec<Node::BorrowedChildren>
}

impl<'a, Node, Iter> Iterator for BorrowedLeavesIterator<'a, Node, Iter> 
    where Node: BorrowedTreeNode<'a>,
        Iter: Iterator<Item = &'a Node> {
    
    type Item = Node::BorrowedValue;
    
    dfs_postorder_leaves_next!(get_value_and_children_iter);
}

pub struct BorrowedBinaryLeavesIterator<'a, Node, Iter>
    where Node: BorrowedBinaryTreeNode<'a>,
        Iter: Iterator<Item = &'a Node> {

    pub (crate) root: Option<&'a Node>,
    pub (crate) item_stack: Vec<Node::BorrowedValue>,
    pub (crate) traversal_stack_bottom: Vec<Iter>,
    pub (crate) traversal_stack_top: Vec<BinaryChildren<&'a Node>>
}

impl<'a, Node, Iter> Iterator for BorrowedBinaryLeavesIterator<'a, Node, Iter> 
    where Node: BorrowedBinaryTreeNode<'a>,
        Iter: Iterator<Item = &'a Node> {
    
    type Item = Node::BorrowedValue;

    dfs_postorder_leaves_next!(get_value_and_children_iter);
}