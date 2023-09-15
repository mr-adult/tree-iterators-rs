use streaming_iterator::StreamingIterator;

use crate::prelude::BorrowedBinaryTreeNode;

use super::{
    dfs_inorder_next, 
    dfs_inorder_streaming_iterator_impl
};

pub struct BorrowedDFSInorderIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {
    
    right_stack: Vec<Option<&'a Node>>,
    item_stack: Vec<Node::BorrowedValue>,
    moved: bool,
}

impl<'a, Node> BorrowedDFSInorderIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a Node) -> BorrowedDFSInorderIterator<'a, Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        BorrowedDFSInorderIterator {
            right_stack,
            item_stack: Vec::new(),
            moved: false,
        }
    }

    pub fn attach_ancestors(mut self) -> BorrowedDFSInorderIteratorWithAncestors<'a, Node> {
        let root = self.right_stack.pop();
        match self.moved {
            true => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            false => BorrowedDFSInorderIteratorWithAncestors::new(root.unwrap().unwrap())
        }
    }
}

impl<'a, Node> Iterator for BorrowedDFSInorderIterator<'a, Node>
    where Node: BorrowedBinaryTreeNode<'a> {
    
    type Item = Node::BorrowedValue;
    
    dfs_inorder_next!(get_value_and_left_right_iter);
}

pub struct BorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {
    
    right_stack: Vec<Option<&'a Node>>,
    item_stack: Vec<Node::BorrowedValue>,
    has_gone_right_stack: Vec<bool>,
    last_iteration_was_just_a_pop: bool,
}

impl<'a, Node> BorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a Node) -> BorrowedDFSInorderIteratorWithAncestors<Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        BorrowedDFSInorderIteratorWithAncestors {
            last_iteration_was_just_a_pop: false,
            right_stack,
            item_stack: Vec::new(),
            has_gone_right_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedDFSInorderIteratorWithAncestors<'a, Node>
    where Node: BorrowedBinaryTreeNode<'a> {
    
    type Item = [Node::BorrowedValue];
    
    dfs_inorder_streaming_iterator_impl!(get_value_and_left_right_iter);
}