use alloc::vec::Vec;
use streaming_iterator::StreamingIterator;

use crate::{
    leaves_iterators::{
        ancestors_depth_first::borrow::BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors,
        depth_first::borrow::BorrowedBinaryLeavesIterator,
    },
    prelude::{
        AncestorsIterator, AncestorsLeavesIterator, BorrowedBinaryTreeNode, LeavesIterator,
        TreeIterator,
    },
};

use super::{dfs_inorder_next, dfs_inorder_streaming_iterator_impl, TraversalStatus};

pub struct BorrowedDFSInorderIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    right_stack: Vec<Option<&'a Node>>,
    item_stack: Vec<Node::BorrowedValue>,
    moved: bool,
}

impl<'a, Node> BorrowedDFSInorderIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> BorrowedDFSInorderIterator<'a, Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        BorrowedDFSInorderIterator {
            right_stack,
            item_stack: Vec::new(),
            moved: false,
        }
    }
}

impl<'a, Node> TreeIterator for BorrowedDFSInorderIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        let mut traversal_stack_bottom = Vec::with_capacity(self.right_stack.capacity());
        for opt in self.right_stack {
            traversal_stack_bottom.push(opt.into_iter());
        }

        BorrowedBinaryLeavesIterator {
            root: None,
            traversal_stack_bottom: traversal_stack_bottom,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    fn attach_ancestors(mut self) -> impl AncestorsIterator<Item = [Node::BorrowedValue]> {
        let root = self.right_stack.pop();
        match self.moved {
            true => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            false => BorrowedDFSInorderIteratorWithAncestors::new(root.unwrap().unwrap())
        }
    }
}

impl<'a, Node> Iterator for BorrowedDFSInorderIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = Node::BorrowedValue;

    dfs_inorder_next!(get_value_and_children_binary_iter);
}

pub struct BorrowedDFSInorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    right_stack: Vec<Option<&'a Node>>,
    item_stack: Vec<Node::BorrowedValue>,
    status_stack: Vec<TraversalStatus>,
}

impl<'a, Node> BorrowedDFSInorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> BorrowedDFSInorderIteratorWithAncestors<Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        BorrowedDFSInorderIteratorWithAncestors {
            right_stack,
            item_stack: Vec::new(),
            status_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedDFSInorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = [Node::BorrowedValue];

    dfs_inorder_streaming_iterator_impl!(get_value_and_children_binary_iter);
}

impl<'a, Node> AncestorsIterator for BorrowedDFSInorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    fn leaves(mut self) -> impl AncestorsLeavesIterator<Item = Self::Item> {
        let root;
        let old_traversal_stack;

        if self.right_stack.len() == 1 {
            root = core::mem::take(self.right_stack.get_mut(0).unwrap());
            old_traversal_stack = Vec::new();
        } else {
            root = None;
            old_traversal_stack = self
                .right_stack
                .into_iter()
                .map(|opt| opt.into_iter())
                .collect();
        }

        BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors {
            root,
            item_stack: self.item_stack,
            old_traversal_stack,
            new_traversal_stack: Vec::new(),
        }
    }
}
