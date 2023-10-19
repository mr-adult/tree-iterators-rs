use crate::prelude::{MutBorrowedTreeNode, BinaryChildren, MutBorrowedBinaryTreeNode};
use crate::make_peekable_iterator::MakePeekableIterator;

use super::dfs_preorder_next_with_children_check;

pub struct MutBorrowedLeavesIterator<'a, Node>
    where Node: MutBorrowedTreeNode<'a> {

    root: Option<&'a mut Node>,
    traversal_stack: Vec<MakePeekableIterator<Node::MutBorrowedChildren>>,
}

impl<'a, Node> MutBorrowedLeavesIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {
        
    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedLeavesIterator<'a, Node> {
        MutBorrowedLeavesIterator { 
            root: Some(root),
            traversal_stack: Vec::new()
        }
    }

    dfs_preorder_next_with_children_check!(get_value_and_children_iter_mut, Node::MutBorrowedValue);
}

impl<'a, Node> Iterator for MutBorrowedLeavesIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {
    
    type Item = Node::MutBorrowedValue;
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

pub struct MutBorrowedBinaryLeavesIterator<'a, Node>
    where Node: MutBorrowedBinaryTreeNode<'a> {

    root: Option<&'a mut Node>,
    traversal_stack: Vec<MakePeekableIterator<BinaryChildren<&'a mut Node>>>,
}

impl<'a, Node> MutBorrowedBinaryLeavesIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {
        
    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedBinaryLeavesIterator<'a, Node> {
        MutBorrowedBinaryLeavesIterator { 
            root: Some(root),
            traversal_stack: Vec::new()
        }
    }

    dfs_preorder_next_with_children_check!(get_value_and_children_iter_mut, Node::MutBorrowedValue);
}

impl<'a, Node> Iterator for MutBorrowedBinaryLeavesIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    type Item = Node::MutBorrowedValue;
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