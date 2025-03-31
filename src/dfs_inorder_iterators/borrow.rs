use core::{mem::MaybeUninit, option::IntoIter};

use alloc::vec::Vec;
use streaming_iterator::StreamingIterator;

use crate::{
    leaves_iterators::{
        ancestors_depth_first::borrow::BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors,
        depth_first::borrow::BorrowedBinaryLeavesIterator,
    },
    prelude::{BinaryTreeContextRef, BorrowedBinaryTreeNode},
};

use super::{dfs_inorder_ancestors_streaming_iterator_impl, dfs_inorder_next, TraversalStatus};

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

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(self) -> BorrowedBinaryLeavesIterator<'a, Node, IntoIter<&'a Node>> {
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

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(mut self) -> BorrowedDFSInorderIteratorWithContext<'a, Node> {
        let root = self.right_stack.pop();
        match self.moved {
            true => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            false => BorrowedDFSInorderIteratorWithContext::new(root.unwrap().unwrap())
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(mut self) -> BorrowedDFSInorderIteratorWithAncestors<'a, Node> {
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

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(
        mut self,
    ) -> BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors<'a, Node, IntoIter<&'a Node>> {
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

impl<'a, Node> StreamingIterator for BorrowedDFSInorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = [Node::BorrowedValue];
    dfs_inorder_ancestors_streaming_iterator_impl!(get_value_and_children_binary_iter);
}

pub struct BorrowedDFSInorderIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    right_stack: Vec<Option<&'a Node>>,
    current_context: BinaryTreeContextRef<'a, Node>,
    into_iterator_stack: Vec<[Option<&'a Node>; 2]>,
    status_stack: Vec<TraversalStatus>,
}

impl<'a, Node> BorrowedDFSInorderIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> BorrowedDFSInorderIteratorWithContext<Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        let context = BinaryTreeContextRef::new();

        Self {
            right_stack,
            current_context: context,
            into_iterator_stack: Vec::new(),
            status_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(
        mut self,
    ) -> BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors<'a, Node, IntoIter<&'a Node>> {
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
            item_stack: self.current_context.ancestors,
            old_traversal_stack,
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedDFSInorderIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = BinaryTreeContextRef<'a, Node>;

    fn advance(&mut self) {
        let mut current = None;
        while current.is_none() {
            if self.right_stack.is_empty() {
                self.current_context.ancestors.clear();
                break;
            }

            while let Some(TraversalStatus::WentRight) = self.status_stack.last() {
                self.current_context.ancestors.pop();
                self.current_context.path.pop();
                self.status_stack.pop();
            }

            if let Some(last_status) = self.status_stack.last_mut() {
                if !matches!(last_status, TraversalStatus::ReturnedSelf) {
                    *last_status = TraversalStatus::ReturnedSelf;
                    self.current_context.children =
                        MaybeUninit::new(self.into_iterator_stack.pop().unwrap());
                    return;
                }
            }

            if current.is_some() {
                continue;
            }

            if let Some(last_status) = self.status_stack.last_mut() {
                *last_status = TraversalStatus::WentRight;
            }

            if let Some(top_of_right_stack) = self.right_stack.pop() {
                current = top_of_right_stack;
                continue;
            }

            while let Some(TraversalStatus::WentRight) = self.status_stack.last() {
                self.current_context.ancestors.pop();
                self.current_context.path.pop();
                self.status_stack.pop();
                self.current_context.children =
                    MaybeUninit::new(self.into_iterator_stack.pop().unwrap());
            }
            return;
        }

        while let Some(current_val) = current {
            let (value, children) = current_val.get_value_and_children_binary_iter();

            self.right_stack
                .push(unsafe { core::ptr::read(&children[1] as *const Option<&'a Node>) });
            let left = unsafe { core::ptr::read(&children[0] as *const Option<&'a Node>) };
            self.into_iterator_stack.push(children);

            self.current_context.ancestors.push(value);
            match self.status_stack.last() {
                None => {}
                Some(TraversalStatus::WentLeft | TraversalStatus::ReturnedSelf) => {
                    self.current_context.path.push(0)
                }
                Some(TraversalStatus::WentRight) => self.current_context.path.push(1),
            }
            self.status_stack.push(TraversalStatus::WentLeft);
            current = left;
        }

        if let Some(last_status) = self.status_stack.last_mut() {
            *last_status = TraversalStatus::ReturnedSelf;
        }

        if let Some(children) = self.into_iterator_stack.pop() {
            self.current_context.children = MaybeUninit::new(children);
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        if self.current_context.ancestors.is_empty() {
            None
        } else {
            Some(&self.current_context)
        }
    }
}
