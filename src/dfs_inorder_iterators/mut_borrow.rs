use alloc::vec::Vec;
use streaming_iterator::{
    StreamingIterator,
    StreamingIteratorMut
};

use crate::{
    prelude::{
        MutBorrowedBinaryTreeNode, 
        LeavesIterator, 
        AncestorsIterator, 
        TreeIteratorMut, 
        AncestorsIteratorMut
    }, 
    leaves_iterators::depth_first::mut_borrow::MutBorrowedBinaryLeavesIterator,
};

use super::{
    get_mut,
    dfs_inorder_next, 
    dfs_inorder_streaming_iterator_impl,
    TraversalStatus
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
}

impl<'a, Node> TreeIteratorMut for MutBorrowedDFSInorderIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        let mut traversal_stack_bottom = Vec::with_capacity(self.right_stack.capacity());
        for opt in self.right_stack {
            traversal_stack_bottom.push(opt.into_iter());
        }

        MutBorrowedBinaryLeavesIterator {
            root: None,
            traversal_stack_bottom: traversal_stack_bottom,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    fn attach_ancestors(mut self) -> impl AncestorsIteratorMut<Item = [Node::MutBorrowedValue]> {
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
    
    dfs_inorder_next!(get_value_and_children_binary_iter_mut);
}

pub struct MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    right_stack: Vec<Option<&'a mut Node>>,
    item_stack: Vec<Node::MutBorrowedValue>,
    status_stack: Vec<TraversalStatus>,
}

impl<'a, Node> MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        MutBorrowedDFSInorderIteratorWithAncestors {
            right_stack,
            item_stack: Vec::new(),
            status_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedDFSInorderIteratorWithAncestors<'a, Node>
    where Node: MutBorrowedBinaryTreeNode<'a> {
    
    type Item = [Node::MutBorrowedValue];
    
    dfs_inorder_streaming_iterator_impl!(get_value_and_children_binary_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    get_mut!();
}

impl<'a, Node> AncestorsIterator for MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {}

impl<'a, Node> AncestorsIteratorMut for MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {}