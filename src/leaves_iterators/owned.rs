use crate::prelude::{OwnedTreeNode, BinaryChildren, OwnedBinaryTreeNode};
use crate::make_peekable_iterator::MakePeekableIterator;

use super::dfs_preorder_next_with_children_check;

pub struct OwnedLeavesIterator<Node>
    where Node: OwnedTreeNode {

    root: Option<Node>,
    traversal_stack: Vec<MakePeekableIterator<Node::OwnedChildren>>,
}

impl<'a, Node> OwnedLeavesIterator<Node> 
    where Node: OwnedTreeNode {
        
    pub (crate) fn new(root: Node) -> OwnedLeavesIterator<Node> {
        OwnedLeavesIterator { 
            root: Some(root),
            traversal_stack: Vec::new()
        }
    }

    dfs_preorder_next_with_children_check!(get_value_and_children, Node::OwnedValue);
}

impl<'a, Node> Iterator for OwnedLeavesIterator<Node> 
    where Node: OwnedTreeNode {
    
    type Item = Node::OwnedValue;
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

pub struct OwnedBinaryLeavesIterator<Node>
    where Node: OwnedBinaryTreeNode {

    root: Option<Node>,
    traversal_stack: Vec<MakePeekableIterator<BinaryChildren<Node>>>,
}

impl<'a, Node> OwnedBinaryLeavesIterator<Node> 
    where Node: OwnedBinaryTreeNode {
        
    pub (crate) fn new(root: Node) -> OwnedBinaryLeavesIterator<Node> {
        OwnedBinaryLeavesIterator { 
            root: Some(root),
            traversal_stack: Vec::new()
        }
    }

    dfs_preorder_next_with_children_check!(get_value_and_children, Node::OwnedValue);
}

impl<'a, Node> Iterator for OwnedBinaryLeavesIterator<Node> 
    where Node: OwnedBinaryTreeNode {
    
    type Item = Node::OwnedValue;
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