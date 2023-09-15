use streaming_iterator::StreamingIterator;

use crate::prelude::MutBorrowedBinaryTreeNode;

use super::{
    dfs_inorder_next, 
    dfs_inorder_streaming_iterator_impl
};

pub struct MutBorrowedDFSInorderIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    right_stack: Vec<Option<&'a mut Node>>,
    item_stack: Vec<Node::MutBorrowedValue>,
    moved: bool
}

impl<'a, Node> MutBorrowedDFSInorderIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedDFSInorderIterator<'a, Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        MutBorrowedDFSInorderIterator {
            right_stack,
            item_stack: Vec::new(),
            moved: false,
        }
    }

    pub fn attach_ancestors(mut self) -> MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> {
        let root = self.right_stack.pop();
        match self.moved {
            true => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            false => MutBorrowedDFSInorderIteratorWithAncestors::new(root.unwrap().unwrap())
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedDFSInorderIterator<'a, Node>
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    type Item = Node::MutBorrowedValue;
    
    dfs_inorder_next!(get_value_and_left_right_iter_mut);
}

pub struct MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    right_stack: Vec<Option<&'a mut Node>>,
    item_stack: Vec<Node::MutBorrowedValue>,
    has_gone_right_stack: Vec<bool>,
    last_iteration_was_just_a_pop: bool,
}

impl<'a, Node> MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        MutBorrowedDFSInorderIteratorWithAncestors {
            last_iteration_was_just_a_pop: false,
            right_stack,
            item_stack: Vec::new(),
            has_gone_right_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedDFSInorderIteratorWithAncestors<'a, Node>
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    type Item = [Node::MutBorrowedValue];
    
    dfs_inorder_streaming_iterator_impl!(get_value_and_left_right_iter_mut);
}