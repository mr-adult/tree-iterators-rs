use crate::prelude::OwnedBinaryTreeNode;
use streaming_iterator::StreamingIterator;

use super::{dfs_inorder_next, dfs_inorder_streaming_iterator_impl};

pub struct OwnedDFSInorderIterator<Node> 
    where Node: OwnedBinaryTreeNode {
    
    right_stack: Vec<Option<Node>>,
    item_stack: Vec<Node::OwnedValue>,
    moved: bool,
}

impl<Node> OwnedDFSInorderIterator<Node> 
    where Node: OwnedBinaryTreeNode {

    pub (crate) fn new(root: Node) -> OwnedDFSInorderIterator<Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        OwnedDFSInorderIterator {
            right_stack,
            item_stack: Vec::new(),
            moved: false,
        }
    }

    pub (crate) fn attach_ancestors(mut self) -> OwnedDFSInorderIteratorWithAncestors<Node> {
        let root = self.right_stack.pop();
        match self.moved {
            true => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            false => OwnedDFSInorderIteratorWithAncestors::new(root.unwrap().unwrap())
        }
    }
}

impl<Node> Iterator for OwnedDFSInorderIterator<Node>
    where Node: OwnedBinaryTreeNode {
    
    type Item = Node::OwnedValue;
    
    dfs_inorder_next!(get_value_and_left_right);
}

pub struct OwnedDFSInorderIteratorWithAncestors<Node> 
    where Node: OwnedBinaryTreeNode {
    
    right_stack: Vec<Option<Node>>,
    item_stack: Vec<Node::OwnedValue>,
    has_gone_right_stack: Vec<bool>,
    last_iteration_was_just_a_pop: bool,
}

impl<Node> OwnedDFSInorderIteratorWithAncestors<Node> 
    where Node: OwnedBinaryTreeNode {

    pub (crate) fn new(root: Node) -> OwnedDFSInorderIteratorWithAncestors<Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        OwnedDFSInorderIteratorWithAncestors {
            last_iteration_was_just_a_pop: false,
            right_stack,
            item_stack: Vec::new(),
            has_gone_right_stack: Vec::new(),
        }
    }
}

impl<Node> StreamingIterator for OwnedDFSInorderIteratorWithAncestors<Node>
    where Node: OwnedBinaryTreeNode {
    
    type Item = [Node::OwnedValue];
    
    dfs_inorder_streaming_iterator_impl!(get_value_and_left_right);
}