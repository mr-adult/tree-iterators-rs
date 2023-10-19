use crate::prelude::{BorrowedTreeNode, BinaryChildren, BorrowedBinaryTreeNode};
use crate::make_peekable_iterator::MakePeekableIterator;

use super::dfs_preorder_next_with_children_check;

pub struct BorrowedLeavesIterator<'a, Node>
    where Node: BorrowedTreeNode<'a> {

    root: Option<&'a Node>,
    traversal_stack: Vec<MakePeekableIterator<Node::BorrowedChildren>>,
    
}

impl<'a, Node> BorrowedLeavesIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
        
    pub (crate) fn new(root: &'a Node) -> BorrowedLeavesIterator<'a, Node> {
        BorrowedLeavesIterator { 
            root: Some(root),
            traversal_stack: Vec::new()
        }
    }

    dfs_preorder_next_with_children_check!(get_value_and_children_iter, Node::BorrowedValue);
}

impl<'a, Node> Iterator for BorrowedLeavesIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
    
    type Item = Node::BorrowedValue;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let value = self.dfs_preorder_next();
            match value {
                None => return None,
                Some(value) => {
                    if value.0 { continue; }
                    return Some(value.1);
                }
            }
        }
    }
}

pub struct BorrowedBinaryLeavesIterator<'a, Node>
    where Node: BorrowedBinaryTreeNode<'a> {

    root: Option<&'a Node>,
    traversal_stack: Vec<MakePeekableIterator<BinaryChildren<&'a Node>>>,
}

impl<'a, Node> BorrowedBinaryLeavesIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {
        
    pub (crate) fn new(root: &'a Node) -> BorrowedBinaryLeavesIterator<'a, Node> {
        BorrowedBinaryLeavesIterator { 
            root: Some(root),
            traversal_stack: Vec::new()
        }
    }

    dfs_preorder_next_with_children_check!(get_value_and_children_iter, Node::BorrowedValue);
}

impl<'a, Node> Iterator for BorrowedBinaryLeavesIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {
    
    type Item = Node::BorrowedValue;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let value = self.dfs_preorder_next();
            match value {
                None => return None,
                Some(value) => {
                    if value.0 { continue; }
                    return Some(value.1);
                }
            }
        }
    }
}