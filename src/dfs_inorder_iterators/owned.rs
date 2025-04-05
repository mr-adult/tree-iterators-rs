use core::option::IntoIter;

use crate::{
    leaves_iterators::{
        ancestors_depth_first::owned::OwnedBinaryDFSLeavesPostorderIteratorWithAncestors,
        depth_first::owned::OwnedBinaryLeavesIterator,
    },
    prelude::OwnedBinaryTreeNode,
    tree_context::BinaryTreeContextNoChildren,
};
use alloc::vec::Vec;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use super::{
    dfs_inorder_ancestors_streaming_iterator_impl, dfs_inorder_next, get_mut_ancestors,
    get_mut_context, TraversalStatus,
};

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

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(self) -> OwnedBinaryLeavesIterator<Node, IntoIter<Node>> {
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

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(mut self) -> OwnedDFSInorderIteratorWithContext<Node> {
        let root = self.right_stack.pop();
        match self.moved {
            true => panic!("Attempted to attach metadata to a DFS in order iterator in the middle of a tree traversal. This is forbidden."),
            false => OwnedDFSInorderIteratorWithContext::new(root.unwrap().unwrap())
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(mut self) -> OwnedDFSInorderIteratorWithAncestors<Node> {
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

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(
        mut self,
    ) -> OwnedBinaryDFSLeavesPostorderIteratorWithAncestors<Node, IntoIter<Node>> {
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

impl<Node> StreamingIterator for OwnedDFSInorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = [Node::OwnedValue];

    dfs_inorder_ancestors_streaming_iterator_impl!(get_value_and_children_binary);
}

impl<Node> StreamingIteratorMut for OwnedDFSInorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    get_mut_ancestors!();
}

pub struct OwnedDFSInorderIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    right_stack: Vec<Option<Node>>,
    current_context: BinaryTreeContextNoChildren<Node>,
    status_stack: Vec<TraversalStatus>,
}

impl<Node> OwnedDFSInorderIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) fn new(root: Node) -> OwnedDFSInorderIteratorWithContext<Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        let context = BinaryTreeContextNoChildren::new();

        Self {
            right_stack,
            current_context: context,
            status_stack: Vec::new(),
        }
    }
}

impl<Node> StreamingIterator for OwnedDFSInorderIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = BinaryTreeContextNoChildren<Node>;

    fn advance(&mut self) {
        let mut current = None;
        while current.is_none() {
            if let Some(last_status) = self.status_stack.last_mut() {
                match last_status {
                    TraversalStatus::WentRight => {
                        self.current_context.ancestors.pop();
                        self.current_context.path.pop();
                        self.status_stack.pop();
                        continue;
                    }
                    TraversalStatus::WentLeft => {
                        *last_status = TraversalStatus::ReturnedSelf;
                        return;
                    }
                    TraversalStatus::ReturnedSelf => {
                        *last_status = TraversalStatus::WentRight;
                    }
                }
            }

            if let Some(top_of_right_stack) = self.right_stack.pop() {
                current = top_of_right_stack;
                continue;
            } else {
                self.current_context.ancestors.clear();
                return;
            }
        }

        while let Some(current_val) = current {
            let (value, [left, right]) = current_val.get_value_and_children_binary();

            self.current_context.ancestors.push(value);
            match self.status_stack.last() {
                None => {}
                Some(TraversalStatus::WentLeft | TraversalStatus::ReturnedSelf) => {
                    self.current_context.path.push(0)
                }
                Some(TraversalStatus::WentRight) => self.current_context.path.push(1),
            }

            self.right_stack.push(right);

            self.status_stack.push(TraversalStatus::WentLeft);
            current = left;
        }

        let status_stack_len = self.status_stack.len();
        self.status_stack[status_stack_len - 1] = TraversalStatus::ReturnedSelf;
    }

    fn get(&self) -> Option<&Self::Item> {
        if self.current_context.ancestors.is_empty() {
            None
        } else {
            Some(&self.current_context)
        }
    }
}

impl<'a, Node> StreamingIteratorMut for OwnedDFSInorderIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    get_mut_context!();
}
