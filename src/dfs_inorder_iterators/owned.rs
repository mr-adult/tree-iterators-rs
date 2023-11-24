use crate::{
    leaves_iterators::{
        ancestors_depth_first::owned::OwnedBinaryDFSLeavesPostorderIteratorWithAncestors,
        depth_first::owned::OwnedBinaryLeavesIterator,
    },
    prelude::{
        AncestorsIteratorMut, AncestorsLeavesIteratorMut, LeavesIterator, OwnedBinaryTreeNode,
        TreeIteratorMut,
    },
};
use alloc::vec::Vec;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use super::{dfs_inorder_next, dfs_inorder_streaming_iterator_impl, get_mut, TraversalStatus};

pub struct OwnedDFSInorderIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    right_stack: Vec<Option<Node>>,
    item_stack: Vec<Node::OwnedValue>,
    moved: bool,
}

impl<Node> OwnedDFSInorderIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) fn new(root: Node) -> OwnedDFSInorderIterator<Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        OwnedDFSInorderIterator {
            right_stack,
            item_stack: Vec::new(),
            moved: false,
        }
    }
}

impl<Node> TreeIteratorMut for OwnedDFSInorderIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        let mut traversal_stack_bottom = Vec::with_capacity(self.right_stack.capacity());
        for opt in self.right_stack {
            traversal_stack_bottom.push(opt.into_iter());
        }

        OwnedBinaryLeavesIterator {
            root: None,
            traversal_stack_bottom: traversal_stack_bottom,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    fn attach_ancestors(mut self) -> impl AncestorsIteratorMut<Item = [Node::OwnedValue]> {
        let root = self.right_stack.pop();
        match self.moved {
            true => panic!("Attempted to attach metadata to a DFS in order iterator in the middle of a tree traversal. This is forbidden."),
            false => OwnedDFSInorderIteratorWithAncestors::new(root.unwrap().unwrap())
        }
    }
}

impl<Node> Iterator for OwnedDFSInorderIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = Node::OwnedValue;

    dfs_inorder_next!(get_value_and_children_binary);
}

pub struct OwnedDFSInorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    right_stack: Vec<Option<Node>>,
    item_stack: Vec<Node::OwnedValue>,
    status_stack: Vec<TraversalStatus>,
}

impl<Node> OwnedDFSInorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) fn new(root: Node) -> OwnedDFSInorderIteratorWithAncestors<Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        OwnedDFSInorderIteratorWithAncestors {
            right_stack,
            item_stack: Vec::new(),
            status_stack: Vec::new(),
        }
    }
}

impl<Node> StreamingIterator for OwnedDFSInorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = [Node::OwnedValue];

    dfs_inorder_streaming_iterator_impl!(get_value_and_children_binary);
}

impl<Node> StreamingIteratorMut for OwnedDFSInorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    get_mut!();
}

impl<Node> AncestorsIteratorMut for OwnedDFSInorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    fn leaves(mut self) -> impl AncestorsLeavesIteratorMut<Item = Self::Item> {
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

        OwnedBinaryDFSLeavesPostorderIteratorWithAncestors {
            root,
            item_stack: self.item_stack,
            old_traversal_stack,
            new_traversal_stack: Vec::new(),
        }
    }
}
